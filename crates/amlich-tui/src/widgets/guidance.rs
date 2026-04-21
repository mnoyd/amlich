use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use amlich_api::{
    v2::DayBundleDto, RecommendationBucketDto, RecommendationEvidenceSourceDto,
    RecommendationSeverityDto,
};

use crate::layout::LayoutMode;
use crate::state::AppState;

const SMALL_LIMIT: usize = 2;
const MEDIUM_LIMIT: usize = 3;
const LARGE_LIMIT: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DecisionEmphasis {
    Primary,
    Normal,
}

#[derive(Clone)]
struct DecisionRow {
    text: String,
    emphasis: DecisionEmphasis,
    reason_chip: Option<String>,
    reason_details: Vec<String>,
}

pub struct GuidanceWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> GuidanceWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for GuidanceWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };
        let layers = self.app.recommendation_layers();
        let Some(active_layer) = layers.first() else {
            return;
        };
        let Some(recommendations) = bundle
            .contextual_recommendations
            .as_ref()
            .or(bundle.daily_recommendations.as_ref())
        else {
            return;
        };

        let expanded = self.app.show_guidance_details;
        let limit = display_limit(self.mode, expanded);

        let mut nen_rows = build_rows(recommendations, RecommendationBucketDto::Nen);
        let mut co_the_rows = build_rows(recommendations, RecommendationBucketDto::CoThe);
        let mut tranh_rows = build_rows(recommendations, RecommendationBucketDto::Tranh);
        let mut ky_manh_rows = build_rows(recommendations, RecommendationBucketDto::KyManh);

        if nen_rows.is_empty()
            && co_the_rows.is_empty()
            && tranh_rows.is_empty()
            && ky_manh_rows.is_empty()
        {
            return;
        }

        mark_primary(&mut nen_rows);
        mark_primary(&mut co_the_rows);
        mark_primary(&mut tranh_rows);
        mark_primary(&mut ky_manh_rows);

        let mut lines = vec![];

        let summary_style = Style::default().fg(Color::Yellow);
        let hint_style = Style::default().fg(Color::DarkGray);

        let expand_hint = if expanded {
            "Thu gọn (a) ▼"
        } else {
            "Mở rộng (a) ▶"
        };

        let title = format!(" Khuyến Nghị [{}] ", expand_hint);
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);

        lines.push(Line::from(vec![
            Span::styled("   ", summary_style),
            Span::styled(recommendations.summary_vi.clone(), summary_style),
        ]));
        if layers.len() > 1 {
            lines.push(Line::from(vec![
                Span::styled("   ", hint_style),
                Span::styled(
                    format!(
                        "Ưu tiên lớp {} · nền vẫn xem riêng: {}",
                        active_layer.profile, layers[1].summary
                    ),
                    hint_style,
                ),
            ]));
        } else if !recommendations.active_packs.is_empty() || recommendations.profile != "baseline"
        {
            let pack_labels = recommendations
                .active_packs
                .iter()
                .map(|pack| pack.pack_id.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            let context_note = if pack_labels.is_empty() {
                format!("Ngữ cảnh: {}", recommendations.profile)
            } else {
                format!(
                    "Ngữ cảnh: {} · pack: {}",
                    recommendations.profile, pack_labels
                )
            };
            lines.push(Line::from(vec![
                Span::styled("   ", hint_style),
                Span::styled(context_note, hint_style),
            ]));
        }
        lines.push(Line::from(""));

        render_bucket_section(
            &mut lines,
            "Nên",
            &nen_rows,
            limit,
            self.app.show_evidence,
            Style::default().fg(Color::Green),
        );
        render_bucket_section(
            &mut lines,
            "Có thể",
            &co_the_rows,
            limit,
            self.app.show_evidence,
            Style::default().fg(Color::Cyan),
        );
        render_bucket_section(
            &mut lines,
            "Tránh",
            &tranh_rows,
            limit,
            self.app.show_evidence,
            Style::default().fg(Color::Red),
        );
        render_bucket_section(
            &mut lines,
            "Kỵ mạnh",
            &ky_manh_rows,
            limit,
            self.app.show_evidence,
            Style::default().fg(Color::Magenta),
        );

        if let Some(note) = build_sensitive_domain_note(recommendations) {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("   ", hint_style),
                Span::styled(note, hint_style.add_modifier(Modifier::ITALIC)),
            ]));
        }

        if let Some(footer) = build_footer_hint(bundle) {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("   ", hint_style),
                Span::styled(footer, hint_style.add_modifier(Modifier::ITALIC)),
            ]));
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}

