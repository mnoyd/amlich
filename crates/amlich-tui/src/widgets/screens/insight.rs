use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{
    layout::LayoutMode,
    state::AppState,
    widgets::{
        action_board::ActionBoardWidget, guidance::GuidanceWidget,
        guidance_panel::GuidancePanelWidget, risk::RiskWidget,
    },
};

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
        if self.app.bundle.is_none() {
            Paragraph::new("Chưa có dữ liệu Insight.").render(area, buf);
            return;
        }

        match self.mode {
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(14),
                    Constraint::Length(14),
                    Constraint::Min(12),
                ])
                .split(area);
                render_header(self.app, rows[0], buf);
                ActionBoardWidget::new(self.app, self.mode).render(rows[1], buf);
                GuidanceWidget::new(self.app, self.mode).render(rows[2], buf);
                RiskWidget::new(self.app, self.mode).render(rows[3], buf);
            }
            LayoutMode::Medium | LayoutMode::Large => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(14),
                    Constraint::Min(14),
                ])
                .split(area);
                render_header(self.app, rows[0], buf);
                ActionBoardWidget::new(self.app, self.mode).render(rows[1], buf);
                let support = Layout::horizontal([
                    Constraint::Percentage(45),
                    Constraint::Percentage(30),
                    Constraint::Percentage(25),
                ])
                .split(rows[2]);
                GuidanceWidget::new(self.app, self.mode).render(support[0], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(support[1], buf);
                RiskWidget::new(self.app, self.mode).render(support[2], buf);
            }
        }
    }
}

fn render_header(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Insight / Lập Kế Hoạch ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let inner = block.inner(area);
    block.render(area, buf);
    let summary = app
        .hero_verdict()
        .map(|verdict| verdict.summary)
        .filter(|summary| !summary.trim().is_empty())
        .unwrap_or_else(|| "Chưa có tổng luận hành động cho ngày này.".to_string());
    Paragraph::new(vec![
        Line::from(Span::styled(
            format!("  {summary}"),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from("  Đọc từ quyết định → hành động → lý do → rủi ro; dữ liệu ngày không đổi."),
    ])
    .wrap(Wrap { trim: true })
    .render(inner, buf);
}
