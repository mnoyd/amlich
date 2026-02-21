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
    compact: bool,
}

impl<'a> HoursWidget<'a> {
    pub fn new(app: &'a App) -> Self {
        Self {
            app,
            compact: false,
        }
    }

    pub fn compact(mut self, yes: bool) -> Self {
        self.compact = yes;
        self
    }
}

/// Shorten "23:00-01:00" to "23-01"
fn shorten_time_range(range: &str) -> String {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() == 2 {
        let start = parts[0].split(':').next().unwrap_or(parts[0]);
        let end = parts[1].split(':').next().unwrap_or(parts[1]);
        format!("{}-{}", start, end)
    } else {
        range.to_string()
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

        let inner = block.inner(area);
        let use_compact = self.compact || inner.height < 16;

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

        if use_compact {
            // Compact: horizontal timeline wrapping across available width
            let mut spans: Vec<Span> = Vec::new();
            let mut col = 0;
            let width = inner.width as usize;

            for (i, hour) in ghd.all_hours.iter().enumerate() {
                let label = if hour.is_good {
                    format!("{}★", hour.hour_chi)
                } else {
                    hour.hour_chi.clone()
                };
                let sep = if i > 0 { " " } else { "" };
                let entry_len = sep.len() + label.chars().count();

                if col > 0 && col + entry_len > width {
                    lines.push(Line::from(std::mem::take(&mut spans)));
                    col = 0;
                }

                if col > 0 {
                    spans.push(Span::raw(" "));
                    col += 1;
                }

                let style = if hour.is_good {
                    Style::default()
                        .fg(theme::GOOD_HOUR_FG)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme::BAD_HOUR_FG)
                };
                spans.push(Span::styled(label.clone(), style));
                col += label.chars().count();
            }
            if !spans.is_empty() {
                lines.push(Line::from(spans));
            }
        } else {
            // Full mode
            lines.push(Line::from(""));

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

                let short_time = shorten_time_range(&hour.time_range);

                lines.push(Line::from(vec![
                    Span::styled(format!(" {} ", marker), star_style),
                    Span::styled(format!("{:<4}", hour.hour_chi), style),
                    Span::styled(format!("{:<7}", short_time), style),
                    Span::styled(&hour.star, style),
                ]));
            }
        }

        let p = Paragraph::new(lines).block(block);
        p.render(area, buf);
    }
}
