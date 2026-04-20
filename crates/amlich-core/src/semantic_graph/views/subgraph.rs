use crate::semantic_graph::SemanticGraph;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubgraphView {
    pub root_ids: Vec<String>,
    pub node_ids: Vec<String>,
    pub edge_ids: Vec<String>,
}

impl SubgraphView {
    pub fn extract(graph: &SemanticGraph, root_ids: &[&str], depth: usize) -> Self {
        let mut visited_nodes: Vec<String> = Vec::new();
        let mut visited_edges: Vec<String> = Vec::new();
        let mut queue: Vec<(String, usize)> =
            root_ids.iter().map(|&id| (id.to_string(), 0)).collect();

        while let Some((node_id, current_depth)) = queue.pop() {
            if visited_nodes.contains(&node_id) {
                continue;
            }
            if current_depth > depth {
                continue;
            }
            if !graph.nodes().contains_key(&node_id) {
                continue;
            }
            visited_nodes.push(node_id.clone());

            for edge in graph.outgoing_edges(&node_id) {
                if !visited_edges.contains(&edge.edge_id) {
                    visited_edges.push(edge.edge_id.clone());
                    if !visited_nodes.contains(&edge.to_node_id) {
                        queue.push((edge.to_node_id.clone(), current_depth + 1));
                    }
                }
            }

            for edge in graph.incoming_edges(&node_id) {
                if !visited_edges.contains(&edge.edge_id) {
                    visited_edges.push(edge.edge_id.clone());
                    if !visited_nodes.contains(&edge.from_node_id) {
                        queue.push((edge.from_node_id.clone(), current_depth + 1));
                    }
                }
            }
        }

        let root_ids: Vec<String> = root_ids.iter().map(|&s| s.to_string()).collect();

        Self {
            root_ids,
            node_ids: visited_nodes,
            edge_ids: visited_edges,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.node_ids.is_empty()
    }

    pub fn node_count(&self) -> usize {
        self.node_ids.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_ids.len()
    }
}
