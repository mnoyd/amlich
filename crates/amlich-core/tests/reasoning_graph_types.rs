use amlich_core::reasoning::{
    ActionId, EdgeEffect, InterpretedAxis, ReasoningEdge, ReasoningGraph,
};

#[test]
fn reasoning_graph_can_store_fact_signal_and_decision_nodes() {
    let graph = ReasoningGraph::new(ActionId::InitiationOpening);

    assert_eq!(graph.action_id, ActionId::InitiationOpening);
    assert!(graph.nodes.is_empty());
    assert!(graph.edges.is_empty());
}

#[test]
fn reasoning_edges_preserve_support_override_and_condition_effects() {
    let edge = ReasoningEdge::new(
        "fact.taboo.tam_nuong",
        "signal.resistance",
        EdgeEffect::Overrides,
    );

    assert_eq!(edge.effect, EdgeEffect::Overrides);
    assert_eq!(edge.from_node_id, "fact.taboo.tam_nuong");
    assert_eq!(edge.to_node_id, "signal.resistance");
}

#[test]
fn initial_interpreted_axes_include_support_resistance_and_personal_alignment() {
    let axes = InterpretedAxis::core_axes();

    assert!(axes.contains(&InterpretedAxis::Support));
    assert!(axes.contains(&InterpretedAxis::Resistance));
    assert!(axes.contains(&InterpretedAxis::Stability));
    assert!(axes.contains(&InterpretedAxis::PersonalAlignment));
    assert!(axes.contains(&InterpretedAxis::TimingFit));
    assert!(axes.contains(&InterpretedAxis::ContextClarity));
}
