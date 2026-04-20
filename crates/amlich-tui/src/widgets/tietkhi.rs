use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
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

        if let Some(recommendations) = bundle
            .contextual_recommendations
            .as_ref()
            .or(bundle.daily_recommendations.as_ref())
        {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    format!(
                        "Metadata: ruleset_id={} · ruleset_version={} · profile={}",
                        recommendations.ruleset_id,
                        recommendations.ruleset_version,
                        recommendations.profile
                    ),
                    Style::default().fg(Color::DarkGray),
                ),
            ]));
        }

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
        ActivityLabelDto, DailyRecommendationsDto, LunarDto, RecommendationBucketDto,
        RecommendationEvidenceDto, RecommendationEvidenceSourceDto, RecommendationReasonDto,
        RecommendationScopeDto, RecommendationSeverityDto, SolarDto, SynthesizedRecommendationDto,
        TietKhiDto,
    };
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };
    use chrono::NaiveDate;

    fn sample_app_state(expanded: bool) -> AppState {
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
        let mut expanded_sections = std::collections::BTreeSet::new();
        if expanded {
            expanded_sections.insert(PageSection::TraditionalEvidence);
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
                tiet_khi: Some(TietKhiDto {
                    index: 3,
                    name: "Kinh Trập".to_string(),
                    description: "Tóm tắt\n- Giai đoạn chuyển mùa\n- Nên giữ nhịp sinh hoạt đều"
                        .to_string(),
                    longitude: 345,
                    current_longitude: 345.0,
                    season: "Xuân".to_string(),
                }),
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
            show_tietkhi_details: expanded,
            show_evidence: false,
            show_week_strip: true,
            show_graph_recommendations: false,
            verbosity: crate::state::ui_prefs::VerbosityMode::Compact,
            focused_section: PageSection::TraditionalEvidence,
            zoomed_section: None,
            expanded_sections,
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

    #[test]
    fn tietkhi_widget_collapses_to_summary_and_expands_details() {
        let collapsed = render_tiet_khi(&sample_app_state(false));
        let expanded = render_tiet_khi(&sample_app_state(true));

        assert!(collapsed.contains("Tiết Khí Tham Chiếu"));
        assert!(collapsed.contains("Kinh Trập"));
        assert!(
            collapsed.contains("Metadata: ruleset_id=test · ruleset_version=v1 · profile=baseline")
        );
        assert!(!collapsed.contains("Giai đoạn chuyển mùa"));
        assert!(expanded.contains("Giai đoạn chuyển mùa"));
    }
}
