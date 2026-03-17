use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use crate::{layout::LayoutMode, state::AppState};
use crate::widgets::{
    action_board::ActionBoardWidget,
    almanac::AlmanacGridWidget,
    hero::HeroWidget,
    mini_calendar::MiniCalendarWidget,
    timeline::TimelineWidget,
};

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
        // Main layout:
        // Left column (Hero + Almanac)
        // Right column (Mini Calendar + Action Board)
        // Bottom (Timeline)
        
        let chunks = Layout::vertical([
            Constraint::Min(20),   // Top content
            Constraint::Length(5), // Timeline bar
        ]).split(area);

        let top_chunks = Layout::horizontal([
            Constraint::Percentage(60), // Left: Hero + Almanac
            Constraint::Percentage(40), // Right: Calendar + Actions
        ]).split(chunks[0]);

        let left_chunks = Layout::vertical([
            Constraint::Length(10), // Hero
            Constraint::Min(8),     // Almanac
        ]).split(top_chunks[0]);

        let right_chunks = Layout::vertical([
            Constraint::Length(9), // Mini Calendar
            Constraint::Min(8),    // Actions
        ]).split(top_chunks[1]);

        HeroWidget::new(self.app, self.mode).render(left_chunks[0], buf);
        AlmanacGridWidget::new(self.app, self.mode).render(left_chunks[1], buf);

        MiniCalendarWidget::new(self.app, self.mode).render(right_chunks[0], buf);
        ActionBoardWidget::new(self.app, self.mode).render(right_chunks[1], buf);

        TimelineWidget::new(self.app, self.mode).render(chunks[1], buf);
    }
}
