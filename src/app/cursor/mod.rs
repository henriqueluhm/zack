use crate::{app::modes::EditorMode, event::AppEvent};
use crossterm::{
    QueueableCommand,
    cursor::{self, SetCursorStyle},
};
use ratatui::{Frame, layout::Rect};
use ropey::Rope;
use std::io::{Write, stdout};

#[derive(Debug)]
pub struct Cursor {
    pub position: (usize, usize),
}

#[derive(Clone, Debug)]
pub enum CursorEvent {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveToLineStart,
    MoveToLineEnd,
    SetPosition { line: usize, col: usize },
    SetLinePosition(usize),
    SetColPosition(usize),
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self { position: (0, 0) }
    }

    pub fn handle_event(&mut self, event: CursorEvent, buffer: &Rope) -> Option<AppEvent> {
        match event {
            CursorEvent::MoveLeft => {
                self.move_left();
                None
            }
            CursorEvent::MoveRight => {
                self.move_right(buffer);
                None
            }
            CursorEvent::MoveUp => {
                self.move_up(buffer);
                None
            }
            CursorEvent::MoveDown => {
                self.move_down(buffer);
                None
            }
            CursorEvent::MoveToLineStart => {
                self.move_to_line_start();
                None
            }
            CursorEvent::MoveToLineEnd => {
                self.move_to_line_end(buffer);
                None
            }
            CursorEvent::SetPosition { line, col } => {
                self.set_position(line, col, buffer);
                None
            }
            CursorEvent::SetLinePosition(line) => {
                self.set_line_position(line, buffer);
                None
            }
            CursorEvent::SetColPosition(col) => {
                self.set_col_position(col, buffer);
                None
            }
        }
    }

    pub fn render_cursor(&self, frame: &mut Frame, current_mode: EditorMode) {
        let cursor_position = self.calculate_cursor_position(frame.area());

        let mut stdout = stdout();
        frame.set_cursor_position(cursor_position);
        stdout.queue(self.set_cursor_style(current_mode)).unwrap();
        stdout.flush().unwrap();
    }

    pub fn move_left(&mut self) {
        if self.position.1 > 0 {
            self.position.1 -= 1;
        }
    }

    pub fn move_right(&mut self, buffer: &Rope) {
        let max_col = self.max_visible_col(buffer);

        if self.position.1 < max_col {
            self.position.1 += 1;
        }
    }

    pub fn move_up(&mut self, buffer: &Rope) {
        if self.position.0 > 0 {
            self.position.0 -= 1;
            self.clamp_col(buffer);
        }
    }

    pub fn move_down(&mut self, buffer: &Rope) {
        let total_lines = buffer.len_lines();

        if self.position.0 + 1 < total_lines {
            self.position.0 += 1;
            self.clamp_col(buffer);
        }
    }

    pub fn move_to_line_start(&mut self) {
        self.position.1 = 0;
    }

    pub fn move_to_line_end(&mut self, buffer: &Rope) {
        self.position.1 = self.max_visible_col(buffer);
    }

    pub fn set_position(&mut self, line: usize, col: usize, buffer: &Rope) {
        self.set_line_position(line, buffer);
        self.set_col_position(col, buffer);
    }

    pub fn set_line_position(&mut self, line: usize, buffer: &Rope) {
        let total_lines = buffer.len_lines().saturating_sub(1);
        self.position.0 = line.min(total_lines);
        self.clamp_col(buffer);
    }

    pub fn set_col_position(&mut self, col: usize, buffer: &Rope) {
        let max_col = self.max_visible_col(buffer);
        self.position.1 = col.min(max_col);
    }

    fn clamp_col(&mut self, buffer: &Rope) {
        self.position.1 = self.position.1.min(self.max_visible_col(buffer));
    }

    fn max_visible_col(&self, buffer: &Rope) -> usize {
        let line = buffer.line(self.position.0);
        let len = line.len_chars();
        if len == 0 {
            return 0;
        }

        let last_char = line.char(len.saturating_sub(1));
        if last_char == '\n' { len - 1 } else { len }
    }

    fn set_cursor_style(&self, current_mode: EditorMode) -> SetCursorStyle {
        if current_mode == EditorMode::Insert {
            return cursor::SetCursorStyle::SteadyBar;
        }

        cursor::SetCursorStyle::SteadyBlock
    }

    fn calculate_cursor_position(&self, area: Rect) -> ratatui::layout::Position {
        let text_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let (line, col) = self.position;

        let clamped_line = line.min(text_area.height.saturating_sub(1) as usize);
        let clamped_col = col.min(text_area.width.saturating_sub(1) as usize);

        ratatui::layout::Position {
            x: text_area.x + clamped_col as u16,
            y: text_area.y + clamped_line as u16,
        }
    }
}
