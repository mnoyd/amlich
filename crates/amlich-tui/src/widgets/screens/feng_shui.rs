use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::theme::Theme;
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
        let shell = Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).split(area);
        Paragraph::new(Line::from(vec![
            Span::styled(
                "▶ FengShui",
                Theme::accent_warn().add_modifier(Modifier::BOLD),
            ),
            Span::styled(" · Tứ mệnh, hướng và đại vận", Theme::text_dim()),
        ]))
        .render(shell[0], buf);

        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(shell[1], buf);
            return;
        };
        let Some(insight) = &bundle.insight else {
            Paragraph::new("Chưa có dữ liệu insight.").render(shell[1], buf);
            return;
        };

        if insight.tu_menh.is_none() && insight.dai_van.is_none() {
            let block = Block::default()
                .title(" Phong Thủy ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray));
            let text = "Chưa cấu hình hồ sơ cá nhân.\n\nCần birth year + gender để tính Tứ Mệnh và Đại Vận.";
            Paragraph::new(text).block(block).render(shell[1], buf);
            return;
        }

        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let rows =
                    Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(shell[1]);
                let top =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[0]);
                let bottom =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[1]);

                render_kua(insight, top[0], buf);
                render_directions(insight, top[1], buf);
                render_dai_van(insight, bottom[0], buf);
                render_compass(insight, bottom[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(9),
                    Constraint::Min(10),
                    Constraint::Min(14),
                ])
                .split(shell[1]);
                render_kua(insight, rows[0], buf);
                render_directions(insight, rows[1], buf);
                render_dai_van(insight, rows[2], buf);
            }
        }
    }
}

fn render_kua(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tứ Mệnh / Kua ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(tm) = &insight.tu_menh else { return };
    let lines = vec![
        Line::from(vec![
            Span::raw("  Quẻ số: "),
            Span::styled(
                tm.kua.to_string(),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Quẻ: "),
            Span::styled(&tm.trigram.vi, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("  Nhóm: "),
            Span::styled(&tm.group, Style::default().fg(Color::Green)),
        ]),
        Line::from(format!("   \u{2514} {}", tm.group_meaning.vi)),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Hướng mệnh: "),
            Span::styled(&tm.direction.vi, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(format!("   \u{2514} {}", tm.meaning.vi)),
    ];
    Paragraph::new(lines).render(inner, buf);
}

fn render_directions(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Hướng Tốt / Xấu ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(tm) = &insight.tu_menh else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(Span::styled(
        "  Hướng tốt:",
        Style::default().fg(Color::Green),
    )));
    for d in &tm.favorable_directions {
        lines.push(Line::from(vec![
            Span::styled("   \u{2605} ", Style::default().fg(Color::Green)),
            Span::raw(d.as_str()),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Hướng xấu:",
        Style::default().fg(Color::Red),
    )));
    for d in &tm.unfavorable_directions {
        lines.push(Line::from(vec![
            Span::styled("   \u{2716} ", Style::default().fg(Color::Red)),
            Span::raw(d.as_str()),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_dai_van(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Đại Vận ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(dv) = &insight.dai_van else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(vec![
        Span::raw("  Hướng vận: "),
        Span::styled(&dv.direction, Style::default().fg(Color::Yellow)),
    ]));
    lines.push(Line::from(format!(
        "   \u{2514} {}",
        dv.direction_meaning.vi
    )));

    if let Some(cur) = &dv.current_pillar {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled(
                format!("  \u{25B6} {} ", cur.can_chi),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "({}-{} tuổi) ",
                cur.start_age as u32, cur.end_age as u32
            )),
            Span::styled(&cur.element, Style::default().fg(Color::Yellow)),
        ]));
        lines.push(Line::from(format!("    {}", cur.element_meaning.vi)));
    }

    lines.push(Line::from(""));
    for p in &dv.all_pillars {
        let is_cur = dv
            .current_pillar
            .as_ref()
            .map(|c| c.index == p.index)
            .unwrap_or(false);
        let marker = if is_cur { "\u{25C4}" } else { " " };
        let style = if is_cur {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        lines.push(Line::from(Span::styled(
            format!(
                "  {}. {:<10} ({:>2}-{:>2}) {:>4} {marker}",
                p.index, p.can_chi, p.start_age as u32, p.end_age as u32, p.element
            ),
            style,
        )));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_compass(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" La Bàn Hướng ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(tm) = &insight.tu_menh else { return };
    let good: Vec<&str> = tm.favorable_directions.iter().map(|s| s.as_str()).collect();
    let bad: Vec<&str> = tm
        .unfavorable_directions
        .iter()
        .map(|s| s.as_str())
        .collect();

    let ds = |name: &str| -> Style {
        if good.iter().any(|d| d.contains(name)) {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        } else if bad.iter().any(|d| d.contains(name)) {
            Style::default().fg(Color::Red)
        } else {
            Style::default().fg(Color::DarkGray)
        }
    };
    let mk = |name: &str| -> &str {
        if good.iter().any(|d| d.contains(name)) {
            "\u{2605}"
        } else if bad.iter().any(|d| d.contains(name)) {
            "\u{2716}"
        } else {
            "\u{00B7}"
        }
    };

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("          "),
            Span::styled(format!("{} Bắc", mk("Bắc")), ds("Bắc")),
        ]),
        Line::from(vec![
            Span::raw("     "),
            Span::styled(format!("{} TB", mk("Tây Bắc")), ds("Tây Bắc")),
            Span::raw("    |    "),
            Span::styled(format!("ĐB {}", mk("Đông Bắc")), ds("Đông Bắc")),
        ]),
        Line::from("             |"),
        Line::from(vec![
            Span::raw("    "),
            Span::styled(format!("{} Tây", mk("Tây")), ds("Tây")),
            Span::raw(" \u{2014}\u{2014}\u{25CF}\u{2014}\u{2014} "),
            Span::styled(format!("Đông {}", mk("Đông")), ds("Đông")),
        ]),
        Line::from("             |"),
        Line::from(vec![
            Span::raw("     "),
            Span::styled(format!("{} TN", mk("Tây Nam")), ds("Tây Nam")),
            Span::raw("    |    "),
            Span::styled(format!("ĐN {}", mk("Đông Nam")), ds("Đông Nam")),
        ]),
        Line::from(vec![
            Span::raw("          "),
            Span::styled(format!("{} Nam", mk("Nam")), ds("Nam")),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(" \u{2605} Tốt ", Style::default().fg(Color::Green)),
            Span::styled(" \u{2716} Xấu", Style::default().fg(Color::Red)),
        ]),
    ];
    Paragraph::new(lines).render(inner, buf);
}
