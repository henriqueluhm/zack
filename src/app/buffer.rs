use crate::{app::cursor::CursorEvent, event::AppEvent, types::position::Position};
use ropey::{Rope, iter::Lines};

#[derive(Debug)]
pub struct Buffer {
    rope: Rope,
}

#[derive(Clone, Debug)]
pub enum BufferEvent {
    InsertChar { char: char, position: Position },
    DeleteChar { position: Position },
    InsertNewline { position: Position },
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::from_str("Hello, Zack!"),
        }
    }

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

    pub fn len_lines(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn clamp_col_position(&self, position: &Position) -> usize {
        position.col.min(self.max_visible_col(position))
    }

    pub fn max_visible_col(&self, position: &Position) -> usize {
        let rope_line = self.rope.line(position.line);

        let len = rope_line.len_chars();

        if len == 0 {
            return 0;
        }

        let last_char = rope_line.char(len.saturating_sub(1));

        if last_char == '\n' { len - 1 } else { len }
    }

    pub fn lines(&self) -> Lines<'_> {
        self.rope.lines()
    }

    pub fn as_rope(&self) -> &Rope {
        &self.rope
    }

    fn insert_char(&mut self, char: char, position: Position) -> Vec<AppEvent> {
        let char_index = self.calculate_char_index(position);
        self.rope.insert_char(char_index, char);

        vec![AppEvent::Cursor(CursorEvent::MoveRight)]
    }

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

    fn insert_new_line(&mut self, position: Position) -> Vec<AppEvent> {
        let mut events = vec![];

        let char_index = self.calculate_char_index(position);

        self.rope.insert(char_index, "\n");

        events.push(AppEvent::Cursor(CursorEvent::MoveDown));
        events.push(AppEvent::Cursor(CursorEvent::MoveToLineStart));

        events
    }

    fn calculate_char_index(&self, position: Position) -> usize {
        let line_start = self.rope.line_to_char(position.line);
        let line_len = self.rope.line(position.line).len_chars();

        let clamped_col = position.col.min(line_len);

        line_start + clamped_col
    }
}
