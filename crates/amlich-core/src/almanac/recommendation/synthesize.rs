use std::collections::BTreeMap;

use crate::{
    almanac::data::default_ruleset,
    almanac::types::{DayDeityClassification, DayFortune},
    gio_hoang_dao::GioHoangDao,
};

use super::{
    activity::ActivityId,
    evidence::collect_truc_hits,
    event_kind::EventKindLayer,
    matrix::taboo_entry,
    pack::{RecommendationPackDescriptor, RecommendationPackLookupError},
    packs::nhi_thap_bat_tu::{NhiThapBatTuPack, NHI_THAP_BAT_TU_PACK},
    policy::{resolve_bucket, RecommendationPolicyInput},
    rules::{avoided_bucket, favored_bucket, truc_insight},
    BaseDirection, BaseEvidenceHit, DailyRecommendations, RecommendationBucket,
    RecommendationEvidence, RecommendationEvidenceSource, RecommendationReason,
    RecommendationScope, RecommendationSeverity, SynthesizedRecommendation,
};

#[derive(Debug, Clone)]
pub struct RecommendationSynthesisContext<'a> {
    pub day_chi: &'a str,
    pub day_fortune: &'a DayFortune,
    pub gio_hoang_dao: Option<&'a GioHoangDao>,
    pub tiet_khi_name: Option<&'a str>,
    pub profile_id: Option<&'a str>,
    pub event_kind: Option<&'a str>,
    pub enabled_pack_ids: &'a [&'a str],
}

#[derive(Debug, Clone)]
pub struct RecommendationLayerHit {
    pub activity_id: ActivityId,
    pub source: RecommendationEvidenceSource,
    pub source_code: String,
    pub direction: BaseDirection,
    pub summary_vi: String,
    pub summary_en: String,
    pub severity: RecommendationSeverity,
    pub hard_stop: bool,
}

pub trait RecommendationLayer {
    fn layer_id(&self) -> &'static str;
    fn collect_hits(
        &self,
        context: &RecommendationSynthesisContext<'_>,
    ) -> Vec<RecommendationLayerHit>;
}

#[derive(Debug, Clone)]
struct CollectedHit {
    activity_id: ActivityId,
    source: RecommendationEvidenceSource,
    source_code: String,
    direction: BaseDirection,
    summary_vi: String,
    summary_en: String,
    severity: RecommendationSeverity,
    hard_stop: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HitOrigin {
    CorePolicy,
    ExtensionLayer,
}

#[derive(Debug, Clone)]
struct AggregateRecommendation {
    activity: SynthesizedRecommendation,
    saw_favor: bool,
    saw_hard_stop: bool,
    favor_sources: usize,
    strong_avoid_sources: usize,
    supporting_avoid_sources: usize,
}

pub fn synthesize_base_daily_recommendations(
    day_chi: &str,
    truc_name: &str,
) -> DailyRecommendations {
    synthesize_internal(day_chi, truc_name, None, None, None, &[])
        .expect("base recommendations should always synthesize")
}

pub fn synthesize_daily_recommendations(
    context: &RecommendationSynthesisContext<'_>,
) -> DailyRecommendations {
    synthesize_daily_recommendations_with_layers(context, &[])
        .expect("validated recommendation context should synthesize")
}

pub fn synthesize_daily_recommendations_with_layers(
    context: &RecommendationSynthesisContext<'_>,
    layers: &[&dyn RecommendationLayer],
) -> Result<DailyRecommendations, RecommendationPackLookupError> {
    synthesize_internal(
        context.day_chi,
        &context.day_fortune.truc.name,
        Some(context),
        context.gio_hoang_dao,
        context.tiet_khi_name,
        layers,
    )
}

fn synthesize_internal(
    _day_chi: &str,
    truc_name: &str,
    context: Option<&RecommendationSynthesisContext<'_>>,
    gio_hoang_dao: Option<&GioHoangDao>,
    tiet_khi_name: Option<&str>,
    layers: &[&dyn RecommendationLayer],
) -> Result<DailyRecommendations, RecommendationPackLookupError> {
    let mut hits = collect_base_hits(truc_name);
    let mut active_packs = Vec::new();

    if let Some(ctx) = context {
        hits.extend(collect_star_modifier_hits(ctx.day_fortune));
        hits.extend(collect_day_deity_modifier_hits(ctx.day_fortune));
        hits.extend(collect_taboo_modifier_hits(ctx.day_fortune));
        hits.extend(collect_xung_hop_modifier_hits(ctx.day_fortune));
        hits.extend(collect_travel_modifier_hits(ctx.day_fortune));

        if let Some(name) = tiet_khi_name {
            hits.extend(collect_tiet_khi_modifier_hits(name));
        }

        if let Some(hours) = gio_hoang_dao {
            hits.extend(collect_hours_modifier_hits(hours));
        }

        let event_kind_layer = EventKindLayer;
        if ctx.event_kind.is_some() {
            let extra = event_kind_layer.collect_hits(ctx).into_iter().map(|hit| CollectedHit {
                activity_id: hit.activity_id,
                source: hit.source,
                source_code: hit.source_code,
                direction: hit.direction,
                summary_vi: hit.summary_vi,
                summary_en: hit.summary_en,
                severity: hit.severity,
                hard_stop: allow_hard_stop(hit.source, hit.hard_stop, HitOrigin::ExtensionLayer),
            });
            hits.extend(extra);
        }

        let mut pack_layers: Vec<&dyn RecommendationLayer> = Vec::new();
        for descriptor in validate_enabled_pack_ids(ctx.enabled_pack_ids)? {
            match descriptor.pack_id {
                id if id == NHI_THAP_BAT_TU_PACK.pack_id => {
                    pack_layers.push(&NhiThapBatTuPack);
                    active_packs.push(descriptor.to_active());
                }
                id => {
                    return Err(RecommendationPackLookupError::UnsupportedPackId(
                        id.to_string(),
                    ));
                }
            }
        }

        for layer in pack_layers.into_iter().chain(layers.iter().copied()) {
            let _layer_id = layer.layer_id();
            let extra = layer.collect_hits(ctx).into_iter().map(|hit| CollectedHit {
                activity_id: hit.activity_id,
                source: hit.source,
                source_code: hit.source_code,
                direction: hit.direction,
                summary_vi: hit.summary_vi,
                summary_en: hit.summary_en,
                severity: hit.severity,
                hard_stop: allow_hard_stop(hit.source, hit.hard_stop, HitOrigin::ExtensionLayer),
            });
            hits.extend(extra);
        }
    }

    let activities = merge_hits(hits);
    let (summary_vi, summary_en) = build_summary(&activities);
    let (ruleset_id, ruleset_version, profile) = recommendation_provenance(context);

    Ok(DailyRecommendations {
        ruleset_id,
        ruleset_version,
        profile,
        scope: RecommendationScope::GeneralDay,
        version: if context.is_some() {
            "v1-layered".to_string()
        } else {
            "v1-base".to_string()
        },
        summary_vi,
        summary_en,
        active_packs,
        activities,
    })
}

fn validate_enabled_pack_ids(
    enabled_pack_ids: &[&str],
) -> Result<Vec<&'static RecommendationPackDescriptor>, RecommendationPackLookupError> {
    let mut descriptors = Vec::new();
    let mut seen = std::collections::BTreeSet::new();

