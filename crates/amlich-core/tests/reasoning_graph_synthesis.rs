use amlich_core::{calculate_day_snapshot, reasoning::build_initiation_opening_decision};

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
