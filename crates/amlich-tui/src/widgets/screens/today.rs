use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Widget,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::widgets::{
    action_board::ActionBoardWidget, almanac::AlmanacGridWidget,
    direction_panel::DirectionPanelWidget, event_summary::EventSummaryWidget,
    guidance::GuidanceWidget, guidance_panel::GuidancePanelWidget, hero::HeroWidget,
    mini_calendar::MiniCalendarWidget, risk::RiskWidget, timeline::TimelineWidget,
    tietkhi::TietKhiWidget, travel::TravelWidget,
};
use crate::{
    layout::LayoutMode,
    state::{ui_prefs::VerbosityMode, AppState},
};

pub struct TodayScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> TodayScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for TodayScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match (self.mode, self.app.active_verbosity()) {
            (LayoutMode::Small, VerbosityMode::Compact) => {
                render_small_compact(self.app, area, buf)
            }
            (LayoutMode::Small, VerbosityMode::Verbose) => {
                render_small_verbose(self.app, area, buf)
            }
            (_, VerbosityMode::Compact) => render_standard_compact(self.app, self.mode, area, buf),
            (_, VerbosityMode::Verbose) => render_standard_verbose(self.app, self.mode, area, buf),
        }
    }
}

fn render_small_compact(app: &AppState, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Length(7),
        Constraint::Min(8),
        Constraint::Length(10),
        Constraint::Length(8),
        Constraint::Min(8),
    ])
    .split(area);

    HeroWidget::new(app, LayoutMode::Small).render(rows[0], buf);
    render_overview_verdict(app, rows[1], buf);
    EventSummaryWidget::new(app, LayoutMode::Small).render(rows[2], buf);
    ActionBoardWidget::new(app, LayoutMode::Small).render(rows[3], buf);
    GuidanceWidget::new(app, LayoutMode::Small).render(rows[4], buf);
    RiskWidget::new(app, LayoutMode::Small).render(rows[5], buf);
    render_direction_and_travel_compact(app, rows[6], buf);
}

fn render_small_verbose(app: &AppState, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Length(7),
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(5),
        Constraint::Length(12),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Length(8),
        Constraint::Min(7),
    ])
    .split(area);

    HeroWidget::new(app, LayoutMode::Small).render(rows[0], buf);
    render_overview_verdict(app, rows[1], buf);
    EventSummaryWidget::new(app, LayoutMode::Small).render(rows[2], buf);
    MiniCalendarWidget::new(app, LayoutMode::Small).render(rows[3], buf);
    ActionBoardWidget::new(app, LayoutMode::Small).render(rows[4], buf);
    TimelineWidget::new(app, LayoutMode::Small).render(rows[5], buf);
    GuidanceWidget::new(app, LayoutMode::Small).render(rows[6], buf);
    RiskWidget::new(app, LayoutMode::Small).render(rows[7], buf);
    DirectionPanelWidget::new(app, LayoutMode::Small).render(rows[8], buf);
    TravelWidget::new(app, LayoutMode::Small).render(rows[9], buf);
    render_detail_footer(app, rows[10], buf);
}

fn render_standard_compact(app: &AppState, mode: LayoutMode, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Min(12),
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(8),
        Constraint::Min(10),
    ])
    .split(area);

    HeroWidget::new(app, mode).render(rows[0], buf);
    render_overview_verdict(app, rows[1], buf);

    let middle =
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).split(rows[2]);
    ActionBoardWidget::new(app, mode).render(middle[0], buf);
    EventSummaryWidget::new(app, mode).render(middle[1], buf);

    TimelineWidget::new(app, mode).render(rows[3], buf);

    GuidanceWidget::new(app, mode).render(rows[4], buf);
    RiskWidget::new(app, mode).render(rows[5], buf);
    GuidancePanelWidget::new(app, mode).render(rows[6], buf);
}

fn render_standard_verbose(app: &AppState, mode: LayoutMode, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::vertical([
        Constraint::Length(6),
        Constraint::Length(20),
        Constraint::Length(5),
        Constraint::Length(10),
        Constraint::Length(8),
        Constraint::Length(9),
        Constraint::Min(7),
    ])
    .split(area);

    render_overview_verdict(app, chunks[0], buf);

    let top_chunks = Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(chunks[1]);

    let left_chunks =
        Layout::vertical([Constraint::Length(10), Constraint::Min(8)]).split(top_chunks[0]);

    let right_chunks = Layout::vertical([
        Constraint::Length(9),
        Constraint::Length(4),
        Constraint::Min(4),
    ])
    .split(top_chunks[1]);

    HeroWidget::new(app, mode).render(left_chunks[0], buf);
    AlmanacGridWidget::new(app, mode).render(left_chunks[1], buf);

    MiniCalendarWidget::new(app, mode).render(right_chunks[0], buf);
    EventSummaryWidget::new(app, mode).render(right_chunks[1], buf);
    ActionBoardWidget::new(app, mode).render(right_chunks[2], buf);

    TimelineWidget::new(app, mode).render(chunks[2], buf);

    GuidanceWidget::new(app, mode).render(chunks[3], buf);
    RiskWidget::new(app, mode).render(chunks[4], buf);

    let application =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[5]);
    DirectionPanelWidget::new(app, mode).render(application[0], buf);
    TravelWidget::new(app, mode).render(application[1], buf);

    render_detail_footer(app, chunks[6], buf);
}

fn render_direction_and_travel_compact(app: &AppState, area: Rect, buf: &mut Buffer) {
    let application =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    DirectionPanelWidget::new(app, LayoutMode::Small).render(application[0], buf);
    TravelWidget::new(app, LayoutMode::Small).render(application[1], buf);
}

fn render_detail_footer(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tầng Dữ Liệu / Tiết Khí ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let columns =
        Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)]).split(inner);
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

fn render_overview_verdict(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tổng Quan ")
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
                "Chưa có dữ liệu tổng luận.",
                Style::default().fg(Color::Gray),
            ),
        ]));
    }

    if let Some(timing) = app.hours_verdict() {
        if let Some(window) = timing.top_windows.first() {
            lines.push(Line::from(vec![
                Span::styled("  Giờ đẹp: ", Style::default().fg(Color::Green)),
                Span::raw(window.clone()),
            ]));
        }
    }

    if let Some(support) = app.day_detail_verdict_support() {
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
            Span::styled("  Tiết khí: ", Style::default().fg(Color::Cyan)),
            Span::raw(seasonal.headline),
        ]));
    }

    if let Some(event_line) = event_headline(app) {
        lines.push(Line::from(""));
        lines.push(event_line);
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn event_headline(app: &AppState) -> Option<Line<'static>> {
    let bundle = app.bundle.as_ref()?;

    if let Some(insight) = &bundle.insight {
        if let Some(festival) = &insight.festival {
            return Some(Line::from(vec![
                Span::styled("  Sự kiện: ", Style::default().fg(Color::Yellow)),
                Span::raw(festival.names.vi.join(" / ")),
            ]));
        }

        if let Some(holiday) = &insight.holiday {
            return Some(Line::from(vec![
                Span::styled("  Sự kiện: ", Style::default().fg(Color::Yellow)),
                Span::raw(holiday.names.vi.join(" / ")),
            ]));
        }
    }

    None
}
