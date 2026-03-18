use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use crate::theme::Theme;

pub struct StatusStripWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> StatusStripWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }

    fn max_pack_label_width(&self) -> usize {
        match self.mode {
            LayoutMode::Small => 10,
            LayoutMode::Medium => 18,
            LayoutMode::Large => 28,
        }
    }
}

impl Widget for StatusStripWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let date = self.app.date.format("%Y-%m-%d").to_string();
        let ruleset = self
            .app
            .ruleset_brief_label(self.app.applied_selection.ruleset_id.as_deref());
        let event_kind = self
            .app
            .event_kind_label(self.app.applied_selection.event_kind.as_deref());
        let packs = truncate_text(
            &self.app.active_bundle_packs_summary(),
            self.max_pack_label_width(),
        );

        let strip = Line::from(vec![
            Span::styled(
                format!(" {} ", date),
                Theme::accent_info().add_modifier(Modifier::BOLD),
            ),
            Span::styled("│", Theme::text_dim()),
            Span::styled(format!(" r:{} ", ruleset), Theme::text_muted()),
            Span::styled("│", Theme::text_dim()),
            Span::styled(format!(" e:{} ", event_kind), Theme::text_muted()),
            Span::styled("│", Theme::text_dim()),
            Span::styled(format!(" p:{} ", packs), Theme::text_muted()),
        ]);

        Paragraph::new(strip)
            .alignment(Alignment::Center)
            .style(Style::default().bg(Theme::SURFACE_BG))
            .render(area, buf);
    }
}

fn truncate_text(value: &str, max_chars: usize) -> String {
    let char_count = value.chars().count();
    if char_count <= max_chars {
        return value.to_string();
    }

    let keep = max_chars.saturating_sub(1);
    let shortened: String = value.chars().take(keep).collect();
    format!("{}…", shortened)
}

#[cfg(test)]
mod tests {
    use super::truncate_text;

    #[test]
    fn truncate_text_keeps_short_values() {
        assert_eq!(truncate_text("short", 8), "short");
    }

    #[test]
    fn truncate_text_adds_ellipsis_for_long_values() {
        assert_eq!(truncate_text("pack.nhi_thap_bat_tu.v1", 8), "pack.nh…");
    }
}
