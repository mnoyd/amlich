use amlich_core::{
    calculate_day_snapshot,
    reasoning::{ActionId, build_fact_graph, derive_interpreted_signals},
};

#[test]
fn interpreted_signals_include_the_six_core_axes() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let graph = build_fact_graph(ActionId::InitiationOpening, &snapshot, None).expect("graph");
    let graph = derive_interpreted_signals(graph).expect("signals");

    assert!(graph.nodes.iter().any(|n| n.id == "signal.support"));
    assert!(graph.nodes.iter().any(|n| n.id == "signal.resistance"));
    assert!(graph.nodes.iter().any(|n| n.id == "signal.stability"));
    assert!(graph.nodes.iter().any(|n| n.id == "signal.personal_alignment"));
    assert!(graph.nodes.iter().any(|n| n.id == "signal.timing_fit"));
    assert!(graph.nodes.iter().any(|n| n.id == "signal.context_clarity"));
}

#[test]
fn hard_taboo_creates_override_pressure_on_resistance_and_clarity() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let graph = build_fact_graph(ActionId::InitiationOpening, &snapshot, None).expect("graph");
    let graph = derive_interpreted_signals(graph).expect("signals");

    assert!(graph.edges.iter().any(|e| e.effect.is_override()));
    assert!(graph.edges.iter().any(|e| e.to_node_id == "signal.context_clarity"));
}
