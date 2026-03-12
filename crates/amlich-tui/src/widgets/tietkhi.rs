use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
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
        let Some(bundle) = &self.app.bundle else {
            return;
        };
        let Some(tietkhi) = &bundle.tiet_khi else {
            return;
        };

        let mut lines: Vec<Line<'_>> = vec![];

        let text_style = Style::default().fg(Color::White);
        let highlight = Style::default().fg(Color::Yellow);

        let expand_hint = if self.app.show_tietkhi_details {
            "▼ Thu gọn (Enter)"
        } else {
            "▶ Chi tiết (Enter)"
        };

        let title = format!(" Tiết Khí Tham Chiếu [{}] ", expand_hint);
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
            
        let inner = block.inner(area);
        block.render(area, buf);

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
                    Line::from(vec![Span::raw("   "), Span::styled(*line, highlight)])
                } else {
                    Line::from(vec![Span::raw("   "), Span::styled(*line, text_style)])
                };

                lines.push(styled_line);
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::{LunarDto, SolarDto, TietKhiDto};
    use amlich_api::v2::{ApiMetaDto, DayBundleDto};
    use chrono::NaiveDate;
    use crate::state::{FocusLens, PageSection, ViewMode};
    use crate::widgets::page::PageWidget;

    fn sample_app_state(expanded: bool) -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        let mut expanded_sections = std::collections::BTreeSet::new();
        if expanded {
            expanded_sections.insert(PageSection::TraditionalEvidence);
        }

        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: Some(DayBundleDto {
                meta: ApiMetaDto {
                    schema_version: "amlich.api/v2".to_string(),
                    ruleset_id: "test".to_string(),
                    ruleset_version: "v1".to_string(),
                    profile: "baseline".to_string(),
                    generated_at: "2026-03-12T00:00:00Z".to_string(),
                },
                solar: SolarDto {
                    day: 12,
                    month: 3,
                    year: 2026,
                    day_of_week: 4,
                    day_of_week_name: "Thứ Năm".to_string(),
                    date_string: "2026-03-12".to_string(),
                },
                lunar: LunarDto {
                    day: 4,
                    month: 2,
                    year: 2026,
                    is_leap_month: false,
                    date_string: "Mùng 4 tháng Hai".to_string(),
                },
                jd: 0,
                canchi: None,
                tiet_khi: Some(TietKhiDto {
                    index: 3,
                    name: "Kinh Trập".to_string(),
                    description: "Tóm tắt\n- Giai đoạn chuyển mùa\n- Nên giữ nhịp sinh hoạt đều".to_string(),
                    longitude: 345,
                    current_longitude: 345.0,
                    season: "Xuân".to_string(),
                }),
                gio_hoang_dao: None,
                day_fortune: None,
                daily_recommendations: None,
                insight: None,
            }),
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: expanded,
            show_evidence: false,
            focused_section: PageSection::TraditionalEvidence,
            zoomed_section: None,
            expanded_sections,
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
        }
    }

    fn render_tiet_khi(app: &AppState) -> String {
        let area = Rect::new(0, 0, 90, 8);
        let mut buf = Buffer::empty(area);
        TietKhiWidget::new(app, LayoutMode::Large).render(area, &mut buf);
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

    fn render_page(app: &AppState) -> String {
        let area = Rect::new(0, 0, 100, 40);
        let mut buf = Buffer::empty(area);
        PageWidget::new(app, LayoutMode::Large).render(area, &mut buf);
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
    fn tietkhi_widget_collapses_to_summary_and_expands_details() {
        let collapsed = render_tiet_khi(&sample_app_state(false));
        let expanded = render_tiet_khi(&sample_app_state(true));

        assert!(collapsed.contains("Tiết Khí Tham Chiếu"));
        assert!(collapsed.contains("Kinh Trập"));
        assert!(!collapsed.contains("Giai đoạn chuyển mùa"));
        assert!(expanded.contains("Giai đoạn chuyển mùa"));
    }

    #[test]
    fn evidence_sections_respect_focus_and_zoom_flags() {
        let mut app = sample_app_state(true);
        app.zoomed_section = Some(PageSection::TraditionalEvidence);

        let text = render_page(&app);

        assert!(text.contains("Chứng Cứ Truyền Thống"));
        assert!(text.contains("Tiết Khí Tham Chiếu"));
        assert!(!text.contains("Khung Giờ Và Hành Động"));
    }
}
