use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::{ui_prefs::VerbosityMode, AppState}, widgets::tietkhi::TietKhiWidget};

pub struct SolarTermsScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> SolarTermsScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for SolarTermsScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(area, buf);
            return;
        };

        match (self.mode, self.app.active_verbosity()) {
            (_, VerbosityMode::Compact) => {
                let rows = Layout::vertical([
                    Constraint::Length(7),
                    Constraint::Length(9),
                    Constraint::Min(14),
                ])
                .split(area);
                render_seasonal_verdict(self.app, rows[0], buf);
                TietKhiWidget::new(self.app, self.mode).render(rows[1], buf);
                render_compact_seasonal_actions(bundle, rows[2], buf);
            }
            (LayoutMode::Large | LayoutMode::Medium, VerbosityMode::Verbose) => {
                let rows = Layout::vertical([
                    Constraint::Length(7),
                    Constraint::Length(9),
                    Constraint::Min(10),
                ])
                .split(area);
                render_seasonal_verdict(self.app, rows[0], buf);
                TietKhiWidget::new(self.app, self.mode).render(rows[1], buf);

                let bottom = Layout::horizontal([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(rows[2]);
                render_astronomy(bundle, bottom[0], buf);
                render_agriculture(bundle, bottom[1], buf);
                render_health(bundle, bottom[2], buf);
            }
            (LayoutMode::Small, VerbosityMode::Verbose) => {
                let rows = Layout::vertical([
                    Constraint::Length(7),
                    Constraint::Length(9),
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Min(8),
                ])
                .split(area);
                render_seasonal_verdict(self.app, rows[0], buf);
                TietKhiWidget::new(self.app, self.mode).render(rows[1], buf);
                render_astronomy(bundle, rows[2], buf);
                render_agriculture(bundle, rows[3], buf);
                render_health(bundle, rows[4], buf);
            }
        }
    }
}

fn render_compact_seasonal_actions(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Nhịp Mùa / Ứng Dụng Nhanh ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        Paragraph::new("  Chưa có dữ liệu tiết khí.").render(inner, buf);
        return;
    };
    let Some(tiet_khi) = &insight.tiet_khi else {
        Paragraph::new("  Chưa có dữ liệu tiết khí.").render(inner, buf);
        return;
    };

    let mut lines: Vec<Line<'_>> = vec![];
    if let Some(current) = &bundle.tiet_khi {
        lines.push(Line::from(vec![
            Span::raw("  Kinh độ: "),
            Span::styled(
                format!("{:.1}°", current.current_longitude),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    if let Some(item) = tiet_khi.agriculture.vi.first() {
        lines.push(Line::from(vec![
            Span::styled("  • Việc mùa: ", Style::default().fg(Color::Green)),
            Span::styled(item.clone(), Style::default().fg(Color::White)),
        ]));
    }
    if let Some(item) = tiet_khi.health.vi.first() {
        lines.push(Line::from(vec![
            Span::styled("  • Dưỡng sinh: ", Style::default().fg(Color::Cyan)),
            Span::styled(item.clone(), Style::default().fg(Color::White)),
        ]));
    }
    let weather = tiet_khi.weather.vi.trim();
    if !weather.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  Thời khí: ", Style::default().fg(Color::Gray)),
            Span::styled(weather.to_string(), Style::default().fg(Color::White)),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_seasonal_verdict(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Nhận Định Theo Mùa ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines = Vec::new();
    if let Some(verdict) = app.seasonal_verdict() {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(verdict.headline, Style::default().fg(Color::Cyan)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(verdict.implication, Style::default().fg(Color::Yellow)),
        ]));
        for line in verdict.application_lines.iter().take(2) {
            lines.push(Line::from(vec![
                Span::raw("  • "),
                Span::styled(line.clone(), Style::default().fg(Color::White)),
            ]));
        }
    } else {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Chưa có dữ liệu tiết khí.",
                Style::default().fg(Color::Gray),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_astronomy(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Thiên Văn / Thời Khí ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        return;
    };
    let Some(tiet_khi) = &insight.tiet_khi else {
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![
        Line::from(vec![
            Span::raw("  Thiên văn: "),
            Span::styled(
                tiet_khi.astronomy.vi.clone(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Thời tiết: "),
            Span::styled(
                tiet_khi.weather.vi.clone(),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    if let Some(current) = &bundle.tiet_khi {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Kinh độ hiện tại: "),
            Span::styled(
                format!("{:.1}°", current.current_longitude),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_agriculture(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Ứng Dụng Theo Mùa ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        return;
    };
    let Some(tiet_khi) = &insight.tiet_khi else {
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];
    for item in tiet_khi.agriculture.vi.iter().take(4) {
        lines.push(Line::from(vec![
            Span::raw("  • "),
            Span::styled(item.clone(), Style::default().fg(Color::Green)),
        ]));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_health(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Dưỡng Sinh / Sức Khỏe ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        return;
    };
    let Some(tiet_khi) = &insight.tiet_khi else {
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];
    for item in tiet_khi.health.vi.iter().take(4) {
        lines.push(Line::from(vec![
            Span::raw("  • "),
            Span::styled(item.clone(), Style::default().fg(Color::Cyan)),
        ]));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}
