use amlich_core::{
    calculate_day_snapshot,
    reasoning::{assemble_action_vector, build_fact_graph, derive_interpreted_signals, ActionId},
};

#[test]
fn initiation_opening_vector_captures_axis_scores_before_final_synthesis() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let graph = build_fact_graph(ActionId::InitiationOpening, &snapshot, None).expect("graph");
    let graph = derive_interpreted_signals(graph).expect("signals");
    let vector = assemble_action_vector(&graph).expect("vector");

    assert!(vector.support >= 0.0);
    assert!(vector.resistance >= 0.0);
    assert!(vector.timing_fit >= 0.0);
    assert!(vector.context_clarity >= 0.0);
}

#[test]
fn initiation_opening_vector_can_remain_mixed_before_any_final_verdict() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let graph = build_fact_graph(ActionId::InitiationOpening, &snapshot, None).expect("graph");
    let graph = derive_interpreted_signals(graph).expect("signals");
    let vector = assemble_action_vector(&graph).expect("vector");

    assert!(vector.support > 0.0);
    assert!(vector.resistance > 0.0);
}
