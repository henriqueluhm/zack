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
    pub input: String,
}

impl Default for FilenamePrompt {
    fn default() -> Self {
        Self::new()
    }
}

impl FilenamePrompt {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

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
