use super::{
    activity::ActivityId, BaseDirection, RecommendationEvidenceSource, RecommendationLayer,
    RecommendationLayerHit, RecommendationSeverity, RecommendationSynthesisContext,
};

pub struct EventKindLayer;

impl RecommendationLayer for EventKindLayer {
    fn layer_id(&self) -> &'static str {
        "profile.event_kind"
    }

    fn collect_hits(
        &self,
        context: &RecommendationSynthesisContext<'_>,
    ) -> Vec<RecommendationLayerHit> {
        match context.event_kind {
            Some("contract_signing") => vec![RecommendationLayerHit {
                activity_id: ActivityId::ContractAgreement,
                source: RecommendationEvidenceSource::ProductRule,
                source_code: "event_kind.contract_signing".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: "Ngữ cảnh sự kiện ưu tiên việc ký kết".to_string(),
                summary_en: "Event context prioritizes contract work".to_string(),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            }],
            Some("medical_checkup") => vec![RecommendationLayerHit {
                activity_id: ActivityId::MedicalTreatment,
                source: RecommendationEvidenceSource::ProductRule,
                source_code: "event_kind.medical_checkup".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: "Ngữ cảnh sự kiện nhấn mạnh việc chữa bệnh".to_string(),
                summary_en: "Event context emphasizes medical care".to_string(),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            }],
            Some("travel") => vec![RecommendationLayerHit {
                activity_id: ActivityId::Travel,
                source: RecommendationEvidenceSource::ProductRule,
                source_code: "event_kind.travel".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: "Ngữ cảnh sự kiện ưu tiên xuất hành".to_string(),
                summary_en: "Event context prioritizes travel".to_string(),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            }],
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate_day_snapshot;

    use super::*;

    #[test]
    fn emits_contract_event_hit() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &snapshot.context.canchi.day.chi,
            day_fortune: &snapshot.day_fortune,
            gio_hoang_dao: Some(&snapshot.context.gio_hoang_dao),
            tiet_khi_name: Some(&snapshot.context.tiet_khi.name),
            profile_id: Some("session"),
            event_kind: Some("contract_signing"),
            enabled_pack_ids: &[],
        };

        let hits = EventKindLayer.collect_hits(&context);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].activity_id, ActivityId::ContractAgreement);
    }
}
