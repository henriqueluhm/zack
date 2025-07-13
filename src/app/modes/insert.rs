use super::Mode;
use crate::event::{AppEvent, BufferEvent};
use crate::types::position::Position;
use crate::{app::modes::EditorMode, event::CursorEvent};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub struct InsertMode;

impl Mode for InsertMode {
    fn get_mode_label(&self) -> &'static str {
        "insert"
    }

    fn get_current_mode(&self) -> EditorMode {
        EditorMode::Insert
    }

    fn handle_key(&self, key: KeyEvent, current_cursor_position: Position) -> Vec<AppEvent> {
        let mut events = vec![];

        match key.code {
            KeyCode::Esc => {
                events.push(AppEvent::Cursor(CursorEvent::MoveLeft));
                events.push(AppEvent::ChangeToMode(EditorMode::Normal));
            }
            KeyCode::Left => events.push(AppEvent::Cursor(CursorEvent::MoveLeft)),
            KeyCode::Right => events.push(AppEvent::Cursor(CursorEvent::MoveRight)),
            KeyCode::Up => events.push(AppEvent::Cursor(CursorEvent::MoveUp)),
            KeyCode::Down => events.push(AppEvent::Cursor(CursorEvent::MoveDown)),
            KeyCode::Backspace => events.push(AppEvent::Buffer(BufferEvent::DeleteChar {
                position: current_cursor_position,
            })),
            KeyCode::Char(char) => events.push(AppEvent::Buffer(BufferEvent::InsertChar {
                char,
                position: current_cursor_position,
            })),
            KeyCode::Enter => events.push(AppEvent::Buffer(BufferEvent::InsertNewline {
                position: current_cursor_position,
            })),
            _ => {}
        }

        events
    }
}
