use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

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

        match self.mode {
            LayoutMode::Large => {
                let rows = Layout::vertical([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]).split(area);
                let top = Layout::horizontal([
                    Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33),
                ]).split(rows[0]);
                let bottom = Layout::horizontal([
                    Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33),
                ]).split(rows[1]);

                render_tang_can(bundle, top[0], buf);
                render_ten_gods(bundle, top[1], buf);
                render_xung_hop(bundle, top[2], buf);
                render_element_relations(bundle, bottom[0], buf);
                render_pillars(bundle, bottom[1], buf);
                render_element_chart(bundle, bottom[2], buf);
            }
            LayoutMode::Medium => {
                let rows = Layout::vertical([
                    Constraint::Percentage(34), Constraint::Percentage(33), Constraint::Percentage(33),
                ]).split(area);
                let r0 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[0]);
                let r1 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[1]);
                let r2 = Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(rows[2]);

                render_tang_can(bundle, r0[0], buf);
                render_ten_gods(bundle, r0[1], buf);
                render_xung_hop(bundle, r1[0], buf);
                render_element_relations(bundle, r1[1], buf);
                render_pillars(bundle, r2[0], buf);
                render_element_chart(bundle, r2[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(8), Constraint::Min(10), Constraint::Min(8),
                    Constraint::Min(8), Constraint::Min(8), Constraint::Min(8),
                ]).split(area);
                render_tang_can(bundle, rows[0], buf);
                render_ten_gods(bundle, rows[1], buf);
                render_xung_hop(bundle, rows[2], buf);
                render_element_relations(bundle, rows[3], buf);
                render_pillars(bundle, rows[4], buf);
                render_element_chart(bundle, rows[5], buf);
            }
        }
    }
}

