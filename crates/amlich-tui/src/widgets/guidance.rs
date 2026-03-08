use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use amlich_api::{
    v2::DayBundleDto, RecommendationBucketDto, RecommendationEvidenceSourceDto,
    RecommendationSeverityDto,
};

use crate::layout::LayoutMode;
use crate::state::AppState;

const SMALL_LIMIT: usize = 2;
const MEDIUM_LIMIT: usize = 4;
const LARGE_LIMIT: usize = 6;

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
        let Some(recommendations) = &bundle.daily_recommendations else {
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
        let header_style = Style::default().fg(Color::DarkGray);
        let summary_style = Style::default().fg(Color::Yellow);
        let hint_style = Style::default().fg(Color::DarkGray);

        let expand_hint = if expanded {
            "▼ Thu gọn (a)"
        } else {
            "▶ Mở rộng (a)"
        };

        lines.push(Line::from(vec![
            Span::styled("── Khuyến Nghị ", header_style),
            Span::styled(format!("{:─<20}", ""), header_style),
            Span::styled(expand_hint, hint_style),
        ]));

        lines.push(Line::from(vec![
            Span::styled("   ", summary_style),
            Span::styled(recommendations.summary_vi.clone(), summary_style),
        ]));
        lines.push(Line::from(""));

        render_bucket_section(
            &mut lines,
            "Nên",
            &nen_rows,
            limit,
            Style::default().fg(Color::Green),
        );
        render_bucket_section(
            &mut lines,
            "Có thể",
            &co_the_rows,
            limit,
            Style::default().fg(Color::Cyan),
        );
        render_bucket_section(
            &mut lines,
            "Tránh",
            &tranh_rows,
            limit,
            Style::default().fg(Color::Red),
        );
        render_bucket_section(
            &mut lines,
            "Kỵ mạnh",
            &ky_manh_rows,
            limit,
            Style::default().fg(Color::Magenta),
        );

        if let Some(footer) = build_footer_hint(bundle) {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("   ", hint_style),
                Span::styled(footer, hint_style.add_modifier(Modifier::ITALIC)),
            ]));
        }

        Paragraph::new(lines).render(area, buf);
    }
}

fn render_bucket_section(
    lines: &mut Vec<Line<'static>>,
    title: &str,
    rows: &[DecisionRow],
    limit: usize,
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

    lines.push(Line::from(vec![
        Span::styled(format!("── {title} ({}) ", rows.len()), header_style),
        Span::styled(format!("{:─<22}", ""), header_style),
    ]));

    let take = rows.len().min(limit);
    for row in rows.iter().take(take) {
        for line in render_row_lines(row, marker_style, text_style, chip_style, 56) {
            lines.push(line);
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
) -> Vec<Line<'static>> {
    let marker = match row.emphasis {
        DecisionEmphasis::Primary => "★ ",
        DecisionEmphasis::Normal => "• ",
    };

    let base_indent = 3;
    let content_width = width.saturating_sub(base_indent + marker.len());
    let chip = row.reason_chip.as_ref().map(|c| format!(" [{c}]"));

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
    let Some(hours) = bundle.gio_hoang_dao.as_ref() else {
        return None;
    };

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
    use amlich_api::{
        ActivityLabelDto, DailyRecommendationsDto, RecommendationBucketDto,
        RecommendationEvidenceDto, RecommendationEvidenceSourceDto, RecommendationReasonDto,
        RecommendationScopeDto, RecommendationSeverityDto, SynthesizedRecommendationDto,
    };

    fn sample_recommendations() -> DailyRecommendationsDto {
        DailyRecommendationsDto {
            scope: RecommendationScopeDto::GeneralDay,
            version: "v1-layered".to_string(),
            summary_vi: "Ngày thuận".to_string(),
            summary_en: "Supportive day".to_string(),
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
            ],
        }
    }

    #[test]
    fn build_rows_filters_by_bucket() {
        let recommendations = sample_recommendations();
        let nen_rows = build_rows(&recommendations, RecommendationBucketDto::Nen);
        let tranh_rows = build_rows(&recommendations, RecommendationBucketDto::Tranh);
        let co_the_rows = build_rows(&recommendations, RecommendationBucketDto::CoThe);

        assert_eq!(nen_rows.len(), 1);
        assert_eq!(nen_rows[0].text, "Khai mở");
        assert_eq!(tranh_rows.len(), 1);
        assert_eq!(tranh_rows[0].text, "Ký kết");
        assert!(co_the_rows.is_empty());
    }

    #[test]
    fn display_limit_changes_with_mode_and_expand() {
        assert_eq!(display_limit(LayoutMode::Small, false), SMALL_LIMIT);
        assert_eq!(display_limit(LayoutMode::Medium, false), MEDIUM_LIMIT);
        assert_eq!(display_limit(LayoutMode::Large, false), LARGE_LIMIT);
        assert_eq!(display_limit(LayoutMode::Small, true), usize::MAX);
    }
}
