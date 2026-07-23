use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::state::AppState;

pub struct StatusFooterWidget<'a> {
    app: &'a AppState,
}

impl<'a> StatusFooterWidget<'a> {
    pub fn new(app: &'a AppState) -> Self {
        Self { app }
    }
}

impl Widget for StatusFooterWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.is_calendar_view() {
            let line = Line::from(vec![
                Span::styled("j/k: di chuyển  ", Style::default().fg(Color::DarkGray)),
                Span::styled("[ ]: đổi tháng  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "Enter: chọn  Esc: đóng",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            Paragraph::new(line)
                .alignment(Alignment::Center)
                .render(area, buf);
            return;
        }

        if self.app.app_mode == crate::state::AppMode::HoursModal {
            let line = Line::from(vec![Span::styled(
                "h/Esc: đóng",
                Style::default().fg(Color::DarkGray),
            )]);

            Paragraph::new(line)
                .alignment(Alignment::Center)
                .render(area, buf);
            return;
        }

        let hotkey_line = Line::from(vec![Span::styled(
            "Tab: màn  h: giờ  m: lịch  ←/→: ngày  t: hôm nay  ?: trợ giúp",
            Style::default().fg(Color::DarkGray),
        )]);

        Paragraph::new(hotkey_line)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
