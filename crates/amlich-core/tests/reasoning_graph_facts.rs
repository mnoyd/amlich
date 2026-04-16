use amlich_core::{
    calculate_day_snapshot,
    reasoning::{build_fact_graph, ActionId},
};

#[test]
fn build_fact_graph_maps_existing_day_snapshot_fields_to_fact_nodes() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let graph = build_fact_graph(ActionId::InitiationOpening, &snapshot, None).expect("fact graph");

    assert!(graph.nodes.iter().any(|n| n.id == "fact.day.truc"));
    assert!(graph.nodes.iter().any(|n| n.id == "fact.day.day_deity"));
    assert!(graph.nodes.iter().any(|n| n.id == "fact.day.taboos"));
    assert!(graph
        .nodes
        .iter()
        .any(|n| n.id == "fact.day.travel_directions"));
    assert!(graph
        .nodes
        .iter()
        .any(|n| n.id == "fact.day.hoang_dao_hours"));
    assert!(!graph
        .nodes
        .iter()
        .any(|n| n.id.starts_with("fact.legacy.")));
}

#[test]
fn fact_graph_keeps_hard_stop_taboo_metadata() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let graph = build_fact_graph(ActionId::InitiationOpening, &snapshot, None).expect("fact graph");

    let taboo = graph
        .nodes
        .iter()
        .find(|n| n.id == "fact.day.taboos")
        .expect("taboo node");
    assert!(taboo.summary_vi.contains("kiêng") || taboo.summary_vi.contains("kỵ"));
}
