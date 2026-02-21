use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::app::App;
use crate::theme;

pub struct DateJumpPopup<'a> {
    app: &'a App,
}

impl<'a> DateJumpPopup<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for DateJumpPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the popup
        let width = 30;
        let height = 8;
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let popup_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(popup_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::ACCENT_FG))
            .title(Line::from(vec![Span::styled(
                " 📅 Jump to Date ",
                theme::title_style(),
            )]))
            .title_bottom(Line::from(" Enter │ Esc ").alignment(Alignment::Center));

        // Instructions
        let instructions = Line::from(vec![
            Span::styled("Format: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled("dd/mm/yyyy", Style::default().fg(theme::ACCENT_FG)),
        ]);

        // Input field with cursor
        let input_text = format!("{}_", self.app.date_jump_input);
        let input_line = Line::from(vec![
            Span::styled(
                &input_text,
                Style::default()
                    .fg(theme::VALUE_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        let p = Paragraph::new(vec![instructions, Line::from(""), input_line])
            .block(block)
            .alignment(Alignment::Center);

        p.render(popup_area, buf);
    }
}
