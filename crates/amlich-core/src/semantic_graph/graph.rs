use super::edge::SemanticEdge;
use super::node::SemanticNode;
use super::provenance::{ProvenanceEntry, ProvenanceTracker};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticGraph {
    nodes: HashMap<String, SemanticNode>,
    edges: HashMap<String, SemanticEdge>,
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
        if self.nodes.contains_key(&edge.from_node_id) && self.nodes.contains_key(&edge.to_node_id)
        {
            self.edges.insert(edge.edge_id.clone(), edge);
        }
    }

    pub fn get_node(&self, node_id: &str) -> Option<&SemanticNode> {
        self.nodes.get(node_id)
    }

    pub fn has_node(&self, node_id: &str) -> bool {
        self.nodes.contains_key(node_id)
    }

    pub fn get_edge(&self, edge_id: &str) -> Option<&SemanticEdge> {
        self.edges.get(edge_id)
    }

    pub fn has_edge(&self, edge_id: &str) -> bool {
        self.edges.contains_key(edge_id)
    }

    pub fn nodes(&self) -> &HashMap<String, SemanticNode> {
        &self.nodes
    }

    pub fn edges(&self) -> &HashMap<String, SemanticEdge> {
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
            .values()
            .filter(|e| e.from_node_id == node_id)
            .collect()
    }

    pub fn incoming_edges(&self, node_id: &str) -> Vec<&SemanticEdge> {
        self.edges
            .values()
            .filter(|e| e.to_node_id == node_id)
            .collect()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn merge(&mut self, other: SemanticGraph) -> Result<(), GraphMergeError> {
        for (_, node) in other.nodes {
            if let Some(existing) = self.nodes.get(&node.node_id) {
                if existing.concept != node.concept {
                    return Err(GraphMergeError::NodeKindConflict {
                        node_id: node.node_id.clone(),
                        existing_kind: existing.concept,
                        incoming_kind: node.concept,
                    });
                }
                let mut merged = existing.clone();
                for prov in node.provenance {
                    if !merged.provenance.contains(&prov) {
                        merged.provenance.push(prov);
                    }
                }
                self.nodes.insert(node.node_id.clone(), merged);
            } else {
                self.nodes.insert(node.node_id.clone(), node);
            }
        }
        for (_, edge) in other.edges {
            if let Some(existing) = self.edges.get(&edge.edge_id) {
                if existing.label.concept != edge.label.concept {
                    return Err(GraphMergeError::EdgePayloadConflict {
                        edge_id: edge.edge_id.clone(),
                    });
                }
                if existing.from_node_id != edge.from_node_id
                    || existing.to_node_id != edge.to_node_id
                {
                    return Err(GraphMergeError::EdgePayloadConflict {
                        edge_id: edge.edge_id.clone(),
                    });
                }
                let mut merged = existing.clone();
                for prov in edge.provenance {
                    if !merged.provenance.contains(&prov) {
                        merged.provenance.push(prov);
                    }
                }
                self.edges.insert(edge.edge_id.clone(), merged);
            } else {
                if self.nodes.contains_key(&edge.from_node_id)
                    && self.nodes.contains_key(&edge.to_node_id)
                {
                    self.edges.insert(edge.edge_id.clone(), edge);
                }
            }
        }
        self.provenance.merge(other.provenance);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), GraphValidationError> {
        let node_ids: HashSet<_> = self.nodes.keys().collect();
        for edge in self.edges.values() {
            if !node_ids.contains(&edge.from_node_id) {
                return Err(GraphValidationError::MissingSourceNode(
                    edge.from_node_id.clone(),
                ));
            }
            if !node_ids.contains(&edge.to_node_id) {
                return Err(GraphValidationError::MissingTargetNode(
                    edge.to_node_id.clone(),
                ));
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphMergeError {
    NodeKindConflict {
        node_id: String,
        existing_kind: super::ontology::NodeConcept,
        incoming_kind: super::ontology::NodeConcept,
    },
    EdgePayloadConflict {
        edge_id: String,
    },
}
