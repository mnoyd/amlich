use crate::state::{
    AppState, RecommendationLayerKind, RecommendationLayerVm, RecommendationRowVm,
};
use amlich_api::{
    DailyRecommendationsDto, RecommendationBucketDto, RecommendationEvidenceSourceDto,
    RecommendationSeverityDto,
};

pub fn selected_recommendations(app: &AppState) -> Option<&DailyRecommendationsDto> {
    let bundle = app.bundle.as_ref()?;
    bundle
        .contextual_recommendations
        .as_ref()
        .or(bundle.daily_recommendations.as_ref())
}

pub fn recommendation_layers(app: &AppState) -> Vec<RecommendationLayerVm> {
    let Some(bundle) = app.bundle.as_ref() else {
        return Vec::new();
    };

    let mut layers = Vec::new();

    if let Some(contextual) = bundle.contextual_recommendations.as_ref() {
        layers.push(RecommendationLayerVm {
            kind: RecommendationLayerKind::Contextual,
            label: "Đang áp dụng".to_string(),
            summary: contextual.summary_vi.clone(),
            scope_label: recommendation_scope_label(contextual.scope).to_string(),
            ruleset_id: contextual.ruleset_id.clone(),
            ruleset_version: contextual.ruleset_version.clone(),
            profile: contextual.profile.clone(),
            active_pack_ids: contextual
                .active_packs
                .iter()
                .map(|pack| pack.pack_id.clone())
                .collect(),
        });
    }

    if let Some(baseline) = bundle.daily_recommendations.as_ref() {
        layers.push(RecommendationLayerVm {
            kind: RecommendationLayerKind::Baseline,
            label: if bundle.contextual_recommendations.is_some() {
                "Nền tham chiếu".to_string()
            } else {
                "Đang áp dụng".to_string()
            },
            summary: baseline.summary_vi.clone(),
            scope_label: recommendation_scope_label(baseline.scope).to_string(),
            ruleset_id: baseline.ruleset_id.clone(),
            ruleset_version: baseline.ruleset_version.clone(),
            profile: baseline.profile.clone(),
            active_pack_ids: baseline
                .active_packs
                .iter()
                .map(|pack| pack.pack_id.clone())
                .collect(),
        });
    }

    layers
}

pub fn top_recommendation_rows(app: &AppState) -> Vec<RecommendationRowVm> {
    let Some(recommendations) = selected_recommendations(app) else {
        return Vec::new();
    };

    recommendation_bucket_order()
        .into_iter()
        .filter_map(|bucket| top_row_for_bucket(recommendations, bucket))
        .collect()
}

pub fn recommendation_bucket_order() -> [RecommendationBucketDto; 4] {
    [
        RecommendationBucketDto::Nen,
        RecommendationBucketDto::CoThe,
        RecommendationBucketDto::Tranh,
        RecommendationBucketDto::KyManh,
    ]
}

pub fn top_row_for_bucket(
    recommendations: &DailyRecommendationsDto,
    bucket: RecommendationBucketDto,
) -> Option<RecommendationRowVm> {
    let activity = recommendations
        .activities
        .iter()
        .find(|activity| activity.bucket == bucket)?;
    let reason_chip = activity
        .reasons
        .iter()
        .min_by_key(|reason| severity_rank(reason.severity))
        .map(|reason| {
            format!(
                "{} • {}",
                severity_label(reason.severity),
                source_label(reason.evidence.source)
            )
        });

    Some(RecommendationRowVm {
        bucket,
        label: activity.label.vi.clone(),
        reason_chip,
        reason_details: activity
            .reasons
            .iter()
            .map(|reason| {
                format!(
                    "{} · {} · {} · {} · {}",
                    severity_label(reason.severity),
                    source_label(reason.evidence.source),
                    reason.summary_vi,
                    reason.evidence.code,
                    reason.evidence.note
                )
            })
            .collect(),
    })
}

pub fn severity_rank(severity: RecommendationSeverityDto) -> u8 {
    match severity {
        RecommendationSeverityDto::Override => 0,
        RecommendationSeverityDto::Primary => 1,
        RecommendationSeverityDto::Supporting => 2,
    }
}

pub fn severity_label(severity: RecommendationSeverityDto) -> &'static str {
    match severity {
        RecommendationSeverityDto::Override => "override",
        RecommendationSeverityDto::Primary => "primary",
        RecommendationSeverityDto::Supporting => "support",
    }
}

pub fn source_label(source: RecommendationEvidenceSourceDto) -> &'static str {
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

pub fn recommendation_scope_label(scope: amlich_api::RecommendationScopeDto) -> &'static str {
    match scope {
        amlich_api::RecommendationScopeDto::GeneralDay => "general_day",
    }
}

pub fn format_good_hour_count_summary(good_hour_count: usize) -> String {
    match good_hour_count {
        0 => "Không thấy khung giờ đẹp nổi bật.".to_string(),
        1 => "Có 1 khung giờ thuận để hành sự.".to_string(),
        count => format!("Có {count} khung giờ thuận để hành sự."),
    }
}

pub fn format_hour_window(chi: &str, time_range: &str, star: Option<&str>) -> String {
    match star.map(str::trim).filter(|star| !star.is_empty()) {
        Some(star) => format!("{chi} {time_range} · {star}"),
        None => format!("{chi} {time_range}"),
    }
}
