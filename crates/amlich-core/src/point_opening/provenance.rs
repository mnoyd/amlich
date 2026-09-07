//! Per-row provenance emission for the frozen Tý Ngọ Lưu Chú corpus
//! (bead `amlich-xlag.2.2.5`, ADR-0004).
//!
//! The corpus loader (`amlich-xlag.2.2.2`) already validates every
//! table row's `sources` block — work, juan, passage key, edition,
//! transcription, and cross-reference URIs — but until this bead the
//! validated evidence was discarded at load time. Here it is attached
//! to every emitted [`FrozenPointOpeningRecord`] as a
//! [`PointOpeningProvenance`] block and re-emitted as method evidence
//! ([`ProvenanceEntry`] / [`ReasoningEvidenceEnvelope`]).
//!
//! Source-separation contract (the bead's acceptance criteria):
//!
//! - **Method evidence** — the frozen point-opening lookup itself —
//!   always cites the reserved primitive source
//!   ([`SOURCE_TY_NGO_LUU_CHU`]) and never the v1.10 Tier-0
//!   `shi-er-jing-na-di-zhi` id (ADR-0003 reservation, discharged by
//!   ADR-0004).
//! - **Calendar-engine evidence** — the day pillar and the hour-branch
//!   slot — keeps its existing engine sources (see
//!   [`CALENDAR_ENGINE_SOURCE_ID`] and the `khcbppt`
//!   `hour-pillar-seed-table` rule), emitted separately on
//!   [`super::civil_time::LocalCivilPointOpening::calendar_evidence`]
//!   and never folded into the method evidence.
//!
//! Until the four human review gates sign (`amlich-xlag.2.5`–`.2.8`),
//! every emitted record stays `ExternalReviewPending` with disclaimer
//! v2, its safety class, and its applicable `TNLC-DIV-*` divergences
//! riding the [`super::state::PointOpeningContext`] — unchanged by
//! this bead.

use serde::{Deserialize, Serialize};

use crate::reasoning::ReasoningEvidenceEnvelope;
use crate::sources::SOURCE_TY_NGO_LUU_CHU;
use crate::ProvenanceEntry;

use super::corpus::FrozenPointOpeningRecord;
use super::state::PointOpeningSlotState;

/// Source id of Amlich's built-in civil-calendar engine (day-pillar
/// Can Chi over Julian day numbers), used by the v1.11 point-opening
/// calendar evidence. Mirrors the `amlich-solar-term-engine` precedent
/// from v1.10 (`traditional_wellness::seasonal`): the astronomical /
/// calendar engine keeps its own provenance and is never retagged as
/// a classical source.
pub const CALENDAR_ENGINE_SOURCE_ID: &str = "amlich-calendar-engine";

/// The method tag every point-opening lookup entry carries, prefixed
/// with the slot identity (e.g. `point_opening_lookup:甲/子`). Mirrors
/// the `branch_channel_lookup:{branch_vi}` convention from v1.10.
const METHOD_POINT_OPENING_LOOKUP: &str = "point_opening_lookup";

/// One per-row work citation, the validated `sources` entry of the
/// referenced frozen table row. Carries the full audit metadata
/// (work, juan, passage, edition, transcription, and cross-reference
/// URIs) that method-evidence notes summarize but do not duplicate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointOpeningSourceCitation {
    /// Always the reserved primitive source id
    /// ([`SOURCE_TY_NGO_LUU_CHU`]); asserted equal to the policy
    /// contract at load time.
    pub source_id: String,
    /// Bibliographic title (e.g. `"Zhenjiu Dacheng (針灸大成)"`).
    pub work_title: String,
    /// Volume / chapter (e.g. `"卷七"`).
    pub volume_or_chapter: String,
    /// Passage / table key (e.g. `"徐氏子午流注逐日按時定穴歌；流注圖"`).
    pub passage_key: String,
    /// Edition or facsimile URI consulted by the reviewer.
    /// `PENDING_CLASSICAL_REVIEW` until Gate 1 signs.
    pub edition_or_facsimile_uri: String,
    /// Public transcription URI of the consulted edition.
    pub transcription_uri: String,
    /// Cross-reference (collation) URI — e.g. the ctext Zhenjiu
    /// Daquan chapter reproducing the same Xu-style tables.
    pub cross_reference_uri: String,
    /// Translation kind; the frozen corpus value is
    /// `"verbatim_classical_table_with_project_paraphrase_gloss"`.
    pub translation_kind: String,
}

/// Table identity of the frozen row backing an open record: which day
/// table (`table_id`, e.g. `"jia"`) and which row (`row_index`, 1..=6)
/// the grid cell resolves to. Closed records carry no row identity —
/// their evidence is the explicit closed state itself.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointOpeningTableEvidence {
    /// The referenced day table's id (e.g. `"jia"`).
    pub table_id: String,
    /// The referenced row's 1-based index within the table (1..=6).
    pub row_index: usize,
}

