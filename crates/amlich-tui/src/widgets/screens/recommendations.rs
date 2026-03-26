use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::{
    action_board::ActionBoardWidget, guidance::GuidanceWidget, guidance_panel::GuidancePanelWidget,
    risk::RiskWidget, travel::TravelWidget,
};
use crate::{layout::LayoutMode, state::{ui_prefs::VerbosityMode, AppState}};

pub struct RecommendationsScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> RecommendationsScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for RecommendationsScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match (self.mode, self.app.active_verbosity()) {
            (LayoutMode::Small, VerbosityMode::Compact) => {
                let rows = Layout::vertical([
                    Constraint::Length(12),
                    Constraint::Length(8),
                    Constraint::Min(8),
                ])
                .split(area);
                GuidanceWidget::new(self.app, self.mode).render(rows[0], buf);
                TravelWidget::new(self.app, self.mode).render(rows[1], buf);
                RiskWidget::new(self.app, self.mode).render(rows[2], buf);
            }
            (LayoutMode::Small, VerbosityMode::Verbose) => {
                let rows =
                    Layout::vertical([Constraint::Length(16), Constraint::Min(10)]).split(area);
                GuidanceWidget::new(self.app, self.mode).render(rows[0], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[1], buf);
            }
            (_, VerbosityMode::Compact) => {
                let rows = Layout::vertical([
                    Constraint::Length(12),
                    Constraint::Length(8),
                    Constraint::Length(9),
                    Constraint::Min(9),
                ])
                .split(area);
                GuidanceWidget::new(self.app, self.mode).render(rows[0], buf);
                TravelWidget::new(self.app, self.mode).render(rows[1], buf);
                ActionBoardWidget::new(self.app, self.mode).render(rows[2], buf);
                RiskWidget::new(self.app, self.mode).render(rows[3], buf);
            }
            (_, VerbosityMode::Verbose) => {
                let rows =
                    Layout::vertical([Constraint::Length(16), Constraint::Min(10)]).split(area);
                GuidanceWidget::new(self.app, self.mode).render(rows[0], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[1], buf);
            }
        }
    }
}
