use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("zack")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let mut text = String::new();
        for line in self.buffer.lines() {
            text.push_str(&line.to_string());
        }

        let cursor_info = format!("\n\n-- ({}, {}) --", self.cursor.0 + 1, self.cursor.1 + 1);
        text.push_str(&cursor_info);

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .alignment(Alignment::Left);

        paragraph.render(area, buf);
    }
}