    for pack_id in enabled_pack_ids {
        if !seen.insert(*pack_id) {
            return Err(RecommendationPackLookupError::DuplicatePackId(
                (*pack_id).to_string(),
            ));
        }

        let descriptor = recommendation_pack_descriptor(pack_id)?;
        descriptors.push(descriptor);
    }

    Ok(descriptors)
}

fn recommendation_pack_descriptor(
    pack_id: &str,
) -> Result<&'static RecommendationPackDescriptor, RecommendationPackLookupError> {
    match pack_id {
        id if id == NHI_THAP_BAT_TU_PACK.pack_id => Ok(&NHI_THAP_BAT_TU_PACK),
        _ => Err(RecommendationPackLookupError::UnknownPackId(
            pack_id.to_string(),
        )),
    }
}

fn recommendation_provenance(
    context: Option<&RecommendationSynthesisContext<'_>>,
) -> (String, String, String) {
    if let Some(ctx) = context {
        return (
            ctx.day_fortune.ruleset_id.clone(),
            ctx.day_fortune.ruleset_version.clone(),
            ctx.day_fortune.profile.clone(),
        );
    }

    let descriptor = default_ruleset().descriptor;
    (
        descriptor.id.to_string(),
        descriptor.version.to_string(),
        descriptor.profile.to_string(),
    )
}

fn collect_base_hits(truc_name: &str) -> Vec<CollectedHit> {
    let mut hits = Vec::new();

    if let Some(truc) = truc_insight(truc_name) {
        hits.extend(collect_truc_hits(truc).into_iter().map(base_to_collected));
    }

    hits
}

fn base_to_collected(hit: BaseEvidenceHit) -> CollectedHit {
    CollectedHit {
        activity_id: hit.activity_id,
        source: hit.source,
        source_code: hit.source_code,
        direction: hit.direction,
        summary_vi: hit.summary_vi,
        summary_en: hit.summary_en,
        severity: match hit.direction {
            BaseDirection::Favor => RecommendationSeverity::Primary,
            BaseDirection::Avoid => RecommendationSeverity::Override,
        },
        hard_stop: allow_hard_stop(hit.source, false, HitOrigin::CorePolicy),
    }
}

fn allow_hard_stop(
    source: RecommendationEvidenceSource,
    requested_hard_stop: bool,
    origin: HitOrigin,
) -> bool {
    requested_hard_stop
        && origin == HitOrigin::CorePolicy
        && matches!(source, RecommendationEvidenceSource::Taboo)
}

