use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::App;
use crate::theme;

pub struct HoursWidget<'a> {
    app: &'a App,
}

impl<'a> HoursWidget<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for HoursWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Line::from(vec![Span::styled(
                " ⏰ Giờ Hoàng Đạo ",
                theme::title_style(),
            )]));

        let Some(info) = self.app.selected_info() else {
            let p = Paragraph::new("").block(block);
            p.render(area, buf);
            return;
        };

        let ghd = &info.gio_hoang_dao;
        let mut lines = Vec::new();

        // Summary
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} giờ tốt", ghd.good_hour_count),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" ({})", ghd.day_chi),
                Style::default().fg(theme::LABEL_FG),
            ),
        ]));

        lines.push(Line::from(""));

        // All 12 hours
        for hour in &ghd.all_hours {
            let marker = if hour.is_good { "★" } else { "·" };
            let style = if hour.is_good {
                Style::default().fg(theme::GOOD_HOUR_FG)
            } else {
                Style::default().fg(theme::BAD_HOUR_FG)
            };

            let star_style = if hour.is_good {
                Style::default()
                    .fg(theme::GOOD_HOUR_FG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::BAD_HOUR_FG)
            };

            lines.push(Line::from(vec![
                Span::styled(format!(" {} ", marker), star_style),
                Span::styled(format!("{:<4}", hour.hour_chi), style),
                Span::styled(format!("{:<12}", hour.time_range), style),
                Span::styled(&hour.star, style),
            ]));
        }

        let p = Paragraph::new(lines).block(block);
        p.render(area, buf);
    }
}
