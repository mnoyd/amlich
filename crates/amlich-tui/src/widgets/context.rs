use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Clear, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use crate::widgets::explorer::ExplorerWidget;

pub struct ContextModalWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> ContextModalWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for ContextModalWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.app_mode != crate::state::AppMode::ContextModal {
            return;
        }

        let popup_width = 60;
        let popup_height = 20;

        let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
        let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

        let popup_area = Rect::new(x, y, popup_width, popup_height);

        Clear.render(popup_area, buf);

        ExplorerWidget::new(self.app, self.mode).render(popup_area, buf);
    }
}
