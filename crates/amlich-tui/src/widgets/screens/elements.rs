use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
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
        if self.app.bundle.is_none() {
            Paragraph::new("Chưa có dữ liệu Ngũ Hành.").render(area, buf);
            return;
        }

        match self.mode {
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Length(8),
                    Constraint::Length(9),
                    Constraint::Length(11),
                    Constraint::Length(9),
                    Constraint::Min(9),
                ])
                .split(area);
                render_day_element(self.app, rows[0], buf);
                render_relationships(self.app, rows[1], buf);
                render_canchi(self.app, rows[2], buf);
                render_truc(self.app, rows[3], buf);
                render_combined_guidance(self.app, rows[4], buf);
            }
            LayoutMode::Medium | LayoutMode::Large => {
                let rows = Layout::vertical([
                    Constraint::Length(9),
                    Constraint::Length(10),
                    Constraint::Min(15),
                ])
                .split(area);
                let top =
                    Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)])
                        .split(rows[0]);
                render_day_element(self.app, top[0], buf);
                render_relationships(self.app, top[1], buf);
                render_canchi(self.app, rows[1], buf);
                let bottom =
                    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                        .split(rows[2]);
                render_truc(self.app, bottom[0], buf);
                render_combined_guidance(self.app, bottom[1], buf);
            }
        }
    }
}

fn insight(app: &AppState) -> Option<&amlich_api::DayInsightDto> {
    app.bundle.as_ref()?.insight.as_ref()
}

fn day_element(app: &AppState) -> Option<&str> {
    insight(app)
        .and_then(|insight| insight.canchi.as_ref())
        .and_then(|canchi| canchi.element.as_ref())
        .map(|element| element.name.vi.as_str())
        .or_else(|| {
            app.bundle
                .as_ref()
                .and_then(|bundle| bundle.day_fortune.as_ref())
                .map(|fortune| fortune.day_element.element.as_str())
        })
}

