use crate::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Component {
    Editor,
    FilenamePrompt,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("zack")
            .title_alignment(Alignment::Center)
            .title_bottom(self.mode.get_mode_label())
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let mut text = String::new();
        for line in self.buffer.lines() {
            text.push_str(&line.to_string());
        }

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Gray)
            .bg(Color::Reset)
            .alignment(Alignment::Left);

        paragraph.render(area, buf);

        if self.focus == Component::FilenamePrompt {
            let input = format!("Save as: {}", self.filename_input);
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
}
