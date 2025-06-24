use crate::app::{App, modes::EditorMode};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let editor_mode = match self.mode {
            EditorMode::Normal => "Normal",
            EditorMode::Insert => "Insert",
        };

        let block = Block::bordered()
            .title("zack")
            .title_alignment(Alignment::Center)
            .title_bottom(editor_mode)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let mut text = String::new();
        for line in self.buffer.lines() {
            text.push_str(&line.to_string());
        }

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::Cyan)
            .bg(Color::Black)
            .alignment(Alignment::Left);

        paragraph.render(area, buf);
    }
}
