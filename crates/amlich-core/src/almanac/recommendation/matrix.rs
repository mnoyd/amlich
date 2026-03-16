use super::{
    activity::ActivityId, BaseDirection, RecommendationEvidenceSource, RecommendationSeverity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecommendationMatrixEntry {
    pub activity_id: ActivityId,
    pub source: RecommendationEvidenceSource,
    pub direction: BaseDirection,
    pub severity: RecommendationSeverity,
    pub hard_stop_eligible: bool,
    pub sensitive_domain: bool,
}

pub fn taboo_entry(
    rule_id: &str,
    severity: RecommendationSeverity,
) -> Vec<RecommendationMatrixEntry> {
    let hard_stop_eligible = matches!(severity, RecommendationSeverity::Override);

    taboo_target_activities(rule_id)
        .into_iter()
        .map(|activity_id| RecommendationMatrixEntry {
            activity_id,
            source: RecommendationEvidenceSource::Taboo,
            direction: BaseDirection::Avoid,
            severity,
            hard_stop_eligible,
            sensitive_domain: activity_id == ActivityId::BurialMemorial,
        })
        .collect()
}

fn taboo_target_activities(rule_id: &str) -> Vec<ActivityId> {
    match rule_id {
        "tam_nuong" => vec![
            ActivityId::WeddingEngagement,
            ActivityId::ConstructionGroundbreaking,
            ActivityId::OpeningStart,
            ActivityId::ContractAgreement,
            ActivityId::FinanceInvestment,
        ],
        "nguyet_ky" => vec![
            ActivityId::ConstructionGroundbreaking,
            ActivityId::MoveRelocation,
            ActivityId::WeddingEngagement,
        ],
        _ => vec![ActivityId::OpeningStart, ActivityId::ContractAgreement],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn taboo_matrix_maps_tam_nuong_to_major_activities() {
        let entries = taboo_entry("tam_nuong", RecommendationSeverity::Override);
        assert!(entries
            .iter()
            .any(|entry| entry.activity_id == ActivityId::WeddingEngagement));
        assert!(entries.iter().all(|entry| entry.hard_stop_eligible));
    }
}
