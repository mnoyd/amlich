use serde::{Deserialize, Serialize};
use super::ids::SemanticId;
use super::ontology::NodeConcept;
use super::provenance::ProvenanceEntry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeOrigin {
    Fact,
    Interpreted,
    Decision,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticNode {
    pub id: SemanticId,
    pub node_id: String,
    pub concept: NodeConcept,
    pub origin: NodeOrigin,
    pub summary_vi: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance: Vec<ProvenanceEntry>,
}

impl SemanticNode {
    pub fn new(
        id: SemanticId,
        concept: NodeConcept,
        origin: NodeOrigin,
        summary_vi: impl Into<String>,
    ) -> Self {
        let node_id = id.to_node_id();
        Self {
            id,
            node_id,
            concept,
            origin,
            summary_vi: summary_vi.into(),
            severity: None,
            tags: Vec::new(),
            provenance: Vec::new(),
        }
    }

    pub fn with_severity(mut self, severity: impl Into<String>) -> Self {
        self.severity = Some(severity.into());
        self
    }

    pub fn with_tags(mut self, tags: impl Into<Vec<String>>) -> Self {
        self.tags = tags.into();
        self
    }

    pub fn with_provenance(mut self, entry: ProvenanceEntry) -> Self {
        self.provenance.push(entry);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticNodeId(pub String);

impl From<&SemanticNode> for SemanticNodeId {
    fn from(node: &SemanticNode) -> Self {
        SemanticNodeId(node.node_id.clone())
    }
}

impl std::fmt::Display for SemanticNodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
