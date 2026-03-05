use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use amlich_api::LocalizedListDto;
use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct GuidanceWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> GuidanceWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for GuidanceWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else { return };
        let Some(insight) = &bundle.insight else { return };
        let Some(guidance) = &insight.day_guidance else { return };

        let nen = localized_items(&guidance.good_for);
        let tranh = localized_items(&guidance.avoid_for);

        // If both are empty, nothing to render
        if nen.is_empty() && tranh.is_empty() {
            return;
        }

        // We determine layout based on mode
        let mut lines = vec![];
        
        let header_style = Style::default().fg(Color::DarkGray);
        let nen_style = Style::default().fg(Color::Green);
        let tranh_style = Style::default().fg(Color::Red);
        let text_style = Style::default().fg(Color::White);

        match self.mode {
            LayoutMode::Small => {
                // Stacked 1 column
                if !nen.is_empty() {
                    lines.push(Line::from(vec![
                        Span::styled("── Nên ", header_style),
                        Span::styled(format!("{:─<20}", ""), header_style),
                    ]));
                    for item in nen.iter().take(5) {
                        lines.push(Line::from(vec![
                            Span::styled("  ✓ ", nen_style),
                            Span::styled(item.as_str(), text_style),
                        ]));
                    }
                }
                
                if !tranh.is_empty() {
                    if !nen.is_empty() { lines.push(Line::from("")); }
                    lines.push(Line::from(vec![
                        Span::styled("── Tránh ", header_style),
                        Span::styled(format!("{:─<18}", ""), header_style),
                    ]));
                    for item in tranh.iter().take(5) {
                        lines.push(Line::from(vec![
                            Span::styled("  ✗ ", tranh_style),
                            Span::styled(item.as_str(), text_style),
                        ]));
                    }
                }
            }
            LayoutMode::Medium | LayoutMode::Large => {
                // 2 Column layout. 
                // We fake a 2-column layout in a single paragraph using padding spaces.
                // In a real ratatui app we'd split the Rect, but since this is a scrollable
                // continuous document, inline spacing is easier to map to text height.
                
                let max_len = std::cmp::max(nen.len(), tranh.len());
                // Limit to top 8 items for Medium, all for Large
                let limit = if self.mode == LayoutMode::Medium { 8 } else { 20 };
                let render_len = std::cmp::min(max_len, limit);
                
                let col_width = if self.mode == LayoutMode::Medium { 25 } else { 35 };

                // Header
                lines.push(Line::from(vec![
                    Span::styled("── Nên ", header_style),
                    Span::styled(format!("{:─<width$}", "", width = col_width - 7), header_style),
                    Span::raw("   "),
                    Span::styled("── Tránh ", header_style),
                    Span::styled(format!("{:─<width$}", "", width = col_width - 9), header_style),
                ]));
                
                for i in 0..render_len {
                    let mut line_spans = vec![];
                    
                    // Column 1 (Nên)
                    let n = nen.get(i).map(String::as_str).unwrap_or("");
                    if n.is_empty() {
                        line_spans.push(Span::raw(format!("  {:<width$}", "", width = col_width)));
                    } else {
                        line_spans.push(Span::styled("  ✓ ", nen_style));
                        let padded = format!("{:<width$}", n, width = col_width - 4);
                        line_spans.push(Span::styled(padded, text_style));
                    }
                    
                    line_spans.push(Span::raw("   ")); // gap
                    
                    // Column 2 (Tránh)
                    let t = tranh.get(i).map(String::as_str).unwrap_or("");
                    if !t.is_empty() {
                        line_spans.push(Span::styled("  ✗ ", tranh_style));
                        line_spans.push(Span::styled(t, text_style));
                    }
                    
                    lines.push(Line::from(line_spans));
                }
            }
        }

        Paragraph::new(lines).render(area, buf);
    }
}

fn localized_items(list: &LocalizedListDto) -> &[String] {
    if !list.vi.is_empty() {
        &list.vi
    } else {
        &list.en
    }
}
