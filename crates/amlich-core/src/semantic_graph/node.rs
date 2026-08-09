use super::ids::SemanticId;
use super::ontology::NodeConcept;
use super::provenance::ProvenanceEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeOrigin {
    Fact,
    Interpreted,
    Decision,
}

/// Typed decision facts carried by semantic nodes.
///
/// `summary_vi` is presentation-only. Consumers that make decisions must
/// read this field (or another typed domain field), never parse localized
/// prose or ad-hoc tag strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum SemanticFact {
    Truc {
        opening_avoid_count: u8,
        opening_favorable: bool,
    },
    Star {
        polarity: SemanticPolarity,
    },
    XungHop {
        has_clash: bool,
        has_harmony: bool,
    },
    Direction {
        net_score: i8,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticPolarity {
    Favorable,
    Unfavorable,
    Mixed,
    Neutral,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fact: Option<SemanticFact>,
    /// Optional generic JSON payload (Phase 19, INT-08 SC#2 literal interpretation).
    /// For `NodeConcept::Ritual` aggregate nodes, this carries
    /// `{"offering_refs": [...], "offerings": [...]}` derived from the matching
    /// ritual entries' `RitualEntry::offerings`. Other concepts may use this
    /// field for concept-specific structured data. Absent in JSON when None.
    ///
    /// NOTE: kept as `serde_json::Value` (NOT a typed enum) per 19-RESEARCH.md
    /// Option B. This is the lightweight generic payload mechanism — it avoids
    /// the cost of a typed `RitualNodePayload` enum (Option C) and matches the
    /// `v1.5` discipline of "additive `Option<T>` fields on existing structs".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
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
            fact: None,
            payload: None,
        }
    }

    pub fn with_severity(mut self, severity: impl Into<String>) -> Self {
        self.severity = Some(severity.into());
        self
    }

    /// Conditionally attach a non-numeric severity classification when
    /// `condition` holds. Used to mark favorability without overloading the
    /// severity slot with a numeric count (amlich-0q2f).
    pub fn with_severity_if(self, condition: bool, severity: &str) -> Self {
        if condition {
            self.with_severity(severity)
        } else {
            self
        }
    }

    pub fn with_tags(mut self, tags: impl Into<Vec<String>>) -> Self {
        self.tags = tags.into();
        self
    }

    pub fn with_provenance(mut self, entry: ProvenanceEntry) -> Self {
        self.provenance.push(entry);
        self
    }

    pub fn with_fact(mut self, fact: SemanticFact) -> Self {
        self.fact = Some(fact);
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
