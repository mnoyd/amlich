use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct SolarTermsScreenWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> SolarTermsScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for SolarTermsScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Tiết Khí ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let text = if self.app.bundle.is_some() {
            "Đang phát triển — Tiết Khí & Sức Khỏe"
        } else {
            "Chưa có dữ liệu."
        };
        Paragraph::new(text).block(block).render(area, buf);
    }
}
