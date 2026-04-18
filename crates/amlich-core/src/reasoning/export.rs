use super::{
    EdgeEffect, InterpretedAxis, ReasoningEdge, ReasoningEdgeExport, ReasoningGraph,
    ReasoningGraphExport, ReasoningNode, ReasoningNodeExport, ReasoningNodeSeverity,
};

pub fn export_reasoning_graph(graph: &ReasoningGraph) -> ReasoningGraphExport {
    ReasoningGraphExport {
        action_id: graph.action_id,
        nodes: graph.nodes.iter().map(export_node).collect(),
        edges: graph.edges.iter().map(export_edge).collect(),
    }
}

fn export_node(node: &ReasoningNode) -> ReasoningNodeExport {
    ReasoningNodeExport {
        id: node.id.clone(),
        kind: node.kind,
        axis: axis_for_node(node.id.as_str()),
        severity: severity_for_node(node),
        tags: tags_for_node(node),
        summary_vi: node.summary_vi.clone(),
        evidence: node.evidence.clone(),
    }
}

fn export_edge(edge: &ReasoningEdge) -> ReasoningEdgeExport {
    ReasoningEdgeExport {
        from_node_id: edge.from_node_id.clone(),
        to_node_id: edge.to_node_id.clone(),
        effect: edge.effect,
        weight: effect_weight(edge.effect),
        justification: edge.justification,
        evidence: edge.evidence.clone(),
        tags: tags_for_edge(edge),
    }
}

fn axis_for_node(node_id: &str) -> Option<InterpretedAxis> {
    match node_id {
        "signal.support" => Some(InterpretedAxis::Support),
        "signal.resistance" => Some(InterpretedAxis::Resistance),
        "signal.stability" => Some(InterpretedAxis::Stability),
        "signal.personal_alignment" => Some(InterpretedAxis::PersonalAlignment),
        "signal.timing_fit" => Some(InterpretedAxis::TimingFit),
        "signal.context_clarity" => Some(InterpretedAxis::ContextClarity),
        _ => None,
    }
}

fn severity_for_node(node: &ReasoningNode) -> Option<ReasoningNodeSeverity> {
    match node.id.as_str() {
        "fact.day.truc" => match node.severity.as_deref() {
            Some("cat") => Some(ReasoningNodeSeverity::Auspicious),
            Some("hung") => Some(ReasoningNodeSeverity::Inauspicious),
            _ => None,
        },
        "fact.day.day_deity" => match node.severity.as_deref() {
            Some("hoang_dao") => Some(ReasoningNodeSeverity::HoangDao),
            Some("hac_dao") => Some(ReasoningNodeSeverity::HacDao),
            _ => None,
        },
        "fact.day.taboos" => match node.severity.as_deref() {
            Some("hard") => Some(ReasoningNodeSeverity::HardTaboo),
            Some("soft") => Some(ReasoningNodeSeverity::SoftTaboo),
            _ => None,
        },
        "fact.day.nhi_thap_bat_tu" => {
            if node.summary_vi.contains("cát tinh") {
                Some(ReasoningNodeSeverity::Auspicious)
            } else if node.summary_vi.contains("sát tinh") {
                Some(ReasoningNodeSeverity::Inauspicious)
            } else {
                None
            }
        }
        "fact.day.hoang_dao_hours" => node
            .severity
            .as_deref()
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|&count| count > 0)
            .map(|_| ReasoningNodeSeverity::Auspicious),
        "fact.day.xung_hop" => {
            if node.summary_vi.starts_with("Xung") && !node.summary_vi.contains(", hợp ") {
                Some(ReasoningNodeSeverity::Inauspicious)
            } else {
                None
            }
        }
        _ => None,
    }
}

fn tags_for_node(node: &ReasoningNode) -> Vec<String> {
    let mut tags = Vec::new();
    if node.id.starts_with("fact.personal.") {
        tags.push("personal".to_string());
    }
    if node.id.starts_with("fact.day.") {
        tags.push("day".to_string());
    }
    if node.id.starts_with("signal.") {
        tags.push("signal".to_string());
    }
    match node.id.as_str() {
        "fact.day.taboos" | "fact.day.xung_hop" | "signal.resistance" => {
            tags.push("resistance".to_string());
        }
        "fact.day.truc"
        | "fact.day.day_deity"
        | "fact.day.nhi_thap_bat_tu"
        | "fact.day.hoang_dao_hours"
        | "signal.support" => {
            tags.push("support".to_string());
        }
        "fact.day.travel_directions" | "signal.timing_fit" => {
            tags.push("timing".to_string());
        }
        "fact.day.solar_term" => {
            tags.push("context".to_string());
        }
        _ => {}
    }
    tags
}

fn tags_for_edge(edge: &ReasoningEdge) -> Vec<String> {
    let mut tags = Vec::new();
    if edge.effect.is_override() {
        tags.push("override".to_string());
    }
    if matches!(edge.effect, EdgeEffect::ConflictsWith) {
        tags.push("conflict".to_string());
    }
    if edge.to_node_id == InterpretedAxis::Support.signal_node_id() {
        tags.push("support".to_string());
    }
    if edge.to_node_id == InterpretedAxis::Resistance.signal_node_id() {
        tags.push("resistance".to_string());
    }
    if edge.to_node_id == InterpretedAxis::ContextClarity.signal_node_id() {
        tags.push("context".to_string());
    }
    tags
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