/// The provenance block emitted with every frozen record: the
/// primitive method source id, the per-row work citations, and — for
/// open records — the backing table-row identity. Closed records keep
/// an empty `work_evidence` / absent `table_evidence` because their
/// frozen truth is the explicit closed state (`閉穴`), never a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointOpeningProvenance {
    /// The reserved primitive source id; validated equal to the
    /// `TY_NGO_LUU_CHU_POLICY_V1` contract's `source_id` at load.
    pub source_id: String,
    /// Per-row work citations from the referenced table row. Empty
    /// for closed records.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub work_evidence: Vec<PointOpeningSourceCitation>,
    /// Backing table-row identity. `None` for closed records.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub table_evidence: Option<PointOpeningTableEvidence>,
}

fn slot_label(record: &FrozenPointOpeningRecord) -> String {
    format!(
        "{}:{}/{}",
        METHOD_POINT_OPENING_LOOKUP, record.day_stem_zh, record.hour_branch_zh
    )
}

/// The closed-state note summarized into method evidence: neither
/// running day-table lists the hour block, so the slot stays closed
/// as printed (TNLC-DIV-01).
fn closed_note(state: &PointOpeningSlotState) -> String {
    match state {
        PointOpeningSlotState::Closed { note, .. } => {
            format!("closed slot (閉穴): {note}")
        }
        PointOpeningSlotState::Open { .. } => {
            unreachable!("closed_note is only called for closed records")
        }
    }
}

impl FrozenPointOpeningRecord {
    /// Method evidence as semantic-graph provenance entries: the
    /// frozen point-opening lookup itself, always citing the reserved
    /// primitive source — never the v1.10 Tier-0 id (bead
    /// `amlich-xlag.2.2.5`).
    ///
    /// Emits exactly one entry per per-row work citation for open
    /// records, and exactly one corpus-level entry for closed records
    /// (their frozen truth is the explicit closed state, which the
    /// note summarizes). The `source_id` is the canonical
    /// [`SOURCE_TY_NGO_LUU_CHU`] constant rather than the loaded
    /// citation's field — the loader has already asserted equality,
    /// and the static source-id discipline
    /// (`tests/source_id_guard.rs`) forbids bare string literals at
    /// call-sites.
    pub fn provenance_entries(&self) -> Vec<ProvenanceEntry> {
        let method = slot_label(self);
        if self.provenance.work_evidence.is_empty() {
            return vec![ProvenanceEntry::almanac_rule(SOURCE_TY_NGO_LUU_CHU, method)
                .with_note(closed_note(&self.context.state))];
        }
        self.provenance
            .work_evidence
            .iter()
            .map(|citation| {
                ProvenanceEntry::almanac_rule(SOURCE_TY_NGO_LUU_CHU, method.clone()).with_note(
                    format!(
                        "{} — {} ({})",
                        citation.work_title, citation.volume_or_chapter, citation.passage_key
                    ),
                )
            })
            .collect()
    }

