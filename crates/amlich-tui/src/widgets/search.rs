use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::app::App;
use crate::search::SearchResult;
use crate::theme;

pub struct SearchPopup<'a> {
    app: &'a App,
}

impl<'a> SearchPopup<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    /// Format a single search result as a display line
    fn format_result(result: &SearchResult, is_selected: bool) -> Line<'_> {
        let e = result.entry;
        let date_str = format!("{:02}/{:02}/{}", e.day, e.month, e.year);

        if is_selected {
            // Highlighted/selected row
            Line::from(vec![
                Span::styled("▸ ", Style::default().fg(theme::SELECTED_FG)),
                Span::styled(
                    format!("{} - {} ({})", date_str, result.holiday_name, result.lunar),
                    Style::default()
                        .fg(theme::SELECTED_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ])
        } else {
            // Normal row
            Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(
                    format!("{} - {} ({})", date_str, result.holiday_name, result.lunar),
                    Style::default().fg(theme::LUNAR_FG),
                ),
            ])
        }
    }
}

impl Widget for SearchPopup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate popup size based on results
        let num_results = self.app.search_results.len();
        let max_visible = num_results.min(8);
        let result_height = if num_results > 0 { max_visible + 4 } else { 6 };
        let result_height = result_height as u16;

        let width = 56u16;
        let x = area.x + area.width.saturating_sub(width) / 2;
        let y = area.y + area.height.saturating_sub(result_height) / 2;
        let popup_area = Rect::new(x, y, width, result_height);

        // Clear background
        Clear.render(popup_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::ACCENT_FG))
            .title(Line::from(vec![Span::styled(
                " 🔍 Search Holidays ",
                theme::title_style(),
            )]))
            .title_bottom(
                Line::from(" ↑↓ or Tab: Navigate  │  Esc: Close ").alignment(Alignment::Center),
            );

        let mut lines: Vec<Line> = Vec::new();

        // Input field with cursor
        let input_text = format!("{}_", self.app.search_query);
        lines.push(Line::from(vec![
            Span::styled("Search: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled(
                input_text,
                Style::default()
                    .fg(theme::VALUE_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(""));

        // Results list
        if self.app.search_results.is_empty() {
            if self.app.search_query.is_empty() {
                lines.push(Line::from(vec![Span::styled(
                    "Type to search for holidays",
                    Style::default().fg(theme::LUNAR_FG),
                )]));
            } else {
                lines.push(Line::from(vec![Span::styled(
                    "No results found",
                    Style::default().fg(theme::LABEL_FG),
                )]));
            }
        } else {
            // Show up to 8 results with the selected one highlighted
            let results = &self.app.search_results;
            let idx = self.app.search_index;

            // Determine visible range (centered on selection if possible)
            let start = if max_visible >= num_results {
                0
            } else {
                idx.saturating_sub(3).min(num_results - max_visible)
            };
            let end = (start + max_visible).min(num_results);

            for (offset, result) in results[start..end].iter().enumerate() {
                let i = start + offset;
                let is_selected = i == idx;
                lines.push(Self::format_result(result, is_selected));
            }

            // Show "more" indicator if results are scrolled
            if end < num_results {
                lines.push(Line::from(vec![Span::styled(
                    format!("  ... and {} more", num_results - end),
                    Style::default().fg(theme::LABEL_FG),
                )]));
            }
        }

        let p = Paragraph::new(lines).block(block);
        p.render(popup_area, buf);

        // Highlight the selected row background
        if !self.app.search_results.is_empty() {
            let idx = self.app.search_index;
            let max_visible = num_results.min(8);
            let start = if max_visible >= num_results {
                0
            } else {
                idx.saturating_sub(3).min(num_results - max_visible)
            };

            if idx >= start && idx < start + max_visible {
                let row_offset = idx - start;
                let row_y = popup_area.y + 3 + row_offset as u16; // +3 for title, input, spacer

                // Set background for selected row
                for x in (popup_area.x + 1)..(popup_area.x + popup_area.width - 1) {
                    let cell = &mut buf[(x, row_y)];
                    cell.set_bg(theme::SELECTED_BG);
                    cell.set_fg(theme::SELECTED_FG);
                }
            }
        }
    }
}
