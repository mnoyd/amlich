use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use chrono::Datelike;
use crate::layout::LayoutMode;
use crate::state::AppState;

const WEEKDAY_NAMES: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

pub struct RibbonWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> RibbonWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for RibbonWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::raw("   ◂ "),
        ];

        for (i, name) in WEEKDAY_NAMES.iter().enumerate() {
            if i == dow0 {
                // Today/Selected
                spans.push(Span::styled(
                    format!("[{}] ", name),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
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
