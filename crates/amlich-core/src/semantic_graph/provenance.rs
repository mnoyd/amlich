use crate::reasoning::ReasoningEvidenceEnvelope;

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
    /// Phase 24 (ICH-05): I Ching variant for Hexagram nodes + IChing
    /// composite cross-link. Constructed-only (never matched on the public
    /// graph surface); reasoning consumers read this via
    /// [`to_reasoning_evidence`] which maps the variant to
    /// [`Family::IChing`]. Matches the Phase 20-03 ActionId::IChing
    /// addition discipline — extending the enum is an additive-safe
    /// change.
    IChing,
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
    pub fn new(
        source: ProvenanceSource,
        source_id: impl Into<String>,
        method: impl Into<String>,
    ) -> Self {
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

    /// Phase 24 (ICH-05): IChing variant helper mirroring
    /// `almanac_rule` / `derived` / `snapshot` / etc.
    pub fn iching(source_id: impl Into<String>, method: impl Into<String>) -> Self {
        Self::new(ProvenanceSource::IChing, source_id, method)
    }

    pub fn from_rule_evidence(
        source: ProvenanceSource,
        evidence: &crate::almanac::types::RuleEvidence,
    ) -> Self {
        Self::new(source, evidence.source_id.clone(), evidence.method.clone())
            .with_profile(evidence.profile.clone())
    }

    pub fn from_rule_evidence_opt(
        source: ProvenanceSource,
        evidence: &Option<crate::almanac::types::RuleEvidence>,
    ) -> Option<Self> {
        evidence
            .as_ref()
            .map(|e| Self::from_rule_evidence(source, e))
    }

    pub fn from_source_meta(
        source: ProvenanceSource,
        meta: &crate::almanac::types::SourceMeta,
    ) -> Self {
        Self::new(source, meta.source_id.clone(), meta.method.clone())
    }

    pub fn to_reasoning_evidence(&self) -> ReasoningEvidenceEnvelope {
        use crate::reasoning::ReasoningEvidenceSourceFamily as Family;
        let source_family = match self.source {
            ProvenanceSource::Snapshot => Family::Snapshot,
            ProvenanceSource::Interaction => Family::Interaction,
            ProvenanceSource::Bazi => Family::Bazi,
            ProvenanceSource::AlmanacRule => Family::AlmanacRule,
            ProvenanceSource::Insight => Family::Insight,
            ProvenanceSource::Derived => Family::Derived,
            ProvenanceSource::IChing => Family::IChing,
        };
        ReasoningEvidenceEnvelope {
            source_family,
            source_id: self.source_id.clone(),
            method: self.method.clone(),
            note: self.note.clone(),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reasoning::ReasoningEvidenceSourceFamily as Family;

    /// Phase 24 (ICH-05): the new `ProvenanceSource::IChing` variant
    /// resolves to `ReasoningEvidenceSourceFamily::IChing` via
    /// `to_reasoning_evidence`.
    #[test]
    fn to_reasoning_evidence_maps_iching_to_iching_family() {
        let entry = ProvenanceEntry::iching("kinh-dich", "corpus_lookup")
            .with_note("Hexagram lookup for chu quẻ");
        let envelope = entry.to_reasoning_evidence();
        assert_eq!(
            envelope.source_family,
            Family::IChing,
            "ProvenanceSource::IChing must map to ReasoningEvidenceSourceFamily::IChing"
        );
        assert_eq!(envelope.source_id, "kinh-dich");
        assert_eq!(envelope.method, "corpus_lookup");
        assert_eq!(
            envelope.note.as_deref(),
            Some("Hexagram lookup for chu quẻ")
        );
    }

    /// Locked mapping for every existing variant — guards against silent
    /// drift if a future commit removes / reorders a match arm.
    #[test]
    fn to_reasoning_evidence_preserves_all_existing_match_arms() {
        // Each (ProvenanceSource, Family) pair below MUST remain
        // (constructor-and-rename-stable). If a future commit changes the
        // mapping, this test fails with a loud message naming the pair.
        let cases: Vec<(ProvenanceSource, Family, &str, &str)> = vec![
            (ProvenanceSource::Snapshot, Family::Snapshot, "snap", "compute_day_context"),
            (ProvenanceSource::Interaction, Family::Interaction, "interaction.x", "y"),
            (ProvenanceSource::Bazi, Family::Bazi, "bazi.x", "y"),
            (ProvenanceSource::AlmanacRule, Family::AlmanacRule, "khcbppt", "thai_tue"),
            (ProvenanceSource::Insight, Family::Insight, "insight.x", "y"),
            (ProvenanceSource::Derived, Family::Derived, "rule.composite.direction_cross_link", "v17.read_only_join"),
            (ProvenanceSource::IChing, Family::IChing, "kinh-dich", "corpus_lookup"),
        ];
        for (source, family, source_id, method) in cases {
            let entry = ProvenanceEntry::new(source, source_id, method);
            let envelope = entry.to_reasoning_evidence();
            assert_eq!(
                envelope.source_family, family,
                "mapping for {source:?} -> {family:?} must be preserved"
            );
            assert_eq!(envelope.source_id, source_id);
            assert_eq!(envelope.method, method);
        }
    }
}
