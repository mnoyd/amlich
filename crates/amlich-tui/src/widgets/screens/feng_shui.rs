use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::{ui_prefs::VerbosityMode, AppState}, widgets::direction_panel::DirectionPanelWidget};

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
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(area, buf);
            return;
        };
        let Some(insight) = &bundle.insight else {
            Paragraph::new("Chưa có dữ liệu insight.").render(area, buf);
            return;
        };

        let profile = self
            .app
            .profile_availability_summary()
            .expect("bundle exists for feng shui");

        match (profile.has_personal_overlay, self.app.active_verbosity(), self.mode) {
            (true, VerbosityMode::Compact, LayoutMode::Small) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(9),
                    Constraint::Min(10),
                ])
                .split(area);
                render_profile_verdict(self.app, rows[0], buf);
                render_kua(insight, rows[1], buf);
                render_directions(insight, rows[2], buf);
            }
            (true, VerbosityMode::Compact, _) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(9),
                    Constraint::Min(10),
                ])
                .split(area);
                render_profile_verdict(self.app, rows[0], buf);
                let middle =
                    Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)])
                        .split(rows[1]);
                DirectionPanelWidget::new(self.app, self.mode).render(middle[0], buf);
                render_kua(insight, middle[1], buf);
                render_directions(insight, rows[2], buf);
            }
            (true, VerbosityMode::Verbose, _) => {
            let rows = Layout::vertical([
                Constraint::Length(6),
                Constraint::Length(9),
                Constraint::Min(16),
            ])
            .split(area);
            render_profile_verdict(self.app, rows[0], buf);

            let middle =
                Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)])
                    .split(rows[1]);
            DirectionPanelWidget::new(self.app, self.mode).render(middle[0], buf);
            render_kua(insight, middle[1], buf);

            match self.mode {
                LayoutMode::Large | LayoutMode::Medium => {
                    let bottom = Layout::horizontal([
                        Constraint::Percentage(34),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ])
                    .split(rows[2]);
                    render_directions(insight, bottom[0], buf);
                    render_dai_van(insight, bottom[1], buf);
                    render_dai_van_timeline(insight, bottom[2], buf);
                }
                LayoutMode::Small => {
                    let bottom = Layout::vertical([
                        Constraint::Length(8),
                        Constraint::Length(10),
                        Constraint::Min(10),
                    ])
                    .split(rows[2]);
                    render_directions(insight, bottom[0], buf);
                    render_dai_van(insight, bottom[1], buf);
                    render_dai_van_timeline(insight, bottom[2], buf);
                }
            }
            }
            (false, _, _) => {
            let rows = Layout::vertical([
                Constraint::Length(6),
                Constraint::Min(9),
                Constraint::Min(6),
            ])
            .split(area);
            render_profile_verdict(self.app, rows[0], buf);
            DirectionPanelWidget::new(self.app, self.mode).render(rows[1], buf);
            render_scope_note(&profile.note, rows[2], buf);
            }
        }
    }
}

fn render_profile_verdict(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Nhận Định ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let profile = app
        .profile_availability_summary()
        .expect("bundle exists for profile verdict");
    let direction = app.direction_verdict();

    let mut lines = Vec::new();
    let verdict = if profile.has_personal_overlay {
        "Màn hình này đang ghép hướng theo ngày với lớp cá nhân hóa."
    } else {
        "Hiện chỉ có hướng theo ngày; chưa đủ dữ liệu để luận phong thủy bản mệnh."
    };
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(verdict, Style::default().fg(Color::Yellow)),
    ]));
    if let Some(direction) = direction {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(direction.summary, Style::default().fg(Color::Cyan)),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_scope_note(note: &str, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Giới Hạn Diễn Giải ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    Paragraph::new(vec![
        Line::from(vec![
            Span::raw("  "),
            Span::styled(note, Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Cần birth year + gender để mở Tứ Mệnh và Đại Vận.",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ])
    .wrap(Wrap { trim: true })
    .render(inner, buf);
}

fn render_kua(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Lớp Cá Nhân ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(tm) = &insight.tu_menh else {
        Paragraph::new("  Chưa có dữ liệu Tứ Mệnh.").render(inner, buf);
        return;
    };
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
        Line::from(format!("  {}", tm.group_meaning.vi)),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Hướng mệnh: "),
            Span::styled(&tm.direction.vi, Style::default().fg(Color::Yellow)),
        ]),
        Line::from(format!("  {}", tm.meaning.vi)),
    ];
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_directions(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Hướng Tốt / Xấu Theo Mệnh ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(tm) = &insight.tu_menh else {
        Paragraph::new("  Chưa có dữ liệu hướng cá nhân.").render(inner, buf);
        return;
    };

    let mut lines: Vec<Line<'_>> = vec![];
    lines.push(Line::from(Span::styled(
        "  Hướng tốt:",
        Style::default().fg(Color::Green),
    )));
    for direction in &tm.favorable_directions {
        lines.push(Line::from(vec![
            Span::styled("   ★ ", Style::default().fg(Color::Green)),
            Span::raw(direction.as_str()),
        ]));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Hướng xấu:",
        Style::default().fg(Color::Red),
    )));
    for direction in &tm.unfavorable_directions {
        lines.push(Line::from(vec![
            Span::styled("   ✖ ", Style::default().fg(Color::Red)),
            Span::raw(direction.as_str()),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_dai_van(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Chu Kỳ / Đại Vận ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(dv) = &insight.dai_van else {
        Paragraph::new("  Chưa có dữ liệu Đại Vận.").render(inner, buf);
        return;
    };

    let mut lines: Vec<Line<'_>> = vec![];
    lines.push(Line::from(vec![
        Span::raw("  Hướng vận: "),
        Span::styled(&dv.direction, Style::default().fg(Color::Yellow)),
    ]));
    lines.push(Line::from(format!("  {}", dv.direction_meaning.vi)));
    lines.push(Line::from(vec![
        Span::raw("  Khởi vận: "),
        Span::styled(&dv.start_age, Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(format!("  {}", dv.phases_meaning.vi)));

    if let Some(current) = &dv.current_pillar {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Hiện tại: "),
            Span::styled(
                format!(
                    "{} · {}-{} tuổi",
                    current.can_chi, current.start_age, current.end_age
                ),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(format!("  {}", current.element_meaning.vi)));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_dai_van_timeline(insight: &amlich_api::DayInsightDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Các Vận Kế Tiếp ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(dv) = &insight.dai_van else {
        Paragraph::new("  Chưa có timeline đại vận.").render(inner, buf);
        return;
    };

    let mut lines: Vec<Line<'_>> = vec![];
    for pillar in dv.all_pillars.iter().take(4) {
        lines.push(Line::from(vec![
            Span::styled("  • ", Style::default().fg(Color::Yellow)),
            Span::styled(
                format!("{}", pillar.can_chi),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" · {}-{} tuổi", pillar.start_age, pillar.end_age)),
        ]));
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(
                format!("{} · {}", pillar.element, pillar.element_meaning.vi),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}
