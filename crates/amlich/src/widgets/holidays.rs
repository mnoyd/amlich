use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::theme;

pub struct HolidayOverlay<'a> {
    app: &'a App,
}

impl<'a> HolidayOverlay<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for HolidayOverlay<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the overlay
        let width = area.width.clamp(30, 60);
        let height = area.height.clamp(10, 30);
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let overlay_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(overlay_area, buf);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme::HOLIDAY_FG))
            .title(Line::from(vec![Span::styled(
                format!(
                    " Ngày Lễ - Tháng {}/{} ",
                    self.app.view_month, self.app.view_year
                ),
                theme::section_style(),
            )]))
            .title_bottom(Line::from(" H/Esc đóng  ↑↓ cuộn ").alignment(Alignment::Center));

        let holidays = amlich_api::get_holidays(self.app.view_year, false);

        let mut lines = Vec::new();
        for h in &holidays {
            let date_str = format!("{:02}/{:02}", h.solar_day, h.solar_month);
            let is_major = if h.is_major { "●" } else { "○" };

            lines.push(Line::from(vec![
                Span::styled(
                    format!(" {} ", is_major),
                    if h.is_major {
                        Style::default()
                            .fg(theme::HOLIDAY_FG)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme::SECONDARY_FG)
                    },
                ),
                Span::styled(
                    format!("{} ", date_str),
                    Style::default().fg(theme::ACCENT_FG),
                ),
                Span::styled(
                    &h.name,
                    Style::default()
                        .fg(theme::PRIMARY_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            if !h.description.is_empty() {
                lines.push(Line::from(vec![
                    Span::raw("       "),
                    Span::styled(&h.description, Style::default().fg(theme::LUNAR_FG)),
                ]));
            }
        }

        let p = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((self.app.holiday_scroll, 0));

        p.render(overlay_area, buf);
    }
}
