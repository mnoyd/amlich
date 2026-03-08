use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct TimelineWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> TimelineWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for TimelineWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };
        let Some(hours_data) = &bundle.gio_hoang_dao else {
            return;
        };

        let mut lines = vec![];
        let header_style = Style::default().fg(Color::DarkGray);

        lines.push(Line::from(vec![
            Span::styled("── Giờ Hoàng Đạo ", header_style),
            Span::styled(format!("{:─<40}", ""), header_style),
        ]));

        match self.mode {
            LayoutMode::Small => {
                // Fallback to text list for very narrow screens
                let good_hours: Vec<String> = hours_data
                    .good_hours
                    .iter()
                    .map(|h| h.hour_chi.clone())
                    .collect();
                let text = format!("   Tốt: {}", good_hours.join(" · "));
                lines.push(Line::from(Span::styled(
                    text,
                    Style::default().fg(Color::Green),
                )));
            }
            _ => {
                // Visual Timeline Bar chart for Medium/Large
                // Each cell is exactly 5 chars wide, with 1-char separators (│/┬/┴)
                // ┌─────┬─────┬─────┐  (Top border)
                // │ ██  │     │ ██  │  (Bar row)
                // └─────┴─────┴─────┘  (Bottom border)
                // 23-01 01-03 03-05     (Label row)

                let mut top_border = vec![Span::styled("   ┌", header_style)];
                let mut bar_row = vec![Span::styled("   │", header_style)];
                let mut bot_border = vec![Span::styled("   └", header_style)];
                let mut label_row = vec![Span::raw("    ")];

                for (i, hd) in hours_data.all_hours.iter().enumerate() {
                    // Top border cell (5 dashes)
                    top_border.push(Span::styled("─────", header_style));

                    // Bar cell (5 chars)
                    if hd.is_good {
                        bar_row.push(Span::styled(" ██  ", Style::default().fg(Color::Green)));
                    } else {
                        bar_row.push(Span::styled("     ", header_style));
                    }

                    // Bottom border cell (5 dashes)
                    bot_border.push(Span::styled("─────", header_style));

                    // Separators
                    if i < 11 {
                        top_border.push(Span::styled("┬", header_style));
                        bar_row.push(Span::styled("│", header_style));
                        bot_border.push(Span::styled("┴", header_style));
                    } else {
                        top_border.push(Span::styled("┐", header_style));
                        bar_row.push(Span::styled("│", header_style));
                        bot_border.push(Span::styled("┘", header_style));
                    }

                    // Label cell (5 chars + 1 space gap)
                    let range_short = hd.time_range.replace(":00", "").replace(" - ", "-");
                    label_row.push(Span::styled(
                        format!("{:^5}", range_short),
                        Style::default().fg(Color::DarkGray),
                    ));
                    if i < 11 {
                        label_row.push(Span::raw(" "));
                    }
                }

                lines.push(Line::from(top_border));
                lines.push(Line::from(bar_row));
                lines.push(Line::from(bot_border));
                lines.push(Line::from(label_row));
            }
        }

        Paragraph::new(lines).render(area, buf);
    }
}
