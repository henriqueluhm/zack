use crate::app::modes::EditorMode;
use crate::event::{AppEvent, Event, EventHandler};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};
use ropey::Rope;

pub mod modes;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    pub mode: EditorMode,
    pub buffer: Rope,
    pub cursor: (usize, usize),
    pub events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            buffer: Rope::from_str("Hello, Zack!"),
            mode: EditorMode::Normal,
            cursor: (0, 0),
            events: EventHandler::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render_with_cursor(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    pub fn render_with_cursor(&self, frame: &mut Frame) {
        let area = frame.area();

        frame.render_widget(self, area);

        let cursor_position = self.calculate_cursor_position(area);
        frame.set_cursor_position(cursor_position);
    }

    fn calculate_cursor_position(&self, area: Rect) -> ratatui::layout::Position {
        let text_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let (cursor_line, cursor_col) = self.cursor;

        let clamped_line = cursor_line.min(text_area.height.saturating_sub(1) as usize);
        let clamped_col = cursor_col.min(text_area.width.saturating_sub(1) as usize);

        ratatui::layout::Position {
            x: text_area.x + clamped_col as u16,
            y: text_area.y + clamped_line as u16,
        }
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::InsertChar(char) => {
                    let (line, col) = self.cursor;
                    let char_index = self.calculate_char_index(line, col);
                    self.buffer.insert_char(char_index, char);
                    self.cursor.1 += 1;
                }
                AppEvent::DeleteChar => {
                    let (line, col) = self.cursor;
                    if col > 0 {
                        let char_index = self.calculate_char_index(line, col);
                        self.buffer.remove(char_index - 1..char_index);
                        self.cursor.1 -= 1;
                    }
                }
                AppEvent::MoveCursorLeft => {
                    if self.cursor.1 > 0 {
                        self.cursor.1 -= 1;
                    }
                }
                AppEvent::MoveCursorRight => {
                    self.cursor.1 += 1;
                }
                AppEvent::MoveCursorUp => {
                    if self.cursor.0 > 0 {
                        self.cursor.0 -= 1;
                    }
                }
                AppEvent::MoveCursorDown => {
                    self.cursor.0 += 1;
                }
                AppEvent::Quit => self.quit(),
            },
        }

        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        use EditorMode::*;

        match self.mode {
            Insert => self.handle_insert_mode(key_event),
            Normal => self.handle_normal_mode(key_event),
        }

        Ok(())
    }

    fn handle_insert_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.mode = EditorMode::Normal,
            KeyCode::Backspace => self.events.send(AppEvent::DeleteChar),
            KeyCode::Char(c) => self.events.send(AppEvent::InsertChar(c)),
            KeyCode::Left => self.events.send(AppEvent::MoveCursorLeft),
            KeyCode::Right => self.events.send(AppEvent::MoveCursorRight),
            KeyCode::Up => self.events.send(AppEvent::MoveCursorUp),
            KeyCode::Down => self.events.send(AppEvent::MoveCursorDown),
            _ => {}
        }
    }

    fn handle_normal_mode(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('i') => self.mode = EditorMode::Insert,
            KeyCode::Char('h') => self.events.send(AppEvent::MoveCursorLeft),
            KeyCode::Char('l') => self.events.send(AppEvent::MoveCursorRight),
            KeyCode::Char('j') => self.events.send(AppEvent::MoveCursorDown),
            KeyCode::Char('k') => self.events.send(AppEvent::MoveCursorUp),
            KeyCode::Char('q') | KeyCode::Esc => self.events.send(AppEvent::Quit),

            KeyCode::Char('c' | 'C') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                self.events.send(AppEvent::Quit);
            }

            _ => {}
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
