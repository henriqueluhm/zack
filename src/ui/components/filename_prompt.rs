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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::AppEvent;
    use crate::ui::components::FocusableComponent;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn should_append_char_to_input_on_char_key() {
        let mut prompt = FilenamePrompt::new();

        prompt.handle_key(key(KeyCode::Char('a')));
        prompt.handle_key(key(KeyCode::Char('b')));

        assert_eq!(prompt.input, "ab");
    }

    #[test]
    fn should_remove_last_char_on_backspace() {
        let mut prompt = FilenamePrompt::new();

        prompt.input = String::from("abc");
        prompt.handle_key(key(KeyCode::Backspace));

        assert_eq!(prompt.input, "ab");
    }

    #[test]
    fn should_do_nothing_on_backspace_when_input_empty() {
        let mut prompt = FilenamePrompt::new();

        prompt.input = String::new();
        prompt.handle_key(key(KeyCode::Backspace));

        assert_eq!(prompt.input, "");
    }

    #[test]
    fn should_clear_input_and_change_focus_on_esc() {
        let mut prompt = FilenamePrompt::new();

        prompt.input = String::from("filename.txt");

        let events = prompt.handle_key(key(KeyCode::Esc));

        assert!(events.contains(&AppEvent::ChangeFocus(FocusableComponent::Editor)));
        assert_eq!(prompt.input, "");
    }

    #[test]
    fn should_emit_saveas_and_clear_input_on_enter_with_non_empty_input() {
        let mut prompt = FilenamePrompt::new();

        prompt.input = String::from("file.txt");

        let events = prompt.handle_key(key(KeyCode::Enter));

        assert_eq!(prompt.input, "");
        assert!(matches!(
            events.iter().find(|e| matches!(e, AppEvent::File(_))),
            Some(AppEvent::File(_))
        ));
        assert!(events.contains(&AppEvent::ChangeFocus(FocusableComponent::Editor)));
    }

    #[test]
    fn should_do_nothing_on_enter_with_empty_input() {
        let mut prompt = FilenamePrompt::new();

        prompt.input = String::new();

        let events = prompt.handle_key(key(KeyCode::Enter));

        assert!(events.is_empty());
        assert_eq!(prompt.input, "");
    }

    #[test]
    fn should_do_nothing_on_other_keys() {
        let mut prompt = FilenamePrompt::new();

        let events = prompt.handle_key(key(KeyCode::Tab));

        assert!(events.is_empty());
    }
}
