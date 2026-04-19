use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use super::edge::SemanticEdge;
use super::node::SemanticNode;
use super::provenance::{ProvenanceEntry, ProvenanceTracker};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticGraph {
    nodes: HashMap<String, SemanticNode>,
    edges: Vec<SemanticEdge>,
    provenance: ProvenanceTracker,
}

impl SemanticGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: SemanticNode) {
        self.nodes.insert(node.node_id.clone(), node);
    }

    pub fn add_edge(&mut self, edge: SemanticEdge) {
        if self.nodes.contains_key(&edge.from_node_id) && self.nodes.contains_key(&edge.to_node_id) {
            self.edges.push(edge);
        }
    }

    pub fn get_node(&self, node_id: &str) -> Option<&SemanticNode> {
        self.nodes.get(node_id)
    }

    pub fn has_node(&self, node_id: &str) -> bool {
        self.nodes.contains_key(node_id)
    }

    pub fn nodes(&self) -> &HashMap<String, SemanticNode> {
        &self.nodes
    }

    pub fn edges(&self) -> &Vec<SemanticEdge> {
        &self.edges
    }

    pub fn provenance(&self) -> &ProvenanceTracker {
        &self.provenance
    }

    pub fn track_provenance(&mut self, node_id: &str, entry: ProvenanceEntry) {
        self.provenance.track(node_id, entry);
    }

    pub fn outgoing_edges(&self, node_id: &str) -> Vec<&SemanticEdge> {
        self.edges
            .iter()
            .filter(|e| e.from_node_id == node_id)
            .collect()
    }

    pub fn incoming_edges(&self, node_id: &str) -> Vec<&SemanticEdge> {
        self.edges
            .iter()
            .filter(|e| e.to_node_id == node_id)
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn merge(&mut self, other: SemanticGraph) {
        for (_, node) in other.nodes {
            if !self.nodes.contains_key(&node.node_id) {
                self.nodes.insert(node.node_id.clone(), node);
            }
        }
        for edge in other.edges {
            if self.nodes.contains_key(&edge.from_node_id) && self.nodes.contains_key(&edge.to_node_id) {
                self.edges.push(edge);
            }
        }
        self.provenance.merge(other.provenance);
    }

    pub fn validate(&self) -> Result<(), GraphValidationError> {
        let node_ids: HashSet<_> = self.nodes.keys().collect();
        for edge in &self.edges {
            if !node_ids.contains(&edge.from_node_id) {
                return Err(GraphValidationError::MissingSourceNode(edge.from_node_id.clone()));
            }
            if !node_ids.contains(&edge.to_node_id) {
                return Err(GraphValidationError::MissingTargetNode(edge.to_node_id.clone()));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphValidationError {
    MissingSourceNode(String),
    MissingTargetNode(String),
}
