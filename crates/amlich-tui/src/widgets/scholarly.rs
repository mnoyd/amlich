use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use amlich_api::v2::DayBundleDto;

pub struct ScholarlyWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> ScholarlyWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for ScholarlyWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };
        self.render_evidence(area, buf, bundle);
    }
}

impl<'a> ScholarlyWidget<'a> {
    fn render_evidence(&self, area: Rect, buf: &mut Buffer, bundle: &DayBundleDto) {
        let mut lines: Vec<Line<'_>> = vec![];

        let block = Block::default()
            .title(" Can Chi · Ngũ Hành · Sao ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);

        let can_chi_day = bundle
            .canchi
            .as_ref()
            .map(|canchi| canchi.day.full.clone())
            .unwrap_or_else(|| "chưa có dữ liệu".to_string());
        lines.push(Line::from(vec![
            Span::raw("   Can Chi ngày: "),
            Span::styled(can_chi_day, Style::default().fg(Color::Cyan)),
        ]));

        // CanChi insight detail
        if let Some(insight) = &bundle.insight {
            if let Some(ci) = &insight.canchi {
                lines.push(Line::from(vec![
                    Span::raw("    \u{251C} Can: "),
                    Span::styled(&ci.can.name, Style::default().fg(Color::Cyan)),
                    Span::raw(" \u{2014} "),
                    Span::raw(&ci.can.meaning.vi),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("    \u{2514} Chi: "),
                    Span::styled(&ci.chi.name, Style::default().fg(Color::Cyan)),
                    Span::raw(" \u{2014} "),
                    Span::raw(&ci.chi.meaning.vi),
                    Span::raw(format!(" ({})", ci.chi.animal.vi)),
                ]));
            }
        }

        // Month and year Can Chi
        if let Some(canchi) = &bundle.canchi {
            lines.push(Line::from(vec![
                Span::raw("   Can Chi tháng: "),
                Span::styled(&canchi.month.full, Style::default().fg(Color::Cyan)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("   Can Chi năm: "),
                Span::styled(&canchi.year.full, Style::default().fg(Color::Cyan)),
            ]));
        }

        let ngu_hanh_naam = bundle
            .day_fortune
            .as_ref()
            .map(|fortune| {
                format!(
                    "{} · {}",
                    fortune.day_element.element, fortune.day_element.na_am
                )
            })
            .unwrap_or_else(|| "chưa có dữ liệu".to_string());
        lines.push(Line::from(vec![
            Span::raw("   Ngũ hành/Nạp âm: "),
            Span::styled(ngu_hanh_naam, Style::default().fg(Color::Yellow)),
        ]));

        if let Some(insight) = &bundle.insight {
            if let Some(truc) = &insight.truc {
                lines.push(Line::from(vec![
                    Span::raw("   Trực: "),
                    Span::styled(&truc.name, Style::default().fg(Color::Cyan)),
                    Span::raw(" ("),
                    Span::raw(&truc.quality),
                    Span::raw(")"),
                ]));
            }

            if let Some(stars) = &insight.stars {
                let cat_tinh = stars.cat_tinh.join(", ");
                let cat_str = if stars.cat_tinh.is_empty() {
                    "Không"
                } else {
                    &cat_tinh
                };
                lines.push(Line::from(vec![
                    Span::raw("   Cát tinh: "),
                    Span::styled(cat_str.to_string(), Style::default().fg(Color::Green)),
                ]));

                let sat_tinh = stars.sat_tinh.join(", ");
                let sat_str = if stars.sat_tinh.is_empty() {
                    "Không"
                } else {
                    &sat_tinh
                };
                lines.push(Line::from(vec![
                    Span::raw("   Sát tinh: "),
                    Span::styled(sat_str.to_string(), Style::default().fg(Color::Red)),
                ]));
            }

            if let Some(deity) = &insight.day_deity {
                lines.push(Line::from(vec![
                    Span::raw("   Thần sát: "),
                    Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
                    Span::raw(" ("),
                    Span::raw(&deity.classification),
                    Span::raw(")"),
                ]));
            }
        } else {
            lines.push(Line::from("   Chưa có dữ liệu chứng cứ truyền thống."));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, PageSection};
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
            lens: FocusLens::General,

            scroll_offset: 0,
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
                contextual_recommendations: None, upcoming_events: vec![],
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
            focused_section: PageSection::TraditionalEvidence,
            zoomed_section: None,
            expanded_sections: Default::default(),
            app_mode: crate::state::AppMode::Normal,
            search_input: String::new(),
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Dashboard,
            view_history: Vec::new(),
        }
    }

    fn render_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 90, 8);
        let mut buf = Buffer::empty(area);
        ScholarlyWidget::new(app, LayoutMode::Large).render(area, &mut buf);
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
    fn scholarly_widget_groups_truc_stars_and_deity_as_evidence() {
        let app = sample_app_state();
        let text = render_text(&app);

        assert!(text.contains("Can Chi · Ngũ Hành · Sao"));
        assert!(text.contains("Can Chi ngày:"));
        assert!(text.contains("Ngũ hành/Nạp âm:"));
        assert!(text.contains("Trực:"));
        assert!(text.contains("Cát tinh:"));
        assert!(text.contains("Thần sát:"));
        assert!(!text.contains("Metadata:"));
    }
}
