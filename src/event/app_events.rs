//! Defines high-level application events used to coordinate editor behavior.
//!
//! This module exports the [`AppEvent`] enum, which wraps lower-level domain-specific
//! events (like [`BufferEvent`], [`CursorEvent`], and [`FileEvent`]) and higher-level
//! app signals.

use crate::{app::modes::EditorMode, ui::components::FocusableComponent};

/// Re-exports of domain-specific event types.
pub use crate::app::buffer::BufferEvent;
pub use crate::app::cursor::CursorEvent;
pub use crate::app::file::FileEvent;

/// Represents a high-level application event.
///
/// Used in the event loop or message-passing system to drive state transitions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AppEvent {
    /// Event related to buffer changes (editing, content modification).
    Buffer(BufferEvent),
    /// Event related to cursor movement or visibility.
    Cursor(CursorEvent),
    /// Event for file operations like save/load.
    File(FileEvent),
    /// Change focus to a specific UI component.
    ChangeFocus(FocusableComponent),
    /// Switch to a different editor mode (Insert, Normal, etc.).
    ChangeToMode(EditorMode),
    /// Signal to quit the application.
    Quit,
}
