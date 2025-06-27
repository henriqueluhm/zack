use crossterm::{
    QueueableCommand,
    cursor::{self, SetCursorStyle},
};
use ratatui::{Frame, layout::Rect};
use std::io::{Write, stdout};

use crate::app::modes::EditorMode;

#[derive(Debug)]
pub struct Cursor {
    pub position: (usize, usize),
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

    fn set_cursor_style(&self, current_mode: EditorMode) -> SetCursorStyle {
        if current_mode == EditorMode::Insert {
            return cursor::SetCursorStyle::SteadyBar;
        }

        cursor::SetCursorStyle::SteadyBlock
    }

    pub fn render_cursor(&self, frame: &mut Frame, current_mode: EditorMode) {
        let cursor_position = self.calculate_cursor_position(frame.area());

        let mut stdout = stdout();

        frame.set_cursor_position(cursor_position);

        stdout.queue(self.set_cursor_style(current_mode)).unwrap();

        stdout.flush().unwrap();
    }

    fn calculate_cursor_position(&self, area: Rect) -> ratatui::layout::Position {
        let text_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let (cursor_line, cursor_col) = self.position;

        let clamped_line = cursor_line.min(text_area.height.saturating_sub(1) as usize);
        let clamped_col = cursor_col.min(text_area.width.saturating_sub(1) as usize);

        ratatui::layout::Position {
            x: text_area.x + clamped_col as u16,
            y: text_area.y + clamped_line as u16,
        }
    }
}
