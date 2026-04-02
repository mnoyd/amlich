use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    widgets::Widget,
};

use crate::widgets::{
    action_board::ActionBoardWidget, almanac::AlmanacGridWidget, event_summary::EventSummaryWidget,
    hero::HeroWidget, mini_calendar::MiniCalendarWidget, timeline::TimelineWidget,
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
            (LayoutMode::Small, VerbosityMode::Compact) => render_small_compact(self.app, area, buf),
            (LayoutMode::Small, VerbosityMode::Verbose) => render_small_verbose(self.app, area, buf),
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
    ])
    .split(area);

    HeroWidget::new(app, LayoutMode::Small).render(rows[0], buf);
    render_today_verdict(app, rows[1], buf);
    EventSummaryWidget::new(app, LayoutMode::Small).render(rows[2], buf);
    ActionBoardWidget::new(app, LayoutMode::Small).render(rows[3], buf);
}

fn render_small_verbose(app: &AppState, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Length(7),
        Constraint::Length(9),
        Constraint::Min(8),
        Constraint::Length(5),
    ])
    .split(area);

    HeroWidget::new(app, LayoutMode::Small).render(rows[0], buf);
    render_today_verdict(app, rows[1], buf);
    EventSummaryWidget::new(app, LayoutMode::Small).render(rows[2], buf);
    MiniCalendarWidget::new(app, LayoutMode::Small).render(rows[3], buf);
    ActionBoardWidget::new(app, LayoutMode::Small).render(rows[4], buf);
    TimelineWidget::new(app, LayoutMode::Small).render(rows[5], buf);
}

fn render_standard_compact(app: &AppState, mode: LayoutMode, area: Rect, buf: &mut Buffer) {
    let rows = Layout::vertical([
        Constraint::Length(10),
        Constraint::Length(6),
        Constraint::Min(12),
        Constraint::Length(5),
    ])
    .split(area);

    HeroWidget::new(app, mode).render(rows[0], buf);
    render_today_verdict(app, rows[1], buf);

    let middle = Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)]).split(rows[2]);
    ActionBoardWidget::new(app, mode).render(middle[0], buf);
    EventSummaryWidget::new(app, mode).render(middle[1], buf);

    TimelineWidget::new(app, mode).render(rows[3], buf);
}

fn render_standard_verbose(app: &AppState, mode: LayoutMode, area: Rect, buf: &mut Buffer) {
    let chunks = Layout::vertical([
        Constraint::Length(6),
        Constraint::Length(20),
        Constraint::Length(5),
    ])
    .split(area);

    render_today_verdict(app, chunks[0], buf);

    let top_chunks = Layout::horizontal([Constraint::Percentage(60), Constraint::Percentage(40)]).split(chunks[1]);

    let left_chunks = Layout::vertical([Constraint::Length(10), Constraint::Min(8)]).split(top_chunks[0]);

    let right_chunks = Layout::vertical([Constraint::Length(9), Constraint::Length(4), Constraint::Min(4)]).split(top_chunks[1]);

    HeroWidget::new(app, mode).render(left_chunks[0], buf);
    AlmanacGridWidget::new(app, mode).render(left_chunks[1], buf);

    MiniCalendarWidget::new(app, mode).render(right_chunks[0], buf);
    EventSummaryWidget::new(app, mode).render(right_chunks[1], buf);
    ActionBoardWidget::new(app, mode).render(right_chunks[2], buf);

    TimelineWidget::new(app, mode).render(chunks[2], buf);
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

fn render_today_verdict(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Hôm Nay ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    if let Some(verdict) = app.hero_verdict() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(verdict.summary, Style::default().fg(Color::Yellow)),
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
        lines.push(Line::from("  Chưa có tổng luận cho ngày này."));
    }

    if let Some(timing) = app.hours_verdict() {
        if let Some(window) = timing.top_windows.first() {
            lines.push(Line::from(vec![
                Span::styled("  Giờ đẹp: ", Style::default().fg(Color::Green)),
                Span::raw(window.clone()),
            ]));
        }
    }

    if let Some(seasonal) = app.seasonal_verdict() {
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
