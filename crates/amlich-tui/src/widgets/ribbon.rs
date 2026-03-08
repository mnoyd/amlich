use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use chrono::Datelike;

const WEEKDAY_NAMES: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

pub struct RibbonWidget<'a> {
    app: &'a AppState,
}

impl<'a> RibbonWidget<'a> {
    pub fn new(app: &'a AppState, _mode: LayoutMode) -> Self {
        Self { app }
    }
}

impl Widget for RibbonWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.is_calendar_view() {
            let line = Line::from(vec![
                Span::styled(
                    " [Lịch] ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "h/l, j/k: di chuyển  ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    "PgUp/PgDn: đổi tháng  ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    "Enter: chọn  Esc/Space: đóng",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            let p = Paragraph::new(line).alignment(Alignment::Center);
            let bottom_line = Rect {
                x: area.x,
                y: area.y + 1,
                width: area.width,
                height: 1,
            };
            p.render(bottom_line, buf);
            return;
        }

        // A minimal bottom ribbon
        // Format: [LensName] ◂ T2 T3 [T4] T5 T6 T7 CN ▸

        let lens_name = match self.app.lens {
            crate::state::FocusLens::General => "Chung",
            crate::state::FocusLens::Planning => "Hành Sự",
            crate::state::FocusLens::Scholarly => "Học Thuật",
            crate::state::FocusLens::Personal => "Cá Nhân",
        };

        // Determine current day of week index (0 = Monday, 6 = Sunday)
        let dow0 = self.app.date.weekday().num_days_from_monday() as usize;

        let mut spans = vec![
            Span::styled(
                format!(" [{}] ", lens_name),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(" a: hành sự  ", Style::default().fg(Color::DarkGray)),
            Span::raw("  ◂ "),
        ];

        for (i, name) in WEEKDAY_NAMES.iter().enumerate() {
            if i == dow0 {
                // Today/Selected
                spans.push(Span::styled(
                    format!("[{}] ", name),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    format!("{} ", name),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }
        spans.push(Span::raw("▸"));

        let p = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
        // The area has 2 height (constraint), we put this on the bottom line.
        let bottom_line = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: 1,
        };
        p.render(bottom_line, buf);
    }
}
