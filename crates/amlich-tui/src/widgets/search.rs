use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::app::App;
use crate::theme;

pub struct SearchPopup<'a> {
    app: &'a App,
}

impl<'a> SearchPopup<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for SearchPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the popup
        let width = 50;
        let height = if self.app.search_results.is_empty() { 6 } else { 12 };
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let popup_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(popup_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::ACCENT_FG))
            .title(Line::from(vec![Span::styled(
                " 🔍 Search Holidays/Festivals ",
                theme::title_style(),
            )]))
            .title_bottom(Line::from(" Tab │ Esc ").alignment(Alignment::Center));

        // Build lines with owned strings
        let mut lines: Vec<Line> = Vec::new();

        // Input field with cursor
        let input_text = format!("{}_", self.app.search_query);
        lines.push(Line::from(vec![
            Span::styled("Search: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled(input_text, Style::default().fg(theme::VALUE_FG).add_modifier(Modifier::BOLD)),
        ]));

        lines.push(Line::from(""));

        // Results
        if self.app.search_results.is_empty() {
            if self.app.search_query.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled("Type to search for holidays", Style::default().fg(theme::LUNAR_FG)),
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("No results found", Style::default().fg(theme::LABEL_FG)),
                ]));
            }
        } else {
            let count = self.app.search_results.len();
            let current = self.app.search_index + 1;
            lines.push(Line::from(vec![
                Span::styled(
                    format!("Results: {}/{}", current, count),
                    Style::default().fg(theme::ACCENT_FG),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled(
                    "Press Tab to cycle results",
                    Style::default().fg(theme::LUNAR_FG),
                ),
            ]));

            lines.push(Line::from(""));

            // Show current result
            if let Some(entry) = self.app.search_results.get(self.app.search_index) {
                lines.push(Line::from(vec![
                    Span::styled("Current: ", Style::default().fg(theme::LABEL_FG)),
                    Span::styled(
                        format!("{:02}/{:02}/{}", entry.day, entry.month, entry.year),
                        Style::default().fg(theme::VALUE_FG).add_modifier(Modifier::BOLD),
                    ),
                ]));
            }

            // Show adjacent results
            if self.app.search_index > 0 {
                if let Some(entry) = self.app.search_results.get(self.app.search_index - 1) {
                    lines.push(Line::from(vec![
                        Span::styled("  ↑ ", Style::default().fg(theme::LABEL_FG)),
                        Span::styled(
                            format!("{:02}/{:02}/{}", entry.day, entry.month, entry.year),
                            Style::default().fg(theme::LUNAR_FG),
                        ),
                    ]));
                }
            }

            if self.app.search_index + 1 < self.app.search_results.len() {
                if let Some(entry) = self.app.search_results.get(self.app.search_index + 1) {
                    lines.push(Line::from(vec![
                        Span::styled("  ↓ ", Style::default().fg(theme::LABEL_FG)),
                        Span::styled(
                            format!("{:02}/{:02}/{}", entry.day, entry.month, entry.year),
                            Style::default().fg(theme::LUNAR_FG),
                        ),
                    ]));
                }
            }
        }

        let p = Paragraph::new(lines).block(block);
        p.render(popup_area, buf);
    }
}
