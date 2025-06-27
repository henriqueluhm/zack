use super::Mode;
use crate::app::modes::EditorMode;
use crate::event::AppEvent;
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

    fn handle_key(&self, key: KeyEvent) -> Vec<AppEvent> {
        let mut events = vec![];

        match key.code {
            KeyCode::Esc => events.push(AppEvent::ChangeToMode(EditorMode::Normal)),
            KeyCode::Backspace => events.push(AppEvent::DeleteChar),
            KeyCode::Left => events.push(AppEvent::MoveCursorLeft),
            KeyCode::Right => events.push(AppEvent::MoveCursorRight),
            KeyCode::Up => events.push(AppEvent::MoveCursorUp),
            KeyCode::Down => events.push(AppEvent::MoveCursorDown),
            KeyCode::Char(c) => events.push(AppEvent::InsertChar(c)),
            _ => {}
        }

        events
    }
}
