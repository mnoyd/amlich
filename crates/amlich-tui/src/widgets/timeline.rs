use chrono::{Local, Timelike};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct TimelineWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> TimelineWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for TimelineWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let block = Block::default()
            .title(" Giờ Hoàng Đạo ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);

        let Some(hours_data) = &bundle.gio_hoang_dao else {
            Paragraph::new("Chưa có dữ liệu giờ.").render(inner, buf);
            return;
        };

        let chunks = Layout::vertical([
            Constraint::Length(1), // Bar
            Constraint::Length(1), // Labels
            Constraint::Min(1),    // Legend/Upcoming
        ])
        .split(inner);

        // 12 canh giờ
        // We will render a continuous bar using block characters.
        // Each canh giờ is 2 hours, so 12 blocks. We can use multiple chars per block.
        // Let's allocate 4 chars per canh giờ => 48 chars total.
        let mut bar_spans = vec![];
        let mut label_spans = vec![];

        for (i, hd) in hours_data.all_hours.iter().enumerate() {
            let color = if hd.is_good {
                Color::Yellow // Gold
            } else {
                Color::DarkGray // Grey
            };

            bar_spans.push(Span::styled("████", Style::default().fg(color)));

            // Labels (e.g. 23, 01, 03)
            let start_hour = hd.time_range.split(':').next().unwrap_or("00");
            if i == 0 {
                label_spans.push(Span::styled(
                    format!("{:<4}", start_hour),
                    Style::default().fg(Color::Gray),
                ));
            } else {
                label_spans.push(Span::styled(
                    format!("{:<4}", start_hour),
                    Style::default().fg(Color::Gray),
                ));
            }
        }

        Paragraph::new(Line::from(bar_spans))
            .alignment(Alignment::Center)
            .render(chunks[0], buf);

        Paragraph::new(Line::from(label_spans))
            .alignment(Alignment::Center)
            .render(chunks[1], buf);

        // Upcoming / Top Windows
        let mut text_lines = vec![];
        if let Some(upcoming) = upcoming_good_window(self.app.date, hours_data) {
            text_lines.push(Line::from(vec![
                Span::raw("Sắp tới: "),
                Span::styled(upcoming, Style::default().fg(Color::Yellow)),
            ]));
        }

        Paragraph::new(text_lines)
            .alignment(Alignment::Center)
            .render(chunks[2], buf);
    }
}

fn upcoming_good_window(
    date: chrono::NaiveDate,
    hours_data: &amlich_api::GioHoangDaoDto,
) -> Option<String> {
    if hours_data.good_hours.is_empty() {
        return None;
    }

    if date != Local::now().date_naive() {
        return hours_data
            .good_hours
            .first()
            .map(|hour| format!("{} ({})", hour.time_range, hour.hour_chi));
    }

    let current_hour = Local::now().hour();
    hours_data
        .good_hours
        .iter()
        .find(|hour| {
            parse_start_hour(&hour.time_range)
                .map(|start_hour| start_hour >= current_hour)
                .unwrap_or(false)
        })
        .map(|hour| format!("{} ({})", hour.time_range, hour.hour_chi))
        .or_else(|| {
            hours_data
                .good_hours
                .first()
                .map(|hour| format!("{} ({}) - ngày sau", hour.time_range, hour.hour_chi))
        })
}

fn parse_start_hour(time_range: &str) -> Option<u32> {
    let (start, _) = time_range.split_once(" - ")?;
    let (hour, _) = start.split_once(':')?;
    hour.parse().ok()
}
