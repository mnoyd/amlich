use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::{layout::LayoutMode, state::AppState};

pub struct DirectionPanelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> DirectionPanelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for DirectionPanelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Hướng Hành Sự ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(direction) = self.app.direction_verdict() else {
            return;
        };
        let mut lines: Vec<Line<'_>> = vec![];

        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(direction.summary, Style::default().fg(Color::Green)),
        ]));

        for item in direction.directions.iter().take(3) {
            lines.push(Line::from(vec![Span::raw("  • "), Span::raw(item.clone())]));
        }

        if let Some(deity_context) = direction.deity_context {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(deity_context, Style::default().fg(Color::Yellow)),
            ]));
        }

        if let Some(note) = direction.note {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(note, Style::default().fg(Color::DarkGray)),
            ]));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
