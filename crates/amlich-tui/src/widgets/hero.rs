use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use amlich_api::v2::DayBundleDto;
use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct HeroWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> HeroWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for HeroWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else { return };

        // 1. Solar Date Headline (e.g., THỨ BA · 5 THÁNG 3)
        let solar_str = format!(
            "{} · {} THÁNG {}",
            bundle.solar.day_of_week_name.to_uppercase(),
            bundle.solar.day,
            bundle.solar.month
        );

        // 2. Lunar Phase calculation
        // Simple mapping: 1 (🌑), 7/8 (🌓), 15 (🌕), 22/23 (🌗), 30 (🌑)
        let phase_emoji = match bundle.lunar.day {
            1..=3 => "🌑",
            4..=6 => "🌒",
            7..=10 => "🌓",
            11..=13 => "🌔",
            14..=17 => "🌕",
            18..=21 => "🌖",
            22..=25 => "🌗",
            26..=29 => "🌘",
            30 => "🌑",
            _ => "🌙", // Fallback
        };

        // 3. Lunar Date string (e.g., Mùng 7 tháng Giêng, Bính Ngọ)
        let lunar_str = if let Some(canchi) = &bundle.canchi {
            format!(
                "{} {}, {}",
                phase_emoji,
                bundle.lunar.date_string,
                canchi.year.full
            )
        } else {
            format!("{} {}", phase_emoji, bundle.lunar.date_string)
        };

        // 4. Verdict Badge (TL;DR)
        // A simple heuristic for now: count good vs bad hours/stars.
        // In a real app we'd query the specific day events.
        let is_good = bundle
            .gio_hoang_dao
            .as_ref()
            .map(|h| h.good_hour_count >= 6)
            .unwrap_or(true);

        let (verdict_text, verdict_color) = if is_good {
            ("[ Nên tiến hành việc quan trọng ]", Color::Green)
        } else {
            ("[ Ngày bình thường - Cẩn trọng việc lớn ]", Color::Yellow)
        };

        // Render based on mode (centering usually looks best for the hero)
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let lines = vec![
            Line::from(Span::styled(
                solar_str,
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            )),
            Line::from(""), // spacing
            Line::from(Span::styled(lunar_str, Style::default().fg(Color::Cyan))),
            Line::from(""), // spacing
            Line::from(Span::styled(
                verdict_text,
                Style::default().fg(verdict_color),
            )),
        ];

        Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Center)
            .render(area, buf);
    }
}
