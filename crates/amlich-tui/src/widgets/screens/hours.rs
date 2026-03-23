use chrono::{Local, NaiveDate, Timelike};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
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

        let timeline_height = match self.mode {
            LayoutMode::Small => Constraint::Length(0), // Hide horizontal timeline completely on thin screens
            _ => Constraint::Length(6),
        };

        let rows = Layout::vertical([
            Constraint::Length(6),
            Constraint::Length(8),
            timeline_height,
            Constraint::Min(10),
        ])
        .split(area);

        render_hours_verdict(self.app, rows[0], buf);
        render_top_windows(self.app, rows[1], buf);

        if !matches!(self.mode, LayoutMode::Small) {
            render_timeline(gio, self.app.date, rows[2], buf);
        }

        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let cols =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[3]);
                render_hour_list(&gio.all_hours, true, cols[0], buf);
                render_hour_list(&gio.all_hours, false, cols[1], buf);
            }
            LayoutMode::Small => {
                render_combined_hour_list(&gio.all_hours, self.app.date, rows[3], buf);
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

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
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

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_timeline(
    gio: &amlich_api::GioHoangDaoDto,
    selected_date: NaiveDate,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(format!(
            " Dòng Thời Gian 12 Giờ — {} giờ tốt ",
            gio.good_hour_count
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    render_block_timeline(gio, selected_date, inner, buf);
}

fn render_block_timeline(
    gio: &amlich_api::GioHoangDaoDto,
    selected_date: NaiveDate,
    area: Rect,
    buf: &mut Buffer,
) {
    let current_idx = current_hour_block_index(selected_date);

    let hour_chunks =
        Layout::horizontal(std::iter::repeat(Constraint::Ratio(1, 12)).take(12)).split(area);

    for (i, hour) in gio.all_hours.iter().enumerate() {
        let is_current = current_idx == Some(i as u32);
        let is_past = current_idx.is_some_and(|idx| (i as u32) < idx);

        let base_color = if hour.is_good {
            Color::Green
        } else {
            Color::Red
        };
        let mut style = Style::default().fg(base_color);
        if is_past && !is_current {
            style = style.fg(Color::DarkGray);
        }

        let chi = Span::styled(
            hour.hour_chi.clone(),
            if is_current {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            },
        );

        let width = hour_chunks[i].width as usize;
        let block_char = if hour.is_good { "█" } else { "▄" };
        let bar_text = block_char.repeat(width.max(1));
        let bar = Span::styled(bar_text, style);

        let indicator = if is_current {
            Span::styled(
                "▲",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw(" ")
        };

        let chunk_layout = Layout::vertical([
            Constraint::Length(1), // Chi
            Constraint::Length(1), // Bar
            Constraint::Length(1), // Indicator
            Constraint::Min(0),
        ])
        .split(hour_chunks[i]);

        use ratatui::layout::Alignment;
        Paragraph::new(chi)
            .alignment(Alignment::Center)
            .render(chunk_layout[0], buf);
        Paragraph::new(bar)
            .alignment(Alignment::Center)
            .render(chunk_layout[1], buf);
        Paragraph::new(indicator)
            .alignment(Alignment::Center)
            .render(chunk_layout[2], buf);
    }
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

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_combined_hour_list(
    all_hours: &[amlich_api::HourInfoDto],
    selected_date: NaiveDate,
    area: Rect,
    buf: &mut Buffer,
) {
    let block = Block::default()
        .title(" Chi Tiết Các Giờ ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let current_idx = current_hour_block_index(selected_date);

    let mut lines: Vec<Line<'_>> = vec![];

    for (i, hour) in all_hours.iter().enumerate() {
        let is_current = current_idx == Some(i as u32);
        let is_past = current_idx.is_some_and(|idx| (i as u32) < idx);

        let (marker, base_color) = if hour.is_good {
            ("★", Color::Green)
        } else {
            ("·", Color::Red)
        };

        // Construct prefix
        let prefix = if is_current {
            Span::styled(
                ">> ",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw("   ")
        };

        let marker_span = if is_past {
            Span::styled(format!("{marker} "), Style::default().fg(Color::DarkGray))
        } else {
            Span::styled(format!("{marker} "), Style::default().fg(base_color))
        };

        let chi_span = if is_current {
            Span::styled(
                format!("{:<6}", hour.hour_chi),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
        } else if is_past {
            Span::styled(
                format!("{:<6}", hour.hour_chi),
                Style::default().fg(Color::DarkGray),
            )
        } else {
            Span::styled(
                format!("{:<6}", hour.hour_chi),
                Style::default().fg(Color::White),
            )
        };

        let time_span = Span::styled(
            format!("({}) ", hour.time_range),
            if is_current {
                Style::default().fg(Color::Gray)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        );

        let star_span = if is_past {
            Span::styled(
                format!("· {}", hour.star),
                Style::default().fg(Color::DarkGray),
            )
        } else {
            Span::styled(format!("· {}", hour.star), Style::default().fg(Color::Gray))
        };

        lines.push(Line::from(vec![
            prefix,
            marker_span,
            chi_span,
            time_span,
            star_span,
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn current_hour_block_index(selected_date: NaiveDate) -> Option<u32> {
    let now = Local::now();
    current_hour_block_index_at(selected_date, now.naive_local().date(), now.hour())
}

fn current_hour_block_index_at(
    selected_date: NaiveDate,
    today: NaiveDate,
    current_hour: u32,
) -> Option<u32> {
    (selected_date == today).then_some(((current_hour + 1) % 24) / 2)
}

#[cfg(test)]
mod tests {
    use super::current_hour_block_index_at;
    use chrono::NaiveDate;

    #[test]
    fn returns_current_block_for_today_only() {
        let today = NaiveDate::from_ymd_opt(2026, 3, 23).expect("valid date");

        assert_eq!(current_hour_block_index_at(today, today, 9), Some(5));
        assert_eq!(current_hour_block_index_at(today, today, 23), Some(0));
    }

    #[test]
    fn suppresses_current_block_for_non_today_dates() {
        let today = NaiveDate::from_ymd_opt(2026, 3, 23).expect("valid date");
        let future = NaiveDate::from_ymd_opt(2026, 3, 25).expect("valid date");
        let past = NaiveDate::from_ymd_opt(2026, 3, 21).expect("valid date");

        assert_eq!(current_hour_block_index_at(future, today, 9), None);
        assert_eq!(current_hour_block_index_at(past, today, 9), None);
    }
}
