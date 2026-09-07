//! v1.11 Tý Ngọ Lưu Chú point-opening citation section (bead
//! `amlich-xlag.2.3.2`).
//!
//! Renders the canonical citation block
//! (`amlich_core::point_opening::point_opening_citation_lines`)
//! verbatim — the same strings the desktop inspector renders — inside
//! a bordered section on the default Today surface. The wording is
//! historical citation only: point identity, class, review state,
//! time basis, divergences, and disclaimer v2 stay visible on both
//! open and closed states, and no action language is ever introduced
//! here.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use amlich_core::point_opening::{point_opening_citation_lines, DayPointOpeningContext};

/// Minimum rows the citation section reserves on small (narrow)
/// layouts — the bilingual disclaimer wraps hard at ~38 columns.
pub const POINT_OPENING_SECTION_HEIGHT_SMALL: u16 = 50;
/// Minimum rows the citation section reserves on medium/large layouts.
pub const POINT_OPENING_SECTION_HEIGHT_STANDARD: u16 = 30;

/// The section height for a layout mode (0 rows when absent is handled
/// by the caller).
pub fn point_opening_section_height(small: bool) -> u16 {
    if small {
        POINT_OPENING_SECTION_HEIGHT_SMALL
    } else {
        POINT_OPENING_SECTION_HEIGHT_STANDARD
    }
}

pub struct PointOpeningWidget<'a> {
    context: &'a DayPointOpeningContext,
}

impl<'a> PointOpeningWidget<'a> {
    pub fn new(context: &'a DayPointOpeningContext) -> Self {
        Self { context }
    }
}

impl Widget for PointOpeningWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Tý Ngọ Lưu Chú ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let lines = point_opening_citation_lines(self.context);
        let mut text_lines: Vec<Line<'_>> = Vec::with_capacity(lines.len());
        for (index, line) in lines.iter().enumerate() {
            let style = if index == 0 {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if line.starts_with("Miễn trừ (") {
                Style::default().fg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            };
            text_lines.push(Line::from(Span::styled(format!("  {line}"), style)));
        }

        Paragraph::new(text_lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_core::point_opening::resolve_day_point_opening_context;
    use ratatui::widgets::Widget;

    /// 2024-02-10 — the shared Giáp-day fixture; 19:30 is the frozen
    /// open slot (甲/戌 → 竅陰 GB44), 00:30 the explicit closed one.
    const JD_GIAP_DAY: i32 = 2_460_351;

    fn buffer_text(buf: &Buffer, area: Rect) -> String {
        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn render_narrow(context: &DayPointOpeningContext) -> String {
        // Narrow terminal width (the Small layout floor is 40 cols).
        let area = Rect::new(0, 0, 40, 60);
        let mut buf = Buffer::empty(area);
        PointOpeningWidget::new(context).render(area, &mut buf);
        buffer_text(&buf, area)
    }

    #[test]
    fn open_state_renders_the_canonical_citation_on_a_narrow_terminal() {
        let context =
            resolve_day_point_opening_context(JD_GIAP_DAY, 19, 30).expect("open fixture resolves");
        let text = render_narrow(&context);
        // Wide CJK glyphs pad the per-column join with spaces, so CJK
        // tokens are asserted against a space-stripped copy.
        let dense = text.replace(' ', "");

        // Wrap-safe token assertions: the exact wording is byte-locked
        // by the core golden suite; here we prove the canonical lines
        // all reach a 40-column terminal.
        assert!(text.contains("Tý Ngọ Lưu Chú"));
        assert!(text.contains("trích dẫn lịch sử"));
        assert!(dense.contains("竅陰"));
        assert!(text.contains("Kiếu"));
        assert!(text.contains("[GB44]"));
        assert!(text.contains("Đởm"));
        assert!(dense.contains("(井金)"));
        assert!(text.contains("PENDING_CLASSICAL_REVIEW"));
        assert!(text.contains("ExternalReviewPending("));
        assert!(text.contains("local_civil_hour_branch"));
        assert!(text.contains("TNLC-DIV-01"));
        assert!(text.contains("historical_procedural_citation_v1"));
        assert!(text.contains("trì hoãn"));
        assert!(text.contains("acupuncture"));
    }

    #[test]
    fn closed_state_renders_explicitly_closed_with_all_disclosures() {
        let context =
            resolve_day_point_opening_context(JD_GIAP_DAY, 0, 30).expect("closed fixture resolves");
        let text = render_narrow(&context);
        let dense = text.replace(' ', "");

        assert!(dense.contains("閉穴"));
        assert!(text.contains("đóng"));
        assert!(dense.contains("得時為之開"));
        assert!(text.contains("ExternalReviewPending("));
        assert!(text.contains("historical_procedural_citation_v1"));
        assert!(!text.contains("[GB44]"));
        assert!(!dense.contains("竅陰"));
    }

    #[test]
    fn rendered_text_is_deterministic_across_renders() {
        let context =
            resolve_day_point_opening_context(JD_GIAP_DAY, 19, 30).expect("open fixture resolves");
        assert_eq!(render_narrow(&context), render_narrow(&context));
    }
}
