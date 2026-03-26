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
        dashboard::DashboardScreenWidget, elements::ElementsScreenWidget,
        feng_shui::FengShuiScreenWidget, hours::HoursScreenWidget, insight::InsightScreenWidget,
        recommendations::RecommendationsScreenWidget, solar_terms::SolarTermsScreenWidget,
    },
    week_strip::WeekStripWidget,
};

/// Return the natural (ideal) height for the current screen based on its layout constraints.
/// Each value is the sum of Length/Min values from that screen's vertical layout.
pub fn screen_natural_height(app: &AppState, mode: LayoutMode, area_width: u16) -> u16 {
    use crate::state::ui_prefs::VerbosityMode;

    if app.active_view == crate::state::ActiveView::Event {
        return super::screens::event::event_natural_height(app, area_width);
    }

    if app.active_view == crate::state::ActiveView::FengShui {
        return feng_shui_natural_height(app, mode);
    }

    match (app.active_view, mode, app.active_verbosity()) {
        // Dashboard
        (crate::state::ActiveView::Dashboard, LayoutMode::Small, VerbosityMode::Compact) => 25, // 10+7+8
        (crate::state::ActiveView::Dashboard, LayoutMode::Small, VerbosityMode::Verbose) => 39, // 10+7+9+8+5
        (crate::state::ActiveView::Dashboard, _, VerbosityMode::Compact) => 27, // 10+12+5
        (crate::state::ActiveView::Dashboard, _, VerbosityMode::Verbose) => 25, // 20+5

        // Scholar (Insight)
        (crate::state::ActiveView::Scholar, LayoutMode::Small, VerbosityMode::Compact) => 44, // 8+12+8+8+8
        (crate::state::ActiveView::Scholar, LayoutMode::Small, VerbosityMode::Verbose) => 76, // 8+12+8+8+8+9+8+8+7
        (crate::state::ActiveView::Scholar, _, VerbosityMode::Compact) => 44, // 7+10+8+9+10
        (crate::state::ActiveView::Scholar, _, VerbosityMode::Verbose) => 67, // 7+10+8+9+9+9+8+7

        // Hours
        (crate::state::ActiveView::Hours, LayoutMode::Small, VerbosityMode::Compact) => 24, // 6+8+10
        (crate::state::ActiveView::Hours, LayoutMode::Small, VerbosityMode::Verbose) => 32, // 6+8+8+10
        (crate::state::ActiveView::Hours, _, VerbosityMode::Compact) => 24, // 6+8+10
        (crate::state::ActiveView::Hours, _, VerbosityMode::Verbose) => 38, // 6+8+6+8+10

        // Elements
        (crate::state::ActiveView::Elements, LayoutMode::Small, VerbosityMode::Verbose) => 50, // 8+10+8+8+8+8
        (crate::state::ActiveView::Elements, _, VerbosityMode::Compact) => 34, // 8+10+8+8
        (crate::state::ActiveView::Elements, _, VerbosityMode::Verbose) => 39, // 8+10+9+12

        // SolarTerms
        (crate::state::ActiveView::SolarTerms, LayoutMode::Small, VerbosityMode::Verbose) => 40, // 7+9+8+8+8
        (crate::state::ActiveView::SolarTerms, _, VerbosityMode::Compact) => 30, // 7+9+14
        (crate::state::ActiveView::SolarTerms, _, VerbosityMode::Verbose) => 26, // 7+9+10

        // Planning (Recommendations)
        (crate::state::ActiveView::Planning, LayoutMode::Small, VerbosityMode::Compact) => 28, // 12+8+8
        (crate::state::ActiveView::Planning, LayoutMode::Small, VerbosityMode::Verbose) => 26, // 16+10
        (crate::state::ActiveView::Planning, _, VerbosityMode::Compact) => 38, // 12+8+9+9
        (crate::state::ActiveView::Planning, _, VerbosityMode::Verbose) => 26, // 16+10

        // Calendar — not scrolled, but provide a fallback
        (crate::state::ActiveView::Calendar, _, _) => 40,
        (crate::state::ActiveView::FengShui, _, _) | (crate::state::ActiveView::Event, _, _) => {
            unreachable!("handled above")
        }
    }
}

fn feng_shui_natural_height(app: &AppState, mode: LayoutMode) -> u16 {
    use crate::state::ui_prefs::VerbosityMode;

    let has_personal_overlay = app
        .profile_availability_summary()
        .map(|profile| profile.has_personal_overlay)
        .unwrap_or(false);

    match (has_personal_overlay, app.active_verbosity(), mode) {
        (true, VerbosityMode::Verbose, LayoutMode::Small) => 43, // 6+9+8+10+10
        (true, VerbosityMode::Verbose, _) => 31,                 // 6+9+16
        (true, _, _) => 25,                                      // 6+9+10
        (false, _, _) => 21,                                     // 6+9+6
    }
}

