use super::Mode;
use crate::app::modes::EditorMode;
use crate::event::AppEvent;
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

    fn handle_key(&self, key: KeyEvent) -> Vec<AppEvent> {
        let mut events = vec![];

        if key.code == KeyCode::Esc {
            events.push(AppEvent::ChangeToMode(EditorMode::Normal))
        }

        events
    }
}
