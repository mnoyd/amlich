use amlich_core::{
    calculate_day_snapshot,
    reasoning::build_initiation_opening_decision,
};

#[test]
fn initiation_opening_decision_includes_ranked_hours_and_directions() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert!(!decision.suggested_hours.is_empty());
    assert!(!decision.suggested_directions.is_empty());
}

#[test]
fn refinement_narrows_scope_without_replacing_top_level_decision() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert!(!decision.primary_conclusion.is_empty());
    assert!(
        !decision.suggested_hours.is_empty() || !decision.suggested_directions.is_empty()
    );
    assert!(
        decision
            .suggested_hours
            .iter()
            .chain(decision.suggested_directions.iter())
            .all(|refinement| refinement.contains("Nếu vẫn tiến hành"))
    );
}
