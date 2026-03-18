use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::{Clear, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use crate::theme::Theme;
use crate::widgets::modal_shell::{centered_modal, modal_block, ModalPreset};

pub struct SearchOverlayWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> SearchOverlayWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for SearchOverlayWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.app_mode != crate::state::AppMode::SearchModal {
            return;
        }

        let popup_area = centered_modal(area, self._mode, ModalPreset::Search);

        Clear.render(popup_area, buf);

        let block = modal_block(" Tìm kiếm ngày ");

        let input_display = format!("> {}{}", self.app.search_input, "█");

        let lines = vec![
            Line::from(Span::styled(
                " Nhập ngày (YYYY-MM-DD hoặc DD/MM/YYYY):",
                Theme::text_muted(),
            )),
            Line::from(Span::styled(
                input_display,
                Theme::accent_warn().add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(" Enter: đi tới  Esc: đóng", Theme::text_dim())),
        ];

        Paragraph::new(lines).block(block).render(popup_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppMode;
    use chrono::NaiveDate;
    use ratatui::{buffer::Buffer, layout::Rect};

    #[test]
    fn search_modal_uses_shared_shell_title_format() {
        let mut app = AppState::new(NaiveDate::from_ymd_opt(2026, 3, 18));
        app.app_mode = AppMode::SearchModal;
        let area = Rect::new(0, 0, 120, 30);
        let mut buf = Buffer::empty(area);

        SearchOverlayWidget::new(&app, LayoutMode::Large).render(area, &mut buf);

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

        assert!(rendered.contains("Esc: đóng"));
    }
}
