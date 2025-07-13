use std::path::PathBuf;

use crate::event::AppEvent;

#[derive(Debug)]
pub struct File {
    pub path: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub enum FileEvent {
    Save,
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

    pub fn handle_event(&mut self, event: FileEvent) -> Vec<AppEvent> {
        let mut events = vec![];

        match event {
            FileEvent::Save => events.extend(self.save_file()),
        }

        events
    }

    fn save_file(&self) -> Vec<AppEvent> {
        vec![]
    }
}
