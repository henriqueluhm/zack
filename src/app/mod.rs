use crate::app::cursor::{Cursor, CursorEvent};
use crate::app::modes::normal::NormalMode;
use crate::app::modes::{Mode, change_mode};
use crate::event::{AppEvent, Event, EventHandler};
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ropey::Rope;

pub mod cursor;
pub mod modes;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub mode: Box<dyn Mode>,
    pub cursor: Cursor,
    pub buffer: Rope,
    pub event_handler: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            buffer: Rope::from_str("Hello, Zack!"),
            mode: Box::new(NormalMode),
            cursor: Cursor::new(),
            event_handler: EventHandler::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_event()?;
        }

        Ok(())
    }

    pub fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

        self.cursor
            .render_cursor(frame, self.mode.get_current_mode())
    }

    pub fn handle_event(&mut self) -> color_eyre::Result<()> {
        match self.event_handler.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => self.handle_crossterm_event(event),
            Event::App(event) => self.handle_app_event(event),
        }

        Ok(())
    }

    pub fn handle_crossterm_event(&mut self, event: crossterm::event::Event) {
        if let crossterm::event::Event::Key(key_event) = event {
            for event in self.mode.handle_key(key_event) {
                self.event_handler.send(event);
            }
        }
    }

    pub fn handle_app_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Cursor(cursor_event) => {
                if let Some(next_event) = self.cursor.handle_event(cursor_event, &self.buffer) {
                    self.event_handler.send(next_event);
                }
            }

            AppEvent::ChangeToMode(new_mode) => change_mode(new_mode, self),

            // TODO: Move ropey buffer manipulations to its own module with event handling
            AppEvent::InsertChar(char) => {
                let (line, col) = self.cursor.position;
                let char_index = self.calculate_char_index(line, col);
                self.buffer.insert_char(char_index, char);

                self.event_handler
                    .send(AppEvent::Cursor(CursorEvent::MoveRight));
            }
            AppEvent::DeleteChar => {
                let (line, col) = self.cursor.position;

                if col == 0 && line > 0 {
                    let prev_line_len = self.buffer.line(line - 1).len_chars();
                    let char_index = self.calculate_char_index(line, 0);

                    if char_index > 0 {
                        self.buffer.remove(char_index - 1..char_index);

                        self.event_handler
                            .send(AppEvent::Cursor(CursorEvent::SetLinePosition(line - 1)));
                        self.event_handler
                            .send(AppEvent::Cursor(CursorEvent::SetColPosition(
                                prev_line_len - 1,
                            )));
                    }
                } else if col > 0 {
                    let char_index = self.calculate_char_index(line, col);
                    self.buffer.remove(char_index - 1..char_index);
                    self.event_handler
                        .send(AppEvent::Cursor(CursorEvent::MoveLeft));
                }
            }
            AppEvent::InsertNewline => {
                let (line, col) = self.cursor.position;
                let char_index = self.calculate_char_index(line, col);

                self.buffer.insert(char_index, "\n");

                self.event_handler
                    .send(AppEvent::Cursor(CursorEvent::MoveDown));
                self.event_handler
                    .send(AppEvent::Cursor(CursorEvent::MoveToLineStart));
            }

            AppEvent::Quit => self.quit(),
        }
    }

    pub fn calculate_char_index(&self, line: usize, col: usize) -> usize {
        let line_start = self.buffer.line_to_char(line);
        let line_len = self.buffer.line(line).len_chars();

        let clamped_col = col.min(line_len);

        line_start + clamped_col
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }
}
