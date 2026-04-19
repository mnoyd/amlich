use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceSource {
    Snapshot,
    Interaction,
    Bazi,
    AlmanacRule,
    Insight,
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProvenanceEntry {
    pub source: ProvenanceSource,
    pub source_id: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl ProvenanceEntry {
    pub fn new(source: ProvenanceSource, source_id: impl Into<String>, method: impl Into<String>) -> Self {
        Self {
            source,
            source_id: source_id.into(),
            method: method.into(),
            profile: None,
            note: None,
        }
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }

    pub fn with_profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }

    pub fn snapshot(source_id: impl Into<String>, method: impl Into<String>) -> Self {
        Self::new(ProvenanceSource::Snapshot, source_id, method)
    }

    pub fn interaction(source_id: impl Into<String>, method: impl Into<String>) -> Self {
        Self::new(ProvenanceSource::Interaction, source_id, method)
    }

    pub fn bazi(source_id: impl Into<String>, method: impl Into<String>) -> Self {
        Self::new(ProvenanceSource::Bazi, source_id, method)
    }

    pub fn almanac_rule(source_id: impl Into<String>, method: impl Into<String>) -> Self {
        Self::new(ProvenanceSource::AlmanacRule, source_id, method)
    }

    pub fn insight(source_id: impl Into<String>, method: impl Into<String>) -> Self {
        Self::new(ProvenanceSource::Insight, source_id, method)
    }

    pub fn derived(source_id: impl Into<String>, method: impl Into<String>) -> Self {
        Self::new(ProvenanceSource::Derived, source_id, method)
    }

    pub fn from_rule_evidence(
        source: ProvenanceSource,
        evidence: &crate::almanac::types::RuleEvidence,
    ) -> Self {
        Self::new(
            source,
            evidence.source_id.clone(),
            evidence.method.clone(),
        ).with_profile(evidence.profile.clone())
    }

    pub fn from_rule_evidence_opt(
        source: ProvenanceSource,
        evidence: &Option<crate::almanac::types::RuleEvidence>,
    ) -> Option<Self> {
        evidence.as_ref().map(|e| Self::from_rule_evidence(source, e))
    }

    pub fn from_source_meta(
        source: ProvenanceSource,
        meta: &crate::almanac::types::SourceMeta,
    ) -> Self {
        Self::new(source, meta.source_id.clone(), meta.method.clone())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceTracker {
    entries: HashMap<String, Vec<ProvenanceEntry>>,
}

impl ProvenanceTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn track(&mut self, node_id: &str, entry: ProvenanceEntry) {
        self.entries
            .entry(node_id.to_string())
            .or_default()
            .push(entry);
    }

    pub fn get(&self, node_id: &str) -> Option<&Vec<ProvenanceEntry>> {
        self.entries.get(node_id)
    }

    pub fn contains(&self, node_id: &str) -> bool {
        self.entries.contains_key(node_id)
    }

    pub fn merge(&mut self, other: ProvenanceTracker) {
        for (node_id, mut entries) in other.entries {
            self.entries
                .entry(node_id)
                .or_default()
                .append(&mut entries);
        }
    }
}
