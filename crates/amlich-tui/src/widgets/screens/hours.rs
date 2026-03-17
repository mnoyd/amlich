use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct HoursScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> HoursScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for HoursScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(area, buf);
            return;
        };
        let Some(gio) = &bundle.gio_hoang_dao else {
            Paragraph::new("Chưa có dữ liệu giờ hoàng đạo.").render(area, buf);
            return;
        };

        let rows = Layout::vertical([
            Constraint::Length(7),
            Constraint::Min(10),
        ]).split(area);

        // Top: Timeline overview
        {
            let block = Block::default()
                .title(format!(" Tổng Quan 12 Giờ — {} giờ tốt ", gio.good_hour_count))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            let inner = block.inner(rows[0]);
            block.render(rows[0], buf);

            let mut chi_spans: Vec<Span<'_>> = vec![Span::raw(" ")];
            let mut marker_spans: Vec<Span<'_>> = vec![Span::raw(" ")];
            let mut star_spans: Vec<Span<'_>> = vec![Span::raw(" ")];

            let col_w = 10;
            for h in &gio.all_hours {
                let style = if h.is_good {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                };
                chi_spans.push(Span::styled(format!("{:^w$}", h.hour_chi, w = col_w), style));
                let m = if h.is_good { "\u{2605} Tốt" } else { "  Xấu" };
                marker_spans.push(Span::styled(
                    format!("{:^w$}", m, w = col_w),
                    if h.is_good {
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::Red)
                    },
                ));
                star_spans.push(Span::styled(format!("{:^w$}", h.star, w = col_w), style));
            }

            Paragraph::new(vec![
                Line::from(chi_spans),
                Line::from(marker_spans),
                Line::from(star_spans),
            ]).render(inner, buf);
        }

        // Bottom: Detail columns
        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let cols = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(rows[1]);
                render_hour_list(&gio.all_hours, true, cols[0], buf);
                render_hour_list(&gio.all_hours, false, cols[1], buf);
            }
            LayoutMode::Small => {
                let detail = Layout::vertical([
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ]).split(rows[1]);
                render_hour_list(&gio.all_hours, true, detail[0], buf);
                render_hour_list(&gio.all_hours, false, detail[1], buf);
            }
        }
    }
}

fn render_hour_list(
    all_hours: &[amlich_api::HourInfoDto],
    show_good: bool,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = if show_good { " \u{2605} Giờ Hoàng Đạo " } else { " Giờ Hắc Đạo " };
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines: Vec<Line<'_>> = vec![];
    let filtered: Vec<_> = all_hours.iter().filter(|h| h.is_good == show_good).collect();

    for h in &filtered {
        let (marker, color) = if show_good {
            ("\u{2605}", Color::Green)
        } else {
            ("\u{00B7}", Color::Red)
        };
        lines.push(Line::from(vec![
            Span::styled(format!("  {marker} "), Style::default().fg(color)),
            Span::styled(
                format!("{:<6}", h.hour_chi),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("({}) ", h.time_range), Style::default().fg(Color::DarkGray)),
            Span::raw(format!("\u{2014} {}", h.star)),
        ]));
    }

    if show_good {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  Tổng: {}/{} giờ tốt", filtered.len(), all_hours.len()),
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines).render(inner, buf);
}