fn render_tang_can(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Tàng Can ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(tc) = &insight.tang_can else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    if let Some(canchi) = &bundle.canchi {
        lines.push(Line::from(vec![
            Span::raw("  Chi ngày: "),
            Span::styled(&canchi.day.chi, Style::default().fg(Color::Cyan)),
        ]));
        lines.push(Line::from(""));
    }

    let labels = ["Chính", "Trung", "Dư"];
    let values = [&tc.main, &tc.central, &tc.residual];
    for (i, (label, value)) in labels.iter().zip(values.iter()).enumerate() {
        let s = tc.strength[i];
        let bar_len = (s as usize * 10) / 100;
        let bar = "\u{2588}".repeat(bar_len) + &"\u{2591}".repeat(10 - bar_len);
        lines.push(Line::from(vec![
            Span::raw(format!("  {label}: ")),
            Span::styled(
                format!("{value}"),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" {bar} {s}%")),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_ten_gods(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Thập Thần ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(tg) = &insight.ten_gods else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    if let Some(e) = &tg.to_year_stem {
        lines.push(Line::from(Span::styled("  Với năm sinh:", Style::default().fg(Color::DarkGray))));
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(&e.label, Style::default().fg(Color::Yellow)),
            Span::raw(format!(": {}", e.name.vi)),
        ]));
        lines.push(Line::from(format!("   Nghĩa: {}", e.meaning.vi)));
        lines.push(Line::from(format!("   Quan hệ: {} ({})",
            e.relation,
            if e.same_polarity { "đồng cực" } else { "khác cực" },
        )));
        lines.push(Line::from(""));
    }

    if let Some(e) = &tg.to_self {
        lines.push(Line::from(Span::styled("  Với bản thân:", Style::default().fg(Color::DarkGray))));
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(&e.label, Style::default().fg(Color::Yellow)),
            Span::raw(format!(": {}", e.name.vi)),
        ]));
        lines.push(Line::from(format!("   Nghĩa: {}", e.meaning.vi)));
        lines.push(Line::from(format!("   Quan hệ: {} ({})",
            e.relation,
            if e.same_polarity { "đồng cực" } else { "khác cực" },
        )));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_xung_hop(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Xung Hợp ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else { return };
    let Some(xh) = &insight.xung_hop else {
        Paragraph::new("  Chưa có dữ liệu").render(inner, buf);
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(vec![
        Span::raw("  Lục xung: "),
        Span::styled(&xh.luc_xung, Style::default().fg(Color::Red)),
    ]));
    if !xh.tam_hop.is_empty() {
        lines.push(Line::from(vec![
            Span::raw("  Tam hợp: "),
            Span::styled(xh.tam_hop.join(" \u{2014} "), Style::default().fg(Color::Green)),
        ]));
    }
    if let Some(lh) = &xh.liu_he {
        lines.push(Line::from(vec![
            Span::raw("  Lục hợp: "),
            Span::styled(lh.as_str(), Style::default().fg(Color::Green)),
        ]));
    }
    if let Some(xhai) = &xh.xiang_hai {
        lines.push(Line::from(vec![
            Span::raw("  Tương hại: "),
            Span::styled(xhai.as_str(), Style::default().fg(Color::Red)),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_element_relations(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Ngũ Hành Tương Quan ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(canchi) = &bundle.canchi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    let can_e = &canchi.day.ngu_hanh.can;
    let chi_e = &canchi.day.ngu_hanh.chi;

    lines.push(Line::from(vec![
        Span::raw("  Can ngày: "),
        Span::styled(format!("{} ({})", canchi.day.can, can_e), Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(vec![
        Span::raw("  Chi ngày: "),
        Span::styled(format!("{} ({})", canchi.day.chi, chi_e), Style::default().fg(Color::Cyan)),
    ]));

    let rel = element_relation(can_e, chi_e);
    let rel_color = if rel.contains("sinh") { Color::Green } else if rel.contains("khắc") { Color::Red } else { Color::Yellow };
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::raw("  Quan hệ: "),
        Span::styled(format!("{can_e} {rel} {chi_e}"), Style::default().fg(rel_color)),
    ]));

    Paragraph::new(lines).render(inner, buf);
}

fn render_pillars(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Can Chi 3 Trụ ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(canchi) = &bundle.canchi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(vec![
        Span::raw("            "),
        Span::styled("Can    Chi    Hành", Style::default().fg(Color::DarkGray).add_modifier(Modifier::BOLD)),
    ]));
    for (label, p) in [("Năm:  ", &canchi.year), ("Tháng:", &canchi.month), ("Ngày: ", &canchi.day)] {
        lines.push(Line::from(vec![
            Span::raw(format!("  {label} ")),
            Span::styled(format!("{:<6}", p.can), Style::default().fg(Color::Cyan)),
            Span::styled(format!("{:<6}", p.chi), Style::default().fg(Color::Cyan)),
            Span::styled(format!("{}/{}", p.ngu_hanh.can, p.ngu_hanh.chi), Style::default().fg(Color::Yellow)),
        ]));
    }

    if let Some(fortune) = &bundle.day_fortune {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Nạp âm: "),
            Span::styled(&fortune.day_element.na_am, Style::default().fg(Color::Yellow)),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_element_chart(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default().title(" Ngũ Hành Tổng Hợp ").borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(canchi) = &bundle.canchi else { return };
    let mut lines: Vec<Line<'_>> = vec![];

    let elements = [
        &canchi.year.ngu_hanh.can, &canchi.year.ngu_hanh.chi,
        &canchi.month.ngu_hanh.can, &canchi.month.ngu_hanh.chi,
        &canchi.day.ngu_hanh.can, &canchi.day.ngu_hanh.chi,
    ];

    let names = ["Kim", "Mộc", "Thủy", "Hỏa", "Thổ"];
    let colors = [Color::White, Color::Green, Color::Blue, Color::Red, Color::Yellow];
    let mut dominant = ("", 0usize);

    for (i, name) in names.iter().enumerate() {
        let count = elements.iter().filter(|e| e.as_str() == *name).count();
        if count > dominant.1 { dominant = (name, count); }
        let bar = "\u{2588}".repeat(count * 3) + &"\u{2591}".repeat(18usize.saturating_sub(count * 3));
        lines.push(Line::from(vec![
            Span::styled(format!("  {name:<4} "), Style::default().fg(colors[i]).add_modifier(Modifier::BOLD)),
            Span::raw(format!("{bar} {count}/6")),
        ]));
    }

    if !dominant.0.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Hành vượng: "),
            Span::styled(dominant.0, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn element_relation(a: &str, b: &str) -> &'static str {
    match (a, b) {
        ("Kim", "Thủy") | ("Thủy", "Mộc") | ("Mộc", "Hỏa") | ("Hỏa", "Thổ") | ("Thổ", "Kim") => "sinh",
        ("Thủy", "Kim") | ("Mộc", "Thủy") | ("Hỏa", "Mộc") | ("Thổ", "Hỏa") | ("Kim", "Thổ") => "được sinh",
        ("Kim", "Mộc") | ("Mộc", "Thổ") | ("Thổ", "Thủy") | ("Thủy", "Hỏa") | ("Hỏa", "Kim") => "khắc",
        ("Mộc", "Kim") | ("Thổ", "Mộc") | ("Thủy", "Thổ") | ("Hỏa", "Thủy") | ("Kim", "Hỏa") => "bị khắc",
        _ if a == b => "tỷ hòa",
        _ => "\u{2014}",
    }
}
