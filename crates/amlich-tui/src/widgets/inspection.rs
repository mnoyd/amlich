use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{
    layout::LayoutMode,
    state::{AppState, RecommendationLayerKind},
};

pub struct InspectionWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> InspectionWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for InspectionWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let block = Block::default()
            .title(" Cách Luận / Nguồn Dữ Liệu ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let recommendation_layers = self.app.recommendation_layers();
        let active_packs = self.app.active_bundle_packs();

        let mut lines = vec![
            Line::from(vec![
                Span::styled(
                    "Bộ quy tắc đang dùng: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    "{}@{} · {}",
                    bundle.ruleset_id, bundle.ruleset_version, bundle.profile
                )),
            ]),
            Line::from(vec![
                Span::styled(
                    "Ngữ cảnh xem ngày: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(self.app.inspection_context_summary()),
            ]),
            Line::from(vec![
                Span::styled(
                    "Gói tăng cường: ",
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(self.app.active_bundle_packs_summary()),
            ]),
        ];

        for layer in recommendation_layers.iter().take(2) {
            let layer_label = match layer.kind {
                RecommendationLayerKind::Contextual => "Luận theo mục đích",
                RecommendationLayerKind::Baseline => "Luận nền",
            };
            let pack_label = if layer.active_pack_ids.is_empty() {
                "packs=none".to_string()
            } else {
                format!("packs={}", layer.active_pack_ids.join(", "))
            };
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{layer_label}: "),
                    Style::default().fg(Color::Yellow),
                ),
                Span::raw(format!("{} · {}", layer.summary, pack_label)),
            ]));
        }

        if self.app.show_evidence {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled(
                    "Chứng cứ kỹ thuật: ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("chi tiết tổng hợp phía sau màn hình"),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  • "),
                Span::raw(format!("schema_version={}", bundle.schema_version)),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  • "),
                Span::raw(format!(
                    "ruleset_id={} · ruleset_version={} · profile={}",
                    bundle.ruleset_id, bundle.ruleset_version, bundle.profile
                )),
            ]));

            if let Some(ruleset) = self.app.ruleset_catalog.iter().find(|ruleset| {
                ruleset.id == bundle.ruleset_id || ruleset.canonical_id == bundle.ruleset_id
            }) {
                for note in ruleset.source_notes.iter().take(3) {
                    lines.push(Line::from(vec![
                        Span::raw("  • "),
                        Span::raw(format!(
                            "source_note family={} · source={} · {}",
                            note.family, note.source_id, note.note
                        )),
                    ]));
                }
            }

            for pack in active_packs {
                lines.push(Line::from(vec![
                    Span::raw("  • "),
                    Span::raw(format!(
                        "{} · version={} · family={} · mode={}",
                        pack.pack_id, pack.version, pack.source_family, pack.mode
                    )),
                ]));
            }
        } else {
            lines.push(Line::from(vec![Span::styled(
                "Nhấn e để xem bộ quy tắc, nguồn và metadata chi tiết.",
                Style::default().fg(Color::DarkGray),
            )]));
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::InspectionWidget;
    use crate::layout::LayoutMode;
    use crate::state::{
        AppState, ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, PageSection,
    };
    use amlich_api::v2::DayBundleDto;
    use amlich_api::{
        ActiveRecommendationPackDto, DailyRecommendationsDto, RecommendationPackCatalogEntryDto,
        RecommendationScopeDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };
    use chrono::NaiveDate;
    use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

    fn sample_app() -> AppState {
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
            app_mode: crate::state::AppMode::Normal,
            search_input: String::new(),
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Today,
            view_history: Vec::new(),
        }
    }

    #[test]
    fn bundle_pack_summary_prefers_runtime_active_packs() {
        let mut app = sample_app();
        app.applied_selection.enabled_pack_ids = vec!["staged.pack".to_string()];
        let recommendations = DailyRecommendationsDto {
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "contextual".to_string(),
            scope: RecommendationScopeDto::GeneralDay,
            version: "v1".to_string(),
            summary_vi: String::new(),
            summary_en: String::new(),
            active_packs: vec![ActiveRecommendationPackDto {
                pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
                version: "v1".to_string(),
                source_family: "traditional".to_string(),
                mode: "advisory".to_string(),
            }],
            activities: vec![],
        };
        app.bundle = Some(DayBundleDto {
            schema_version: "amlich.engine/v1".to_string(),
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            generated_at: "2026-03-12T00:00:00Z".to_string(),
            solar: amlich_api::SolarDto {
                day: 12,
                month: 3,
                year: 2026,
                day_of_week: 4,
                day_of_week_name: "Thứ Năm".to_string(),
                date_string: "2026-03-12".to_string(),
            },
            lunar: amlich_api::LunarDto {
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
            daily_recommendations: Some(recommendations),
            contextual_recommendations: None,
            insight: None,
            upcoming_events: vec![],
        });

        assert_eq!(app.active_bundle_packs_summary(), "pack.nhi_thap_bat_tu.v1");
    }

    fn render_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 90, 24);
        let mut buf = Buffer::empty(area);
        InspectionWidget::new(app, LayoutMode::Large).render(area, &mut buf);

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
    fn inspection_render_separates_bundle_and_recommendation_layers() {
        let mut app = sample_app();
        let baseline = DailyRecommendationsDto {
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            scope: RecommendationScopeDto::GeneralDay,
            version: "v1".to_string(),
            summary_vi: "Ngày nền ổn định".to_string(),
            summary_en: String::new(),
            active_packs: vec![],
            activities: vec![],
        };
        let contextual = DailyRecommendationsDto {
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "contract_signing".to_string(),
            scope: RecommendationScopeDto::GeneralDay,
            version: "v1".to_string(),
            summary_vi: "Ưu tiên ký kết theo ngữ cảnh".to_string(),
            summary_en: String::new(),
            active_packs: vec![ActiveRecommendationPackDto {
                pack_id: "pack.contract.v1".to_string(),
                version: "v1".to_string(),
                source_family: "contract".to_string(),
                mode: "advisory".to_string(),
            }],
            activities: vec![],
        };
        app.bundle = Some(DayBundleDto {
            schema_version: "amlich.engine/v1".to_string(),
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            generated_at: "2026-03-12T00:00:00Z".to_string(),
            solar: amlich_api::SolarDto {
                day: 12,
                month: 3,
                year: 2026,
                day_of_week: 4,
                day_of_week_name: "Thứ Năm".to_string(),
                date_string: "2026-03-12".to_string(),
            },
            lunar: amlich_api::LunarDto {
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
            daily_recommendations: Some(baseline),
            contextual_recommendations: Some(contextual),
            insight: None,
            upcoming_events: vec![],
        });

        let text = render_text(&app);

        assert!(text.contains("Bộ quy tắc đang dùng:"));
        assert!(text.contains("Ngữ cảnh xem ngày:"));
        assert!(text.contains("pack.contract.v1"));
        assert!(text.contains("Ngày nền ổn định"));
        assert!(!text.contains("schema_version="));
        assert!(!text.contains("ruleset_id="));
    }

    #[test]
    fn inspection_render_includes_runtime_provenance_fields_for_active_packs() {
        let mut app = sample_app();
        let recommendations = DailyRecommendationsDto {
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "contextual".to_string(),
            scope: RecommendationScopeDto::GeneralDay,
            version: "v1".to_string(),
            summary_vi: String::new(),
            summary_en: String::new(),
            active_packs: vec![ActiveRecommendationPackDto {
                pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
                version: "v1".to_string(),
                source_family: "traditional".to_string(),
                mode: "advisory".to_string(),
            }],
            activities: vec![],
        };
        app.bundle = Some(DayBundleDto {
            schema_version: "amlich.engine/v1".to_string(),
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            generated_at: "2026-03-12T00:00:00Z".to_string(),
            solar: amlich_api::SolarDto {
                day: 12,
                month: 3,
                year: 2026,
                day_of_week: 4,
                day_of_week_name: "Thứ Năm".to_string(),
                date_string: "2026-03-12".to_string(),
            },
            lunar: amlich_api::LunarDto {
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
            daily_recommendations: Some(recommendations),
            contextual_recommendations: None,
            insight: None,
            upcoming_events: vec![],
        });
        app.show_evidence = true;

        let text = render_text(&app);

        assert!(text.contains("Chứng cứ kỹ thuật:"));
        assert!(text.contains("family=traditional"));
        assert!(text.contains("mode=advisory"));
    }
}
