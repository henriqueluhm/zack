use crate::{app::buffer::Buffer, event::AppEvent, ui::components::FocusableComponent};
use std::path::PathBuf;

#[derive(Debug)]
pub struct File {
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileEvent {
    Save,
    SaveAs(PathBuf),
}

impl Default for File {
    fn default() -> Self {
        Self::new(None)
    }
}

impl File {
    pub fn new(path: Option<PathBuf>) -> Self {
        Self { path }
    }

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

    fn save_file(&self, buffer: &Buffer) -> Vec<AppEvent> {
        match &self.path {
            Some(path) => match self.write_to_file(path, buffer) {
                Ok(_) => {
                    vec![]
                }
                Err(err) => {
                    eprintln!("Failed to save file: {}", err);
                    vec![]
                }
            },

            None => vec![AppEvent::ChangeFocus(FocusableComponent::FilenamePrompt)],
        }
    }

    fn write_to_file(&self, path: &PathBuf, buffer: &Buffer) -> std::io::Result<()> {
        let mut content = String::new();
        for line in buffer.lines() {
            content.push_str(&line.to_string());
        }

        std::fs::write(path, content)
    }
}
