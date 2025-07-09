use crate::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

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
    }
}
