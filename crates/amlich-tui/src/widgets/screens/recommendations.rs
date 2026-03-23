use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

use crate::widgets::{guidance::GuidanceWidget, guidance_panel::GuidancePanelWidget};
use crate::{layout::LayoutMode, state::AppState};

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
        let rows =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)]).split(area);
        GuidanceWidget::new(self.app, self.mode).render(rows[0], buf);
        GuidancePanelWidget::new(self.app, self.mode).render(rows[1], buf);
    }
}
