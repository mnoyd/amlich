use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct EventSummaryWidget<'a> {
    app: &'a AppState,
}

impl<'a> EventSummaryWidget<'a> {
    pub fn new(app: &'a AppState, _mode: LayoutMode) -> Self {
        Self { app }
    }
}

impl Widget for EventSummaryWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let block = Block::default()
            .title(" Sự Kiện / Nhắc Nhở ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = vec![];

        if let Some(insight) = &bundle.insight {
            if let Some(festival) = &insight.festival {
                let name = festival.names.vi.first().unwrap_or(&String::new()).clone();
                lines.push(Line::from(vec![
                    Span::styled(
                        "✨ Hôm nay: ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        name,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                if let Some(activities) = &festival.activities {
                    if let Some(act) = activities.vi.first() {
                        lines.push(Line::from(Span::styled(
                            format!("👉 Nhớ: {}", act),
                            Style::default().fg(Color::Green),
                        )));
                    }
                }
            } else if let Some(holiday) = &insight.holiday {
                let name = holiday.names.vi.first().unwrap_or(&String::new()).clone();
                lines.push(Line::from(vec![
                    Span::styled(
                        "✨ Hôm nay: ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        name,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                if let Some(activities) = &holiday.activities {
                    if let Some(act) = activities.vi.first() {
                        lines.push(Line::from(Span::styled(
                            format!("👉 Nên làm: {}", act),
                            Style::default().fg(Color::Green),
                        )));
                    }
                }
            }
        }

        if lines.is_empty() {
            if bundle.lunar.day == 1 {
                lines.push(Line::from(vec![
                    Span::styled(
                        "✨ Hôm nay: ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "Mùng Một",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(Span::styled(
                    "👉 Nhớ: thắp hương, dọn dẹp ban thờ, làm việc thiện.",
                    Style::default().fg(Color::Green),
                )));
            } else if bundle.lunar.day == 15 {
                lines.push(Line::from(vec![
                    Span::styled(
                        "✨ Hôm nay: ",
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        "Ngày Rằm",
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                lines.push(Line::from(Span::styled(
                    "👉 Nhớ: cúng bái tổ tiên, đi chùa lễ Phật, ăn chay.",
                    Style::default().fg(Color::Green),
                )));
            } else if let Some(upcoming) = bundle.upcoming_events.first() {
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("Sắp tới ({} ngày): ", upcoming.days_left),
                        Style::default()
                            .fg(Color::Gray)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        &upcoming.name,
                        Style::default()
                            .fg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]));
                for event in bundle.upcoming_events.iter().skip(1).take(2) {
                    lines.push(Line::from(vec![
                        Span::styled("  • ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            format!("{} ngày nữa: {}", event.days_left, event.name),
                            Style::default().fg(Color::Gray),
                        ),
                    ]));
                }
            } else {
                lines.push(Line::from(Span::styled(
                    "Không có sự kiện đặc biệt hôm nay.",
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
