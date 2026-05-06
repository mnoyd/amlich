use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{
    layout::LayoutMode,
    state::{ui_prefs::VerbosityMode, AppState},
    widgets::direction_panel::DirectionPanelWidget,
};

pub struct PersonalScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> PersonalScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for PersonalScreenWidget<'_> {
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
            .expect("bundle exists for personal screen");

        match (
            profile.has_personal_overlay,
            self.app.active_verbosity(),
            self.mode,
        ) {
            (true, VerbosityMode::Compact, LayoutMode::Small) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(9),
                    Constraint::Length(9),
                    Constraint::Min(10),
                ])
                .split(area);
                render_profile_verdict(self.app, rows[0], buf);
                render_kua(insight, rows[1], buf);
                render_matrix_summary(self.app, rows[2], buf);
                render_directions(insight, rows[3], buf);
            }
            (true, VerbosityMode::Compact, _) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(9),
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
                render_matrix_summary(self.app, rows[2], buf);
                render_directions(insight, rows[3], buf);
            }
            (true, VerbosityMode::Verbose, _) => {
                let rows = Layout::vertical([
                    Constraint::Length(6),
                    Constraint::Length(9),
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
                render_matrix_summary(self.app, rows[2], buf);

                match self.mode {
                    LayoutMode::Large | LayoutMode::Medium => {
                        let bottom = Layout::horizontal([
                            Constraint::Percentage(34),
                            Constraint::Percentage(33),
                            Constraint::Percentage(33),
                        ])
                        .split(rows[3]);
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
                        .split(rows[3]);
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
        .title(" Hồ Sơ Cá Nhân ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let profile = app
        .profile_availability_summary()
        .expect("bundle exists for personal profile verdict");
    let direction = app.direction_verdict();

    let mut lines = Vec::new();
    let verdict = if profile.has_personal_overlay {
        "Đang xem lớp cá nhân hóa nhẹ dựa trên hồ sơ của bạn."
    } else {
        "Chưa có đủ hồ sơ cá nhân; hiện chỉ hiển thị dữ liệu theo ngày."
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
    if !profile.missing_requirements.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("Thiếu để mở thêm:", Style::default().fg(Color::DarkGray)),
        ]));
        for item in profile.missing_requirements.iter().take(2) {
            lines.push(Line::from(vec![
                Span::raw("  • "),
                Span::styled(item.clone(), Style::default().fg(Color::DarkGray)),
            ]));
        }
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_scope_note(note: &str, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Thiếu Hồ Sơ Cá Nhân ")
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
                "Nhấn [p] để nhập hồ sơ cá nhân ngay trong TUI.",
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                "Cần năm sinh + giới tính để mở Tứ Mệnh, hướng hợp và Đại Vận.",
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  "),
            Span::styled("Thiếu hiện tại:", Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::raw("  • "),
            Span::styled(
                "Hồ sơ chưa đủ để mở hết lớp cá nhân hóa.",
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
        Line::from(""),
        Line::from(vec![
            Span::raw("  Trạng thái: "),
            Span::styled("đã bật lớp cá nhân hóa", Style::default().fg(Color::Green)),
        ]),
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

fn render_matrix_summary(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Ma Trận Cá Nhân Hóa ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(matrix) = &app.personal_matrix else {
        Paragraph::new("  Chưa có dữ liệu ma trận cá nhân.").render(inner, buf);
        return;
    };

    let lines = matrix_summary_lines(matrix);

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn matrix_summary_lines(matrix: &amlich_api::PersonalDayMatrixReportDto) -> Vec<Line<'static>> {
    let harmonious_pillars = matrix
        .day_person
        .pillars
        .iter()
        .filter(|pillar| pillar.branch_relation.has_harmony())
        .count();
    let conflicting_pillars = matrix
        .day_person
        .pillars
        .iter()
        .filter(|pillar| pillar.branch_relation.has_conflict())
        .count();
    let supportive_elements = matrix
        .element_resonance
        .entries
        .iter()
        .filter(|entry| entry.day_helps_deficit)
        .count();

    let best_direction = matrix.direction_merge.as_ref().and_then(|merge| {
        merge
            .entries
            .iter()
            .max_by_key(|entry| entry.net_score)
            .map(|entry| {
                format!(
                    "Hướng nổi bật: {} (điểm {})",
                    entry.direction, entry.net_score
                )
            })
    });
    let best_hour = matrix.personal_hours.as_ref().and_then(|hours| {
        hours
            .hours
            .iter()
            .max_by_key(|entry| entry.score)
            .map(|entry| format!("Giờ hợp cá nhân: {} ({})", entry.chi, entry.score))
    });
    let best_domain = matrix.domain_day_boost.as_ref().and_then(|boost| {
        boost
            .entries
            .iter()
            .max_by(|a, b| a.boosted_score.total_cmp(&b.boosted_score))
            .map(|entry| {
                format!(
                    "Miền nổi bật: {} ({:.0})",
                    entry.domain, entry.boosted_score
                )
            })
    });

    let mut lines: Vec<Line<'static>> = vec![
        Line::from(vec![
            Span::raw("  • "),
            Span::raw(format!(
                "Ngày-person: {} trụ hợp, {} trụ xung/khắc",
                harmonious_pillars, conflicting_pillars
            )),
        ]),
        Line::from(vec![
            Span::raw("  • "),
            Span::raw(format!(
                "Cộng hưởng ngũ hành: {} hành được ngày hỗ trợ đúng chỗ thiếu",
                supportive_elements
            )),
        ]),
    ];

    if let Some(line) = best_direction {
        lines.push(Line::from(vec![Span::raw("  • "), Span::raw(line)]));
    }
    if let Some(line) = best_hour {
        lines.push(Line::from(vec![Span::raw("  • "), Span::raw(line)]));
    }
    if let Some(line) = best_domain {
        lines.push(Line::from(vec![Span::raw("  • "), Span::raw(line)]));
    }

    if lines.is_empty() {
        lines.push(Line::from("  Chưa có điểm nhấn ma trận để hiển thị."));
    }

    lines
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
                pillar.can_chi.to_string(),
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

#[cfg(test)]
mod tests {
    use super::matrix_summary_lines;
    use amlich_api::{get_personal_day_matrix_report, BaziQuery, DateQuery};

    fn sample_birth() -> BaziQuery {
        BaziQuery {
            day: 1,
            month: 1,
            year: 1990,
            hour: 9,
            minute: 30,
            timezone: Some(7.0),
            longitude: None,
            use_solar_time: false,
            gender: Some("male".to_string()),
        }
    }

    fn sample_date() -> DateQuery {
        DateQuery {
            day: 10,
            month: 2,
            year: 2024,
            timezone: Some(7.0),
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
        }
    }

    #[test]
    fn matrix_summary_surfaces_day_person_and_element_resonance() {
        let matrix =
            get_personal_day_matrix_report(&sample_birth(), &sample_date()).expect("matrix");
        let lines = matrix_summary_lines(&matrix);
        let rendered = lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("Ngày-person:"));
        assert!(rendered.contains("Cộng hưởng ngũ hành:"));
    }

    #[test]
    fn matrix_summary_keeps_best_direction_hour_and_domain_when_available() {
        let matrix =
            get_personal_day_matrix_report(&sample_birth(), &sample_date()).expect("matrix");
        let lines = matrix_summary_lines(&matrix);
        let rendered = lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(rendered.contains("Hướng nổi bật:"));
        assert!(rendered.contains("Giờ hợp cá nhân:"));
        assert!(rendered.contains("Miền nổi bật:"));
    }
}
