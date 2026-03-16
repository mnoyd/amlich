use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct RecommendationsScreenWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> RecommendationsScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for RecommendationsScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Màn hình Recommendations ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = vec![];

        if let Some(verdict) = self.app.hero_verdict() {
            lines.push(Line::from(vec![
                Span::styled("Kết luận nhanh: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    verdict.summary,
                    Style::default().add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(""));
        }

        let rows = self.app.top_recommendation_rows();
        if rows.is_empty() {
            lines.push(Line::from("Chưa có khuyến nghị cho ngày này."));
            Paragraph::new(lines).render(inner, buf);
            return;
        }

        for row in rows {
            lines.push(Line::from(format!("• {}", row.label)));
            if let Some(chip) = row.reason_chip {
                lines.push(Line::from(vec![
                    Span::raw("  ↳ "),
                    Span::styled(chip, Style::default().fg(Color::DarkGray)),
                ]));
            }
            if let Some(primary) = row.reason_details.first() {
                lines.push(Line::from(vec![
                    Span::raw("  ↳ "),
                    Span::styled(primary.clone(), Style::default().fg(Color::Gray)),
                ]));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