fn render_bucket_section(
    lines: &mut Vec<Line<'static>>,
    title: &str,
    rows: &[DecisionRow],
    limit: usize,
    show_evidence: bool,
    marker_style: Style,
) {
    if rows.is_empty() {
        return;
    }

    let header_style = Style::default().fg(Color::DarkGray);
    let text_style = Style::default().fg(Color::White);
    let chip_style = Style::default().fg(Color::Yellow);

    if !lines.is_empty() {
        lines.push(Line::from(""));
    }

    lines.push(Line::from(vec![Span::styled(
        format!("── {title} ({}) ", rows.len()),
        header_style,
    )]));

    let take = rows.len().min(limit);
    for row in rows.iter().take(take) {
        for line in render_row_lines(row, marker_style, text_style, chip_style, 56, show_evidence) {
            lines.push(line);
        }
        if show_evidence {
            for detail in &row.reason_details {
                lines.push(Line::from(vec![
                    Span::raw("      ↳ "),
                    Span::styled(detail.clone(), Style::default().fg(Color::DarkGray)),
                ]));
            }
        }
    }

    if rows.len() > take {
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(
                format!("+{} mục ẩn", rows.len() - take),
                Style::default().fg(Color::DarkGray),
            ),
        ]));
    }
}

fn render_row_lines(
    row: &DecisionRow,
    marker_style: Style,
    text_style: Style,
    chip_style: Style,
    width: usize,
    show_evidence: bool,
) -> Vec<Line<'static>> {
    let marker = match row.emphasis {
        DecisionEmphasis::Primary => "★ ",
        DecisionEmphasis::Normal => "• ",
    };

    let base_indent = 3;
    let content_width = width.saturating_sub(base_indent + marker.len());
    let chip = show_evidence
        .then_some(row.reason_chip.as_ref())
        .flatten()
        .map(|c| format!(" [{c}]"));

    let mut first = row.text.clone();
    if let Some(chip) = chip.as_ref() {
        if first.chars().count() + chip.chars().count() <= content_width {
            first.push_str(chip);
        }
    }

    let wrapped = wrap_text(&first, content_width.max(8));
    let mut lines = Vec::new();

    for (i, segment) in wrapped.into_iter().enumerate() {
        let mut spans = vec![Span::raw("   ")];
        if i == 0 {
            spans.push(Span::styled(marker.to_string(), marker_style));
        } else {
            spans.push(Span::raw("  "));
        }

        if let Some(chip_text) = chip.as_ref() {
            if i == 0 && segment.ends_with(chip_text) {
                let item_len = segment.len() - chip_text.len();
                spans.push(Span::styled(segment[..item_len].to_string(), text_style));
                spans.push(Span::styled(chip_text.to_string(), chip_style));
            } else {
                spans.push(Span::styled(segment, text_style));
            }
        } else {
            spans.push(Span::styled(segment, text_style));
        }

        lines.push(Line::from(spans));
    }

    lines
}

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if text.chars().count() <= width {
        return vec![text.to_string()];
    }

    let mut lines = Vec::new();
    let mut current = String::new();

    for word in text.split_whitespace() {
        let candidate_len = if current.is_empty() {
            word.chars().count()
        } else {
            current.chars().count() + 1 + word.chars().count()
        };

        if candidate_len > width && !current.is_empty() {
            lines.push(current);
            current = word.to_string();
        } else {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(word);
        }
    }

    if !current.is_empty() {
        lines.push(current);
    }

    lines
}

