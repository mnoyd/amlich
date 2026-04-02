use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::widgets::{
    direction_panel::DirectionPanelWidget, guidance::GuidanceWidget,
    guidance_panel::GuidancePanelWidget, inspection::InspectionWidget, risk::RiskWidget,
    scholarly::ScholarlyWidget, stars_panel::StarsPanelWidget, tietkhi::TietKhiWidget,
};
use crate::{layout::LayoutMode, state::{ui_prefs::VerbosityMode, AppState}, widgets::travel::TravelWidget};

pub struct DayDetailScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> DayDetailScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for DayDetailScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match (self.mode, self.app.active_verbosity()) {
            (LayoutMode::Large | LayoutMode::Medium, VerbosityMode::Verbose) => {
                let rows = Layout::vertical([
                    Constraint::Length(7),
                    Constraint::Length(10),
                    Constraint::Length(8),
                    Constraint::Length(9),
                    Constraint::Length(9),
                    Constraint::Length(9),
                    Constraint::Length(8),
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
                TravelWidget::new(self.app, self.mode).render(application[1], buf);

                GuidancePanelWidget::new(self.app, self.mode).render(rows[4], buf);

                let interpretation =
                    Layout::horizontal([Constraint::Percentage(52), Constraint::Percentage(48)])
                        .split(rows[5]);
                ScholarlyWidget::new(self.app, self.mode).render(interpretation[0], buf);
                StarsPanelWidget::new(self.app, self.mode).render(interpretation[1], buf);

                InspectionWidget::new(self.app, self.mode).render(rows[6], buf);
                render_detail_footer(self.app, rows[7], buf);
            }
            (LayoutMode::Small, VerbosityMode::Verbose) => {
                let rows = Layout::vertical([
                    Constraint::Length(8),
                    Constraint::Length(12),
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Length(9),
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Min(7),
                ])
                .split(area);

                render_scholar_verdict(self.app, rows[0], buf);
                GuidanceWidget::new(self.app, self.mode).render(rows[1], buf);
                RiskWidget::new(self.app, self.mode).render(rows[2], buf);
                DirectionPanelWidget::new(self.app, self.mode).render(rows[3], buf);
                TravelWidget::new(self.app, self.mode).render(rows[4], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[5], buf);
                ScholarlyWidget::new(self.app, self.mode).render(rows[6], buf);
                StarsPanelWidget::new(self.app, self.mode).render(rows[7], buf);
                InspectionWidget::new(self.app, self.mode).render(rows[8], buf);
            }
            (LayoutMode::Small, VerbosityMode::Compact) => {
                let rows = Layout::vertical([
                    Constraint::Length(8),
                    Constraint::Length(12),
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Min(8),
                ])
                .split(area);

                render_scholar_verdict(self.app, rows[0], buf);
                GuidanceWidget::new(self.app, self.mode).render(rows[1], buf);
                RiskWidget::new(self.app, self.mode).render(rows[2], buf);
                DirectionPanelWidget::new(self.app, self.mode).render(rows[3], buf);
                TravelWidget::new(self.app, self.mode).render(rows[4], buf);
            }
            (_, VerbosityMode::Compact) => {
                let rows = Layout::vertical([
                    Constraint::Length(7),
                    Constraint::Length(10),
                    Constraint::Length(8),
                    Constraint::Length(9),
                    Constraint::Min(10),
                ])
                .split(area);

                render_scholar_verdict(self.app, rows[0], buf);
                GuidanceWidget::new(self.app, self.mode).render(rows[1], buf);
                RiskWidget::new(self.app, self.mode).render(rows[2], buf);

                let application =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[3]);
                DirectionPanelWidget::new(self.app, self.mode).render(application[0], buf);
                TravelWidget::new(self.app, self.mode).render(application[1], buf);

                GuidancePanelWidget::new(self.app, self.mode).render(rows[4], buf);
            }
        }
    }
}

fn render_scholar_verdict(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Chi Tiết Ngày ")
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

    if let Some(seasonal) = app.seasonal_verdict() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                format!("Ảnh hưởng mùa: {}", seasonal.implication),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_detail_footer(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tầng Dữ Liệu / Tiết Khí ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let columns = Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)]).split(inner);
    TietKhiWidget::new(app, LayoutMode::Small).render(columns[0], buf);
    render_layer_context(app, columns[1], buf);
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
    if let Some(seasonal) = app.seasonal_verdict() {
        lines.push(Line::from(vec![
            Span::raw("  Mùa: "),
            Span::styled(seasonal.headline, Style::default().fg(Color::Cyan)),
        ]));
        for line in seasonal.application_lines.iter().take(1) {
            lines.push(Line::from(vec![
                Span::raw("  • "),
                Span::styled(line.clone(), Style::default().fg(Color::White)),
            ]));
        }
        lines.push(Line::from(""));
    }

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
