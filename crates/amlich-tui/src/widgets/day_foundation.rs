use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use amlich_api::v2::DayBundleDto;

pub struct DayFoundationWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> DayFoundationWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for DayFoundationWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };
        self.render_evidence(area, buf, bundle);
    }
}

impl<'a> DayFoundationWidget<'a> {
    fn render_evidence(&self, area: Rect, buf: &mut Buffer, bundle: &DayBundleDto) {
        let block = Block::default()
            .title(" Nền Ngày ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);

        let mut lines: Vec<Line<'_>> = vec![];

        let Some(summary) = self.app.day_identity_summary() else {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    "Chưa có dữ liệu khí ngày.",
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
            Span::styled(summary.headline, Style::default().fg(Color::Cyan)),
        ]));

        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(
                "Các dấu hiệu dưới đây giúp giải thích vì sao ngày này thuận hay kỵ.",
                Style::default().fg(Color::DarkGray),
            ),
        ]));

        for detail in summary.detail_lines.iter().take(3) {
            lines.push(Line::from(vec![
                Span::raw("   • "),
                Span::raw(detail.clone()),
            ]));
        }

        if let Some(note) = summary.application_note {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(note, Style::default().fg(Color::Yellow)),
            ]));
        }

        if self.app.show_evidence {
            if let Some(canchi) = &bundle.canchi {
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(
                        format!("Tháng {} · Năm {}", canchi.month.full, canchi.year.full),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));
            }
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
        ActivityLabelDto, DailyRecommendationsDto, DayDeityInsightDto, DayInsightDto,
        LocalizedListDto, LocalizedTextDto, LunarDto, RecommendationBucketDto,
        RecommendationEvidenceDto, RecommendationEvidenceSourceDto, RecommendationReasonDto,
        RecommendationScopeDto, RecommendationSeverityDto, SolarDto, StarsInsightDto,
        SynthesizedRecommendationDto, TrucInsightDto,
    };
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };
    use chrono::NaiveDate;

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
                day_fortune: None,
                daily_recommendations: Some(DailyRecommendationsDto {
                    ruleset_id: "test".to_string(),
                    ruleset_version: "v1".to_string(),
                    profile: "baseline".to_string(),
                    scope: RecommendationScopeDto::GeneralDay,
                    version: "v1".to_string(),
                    summary_vi: "Ngày thử nghiệm".to_string(),
                    summary_en: String::new(),
                    active_packs: vec![],
                    activities: vec![SynthesizedRecommendationDto {
                        activity_id: "opening_start".to_string(),
                        label: ActivityLabelDto {
                            vi: "Khai mở".to_string(),
                            en: "Opening".to_string(),
                        },
                        bucket: RecommendationBucketDto::Nen,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "truc.khai.good".to_string(),
                            severity: RecommendationSeverityDto::Primary,
                            summary_vi: "Hợp trực Khai".to_string(),
                            summary_en: String::new(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::Truc,
                                code: "truc.khai".to_string(),
                                note: "test".to_string(),
                            },
                        }],
                    }],
                }),
                contextual_recommendations: None,
                upcoming_events: vec![],
                insight: Some(DayInsightDto {
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
                    festival: None,
                    holiday: None,
                    canchi: None,
                    day_guidance: None,
                    tiet_khi: None,
                    na_am: None,
                    truc: Some(TrucInsightDto {
                        name: "Khai".to_string(),
                        quality: "cat".to_string(),
                        meaning: LocalizedTextDto {
                            vi: "Tốt cho mở đầu".to_string(),
                            en: "Good for opening".to_string(),
                        },
                        good_for: LocalizedListDto {
                            vi: vec![],
                            en: vec![],
                        },
                        avoid_for: LocalizedListDto {
                            vi: vec![],
                            en: vec![],
                        },
                    }),
                    day_deity: Some(DayDeityInsightDto {
                        name: "Kim Quỹ".to_string(),
                        classification: "hoang_dao".to_string(),
                        classification_meaning: LocalizedTextDto {
                            vi: "Cát thần".to_string(),
                            en: "Good deity".to_string(),
                        },
                        deity_meaning: None,
                    }),
                    stars: Some(StarsInsightDto {
                        cat_tinh: vec!["Thiên Đức".to_string()],
                        sat_tinh: vec!["Thiên Cương".to_string()],
                        day_star: Some("Kim Quỹ".to_string()),
                        day_star_quality: Some("cat".to_string()),
                    }),
                    taboos: None,
                    travel: None,
                    xung_hop: None,
                    tang_can: None,
                    ten_gods: None,
                    hours: None,
                    tu_menh: None,
                    dai_van: None,
                }),
            }),
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
            focused_section: PageSection::TraditionalEvidence,
            zoomed_section: None,
            expanded_sections: Default::default(),
            app_mode: crate::state::AppMode::Normal,
            search_input: String::new(),
            personal_focus: crate::state::PersonalField::BirthYear,
            personal_draft: crate::state::PersonalDraft { birth_year: String::new(), gender: None },
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Today,
            view_history: Vec::new(),
        }
    }

    fn render_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 90, 8);
        let mut buf = Buffer::empty(area);
        DayFoundationWidget::new(app, LayoutMode::Large).render(area, &mut buf);
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
    fn day_foundation_widget_renders_day_identity_section() {
        let app = sample_app_state();
        let text = render_text(&app);

        assert!(text.contains("Nền Ngày"));
        assert!(text.contains("Khí ngày chưa đủ dữ liệu để luận"));
        assert!(!text.contains("Metadata:"));
    }
}
