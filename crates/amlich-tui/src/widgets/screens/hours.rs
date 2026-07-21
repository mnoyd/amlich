use amlich_core::almanac::hour_pillar::compute_hour_pillar;
use amlich_core::HeavenlyStem;
use chrono::{Local, NaiveDate, Timelike};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{
    layout::LayoutMode,
    state::{ui_prefs::VerbosityMode, AppState},
};

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

        match (self.mode, self.app.active_verbosity()) {
            (LayoutMode::Small, VerbosityMode::Compact) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(8),
                    Constraint::Length(4),
                    Constraint::Min(10),
                ])
                .split(area);

                render_hours_verdict(self.app, rows[0], buf);
                render_top_windows(self.app, rows[1], buf);
                render_micro_timeline(gio, self.app.date, rows[2], buf);
                render_combined_hour_list(&gio.all_hours, self.app.date, rows[3], buf);
            }
            (LayoutMode::Small, VerbosityMode::Verbose) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(8),
                    Constraint::Length(4),
                    Constraint::Length(8),
                    Constraint::Min(10),
                ])
                .split(area);

                render_hours_verdict(self.app, rows[0], buf);
                render_top_windows(self.app, rows[1], buf);
                render_micro_timeline(gio, self.app.date, rows[2], buf);
                render_hour_pillar_detail(self.app, rows[3], buf);
                render_combined_hour_list(&gio.all_hours, self.app.date, rows[4], buf);
            }
            (_, VerbosityMode::Compact) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(8),
                    Constraint::Min(10),
                ])
                .split(area);

                render_hours_verdict(self.app, rows[0], buf);
                render_top_windows(self.app, rows[1], buf);
                render_combined_hour_list(&gio.all_hours, self.app.date, rows[2], buf);
            }
            (_, VerbosityMode::Verbose) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(8),
                    Constraint::Length(6),
                    Constraint::Length(8),
                    Constraint::Min(10),
                ])
                .split(area);

                render_hours_verdict(self.app, rows[0], buf);
                render_top_windows(self.app, rows[1], buf);
                render_timeline(gio, self.app.date, rows[2], buf);
                render_hour_pillar_detail(self.app, rows[3], buf);

                let cols =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[4]);
                render_hour_list(&gio.all_hours, true, cols[0], buf);
                render_hour_list(&gio.all_hours, false, cols[1], buf);
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

