use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use chrono::{Datelike, NaiveDate};
use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct CalendarOverlayWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> CalendarOverlayWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for CalendarOverlayWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.app.show_calendar {
            return;
        }

        // Draw a centered popup box
        let popup_width = 30; // standard calendar width (7 * 4 + 2)
        let popup_height = 12; // header + 6 weeks + borders

        let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
        let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

        let popup_area = Rect::new(x, y, popup_width, popup_height);

        // Clear the background behind the popup
        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(" Lịch Tháng ")
            .title_alignment(ratatui::layout::Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));

        let current_date = self.app.date;
        let (y_val, m_val) = (current_date.year(), current_date.month());
        
        let header = Line::from(vec![
            Span::styled(
                format!("    Tháng {} năm {}    ", m_val, y_val),
                Style::default().add_modifier(Modifier::BOLD),
            )
        ]);

        let mut lines = vec![header, Line::from("")];

        // Draw days of week
        lines.push(Line::from(vec![
            Span::styled(" T2", Style::default().fg(Color::DarkGray)),
            Span::styled(" T3", Style::default().fg(Color::DarkGray)),
            Span::styled(" T4", Style::default().fg(Color::DarkGray)),
            Span::styled(" T5", Style::default().fg(Color::DarkGray)),
            Span::styled(" T6", Style::default().fg(Color::DarkGray)),
            Span::styled(" T7", Style::default().fg(Color::DarkGray)),
            Span::styled(" CN", Style::default().fg(Color::Red)),
        ]));

        // Calculate days logic
        let mut first_day = NaiveDate::from_ymd_opt(y_val, m_val, 1).unwrap();
        let padding_days = first_day.weekday().num_days_from_monday();
        let mut row_spans = vec![];

        for _ in 0..padding_days {
            row_spans.push(Span::raw("   "));
        }

        // Note: For simplicity in the TUI MVP we only show Solar dates in the Month calendar,
        // but we'll highlight the current `self.app.date`.
        let mut cur_d = first_day;
        while cur_d.month() == m_val {
            let day_str = format!("{:>3}", cur_d.day());
            
            let mut style = Style::default();
            
            if cur_d == current_date {
                style = style.bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD);
            } else if cur_d.weekday() == chrono::Weekday::Sun {
                style = style.fg(Color::Red);
            }
            
            row_spans.push(Span::styled(day_str, style));

            if cur_d.weekday() == chrono::Weekday::Sun {
                lines.push(Line::from(row_spans.clone()));
                row_spans.clear();
            }
            
            cur_d = cur_d.succ_opt().unwrap();
        }

        if !row_spans.is_empty() {
            lines.push(Line::from(row_spans));
        }

        // Help text at bottom
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("   (Space) > Đóng   ", Style::default().fg(Color::DarkGray))));

        Paragraph::new(lines)
            .block(block)
            .render(popup_area, buf);
    }
}
