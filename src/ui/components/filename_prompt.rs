//! Filename prompt component rendering logic.
//!
//! This module defines the `FilenamePrompt` component. This component is shown when the user is prompted to enter a file name
//! to save the current buffer. It handles basic input editing and renders
//! the UI prompt at the bottom of the terminal window.

use crate::event::AppEvent;
use crate::ui::components::FocusableComponent;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::buffer::Buffer;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};
use std::path::PathBuf;

#[derive(Debug)]
pub struct FilenamePrompt {
    /// The current input from the user for the file name.
    pub input: String,
}

impl Default for FilenamePrompt {
    fn default() -> Self {
        Self::new()
    }
}

impl FilenamePrompt {
    /// Creates a new `FilenamePrompt` with an empty input.
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    /// Handles a key event while the prompt is active.
    ///
    /// Returns a vector of `AppEvent`s that may trigger further actions:
    /// - `Esc`: Clears the input and returns focus to the editor.
    /// - `Enter`: If input is not empty, emits a `SaveAs` event and returns focus.
    /// - `Backspace`: Removes the last character in the input.
    /// - Character keys: Appends the character to the input.
    ///
    /// Other keys are ignored.
    pub fn handle_key(&mut self, key: KeyEvent) -> Vec<AppEvent> {
        match key.code {
            KeyCode::Esc => {
                self.input.clear();
                vec![AppEvent::ChangeFocus(FocusableComponent::Editor)]
            }
            KeyCode::Enter => {
                if self.input.is_empty() {
                    vec![]
                } else {
                    let path = PathBuf::from(&self.input);
                    self.input.clear();
                    vec![
                        AppEvent::File(crate::app::file::FileEvent::SaveAs(path)),
                        AppEvent::ChangeFocus(FocusableComponent::Editor),
                    ]
                }
            }
            KeyCode::Backspace => {
                self.input.pop();
                vec![]
            }
            KeyCode::Char(c) => {
                self.input.push(c);
                vec![]
            }
            _ => vec![],
        }
    }

    /// Renders the filename prompt at the bottom of the screen.
    ///
    /// This includes a block with borders and a "Save as:" label followed
    /// by the current user input.
    ///
    /// The prompt is drawn 3 lines from the bottom, inset by 2 columns on each side.
    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let input = format!("Save as: {}", self.input);
        let prompt = Paragraph::new(input)
            .block(
                Block::default()
                    .border_type(BorderType::Plain)
                    .title("Filename")
                    .title_alignment(Alignment::Left),
            )
            .fg(Color::Yellow)
            .bg(Color::Black)
            .alignment(Alignment::Left);

        let area = Rect {
            x: 2,
            y: area.height.saturating_sub(3),
            width: area.width.saturating_sub(4),
            height: 3,
        };

        prompt.render(area, buf);
    }
}
