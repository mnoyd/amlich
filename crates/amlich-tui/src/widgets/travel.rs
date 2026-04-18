use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct TravelWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> TravelWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for TravelWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let body_style = Style::default().fg(Color::White);
        let block = Block::default()
            .title(" Hướng / Xuất Hành ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);
        let mut lines = vec![];

        let Some(fortune) = self
            .app
            .bundle
            .as_ref()
            .and_then(|bundle| bundle.day_fortune.as_ref())
        else {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    "Chưa có dữ liệu xuất hành.",
                    Style::default().fg(Color::Gray),
                ),
            ]));
            Paragraph::new(lines)
                .wrap(Wrap { trim: true })
                .render(inner, buf);
            return;
        };

        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(
                format!("Xuất hành: {}", fortune.travel.xuat_hanh_huong),
                body_style,
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(format!("Hỷ Thần: {}", fortune.travel.hy_than), body_style),
        ]));
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(format!("Tài Thần: {}", fortune.travel.tai_than), body_style),
        ]));
        if fortune.travel.evidence.is_none() {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    "Chứng cứ hướng đi hiện chưa đầy đủ.",
                    Style::default().fg(Color::Gray),
                ),
            ]));
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ExplorerAction, ExplorerField, ExplorerSelection, PageSection};
    use amlich_api::v2::DayBundleDto;
    use amlich_api::{
        DailyRecommendationsDto, DayConflictDto, DayElementDto, DayFortuneDto, DayStarsDto,
        LunarDto, RecommendationScopeDto, SolarDto, TravelDirectionDto, TrucDto, XungHopDto,
    };
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };
    use chrono::NaiveDate;

    fn sample_app_state(with_fortune: bool) -> AppState {
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
            bundle: Some(DayBundleDto {
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
                day_fortune: with_fortune.then_some(DayFortuneDto {
                    ruleset_id: "test".to_string(),
                    ruleset_version: "v1".to_string(),
                    profile: "baseline".to_string(),
                    day_element: DayElementDto {
                        na_am: "Thiên Hà Thủy".to_string(),
                        element: "Thủy".to_string(),
                        can_element: "Hỏa".to_string(),
                        chi_element: "Hỏa".to_string(),
                        evidence: None,
                    },
                    conflict: DayConflictDto {
                        opposing_chi: "Tý".to_string(),
                        opposing_con_giap: "Chuột".to_string(),
                        tuoi_xung: vec![],
                        sat_huong: "Bắc".to_string(),
                        evidence: None,
                    },
                    travel: TravelDirectionDto {
                        xuat_hanh_huong: "Đông Nam".to_string(),
                        tai_than: "Chính Nam".to_string(),
                        hy_than: "Đông Bắc".to_string(),
                        evidence: None,
                    },
                    stars: DayStarsDto {
                        cat_tinh: vec![],
                        sat_tinh: vec![],
                        day_star: None,
                        star_system: None,
                        evidence: None,
                        matched_rules: vec![],
                    },
                    day_deity: None,
                    taboos: vec![],
                    xung_hop: XungHopDto {
                        luc_xung: "Tý".to_string(),
                        tam_hop: vec![],
                        tu_hanh_xung: vec![],
                        liu_he: None,
                        xiang_hai: None,
                        xiang_xing: None,
                    },
                    truc: TrucDto {
                        index: 4,
                        name: "Khai".to_string(),
                        quality: "cat".to_string(),
                        evidence: None,
                    },
                    tang_can: None,
                    ten_gods: None,
                    tu_menh: None,
                }),
                daily_recommendations: Some(DailyRecommendationsDto {
                    ruleset_id: "test".to_string(),
                    ruleset_version: "v1".to_string(),
                    profile: "baseline".to_string(),
                    scope: RecommendationScopeDto::GeneralDay,
                    version: "v1".to_string(),
                    summary_vi: "Tóm tắt".to_string(),
                    summary_en: "Summary".to_string(),
                    active_packs: vec![],
                    activities: vec![],
                }),
                contextual_recommendations: None,
                insight: None,
                upcoming_events: vec![],
            }),
            personal_matrix: None,
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
            focused_section: PageSection::Travel,
            zoomed_section: None,
            expanded_sections: Default::default(),
            app_mode: crate::state::AppMode::Normal,
            search_input: String::new(),
            personal_focus: crate::state::PersonalField::BirthYear,
            personal_draft: crate::state::PersonalDraft {
                birth_year: String::new(),
                birth_month: String::new(),
                birth_day: String::new(),
                birth_hour: String::new(),
                birth_minute: String::new(),
                gender: None,
            },
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Today,
            view_history: Vec::new(),
        }
    }

    fn render_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 80, 6);
        let mut buf = Buffer::empty(area);
        TravelWidget::new(app, LayoutMode::Large).render(area, &mut buf);
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
    fn travel_widget_shows_hy_than_tai_than_and_xuat_hanh() {
        let app = sample_app_state(true);
        let text = render_text(&app);

        assert!(text.contains("Hỷ Thần: Đông Bắc"));
        assert!(text.contains("Tài Thần: Chính Nam"));
        assert!(text.contains("Xuất hành: Đông Nam"));
    }

    #[test]
    fn widgets_render_empty_state_when_fortune_data_is_missing() {
        let app = sample_app_state(false);
        let text = render_text(&app);

        assert!(text.contains("Chưa có dữ liệu xuất hành."));
    }
}
