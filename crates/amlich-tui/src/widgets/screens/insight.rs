use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct InsightScreenWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> InsightScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for InsightScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = self.app.bundle.as_ref() else {
            return;
        };

        let block = Block::default()
            .title(" Màn hình Insight ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines = vec![];
        let Some(insight) = bundle.insight.as_ref() else {
            lines.push(Line::from("Insight chưa có dữ liệu cho ngày này."));
            Paragraph::new(lines).render(inner, buf);
            return;
        };

        if let Some(holiday) = insight.holiday.as_ref() {
            let name = holiday
                .names
                .vi
                .first()
                .cloned()
                .unwrap_or_else(|| "Lễ".to_string());
            lines.push(Line::from(vec![
                Span::styled("Lễ: ", Style::default().fg(Color::Magenta)),
                Span::raw(name),
            ]));
            if let Some(origin) = holiday.origin.as_ref() {
                lines.push(Line::from(format!("  Nguồn gốc: {}", origin.vi)));
            }
            if let Some(significance) = holiday.significance.as_ref() {
                lines.push(Line::from(format!("  Ý nghĩa: {}", significance.vi)));
            }
        }

        if let Some(festival) = insight.festival.as_ref() {
            let name = festival
                .names
                .vi
                .first()
                .cloned()
                .unwrap_or_else(|| "Lễ hội".to_string());
            lines.push(Line::from(vec![
                Span::styled("Lễ hội: ", Style::default().fg(Color::Magenta)),
                Span::raw(name),
            ]));
            if let Some(origin) = festival.origin.as_ref() {
                lines.push(Line::from(format!("  Nguồn gốc: {}", origin.vi)));
            }
        }

        if let Some(tiet_khi) = insight.tiet_khi.as_ref() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("Tiết khí: ", Style::default().fg(Color::Yellow)),
                Span::raw(tiet_khi.name.vi.clone()),
            ]));
            lines.push(Line::from(format!("  Ý nghĩa: {}", tiet_khi.meaning.vi)));
            lines.push(Line::from(format!("  Thời tiết: {}", tiet_khi.weather.vi)));
            if !tiet_khi.health.vi.is_empty() {
                lines.push(Line::from(format!(
                    "  Gợi ý sức khỏe: {}",
                    tiet_khi.health.vi.join(", ")
                )));
            }
        }

        if let Some(guidance) = insight.day_guidance.as_ref() {
            lines.push(Line::from(""));
            if !guidance.good_for.vi.is_empty() {
                lines.push(Line::from(format!(
                    "Nên làm: {}",
                    guidance.good_for.vi.join(", ")
                )));
            }
            if !guidance.avoid_for.vi.is_empty() {
                lines.push(Line::from(format!(
                    "Nên tránh: {}",
                    guidance.avoid_for.vi.join(", ")
                )));
            }
        }

        if lines.is_empty() {
            lines.push(Line::from("Insight chưa có dữ liệu diễn giải chi tiết."));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
