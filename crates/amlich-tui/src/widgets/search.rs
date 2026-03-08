use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

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
        if !self.app.show_search {
            return;
        }

        let popup_width = 40;
        let popup_height = 5;

        let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
        let y = area.y + (area.height.saturating_sub(popup_height)) / 2;

        let popup_area = Rect::new(x, y, popup_width, popup_height);

        Clear.render(popup_area, buf);

        let block = Block::default()
            .title(" Đến Ngày (Tìm kiếm) ")
            .title_alignment(ratatui::layout::Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let input_display = format!("> {}{}", self.app.search_input, "█");

        let lines = vec![
            Line::from(" Nhập ngày (YYYY-MM-DD hoặc DD/MM/YYYY):"),
            Line::from(Span::styled(
                input_display,
                Style::default().fg(Color::Yellow),
            )),
        ];

        Paragraph::new(lines).block(block).render(popup_area, buf);
    }
}
