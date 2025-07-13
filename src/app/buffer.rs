//! Buffer management module for the Zack text editor.
//!
//! This module defines the [`Buffer`] struct, which wraps a [`Rope`] from the
//! [`ropey`](https://docs.rs/ropey) crate to efficiently manage and manipulate
//! large and mutable text content. It supports inserting and deleting characters
//! or lines, querying line and column bounds, and converting logical positions
//! to rope indices.
//!
//! It also defines [`BufferEvent`], an enum representing edit operations that
//! can be dispatched to the buffer. These events are translated into
//! [`AppEvent`]s to propagate changes and trigger UI or cursor updates.

use crate::{
    event::{AppEvent, CursorEvent},
    types::position::Position,
};
use ropey::{Rope, iter::Lines};

/// Represents the main text buffer for editing, backed by a `Rope` for efficient operations.
#[derive(Debug)]
pub struct Buffer {
    rope: Rope,
}

/// Describes high-level buffer modification events.
#[derive(Clone, Debug)]
pub enum BufferEvent {
    /// Inserts a character at a given position.
    InsertChar { char: char, position: Position },

    /// Deletes a character at a given position.
    DeleteChar { position: Position },

    /// Inserts a new line at a given position.
    InsertNewline { position: Position },
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new(String::from(""))
    }
}

impl Buffer {
    /// Creates a new buffer with the given initial text content.
    pub fn new(initial_text: String) -> Self {
        Self {
            rope: Rope::from_str(&initial_text),
        }
    }

    /// Handles a `BufferEvent` and returns the resulting `AppEvent`s.
    pub fn handle_event(&mut self, event: BufferEvent) -> Vec<AppEvent> {
        let mut events = vec![];

        match event {
            BufferEvent::InsertChar { char, position } => {
                events.extend(self.insert_char(char, position))
            }
            BufferEvent::DeleteChar { position } => events.extend(self.delete_char(position)),
            BufferEvent::InsertNewline { position } => {
                events.extend(self.insert_new_line(position))
            }
        }

        events
    }

    /// Returns the total number of lines in the buffer.
    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    /// Clamps a column position to the visible max column of the given line.
    pub fn clamp_col_position(&self, position: &Position) -> usize {
        position.col.min(self.max_visible_col(position))
    }

    /// Returns the maximum visible column in a given line, accounting for trailing newline.
    pub fn max_visible_col(&self, position: &Position) -> usize {
        let rope_line = self.rope.line(position.line);
        let len = rope_line.len_chars();

        if len == 0 {
            return 0;
        }

        let last_char = rope_line.char(len.saturating_sub(1));

        if last_char == '\n' { len - 1 } else { len }
    }

    /// Returns a `Lines` iterator over the rope buffer.
    pub fn lines(&self) -> Lines<'_> {
        self.rope.lines()
    }

    /// Returns a reference to the internal rope structure.
    pub fn as_rope(&self) -> &Rope {
        &self.rope
    }

    /// Calculates the character index in the rope from a `Position`.
    fn calculate_char_index(&self, position: Position) -> usize {
        let line_start = self.rope.line_to_char(position.line);
        let line_len = self.rope.line(position.line).len_chars();

        let clamped_col = position.col.min(line_len);

        line_start + clamped_col
    }

    /// Inserts a character at the given position and emits a cursor move.
    fn insert_char(&mut self, char: char, position: Position) -> Vec<AppEvent> {
        let char_index = self.calculate_char_index(position);
        self.rope.insert_char(char_index, char);

        vec![AppEvent::Cursor(CursorEvent::MoveRight)]
    }

    /// Deletes a character at the given position and emits appropriate cursor events.
    fn delete_char(&mut self, position: Position) -> Vec<AppEvent> {
        let mut events = vec![];

        if position.col == 0 && position.line > 0 {
            self.merge_with_line_above(position, &mut events);
        } else if position.col > 0 {
            let char_index = self.calculate_char_index(position);
            self.rope.remove(char_index - 1..char_index);

            events.push(AppEvent::Cursor(CursorEvent::MoveLeft));
        }

        events
    }

    /// Merges the current line with the previous one (when deleting at column 0).
    fn merge_with_line_above(&mut self, position: Position, events: &mut Vec<AppEvent>) {
        let prev_line_len = self.rope.line(position.line - 1).len_chars();

        let char_index = self.calculate_char_index(Position::new(position.line, 0));

        if char_index > 0 {
            self.rope.remove(char_index - 1..char_index);

            events.push(AppEvent::Cursor(CursorEvent::SetLinePosition(
                position.line - 1,
            )));
            events.push(AppEvent::Cursor(CursorEvent::SetColPosition(
                prev_line_len - 1,
            )));
        }
    }

    /// Inserts a newline character at the given position and emits appropriate cursor movement.
    fn insert_new_line(&mut self, position: Position) -> Vec<AppEvent> {
        let mut events = vec![];

        let char_index = self.calculate_char_index(position);

        self.rope.insert(char_index, "\n");

        events.push(AppEvent::Cursor(CursorEvent::MoveDown));
        events.push(AppEvent::Cursor(CursorEvent::MoveToLineStart));

        events
    }
}

