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
        day_detail::DayDetailScreenWidget, hours::HoursScreenWidget, today::TodayScreenWidget,
    },
    week_strip::WeekStripWidget,
};

/// Return the natural (ideal) height for the current screen based on its layout constraints.
/// Each value is the sum of Length/Min values from that screen's vertical layout.
pub fn screen_natural_height(app: &AppState, mode: LayoutMode, _area_width: u16) -> u16 {
    use crate::state::ui_prefs::VerbosityMode;

    match (app.active_view, mode, app.active_verbosity()) {
        (crate::state::ActiveView::Today, LayoutMode::Small, VerbosityMode::Compact) => 31,
        (crate::state::ActiveView::Today, LayoutMode::Small, VerbosityMode::Verbose) => 43,
        (crate::state::ActiveView::Today, _, VerbosityMode::Compact) => 35,
        (crate::state::ActiveView::Today, _, VerbosityMode::Verbose) => 36,

        (crate::state::ActiveView::DayDetail, LayoutMode::Small, VerbosityMode::Compact) => 44,
        (crate::state::ActiveView::DayDetail, LayoutMode::Small, VerbosityMode::Verbose) => 76,
        (crate::state::ActiveView::DayDetail, _, VerbosityMode::Compact) => 44,
        (crate::state::ActiveView::DayDetail, _, VerbosityMode::Verbose) => 67,

        (crate::state::ActiveView::Hours, LayoutMode::Small, VerbosityMode::Compact) => 24, // 6+8+10
        (crate::state::ActiveView::Hours, LayoutMode::Small, VerbosityMode::Verbose) => 32, // 6+8+8+10
        (crate::state::ActiveView::Hours, _, VerbosityMode::Compact) => 24, // 6+8+10
        (crate::state::ActiveView::Hours, _, VerbosityMode::Verbose) => 38, // 6+8+6+8+10

        (crate::state::ActiveView::Calendar, _, _) => 40,
        (crate::state::ActiveView::Personal, _, _) => personal_natural_height(app, mode),
    }
}

fn personal_natural_height(app: &AppState, mode: LayoutMode) -> u16 {
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
        crate::state::ActiveView::Today => TodayScreenWidget::new(app, mode).render(area, buf),
        crate::state::ActiveView::DayDetail => {
            DayDetailScreenWidget::new(app, mode).render(area, buf)
        }
        crate::state::ActiveView::Hours => HoursScreenWidget::new(app, mode).render(area, buf),
        crate::state::ActiveView::Calendar => CalendarViewWidget::new(app, mode).render(area, buf),
        crate::state::ActiveView::Personal => {
            crate::widgets::screens::personal::PersonalScreenWidget::new(app, mode)
                .render(area, buf)
        }
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
                    Line::from("Đang tải dữ liệu cho giao diện âm lịch..."),
                    Line::from("Luồng chính: Hôm Nay -> Chi Tiết Ngày -> Giờ Tốt -> Lịch."),
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
                    Line::from("Giữ ngữ cảnh shell để thử lại hoặc quay lại màn trước."),
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
            crate::state::ActiveView::Today => {
                TodayScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::DayDetail => {
                DayDetailScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::Hours => {
                HoursScreenWidget::new(self.app, self.mode).render(content_area, buf)
            }
            crate::state::ActiveView::Calendar => {
                CalendarViewWidget::new(self.app, self.mode).render(area, buf)
            }
            crate::state::ActiveView::Personal => {
                crate::widgets::screens::personal::PersonalScreenWidget::new(self.app, self.mode)
                    .render(content_area, buf)
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
        ActiveView, AppMode, ExplorerAction, ExplorerField, ExplorerSelection, PageSection,
    };
    use amlich_api::v2::DayBundleDto;
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
            personal_focus: crate::state::PersonalField::BirthYear,
            personal_draft: crate::state::PersonalDraft {
                birth_year: String::new(),
                birth_month: String::new(),
                birth_day: String::new(),
                gender: None,
            },
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Today,
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
    fn page_routes_to_today_screen_widget() {
        let mut app = sample_app_state();
        app.bundle = Some(sample_bundle());
        app.active_view = ActiveView::Today;

        let text = render_text(&app);

        assert!(text.contains("Âm lịch"));
    }

    #[test]
    fn page_routes_to_calendar_screen_widget() {
        let mut app = sample_app_state();
        app.bundle = Some(sample_bundle());
        app.active_view = ActiveView::Calendar;

        let _text = render_text(&app);
    }

    #[test]
    fn personal_small_verbose_overlay_reports_full_min_height() {
        let mut app = sample_app_state();
        app.active_view = ActiveView::Personal;
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
            yearly_han: None,
        });
        app.bundle = Some(bundle);

        assert_eq!(screen_natural_height(&app, LayoutMode::Small, 48), 43);
    }
}
