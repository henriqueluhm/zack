use crate::{
    app::App,
    ui::components::{FocusableComponent, editor::Editor},
};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

pub mod components;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Editor::render(self, area, buf);

        if self.focus == FocusableComponent::FilenamePrompt {
            self.filename_prompt.render(area, buf);
        }
    }
}