    /// Method evidence as reasoning envelopes for the high-level
    /// evidence surface. Same shape and source-id discipline as
    /// [`provenance_entries`][Self::provenance_entries].
    pub fn reasoning_evidence(&self) -> Vec<ReasoningEvidenceEnvelope> {
        self.provenance_entries()
            .into_iter()
            .map(|entry| entry.to_reasoning_evidence())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point_opening::{
        PointOpeningContext, PointOpeningIdentity, PointOpeningProvenance, PointOpeningSlotState,
    };
    use crate::traditional_wellness::divergence::ExternalReviewState;

    fn sample_citation() -> PointOpeningSourceCitation {
        PointOpeningSourceCitation {
            source_id: SOURCE_TY_NGO_LUU_CHU.to_string(),
            work_title: "Zhenjiu Dacheng (針灸大成)".to_string(),
            volume_or_chapter: "卷七".to_string(),
            passage_key: "徐氏子午流注逐日按時定穴歌；流注圖".to_string(),
            edition_or_facsimile_uri: "PENDING_CLASSICAL_REVIEW".to_string(),
            transcription_uri: "https://zh.wikisource.org/zh-hant/example".to_string(),
            cross_reference_uri: "https://ctext.org/wiki.pl?chapter=688012&if=en".to_string(),
            translation_kind: "verbatim_classical_table_with_project_paraphrase_gloss".to_string(),
        }
    }

    fn sample_review_state() -> ExternalReviewState {
        ExternalReviewState::ExternalReviewPending {
            reason: "najia_xu_style_table_row_review_pending".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: "classical_chinese_reviewer".to_string(),
        }
    }

    fn sample_record(
        state: PointOpeningSlotState,
        provenance: PointOpeningProvenance,
    ) -> FrozenPointOpeningRecord {
        FrozenPointOpeningRecord {
            day_stem_zh: "甲".to_string(),
            hour_branch_zh: "戌".to_string(),
            hour_pillar_zh: "甲戌".to_string(),
            cross_day_spillover: false,
            provenance,
            context: PointOpeningContext::new(
                state,
                sample_review_state(),
                sample_review_state(),
                vec![
                    "TNLC-DIV-01".to_string(),
                    "TNLC-DIV-02".to_string(),
                    "TNLC-DIV-03".to_string(),
                    "TNLC-DIV-05".to_string(),
                ],
            ),
        }
    }

    fn sample_open_state() -> PointOpeningSlotState {
        PointOpeningSlotState::Open {
            slot_class_zh_as_printed: "井".to_string(),
            phase_annotation_as_printed: "井金".to_string(),
            points: vec![PointOpeningIdentity {
                point_key: "qiao-yin".to_string(),
                xue_ming_zh: "竅陰".to_string(),
                huyet_danh_vi: "Kiếu âm".to_string(),
                standard_code_gloss: "GB44".to_string(),
                channel_zh: "足少陽膽".to_string(),
                channel_vi: "Đởm".to_string(),
                channel_en: "Gallbladder".to_string(),
                role: "primary".to_string(),
            }],
            substitution: None,
        }
    }

    fn sample_closed_state() -> PointOpeningSlotState {
        PointOpeningSlotState::Closed {
            running_tables: vec!["gui".to_string(), "jia".to_string()],
            doctrine_zh: "得時為之開，失時為之闔".to_string(),
            note: "the Xu-style tables as printed leave it without an assigned point (閉穴)"
                .to_string(),
        }
    }

    #[test]
    fn provenance_blocks_round_trip_preserving_every_field() {
        let open = PointOpeningProvenance {
            source_id: SOURCE_TY_NGO_LUU_CHU.to_string(),
            work_evidence: vec![sample_citation()],
            table_evidence: Some(PointOpeningTableEvidence {
                table_id: "jia".to_string(),
                row_index: 1,
            }),
        };
        let closed = PointOpeningProvenance {
            source_id: SOURCE_TY_NGO_LUU_CHU.to_string(),
            work_evidence: Vec::new(),
            table_evidence: None,
        };
        for original in [open, closed] {
            let json = serde_json::to_string(&original).unwrap();
            let recovered: PointOpeningProvenance = serde_json::from_str(&json).unwrap();
            assert_eq!(recovered, original);
            assert_eq!(
                serde_json::to_string(&recovered).unwrap(),
                json,
                "wire shape must be stable"
            );
        }
    }

    #[test]
    fn citations_round_trip_preserving_every_field() {
        let original = sample_citation();
        let json = serde_json::to_string(&original).unwrap();
        let recovered: PointOpeningSourceCitation = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered, original);
        assert_eq!(serde_json::to_string(&recovered).unwrap(), json);
    }

    #[test]
    fn method_evidence_cites_the_primitive_source_with_work_notes() {
        let record = sample_record(
            sample_open_state(),
            PointOpeningProvenance {
                source_id: SOURCE_TY_NGO_LUU_CHU.to_string(),
                work_evidence: vec![sample_citation()],
                table_evidence: Some(PointOpeningTableEvidence {
                    table_id: "jia".to_string(),
                    row_index: 1,
                }),
            },
        );
        let entries = record.provenance_entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source_id, SOURCE_TY_NGO_LUU_CHU);
        assert_eq!(entries[0].method, "point_opening_lookup:甲/戌");
        let note = entries[0].note.as_deref().unwrap();
        assert!(note.contains("Zhenjiu Dacheng"));
        assert!(note.contains("卷七"));
        assert!(note.contains("徐氏子午流注逐日按時定穴歌"));

        let envelopes = record.reasoning_evidence();
        assert_eq!(envelopes.len(), 1);
        assert_eq!(envelopes[0].source_id, SOURCE_TY_NGO_LUU_CHU);
        assert_eq!(envelopes[0].method, "point_opening_lookup:甲/戌");
    }

    #[test]
    fn closed_method_evidence_summarizes_the_explicit_closed_state() {
        let record = sample_record(
            sample_closed_state(),
            PointOpeningProvenance {
                source_id: SOURCE_TY_NGO_LUU_CHU.to_string(),
                work_evidence: Vec::new(),
                table_evidence: None,
            },
        );
        let entries = record.provenance_entries();
        assert_eq!(entries.len(), 1, "closed records emit exactly one entry");
        assert_eq!(entries[0].source_id, SOURCE_TY_NGO_LUU_CHU);
        let note = entries[0].note.as_deref().unwrap();
        assert!(note.contains("閉穴"));
        assert!(note.contains("without an assigned point"));
    }
}
