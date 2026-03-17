use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::widgets::{risk::RiskWidget, scholarly::ScholarlyWidget, travel::TravelWidget};
use crate::{layout::LayoutMode, state::AppState};

pub struct InsightScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> InsightScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for InsightScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);

        let top_chunks =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

        let bottom_chunks =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);

        ScholarlyWidget::new(self.app, self.mode).render(top_chunks[0], buf);
        RiskWidget::new(self.app, self.mode).render(top_chunks[1], buf);

        TravelWidget::new(self.app, self.mode).render(bottom_chunks[0], buf);

        let block = Block::default()
            .title(" Giải Nghĩa Insight ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let mut text = String::from("Chưa có dữ liệu giải nghĩa chi tiết.");
        if let Some(bundle) = self.app.bundle.as_ref() {
            if let Some(insight) = bundle.insight.as_ref() {
                if let Some(truc) = insight.truc.as_ref() {
                    text = format!(
                        "Trực {}: {}\n\nNên làm: {}\nTránh làm: {}",
                        truc.name,
                        truc.meaning.vi,
                        truc.good_for.vi.join(", "),
                        truc.avoid_for.vi.join(", ")
                    );
                }
            }
        }

        Paragraph::new(text)
            .block(block)
            .render(bottom_chunks[1], buf);
    }
}