fn render_hour_pillar_detail(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Trụ Giờ / Luận Chi Tiết ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    let Some(bundle) = &app.bundle else {
        Paragraph::new("Chưa có dữ liệu.").render(inner, buf);
        return;
    };

    let now = Local::now();
    let use_current_time = app.date == now.naive_local().date();
    let (focus_hour, focus_minute, focus_label) = if use_current_time {
        (
            now.hour() as u8,
            now.minute() as u8,
            "Giờ hiện tại".to_string(),
        )
    } else {
        let hour = focus_hour_for_best_window(bundle).unwrap_or(11);
        (hour, 0, format!("Giờ tham chiếu cho ngày {}", app.date))
    };

    if let Some(canchi) = &bundle.canchi {
        if let Ok(day_stem) = HeavenlyStem::try_from(canchi.day.can.as_str()) {
            if let Some(result) = compute_hour_pillar(day_stem, focus_hour, focus_minute) {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(focus_label, Style::default().fg(Color::Cyan)),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("  Trụ giờ: "),
                    Span::styled(result.can_chi.full, Style::default().fg(Color::Yellow)),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("  Khung giờ: "),
                    Span::styled(result.slot.time_range, Style::default().fg(Color::Cyan)),
                    Span::raw(format!(" · Chi {}", result.slot.branch)),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("  Cách tính: "),
                    Span::styled(
                        format!(
                            "{} / {} / {}",
                            result.evidence.source_id,
                            result.evidence.method,
                            result.evidence.profile
                        ),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));
            }
        }
    }

    if let Some(verdict) = app.hours_verdict() {
        if let Some(window) = verdict.top_windows.first() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Giờ đáng ưu tiên: "),
                Span::styled(window.clone(), Style::default().fg(Color::Green)),
            ]));
        }
        if let Some(window) = verdict.bad_windows.first() {
            lines.push(Line::from(vec![
                Span::raw("  Giờ nên dè chừng: "),
                Span::styled(window.clone(), Style::default().fg(Color::Red)),
            ]));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Chưa có đủ dữ liệu để luận trụ giờ.",
                Style::default().fg(Color::Gray),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn focus_hour_for_best_window(bundle: &amlich_api::v2::DayBundleDto) -> Option<u8> {
    bundle
        .gio_hoang_dao
        .as_ref()?
        .good_hours
        .first()
        .and_then(|hour| parse_start_hour(&hour.time_range))
        .map(|hour| hour as u8)
}

fn parse_start_hour(time_range: &str) -> Option<u32> {
    let (start, _) = time_range.split_once(" - ")?;
    let (hour, _) = start.split_once(':')?;
    hour.parse().ok()
}

fn render_timeline(
    gio: &amlich_api::GioHoangDaoDto,
    selected_date: NaiveDate,
    area: Rect,
    buf: &mut Buffer,
) {
    let context = hour_timeline_context(selected_date);
    let block = Block::default()
        .title(format!(
            " Dòng Thời Gian 12 Giờ — {} giờ tốt · {} ",
            gio.good_hour_count,
            context.label()
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(context.border_color()));
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

    let hour_chunks = Layout::horizontal(vec![Constraint::Ratio(1, 12); 12]).split(area);

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

/// Compact 2-row timeline for small screens: a colored bar row with the current
/// block marked by `▲`, and a Chi-label row beneath. Reuses the same realtime
/// context (`current_hour_block_index`, `hour_timeline_context`) as the full
/// timeline so today/other-day behaviour stays consistent.
fn render_micro_timeline(
    gio: &amlich_api::GioHoangDaoDto,
    selected_date: NaiveDate,
    area: Rect,
    buf: &mut Buffer,
) {
    let context = hour_timeline_context(selected_date);
    let block = Block::default()
        .title(format!(
            " Dòng Giờ · {} tốt · {} ",
            gio.good_hour_count,
            context.label()
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(context.border_color()));
    let inner = block.inner(area);
    block.render(area, buf);

    if inner.height < 1 || gio.all_hours.is_empty() {
        return;
    }

    let current_idx = current_hour_block_index(selected_date);
    let chunks = Layout::horizontal(vec![Constraint::Ratio(1, 12); 12]).split(Rect::new(
        inner.x,
        inner.y,
        inner.width,
        inner.height.min(2),
    ));

    use ratatui::layout::Alignment;
    for (i, hour) in gio.all_hours.iter().enumerate() {
        let is_current = current_idx == Some(i as u32);
        let is_past = current_idx.is_some_and(|idx| (i as u32) < idx);

        let bar_style = if is_current {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if is_past {
            Style::default().fg(Color::DarkGray)
        } else if hour.is_good {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Red)
        };

        let chi_style = if is_current {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else if is_past {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::Gray)
        };

        let width = chunks[i].width.max(1) as usize;
        let symbol = if is_current {
            "▲"
        } else if hour.is_good {
            "█"
        } else {
            "▄"
        };
        let bar = Span::styled(symbol.repeat(width), bar_style);
        Paragraph::new(bar)
            .alignment(Alignment::Center)
            .render(chunks[i], buf);

        if inner.height >= 2 {
            let chi = Span::styled(hour.hour_chi.clone(), chi_style);
            let label_area = Rect::new(chunks[i].x, chunks[i].y + 1, chunks[i].width, 1);
            Paragraph::new(chi)
                .alignment(Alignment::Center)
                .render(label_area, buf);
        }
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
    let context = hour_timeline_context(selected_date);
    let block = Block::default()
        .title(format!(" Chi Tiết Các Giờ · {} ", context.label()))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(context.border_color()));
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

fn hour_timeline_context(selected_date: NaiveDate) -> HourTimelineContext {
    let today = Local::now().naive_local().date();
    hour_timeline_context_at(selected_date, today)
}

fn current_hour_block_index_at(
    selected_date: NaiveDate,
    today: NaiveDate,
    current_hour: u32,
) -> Option<u32> {
    (selected_date == today).then_some(((current_hour + 1) % 24) / 2)
}

fn hour_timeline_context_at(selected_date: NaiveDate, today: NaiveDate) -> HourTimelineContext {
    if selected_date == today {
        HourTimelineContext::Today
    } else {
        HourTimelineContext::OtherDay
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HourTimelineContext {
    Today,
    OtherDay,
}

impl HourTimelineContext {
    fn label(self) -> &'static str {
        match self {
            Self::Today => "Hôm Nay",
            Self::OtherDay => "Ngày Khác",
        }
    }

    fn border_color(self) -> Color {
        match self {
            Self::Today => Color::Cyan,
            Self::OtherDay => Color::DarkGray,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        current_hour_block_index_at, hour_timeline_context_at, render_micro_timeline,
        HourTimelineContext,
    };
    use crate::layout::LayoutMode;
    use amlich_api::{GioHoangDaoDto, HourInfoDto};
    use chrono::{Datelike, NaiveDate};
    use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

    fn sample_gio() -> GioHoangDaoDto {
        let chi_names = [
            "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi",
        ];
        let time_ranges = [
            "23:00 - 01:00",
            "01:00 - 03:00",
            "03:00 - 05:00",
            "05:00 - 07:00",
            "07:00 - 09:00",
            "09:00 - 11:00",
            "11:00 - 13:00",
            "13:00 - 15:00",
            "15:00 - 17:00",
            "17:00 - 19:00",
            "19:00 - 21:00",
            "21:00 - 23:00",
        ];
        let good = [
            false, true, false, true, false, true, false, true, false, true, false, true,
        ];
        let all_hours: Vec<HourInfoDto> = (0..12)
            .map(|i| HourInfoDto {
                hour_index: i,
                hour_chi: chi_names[i].to_string(),
                time_range: time_ranges[i].to_string(),
                star: "Sao Mẫu".to_string(),
                is_good: good[i],
            })
            .collect();
        let good_hours: Vec<HourInfoDto> =
            all_hours.iter().filter(|h| h.is_good).cloned().collect();
        GioHoangDaoDto {
            day_chi: "Tý".to_string(),
            good_hour_count: good_hours.len(),
            good_hours,
            all_hours,
            summary: "6/12 giờ tốt".to_string(),
        }
    }

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

    #[test]
    fn distinguishes_today_from_other_days_for_chrome() {
        let today = NaiveDate::from_ymd_opt(2026, 3, 23).expect("valid date");
        let other_day = NaiveDate::from_ymd_opt(2026, 3, 25).expect("valid date");

        assert_eq!(
            hour_timeline_context_at(today, today),
            HourTimelineContext::Today
        );
        assert_eq!(
            hour_timeline_context_at(other_day, today),
            HourTimelineContext::OtherDay
        );
    }

    fn buffer_text(buf: &Buffer, area: Rect) -> String {
        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn micro_timeline_renders_bar_and_chi_labels() {
        let gio = sample_gio();
        // Use a non-today date so no block is marked current; behaviour is
        // purely good/bad coloring and label presence.
        let selected = NaiveDate::from_ymd_opt(2026, 3, 25).expect("valid date");
        let area = Rect::new(0, 0, 48, 4);
        let mut buf = Buffer::empty(area);

        render_micro_timeline(&gio, selected, area, &mut buf);

        let text = buffer_text(&buf, area);

        // Title reports the good-hour count and the "Ngày Khác" context.
        assert!(
            text.contains("6 tốt"),
            "title should show good count: {text}"
        );
        assert!(
            text.contains("Ngày Khác"),
            "should show other-day context: {text}"
        );
        // Every Chi label should appear in the label row.
        for chi in ["Tý", "Sửu", "Dần", "Mão", "Thìn", "Hợi"] {
            assert!(text.contains(chi), "label row missing {chi}: {text}");
        }
        // Good-hour bar segments use the full block glyph; bad ones use the
        // lower-half glyph. Both should be present.
        assert!(text.contains('█'), "missing good-hour bar glyph: {text}");
        assert!(text.contains('▄'), "missing bad-hour bar glyph: {text}");
    }

    #[test]
    fn micro_timeline_marks_current_block_on_today() {
        let gio = sample_gio();
        // For a today-selected date we cannot control wall-clock hour here, but
        // we can at least assert the current marker appears somewhere when the
        // selected date equals today (current_hour_block_index returns Some).
        let today = chrono::Local::now().naive_local().date();
        let area = Rect::new(0, 0, 48, 4);
        let mut buf = Buffer::empty(area);

        render_micro_timeline(&gio, today, area, &mut buf);

        let text = buffer_text(&buf, area);
        assert!(
            text.contains('▲'),
            "current block should be marked on today: {text}"
        );
        assert!(
            text.contains("Hôm Nay"),
            "today context label should render: {text}"
        );
    }

    #[test]
    fn micro_timeline_is_part_of_small_hours_screen() {
        use crate::state::{
            ui_prefs::VerbosityMode, ActiveView, AppMode, AppState, ExplorerAction, ExplorerField,
            ExplorerSelection,
        };
        use amlich_api::v2::DayBundleDto;
        use amlich_api::{
            LunarDto, RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto,
            RulesetDefaultsDto, SolarDto,
        };

        let date = chrono::Local::now().naive_local().date();
        let ruleset_catalog = vec![RulesetCatalogEntryDto {
            id: "vn_baseline_v1".to_string(),
            canonical_id: "vn_baseline_v1".to_string(),
            version: "v1".to_string(),
            region: "vn".to_string(),
            profile: "baseline".to_string(),
            schema_version: "amlich.engine/v1".to_string(),
            is_default: true,
            aliases: vec![],
            defaults: RulesetDefaultsDto {
                tz_offset: 7.0,
                meridian: None,
            },
            source_notes: vec![],
        }];
        let recommendation_pack_catalog = vec![RecommendationPackCatalogEntryDto {
            pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
            request_field: "enabled_pack_ids".to_string(),
            version: "v1".to_string(),
            source_family: "traditional".to_string(),
            mode: "advisory".to_string(),
        }];
        let selection = ExplorerSelection::defaults(date, &ruleset_catalog);
        let bundle = DayBundleDto {
            schema_version: "amlich.engine/v1".to_string(),
            ruleset_id: "t".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            generated_at: "2026-03-12T00:00:00Z".to_string(),
            solar: SolarDto {
                day: date.day() as i32,
                month: date.month() as i32,
                year: date.year(),
                day_of_week: date.weekday().num_days_from_monday() as usize,
                day_of_week_name: "Hôm nay".to_string(),
                date_string: date.to_string(),
            },
            lunar: LunarDto {
                day: 1,
                month: 1,
                year: date.year(),
                is_leap_month: false,
                date_string: "Mùng 1 tháng Giêng".to_string(),
            },
            jd: 0,
            canchi: None,
            tiet_khi: None,
            gio_hoang_dao: Some(sample_gio()),
            day_fortune: None,
            daily_recommendations: None,
            contextual_recommendations: None,
            insight: None,
            upcoming_events: vec![],
        };

        let app = AppState {
            running: true,
            app_mode: AppMode::HoursModal,
            date,
            scroll_offset: 0,
            content_height: 0,
            viewport_height: 0,
            bundle: Some(bundle),
            personal_matrix: None,
            is_loading: false,
            error_msg: None,
            ruleset_catalog,
            recommendation_pack_catalog,
            applied_selection: selection.clone(),
            staged_selection: selection,
            explorer_focus: ExplorerField::Date,
            explorer_action: ExplorerAction::Apply,
            pack_cursor: 0,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            show_week_strip: false,
            show_graph_recommendations: false,
            verbosity: VerbosityMode::Compact,
            focused_section: crate::state::PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            search_input: String::new(),
            personal_focus: crate::state::PersonalField::BirthYear,
            personal_draft: crate::state::PersonalDraft {
                birth_year: String::new(),
                birth_month: String::new(),
                birth_day: String::new(),
                birth_hour: String::new(),
                birth_minute: String::new(),
                gender: None,
            },
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: ActiveView::Today,
            view_history: Vec::new(),
            graph_inspector_focus: crate::state::GraphInspectorFocus::Summary,
            graph_inspector_cursor: 0,
            graph_inspector_search_query: String::new(),
            graph_inspector_search_cursor: 0,
            graph_inspector_focus_before_search: None,
            graph_inspector_lens: crate::state::GraphInspectorLens::General,
            dev_inspector_mode: false,
            explanation_lens: crate::state::UserExplanationLens::ViSao,
            causality_focus: crate::state::CausalityFocus::SummaryList,
        };

        // Small-screen width (~48 cols) at the minimum supported height.
        let area = Rect::new(0, 0, 48, 30);
        let mut buf = Buffer::empty(area);
        super::HoursScreenWidget::new(&app, LayoutMode::Small).render(area, &mut buf);

        let text = buffer_text(&buf, area);
        assert!(
            text.contains("Dòng Giờ"),
            "small hours screen should include the micro-timeline: {text}"
        );
    }
}
