use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct HoursScreenWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> HoursScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for HoursScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Giờ Tốt ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let text = if self.app.bundle.is_some() {
            "Đang phát triển — Giờ Hoàng Đạo chi tiết"
        } else {
            "Chưa có dữ liệu."
        };
        Paragraph::new(text).block(block).render(area, buf);
    }
}
