use crate::{
    app::{
        App,
        modes::{insert::InsertMode, normal::NormalMode, visual::VisualMode},
    },
    event::AppEvent,
    types::position::Position,
};
use crossterm::event::KeyEvent;
use std::fmt::Debug;

pub mod insert;
pub mod normal;
pub mod visual;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorMode {
    Insert,
    Normal,
    Visual,
}

pub trait Mode: Debug {
    fn get_current_mode(&self) -> EditorMode;
    fn get_mode_label(&self) -> &'static str;
    fn handle_key(&self, key: KeyEvent, current_cursor_position: Position) -> Vec<AppEvent>;
}

pub fn change_mode(new_mode: EditorMode, app: &mut App) {
    match new_mode {
        EditorMode::Insert => app.mode = Box::new(InsertMode),
        EditorMode::Normal => app.mode = Box::new(NormalMode),
        EditorMode::Visual => app.mode = Box::new(VisualMode),
    }
}
