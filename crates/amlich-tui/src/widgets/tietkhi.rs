use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct TietKhiWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> TietKhiWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for TietKhiWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else { return };
        let Some(tietkhi) = &bundle.tiet_khi else { return };

        let mut lines = vec![];
        let header_style = Style::default().fg(Color::DarkGray);
        let text_style = Style::default().fg(Color::White);
        let highlight = Style::default().fg(Color::Yellow);

        let expand_hint = if self.app.show_tietkhi_details {
            "▼ Thu gọn (Enter)"
        } else {
            "▶ Chi tiết (Enter)"
        };

        lines.push(Line::from(vec![
            Span::styled("── Tiết Khí ", header_style),
            Span::styled(format!("{:─<35}", ""), header_style),
            Span::styled(expand_hint, Style::default().fg(Color::DarkGray)),
        ]));

        // Summary (Always shown)
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(&tietkhi.name, highlight),
            Span::raw(" · "),
            Span::styled(&tietkhi.season, text_style),
        ]));
        
        let desc_lines: Vec<&str> = tietkhi.description.split('\n').collect();
        if let Some(first_line) = desc_lines.first() {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(*first_line, Style::default().fg(Color::Gray)),
            ]));
        }

        // Expanded view (Accordion)
        if self.app.show_tietkhi_details {
            lines.push(Line::from(""));
            for line in desc_lines.iter().skip(1) {
                if line.trim().is_empty() {
                    lines.push(Line::from(""));
                    continue;
                }
                
                // Very basic markdown-like bullet point styling
                let styled_line = if line.starts_with("- ") || line.starts_with("* ") {
                    Line::from(vec![
                        Span::raw("   • "),
                        Span::styled(line[2..].to_string(), text_style),
                    ])
                } else if line.ends_with(':') {
                    Line::from(vec![
                        Span::raw("   "),
                        Span::styled(*line, highlight),
                    ])
                } else {
                    Line::from(vec![
                        Span::raw("   "),
                        Span::styled(*line, text_style),
                    ])
                };
                
                lines.push(styled_line);
            }
        }

        Paragraph::new(lines).render(area, buf);
    }
}
