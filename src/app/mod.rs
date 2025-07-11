use crate::app::buffer::Buffer;
use crate::app::cursor::Cursor;
use crate::app::modes::normal::NormalMode;
use crate::app::modes::{Mode, change_mode};
use crate::event::{AppEvent, Event, EventHandler};
use ratatui::DefaultTerminal;
use ratatui::Frame;

pub mod buffer;
pub mod cursor;
pub mod modes;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub mode: Box<dyn Mode>,
    pub cursor: Cursor,
    pub buffer: Buffer,
    pub event_handler: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            buffer: Buffer::new(),
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
            for event in self.mode.handle_key(key_event, self.cursor.position) {
                self.event_handler.send(event);
            }
        }
    }

    fn handle_app_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Cursor(cursor_event) => {
                let returned_events = self.cursor.handle_event(cursor_event, &self.buffer);

                for event in returned_events {
                    self.event_handler.send(event);
                }
            }

            AppEvent::Buffer(buffer_event) => {
                let returned_events = self.buffer.handle_event(buffer_event);

                for event in returned_events {
                    self.event_handler.send(event);
                }
            }

            AppEvent::ChangeToMode(new_mode) => change_mode(new_mode, self),

            AppEvent::Quit => self.quit(),
        }
    }
}
