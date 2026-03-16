use crate::almanac::types::StarQuality;

use super::super::{
    activity::ActivityId, pack::RecommendationPackDescriptor, BaseDirection, RecommendationLayer,
    RecommendationLayerHit, RecommendationPackMode, RecommendationSeverity,
    RecommendationSynthesisContext, RecommendationEvidenceSource,
};

pub const NHI_THAP_BAT_TU_PACK: RecommendationPackDescriptor = RecommendationPackDescriptor {
    pack_id: "pack.nhi_thap_bat_tu.v1",
    version: "v1",
    source_family: "nhi_thap_bat_tu",
    mode: RecommendationPackMode::Advisory,
};

pub struct NhiThapBatTuPack;

impl RecommendationLayer for NhiThapBatTuPack {
    fn layer_id(&self) -> &'static str {
        NHI_THAP_BAT_TU_PACK.pack_id
    }

    fn collect_hits(
        &self,
        context: &RecommendationSynthesisContext<'_>,
    ) -> Vec<RecommendationLayerHit> {
        let Some(day_star) = context.day_fortune.stars.day_star.as_ref() else {
            return Vec::new();
        };

        match day_star.quality {
            StarQuality::Cat => vec![RecommendationLayerHit {
                activity_id: ActivityId::OpeningStart,
                source: RecommendationEvidenceSource::ProductRule,
                source_code: "pack.nhi_thap_bat_tu.day_star.cat".to_string(),
                direction: BaseDirection::Favor,
                summary_vi: format!("Nhị thập bát tú {} cho tín hiệu hỗ trợ", day_star.name),
                summary_en: format!("28 mansion {} provides advisory support", day_star.name),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            }],
            StarQuality::Hung => vec![RecommendationLayerHit {
                activity_id: ActivityId::ConstructionGroundbreaking,
                source: RecommendationEvidenceSource::ProductRule,
                source_code: "pack.nhi_thap_bat_tu.day_star.hung".to_string(),
                direction: BaseDirection::Avoid,
                summary_vi: format!("Nhị thập bát tú {} phát tín hiệu bất lợi", day_star.name),
                summary_en: format!("28 mansion {} adds advisory caution", day_star.name),
                severity: RecommendationSeverity::Supporting,
                hard_stop: false,
            }],
            StarQuality::Binh => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate_day_snapshot;

    use super::*;

    #[test]
    fn emits_advisory_hits_when_day_star_present() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &snapshot.context.canchi.day.chi,
            day_fortune: &snapshot.day_fortune,
            gio_hoang_dao: Some(&snapshot.context.gio_hoang_dao),
            tiet_khi_name: Some(&snapshot.context.tiet_khi.name),
            profile_id: None,
            event_kind: None,
            enabled_pack_ids: &[NHI_THAP_BAT_TU_PACK.pack_id],
        };

        let hits = NhiThapBatTuPack.collect_hits(&context);
        if let Some(day_star) = snapshot.day_fortune.stars.day_star.as_ref() {
            match day_star.quality {
                StarQuality::Binh => assert!(hits.is_empty()),
                _ => assert_eq!(hits.len(), 1),
            }
        }
    }
}
