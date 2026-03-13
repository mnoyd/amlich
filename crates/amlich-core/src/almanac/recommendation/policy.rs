use super::{activity::ActivityId, RecommendationBucket};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecommendationPolicyInput {
    pub activity_id: ActivityId,
    pub current: RecommendationBucket,
    pub saw_favor: bool,
    pub saw_hard_stop: bool,
    pub favor_sources: usize,
    pub strong_avoid_sources: usize,
    pub supporting_avoid_sources: usize,
}

pub fn resolve_bucket(input: RecommendationPolicyInput) -> RecommendationBucket {
    if input.saw_hard_stop {
        return RecommendationBucket::KyManh;
    }

    if input.strong_avoid_sources > 0 {
        return apply_activity_guardrails(input.activity_id, RecommendationBucket::Tranh);
    }

    if input.saw_favor && input.supporting_avoid_sources > 0 {
        return apply_activity_guardrails(input.activity_id, RecommendationBucket::CoThe);
    }

    if input.supporting_avoid_sources > 0 {
        return apply_activity_guardrails(input.activity_id, RecommendationBucket::Tranh);
    }

    if input.saw_favor && input.favor_sources >= 2 {
        return apply_activity_guardrails(input.activity_id, RecommendationBucket::Nen);
    }

    apply_activity_guardrails(input.activity_id, input.current)
}

pub fn apply_activity_guardrails(
    activity_id: ActivityId,
    bucket: RecommendationBucket,
) -> RecommendationBucket {
    match activity_id {
        ActivityId::BurialMemorial if bucket == RecommendationBucket::Nen => {
            RecommendationBucket::CoThe
        }
        _ => bucket,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hard_stop_wins() {
        let bucket = resolve_bucket(RecommendationPolicyInput {
            activity_id: ActivityId::OpeningStart,
            current: RecommendationBucket::CoThe,
            saw_favor: true,
            saw_hard_stop: true,
            favor_sources: 2,
            strong_avoid_sources: 0,
            supporting_avoid_sources: 0,
        });

        assert_eq!(bucket, RecommendationBucket::KyManh);
    }

    #[test]
    fn burial_positive_is_capped() {
        let bucket = resolve_bucket(RecommendationPolicyInput {
            activity_id: ActivityId::BurialMemorial,
            current: RecommendationBucket::CoThe,
            saw_favor: true,
            saw_hard_stop: false,
            favor_sources: 2,
            strong_avoid_sources: 0,
            supporting_avoid_sources: 0,
        });

        assert_eq!(bucket, RecommendationBucket::CoThe);
    }
}