fn collect_star_modifier_hits(day_fortune: &DayFortune) -> Vec<CollectedHit> {
    let mut hits = Vec::new();

    for star in &day_fortune.stars.cat_tinh {
        let activities = match star.as_str() {
            "Thiên Đức" | "Nguyệt Đức" | "Thiên Quý" => vec![
                ActivityId::OpeningStart,
                ActivityId::ContractAgreement,
                ActivityId::WeddingEngagement,
            ],
            "Thiên Hỷ" => vec![ActivityId::WeddingEngagement, ActivityId::MeetingSocial],
            "Thanh Long" => vec![ActivityId::Travel, ActivityId::OpeningStart],
            _ => vec![ActivityId::OpeningStart],
        };

        for activity_id in activities {
            hits.push(CollectedHit {
                activity_id,
                source: RecommendationEvidenceSource::Stars,
                source_code: format!("stars.cat_tinh.{star}"),
                direction: BaseDirection::Favor,
                summary_vi: format!("Cát tinh {star} hỗ trợ"),
                summary_en: format!("Auspicious star {star} provides support"),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            });
        }
    }

    for star in &day_fortune.stars.sat_tinh {
        let activities = match star.as_str() {
            "Bạch Hổ" | "Thiên Hình" | "Thiên Lao" => vec![
                ActivityId::ConstructionGroundbreaking,
                ActivityId::WeddingEngagement,
                ActivityId::MedicalTreatment,
            ],
            "Chu Tước" => vec![ActivityId::LawsuitDispute, ActivityId::ContractAgreement],
            _ => vec![ActivityId::ContractAgreement],
        };

        for activity_id in activities {
            hits.push(CollectedHit {
                activity_id,
                source: RecommendationEvidenceSource::Stars,
                source_code: format!("stars.sat_tinh.{star}"),
                direction: BaseDirection::Avoid,
                summary_vi: format!("Sát tinh {star} gây bất lợi"),
                summary_en: format!("Inauspicious star {star} adds risk"),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            });
        }
    }

    hits
}

fn collect_day_deity_modifier_hits(day_fortune: &DayFortune) -> Vec<CollectedHit> {
    let Some(deity) = &day_fortune.day_deity else {
        return Vec::new();
    };

    let (direction, activities, summary_vi, summary_en) = match deity.classification {
        DayDeityClassification::HoangDao => (
            BaseDirection::Favor,
            vec![
                ActivityId::OpeningStart,
                ActivityId::Travel,
                ActivityId::MeetingSocial,
            ],
            format!("Hoàng đạo {} hỗ trợ việc hanh thông", deity.name),
            format!("Auspicious deity {} supports smooth activities", deity.name),
        ),
        DayDeityClassification::HacDao => (
            BaseDirection::Avoid,
            vec![
                ActivityId::ConstructionGroundbreaking,
                ActivityId::ContractAgreement,
                ActivityId::WeddingEngagement,
            ],
            format!("Hắc đạo {} cảnh báo việc trọng", deity.name),
            format!(
                "Inauspicious deity {} warns against major activities",
                deity.name
            ),
        ),
    };

    activities
        .into_iter()
        .map(|activity_id| CollectedHit {
            activity_id,
            source: RecommendationEvidenceSource::DayDeity,
            source_code: format!("day_deity.{}", deity.name),
            direction,
            summary_vi: summary_vi.clone(),
            summary_en: summary_en.clone(),
            severity: RecommendationSeverity::Supporting,
            hard_stop: false,
        })
        .collect()
}

fn collect_taboo_modifier_hits(day_fortune: &DayFortune) -> Vec<CollectedHit> {
    let mut hits = Vec::new();

    for taboo in &day_fortune.taboos {
        let hard_stop = matches!(taboo.severity.as_str(), "hard" | "high");
        let severity = if hard_stop {
            RecommendationSeverity::Override
        } else {
            RecommendationSeverity::Supporting
        };

        for entry in taboo_entry(&taboo.rule_id, severity) {
            hits.push(CollectedHit {
                activity_id: entry.activity_id,
                source: entry.source,
                source_code: format!("taboo.{}.{}", taboo.rule_id, taboo.severity),
                direction: entry.direction,
                summary_vi: format!("{}: {}", taboo.name, taboo.reason),
                summary_en: format!("{} taboo: {}", taboo.name, taboo.reason),
                severity: entry.severity,
                hard_stop: allow_hard_stop(
                    RecommendationEvidenceSource::Taboo,
                    hard_stop && entry.hard_stop_eligible,
                    HitOrigin::CorePolicy,
                ),
            });
        }
    }

    hits
}

