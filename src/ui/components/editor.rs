//! Editor component rendering logic.
//!
//! This module defines the `Editor` component, responsible for rendering the main text
//! editing area of the Zack text editor. It displays the contents of the buffer,
//! including the current mode and styling.

use crate::app::App;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph, Widget},
};

/// The `Editor` component responsible for rendering the editable text area.
pub struct Editor;

impl Editor {
    /// Renders the editor component onto the provided area of the screen.
    ///
    /// # Arguments
    ///
    /// - `app`: The current application state (provides the buffer and mode).
    /// - `area`: The screen region to render into.
    /// - `buf`: The terminal buffer to draw on.
    pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title("zack")
            .title_alignment(Alignment::Center)
            .title_bottom(app.mode.get_mode_label())
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let mut text = String::new();
        for line in app.buffer.lines() {
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
