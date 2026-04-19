use serde::{Deserialize, Serialize};
use super::ontology::EdgeConcept;
use super::provenance::ProvenanceEntry;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticEdgeId(pub String);

impl From<&SemanticEdge> for SemanticEdgeId {
    fn from(edge: &SemanticEdge) -> Self {
        SemanticEdgeId(edge.edge_id.clone())
    }
}

impl std::fmt::Display for SemanticEdgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticEdgeLabel {
    pub concept: EdgeConcept,
    pub weight: i32,
}

impl SemanticEdgeLabel {
    pub fn new(concept: EdgeConcept) -> Self {
        Self {
            concept,
            weight: 1,
        }
    }

    pub fn with_weight(mut self, weight: i32) -> Self {
        self.weight = weight;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticEdge {
    pub edge_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub label: SemanticEdgeLabel,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub justification: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance: Vec<ProvenanceEntry>,
}

impl SemanticEdge {
    pub fn new(
        from: impl Into<String>,
        to: impl Into<String>,
        concept: EdgeConcept,
    ) -> Self {
        let from_node_id = from.into();
        let to_node_id = to.into();
        let edge_id = format!("{}->{}", from_node_id, to_node_id);
        Self {
            edge_id,
            from_node_id,
            to_node_id,
            label: SemanticEdgeLabel::new(concept),
            justification: Vec::new(),
            provenance: Vec::new(),
        }
    }

    pub fn edge_id(&self) -> &str {
        &self.edge_id
    }

    pub fn with_weight(mut self, weight: i32) -> Self {
        self.label.weight = weight;
        self
    }

    pub fn with_justification(mut self, justification: impl Into<String>) -> Self {
        self.justification.push(justification.into());
        self
    }

    pub fn with_provenance(mut self, entry: ProvenanceEntry) -> Self {
        self.provenance.push(entry);
        self
    }
}
