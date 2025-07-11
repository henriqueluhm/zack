use super::Mode;
use crate::event::AppEvent;
use crate::{app::modes::EditorMode, types::position::Position};
use crossterm::event::{KeyCode, KeyEvent};

#[derive(Debug)]
pub struct VisualMode;

impl Mode for VisualMode {
    fn get_mode_label(&self) -> &'static str {
        "visual"
    }

    fn get_current_mode(&self) -> EditorMode {
        EditorMode::Visual
    }

    fn handle_key(&self, key: KeyEvent, _: Position) -> Vec<AppEvent> {
        let mut events = vec![];

        if key.code == KeyCode::Esc {
            events.push(AppEvent::ChangeToMode(EditorMode::Normal))
        }

        events
    }
}
