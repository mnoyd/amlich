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
        let width = 32;
        let height = 9;
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let popup_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(popup_area, buf);

        let is_valid = self.app.date_jump_is_valid();
        let border_style = if is_valid {
            Style::default().fg(theme::ACCENT_FG)
        } else {
            Style::default().fg(theme::SECONDARY_FG)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(Line::from(vec![Span::styled(
                " Jump to Date ",
                theme::section_style(),
            )]))
            .title_bottom(Line::from(" Enter │ Esc ").alignment(Alignment::Center));

        // Instructions
        let instructions = Line::from(vec![
            Span::styled("Type: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled("ddmmyyyy", Style::default().fg(theme::ACCENT_FG)),
            Span::raw(" (auto-formats)"),
        ]);

        // Input field with cursor
        let input_text = format!("{}_", self.app.date_jump_input);
        let input_style = if is_valid {
            Style::default()
                .fg(theme::PRIMARY_FG)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::PRIMARY_FG)
        };

        let input_line = Line::from(vec![Span::styled(&input_text, input_style)]);

        // Validation indicator
        let valid_line = if self.app.date_jump_input.len() >= 8 {
            if is_valid {
                Line::from(vec![
                    Span::styled(" ✓ ", Style::default().fg(theme::ACCENT_FG)),
                    Span::styled("Valid date", Style::default().fg(theme::ACCENT_FG)),
                ])
            } else {
                Line::from(vec![
                    Span::styled(" ✗ ", Style::default().fg(theme::HOLIDAY_FG)),
                    Span::styled("Invalid date", Style::default().fg(theme::HOLIDAY_FG)),
                ])
            }
        } else {
            Line::from("")
        };

        let p = Paragraph::new(vec![
            instructions,
            Line::from(""),
            input_line,
            Line::from(""),
            valid_line,
        ])
        .block(block)
        .alignment(Alignment::Center);

        p.render(popup_area, buf);
    }
}
