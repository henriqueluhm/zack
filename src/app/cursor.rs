//! Cursor management module for the Zack text editor.
//!
//! This module provides the [`Cursor`] struct which holds the current position
//! within the text buffer and methods to move or set the cursor position based
//! on various user actions. It also defines [`CursorEvent`], which represents
//! all possible cursor-related actions.

use crate::app::{buffer::Buffer, modes::EditorMode};
use crate::event::AppEvent;
use crate::types::position::Position;
use crossterm::{
    QueueableCommand,
    cursor::{self, SetCursorStyle},
};
use ratatui::{Frame, layout::Rect};
use std::io::{Write, stdout};

/// Represents the text cursor, tracking its current line and column position.
#[derive(Debug)]
pub struct Cursor {
    /// Current position of the cursor in the buffer.
    pub position: Position,
}

/// Events that can trigger cursor movement or repositioning.
#[derive(Clone, Debug, PartialEq, Eq)]
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
    /// Creates a new `Cursor` positioned at the start of the buffer.
    pub fn new() -> Self {
        Self {
            position: Position::new(0, 0),
        }
    }

    /// Handles a `CursorEvent`, possibly adjusting the cursor's position.
    /// Returns any follow-up `AppEvent`s.
    pub fn handle_event(&mut self, event: CursorEvent, buffer: &Buffer) -> Vec<AppEvent> {
        let mut events = vec![];

        match event {
            CursorEvent::MoveLeft => events.extend(self.move_left()),
            CursorEvent::MoveRight => events.extend(self.move_right(buffer)),
            CursorEvent::MoveUp => events.extend(self.move_up(buffer)),
            CursorEvent::MoveDown => events.extend(self.move_down(buffer)),
            CursorEvent::MoveToLineStart => events.extend(self.move_to_line_start()),
            CursorEvent::MoveToLineEnd => events.extend(self.move_to_line_end(buffer)),
            CursorEvent::SetColPosition(col) => events.extend(self.set_col_position(col, buffer)),
            CursorEvent::SetLinePosition(line) => {
                events.extend(self.set_line_position(line, buffer))
            }
            CursorEvent::SetPosition { line, col } => {
                events.extend(self.set_position(line, col, buffer))
            }
        }

        events
    }

    /// Renders the cursor at the correct screen position with appropriate style.
    pub fn render_cursor(&self, frame: &mut Frame, current_mode: EditorMode) {
        let cursor_position = self.calculate_cursor_position(frame.area());

        let mut stdout = stdout();
        frame.set_cursor_position(cursor_position);
        stdout.queue(self.set_cursor_style(current_mode)).unwrap();
        stdout.flush().unwrap();
    }

    fn move_left(&mut self) -> Vec<AppEvent> {
        if self.position.col > 0 {
            self.position.col -= 1;
        }

        vec![]
    }

    fn move_right(&mut self, buffer: &Buffer) -> Vec<AppEvent> {
        let max_col = buffer.max_visible_col(&self.position);

        if self.position.col < max_col {
            self.position.col += 1;
        }

        vec![]
    }

    fn move_up(&mut self, buffer: &Buffer) -> Vec<AppEvent> {
        if self.position.line > 0 {
            self.position.line -= 1;
            self.position.col = buffer.clamp_col_position(&self.position);
        }

        vec![]
    }

    fn move_down(&mut self, buffer: &Buffer) -> Vec<AppEvent> {
        let total_lines = buffer.len_lines();

        if self.position.line + 1 < total_lines {
            self.position.line += 1;
            self.position.col = buffer.clamp_col_position(&self.position);
        }

        vec![]
    }

    fn move_to_line_start(&mut self) -> Vec<AppEvent> {
        self.position.col = 0;

        vec![]
    }

    fn move_to_line_end(&mut self, buffer: &Buffer) -> Vec<AppEvent> {
        self.position.col = buffer.max_visible_col(&self.position);

        vec![]
    }

    fn set_position(&mut self, line: usize, col: usize, buffer: &Buffer) -> Vec<AppEvent> {
        self.set_line_position(line, buffer);
        self.set_col_position(col, buffer);

        vec![]
    }

    fn set_line_position(&mut self, line: usize, buffer: &Buffer) -> Vec<AppEvent> {
        let total_lines = buffer.len_lines().saturating_sub(1);

        self.position.line = line.min(total_lines);
        self.position.col = buffer.clamp_col_position(&self.position);

        vec![]
    }

    fn set_col_position(&mut self, col: usize, buffer: &Buffer) -> Vec<AppEvent> {
        let max_col = buffer.max_visible_col(&self.position);
        self.position.col = col.min(max_col);

        vec![]
    }

    fn set_cursor_style(&self, current_mode: EditorMode) -> SetCursorStyle {
        match current_mode {
            EditorMode::Insert { .. } => cursor::SetCursorStyle::SteadyBar,
            _ => cursor::SetCursorStyle::SteadyBlock,
        }
    }

    /// Calculates the actual terminal coordinates where the cursor should appear.
    fn calculate_cursor_position(&self, area: Rect) -> ratatui::layout::Position {
        let text_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let clamped_line = self
            .position
            .line
            .min(text_area.height.saturating_sub(1) as usize);
        let clamped_col = self
            .position
            .col
            .min(text_area.width.saturating_sub(1) as usize);

        ratatui::layout::Position {
            x: text_area.x + clamped_col as u16,
            y: text_area.y + clamped_line as u16,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::buffer::Buffer;

    fn buffer_with_lines(lines: &[&str]) -> Buffer {
        let joined = lines.join("\n");
        Buffer::new(joined)
    }

    #[test]
    fn should_move_right_until_end_of_line() {
        let buffer = buffer_with_lines(&["abc"]);
        let mut cursor = Cursor::new();

        cursor.handle_event(CursorEvent::MoveRight, &buffer);
        assert_eq!(cursor.position.col, 1);

        cursor.handle_event(CursorEvent::MoveRight, &buffer);
        assert_eq!(cursor.position.col, 2);

        cursor.handle_event(CursorEvent::MoveRight, &buffer);
        assert_eq!(cursor.position.col, 3);

        cursor.handle_event(CursorEvent::MoveRight, &buffer);
        assert_eq!(cursor.position.col, 3, "Should not move past end of line");
    }

    #[test]
    fn should_move_left_and_stop_at_start() {
        let buffer = buffer_with_lines(&["abc"]);
        let mut cursor = Cursor::new();
        cursor.position.col = 2;

        cursor.handle_event(CursorEvent::MoveLeft, &buffer);
        assert_eq!(cursor.position.col, 1);

        cursor.handle_event(CursorEvent::MoveLeft, &buffer);
        assert_eq!(cursor.position.col, 0);

        cursor.handle_event(CursorEvent::MoveLeft, &buffer);
        assert_eq!(cursor.position.col, 0, "Should not go below column 0");
    }

    #[test]
    fn should_move_down_and_up_across_lines_and_clamp_column() {
        let buffer = buffer_with_lines(&["abc", "d", "efg"]);
        let mut cursor = Cursor::new();
        cursor.position.col = 2;

        cursor.handle_event(CursorEvent::MoveDown, &buffer);
        assert_eq!(cursor.position.line, 1);
        assert_eq!(cursor.position.col, 1, "Should clamp to line 1's max col");

        cursor.handle_event(CursorEvent::MoveDown, &buffer);
        assert_eq!(cursor.position.line, 2);
        assert_eq!(cursor.position.col, 1);

        cursor.handle_event(CursorEvent::MoveUp, &buffer);
        assert_eq!(cursor.position.line, 1);
        assert_eq!(cursor.position.col, 1);

        cursor.handle_event(CursorEvent::MoveUp, &buffer);
        assert_eq!(cursor.position.line, 0);
        assert_eq!(cursor.position.col, 1);
    }

    #[test]
    fn should_move_to_line_start_and_end() {
        let buffer = buffer_with_lines(&["abcd"]);
        let mut cursor = Cursor::new();
        cursor.position.col = 2;

        cursor.handle_event(CursorEvent::MoveToLineStart, &buffer);
        assert_eq!(cursor.position.col, 0);

        cursor.handle_event(CursorEvent::MoveToLineEnd, &buffer);
        assert_eq!(cursor.position.col, 4);
    }

    #[test]
    fn should_set_position_within_bounds_and_clamp_if_needed() {
        let buffer = buffer_with_lines(&["123", "4567"]);
        let mut cursor = Cursor::new();

        cursor.handle_event(CursorEvent::SetPosition { line: 5, col: 99 }, &buffer);
        assert_eq!(cursor.position.line, 1);
        assert_eq!(cursor.position.col, 4);
    }
}
