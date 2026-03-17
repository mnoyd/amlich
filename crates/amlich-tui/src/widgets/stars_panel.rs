use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct StarsPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> StarsPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for StarsPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Sao & Trực ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else {
            return;
        };
        let Some(insight) = &bundle.insight else {
            return;
        };
        let mut lines: Vec<Line<'_>> = vec![];

        if let Some(truc) = &insight.truc {
            lines.push(Line::from(vec![
                Span::raw("  Trực: "),
                Span::styled(&truc.name, Style::default().fg(Color::Cyan)),
                Span::raw(" ("),
                Span::raw(&truc.quality),
                Span::raw(")"),
            ]));
            lines.push(Line::from(format!("  {}", truc.meaning.vi)));
            lines.push(Line::from(""));
        }

        if let Some(stars) = &insight.stars {
            if let Some(day_star) = &stars.day_star {
                let q = stars.day_star_quality.as_deref().unwrap_or("");
                lines.push(Line::from(vec![
                    Span::raw("  Sao ngày: "),
                    Span::styled(day_star.as_str(), Style::default().fg(Color::Yellow)),
                    Span::raw(format!(" ({q})")),
                ]));
                lines.push(Line::from(""));
            }

            let cat = stars.cat_tinh.join(", ");
            lines.push(Line::from(vec![
                Span::raw("  Cát tinh: "),
                Span::styled(
                    if stars.cat_tinh.is_empty() {
                        "Không".to_string()
                    } else {
                        cat
                    },
                    Style::default().fg(Color::Green),
                ),
            ]));
            let sat = stars.sat_tinh.join(", ");
            lines.push(Line::from(vec![
                Span::raw("  Sát tinh: "),
                Span::styled(
                    if stars.sat_tinh.is_empty() {
                        "Không".to_string()
                    } else {
                        sat
                    },
                    Style::default().fg(Color::Red),
                ),
            ]));
        }

        if let Some(deity) = &insight.day_deity {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Thần sát: "),
                Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
                Span::raw(format!(" ({})", deity.classification)),
            ]));
            if let Some(m) = &deity.deity_meaning {
                lines.push(Line::from(format!("   {}", m.vi)));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
