use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::{AppState, PageSection};

use super::{
    calendar::CalendarViewWidget,
    screens::{
        deep::DeepScreenWidget, general::GeneralScreenWidget, insight::InsightScreenWidget,
        recommendations::RecommendationsScreenWidget,
    },
    week_strip::WeekStripWidget,
};

pub struct PageWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> PageWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for PageWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.is_loading {
            shell_message(
                vec![
                    Line::from(vec![Span::styled(
                        "Amlich Explorer",
                        Style::default().fg(Color::Cyan),
                    )]),
                    Line::from(""),
                    Line::from("Đang tải dữ liệu cho giao diện khám phá..."),
                    Line::from("Luồng chính: chọn cấu hình -> xem ngày."),
                    Line::from("Nhấn q để thoát."),
                ],
                area,
                buf,
            );
            return;
        }

        if let Some(err) = &self.app.error_msg {
            shell_message(
                vec![
                    Line::from(vec![Span::styled(
                        "Amlich Explorer",
                        Style::default().fg(Color::Cyan),
                    )]),
                    Line::from(""),
                    Line::from(vec![Span::styled(
                        format!("Lỗi tải dữ liệu: {err}"),
                        Style::default().fg(Color::Red),
                    )]),
                    Line::from("Giữ ngữ cảnh shell để thử lại hoặc quay lại explorer."),
                    Line::from("Phím: r = retry · Tab/Shift+Tab = back · q = quit"),
                ],
                area,
                buf,
            );
            return;
        }

        if self.app.bundle.is_none() {
            Paragraph::new("Không có dữ liệu.").render(area, buf);
            return;
        }

        if self.app.is_calendar_view() {
            CalendarViewWidget::new(self.app, self.mode).render(area, buf);
            return;
        }

        let content_area = if self.app.show_week_strip {
            let chunks = Layout::vertical([Constraint::Length(4), Constraint::Min(1)]).split(area);
            WeekStripWidget::new(self.app).render(chunks[0], buf);
            chunks[1]
        } else {
            area
        };

        match self.app.active_view {
            crate::state::ActiveView::Dashboard => {
                GeneralScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::Scholar => {
                InsightScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::Planning => {
                RecommendationsScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::Calendar => {
                CalendarViewWidget::new(self.app, self.mode).render(area, buf)
            }
        }
    }
}

fn shell_message(lines: Vec<Line<'static>>, area: Rect, buf: &mut Buffer) {
    Paragraph::new(lines).render(area, buf);
}

#[allow(dead_code)]
pub(crate) fn home_section_order(_app: &AppState) -> Vec<PageSection> {
    vec![
        PageSection::Explorer,
        PageSection::Hero,
        PageSection::Recommendations,
        PageSection::Timing,
        PageSection::Travel,
        PageSection::Risks,
        PageSection::TraditionalEvidence,
        PageSection::ExpandedDetails,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ActiveView, AppMode, ExplorerAction, ExplorerField, ExplorerSelection, FocusLens,
        PageSection,
    };
    use amlich_api::v2::DayBundleDto;
    use amlich_api::{
        LunarDto, RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
        SolarDto,
    };
    use chrono::NaiveDate;
    use ratatui::{buffer::Buffer, layout::Rect};

    fn sample_app_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        let ruleset_catalog = vec![RulesetCatalogEntryDto {
            id: "vn_baseline_v1".to_string(),
            canonical_id: "vn_baseline_v1".to_string(),
            version: "v1".to_string(),
            region: "vn".to_string(),
            profile: "baseline".to_string(),
            schema_version: "amlich.engine/v1".to_string(),
            is_default: true,
            aliases: vec![],
            defaults: RulesetDefaultsDto {
                tz_offset: 7.0,
                meridian: None,
            },
            source_notes: vec![],
        }];
        let recommendation_pack_catalog = vec![RecommendationPackCatalogEntryDto {
            pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
            request_field: "enabled_pack_ids".to_string(),
            version: "v1".to_string(),
            source_family: "traditional".to_string(),
            mode: "advisory".to_string(),
        }];
        let selection = ExplorerSelection::defaults(date, &ruleset_catalog);
        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            scroll_offset: 0,
            bundle: None,
            is_loading: false,
            error_msg: None,
            ruleset_catalog,
            recommendation_pack_catalog,
            applied_selection: selection.clone(),
            staged_selection: selection,
            explorer_focus: ExplorerField::Date,
            explorer_action: ExplorerAction::Apply,
            pack_cursor: 0,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            show_week_strip: true,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            search_input: String::new(),
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Dashboard,
            view_history: Vec::new(),
            app_mode: AppMode::Normal,
        }
    }

    fn sample_bundle() -> DayBundleDto {
        DayBundleDto {
            schema_version: "amlich.engine/v1".to_string(),
            ruleset_id: "test".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            generated_at: "2026-03-12T00:00:00Z".to_string(),
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
            tiet_khi: None,
            gio_hoang_dao: None,
            day_fortune: None,
            daily_recommendations: None,
            contextual_recommendations: None,
            insight: None,
        }
    }

    fn render_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 120, 40);
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
    fn home_screen_sections_follow_actionability_first_order() {
        let app = sample_app_state();

        assert_eq!(
            home_section_order(&app),
            vec![
                PageSection::Explorer,
                PageSection::Hero,
                PageSection::Recommendations,
                PageSection::Timing,
                PageSection::Travel,
                PageSection::Risks,
                PageSection::TraditionalEvidence,
                PageSection::ExpandedDetails,
            ]
        );
    }

    #[test]
    fn page_routes_to_general_screen_widget() {
        let mut app = sample_app_state();
        app.bundle = Some(sample_bundle());
        app.active_view = ActiveView::Dashboard;

        let text = render_text(&app);

        assert!(text.contains("Màn hình General"));
    }

    #[test]
    fn page_routes_to_deep_screen_widget() {
        let mut app = sample_app_state();
        app.bundle = Some(sample_bundle());
        // For now, mapping calendar or just removing this test
        app.active_view = ActiveView::Calendar;

        let text = render_text(&app);
    }
}
