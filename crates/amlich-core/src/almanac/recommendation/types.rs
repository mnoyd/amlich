use serde::{Deserialize, Serialize};

use super::activity::{ActivityId, ActivityLabel};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationPackMode {
    Advisory,
    TraditionVariant,
    Experimental,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveRecommendationPack {
    pub pack_id: String,
    pub version: String,
    pub source_family: String,
    pub mode: RecommendationPackMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationScope {
    GeneralDay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationBucket {
    Nen,
    CoThe,
    Tranh,
    KyManh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationSeverity {
    Primary,
    Supporting,
    Override,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationEvidenceSource {
    DayGuidance,
    Truc,
    Stars,
    DayDeity,
    Taboo,
    XungHop,
    TietKhi,
    GioHoangDao,
    Travel,
    ProductRule,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationEvidence {
    pub source: RecommendationEvidenceSource,
    pub code: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecommendationReason {
    pub rule_id: String,
    pub severity: RecommendationSeverity,
    pub summary_vi: String,
    pub summary_en: String,
    pub evidence: RecommendationEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SynthesizedRecommendation {
    pub activity_id: ActivityId,
    pub label: ActivityLabel,
    pub bucket: RecommendationBucket,
    #[serde(default)]
    pub reasons: Vec<RecommendationReason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DailyRecommendations {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub scope: RecommendationScope,
    pub version: String,
    pub summary_vi: String,
    pub summary_en: String,
    #[serde(default)]
    pub active_packs: Vec<ActiveRecommendationPack>,
    #[serde(default)]
    pub activities: Vec<SynthesizedRecommendation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_daily_recommendations_contract() {
        let rec = DailyRecommendations {
            ruleset_id: "vn_baseline_v1".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            scope: RecommendationScope::GeneralDay,
            version: "v1alpha".to_string(),
            summary_vi: "Ngay hop viec nho".to_string(),
            summary_en: "A day that suits smaller tasks".to_string(),
            active_packs: vec![ActiveRecommendationPack {
                pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
                version: "v1".to_string(),
                source_family: "nhi_thap_bat_tu".to_string(),
                mode: RecommendationPackMode::Advisory,
            }],
            activities: vec![SynthesizedRecommendation {
                activity_id: ActivityId::Travel,
                label: ActivityId::Travel.labels(),
                bucket: RecommendationBucket::Nen,
                reasons: vec![RecommendationReason {
                    rule_id: "seed.example".to_string(),
                    severity: RecommendationSeverity::Primary,
                    summary_vi: "Vi du ly do".to_string(),
                    summary_en: "Example reason".to_string(),
                    evidence: RecommendationEvidence {
                        source: RecommendationEvidenceSource::DayGuidance,
                        code: "day-guidance.seed".to_string(),
                        note: "Example provenance".to_string(),
                    },
                }],
            }],
        };

        let json = serde_json::to_string(&rec).expect("serialize recommendations");
        let decoded: DailyRecommendations =
            serde_json::from_str(&json).expect("deserialize recommendations");

        assert_eq!(decoded.scope, RecommendationScope::GeneralDay);
        assert_eq!(decoded.ruleset_id, "vn_baseline_v1");
        assert_eq!(decoded.ruleset_version, "v1");
        assert_eq!(decoded.profile, "baseline");
        assert_eq!(decoded.active_packs.len(), 1);
        assert_eq!(decoded.activities.len(), 1);
        assert_eq!(decoded.activities[0].bucket, RecommendationBucket::Nen);
    }
}
