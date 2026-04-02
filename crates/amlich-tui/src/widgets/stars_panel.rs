use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
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
            .title(" Dấu Hiệu Truyền Thống ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(summary) = self.app.traditional_evidence_summary() else {
            return;
        };
        let mut lines: Vec<Line<'_>> = vec![];

        if let Some(headline) = summary.headline {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(headline, Style::default().fg(Color::Cyan)),
            ]));
        }

        if !summary.positive_signals.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Điểm thuận:", Style::default().fg(Color::Green)),
            ]));
        }

        for signal in summary.positive_signals.iter().take(3) {
            lines.push(Line::from(vec![
                Span::styled("  ★ ", Style::default().fg(Color::Green)),
                Span::raw(signal.clone()),
            ]));
        }

        if !summary.caution_signals.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Điểm cần lưu ý:", Style::default().fg(Color::Red)),
            ]));
        }

        for signal in summary.caution_signals.iter().take(3) {
            lines.push(Line::from(vec![
                Span::styled("  ! ", Style::default().fg(Color::Red)),
                Span::raw(signal.clone()),
            ]));
        }

        if self.app.show_evidence && !summary.provenance.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Căn cứ:", Style::default().fg(Color::DarkGray)),
            ]));
            for item in summary.provenance.iter().take(4) {
                lines.push(Line::from(vec![
                    Span::raw("   ↳ "),
                    Span::styled(item.clone(), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }

        if self.app.show_evidence && !summary.source_notes.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled("Nguồn nền:", Style::default().fg(Color::DarkGray)),
            ]));
            for item in summary.source_notes.iter().take(3) {
                lines.push(Line::from(vec![
                    Span::raw("   ↳ "),
                    Span::styled(item.clone(), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}
