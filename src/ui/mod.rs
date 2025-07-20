//! UI module for the Zack text editor.
//!
//! This module defines the global UI rendering logic and exports submodules like components.
//! It implements the [`Widget`] trait for the [`App`] struct, delegating rendering to appropriate components
//! based on application state.

use crate::{
    app::App,
    ui::components::{FocusableComponent, editor::Editor},
};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

/// UI components such as `Editor`, `FilenamePrompt`, etc.
pub mod components;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Editor::render(self, area, buf);

        if self.focus == FocusableComponent::FilenamePrompt {
            self.filename_prompt.render(area, buf);
        }
    }
}

