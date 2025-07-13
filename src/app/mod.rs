use std::path::PathBuf;

use crate::app::buffer::Buffer;
use crate::app::cursor::Cursor;
use crate::app::file::{File, FileEvent};
use crate::app::modes::normal::NormalMode;
use crate::app::modes::{Mode, change_mode};
use crate::event::{AppEvent, Event, EventHandler};
use crate::ui::FocusState;
use crossterm::event::KeyCode;
use ratatui::DefaultTerminal;
use ratatui::Frame;

pub mod buffer;
pub mod cursor;
pub mod file;
pub mod modes;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub focus: FocusState,
    pub filename_input: String,
    pub mode: Box<dyn Mode>,
    pub cursor: Cursor,
    pub buffer: Buffer,
    pub file: File,
    pub event_handler: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self::new(String::from(""), None)
    }
}

impl App {
    pub fn new(initial_text: String, maybe_path: Option<PathBuf>) -> Self {
        Self {
            running: true,
            buffer: Buffer::new(initial_text),
            file: File::new(maybe_path),
            mode: Box::new(NormalMode),
            cursor: Cursor::new(),
            event_handler: EventHandler::new(),
            focus: FocusState::Editor,
            filename_input: String::from(""),
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_event()?;
        }

        Ok(())
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

        self.cursor
            .render_cursor(frame, self.mode.get_current_mode())
    }

    fn handle_event(&mut self) -> color_eyre::Result<()> {
        match self.event_handler.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => self.handle_crossterm_event(event),
            Event::App(event) => self.handle_app_event(event),
        }

        Ok(())
    }

    fn handle_crossterm_event(&mut self, event: crossterm::event::Event) {
        if let crossterm::event::Event::Key(key_event) = event {
            match self.focus {
                FocusState::FilenamePrompt => match key_event.code {
                    KeyCode::Esc => {
                        self.focus = FocusState::Editor;
                        self.filename_input.clear();
                    }
                    KeyCode::Enter => {
                        if !self.filename_input.is_empty() {
                            self.file.path = Some(PathBuf::from(&self.filename_input));
                            let events = self.file.handle_event(FileEvent::Save, &self.buffer);
                            self.dispatch_multiple_events(events);
                            self.focus = FocusState::Editor;
                            self.filename_input.clear();
                        }
                    }
                    KeyCode::Backspace => {
                        self.filename_input.pop();
                    }
                    KeyCode::Char(c) => {
                        self.filename_input.push(c);
                    }
                    _ => {}
                },

                FocusState::Editor => {
                    for event in self.mode.handle_key(key_event, self.cursor.position) {
                        self.event_handler.send(event);
                    }
                }
            }
        }
    }

    fn handle_app_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Cursor(cursor_event) => {
                let next_events = self.cursor.handle_event(cursor_event, &self.buffer);
                self.dispatch_multiple_events(next_events);
            }

            AppEvent::Buffer(buffer_event) => {
                let next_events = self.buffer.handle_event(buffer_event);
                self.dispatch_multiple_events(next_events);
            }

            AppEvent::File(file_event) => {
                let next_events = self.file.handle_event(file_event, &self.buffer);
                self.dispatch_multiple_events(next_events);
            }

            AppEvent::PromptForFilename => {
                self.focus = FocusState::FilenamePrompt;
            }

            AppEvent::ChangeToMode(new_mode) => change_mode(new_mode, self),

            AppEvent::Quit => self.quit(),
        }
    }

    fn dispatch_multiple_events(&mut self, events: Vec<AppEvent>) {
        for event in events {
            self.event_handler.send(event);
        }
    }
}
