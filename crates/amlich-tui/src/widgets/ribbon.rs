use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use chrono::Datelike;

const WEEKDAY_NAMES: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

pub struct RibbonWidget<'a> {
    app: &'a AppState,
}

impl<'a> RibbonWidget<'a> {
    pub fn new(app: &'a AppState, _mode: LayoutMode) -> Self {
        Self { app }
    }
}

impl Widget for RibbonWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.is_calendar_view() {
            let line = Line::from(vec![
                Span::styled(
                    " [Lịch] ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "h/l, j/k: di chuyển  ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    "PgUp/PgDn: đổi tháng  ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    "Enter: chọn  Esc/Space: đóng",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            let p = Paragraph::new(line).alignment(Alignment::Center);
            let bottom_line = Rect {
                x: area.x,
                y: area.y + 1,
                width: area.width,
                height: 1,
            };
            p.render(bottom_line, buf);
            return;
        }

        let section_name = match self.app.focused_section {
            crate::state::PageSection::Hero => "Tóm tắt",
            crate::state::PageSection::Recommendations => "Khuyến nghị",
            crate::state::PageSection::Timing => "Khung giờ",
            crate::state::PageSection::Travel => "Xuất hành",
            crate::state::PageSection::Risks => "Rủi ro",
            crate::state::PageSection::TraditionalEvidence => "Chứng cứ",
            crate::state::PageSection::ExpandedDetails => "Chi tiết",
        };

        let dow0 = self.app.date.weekday().num_days_from_monday() as usize;

        let mut spans = vec![
            Span::styled(
                format!(" [{}] ", section_name),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " Tab: mục  Enter: mở  a: khuyến nghị  e: chứng cứ  z: zoom  ",
                Style::default().fg(Color::DarkGray),
            ),
            Span::raw("  ◂ "),
        ];

        for (i, name) in WEEKDAY_NAMES.iter().enumerate() {
            if i == dow0 {
                // Today/Selected
                spans.push(Span::styled(
                    format!("[{}] ", name),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    format!("{} ", name),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }
        spans.push(Span::raw("▸"));

        let p = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
        // The area has 2 height (constraint), we put this on the bottom line.
        let bottom_line = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: 1,
        };
        p.render(bottom_line, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use crate::state::{FocusLens, PageSection, ViewMode};

    fn sample_app_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: None,
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            focused_section: PageSection::Recommendations,
            zoomed_section: None,
            expanded_sections: Default::default(),
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
        }
    }

    fn render_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 120, 2);
        let mut buf = Buffer::empty(area);
        RibbonWidget::new(app, LayoutMode::Large).render(area, &mut buf);
        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn ribbon_shows_new_focus_expand_evidence_controls() {
        let app = sample_app_state();
        let text = render_text(&app);

        assert!(text.contains("Tab: mục"));
        assert!(text.contains("Enter: mở"));
        assert!(text.contains("a: khuyến nghị"));
        assert!(text.contains("e: chứng cứ"));
        assert!(text.contains("z: zoom"));
    }
}
