use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::{AppState, FocusLens};

pub struct BattuWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> BattuWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for BattuWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else { return };
        let Some(canchi) = &bundle.canchi else { return };
        let naive = self.app.date;

        // Group 1: The Four Pillars
        let day_str = format!("Ngày   {:<12} {}", canchi.day.full, format_nguhanh(&canchi.day.ngu_hanh.can, &canchi.day.ngu_hanh.chi));
        let month_str = format!("Tháng  {:<12} {}", canchi.month.full, format_nguhanh(&canchi.month.ngu_hanh.can, &canchi.month.ngu_hanh.chi));
        let year_str = format!("Năm    {:<12} {} {}", canchi.year.full, format_nguhanh(&canchi.year.ngu_hanh.can, &canchi.year.ngu_hanh.chi), get_animal_emoji(&canchi.year.chi));

        // In General/Planning lenses, we show basic Can Chi + Nạp Âm.
        // In Scholarly lens, we add Tàng Can and Thập Thần inline.
        let mut lines = vec![];
        
        // Header
        lines.push(Line::from(vec![
            Span::styled("── Bát Tự ", Style::default().fg(Color::DarkGray)),
            Span::styled("──────────────────────────────────────────────────", Style::default().fg(Color::DarkGray)),
        ]));

        if self.app.lens == FocusLens::Scholarly {
            // Scholarly mode: Denser layout
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(day_str, Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!(
                        "      Nạp Âm: {}",
                        bundle
                            .day_fortune
                            .as_ref()
                            .map(|f| f.day_element.na_am.as_str())
                            .unwrap_or("")
                    ),
                    Style::default().fg(Color::Yellow),
                )
            ]));
            
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::raw(month_str),
                Span::styled("      [Tàng Can: ...]", Style::default().fg(Color::DarkGray)), // Stub
            ]));
            
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::raw(year_str),
                Span::styled("      [Thập Thần: ...]", Style::default().fg(Color::DarkGray)), // Stub
            ]));
        } else {
            // Normal mode
            let naam_str = bundle
                .day_fortune
                .as_ref()
                .map(|f| f.day_element.na_am.as_str())
                .unwrap_or("");
            
            // On larger screens, put Na Am on the Day line. On small, put it on its own line.
            if self.mode == LayoutMode::Small {
                lines.push(Line::from(format!("   {}", day_str)));
                lines.push(Line::from(format!("   {}", month_str)));
                lines.push(Line::from(format!("   {}", year_str)));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(format!("   Nạp Âm: {}", naam_str), Style::default().fg(Color::Yellow))));
            } else {
                lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(day_str, Style::default().fg(Color::Cyan)),
                    Span::styled(format!("      Nạp Âm: {}", naam_str), Style::default().fg(Color::Yellow)),
                ]));
                lines.push(Line::from(format!("   {}", month_str)));
                lines.push(Line::from(format!("   {}", year_str)));
            }
        }

        Paragraph::new(lines).render(area, buf);
    }
}

// Helpers
fn format_nguhanh(can: &str, chi: &str) -> String {
    format!("{}·{}", can, chi)
}

fn get_animal_emoji(chi: &str) -> &'static str {
    match chi.to_lowercase().as_str() {
        "tý" => "🐭",
        "sửu" => "🐃",
        "dần" => "🐅",
        "mão" => "🐇",
        "thìn" => "🐉",
        "tỵ" => "🐍",
        "ngọ" => "🐎",
        "mùi" => "🐐",
        "thân" => "🐒",
        "dậu" => "🐓",
        "tuất" => "🐕",
        "hợi" => "🐖",
        _ => "",
    }
}
