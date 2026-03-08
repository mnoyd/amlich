use std::collections::BTreeMap;

use crate::insight_data::{get_day_guidance, DayGuidance};

use super::{
    evidence::{collect_truc_hits, normalize_legacy_guidance_hits},
    rules::{avoided_bucket, favored_bucket, truc_insight},
    BaseDirection, BaseEvidenceHit, DailyRecommendations, RecommendationBucket,
    RecommendationEvidence, RecommendationReason, RecommendationScope, RecommendationSeverity,
    SynthesizedRecommendation,
};

#[derive(Debug, Clone)]
struct AggregateRecommendation {
    activity: SynthesizedRecommendation,
    saw_favor: bool,
    saw_avoid: bool,
    favor_sources: usize,
}

pub fn synthesize_base_daily_recommendations(
    day_chi: &str,
    truc_name: &str,
) -> DailyRecommendations {
    let legacy_guidance = get_day_guidance(day_chi);
    let hits = collect_base_hits(day_chi, truc_name, legacy_guidance);
    let activities = merge_hits(hits);
    let (summary_vi, summary_en) = build_summary(&activities);

    DailyRecommendations {
        scope: RecommendationScope::GeneralDay,
        version: "v1-base".to_string(),
        summary_vi,
        summary_en,
        activities,
    }
}

fn collect_base_hits(
    day_chi: &str,
    truc_name: &str,
    legacy_guidance: Option<&DayGuidance>,
) -> Vec<BaseEvidenceHit> {
    let mut hits = Vec::new();

    if let Some(guidance) = legacy_guidance {
        hits.extend(normalize_legacy_guidance_hits(guidance));
    }

    if let Some(truc) = truc_insight(truc_name) {
        hits.extend(collect_truc_hits(truc));
    }

    if hits.is_empty() {
        let note = format!("No normalized base recommendation hits for day chi {day_chi}");
        let _ = note;
    }

    hits
}

fn merge_hits(hits: Vec<BaseEvidenceHit>) -> Vec<SynthesizedRecommendation> {
    let mut by_activity: BTreeMap<String, AggregateRecommendation> = BTreeMap::new();

    for hit in hits {
        let key = format!("{:?}", hit.activity_id);
        let entry = by_activity
            .entry(key)
            .or_insert_with(|| AggregateRecommendation {
                activity: SynthesizedRecommendation {
                    activity_id: hit.activity_id,
                    label: hit.label.clone(),
                    bucket: RecommendationBucket::CoThe,
                    reasons: vec![],
                },
                saw_favor: false,
                saw_avoid: false,
                favor_sources: 0,
            });

        match hit.direction {
            BaseDirection::Favor => {
                entry.saw_favor = true;
                entry.favor_sources += 1;
                if !entry.saw_avoid {
                    entry.activity.bucket = favored_bucket(hit.source);
                }
            }
            BaseDirection::Avoid => {
                entry.saw_avoid = true;
                entry.activity.bucket = avoided_bucket();
            }
        }

        entry.activity.reasons.push(build_reason(&hit));
    }

    let mut activities: Vec<SynthesizedRecommendation> = by_activity
        .into_values()
        .map(|mut aggregate| {
            aggregate.activity.bucket = resolve_bucket(
                aggregate.activity.bucket,
                aggregate.saw_favor,
                aggregate.saw_avoid,
                aggregate.favor_sources,
            );
            aggregate.activity
        })
        .collect();

    activities.sort_by(|a, b| {
        bucket_rank(&a.bucket)
            .cmp(&bucket_rank(&b.bucket))
            .then_with(|| a.label.vi.cmp(&b.label.vi))
    });
    activities
}

fn resolve_bucket(
    current: RecommendationBucket,
    saw_favor: bool,
    saw_avoid: bool,
    favor_sources: usize,
) -> RecommendationBucket {
    if saw_avoid {
        return RecommendationBucket::Tranh;
    }

    if saw_favor && favor_sources >= 2 {
        return RecommendationBucket::Nen;
    }

    current
}

fn build_reason(hit: &BaseEvidenceHit) -> RecommendationReason {
    RecommendationReason {
        rule_id: match hit.source {
            super::RecommendationEvidenceSource::DayGuidance => "base.day_guidance".to_string(),
            super::RecommendationEvidenceSource::Truc => "base.truc".to_string(),
            _ => "base.unknown".to_string(),
        },
        severity: match hit.direction {
            BaseDirection::Favor => RecommendationSeverity::Primary,
            BaseDirection::Avoid => RecommendationSeverity::Override,
        },
        summary_vi: match hit.direction {
            BaseDirection::Favor => format!("Hợp cho {}", hit.summary_vi),
            BaseDirection::Avoid => format!("Nên tránh {}", hit.summary_vi),
        },
        summary_en: match hit.direction {
            BaseDirection::Favor => format!("Suitable for {}", hit.summary_en),
            BaseDirection::Avoid => format!("Avoid {}", hit.summary_en),
        },
        evidence: RecommendationEvidence {
            source: hit.source,
            code: hit.source_code.clone(),
            note: match hit.source {
                super::RecommendationEvidenceSource::DayGuidance => {
                    "Derived from legacy day guidance".to_string()
                }
                super::RecommendationEvidenceSource::Truc => {
                    "Derived from truc day-duty guidance".to_string()
                }
                _ => "Derived from base recommendation evidence".to_string(),
            },
        },
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

    if nen > 0 && tranh == 0 {
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
    use super::*;

    #[test]
    fn agreement_between_truc_and_guidance_promotes_to_nen() {
        let recommendations = synthesize_base_daily_recommendations("Tý", "Khai");
        let opening = recommendations
            .activities
            .iter()
            .find(|activity| activity.label.vi == "Khai mở")
            .expect("opening recommendation exists");
        assert_eq!(opening.bucket, RecommendationBucket::Nen);
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
    fn every_emitted_activity_has_reasons() {
        let recommendations = synthesize_base_daily_recommendations("Tý", "Khai");
        assert!(!recommendations.activities.is_empty());
        assert!(recommendations
            .activities
            .iter()
            .all(|activity| !activity.reasons.is_empty()));
        assert!(!recommendations.summary_vi.is_empty());
    }
}
