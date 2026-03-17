use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct NaAmPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> NaAmPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for NaAmPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Nạp Âm & Ngũ Hành ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(bundle) = &self.app.bundle else {
            return;
        };
        let mut lines: Vec<Line<'_>> = vec![];

        if let Some(fortune) = &bundle.day_fortune {
            lines.push(Line::from(vec![
                Span::raw("  Nạp âm: "),
                Span::styled(
                    &fortune.day_element.na_am,
                    Style::default().fg(Color::Yellow),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Ngũ hành: "),
                Span::styled(
                    &fortune.day_element.element,
                    Style::default().fg(Color::Cyan),
                ),
            ]));
        }

        if let Some(insight) = &bundle.insight {
            if let Some(na_am) = &insight.na_am {
                lines.push(Line::from(vec![
                    Span::raw("  Hành: "),
                    Span::styled(&na_am.element, Style::default().fg(Color::Green)),
                ]));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "  Ý nghĩa:",
                    Style::default().fg(Color::DarkGray),
                )));
                lines.push(Line::from(format!("  {}", na_am.meaning.vi)));
            }
        }

        if let Some(canchi) = &bundle.canchi {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Con giáp ngày: "),
                Span::styled(&canchi.day.con_giap, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Con giáp tháng: "),
                Span::styled(&canchi.month.con_giap, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Con giáp năm: "),
                Span::styled(&canchi.year.con_giap, Style::default().fg(Color::Cyan)),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
