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
            .title(" Hướng & Thần ")
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

        if let Some(travel) = &insight.travel {
            lines.push(Line::from(vec![
                Span::raw("  Xuất hành: "),
                Span::styled(&travel.xuat_hanh_huong, Style::default().fg(Color::Green)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Hỷ Thần: "),
                Span::styled(&travel.hy_than, Style::default().fg(Color::Green)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Tài Thần: "),
                Span::styled(&travel.tai_than, Style::default().fg(Color::Yellow)),
            ]));
        }

        if let Some(deity) = &insight.day_deity {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  Thần sát: "),
                Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Phân loại: "),
                Span::styled(
                    &deity.classification_meaning.vi,
                    Style::default().fg(Color::Cyan),
                ),
            ]));
            if let Some(meaning) = &deity.deity_meaning {
                lines.push(Line::from(format!("  {}", meaning.vi)));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}
