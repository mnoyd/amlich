use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::{guidance::GuidanceWidget, guidance_panel::GuidancePanelWidget};
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
                GuidanceWidget::new(self.app, self.mode).render(area, buf);
            }
            (LayoutMode::Small, VerbosityMode::Verbose) => {
                let rows =
                    Layout::vertical([Constraint::Percentage(72), Constraint::Percentage(28)]).split(area);
                GuidanceWidget::new(self.app, self.mode).render(rows[0], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[1], buf);
            }
            (_, VerbosityMode::Compact) => {
                let rows =
                    Layout::vertical([Constraint::Percentage(78), Constraint::Percentage(22)]).split(area);
                GuidanceWidget::new(self.app, self.mode).render(rows[0], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[1], buf);
            }
            (_, VerbosityMode::Verbose) => {
                let rows =
                    Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)]).split(area);
                GuidanceWidget::new(self.app, self.mode).render(rows[0], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[1], buf);
            }
        }
    }
}