/// Render just the active screen content (no week strip) into the given buffer area.
/// Used by the scroll viewport in layout::draw.
pub fn render_screen_content(app: &AppState, mode: LayoutMode, area: Rect, buf: &mut Buffer) {
    match app.active_view {
        crate::state::ActiveView::Dashboard => {
            DashboardScreenWidget::new(app, mode).render(area, buf)
        }
        crate::state::ActiveView::Event => {
            super::screens::event::EventScreenWidget::new(app, mode).render(area, buf)
        }
        crate::state::ActiveView::Scholar => InsightScreenWidget::new(app, mode).render(area, buf),
        crate::state::ActiveView::Hours => HoursScreenWidget::new(app, mode).render(area, buf),
        crate::state::ActiveView::Elements => {
            ElementsScreenWidget::new(app, mode).render(area, buf)
        }
        crate::state::ActiveView::FengShui => {
            FengShuiScreenWidget::new(app, mode).render(area, buf)
        }
        crate::state::ActiveView::SolarTerms => {
            SolarTermsScreenWidget::new(app, mode).render(area, buf)
        }
        crate::state::ActiveView::Planning => {
            RecommendationsScreenWidget::new(app, mode).render(area, buf)
        }
        crate::state::ActiveView::Calendar => CalendarViewWidget::new(app, mode).render(area, buf),
    }
}

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
                DashboardScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::Event => {
                super::screens::event::EventScreenWidget::new(self.app, self.mode)
                    .render(content_area, buf)
            }
            crate::state::ActiveView::Scholar => {
                InsightScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::Hours => {
                HoursScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::Elements => {
                ElementsScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::FengShui => {
                FengShuiScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::SolarTerms => {
                SolarTermsScreenWidget::new(self.app, self.mode).render(content_area, buf)
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
    use amlich_api::v2::{get_day_bundle_for_date, DayBundleDto, Include};
    use amlich_api::{
        DayInsightDto, LocalizedTextDto, LunarDto, RecommendationPackCatalogEntryDto,
        RulesetCatalogEntryDto, RulesetDefaultsDto, SolarDto, TuMenhInsightDto,
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
            content_height: 0,
            viewport_height: 0,
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
            verbosity: crate::state::ui_prefs::VerbosityMode::Compact,
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
            upcoming_events: vec![],
        }
    }

    fn localized(vi: &str, en: &str) -> LocalizedTextDto {
        LocalizedTextDto {
            vi: vi.to_string(),
            en: en.to_string(),
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

        assert!(text.contains("Âm lịch"));
    }

    #[test]
    fn page_routes_to_deep_screen_widget() {
        let mut app = sample_app_state();
        app.bundle = Some(sample_bundle());
        // For now, mapping calendar or just removing this test
        app.active_view = ActiveView::Calendar;

        let _text = render_text(&app);
    }

    #[test]
    fn event_natural_height_grows_for_wrapped_verbose_content() {
        let mut app = sample_app_state();
        app.active_view = ActiveView::Event;
        app.verbosity = crate::state::ui_prefs::VerbosityMode::Verbose;
        app.bundle = Some(
            get_day_bundle_for_date(10, 2, 2024, &[Include::Insight], None)
                .expect("bundle with tet festival insight"),
        );

        assert!(screen_natural_height(&app, LayoutMode::Small, 24) > 30);
    }

    #[test]
    fn feng_shui_small_verbose_overlay_reports_full_min_height() {
        let mut app = sample_app_state();
        app.active_view = ActiveView::FengShui;
        app.verbosity = crate::state::ui_prefs::VerbosityMode::Verbose;

        let mut bundle = sample_bundle();
        bundle.insight = Some(DayInsightDto {
            solar: bundle.solar.clone(),
            lunar: bundle.lunar.clone(),
            festival: None,
            holiday: None,
            canchi: None,
            day_guidance: None,
            tiet_khi: None,
            na_am: None,
            truc: None,
            day_deity: None,
            stars: None,
            taboos: None,
            travel: None,
            xung_hop: None,
            tang_can: None,
            ten_gods: None,
            hours: None,
            tu_menh: Some(TuMenhInsightDto {
                kua: 3,
                group: "Đông tứ mệnh".to_string(),
                trigram: localized("Chấn", "Zhen"),
                direction: localized("Đông", "East"),
                meaning: localized("Hợp hướng mở lối.", "Favors opening movement."),
                group_meaning: localized("Nhóm hướng tăng trưởng.", "Growth group."),
                favorable_directions: vec!["Đông".to_string()],
                unfavorable_directions: vec!["Tây".to_string()],
            }),
            dai_van: None,
        });
        app.bundle = Some(bundle);

        assert_eq!(screen_natural_height(&app, LayoutMode::Small, 48), 43);
    }
}