fn collect_xung_hop_modifier_hits(day_fortune: &DayFortune) -> Vec<CollectedHit> {
    let mut hits = Vec::new();

    if !day_fortune.xung_hop.tu_hanh_xung.is_empty() {
        hits.push(CollectedHit {
            activity_id: ActivityId::LawsuitDispute,
            source: RecommendationEvidenceSource::XungHop,
            source_code: "xung_hop.tu_hanh_xung".to_string(),
            direction: BaseDirection::Avoid,
            summary_vi: "Tứ hành xung hiện diện, dễ phát sinh va chạm".to_string(),
            summary_en: "Four-way clash present, disputes can escalate".to_string(),
            severity: RecommendationSeverity::Supporting,
            hard_stop: false,
        });
        hits.push(CollectedHit {
            activity_id: ActivityId::ContractAgreement,
            source: RecommendationEvidenceSource::XungHop,
            source_code: "xung_hop.tu_hanh_xung".to_string(),
            direction: BaseDirection::Avoid,
            summary_vi: "thận trọng khi ràng buộc cam kết".to_string(),
            summary_en: "Exercise caution on binding agreements".to_string(),
            severity: RecommendationSeverity::Supporting,
            hard_stop: false,
        });
    }

    if day_fortune.xung_hop.liu_he.is_some() || !day_fortune.xung_hop.tam_hop.is_empty() {
        hits.push(CollectedHit {
            activity_id: ActivityId::MeetingSocial,
            source: RecommendationEvidenceSource::XungHop,
            source_code: "xung_hop.harmony".to_string(),
            direction: BaseDirection::Favor,
            summary_vi: "Quan hệ hợp khí thuận cho gặp gỡ và hòa giải".to_string(),
            summary_en: "Harmony signals support meetings and reconciliation".to_string(),
            severity: RecommendationSeverity::Supporting,
            hard_stop: false,
        });
    }

    hits
}

fn collect_hours_modifier_hits(gio_hoang_dao: &GioHoangDao) -> Vec<CollectedHit> {
    if gio_hoang_dao.good_hour_count >= 6 {
        return vec![
            CollectedHit {
                activity_id: ActivityId::Travel,
                source: RecommendationEvidenceSource::GioHoangDao,
                source_code: "gio_hoang_dao.good_hours".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: format!("Có {} giờ hoàng đạo", gio_hoang_dao.good_hour_count),
                summary_en: format!(
                    "{} auspicious hours are available",
                    gio_hoang_dao.good_hour_count
                ),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            },
            CollectedHit {
                activity_id: ActivityId::OpeningStart,
                source: RecommendationEvidenceSource::GioHoangDao,
                source_code: "gio_hoang_dao.good_hours".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: "Khung giờ thuận giúp triển khai việc mở đầu".to_string(),
                summary_en: "Auspicious time windows support new starts".to_string(),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            },
        ];
    }

    if gio_hoang_dao.good_hour_count <= 4 {
        return vec![CollectedHit {
            activity_id: ActivityId::Travel,
            source: RecommendationEvidenceSource::GioHoangDao,
            source_code: "gio_hoang_dao.low_good_hours".to_string(),
            direction: BaseDirection::Avoid,
            summary_vi: "Ít giờ hoàng đạo, cần chọn thời điểm kỹ".to_string(),
            summary_en: "Limited auspicious hours; timing needs extra care".to_string(),
            severity: RecommendationSeverity::Supporting,
            hard_stop: false,
        }];
    }

    Vec::new()
}

fn collect_travel_modifier_hits(day_fortune: &DayFortune) -> Vec<CollectedHit> {
    if day_fortune.travel.xuat_hanh_huong.trim().is_empty() {
        return Vec::new();
    }

    vec![CollectedHit {
        activity_id: ActivityId::Travel,
        source: RecommendationEvidenceSource::Travel,
        source_code: "travel.xuat_hanh_huong".to_string(),
        direction: BaseDirection::Favor,
        summary_vi: format!(
            "Có hướng xuất hành tham chiếu: {}",
            day_fortune.travel.xuat_hanh_huong
        ),
        summary_en: format!(
            "A travel direction is available: {}",
            day_fortune.travel.xuat_hanh_huong
        ),
        severity: RecommendationSeverity::Supporting,
        hard_stop: false,
    }]
}

fn collect_tiet_khi_modifier_hits(tiet_khi_name: &str) -> Vec<CollectedHit> {
    match tiet_khi_name {
        "Lập Xuân" | "Lập Hạ" | "Lập Thu" | "Lập Đông" => vec![
            CollectedHit {
                activity_id: ActivityId::OpeningStart,
                source: RecommendationEvidenceSource::TietKhi,
                source_code: "tiet_khi.transition_start".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: format!("{} thuận cho khởi sự có chuẩn bị", tiet_khi_name),
                summary_en: format!("{tiet_khi_name} supports prepared beginnings"),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            },
            CollectedHit {
                activity_id: ActivityId::MoveRelocation,
                source: RecommendationEvidenceSource::TietKhi,
                source_code: "tiet_khi.transition_start".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: "Thời khí chuyển mùa hỗ trợ việc sắp xếp chuyển dịch".to_string(),
                summary_en: "Seasonal transition supports relocation planning".to_string(),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            },
        ],
        "Đại Hàn" | "Tiểu Hàn" | "Đại Thử" => vec![CollectedHit {
            activity_id: ActivityId::ConstructionGroundbreaking,
            source: RecommendationEvidenceSource::TietKhi,
            source_code: "tiet_khi.extreme".to_string(),
            direction: BaseDirection::Avoid,
            summary_vi: format!("{} là tiết khí cực điểm cho việc nặng", tiet_khi_name),
            summary_en: format!("{tiet_khi_name} is climatically extreme for heavy tasks"),
            severity: RecommendationSeverity::Supporting,
            hard_stop: false,
        }],
        _ => Vec::new(),
    }
}

