use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::{
    action_board::ActionBoardWidget, almanac::AlmanacGridWidget, event_summary::EventSummaryWidget,
    hero::HeroWidget, mini_calendar::MiniCalendarWidget, timeline::TimelineWidget,
};
use crate::{layout::LayoutMode, state::{ui_prefs::VerbosityMode, AppState}};

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
        match (self.mode, self.app.active_verbosity()) {
            (LayoutMode::Small, VerbosityMode::Compact) => render_small_compact(self.app, area, buf),
            (LayoutMode::Small, VerbosityMode::Verbose) => render_small_verbose(self.app, area, buf),
            (_, VerbosityMode::Compact) => render_standard_compact(self.app, self.mode, area, buf),
            (_, VerbosityMode::Verbose) => render_standard_verbose(self.app, self.mode, area, buf),
        }
    }
}

fn render_small_compact(app: &AppState, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(7),
        Constraint::Min(8),
    ])
    .split(area);

    HeroWidget::new(app, LayoutMode::Small).render(rows[0], buf);
    EventSummaryWidget::new(app, LayoutMode::Small).render(rows[1], buf);
    ActionBoardWidget::new(app, LayoutMode::Small).render(rows[2], buf);
}

fn render_small_verbose(app: &AppState, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(7),
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(5),
    ])
    .split(area);

    HeroWidget::new(app, LayoutMode::Small).render(rows[0], buf);
    EventSummaryWidget::new(app, LayoutMode::Small).render(rows[1], buf);
    MiniCalendarWidget::new(app, LayoutMode::Small).render(rows[2], buf);
    ActionBoardWidget::new(app, LayoutMode::Small).render(rows[3], buf);
    TimelineWidget::new(app, LayoutMode::Small).render(rows[4], buf);
}

fn render_standard_compact(app: &AppState, mode: LayoutMode, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([
        Constraint::Length(10),
        Constraint::Min(12),
        Constraint::Length(5),
    ])
    .split(area);

    HeroWidget::new(app, mode).render(rows[0], buf);

    let middle = Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).split(rows[1]);
    ActionBoardWidget::new(app, mode).render(middle[0], buf);
    EventSummaryWidget::new(app, mode).render(middle[1], buf);

    TimelineWidget::new(app, mode).render(rows[2], buf);
}

fn render_standard_verbose(app: &AppState, mode: LayoutMode, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::vertical([Constraint::Min(20), Constraint::Length(5)]).split(area);

    let top_chunks = Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).split(chunks[0]);

    let left_chunks = Layout::vertical([Constraint::Length(10), Constraint::Min(8)]).split(top_chunks[0]);

    let right_chunks = Layout::vertical([Constraint::Length(9), Constraint::Length(4), Constraint::Min(4)]).split(top_chunks[1]);

    HeroWidget::new(app, mode).render(left_chunks[0], buf);
    AlmanacGridWidget::new(app, mode).render(left_chunks[1], buf);

    MiniCalendarWidget::new(app, mode).render(right_chunks[0], buf);
    EventSummaryWidget::new(app, mode).render(right_chunks[1], buf);
    ActionBoardWidget::new(app, mode).render(right_chunks[2], buf);

    TimelineWidget::new(app, mode).render(chunks[1], buf);
}
