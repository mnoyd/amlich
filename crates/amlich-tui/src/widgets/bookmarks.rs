use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::theme;

pub struct BookmarksOverlay<'a> {
    app: &'a App,
}

impl<'a> BookmarksOverlay<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for BookmarksOverlay<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the overlay
        let width = area.width.clamp(30, 50);
        let height = area.height.clamp(10, 30);
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let overlay_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(overlay_area, buf);

        let current = self.app.current_entry();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::ACCENT_FG))
            .title(Line::from(vec![Span::styled(
                " 🔖 Bookmarks ",
                theme::title_style(),
            )]))
            .title_bottom(Line::from(" 1-9 jump │ Esc/B close ").alignment(Alignment::Center));

        let mut lines = Vec::new();

        if self.app.bookmarks.is_empty() {
            lines.push(Line::from(vec![
                Span::styled(
                    "Empty",
                    Style::default().fg(theme::LABEL_FG),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled(
                    "Press 'b' on a date to bookmark it",
                    Style::default().fg(theme::LUNAR_FG),
                ),
            ]));
        } else {
            for (i, bookmark) in self.app.bookmarks.iter().enumerate() {
                let is_current = *bookmark == current;
                let date_str = format!("{:02}/{:02}/{}", bookmark.day, bookmark.month, bookmark.year);

                let num = if i < 9 {
                    format!("{} ", i + 1)
                } else {
                    "  ".to_string()
                };

                let marker = if is_current { "→" } else { " " };

                lines.push(Line::from(vec![
                    Span::styled(
                        format!(" {}{} ", num, marker),
                        if is_current {
                            Style::default()
                                .fg(theme::ACCENT_FG)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(theme::LABEL_FG)
                        },
                    ),
                    Span::styled(
                        date_str,
                        if is_current {
                            Style::default()
                                .fg(theme::SELECTED_FG)
                                .add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(theme::VALUE_FG)
                        },
                    ),
                ]));
            }
        }

        let p = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((self.app.bookmark_scroll, 0));

        p.render(overlay_area, buf);
    }
}