fn merge_hits(hits: Vec<CollectedHit>) -> Vec<SynthesizedRecommendation> {
    let mut by_activity: BTreeMap<String, AggregateRecommendation> = BTreeMap::new();

    for hit in hits {
        let key = format!("{:?}", hit.activity_id);
        let entry = by_activity
            .entry(key)
            .or_insert_with(|| AggregateRecommendation {
                activity: SynthesizedRecommendation {
                    activity_id: hit.activity_id,
                    label: hit.activity_id.labels(),
                    bucket: RecommendationBucket::CoThe,
                reasons: vec![],
                },
                saw_favor: false,
                saw_hard_stop: false,
                favor_sources: 0,
                strong_avoid_sources: 0,
                supporting_avoid_sources: 0,
            });

        match hit.direction {
            BaseDirection::Favor => {
                entry.saw_favor = true;
                entry.favor_sources += 1;
                if entry.strong_avoid_sources == 0 && entry.supporting_avoid_sources == 0 {
                    entry.activity.bucket = favored_bucket(hit.source);
                }
            }
            BaseDirection::Avoid => {
                if matches!(hit.severity, RecommendationSeverity::Override) {
                    entry.strong_avoid_sources += 1;
                } else {
                    entry.supporting_avoid_sources += 1;
                }
                if entry.strong_avoid_sources > 0 || !entry.saw_favor {
                    entry.activity.bucket = avoided_bucket();
                }
            }
        }

        if hit.hard_stop {
            entry.saw_hard_stop = true;
        }

        entry.activity.reasons.push(build_reason(&hit));
    }

    let mut activities: Vec<SynthesizedRecommendation> = by_activity
        .into_values()
        .map(|mut aggregate| {
            aggregate.activity.bucket = resolve_bucket(
                RecommendationPolicyInput {
                    activity_id: aggregate.activity.activity_id,
                    current: aggregate.activity.bucket,
                    saw_favor: aggregate.saw_favor,
                    saw_hard_stop: aggregate.saw_hard_stop,
                    favor_sources: aggregate.favor_sources,
                    strong_avoid_sources: aggregate.strong_avoid_sources,
                    supporting_avoid_sources: aggregate.supporting_avoid_sources,
                },
            );
            aggregate.activity.reasons.sort_by(|a, b| {
                severity_rank(a.severity)
                    .cmp(&severity_rank(b.severity))
                    .then_with(|| a.rule_id.cmp(&b.rule_id))
                    .then_with(|| a.summary_vi.cmp(&b.summary_vi))
            });
            aggregate.activity
        })
        .collect();

    activities.sort_by(|a, b| {
        bucket_rank(&a.bucket)
            .cmp(&bucket_rank(&b.bucket))
            .then_with(|| a.label.vi.cmp(&b.label.vi))
            .then_with(|| a.label.en.cmp(&b.label.en))
    });
    activities
}

fn build_reason(hit: &CollectedHit) -> RecommendationReason {
    let summary_vi = match (hit.activity_id, hit.direction) {
        (ActivityId::BurialMemorial, BaseDirection::Favor) => {
            format!("Cần thẩm định thêm: {}", hit.summary_vi)
        }
        (_, BaseDirection::Favor) => format!("Hợp cho {}", hit.summary_vi),
        (_, BaseDirection::Avoid) => format!("Nên tránh {}", hit.summary_vi),
    };
    let summary_en = match (hit.activity_id, hit.direction) {
        (ActivityId::BurialMemorial, BaseDirection::Favor) => {
            format!("Needs expert review: {}", hit.summary_en)
        }
        (_, BaseDirection::Favor) => format!("Suitable for {}", hit.summary_en),
        (_, BaseDirection::Avoid) => format!("Avoid {}", hit.summary_en),
    };

    RecommendationReason {
        rule_id: format!("{}.{}", source_prefix(hit.source), hit.source_code),
        severity: hit.severity,
        summary_vi,
        summary_en,
        evidence: RecommendationEvidence {
            source: hit.source,
            code: hit.source_code.clone(),
            note: evidence_note(hit.source),
        },
    }
}

fn source_prefix(source: RecommendationEvidenceSource) -> &'static str {
    match source {
        RecommendationEvidenceSource::DayGuidance => "base.day_guidance",
        RecommendationEvidenceSource::Truc => "base.truc",
        RecommendationEvidenceSource::Stars => "layer.stars",
        RecommendationEvidenceSource::DayDeity => "layer.day_deity",
        RecommendationEvidenceSource::Taboo => "layer.taboo",
        RecommendationEvidenceSource::XungHop => "layer.xung_hop",
        RecommendationEvidenceSource::TietKhi => "layer.tiet_khi",
        RecommendationEvidenceSource::GioHoangDao => "layer.gio_hoang_dao",
        RecommendationEvidenceSource::Travel => "layer.travel",
        RecommendationEvidenceSource::ProductRule => "layer.product_rule",
    }
}

