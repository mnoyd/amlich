use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use amlich_api::RecommendationBucketDto;

pub struct ActionBoardWidget<'a> {
    app: &'a AppState,
}

impl<'a> ActionBoardWidget<'a> {
    pub fn new(app: &'a AppState, _mode: LayoutMode) -> Self {
        Self { app }
    }
}

impl Widget for ActionBoardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let block = Block::default()
            .title(" Hành Động (Nên / Tránh) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(inner);

        let Some(recs) = &bundle.daily_recommendations else {
            Paragraph::new("Chưa có dữ liệu hành động.").render(inner, buf);
            return;
        };

        // Collect top 2 Nen
        let mut nen = vec![];
        for act in &recs.activities {
            if act.bucket == RecommendationBucketDto::Nen
                || act.bucket == RecommendationBucketDto::CoThe
            {
                if nen.len() < 2 {
                    nen.push(act);
                }
            }
        }

        // Collect top 2 Tranh
        let mut tranh = vec![];
        for act in &recs.activities {
            if act.bucket == RecommendationBucketDto::KyManh
                || act.bucket == RecommendationBucketDto::Tranh
            {
                if tranh.len() < 2 {
                    tranh.push(act);
                }
            }
        }

        let mut nen_lines = vec![
            Line::from(Span::styled(
                " NÊN LÀM ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        if nen.is_empty() {
            nen_lines.push(Line::from(Span::styled(
                "Không có thông tin.",
                Style::default().fg(Color::Gray),
            )));
        } else {
            for act in nen {
                nen_lines.push(Line::from(Span::styled(
                    format!("• {}", act.label.vi),
                    Style::default().fg(Color::Green),
                )));
                if let Some(reason) = act.reasons.first() {
                    nen_lines.push(Line::from(Span::styled(
                        format!("  └ {}", reason.summary_vi),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
            }
        }

        let mut tranh_lines = vec![
            Line::from(Span::styled(
                " CẦN TRÁNH ",
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        if tranh.is_empty() {
            tranh_lines.push(Line::from(Span::styled(
                "Không có thông tin.",
                Style::default().fg(Color::Gray),
            )));
        } else {
            for act in tranh {
                tranh_lines.push(Line::from(Span::styled(
                    format!("• {}", act.label.vi),
                    Style::default().fg(Color::Red),
                )));
                if let Some(reason) = act.reasons.first() {
                    tranh_lines.push(Line::from(Span::styled(
                        format!("  └ {}", reason.summary_vi),
                        Style::default().fg(Color::DarkGray),
                    )));
                }
            }
        }

        Paragraph::new(nen_lines)
            .wrap(Wrap { trim: true })
            .render(chunks[0], buf);
        Paragraph::new(tranh_lines)
            .wrap(Wrap { trim: true })
            .render(chunks[1], buf);
    }
}
