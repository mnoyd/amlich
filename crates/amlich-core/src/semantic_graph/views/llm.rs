use super::subgraph::SubgraphView;
use crate::semantic_graph::SemanticGraph;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LlmGraphSlice {
    pub root_ids: Vec<String>,
    pub node_refs: Vec<String>,
    pub edge_refs: Vec<String>,
    pub summary_points: Vec<String>,
}

impl LlmGraphSlice {
    pub fn from_subgraph(graph: &SemanticGraph, view: &SubgraphView) -> Self {
        let node_refs = view.node_ids.clone();
        let edge_refs = view.edge_ids.clone();

        let mut summary_points = Vec::new();

        for node_id in &view.node_ids {
            if let Some(node) = graph.get_node(node_id) {
                let mut parts = vec![format!(
                    "[{}] {}",
                    node.concept.label().as_str(),
                    node.summary_vi
                )];
                if let Some(sev) = &node.severity {
                    parts.push(format!("severity={}", sev));
                }
                if !node.provenance.is_empty() {
                    let sources: Vec<_> = node
                        .provenance
                        .iter()
                        .map(|p| format!("{:?}", p.source))
                        .collect();
                    parts.push(format!("sources={}", sources.join(",")));
                }
                summary_points.push(parts.join(" | "));
            }
        }

        Self {
            root_ids: view.root_ids.clone(),
            node_refs,
            edge_refs,
            summary_points,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.node_refs.is_empty()
    }

    pub fn node_count(&self) -> usize {
        self.node_refs.len()
    }
}
