use crate::insight_data::{DayGuidance, TrucInsight};

use super::{normalize_activity_alias, ActivityId, ActivityLabel, RecommendationEvidenceSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseDirection {
    Favor,
    Avoid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseEvidenceHit {
    pub activity_id: ActivityId,
    pub label: ActivityLabel,
    pub source: RecommendationEvidenceSource,
    pub source_code: String,
    pub direction: BaseDirection,
    pub summary_vi: String,
    pub summary_en: String,
}

pub fn normalize_legacy_guidance_hits(guidance: &DayGuidance) -> Vec<BaseEvidenceHit> {
    let mut hits = normalize_list(
        &guidance.good_for.vi,
        RecommendationEvidenceSource::DayGuidance,
        BaseDirection::Favor,
        "day_guidance.good_for",
    );
    hits.extend(normalize_list(
        &guidance.avoid_for.vi,
        RecommendationEvidenceSource::DayGuidance,
        BaseDirection::Avoid,
        "day_guidance.avoid_for",
    ));
    hits
}

pub fn collect_truc_hits(truc: &TrucInsight) -> Vec<BaseEvidenceHit> {
    let mut hits = normalize_list(
        &truc.good_for.vi,
        RecommendationEvidenceSource::Truc,
        BaseDirection::Favor,
        &format!("truc.{}.good_for", truc.id),
    );
    hits.extend(normalize_list(
        &truc.avoid_for.vi,
        RecommendationEvidenceSource::Truc,
        BaseDirection::Avoid,
        &format!("truc.{}.avoid_for", truc.id),
    ));
    hits
}

fn normalize_list(
    values: &[String],
    source: RecommendationEvidenceSource,
    direction: BaseDirection,
    source_code: &str,
) -> Vec<BaseEvidenceHit> {
    values
        .iter()
        .filter_map(|value| {
            let normalized = normalize_activity_alias(value)?;
            Some(BaseEvidenceHit {
                activity_id: normalized.activity_id,
                label: normalized.label,
                source,
                source_code: source_code.to_string(),
                direction,
                summary_vi: value.clone(),
                summary_en: normalized.matched_alias,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::insight_data::{BilingualList, BilingualText, DayGuidance, TrucInsight};

    use super::*;

    #[test]
    fn normalizes_legacy_guidance_hits_and_ignores_unknowns() {
        let guidance = DayGuidance {
            good_for: BilingualList {
                vi: vec!["Xuất hành".to_string(), "Đọc sách".to_string()],
                en: vec![],
            },
            avoid_for: BilingualList {
                vi: vec!["Động thổ".to_string()],
                en: vec![],
            },
        };

        let hits = normalize_legacy_guidance_hits(&guidance);
        assert_eq!(hits.len(), 2);
        assert!(hits.iter().any(|hit| hit.activity_id == ActivityId::Travel));
        assert!(hits
            .iter()
            .any(|hit| hit.activity_id == ActivityId::ConstructionGroundbreaking));
    }

    #[test]
    fn collects_truc_hits_from_truc_insight() {
        let truc = TrucInsight {
            id: "Khai".to_string(),
            meaning: BilingualText {
                vi: "...".to_string(),
                en: "...".to_string(),
            },
            good_for: BilingualList {
                vi: vec!["Khai trương".to_string(), "Xuất hành".to_string()],
                en: vec![],
            },
            avoid_for: BilingualList {
                vi: vec!["An táng".to_string()],
                en: vec![],
            },
        };

        let hits = collect_truc_hits(&truc);
        assert!(hits
            .iter()
            .any(|hit| hit.activity_id == ActivityId::OpeningStart));
        assert!(hits.iter().any(|hit| hit.activity_id == ActivityId::Travel));
        assert!(hits
            .iter()
            .any(|hit| hit.activity_id == ActivityId::BurialMemorial));
    }
}
