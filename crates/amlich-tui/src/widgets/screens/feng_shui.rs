use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct FengShuiScreenWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> FengShuiScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for FengShuiScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Phong Thủy ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let text = if self.app.bundle.is_some() {
            "Đang phát triển — Tứ Mệnh & Đại Vận"
        } else {
            "Chưa có dữ liệu."
        };
        Paragraph::new(text).block(block).render(area, buf);
    }
}
