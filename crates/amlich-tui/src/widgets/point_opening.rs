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

    /// The committed canonical goldens — the single byte-locked citation
    /// wording every surface renders verbatim (core suite, API transport,
    /// and the desktop inspector lock the same bytes).
    const GOLDEN_OPEN: &str =
        include_str!("../../../amlich-core/data/ty-ngo-luu-chu/terminal-citation-open.txt");
    const GOLDEN_CLOSED: &str =
        include_str!("../../../amlich-core/data/ty-ngo-luu-chu/terminal-citation-closed.txt");

    /// The extended action/efficacy lexicon shared with the core golden
    /// guards; the rendered terminal surface must survive it identically.
    const FORBIDDEN_PHRASES: &[&str] = &[
        "best time to treat",
        "best hour to treat",
        "best hour",
        "should be needled",
        "should be pressed",
        "should needle",
        "should stimulate",
        "recommended point",
        "optimal point",
        "nên châm",
        "nên bấm",
        "nên cứu",
        "nên kích thích",
        "hãy châm",
        "hãy bấm",
        "điểm nên ",
        "công dụng",
        "chữa",
        "điều trị",
        "thải độc",
        "liều lượng",
        "hoạt động mạnh nhất",
        "đạt đỉnh",
    ];

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

    /// Read the paragraph rows inside the border (no border glyphs), one
    /// string per inner row.
    fn inner_rows(buf: &Buffer, area: Rect) -> Vec<String> {
        let inner = Rect::new(area.x + 1, area.y + 1, area.width - 2, area.height - 2);
        (inner.y..inner.y + inner.height)
            .map(|y| {
                (inner.x..inner.x + inner.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
            })
            .collect()
    }

    /// Render on a terminal wide enough that no line wraps, then read the
    /// paragraph rows inside the border. Each row must equal its canonical
    /// citation line — wide CJK glyphs pad the per-column join with
    /// spaces, so rows are compared space-stripped (the repo's `dense`
    /// convention); every other byte is identical.
    fn render_wide_rows(context: &DayPointOpeningContext) -> Vec<String> {
        let area = Rect::new(0, 0, 400, 40);
        let mut buf = Buffer::empty(area);
        PointOpeningWidget::new(context).render(area, &mut buf);
        let mut rows: Vec<String> = inner_rows(&buf, area)
            .into_iter()
            .map(|row| row.trim().to_string())
            .collect();
        while rows.last().is_some_and(String::is_empty) {
            rows.pop();
        }
        rows
    }

    /// Collapse wrapping whitespace so phrase scans are wrap-invariant,
    /// then strip the two disclaimer v2 negation frames (byte-locked
    /// separately by the core contract guard) before scanning.
    fn rendered_text_is_safe(label: &str, rendered: &str) {
        let dense = rendered.split_whitespace().collect::<Vec<_>>().join(" ");
        let without_disclaimers = dense
            .replace(
                amlich_core::point_opening::DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN,
                "",
            )
            .replace(
                amlich_core::point_opening::DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN,
                "",
            )
            .to_lowercase();
        for phrase in FORBIDDEN_PHRASES {
            let phrase = phrase.trim_end();
            assert!(
            !without_disclaimers.contains(phrase),
            "{label}: prohibited action/efficacy phrasing `{phrase}` in the rendered terminal surface"
        );
        }
    }

    /// The narrow render's inner buffer text (wrapping included, border
    /// glyphs excluded) — the safety-scan input.
    fn render_narrow_inner(context: &DayPointOpeningContext) -> String {
        let area = Rect::new(0, 0, 40, 60);
        let mut buf = Buffer::empty(area);
        PointOpeningWidget::new(context).render(area, &mut buf);
        inner_rows(&buf, area).join("\n")
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

    /// `amlich-xlag.2.3.4` — the terminal surface is byte-locked to the
    /// committed goldens: rendered (unwrapped) rows equal the golden
    /// citation lines exactly, for both the open and the closed
    /// fixture. The same bytes are locked by the core suite, the API
    /// transport, and the desktop inspector.
    #[test]
    fn wide_render_matches_the_committed_goldens_line_for_line() {
        let open =
            resolve_day_point_opening_context(JD_GIAP_DAY, 19, 30).expect("open fixture resolves");
        let golden_open_lines: Vec<String> = GOLDEN_OPEN
            .lines()
            .map(|line| line.replace(' ', ""))
            .collect();
        assert_eq!(
            render_wide_rows(&open)
                .iter()
                .map(|row| row.replace(' ', ""))
                .collect::<Vec<_>>(),
            golden_open_lines,
            "the rendered open citation must match the committed golden (dense compare)"
        );

        let closed =
            resolve_day_point_opening_context(JD_GIAP_DAY, 0, 30).expect("closed fixture resolves");
        let golden_closed_lines: Vec<String> = GOLDEN_CLOSED
            .lines()
            .map(|line| line.replace(' ', ""))
            .collect();
        assert_eq!(
            render_wide_rows(&closed)
                .iter()
                .map(|row| row.replace(' ', ""))
                .collect::<Vec<_>>(),
            golden_closed_lines,
            "the rendered closed citation must match the committed golden (dense compare)"
        );
    }

    /// `amlich-xlag.2.3.4` — the actually-rendered terminal buffers
    /// (wrapping included) carry no clinical or action language on
    /// both fixture states; the widget itself adds only the section
    /// title, never wording.
    #[test]
    fn rendered_buffers_carry_no_clinical_or_action_language() {
        let open =
            resolve_day_point_opening_context(JD_GIAP_DAY, 19, 30).expect("open fixture resolves");
        rendered_text_is_safe("open narrow", &render_narrow_inner(&open));
        rendered_text_is_safe("open wide", &render_wide_rows(&open).join("\n"));

        let closed =
            resolve_day_point_opening_context(JD_GIAP_DAY, 0, 30).expect("closed fixture resolves");
        rendered_text_is_safe("closed narrow", &render_narrow_inner(&closed));
        rendered_text_is_safe("closed wide", &render_wide_rows(&closed).join("\n"));

        // Every hourly state of the fixture day, narrow-rendered.
        for hour in 0u8..=23 {
            let context =
                resolve_day_point_opening_context(JD_GIAP_DAY, hour, 30).expect("valid moment");
            rendered_text_is_safe(
                &format!("hour {hour}:30 narrow"),
                &render_narrow_inner(&context),
            );
        }
    }
}
