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
            Constraint::Length(6),
            Constraint::Length(8),
            Constraint::Length(7),
            Constraint::Min(10),
        ])
        .split(area);

        render_hours_verdict(self.app, rows[0], buf);
        render_top_windows(self.app, rows[1], buf);
        render_timeline(gio, rows[2], buf);

        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let cols =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[3]);
                render_hour_list(&gio.all_hours, true, cols[0], buf);
                render_hour_list(&gio.all_hours, false, cols[1], buf);
            }
            LayoutMode::Small => {
                let detail =
                    Layout::vertical([Constraint::Percentage(58), Constraint::Percentage(42)])
                        .split(rows[3]);
                render_hour_list(&gio.all_hours, true, detail[0], buf);
                render_hour_list(&gio.all_hours, false, detail[1], buf);
            }
        }
    }
}

fn render_hours_verdict(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Nhận Định ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    if let Some(verdict) = app.hours_verdict() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(verdict.summary, Style::default().fg(Color::Green)),
        ]));
        if let Some(caution) = verdict.caution {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(caution, Style::default().fg(Color::Yellow)),
            ]));
        }
    } else {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Chưa có đủ dữ liệu để luận giờ tốt.",
                Style::default().fg(Color::Gray),
            ),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_top_windows(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Khung Giờ Tốt Nhất ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    if let Some(verdict) = app.hours_verdict() {
        for window in verdict.top_windows.iter().take(3) {
            lines.push(Line::from(vec![
                Span::styled("  ★ ", Style::default().fg(Color::Green)),
                Span::styled(window.clone(), Style::default().fg(Color::White)),
            ]));
        }

        if !verdict.bad_windows.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Giờ nên tránh:", Style::default().fg(Color::Red)),
            ]));
            for window in verdict.bad_windows.iter().take(2) {
                lines.push(Line::from(vec![
                    Span::styled("   · ", Style::default().fg(Color::Red)),
                    Span::styled(window.clone(), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
    } else {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Không có khung giờ nổi bật.",
                Style::default().fg(Color::Gray),
            ),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_timeline(gio: &amlich_api::GioHoangDaoDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(format!(
            " Dòng Thời Gian 12 Giờ — {} giờ tốt ",
            gio.good_hour_count
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut chi_spans: Vec<Span<'_>> = vec![Span::raw(" ")];
    let mut marker_spans: Vec<Span<'_>> = vec![Span::raw(" ")];
    let mut star_spans: Vec<Span<'_>> = vec![Span::raw(" ")];

    let col_w = 10;
    for hour in &gio.all_hours {
        let style = if hour.is_good {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        chi_spans.push(Span::styled(
            format!("{:^w$}", hour.hour_chi, w = col_w),
            style,
        ));
        let marker = if hour.is_good { "★ Tốt" } else { "Xấu" };
        marker_spans.push(Span::styled(
            format!("{:^w$}", marker, w = col_w),
            if hour.is_good {
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red)
            },
        ));
        star_spans.push(Span::styled(format!("{:^w$}", hour.star, w = col_w), style));
    }

    Paragraph::new(vec![
        Line::from(chi_spans),
        Line::from(marker_spans),
        Line::from(star_spans),
    ])
    .render(inner, buf);
}

fn render_hour_list(
    all_hours: &[amlich_api::HourInfoDto],
    show_good: bool,
    area: Rect,
    buf: &mut Buffer,
) {
    let title = if show_good {
        " Cửa Sổ Nên Dùng "
    } else {
        " Cửa Sổ Nên Tránh "
    };
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines: Vec<Line<'_>> = vec![];
    let filtered: Vec<_> = all_hours
        .iter()
        .filter(|hour| hour.is_good == show_good)
        .collect();

    for hour in &filtered {
        let (marker, color) = if show_good {
            ("★", Color::Green)
        } else {
            ("·", Color::Red)
        };
        lines.push(Line::from(vec![
            Span::styled(format!("  {marker} "), Style::default().fg(color)),
            Span::styled(
                format!("{:<6}", hour.hour_chi),
                Style::default().fg(color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("({}) ", hour.time_range),
                Style::default().fg(Color::DarkGray),
            ),
            Span::raw(format!("· {}", hour.star)),
        ]));
    }

    if show_good {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  Tổng: {}/{} giờ thuận", filtered.len(), all_hours.len()),
            Style::default().fg(Color::DarkGray),
        )));
    }

    Paragraph::new(lines).render(inner, buf);
}
