use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::widgets::guidance::GuidanceWidget;
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
        // Delegate fully to GuidanceWidget, but tell app to always expand
        // actually we don't need to mutate app, we can just use GuidanceWidget
        GuidanceWidget::new(self.app, self.mode).render(area, buf);
    }
}
