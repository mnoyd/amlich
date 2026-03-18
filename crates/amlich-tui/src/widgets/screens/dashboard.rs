use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::Paragraph,
    widgets::Widget,
};

use crate::theme::Theme;
use crate::widgets::{
    action_board::ActionBoardWidget, almanac::AlmanacGridWidget, event_summary::EventSummaryWidget,
    hero::HeroWidget, mini_calendar::MiniCalendarWidget, timeline::TimelineWidget,
};
use crate::{layout::LayoutMode, state::AppState};

pub struct DashboardScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> DashboardScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for DashboardScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let shell = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).split(area);
        Paragraph::new(Line::from(vec![
            Span::styled(
                "▶ Dashboard",
                Theme::accent_warn().add_modifier(Modifier::BOLD),
            ),
            Span::styled(" · Tóm tắt quyết định trong ngày", Theme::text_dim()),
        ]))
        .render(shell[0], buf);

        // Main layout:
        // Left column (Hero + Almanac)
        // Right column (Mini Calendar + Event Summary + Action Board)
        // Bottom (Timeline)

        let chunks = Layout::vertical([
            Constraint::Min(20),   // Top content
            Constraint::Length(5), // Timeline bar
        ])
        .split(shell[1]);

        let top_chunks = Layout::horizontal([
            Constraint::Percentage(60), // Left: Hero + Almanac
            Constraint::Percentage(40), // Right: Calendar + Event Summary + Actions
        ])
        .split(chunks[0]);

        let left_chunks = Layout::vertical([
            Constraint::Length(10), // Hero
            Constraint::Min(8),     // Almanac
        ])
        .split(top_chunks[0]);

        let right_chunks = Layout::vertical([
            Constraint::Length(9), // Mini Calendar
            Constraint::Length(4), // Event Summary
            Constraint::Min(4),    // Actions
        ])
        .split(top_chunks[1]);

        HeroWidget::new(self.app, self.mode).render(left_chunks[0], buf);
        AlmanacGridWidget::new(self.app, self.mode).render(left_chunks[1], buf);

        MiniCalendarWidget::new(self.app, self.mode).render(right_chunks[0], buf);
        EventSummaryWidget::new(self.app, self.mode).render(right_chunks[1], buf);
        ActionBoardWidget::new(self.app, self.mode).render(right_chunks[2], buf);

        TimelineWidget::new(self.app, self.mode).render(chunks[1], buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use ratatui::{buffer::Buffer, layout::Rect};

    #[test]
    fn dashboard_focus_state_renders_primary_marker() {
        let date = NaiveDate::from_ymd_opt(2026, 3, 18).expect("valid date");
        let app = AppState::new(Some(date));
        let area = Rect::new(0, 0, 120, 30);
        let mut buf = Buffer::empty(area);

        DashboardScreenWidget::new(&app, LayoutMode::Large).render(area, &mut buf);

        let rendered = (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("▶"));
    }
}
