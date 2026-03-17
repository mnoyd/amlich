use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

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

        match self.mode {
            LayoutMode::Large | LayoutMode::Medium => {
                let rows = Layout::vertical([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(area);
                let top = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(rows[0]);
                let bottom = Layout::horizontal([
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ])
                .split(rows[1]);

                render_current(bundle, top[0], buf);
                render_astronomy(bundle, top[1], buf);
                render_agriculture(bundle, bottom[0], buf);
                render_health(bundle, bottom[1], buf);
            }
            LayoutMode::Small => {
                let rows = Layout::vertical([
                    Constraint::Min(10),
                    Constraint::Min(8),
                    Constraint::Min(8),
                    Constraint::Min(8),
                ])
                .split(area);
                render_current(bundle, rows[0], buf);
                render_astronomy(bundle, rows[1], buf);
                render_agriculture(bundle, rows[2], buf);
                render_health(bundle, rows[3], buf);
            }
        }
    }
}

fn render_current(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Tiết Khí Hiện Tại ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let mut lines: Vec<Line<'_>> = vec![];

    if let Some(tk) = &bundle.tiet_khi {
        lines.push(Line::from(vec![
            Span::raw("  Tiết khí: "),
            Span::styled(
                &tk.name,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw("  Kinh độ: "),
            Span::styled(
                format!("{}°", tk.longitude),
                Style::default().fg(Color::Yellow),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw("  Mùa: "),
            Span::styled(&tk.season, Style::default().fg(Color::Green)),
        ]));
        lines.push(Line::from(""));
        lines.push(Line::from(format!("  {}", tk.description)));
    }

    if let Some(insight) = &bundle.insight {
        if let Some(tki) = &insight.tiet_khi {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Ý nghĩa:",
                Style::default().fg(Color::DarkGray),
            )));
            lines.push(Line::from(format!("  {}", tki.meaning.vi)));
        }
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_astronomy(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Thiên Văn & Thời Tiết ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        return;
    };
    let Some(tki) = &insight.tiet_khi else {
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(Span::styled(
        "  Thiên văn:",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(format!("  {}", tki.astronomy.vi)));

    if let Some(tk) = &bundle.tiet_khi {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::raw("  Kinh độ hiện tại: "),
            Span::styled(
                format!("{:.1}°", tk.current_longitude),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Thời tiết:",
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(format!("  {}", tki.weather.vi)));

    Paragraph::new(lines).render(inner, buf);
}

fn render_agriculture(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Nông Nghiệp ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        return;
    };
    let Some(tki) = &insight.tiet_khi else {
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(Span::styled(
        "  Hoạt động nông vụ:",
        Style::default().fg(Color::Green),
    )));
    for item in &tki.agriculture.vi {
        lines.push(Line::from(format!("   \u{251C} {item}")));
    }

    Paragraph::new(lines).render(inner, buf);
}

fn render_health(bundle: &amlich_api::v2::DayBundleDto, area: Rect, buf: &mut Buffer) {
    let block = Block::default()
        .title(" Sức Khỏe ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    let inner = block.inner(area);
    block.render(area, buf);

    let Some(insight) = &bundle.insight else {
        return;
    };
    let Some(tki) = &insight.tiet_khi else {
        return;
    };
    let mut lines: Vec<Line<'_>> = vec![];

    lines.push(Line::from(Span::styled(
        "  Lời khuyên sức khỏe:",
        Style::default().fg(Color::Cyan),
    )));
    for item in &tki.health.vi {
        lines.push(Line::from(format!("   \u{251C} {item}")));
    }

    Paragraph::new(lines).render(inner, buf);
}
