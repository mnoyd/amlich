use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct FengShuiScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> FengShuiScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for FengShuiScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let dai_van = self
            .app
            .bundle
            .as_ref()
            .and_then(|bundle| bundle.insight.as_ref())
            .and_then(|insight| insight.dai_van.as_ref());
        let Some(dai_van) = dai_van else {
            render_unavailable(area, buf);
            return;
        };

        match self.mode {
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Length(8),
                    Constraint::Min(16),
                ])
                .split(area);
                render_direction(dai_van, rows[0], buf);
                render_current(dai_van, rows[1], buf);
                render_phase(dai_van, rows[2], buf);
                render_timeline(dai_van, rows[3], buf);
            }
            LayoutMode::Medium | LayoutMode::Large => {
                let rows = Layout::vertical([
                    Constraint::Length(10),
                    Constraint::Length(8),
                    Constraint::Min(16),
                ])
                .split(area);
                let top =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[0]);
                render_direction(dai_van, top[0], buf);
                render_current(dai_van, top[1], buf);
                render_phase(dai_van, rows[1], buf);
                render_timeline(dai_van, rows[2], buf);
            }
        }
    }
}

fn render_unavailable(area: Rect, buf: &mut Buffer) {
    let block = section_block(" Phong Thủy / Đại Vận ", Color::Yellow);
    let inner = block.inner(area);
    block.render(area, buf);
    Paragraph::new(vec![
        Line::from("  Chưa có dữ liệu Đại Vận cho hồ sơ hiện tại."),
        Line::from(""),
        Line::from(Span::styled(
            "  Nhấn [p] ở màn Cá Nhân để nhập năm sinh và giới tính.",
            Style::default().fg(Color::Cyan),
        )),
    ])
    .wrap(Wrap { trim: true })
    .render(inner, buf);
}

fn render_direction(dai_van: &amlich_api::DaiVanInsightDto, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Phong Thủy / Đại Vận ", Color::Yellow);
    let inner = block.inner(area);
    block.render(area, buf);
    Paragraph::new(vec![
        Line::from(vec![
            Span::raw("  Hướng vận: "),
            Span::styled(
                &dai_van.direction,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(format!("  {}", dai_van.direction_meaning.vi)),
        Line::from(format!("  {}", dai_van.start_age)),
    ])
    .wrap(Wrap { trim: true })
    .render(inner, buf);
}

fn render_current(dai_van: &amlich_api::DaiVanInsightDto, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Trụ Hiện Tại ", Color::Cyan);
    let inner = block.inner(area);
    block.render(area, buf);
    let lines = dai_van
        .current_pillar
        .as_ref()
        .map(|pillar| {
            vec![
                Line::from(Span::styled(
                    format!("  Hiện tại: {}", pillar.can_chi),
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(format!(
                    "  Tuổi {}-{} · hành {}",
                    pillar.start_age, pillar.end_age, pillar.element
                )),
                Line::from(format!("  {}", pillar.element_meaning.vi)),
            ]
        })
        .unwrap_or_else(|| vec![Line::from("  Chưa xác định được trụ hiện tại.")]);
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_phase(dai_van: &amlich_api::DaiVanInsightDto, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Ý Nghĩa Các Pha ", Color::Magenta);
    let inner = block.inner(area);
    block.render(area, buf);
    Paragraph::new(format!("  {}", dai_van.phases_meaning.vi))
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_timeline(dai_van: &amlich_api::DaiVanInsightDto, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Toàn Bộ Chu Kỳ & Tương Tác Hành ", Color::Green);
    let inner = block.inner(area);
    block.render(area, buf);
    let mut lines = Vec::new();
    for (index, pillar) in dai_van.all_pillars.iter().enumerate() {
        lines.push(Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::Yellow)),
            Span::styled(
                &pillar.can_chi,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                " · {}-{} tuổi · {}",
                pillar.start_age, pillar.end_age, pillar.element
            )),
        ]));
        lines.push(Line::from(format!("    {}", pillar.element_meaning.vi)));
        if let Some(next) = dai_van.all_pillars.get(index + 1) {
            lines.push(Line::from(Span::styled(
                format!(
                    "    Chuyển pha: {}",
                    describe_interaction(&pillar.element, &next.element)
                ),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }
    if lines.is_empty() {
        lines.push(Line::from("  Chưa có danh sách trụ Đại Vận."));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn describe_interaction(from: &str, to: &str) -> String {
    if generates(from, to) {
        format!("{from} sinh {to} · chuyển vận tương sinh")
    } else if generates(to, from) {
        format!("{to} sinh {from} · vận sau nâng đỡ vận trước")
    } else if controls(from, to) {
        format!("{from} khắc {to} · chuyển vận cần điều tiết")
    } else if controls(to, from) {
        format!("{to} khắc {from} · vận sau tạo lực chế ước")
    } else if from == to {
        format!("{from} tiếp {to} · khí hành được duy trì")
    } else {
        format!("{from} → {to} · chưa có quan hệ trực tiếp")
    }
}

fn generates(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("Kim", "Thủy") | ("Thủy", "Mộc") | ("Mộc", "Hỏa") | ("Hỏa", "Thổ") | ("Thổ", "Kim")
    )
}

fn controls(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("Kim", "Mộc") | ("Mộc", "Thổ") | ("Thổ", "Thủy") | ("Thủy", "Hỏa") | ("Hỏa", "Kim")
    )
}

fn section_block(title: &str, color: Color) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
}
