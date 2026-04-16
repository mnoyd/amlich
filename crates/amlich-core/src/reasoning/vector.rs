use super::{ActionId, EdgeEffect, InitiationOpeningVector, InterpretedAxis, ReasoningGraph};

pub fn assemble_action_vector(graph: &ReasoningGraph) -> Result<InitiationOpeningVector, String> {
    match graph.action_id {
        ActionId::InitiationOpening => Ok(InitiationOpeningVector {
            support: axis_score(graph, InterpretedAxis::Support),
            resistance: axis_score(graph, InterpretedAxis::Resistance),
            stability: axis_score(graph, InterpretedAxis::Stability),
            personal_alignment: axis_score(graph, InterpretedAxis::PersonalAlignment),
            timing_fit: axis_score(graph, InterpretedAxis::TimingFit),
            context_clarity: axis_score(graph, InterpretedAxis::ContextClarity),
            strongest_support_id: strongest_edge_source(
                graph,
                InterpretedAxis::Support,
                is_support_effect,
            ),
            strongest_support_note: strongest_edge_note(
                graph,
                InterpretedAxis::Support,
                is_support_effect,
            ),
            strongest_resistance_id: strongest_edge_source(
                graph,
                InterpretedAxis::Resistance,
                is_resistance_effect,
            ),
            strongest_resistance_note: strongest_edge_note(
                graph,
                InterpretedAxis::Resistance,
                is_resistance_effect,
            ),
        }),
    }
}

fn axis_score(graph: &ReasoningGraph, axis: InterpretedAxis) -> f32 {
    graph
        .edges
        .iter()
        .filter(|edge| edge.to_node_id == axis.signal_node_id())
        .map(|edge| effect_weight(edge.effect))
        .sum::<i32>()
        .max(0) as f32
}

fn strongest_edge_source(
    graph: &ReasoningGraph,
    axis: InterpretedAxis,
    predicate: impl Fn(EdgeEffect) -> bool,
) -> Option<String> {
    strongest_edge(graph, axis, predicate).map(|(edge, _)| edge.from_node_id.clone())
}

fn strongest_edge_note(
    graph: &ReasoningGraph,
    axis: InterpretedAxis,
    predicate: impl Fn(EdgeEffect) -> bool,
) -> Option<String> {
    let (edge, _) = strongest_edge(graph, axis, predicate)?;
    graph
        .nodes
        .iter()
        .find(|node| node.id == edge.from_node_id)
        .map(|node| node.summary_vi.clone())
        .or_else(|| Some(edge.from_node_id.clone()))
}

fn strongest_edge(
    graph: &ReasoningGraph,
    axis: InterpretedAxis,
    predicate: impl Fn(EdgeEffect) -> bool,
) -> Option<(&super::ReasoningEdge, i32)> {
    graph
        .edges
        .iter()
        .filter(|edge| edge.to_node_id == axis.signal_node_id())
        .filter(|edge| predicate(edge.effect))
        .map(|edge| (edge, effect_weight(edge.effect)))
        .max_by_key(|(_, weight)| *weight)
}

fn is_support_effect(effect: EdgeEffect) -> bool {
    matches!(effect, EdgeEffect::Supports | EdgeEffect::Overrides)
}

fn is_resistance_effect(effect: EdgeEffect) -> bool {
    matches!(effect, EdgeEffect::Supports | EdgeEffect::Overrides)
}

fn effect_weight(effect: EdgeEffect) -> i32 {
    match effect {
        EdgeEffect::Supports => 1,
        EdgeEffect::Weakens => 1,
        EdgeEffect::Overrides => 2,
        EdgeEffect::ConflictsWith => 1,
        EdgeEffect::Conditions => 0,
    }
}
