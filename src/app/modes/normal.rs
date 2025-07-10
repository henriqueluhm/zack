use super::Mode;
use crate::app::{cursor::CursorEvent, modes::EditorMode};
use crate::event::AppEvent;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub struct NormalMode;

impl Mode for NormalMode {
    fn get_mode_label(&self) -> &'static str {
        "normal"
    }

    fn get_current_mode(&self) -> EditorMode {
        EditorMode::Normal
    }

    fn handle_key(&self, key: KeyEvent) -> Vec<AppEvent> {
        let mut events = vec![];

        match key.code {
            KeyCode::Char('v') => events.push(AppEvent::ChangeToMode(EditorMode::Visual)),
            KeyCode::Char('i') => {
                events.push(AppEvent::ChangeToMode(EditorMode::Insert { append: false }))
            }
            KeyCode::Char('a') => {
                events.push(AppEvent::Cursor(CursorEvent::MoveRight));
                events.push(AppEvent::ChangeToMode(EditorMode::Insert { append: true }));
            }
            KeyCode::Char('h') => events.push(AppEvent::Cursor(CursorEvent::MoveLeft)),
            KeyCode::Char('l') => events.push(AppEvent::Cursor(CursorEvent::MoveRight)),
            KeyCode::Char('j') => events.push(AppEvent::Cursor(CursorEvent::MoveDown)),
            KeyCode::Char('k') => events.push(AppEvent::Cursor(CursorEvent::MoveUp)),
            KeyCode::Char('q') | KeyCode::Esc => events.push(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                events.push(AppEvent::Quit);
            }

            _ => {}
        }

        events
    }
}
