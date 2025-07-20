//! UI components for the Zack text editor.
//!
//! This module exposes individual UI components, like the main editor and the filename prompt.
//! It also defines [`FocusableComponent`], an enum used by the application state to determine
//! which component is currently focused.

/// The editor component responsible for text editing.
pub mod editor;

/// The filename prompt component, used for saving or naming files.
pub mod filename_prompt;

/// Represents which component in the UI currently has focus.
/// Used by the main [`App`](crate::app::App) state to direct user input and rendering.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FocusableComponent {
    Editor,
    FilenamePrompt,
}
