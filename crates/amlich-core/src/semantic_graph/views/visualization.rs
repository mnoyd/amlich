use serde::{Deserialize, Serialize};
use crate::semantic_graph::SemanticGraph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationNode {
    pub node_id: String,
    pub label: String,
    pub cluster: String,
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationEdge {
    pub edge_id: String,
    pub from_id: String,
    pub to_id: String,
    pub label: String,
    pub weight: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualizationGraph {
    pub nodes: Vec<VisualizationNode>,
    pub edges: Vec<VisualizationEdge>,
}

impl VisualizationGraph {
    pub fn from_semantic_graph(graph: &SemanticGraph) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for (_, node) in graph.nodes() {
            let cluster = cluster_for_node(&node.node_id);
            nodes.push(VisualizationNode {
                node_id: node.node_id.clone(),
                label: node.summary_vi.clone(),
                cluster,
                severity: node.severity.clone(),
            });
        }

        for (_, edge) in graph.edges() {
            edges.push(VisualizationEdge {
                edge_id: edge.edge_id.clone(),
                from_id: edge.from_node_id.clone(),
                to_id: edge.to_node_id.clone(),
                label: edge.label.concept.label().as_str().to_string(),
                weight: edge.label.weight,
            });
        }

        Self { nodes, edges }
    }
}

fn cluster_for_node(node_id: &str) -> String {
    if node_id.starts_with("bazi_profile:") || node_id.starts_with("pillar:") || node_id.starts_with("element_distribution:") {
        "bazi-core".to_string()
    } else if node_id.starts_with("day:") || node_id.starts_with("solar_term:") || node_id.starts_with("truc:") || node_id.contains(":day:") {
        "day-core".to_string()
    } else {
        "misc".to_string()
    }
}