fn evidence_note(source: RecommendationEvidenceSource) -> String {
    match source {
        RecommendationEvidenceSource::DayGuidance => "Derived from legacy day guidance".to_string(),
        RecommendationEvidenceSource::Truc => "Derived from truc day-duty guidance".to_string(),
        RecommendationEvidenceSource::Stars => "Derived from day star buckets".to_string(),
        RecommendationEvidenceSource::DayDeity => {
            "Derived from hoang dao/hac dao day deity".to_string()
        }
        RecommendationEvidenceSource::Taboo => "Derived from structured day taboos".to_string(),
        RecommendationEvidenceSource::XungHop => {
            "Derived from xung-hop conflict/harmony relationships".to_string()
        }
        RecommendationEvidenceSource::TietKhi => {
            "Derived from tiet-khi seasonal signal".to_string()
        }
        RecommendationEvidenceSource::GioHoangDao => {
            "Derived from good-hour distribution".to_string()
        }
        RecommendationEvidenceSource::Travel => {
            "Derived from travel-direction subsystem".to_string()
        }
        RecommendationEvidenceSource::ProductRule => {
            "Derived from extension layer rule".to_string()
        }
    }
}

fn build_summary(activities: &[SynthesizedRecommendation]) -> (String, String) {
    let nen = activities
        .iter()
        .filter(|activity| activity.bucket == RecommendationBucket::Nen)
        .count();
    let cothe = activities
        .iter()
        .filter(|activity| activity.bucket == RecommendationBucket::CoThe)
        .count();
    let tranh = activities
        .iter()
        .filter(|activity| activity.bucket == RecommendationBucket::Tranh)
        .count();
    let ky_manh = activities
        .iter()
        .filter(|activity| activity.bucket == RecommendationBucket::KyManh)
        .count();

    if ky_manh > 0 {
        (
            format!("Ngày cần kiêng trọng điểm, có {} việc kỵ mạnh", ky_manh),
            format!("A restricted day with {ky_manh} hard-stop activities"),
        )
    } else if nen > 0 && tranh == 0 {
        (
            format!("Ngày thuận cho {} việc chính", nen),
            format!("A supportive day for {nen} primary activities"),
        )
    } else if tranh > nen {
        (
            format!("Ngày nên thận trọng, có {} việc cần tránh", tranh),
            format!("A cautious day with {tranh} activities to avoid"),
        )
    } else {
        (
            format!("Ngày trung hòa, {} việc có thể cân nhắc", cothe.max(nen)),
            format!(
                "A balanced day with {} activities worth considering",
                cothe.max(nen)
            ),
        )
    }
}

fn severity_rank(severity: RecommendationSeverity) -> u8 {
    match severity {
        RecommendationSeverity::Override => 0,
        RecommendationSeverity::Primary => 1,
        RecommendationSeverity::Supporting => 2,
    }
}

