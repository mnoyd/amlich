use amlich_core::{
    build_initiation_opening_reasoning, build_initiation_opening_reasoning_bundle,
    calculate_day_snapshot,
};

#[test]
fn public_api_builds_initiation_opening_reasoning_from_snapshot() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let result = build_initiation_opening_reasoning(&snapshot, None).expect("reasoning result");

    assert!(!result.primary_conclusion.is_empty());
}

#[test]
fn public_api_builds_initiation_opening_reasoning_bundle_from_snapshot() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let result =
        build_initiation_opening_reasoning_bundle(&snapshot, None).expect("reasoning bundle");

    assert!(!result.decision.primary_conclusion.is_empty());
    assert!(!result.graph.nodes.is_empty());
}
