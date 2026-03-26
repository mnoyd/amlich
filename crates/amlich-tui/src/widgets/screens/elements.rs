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
    widgets::{
        guidance_panel::GuidancePanelWidget, naam_panel::NaAmPanelWidget,
        scholarly::ScholarlyWidget,
    },
};

pub struct ElementsScreenWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> ElementsScreenWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for ElementsScreenWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            Paragraph::new("Chưa có dữ liệu.").render(area, buf);
            return;
        };

        match (self.mode, self.app.active_verbosity()) {
            (_, VerbosityMode::Compact) => {
                let rows = Layout::vertical([
                    Constraint::Length(8),
                    Constraint::Length(10),
                    Constraint::Length(8),
                    Constraint::Min(8),
                ])
                .split(area);
                ScholarlyWidget::new(self.app, self.mode).render(rows[0], buf);
                render_element_relations(bundle, rows[1], buf);
                render_compact_element_notes(bundle, rows[2], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[3], buf);
            }
            (LayoutMode::Large | LayoutMode::Medium, VerbosityMode::Verbose) => {
                let rows = Layout::vertical([
                    Constraint::Length(8),
                    Constraint::Length(10),
                    Constraint::Length(9),
                    Constraint::Min(12),
                ])
                .split(area);

                ScholarlyWidget::new(self.app, self.mode).render(rows[0], buf);

                let middle =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[1]);
                NaAmPanelWidget::new(self.app, self.mode).render(middle[0], buf);
                render_element_relations(bundle, middle[1], buf);

                GuidancePanelWidget::new(self.app, self.mode).render(rows[2], buf);

                let bottom = Layout::horizontal([
                    Constraint::Percentage(34),
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                ])
                .split(rows[3]);
                render_tang_can(bundle, bottom[0], buf);
                render_ten_gods(bundle, bottom[1], buf);
                render_xung_hop(bundle, bottom[2], buf);
            }
            (LayoutMode::Small, VerbosityMode::Verbose) => {
                let rows = Layout::vertical([
                    Constraint::Length(8),
                    Constraint::Length(10),
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Length(8),
                    Constraint::Min(8),
                ])
                .split(area);
                ScholarlyWidget::new(self.app, self.mode).render(rows[0], buf);
                NaAmPanelWidget::new(self.app, self.mode).render(rows[1], buf);
                render_element_relations(bundle, rows[2], buf);
                GuidancePanelWidget::new(self.app, self.mode).render(rows[3], buf);
                render_tang_can(bundle, rows[4], buf);
                render_xung_hop(bundle, rows[5], buf);
            }
        }
    }
}