fn bucket_rank(bucket: &RecommendationBucket) -> u8 {
    match bucket {
        RecommendationBucket::Nen => 0,
        RecommendationBucket::CoThe => 1,
        RecommendationBucket::Tranh => 2,
        RecommendationBucket::KyManh => 3,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        almanac::calc::calculate_day_fortune, canchi::get_day_canchi,
        gio_hoang_dao::get_gio_hoang_dao,
    };

    use super::*;

    #[test]
    fn truc_only_favor_stays_primary_without_legacy_guidance_promotion() {
        let recommendations = synthesize_base_daily_recommendations("Tý", "Khai");
        let opening = recommendations
            .activities
            .iter()
            .find(|activity| activity.label.vi == "Khai mở")
            .expect("opening recommendation exists");
        assert_eq!(opening.bucket, RecommendationBucket::CoThe);
    }

    #[test]
    fn direct_avoid_guidance_stays_conservative() {
        let recommendations = synthesize_base_daily_recommendations("Thìn", "Kiến");
        let dispute = recommendations
            .activities
            .iter()
            .find(|activity| activity.label.vi == "Kiện tụng")
            .expect("lawsuit recommendation exists");
        assert_eq!(dispute.bucket, RecommendationBucket::Tranh);
    }

    #[test]
    fn base_synthesis_does_not_emit_legacy_day_guidance_evidence() {
        let recommendations = synthesize_base_daily_recommendations("Tý", "Khai");
        assert!(recommendations.activities.iter().all(|activity| {
            activity
                .reasons
                .iter()
                .all(|reason| reason.evidence.source != RecommendationEvidenceSource::DayGuidance)
        }));
    }

    #[test]
    fn layered_hard_taboo_produces_ky_manh() {
        let day_canchi = get_day_canchi(crate::julian::jd_from_date(14, 2, 2024));
        let year_canchi = crate::canchi::get_year_canchi(2024);
        let day_fortune = calculate_day_fortune(
            crate::julian::jd_from_date(14, 2, 2024),
            &day_canchi,
            5,
            1,
            &year_canchi.can,
            "Lập Xuân",
        );
        assert!(day_fortune
            .taboos
            .iter()
            .any(|taboo| taboo.severity == "hard"));

        let gio = get_gio_hoang_dao(day_canchi.chi_index);
        let context = RecommendationSynthesisContext {
            day_chi: &day_canchi.chi,
            day_fortune: &day_fortune,
            gio_hoang_dao: Some(&gio),
            tiet_khi_name: Some("Lập Xuân"),
            profile_id: None,
            event_kind: None,
            enabled_pack_ids: &[],
        };

        let recommendations = synthesize_daily_recommendations(&context);
        assert!(recommendations
            .activities
            .iter()
            .any(|activity| activity.bucket == RecommendationBucket::KyManh));
        assert_eq!(recommendations.ruleset_id, day_fortune.ruleset_id);
        assert_eq!(recommendations.ruleset_version, day_fortune.ruleset_version);
        assert_eq!(recommendations.profile, day_fortune.profile);
        assert_eq!(recommendations.version, "v1-layered");
    }

    #[test]
    fn every_emitted_activity_has_reasons() {
        let recommendations = synthesize_base_daily_recommendations("Tý", "Khai");
        assert!(!recommendations.activities.is_empty());
        assert!(recommendations
            .activities
            .iter()
            .all(|activity| !activity.reasons.is_empty()));
        assert!(!recommendations.summary_vi.is_empty());
    }

    #[test]
    fn reason_order_is_deterministic() {
        let recommendations_a = synthesize_base_daily_recommendations("Tý", "Khai");
        let recommendations_b = synthesize_base_daily_recommendations("Tý", "Khai");
        assert_eq!(recommendations_a.activities, recommendations_b.activities);
    }

    #[test]
    fn supports_extension_layer_hits() {
        struct TestLayer;
        impl RecommendationLayer for TestLayer {
            fn layer_id(&self) -> &'static str {
                "test.layer"
            }

            fn collect_hits(
                &self,
                _context: &RecommendationSynthesisContext<'_>,
            ) -> Vec<RecommendationLayerHit> {
                vec![RecommendationLayerHit {
                    activity_id: ActivityId::MedicalTreatment,
                    source: RecommendationEvidenceSource::ProductRule,
                    source_code: "test.layer.medical".to_string(),
                    direction: BaseDirection::Favor,
                    summary_vi: "Bổ sung theo ngữ cảnh sự kiện".to_string(),
                    summary_en: "Added from event context".to_string(),
                    severity: RecommendationSeverity::Supporting,
                    hard_stop: false,
                }]
            }
        }

        let info = crate::get_day_info(10, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &info.canchi.day.chi,
            day_fortune: &info.day_fortune,
            gio_hoang_dao: Some(&info.gio_hoang_dao),
            tiet_khi_name: Some(&info.tiet_khi.name),
            profile_id: Some("default"),
            event_kind: Some("medical_checkup"),
            enabled_pack_ids: &[],
        };
        let test_layer = TestLayer;
        let recommendations = synthesize_daily_recommendations_with_layers(&context, &[&test_layer])
            .expect("layered recommendations");

        assert!(recommendations
            .activities
            .iter()
            .any(|activity| activity.activity_id == ActivityId::MedicalTreatment));
    }

    #[test]
    fn supporting_avoid_modifier_does_not_override_primary_favor() {
        let recommendations = merge_hits(vec![
            CollectedHit {
                activity_id: ActivityId::OpeningStart,
                source: RecommendationEvidenceSource::Truc,
                source_code: "truc.khai.good_for".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: "Khai mở".to_string(),
                summary_en: "opening".to_string(),
                severity: RecommendationSeverity::Primary,
                hard_stop: false,
            },
            CollectedHit {
                activity_id: ActivityId::OpeningStart,
                source: RecommendationEvidenceSource::DayDeity,
                source_code: "day_deity.cau_tran".to_string(),
                direction: BaseDirection::Avoid,
                summary_vi: "Hắc đạo cảnh báo việc trọng".to_string(),
                summary_en: "inauspicious deity warns against major tasks".to_string(),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            },
        ]);

        let opening = recommendations
            .iter()
            .find(|activity| activity.activity_id == ActivityId::OpeningStart)
            .expect("opening recommendation exists");
        assert_eq!(opening.bucket, RecommendationBucket::CoThe);
    }

    #[test]
    fn extension_layers_cannot_escalate_to_ky_manh() {
        struct HardStopLayer;
        impl RecommendationLayer for HardStopLayer {
            fn layer_id(&self) -> &'static str {
                "test.hard-stop"
            }

            fn collect_hits(
                &self,
                _context: &RecommendationSynthesisContext<'_>,
            ) -> Vec<RecommendationLayerHit> {
                vec![RecommendationLayerHit {
                    activity_id: ActivityId::ContractAgreement,
                    source: RecommendationEvidenceSource::Taboo,
                    source_code: "extension.spoofed_taboo".to_string(),
                    direction: BaseDirection::Avoid,
                    summary_vi: "Lớp mở rộng yêu cầu chặn cứng".to_string(),
                    summary_en: "Extension requested hard stop".to_string(),
                    severity: RecommendationSeverity::Override,
                    hard_stop: true,
                }]
            }
        }

        let info = crate::get_day_info(10, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &info.canchi.day.chi,
            day_fortune: &info.day_fortune,
            gio_hoang_dao: Some(&info.gio_hoang_dao),
            tiet_khi_name: Some(&info.tiet_khi.name),
            profile_id: Some("default"),
            event_kind: Some("contract_signing"),
            enabled_pack_ids: &[],
        };
        let layer = HardStopLayer;
        let recommendations = synthesize_daily_recommendations_with_layers(&context, &[&layer])
            .expect("layered recommendations");

        let contract = recommendations
            .activities
            .iter()
            .find(|activity| activity.activity_id == ActivityId::ContractAgreement)
            .expect("contract recommendation exists");
        assert_eq!(contract.bucket, RecommendationBucket::Tranh);
        assert!(contract
            .reasons
            .iter()
            .any(|reason| reason.rule_id == "layer.taboo.extension.spoofed_taboo"));
    }

    #[test]
    fn burial_recommendations_are_capped_to_conservative_bucket() {
        let recommendations = merge_hits(vec![
            CollectedHit {
                activity_id: ActivityId::BurialMemorial,
                source: RecommendationEvidenceSource::Truc,
                source_code: "truc.tru.good_for".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: "An táng".to_string(),
                summary_en: "burial".to_string(),
                severity: RecommendationSeverity::Primary,
                hard_stop: false,
            },
            CollectedHit {
                activity_id: ActivityId::BurialMemorial,
                source: RecommendationEvidenceSource::Travel,
                source_code: "travel.reference".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: "Có tín hiệu tham khảo".to_string(),
                summary_en: "supporting reference".to_string(),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            },
        ]);

        let burial = recommendations
            .iter()
            .find(|activity| activity.activity_id == ActivityId::BurialMemorial)
            .expect("burial recommendation exists");
        assert_eq!(burial.bucket, RecommendationBucket::CoThe);
        assert!(burial
            .reasons
            .iter()
            .all(|reason| !reason.summary_vi.starts_with("Hợp cho")));
    }

    #[test]
    fn enabled_pack_metadata_is_emitted() {
        let info = crate::get_day_info(10, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &info.canchi.day.chi,
            day_fortune: &info.day_fortune,
            gio_hoang_dao: Some(&info.gio_hoang_dao),
            tiet_khi_name: Some(&info.tiet_khi.name),
            profile_id: None,
            event_kind: None,
            enabled_pack_ids: &[NHI_THAP_BAT_TU_PACK.pack_id],
        };

        let recommendations = synthesize_daily_recommendations(&context);
        assert_eq!(recommendations.active_packs.len(), 1);
        assert_eq!(
            recommendations.active_packs[0].pack_id,
            NHI_THAP_BAT_TU_PACK.pack_id
        );
    }

    #[test]
    fn event_kind_layer_adds_contract_activity_reason() {
        let info = crate::get_day_info(10, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &info.canchi.day.chi,
            day_fortune: &info.day_fortune,
            gio_hoang_dao: Some(&info.gio_hoang_dao),
            tiet_khi_name: Some(&info.tiet_khi.name),
            profile_id: Some("session"),
            event_kind: Some("contract_signing"),
            enabled_pack_ids: &[],
        };

        let recommendations = synthesize_daily_recommendations(&context);
        let contract = recommendations
            .activities
            .iter()
            .find(|activity| activity.activity_id == ActivityId::ContractAgreement)
            .expect("contract activity exists");

        assert!(contract
            .reasons
            .iter()
            .any(|reason| reason.rule_id == "layer.product_rule.event_kind.contract_signing"));
    }

    #[test]
    fn duplicate_pack_ids_fail_explicitly() {
        let info = crate::get_day_info(10, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &info.canchi.day.chi,
            day_fortune: &info.day_fortune,
            gio_hoang_dao: Some(&info.gio_hoang_dao),
            tiet_khi_name: Some(&info.tiet_khi.name),
            profile_id: None,
            event_kind: None,
            enabled_pack_ids: &[NHI_THAP_BAT_TU_PACK.pack_id, NHI_THAP_BAT_TU_PACK.pack_id],
        };

        let err = synthesize_daily_recommendations_with_layers(&context, &[])
            .expect_err("duplicate pack ids must fail");
        assert_eq!(
            err.to_string(),
            format!("duplicate recommendation pack id: {}", NHI_THAP_BAT_TU_PACK.pack_id)
        );
    }

    #[test]
    fn unknown_pack_ids_fail_explicitly() {
        let info = crate::get_day_info(10, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &info.canchi.day.chi,
            day_fortune: &info.day_fortune,
            gio_hoang_dao: Some(&info.gio_hoang_dao),
            tiet_khi_name: Some(&info.tiet_khi.name),
            profile_id: None,
            event_kind: None,
            enabled_pack_ids: &["pack.unknown.v1"],
        };

        let err = synthesize_daily_recommendations_with_layers(&context, &[])
            .expect_err("unknown pack ids must fail");
        assert_eq!(
            err.to_string(),
            "unknown recommendation pack id: pack.unknown.v1"
        );
    }
}