fn build_rows(
    bundle: &amlich_api::DailyRecommendationsDto,
    bucket: RecommendationBucketDto,
) -> Vec<DecisionRow> {
    bundle
        .activities
        .iter()
        .filter(|activity| activity.bucket == bucket)
        .map(|activity| {
            let strongest = activity
                .reasons
                .iter()
                .min_by_key(|reason| severity_rank(reason.severity));
            let reason_chip = strongest.map(|reason| {
                format!(
                    "{} • {}",
                    severity_label(reason.severity),
                    source_label(reason.evidence.source)
                )
            });

            DecisionRow {
                text: activity.label.vi.clone(),
                emphasis: DecisionEmphasis::Normal,
                reason_chip,
                reason_details: activity
                    .reasons
                    .iter()
                    .map(|reason| {
                        format!(
                            "{} · {} · {} · rule={} · code={}",
                            severity_label(reason.severity),
                            source_label(reason.evidence.source),
                            reason.summary_vi,
                            reason.rule_id,
                            reason.evidence.code
                        )
                    })
                    .collect(),
            }
        })
        .collect()
}

fn mark_primary(rows: &mut [DecisionRow]) {
    if let Some(first) = rows.first_mut() {
        first.emphasis = DecisionEmphasis::Primary;
    }
}

fn severity_rank(severity: RecommendationSeverityDto) -> u8 {
    match severity {
        RecommendationSeverityDto::Override => 0,
        RecommendationSeverityDto::Primary => 1,
        RecommendationSeverityDto::Supporting => 2,
    }
}

fn severity_label(severity: RecommendationSeverityDto) -> &'static str {
    match severity {
        RecommendationSeverityDto::Override => "override",
        RecommendationSeverityDto::Primary => "primary",
        RecommendationSeverityDto::Supporting => "support",
    }
}

fn source_label(source: RecommendationEvidenceSourceDto) -> &'static str {
    match source {
        RecommendationEvidenceSourceDto::DayGuidance => "guidance",
        RecommendationEvidenceSourceDto::Truc => "trực",
        RecommendationEvidenceSourceDto::Stars => "sao",
        RecommendationEvidenceSourceDto::DayDeity => "thần sát",
        RecommendationEvidenceSourceDto::Taboo => "kiêng kỵ",
        RecommendationEvidenceSourceDto::XungHop => "xung-hợp",
        RecommendationEvidenceSourceDto::TietKhi => "tiết khí",
        RecommendationEvidenceSourceDto::GioHoangDao => "giờ tốt",
        RecommendationEvidenceSourceDto::Travel => "xuất hành",
        RecommendationEvidenceSourceDto::ProductRule => "mở rộng",
    }
}

fn build_footer_hint(bundle: &DayBundleDto) -> Option<String> {
    let hours = bundle.gio_hoang_dao.as_ref()?;

    let top_hours: Vec<String> = hours
        .good_hours
        .iter()
        .take(3)
        .map(|hour| hour.time_range.clone())
        .collect();

    if top_hours.is_empty() {
        None
    } else {
        Some(format!("Giờ đẹp tham chiếu: {}", top_hours.join(", ")))
    }
}

fn build_sensitive_domain_note(bundle: &amlich_api::DailyRecommendationsDto) -> Option<String> {
    let has_medical = bundle
        .activities
        .iter()
        .any(|activity| activity.activity_id == "medical_treatment");
    let has_burial = bundle
        .activities
        .iter()
        .any(|activity| activity.activity_id == "burial_memorial");

    let mut notes = Vec::new();
    if has_medical {
        notes.push(
            "Lưu ý: điều trị thực tế luôn ưu tiên đánh giá chuyên môn; lịch chỉ mang tính tham khảo."
                .to_string(),
        );
    }
    if has_burial {
        notes.push(
            "Lưu ý: an táng hoặc tưởng niệm cần thẩm định thêm theo tập tục và chuyên gia địa phương."
                .to_string(),
        );
    }

    if notes.is_empty() {
        None
    } else {
        Some(notes.join(" "))
    }
}

