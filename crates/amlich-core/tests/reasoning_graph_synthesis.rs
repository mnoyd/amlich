use amlich_core::{
    calculate_day_snapshot,
    reasoning::{build_initiation_opening_decision, RecommendationBucket},
};

#[test]
fn decision_payload_includes_primary_conclusion_supports_resistances_and_confidence() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert!(!decision.primary_conclusion.is_empty());
    assert!(!decision.strongest_supports.is_empty() || !decision.strongest_resistances.is_empty());
    assert!(
        !decision.conflict_notes.is_empty()
            || !decision.override_factors.is_empty()
            || decision.context_is_clear
    );
}

#[test]
fn avoid_bucket_mentions_override_reason_explicitly() {
    let snapshot = calculate_day_snapshot(3, 1, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert_eq!(decision.recommendation_bucket, RecommendationBucket::Avoid);
    assert!(!decision.override_factors.is_empty());
    assert!(
        decision.primary_conclusion.contains("cấm")
            || decision.primary_conclusion.contains("kỵ")
            || decision.primary_conclusion.contains("Không nên")
    );
}

#[test]
fn canonical_avoid_bucket_preserves_remaining_context() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert_eq!(decision.recommendation_bucket, RecommendationBucket::Avoid);
    assert!(!decision.context_is_clear || !decision.conflict_notes.is_empty());
    assert!(
        decision.primary_conclusion.contains("thận trọng")
            || decision.primary_conclusion.contains("trái chiều")
            || decision.primary_conclusion.contains("bối cảnh")
            || decision.primary_conclusion.contains("Không nên")
    );
}

#[test]
fn decision_payload_can_preserve_mixed_conclusion_when_signals_conflict() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert!(!decision.primary_conclusion.is_empty());
    assert!(
        !decision.context_is_clear
            || !decision.conflict_notes.is_empty()
            || matches!(
                decision.recommendation_bucket.as_str(),
                "mixed" | "cautious"
            )
    );
}
