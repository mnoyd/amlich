use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::widgets::{
    direction_panel::DirectionPanelWidget, guidance::GuidanceWidget, inspection::InspectionWidget,
    risk::RiskWidget, scholarly::ScholarlyWidget, stars_panel::StarsPanelWidget,
};
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
        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let rows = Layout::vertical([
                    Constraint::Length(7),
                    Constraint::Percentage(25),
                    Constraint::Percentage(15),
                    Constraint::Length(9),
                    Constraint::Percentage(18),
                    Constraint::Percentage(17),
                    Constraint::Min(7),
                ])
                .split(area);

                render_scholar_verdict(self.app, rows[0], buf);
                GuidanceWidget::new(self.app, self.mode).render(rows[1], buf);
                RiskWidget::new(self.app, self.mode).render(rows[2], buf);

                let application =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[3]);
                DirectionPanelWidget::new(self.app, self.mode).render(application[0], buf);
                render_timing_summary(self.app, rows[3], application[1], buf);

                let interpretation =
                    Layout::horizontal([Constraint::Percentage(52), Constraint::Percentage(48)])
                        .split(rows[4]);
                ScholarlyWidget::new(self.app, self.mode).render(interpretation[0], buf);
                StarsPanelWidget::new(self.app, self.mode).render(interpretation[1], buf);

                InspectionWidget::new(self.app, self.mode).render(rows[5], buf);
                render_layer_context(self.app, rows[6], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Length(8),
                    Constraint::Min(12),
                    Constraint::Min(8),
                    Constraint::Min(8),
                    Constraint::Min(8),
                    Constraint::Min(9),
                    Constraint::Min(8),
                    Constraint::Min(7),
                ])
                .split(area);

                render_scholar_verdict(self.app, rows[0], buf);
                GuidanceWidget::new(self.app, self.mode).render(rows[1], buf);
                RiskWidget::new(self.app, self.mode).render(rows[2], buf);
                DirectionPanelWidget::new(self.app, self.mode).render(rows[3], buf);
                render_timing_summary(self.app, rows[4], rows[4], buf);
                ScholarlyWidget::new(self.app, self.mode).render(rows[5], buf);
                StarsPanelWidget::new(self.app, self.mode).render(rows[6], buf);
                InspectionWidget::new(self.app, self.mode).render(rows[7], buf);
            }
        }
    }
}

fn render_scholar_verdict(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tổng Luận Hôm Nay ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    if let Some(verdict) = app.hero_verdict() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                verdict.summary,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        if let Some(positive) = verdict.strongest_positive {
            lines.push(Line::from(vec![
                Span::styled("  Nên: ", Style::default().fg(Color::Green)),
                Span::raw(positive),
            ]));
        }
        if let Some(negative) = verdict.strongest_negative {
            lines.push(Line::from(vec![
                Span::styled("  Tránh: ", Style::default().fg(Color::Red)),
                Span::raw(negative),
            ]));
        }
    } else {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Chưa có đủ dữ liệu để tổng luận hôm nay.",
                Style::default().fg(Color::Gray),
            ),
        ]));
    }

    if let Some(support) = app.scholar_verdict_support() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(support.support_line, Style::default().fg(Color::Cyan)),
        ]));
        if let Some(layer_note) = support.layer_note {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(layer_note, Style::default().fg(Color::DarkGray)),
            ]));
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_timing_summary(app: &AppState, area: Rect, target: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Giờ / Khung Hành Động ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(target);
    block.render(target, buf);

    let mut lines = Vec::new();
    if let Some(timing) = app.hours_verdict() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(timing.summary, Style::default().fg(Color::Green)),
        ]));

        for window in timing.top_windows.iter().take(3) {
            lines.push(Line::from(vec![
                Span::styled("  ★ ", Style::default().fg(Color::Green)),
                Span::raw(window.clone()),
            ]));
        }

        if let Some(caution) = timing.caution {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(caution, Style::default().fg(Color::Yellow)),
            ]));
        }
    } else {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Chưa có dữ liệu giờ tốt để tóm tắt.",
                Style::default().fg(Color::Gray),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);

    let _ = area;
}

fn render_layer_context(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Lớp Đọc Ngày ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let layers = app.recommendation_layers();
    let mut lines = Vec::new();
    if layers.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("Chưa có lớp khuyến nghị.", Style::default().fg(Color::Gray)),
        ]));
    } else {
        for layer in layers.iter().take(2) {
            lines.push(Line::from(vec![
                Span::raw("  • "),
                Span::styled(
                    format!("{}: {}", layer.label, layer.summary),
                    Style::default().fg(Color::White),
                ),
            ]));
        }
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Nhấn e để xem chứng cứ và metadata sâu hơn.",
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}