fn display_limit(mode: LayoutMode, expanded: bool) -> usize {
    if expanded {
        return usize::MAX;
    }

    match mode {
        LayoutMode::Small => SMALL_LIMIT,
        LayoutMode::Medium => MEDIUM_LIMIT,
        LayoutMode::Large => LARGE_LIMIT,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ExplorerAction, ExplorerField, ExplorerSelection, PageSection};
    use amlich_api::v2::DayBundleDto;
    use amlich_api::{
        ActivityLabelDto, DailyRecommendationsDto, RecommendationBucketDto,
        RecommendationEvidenceDto, RecommendationEvidenceSourceDto, RecommendationReasonDto,
        RecommendationScopeDto, RecommendationSeverityDto, SynthesizedRecommendationDto,
    };
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };
    use chrono::NaiveDate;
    use ratatui::layout::Rect;

    fn sample_recommendations() -> DailyRecommendationsDto {
        DailyRecommendationsDto {
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            scope: RecommendationScopeDto::GeneralDay,
            version: "v1-layered".to_string(),
            summary_vi: "Ngày thuận".to_string(),
            summary_en: "Supportive day".to_string(),
            active_packs: vec![],
            activities: vec![
                SynthesizedRecommendationDto {
                    activity_id: "opening_start".to_string(),
                    label: ActivityLabelDto {
                        vi: "Khai mở".to_string(),
                        en: "Opening and launching".to_string(),
                    },
                    bucket: RecommendationBucketDto::Nen,
                    reasons: vec![RecommendationReasonDto {
                        rule_id: "base.truc.truc.Khai.good_for".to_string(),
                        severity: RecommendationSeverityDto::Primary,
                        summary_vi: "Hợp cho khai trương".to_string(),
                        summary_en: "Suitable for opening".to_string(),
                        evidence: RecommendationEvidenceDto {
                            source: RecommendationEvidenceSourceDto::Truc,
                            code: "truc.Khai.good_for".to_string(),
                            note: "Derived from truc".to_string(),
                        },
                    }],
                },
                SynthesizedRecommendationDto {
                    activity_id: "visit_network".to_string(),
                    label: ActivityLabelDto {
                        vi: "Gặp gỡ".to_string(),
                        en: "Visits and networking".to_string(),
                    },
                    bucket: RecommendationBucketDto::Nen,
                    reasons: vec![RecommendationReasonDto {
                        rule_id: "base.travel.good_for".to_string(),
                        severity: RecommendationSeverityDto::Supporting,
                        summary_vi: "Có thể đi gặp mặt".to_string(),
                        summary_en: "Suitable for visits".to_string(),
                        evidence: RecommendationEvidenceDto {
                            source: RecommendationEvidenceSourceDto::Travel,
                            code: "travel.good_for".to_string(),
                            note: "Derived from travel".to_string(),
                        },
                    }],
                },
                SynthesizedRecommendationDto {
                    activity_id: "pray_offering".to_string(),
                    label: ActivityLabelDto {
                        vi: "Cúng lễ".to_string(),
                        en: "Offering".to_string(),
                    },
                    bucket: RecommendationBucketDto::Nen,
                    reasons: vec![RecommendationReasonDto {
                        rule_id: "base.stars.good_for".to_string(),
                        severity: RecommendationSeverityDto::Supporting,
                        summary_vi: "Có thể cúng lễ".to_string(),
                        summary_en: "Suitable for ritual".to_string(),
                        evidence: RecommendationEvidenceDto {
                            source: RecommendationEvidenceSourceDto::Stars,
                            code: "stars.good_for".to_string(),
                            note: "Derived from stars".to_string(),
                        },
                    }],
                },
                SynthesizedRecommendationDto {
                    activity_id: "meet_people".to_string(),
                    label: ActivityLabelDto {
                        vi: "Họp mặt".to_string(),
                        en: "Meeting".to_string(),
                    },
                    bucket: RecommendationBucketDto::CoThe,
                    reasons: vec![RecommendationReasonDto {
                        rule_id: "base.travel.soft".to_string(),
                        severity: RecommendationSeverityDto::Supporting,
                        summary_vi: "Khá ổn cho gặp gỡ".to_string(),
                        summary_en: "Reasonable for meetings".to_string(),
                        evidence: RecommendationEvidenceDto {
                            source: RecommendationEvidenceSourceDto::Travel,
                            code: "travel.soft".to_string(),
                            note: "Derived from travel".to_string(),
                        },
                    }],
                },
                SynthesizedRecommendationDto {
                    activity_id: "contract_agreement".to_string(),
                    label: ActivityLabelDto {
                        vi: "Ký kết".to_string(),
                        en: "Contracts and agreements".to_string(),
                    },
                    bucket: RecommendationBucketDto::Tranh,
                    reasons: vec![RecommendationReasonDto {
                        rule_id: "layer.taboo.taboo.tam_nuong.hard".to_string(),
                        severity: RecommendationSeverityDto::Override,
                        summary_vi: "Nên tránh ký kết lớn".to_string(),
                        summary_en: "Avoid major signing".to_string(),
                        evidence: RecommendationEvidenceDto {
                            source: RecommendationEvidenceSourceDto::Taboo,
                            code: "taboo.tam_nuong.hard".to_string(),
                            note: "Derived from taboo".to_string(),
                        },
                    }],
                },
                SynthesizedRecommendationDto {
                    activity_id: "groundbreaking".to_string(),
                    label: ActivityLabelDto {
                        vi: "Động thổ".to_string(),
                        en: "Groundbreaking".to_string(),
                    },
                    bucket: RecommendationBucketDto::KyManh,
                    reasons: vec![RecommendationReasonDto {
                        rule_id: "layer.taboo.taboo.tam_nuong.hard".to_string(),
                        severity: RecommendationSeverityDto::Override,
                        summary_vi: "Kỵ động thổ".to_string(),
                        summary_en: "Strongly avoid groundbreaking".to_string(),
                        evidence: RecommendationEvidenceDto {
                            source: RecommendationEvidenceSourceDto::Taboo,
                            code: "taboo.tam_nuong.hard".to_string(),
                            note: "Derived from taboo".to_string(),
                        },
                    }],
                },
            ],
        }
    }

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
                gio_hoang_dao: Some(amlich_api::GioHoangDaoDto {
                    day_chi: "Ngọ".to_string(),
                    good_hour_count: 3,
                    good_hours: vec![
                        amlich_api::HourInfoDto {
                            hour_index: 0,
                            hour_chi: "Tý".to_string(),
                            time_range: "23:00 - 01:00".to_string(),
                            star: "Thanh Long".to_string(),
                            is_good: true,
                        },
                        amlich_api::HourInfoDto {
                            hour_index: 1,
                            hour_chi: "Sửu".to_string(),
                            time_range: "01:00 - 03:00".to_string(),
                            star: "Minh Đường".to_string(),
                            is_good: true,
                        },
                        amlich_api::HourInfoDto {
                            hour_index: 2,
                            hour_chi: "Dần".to_string(),
                            time_range: "03:00 - 05:00".to_string(),
                            star: "Kim Quỹ".to_string(),
                            is_good: true,
                        },
                    ],
                    all_hours: vec![],
                    summary: "Giờ đẹp".to_string(),
                }),
                day_fortune: None,
                daily_recommendations: Some(sample_recommendations()),
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
            show_graph_recommendations: false,
            verbosity: crate::state::ui_prefs::VerbosityMode::Compact,
            focused_section: PageSection::Recommendations,
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
            graph_inspector_focus: crate::state::GraphInspectorFocus::Summary,
            graph_inspector_cursor: 0,
            graph_inspector_search_query: String::new(),
            graph_inspector_search_cursor: 0,
            graph_inspector_focus_before_search: None,
            graph_inspector_lens: crate::state::GraphInspectorLens::General,
        }
    }

    fn render_text(app: &AppState, mode: LayoutMode) -> String {
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        GuidanceWidget::new(app, mode).render(area, &mut buf);

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
    fn build_rows_filters_by_bucket() {
        let recommendations = sample_recommendations();
        let nen_rows = build_rows(&recommendations, RecommendationBucketDto::Nen);
        let tranh_rows = build_rows(&recommendations, RecommendationBucketDto::Tranh);
        let co_the_rows = build_rows(&recommendations, RecommendationBucketDto::CoThe);
        let ky_manh_rows = build_rows(&recommendations, RecommendationBucketDto::KyManh);

        assert_eq!(nen_rows.len(), 3);
        assert_eq!(nen_rows[0].text, "Khai mở");
        assert_eq!(tranh_rows.len(), 1);
        assert_eq!(tranh_rows[0].text, "Ký kết");
        assert_eq!(co_the_rows.len(), 1);
        assert_eq!(ky_manh_rows.len(), 1);
        assert!(nen_rows[0]
            .reason_details
            .iter()
            .any(|detail| detail.contains("Hợp cho khai trương")));
    }

    #[test]
    fn display_limit_changes_with_mode_and_expand() {
        assert_eq!(display_limit(LayoutMode::Small, false), SMALL_LIMIT);
        assert_eq!(display_limit(LayoutMode::Medium, false), MEDIUM_LIMIT);
        assert_eq!(display_limit(LayoutMode::Large, false), LARGE_LIMIT);
        assert_eq!(display_limit(LayoutMode::Small, true), usize::MAX);
    }

    #[test]
    fn collapsed_render_keeps_bucket_order_and_counts() {
        let app = sample_app_state();
        let text = render_text(&app, LayoutMode::Small);

        let nen_idx = text.find("Nên (3)").expect("nen header");
        let co_the_idx = text.find("Có thể (1)").expect("co_the header");
        let tranh_idx = text.find("Tránh (1)").expect("tranh header");
        let ky_manh_idx = text.find("Kỵ mạnh (1)").expect("ky_manh header");

        assert!(nen_idx < co_the_idx && co_the_idx < tranh_idx && tranh_idx < ky_manh_idx);
        assert!(text.contains("+1 mục ẩn"));
    }

    #[test]
    fn contextual_recommendations_take_precedence_in_render() {
        let mut app = sample_app_state();
        if let Some(bundle) = app.bundle.as_mut() {
            let mut contextual = sample_recommendations();
            contextual.profile = "contextual".to_string();
            contextual.summary_vi = "Ưu tiên ký kết theo ngữ cảnh".to_string();
            contextual.active_packs = vec![amlich_api::ActiveRecommendationPackDto {
                pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
                version: "v1".to_string(),
                source_family: "nhi_thap_bat_tu".to_string(),
                mode: "advisory".to_string(),
            }];
            bundle.contextual_recommendations = Some(contextual);
        }

        let text = render_text(&app, LayoutMode::Small);
        assert!(text.contains("Ưu tiên ký kết theo ngữ cảnh"));
        assert!(text.contains("nền vẫn xem riêng: Ngày thuận"));
    }

    #[test]
    fn expanded_render_shows_all_rows_for_focused_section() {
        let mut app = sample_app_state();
        app.expand_section(PageSection::Recommendations);

        let text = render_text(&app, LayoutMode::Small);

        assert!(text.contains("Cúng lễ"));
        assert!(!text.contains("+1 mục ẩn"));
    }

    #[test]
    fn evidence_toggle_hides_and_shows_reason_chips() {
        let mut app = sample_app_state();

        let without_evidence = render_text(&app, LayoutMode::Large);
        assert!(!without_evidence.contains("primary • trực"));

        app.show_evidence = true;
        let with_evidence = render_text(&app, LayoutMode::Large);
        assert!(with_evidence.contains("primary • trực"));
        assert!(with_evidence.contains("Hợp cho khai trương"));
        assert!(with_evidence.contains("↳"));
    }

    #[test]
    fn primary_rows_are_visually_marked_first_per_bucket() {
        let app = sample_app_state();
        let text = render_text(&app, LayoutMode::Large);

        assert!(text.contains("★ Khai mở"));
        assert!(text.contains("• Gặp gỡ"));
        assert!(text.contains("★ Họp mặt"));
        assert!(text.contains("★ Ký kết"));
        assert!(text.contains("★ Động thổ"));
    }
}
