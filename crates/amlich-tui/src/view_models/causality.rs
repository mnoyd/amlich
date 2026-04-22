use amlich_core::DebugSemanticGraphInspection;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CausalityNode {
    pub node_id: String,
    pub label: String,
    pub cluster: String,
    pub semantic_kind: String,
    pub severity: Option<String>,
    pub incoming: Vec<CausalityEdge>,
    pub outgoing: Vec<CausalityEdge>,
}

#[derive(Debug, Clone)]
pub struct CausalityEdge {
    pub neighbor_id: String,
    pub neighbor_label: String,
    pub neighbor_kind: String,
    pub edge_label: String,
}

pub fn extract_causality_tree(inspection: &DebugSemanticGraphInspection) -> Vec<CausalityNode> {
    let mut focal_nodes = Vec::new();
    let focal_kinds = ["star", "deity", "taboo", "hoang_dao_hour", "xung_hop", "truc", "day_deity"];

    let nodes_map: HashMap<_, _> = inspection
        .visualization
        .nodes
        .iter()
        .map(|n| (n.node_id.clone(), n))
        .collect();

    for node in &inspection.visualization.nodes {
        if focal_kinds.contains(&node.semantic_kind.as_str()) {
            let mut incoming = Vec::new();
            let mut outgoing = Vec::new();

            for edge in &inspection.visualization.edges {
                if edge.to_id == node.node_id {
                    if let Some(neighbor) = nodes_map.get(&edge.from_id) {
                        incoming.push(CausalityEdge {
                            neighbor_id: neighbor.node_id.clone(),
                            neighbor_label: neighbor.label.clone(),
                            neighbor_kind: neighbor.semantic_kind.clone(),
                            edge_label: edge.label.clone(),
                        });
                    }
                } else if edge.from_id == node.node_id {
                    if let Some(neighbor) = nodes_map.get(&edge.to_id) {
                        outgoing.push(CausalityEdge {
                            neighbor_id: neighbor.node_id.clone(),
                            neighbor_label: neighbor.label.clone(),
                            neighbor_kind: neighbor.semantic_kind.clone(),
                            edge_label: edge.label.clone(),
                        });
                    }
                }
            }

            focal_nodes.push(CausalityNode {
                node_id: node.node_id.clone(),
                label: node.label.clone(),
                cluster: node.cluster.clone(),
                semantic_kind: node.semantic_kind.clone(),
                severity: node.severity.clone(),
                incoming,
                outgoing,
            });
        }
    }

    focal_nodes.sort_by(|a, b| a.cluster.cmp(&b.cluster).then(a.label.cmp(&b.label)));
    focal_nodes
}
