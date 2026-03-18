use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Clear, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use crate::widgets::modal_shell::{centered_modal, ModalPreset};
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

        let popup_area = centered_modal(area, self.mode, ModalPreset::Context);

        Clear.render(popup_area, buf);

        ExplorerWidget::new(self.app, self.mode).render(popup_area, buf);
    }
}