fn render_compact_element_notes(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Điểm Chính / Ứng Dụng ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };

    let mut lines: Vec<Line<'_>> = vec![];
    if let Some(tg) = &insight.ten_gods {
        if let Some(entry) = &tg.to_self {
            lines.push(Line::from(vec![
                Span::styled("  • Nhật chủ: ", Style::default().fg(Color::Cyan)),
                Span::styled(entry.name.vi.clone(), Style::default().fg(Color::Yellow)),
                Span::raw(format!(" · {}", entry.meaning.vi)),
            ]));
        }
    }

    if let Some(xh) = &insight.xung_hop {
        lines.push(Line::from(vec![
            Span::styled("  • Lục xung: ", Style::default().fg(Color::Red)),
            Span::styled(xh.luc_xung.clone(), Style::default().fg(Color::White)),
        ]));
        if !xh.tam_hop.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("  • Tam hợp: ", Style::default().fg(Color::Green)),
                Span::styled(xh.tam_hop.join(" · "), Style::default().fg(Color::White)),
            ]));
        }
    }

    if let Some(note) = self_application_note(bundle) {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(note, Style::default().fg(Color::Yellow)),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from("  Chưa có dữ liệu"));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_tang_can(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tàng Can / Thập Thần ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };

    let mut lines: Vec<Line<'_>> = vec![];
    if let Some(tc) = &insight.tang_can {
        let labels = ["Chính", "Trung", "Dư"];
        let values = [&tc.main, &tc.central, &tc.residual];
        for (index, (label, value)) in labels.iter().zip(values.iter()).enumerate() {
            let strength = tc.strength[index];
            lines.push(Line::from(vec![
                Span::raw(format!("  {label}: ")),
                Span::styled(
                    value.to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(" · {strength}%")),
            ]));
        }
    }

    if let Some(tg) = &insight.ten_gods {
        if let Some(entry) = &tg.to_self {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Bản thân: "),
                Span::styled(&entry.name.vi, Style::default().fg(Color::Cyan)),
                Span::raw(format!(" · {} · {}", entry.relation, entry.meaning.vi)),
            ]));
            lines.push(Line::from(format!(
                "  Đồng cực tính: {}",
                if entry.same_polarity { "có" } else { "không" }
            )));
        }
        if let Some(entry) = &tg.to_year_stem {
            lines.push(Line::from(vec![
                Span::raw("  Với năm sinh: "),
                Span::styled(&entry.name.vi, Style::default().fg(Color::Cyan)),
                Span::raw(format!(" · {} · {}", entry.relation, entry.meaning.vi)),
            ]));
        }
    }

    if lines.is_empty() {
        lines.push(Line::from("  Chưa có dữ liệu"));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_xung_hop(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Xung Hợp / Ứng Dụng ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    if let Some(xh) = &insight.xung_hop {
        lines.push(Line::from(vec![
            Span::raw("  Lục xung: "),
            Span::styled(&xh.luc_xung, Style::default().fg(Color::Red)),
        ]));
        if !xh.tam_hop.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("  Tam hợp: "),
                Span::styled(xh.tam_hop.join(" · "), Style::default().fg(Color::Green)),
            ]));
        }
        if let Some(lh) = &xh.liu_he {
            lines.push(Line::from(vec![
                Span::raw("  Lục hợp: "),
                Span::styled(lh.as_str(), Style::default().fg(Color::Green)),
            ]));
        }
        if let Some(xh) = &xh.xiang_hai {
            lines.push(Line::from(vec![
                Span::raw("  Tương hại: "),
                Span::styled(xh.as_str(), Style::default().fg(Color::Red)),
            ]));
        }
    }

    if let Some(note) = self_application_note(bundle) {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(note, Style::default().fg(Color::Yellow)),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from("  Chưa có dữ liệu"));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_ten_gods(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Thập Thần ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };
    let Some(ten_gods) = &insight.ten_gods else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };

    let mut lines: Vec<Line<'_>> = vec![];
    if let Some(entry) = &ten_gods.to_self {
        lines.push(Line::from(vec![
            Span::raw("  Bản thân: "),
            Span::styled(&entry.name.vi, Style::default().fg(Color::Cyan)),
        ]));
        lines.push(Line::from(format!("  {}", entry.meaning.vi)));
    }
    if let Some(entry) = &ten_gods.to_year_stem {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Với năm sinh: "),
            Span::styled(&entry.name.vi, Style::default().fg(Color::Yellow)),
        ]));
        lines.push(Line::from(format!("  {}", entry.meaning.vi)));
    }

    if lines.is_empty() {
        lines.push(Line::from("  Chưa có dữ liệu"));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_element_relations(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Luận Giải Kết Cấu ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(canchi) = &bundle.canchi else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };

    let can_element = &canchi.day.ngu_hanh.can;
    let chi_element = &canchi.day.ngu_hanh.chi;
    let relation = element_relation(can_element, chi_element);
    let relation_color = if relation.contains("sinh") {
        Color::Green
    } else if relation.contains("khắc") {
        Color::Red
    } else {
        Color::Yellow
    };

    let mut lines: Vec<Line<'_>> = vec![
        Line::from(vec![
            Span::raw("  Can ngày: "),
            Span::styled(
                format!("{} ({})", canchi.day.can, can_element),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("  Chi ngày: "),
            Span::styled(
                format!("{} ({})", canchi.day.chi, chi_element),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::raw("  Quan hệ: "),
            Span::styled(
                format!("{can_element} {relation} {chi_element}"),
                Style::default().fg(relation_color),
            ),
        ]),
    ];

    if let Some(insight) = &bundle.insight {
        if let Some(canchi_insight) = &insight.canchi {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Nghĩa can: "),
                Span::styled(
                    &canchi_insight.can.meaning.vi,
                    Style::default().fg(Color::White),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Tính chất can: "),
                Span::styled(
                    &canchi_insight.can.nature.vi,
                    Style::default().fg(Color::Gray),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Nghĩa chi: "),
                Span::styled(
                    &canchi_insight.chi.meaning.vi,
                    Style::default().fg(Color::White),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Giờ ứng chi: "),
                Span::styled(
                    &canchi_insight.chi.hours,
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
            if let Some(element) = &canchi_insight.element {
                lines.push(Line::from(vec![
                    Span::raw("  Khí hành: "),
                    Span::styled(&element.name.vi, Style::default().fg(Color::Yellow)),
                    Span::raw(format!(" · {}", element.nature.vi)),
                ]));
            }
        }
    }

    if let Some(fortune) = &bundle.day_fortune {
        lines.push(Line::from(vec![
            Span::raw("  Nạp âm: "),
            Span::styled(
                &fortune.day_element.na_am,
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn element_relation(a: &str, b: &str) -> &'static str {
    match (a, b) {
        ("Kim", "Thủy") | ("Thủy", "Mộc") | ("Mộc", "Hỏa") | ("Hỏa", "Thổ") | ("Thổ", "Kim") => {
            "sinh"
        }
        ("Thủy", "Kim") | ("Mộc", "Thủy") | ("Hỏa", "Mộc") | ("Thổ", "Hỏa") | ("Kim", "Thổ") => {
            "được sinh"
        }
        ("Kim", "Mộc") | ("Mộc", "Thổ") | ("Thổ", "Thủy") | ("Thủy", "Hỏa") | ("Hỏa", "Kim") => {
            "khắc"
        }
        ("Mộc", "Kim") | ("Thổ", "Mộc") | ("Thủy", "Thổ") | ("Hỏa", "Thủy") | ("Kim", "Hỏa") => {
            "bị khắc"
        }
        _ if a == b => "tỷ hòa",
        _ => "không nổi trội",
    }
}

fn self_application_note(bundle: &amlich_api::v2::DayBundleDto) -> Option<String> {
    bundle
        .insight
        .as_ref()
        .and_then(|insight| insight.day_guidance.as_ref())
        .and_then(|guidance| guidance.good_for.vi.first())
        .map(|item| format!("Nối lại với thực hành: khí này hợp để {item}."))
}
