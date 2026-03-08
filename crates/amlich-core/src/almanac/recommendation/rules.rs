use crate::insight_data::find_truc_insight;

use super::RecommendationBucket;

pub fn truc_insight(name: &str) -> Option<&'static crate::insight_data::TrucInsight> {
    find_truc_insight(name)
}

pub fn favored_bucket(source: super::RecommendationEvidenceSource) -> RecommendationBucket {
    match source {
        super::RecommendationEvidenceSource::Truc => RecommendationBucket::CoThe,
        super::RecommendationEvidenceSource::DayGuidance => RecommendationBucket::CoThe,
        _ => RecommendationBucket::CoThe,
    }
}

pub fn avoided_bucket() -> RecommendationBucket {
    RecommendationBucket::Tranh
}

#[cfg(test)]
mod tests {
    use crate::almanac::truc::TRUC_NAMES;

    use super::*;

    #[test]
    fn every_supported_truc_has_insight() {
        for name in TRUC_NAMES {
            assert!(truc_insight(name).is_some(), "missing insight for {name}");
        }
    }
}
