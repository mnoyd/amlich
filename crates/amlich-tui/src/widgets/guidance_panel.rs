use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct GuidancePanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> GuidancePanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for GuidancePanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Nên Làm / Tránh ")
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

        if let Some(guidance) = &insight.day_guidance {
            lines.push(Line::from(Span::styled(
                "  Nên làm:",
                Style::default().fg(Color::Green),
            )));
            for item in &guidance.good_for.vi {
                lines.push(Line::from(format!("   \u{251C} {item}")));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Tránh làm:",
                Style::default().fg(Color::Red),
            )));
            for item in &guidance.avoid_for.vi {
                lines.push(Line::from(format!("   \u{251C} {item}")));
            }
        } else if let Some(truc) = &insight.truc {
            lines.push(Line::from(Span::styled(
                "  Nên làm:",
                Style::default().fg(Color::Green),
            )));
            for item in &truc.good_for.vi {
                lines.push(Line::from(format!("   \u{251C} {item}")));
            }
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "  Tránh làm:",
                Style::default().fg(Color::Red),
            )));
            for item in &truc.avoid_for.vi {
                lines.push(Line::from(format!("   \u{251C} {item}")));
            }
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
