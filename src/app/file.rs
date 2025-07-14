//! File management module for the Zack text editor.
//!
//! This module provides the [`File`] struct responsible for managing the
//! file path and saving the buffer content to disk. It defines
//! [`FileEvent`] variants to represent save operations, including saving
//! to the current path or saving as a new file.
//!
//! # Overview
//!
//! - `File` holds an optional path to the current file being edited.
//! - Handles events to save the buffer content to disk.
//! - If no path is set, requests focus change to the filename prompt UI.
//!
//! # Usage
//!
//! Create a `File` instance with an optional path, call `handle_event`
//! with save events to persist buffer content.

use crate::{app::buffer::Buffer, event::AppEvent, ui::components::FocusableComponent};
use std::path::PathBuf;

#[derive(Debug)]
/// Represents the currently loaded file in the editor.
pub struct File {
    /// Optional path to the file on disk.
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Events related to file operations.
pub enum FileEvent {
    /// Save the file to the existing path.
    Save,
    /// Save the file to a new specified path.
    SaveAs(PathBuf),
}

impl Default for File {
    /// Creates a new `File` with no path set.
    fn default() -> Self {
        Self::new(None)
    }
}

impl File {
    /// Creates a new `File` instance with an optional path.
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path }
    }

    /// Handles a file-related event and returns resulting app events.
    pub fn handle_event(&mut self, event: FileEvent, buffer: &Buffer) -> Vec<AppEvent> {
        let mut events = vec![];

        match event {
            FileEvent::Save => events.extend(self.save_file(buffer)),
            FileEvent::SaveAs(path) => {
                self.path = Some(path);

                events.extend(self.save_file(buffer))
            }
        }

        events
    }

    /// Saves the buffer content to the current file path if set.
    ///
    /// If no path is set, requests focus change to the filename prompt.
    fn save_file(&self, buffer: &Buffer) -> Vec<AppEvent> {
        match &self.path {
            Some(path) => match self.write_to_file(path, buffer) {
                Ok(_) => vec![],
                Err(err) => {
                    eprintln!("Failed to save file: {}", err);
                    vec![]
                }
            },

            None => vec![AppEvent::ChangeFocus(FocusableComponent::FilenamePrompt)],
        }
    }

    /// Writes the buffer content to disk at the specified path.
    /// # Errors
    ///
    /// Returns an `std::io::Error` if the write operation fails.
    fn write_to_file(&self, path: &PathBuf, buffer: &Buffer) -> std::io::Result<()> {
        let mut content = String::new();
        for line in buffer.lines() {
            content.push_str(&line.to_string());
        }

        std::fs::write(path, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::buffer::Buffer;
    use std::path::PathBuf;

    fn create_buffer_with_text(text: &str) -> Buffer {
        Buffer::new(text.to_string())
    }

    #[test]
    fn should_create_file_with_none_path() {
        let file = File::default();

        assert!(file.path.is_none());
    }

    #[test]
    fn should_create_file_with_some_path() {
        let path = PathBuf::from("test.txt");
        let file = File::new(Some(path.clone()));

        assert_eq!(file.path, Some(path));
    }

    #[test]
    fn should_return_empty_events_when_saving_to_valid_path() {
        let path = PathBuf::from("test_save.txt");
        let mut file = File::new(Some(path.clone()));
        let buffer = create_buffer_with_text("Hello, Zack!");

        let _ = std::fs::remove_file(&path);

        let events = file.handle_event(FileEvent::Save, &buffer);

        assert!(events.is_empty());

        let saved_content = std::fs::read_to_string(&path).expect("File should exist");

        assert_eq!(saved_content, "Hello, Zack!");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn should_update_path_and_save_on_save_as() {
        let path = PathBuf::from("test_save_as.txt");
        let mut file = File::default();
        let buffer = create_buffer_with_text("New content");

        let _ = std::fs::remove_file(&path);

        let events = file.handle_event(FileEvent::SaveAs(path.clone()), &buffer);

        assert!(events.is_empty());
        assert_eq!(file.path, Some(path.clone()));

        let saved_content = std::fs::read_to_string(&path).expect("File should exist");

        assert_eq!(saved_content, "New content");

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn should_request_focus_change_when_saving_without_path() {
        let mut file = File::default();
        let buffer = create_buffer_with_text("Some text");

        let events = file.handle_event(FileEvent::Save, &buffer);

        assert_eq!(
            events,
            vec![AppEvent::ChangeFocus(FocusableComponent::FilenamePrompt)]
        );
    }
}