fn render_day_element(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Ngũ Hành Ngày ", Color::Yellow);
    let inner = block.inner(area);
    block.render(area, buf);
    let element = day_element(app).unwrap_or("Chưa xác định");
    let nature = insight(app)
        .and_then(|insight| insight.canchi.as_ref())
        .and_then(|canchi| canchi.element.as_ref())
        .map(|element| element.nature.vi.as_str())
        .unwrap_or("Chưa có diễn giải bản chất hành.");
    Paragraph::new(vec![
        Line::from(Span::styled(
            format!("  {element}"),
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(format!("  {nature}")),
    ])
    .wrap(Wrap { trim: true })
    .render(inner, buf);
}

fn render_relationships(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Tương Sinh / Tương Khắc ", Color::Cyan);
    let inner = block.inner(area);
    block.render(area, buf);
    let lines = element_relationships(day_element(app).unwrap_or(""))
        .map(|relations| {
            vec![
                Line::from(format!(
                    "  Được sinh: {} sinh {}",
                    relations.generated_by, relations.element
                )),
                Line::from(format!(
                    "  Sinh xuất: {} sinh {}",
                    relations.element, relations.generates
                )),
                Line::from(format!(
                    "  Chế khắc: {} khắc {}",
                    relations.element, relations.controls
                )),
                Line::from(format!(
                    "  Bị khắc: {} khắc {}",
                    relations.controlled_by, relations.element
                )),
            ]
        })
        .unwrap_or_else(|| vec![Line::from("  Chưa xác định được quan hệ sinh khắc.")]);
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

struct ElementRelationships {
    element: &'static str,
    generated_by: &'static str,
    generates: &'static str,
    controls: &'static str,
    controlled_by: &'static str,
}

fn element_relationships(element: &str) -> Option<ElementRelationships> {
    let values = match element.trim() {
        "Kim" => ("Kim", "Thổ", "Thủy", "Mộc", "Hỏa"),
        "Mộc" => ("Mộc", "Thủy", "Hỏa", "Thổ", "Kim"),
        "Thủy" => ("Thủy", "Kim", "Mộc", "Hỏa", "Thổ"),
        "Hỏa" => ("Hỏa", "Mộc", "Thổ", "Kim", "Thủy"),
        "Thổ" => ("Thổ", "Hỏa", "Kim", "Thủy", "Mộc"),
        _ => return None,
    };
    Some(ElementRelationships {
        element: values.0,
        generated_by: values.1,
        generates: values.2,
        controls: values.3,
        controlled_by: values.4,
    })
}

fn render_canchi(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Can Chi & Ngũ Hành Các Trụ ", Color::Magenta);
    let inner = block.inner(area);
    block.render(area, buf);
    let Some(bundle) = &app.bundle else {
        return;
    };
    let Some(canchi) = &bundle.canchi else {
        Paragraph::new("  Chưa có dữ liệu can chi.").render(inner, buf);
        return;
    };
    let mut lines = vec![
        pillar_line("Ngày", &canchi.day),
        pillar_line("Tháng", &canchi.month),
        pillar_line("Năm", &canchi.year),
    ];
    if let Some(detail) = insight(app).and_then(|insight| insight.canchi.as_ref()) {
        lines.push(Line::from(format!(
            "  Thiên Can {} · hành {} · {}",
            detail.can.name, detail.can.element, detail.can.meaning.vi
        )));
        lines.push(Line::from(format!(
            "  Địa Chi {} · hành {} · {} · giờ {}",
            detail.chi.name, detail.chi.element, detail.chi.meaning.vi, detail.chi.hours
        )));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn pillar_line(label: &'static str, pillar: &amlich_api::CanChiDto) -> Line<'static> {
    Line::from(format!(
        "  {label}: {} · can {} / chi {}",
        pillar.full, pillar.ngu_hanh.can, pillar.ngu_hanh.chi
    ))
}

fn render_truc(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = section_block(" Trực & Ý Nghĩa Đầy Đủ ", Color::Green);
    let inner = block.inner(area);
    block.render(area, buf);
    let lines = insight(app)
        .and_then(|insight| insight.truc.as_ref())
        .map(|truc| {
            vec![
                Line::from(format!("  Trực {} ({})", truc.name, truc.quality)),
                Line::from(format!("  {}", truc.meaning.vi)),
            ]
        })
        .unwrap_or_else(|| vec![Line::from("  Chưa có dữ liệu Trực.")]);
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn render_combined_guidance(app: &AppState, area: Rect, buf: &mut Buffer) {
    let block = section_block(
        " Trực + Hành (tổng hợp trình bày — không phải tương tác ngữ nghĩa) ",
        Color::Blue,
    );
    let inner = block.inner(area);
    block.render(area, buf);
    let element = day_element(app).unwrap_or("chưa rõ hành");
    let truc = insight(app).and_then(|insight| insight.truc.as_ref());
    let guidance = insight(app).and_then(|insight| insight.day_guidance.as_ref());
    let mut lines = vec![
        Line::from(Span::styled(
            "  (Trình bày gộp hai nguồn độc lập; không phải công thức tương tác)",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )),
        Line::from(format!(
            "  Đọc phối hợp: Trực {} trong khí {element}.",
            truc.map(|truc| truc.name.as_str()).unwrap_or("chưa rõ")
        )),
    ];
    if let Some(guidance) = guidance {
        for item in &guidance.good_for.vi {
            lines.push(Line::from(format!("  ✓ Nên: {item}")));
        }
        for item in &guidance.avoid_for.vi {
            lines.push(Line::from(format!("  ✕ Tránh: {item}")));
        }
    } else if let Some(truc) = truc {
        for item in &truc.good_for.vi {
            lines.push(Line::from(format!("  ✓ Thuận: {item}")));
        }
        for item in &truc.avoid_for.vi {
            lines.push(Line::from(format!("  ✕ Kỵ: {item}")));
        }
    } else {
        lines.push(Line::from("  Chưa đủ dữ liệu để phối hợp hướng dẫn."));
    }
    Paragraph::new(lines)
        .wrap(Wrap { trim: true })
        .render(inner, buf);
}

fn section_block(title: &str, color: Color) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
}
