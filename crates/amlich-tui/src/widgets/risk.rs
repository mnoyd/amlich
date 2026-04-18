use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct RiskWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> RiskWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for RiskWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Rủi Ro & Kiêng Kỵ ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);
        let mut lines = vec![];

        let risk_board = self.app.day_detail_risk_board();
        if risk_board.headline.is_none()
            && risk_board.critical_items.is_empty()
            && risk_board.caution_items.is_empty()
            && risk_board.conflict_items.is_empty()
        {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled("Chưa có dữ liệu rủi ro.", Style::default().fg(Color::Gray)),
            ]));
            Paragraph::new(lines)
                .wrap(Wrap { trim: true })
                .render(inner, buf);
            return;
        }

        if let Some(headline) = risk_board.headline {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(headline, Style::default().fg(Color::Yellow)),
            ]));
        }

        for item in risk_board.critical_items.iter().take(2) {
            lines.push(Line::from(vec![
                Span::styled("   ! ", Style::default().fg(Color::Red)),
                Span::styled(item.clone(), Style::default().fg(Color::White)),
            ]));
        }
        for item in risk_board.caution_items.iter().take(2) {
            lines.push(Line::from(vec![
                Span::styled("   • ", Style::default().fg(Color::Yellow)),
                Span::styled(item.clone(), Style::default().fg(Color::White)),
            ]));
        }
        if let Some(note) = risk_board.notice {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(note, Style::default().fg(Color::Yellow)),
            ]));
        }
        for item in risk_board.conflict_items.iter().take(2) {
            lines.push(Line::from(vec![
                Span::styled("   ↳ ", Style::default().fg(Color::DarkGray)),
                Span::styled(item.clone(), Style::default().fg(Color::DarkGray)),
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
        ActivityLabelDto, DailyRecommendationsDto, DayConflictDto, DayElementDto, DayFortuneDto,
        DayStarsDto, DayTabooDto, LunarDto, RecommendationBucketDto, RecommendationEvidenceDto,
        RecommendationEvidenceSourceDto, RecommendationReasonDto, RecommendationScopeDto,
        RecommendationSeverityDto, SolarDto, SynthesizedRecommendationDto, TravelDirectionDto,
        TrucDto, XungHopDto,
    };
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };
    use chrono::NaiveDate;

    fn sample_app_state(with_fortune: bool, include_medical: bool) -> AppState {
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
        let mut activities = vec![SynthesizedRecommendationDto {
            activity_id: "groundbreaking".to_string(),
            label: ActivityLabelDto {
                vi: "Động thổ".to_string(),
                en: "Groundbreaking".to_string(),
            },
            bucket: RecommendationBucketDto::KyManh,
            reasons: vec![RecommendationReasonDto {
                rule_id: "taboo.tam_nuong".to_string(),
                severity: RecommendationSeverityDto::Override,
                summary_vi: "Kỵ động thổ".to_string(),
                summary_en: "Avoid groundbreaking".to_string(),
                evidence: RecommendationEvidenceDto {
                    source: RecommendationEvidenceSourceDto::Taboo,
                    code: "taboo.tam_nuong".to_string(),
                    note: "test".to_string(),
                },
            }],
        }];
        if include_medical {
            activities.push(SynthesizedRecommendationDto {
                activity_id: "medical_treatment".to_string(),
                label: ActivityLabelDto {
                    vi: "Điều trị".to_string(),
                    en: "Treatment".to_string(),
                },
                bucket: RecommendationBucketDto::Tranh,
                reasons: vec![],
            });
        }

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
                    taboos: vec![DayTabooDto {
                        rule_id: "taboo.tam_nuong".to_string(),
                        name: "Tam Nương".to_string(),
                        severity: "high".to_string(),
                        reason: "Không hợp việc lớn".to_string(),
                        evidence: None,
                    }],
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
                    activities,
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
            focused_section: PageSection::Risks,
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
        let area = Rect::new(0, 0, 90, 8);
        let mut buf = Buffer::empty(area);
        RiskWidget::new(app, LayoutMode::Large).render(area, &mut buf);
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
    fn risk_widget_prioritizes_ky_manh_taboos_and_major_clashes() {
        let app = sample_app_state(true, false);
        let text = render_text(&app);

        assert!(text.contains("Kỵ mạnh: Động thổ"));
        assert!(text.contains("Kiêng kỵ: Tam Nương"));
        assert!(text.contains("Lục xung: Tý"));
    }

    #[test]
    fn risk_widget_shows_sensitive_domain_note_when_needed() {
        let app = sample_app_state(true, true);
        let text = render_text(&app);

        assert!(text.contains("đánh giá chuyên môn"));
    }

    #[test]
    fn widget_uses_recommendation_risks_when_fortune_data_is_missing() {
        let app = sample_app_state(false, false);
        let text = render_text(&app);

        assert!(text.contains("Kỵ mạnh: Động thổ"));
    }
}
