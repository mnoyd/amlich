//! v1.10 Tier-0 Traditional Wellness Context — Twelve-Branch Channel
//! Association (Thập nhị kinh nạp địa chi / 十二經納地支).
//!
//! This module is a sibling of `crate::reasoning` — it does not contribute
//! to Day Assessment, Hour Ranking, or Direction Assessment per ADR-0003.
//! Plan 01-01 ships the core lookup + corpus loader + enrich helper;
//! plan 01-02 will add the `DaySnapshot.traditional_wellness` additive
//! field once the API/TUI/desktop surfaces mirror it.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::almanac::hour_pillar;
use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};
use crate::sources::SOURCE_SHI_ER_JING_NA_DI_ZHI;
use crate::ProvenanceEntry;

use super::disclaimer::{cultural_information_disclaimer, LocalizedDisclaimer};
use super::divergence::{divergence_by_id, ExternalReviewState, TimeBasis};

// ---------------------------------------------------------------------------
// SourceCitation
// ---------------------------------------------------------------------------

/// One source citation attached to a corpus row. Captures the full audit
/// metadata required by `LUNAR_HEALTH_RESEARCH.md:170-184`. Distinct
/// from [`ProvenanceEntry`] (which is the thin graph-evidence wrapper)
/// because the citation carries work, volume, passage, edition, and
/// transcription metadata that graph nodes do not need.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceCitation {
    /// `source_id` constant from [`crate::sources`], e.g.
    /// `SOURCE_SHI_ER_JING_NA_DI_ZHI` for the v1.10 branch-channel rows.
    pub source_id: String,
    /// Bibliographic title (e.g. `"Zhenjiu Daquan"`).
    pub work_title: String,
    /// Volume / chapter (e.g. `"卷之五 論子午流注之法"`).
    pub volume_or_chapter: String,
    /// Passage / table key (e.g. `"十二經納地支歌"`).
    pub passage_key: String,
    /// Edition or facsimile URI consulted by the reviewer. `PENDING_CLASSICAL_REVIEW`
    /// while the gate is unsigned; replaced with a real URI on sign-off.
    pub edition_or_facsimile_uri: String,
    /// Public transcription URI used to discover the verse (e.g. the
    /// Chinese Text Project link). Verbatim copy of the corpus JSON field.
    pub transcription_uri: String,
    /// Translation kind — per `LUNAR_HEALTH_RESEARCH.md:178` the canonical
    /// value for v1.10 is `"project_paraphrase"`.
    pub translation_kind: String,
}

// ---------------------------------------------------------------------------
// BranchChannelAssociation
// ---------------------------------------------------------------------------

/// One row of the twelve-branch / twelve-channel association corpus. The
/// twelve rows together form the canonical historical association table
/// (`Thập nhị kinh nạp địa chi`) and live in
/// `crates/amlich-core/data/traditional-wellness/branch-channel.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchChannelAssociation {
    /// Branch index in Amlich's canonical 0..11 ordering (0 = Tý … 11 = Hợi).
    pub branch_index: u8,
    /// Vietnamese branch label (e.g. `"Tý"`).
    pub branch_vi: String,
    /// Chinese branch label (e.g. `"子"`).
    pub branch_zh: String,
    /// Civil-time window (e.g. `"23:00-01:00"`), verbatim from
    /// `crate::gio_hoang_dao::get_hour_time_range`.
    pub time_range: String,
    /// Vietnamese channel label (e.g. `"Đởm"`).
    pub channel_vi: String,
    /// English channel label (e.g. `"Gallbladder"`).
    pub channel_en: String,
    /// Chinese channel name verbatim from the classical verse (e.g. `"足少陽膽"`).
    pub channel_zh: String,
    /// Neutral historical-association wording, Vietnamese.
    pub wording_vi: String,
    /// Neutral historical-association wording, English.
    pub wording_en: String,
    /// Citation entries; every row carries exactly one (the primary verse).
    pub sources: Vec<SourceCitation>,
    /// Review state. Wire-format is the `ExternalReviewPending(...)` /
    /// `Signed(...)` free-text convention used across the iching and
    /// ritual corpora.
    pub reviewer: ExternalReviewState,
    /// Safety classification. Canonical value
    /// `"historical_cultural_non_clinical"` per
    /// `LUNAR_HEALTH_RESEARCH.md:182`.
    pub safety_class: String,
    /// Divergence IDs applicable to this row (e.g. `["LH-DIV-02", "LH-DIV-03", "LH-DIV-06"]`).
    pub known_divergence_ids: Vec<String>,
    /// Time-basis disclosure. Single-variant today
    /// ([`TimeBasis::LocalCivilHourBranch`]).
    pub time_basis: TimeBasis,
}

impl BranchChannelAssociation {
    /// Stable semantic-graph node id for this row's channel, used by plan
    /// 03-01 to wire `TraditionalChannel` nodes (the schema lock lives
    /// there, not in plan 01-01). Mirrors the role-bearing-stable-key
    /// convention from `SemanticId::iching_hexagram`.
    pub fn channel_semantic_id(&self) -> String {
        format!("channel:shi-er-jing-na-di-zhi:{}", self.channel_zh)
    }

    /// Convert each [`SourceCitation`] into a [`ProvenanceEntry`] for the
    /// reasoning-evidence emission path. Every entry uses
    /// `ProvenanceSource::AlmanacRule` per the precedent set by
    /// `crate::almanac::hour_pillar::compute_hour_pillar`.
    pub fn provenance_entries(&self) -> Vec<ProvenanceEntry> {
        self.sources
            .iter()
            .map(|c| {
                ProvenanceEntry::almanac_rule(
                    c.source_id.clone(),
                    format!("branch_channel_lookup:{}", self.branch_vi),
                )
                .with_note(format!(
                    "{} — {} ({})",
                    c.work_title, c.volume_or_chapter, c.passage_key
                ))
            })
            .collect()
    }

    /// Convert each [`SourceCitation`] into a
    /// [`ReasoningEvidenceEnvelope`] for the high-level reasoning surface.
    pub fn reasoning_evidence(&self) -> Vec<ReasoningEvidenceEnvelope> {
        self.sources
            .iter()
            .map(|c| ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
                source_id: c.source_id.clone(),
                method: format!("branch_channel_lookup:{}", self.branch_vi),
                note: Some(format!(
                    "{} — {} ({})",
                    c.work_title, c.volume_or_chapter, c.passage_key
                )),
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// TraditionalWellnessContext
// ---------------------------------------------------------------------------

/// The full Traditional Wellness Context wrapper. Plan 01-01 emits this
/// as a standalone struct; plan 01-02 promotes it to an additive
/// `DaySnapshot.traditional_wellness: Option<TraditionalWellnessContext>`
/// field once the API/TUI/desktop surfaces mirror it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraditionalWellnessContext {
    /// The selected-hour branch-channel association, if the lookup
    /// resolved. `None` only when the input civil time is out of range
    /// (the `resolve_hour_branch_slot` guard at
    /// `crate::almanac::hour_pillar.rs:35`).
    pub hour_branch: Option<BranchChannelAssociation>,
    /// Stable bilingual cultural-information disclaimer that travels
    /// with every Traditional Wellness Context surface.
    pub disclaimer: LocalizedDisclaimer,
    /// Aggregate review state (the per-row `reviewer` field is also
    /// surfaced via `hour_branch.reviewer` when present).
    pub review_state: ExternalReviewState,
    /// Time-basis disclosure.
    pub time_basis: TimeBasis,
    /// High-level reasoning evidence. Empty in plan 01-01; populated in
    /// plan 03-01 when the semantic-graph wiring lands.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

// ---------------------------------------------------------------------------
// Corpus loader
// ---------------------------------------------------------------------------

const BRANCH_CHANNEL_JSON: &str =
    include_str!("../../data/traditional-wellness/branch-channel.json");

#[derive(Debug, Deserialize)]
struct BranchChannelFile {
    metadata: BranchChannelFileMetadata,
    rows: Vec<BranchChannelRow>,
}

#[derive(Debug, Deserialize)]
struct BranchChannelFileMetadata {
    primary_source: BranchChannelPrimarySource,
    time_basis: String,
}

#[derive(Debug, Deserialize)]
struct BranchChannelPrimarySource {
    source_id: String,
}

#[derive(Debug, Deserialize)]
struct BranchChannelRow {
    branch_index: u8,
    branch_vi: String,
    branch_zh: String,
    time_range: String,
    channel_vi: String,
    channel_en: String,
    channel_zh: String,
    wording_vi: String,
    wording_en: String,
    sources: Vec<SourceCitation>,
    reviewer: ExternalReviewState,
    safety_class: String,
    known_divergence_ids: Vec<String>,
}

impl From<BranchChannelRow> for BranchChannelAssociation {
    fn from(row: BranchChannelRow) -> Self {
        BranchChannelAssociation {
            branch_index: row.branch_index,
            branch_vi: row.branch_vi,
            branch_zh: row.branch_zh,
            time_range: row.time_range,
            channel_vi: row.channel_vi,
            channel_en: row.channel_en,
            channel_zh: row.channel_zh,
            wording_vi: row.wording_vi,
            wording_en: row.wording_en,
            sources: row.sources,
            reviewer: row.reviewer,
            safety_class: row.safety_class,
            known_divergence_ids: row.known_divergence_ids,
            time_basis: TimeBasis::LocalCivilHourBranch,
        }
    }
}

static BRANCH_CHANNEL_CORPUS: OnceLock<Vec<BranchChannelAssociation>> = OnceLock::new();

/// Load and validate the 12-row branch-channel corpus. Returns a
/// `&'static` slice; panics on any invariant violation (this is a test
/// oracle / authoritative corpus, not user-facing data).
///
/// Validation rules:
/// - exactly 12 rows,
/// - rows ordered by `branch_index` 0..11 with no gaps,
/// - every row's `sources` array is non-empty and every source_id
///   resolves to a registered `SOURCE_*` constant in [`crate::sources`]
///   (today: `shi-er-jing-na-di-zhi`),
/// - every row's `known_divergence_ids` resolves to a registered entry
///   via [`divergence_by_id`],
/// - the corpus's `metadata.primary_source.source_id` equals
///   [`SOURCE_SHI_ER_JING_NA_DI_ZHI`].
///
/// The reserved-but-never-emitted source id from ADR-0003 is enforced at
/// the CI level by `tests/source_id_guard.rs`
/// (`ty_ngo_luu_chu_substring_never_appears_in_production_source`) rather
/// than here, so that this loader does not need to mention the reserved
/// id in production source.
pub fn load_corpus() -> &'static [BranchChannelAssociation] {
    BRANCH_CHANNEL_CORPUS.get_or_init(|| {
        let file: BranchChannelFile = serde_json::from_str(BRANCH_CHANNEL_JSON)
            .expect("branch-channel.json must parse cleanly");
        assert_eq!(
            file.metadata.primary_source.source_id,
            SOURCE_SHI_ER_JING_NA_DI_ZHI,
            "corpus metadata.primary_source.source_id must equal SOURCE_SHI_ER_JING_NA_DI_ZHI"
        );
        assert_eq!(
            file.metadata.time_basis,
            TimeBasis::LocalCivilHourBranch.as_str(),
            "corpus metadata.time_basis must equal local_civil_hour_branch"
        );
        assert_eq!(
            file.rows.len(),
            12,
            "branch-channel corpus must contain exactly 12 rows"
        );
        let mut by_index: Vec<Option<BranchChannelAssociation>> = (0..12).map(|_| None).collect();
        for row in file.rows {
            let idx = row.branch_index as usize;
            assert!(
                idx < 12,
                "branch_index out of range: {idx}"
            );
            assert!(
                by_index[idx].is_none(),
                "duplicate branch_index: {idx}"
            );
            assert!(
                !row.sources.is_empty(),
                "row {} carries no sources",
                row.branch_index
            );
            for src in &row.sources {
                assert_eq!(
                    src.source_id,
                    SOURCE_SHI_ER_JING_NA_DI_ZHI,
                    "every branch-channel source_id must equal SOURCE_SHI_ER_JING_NA_DI_ZHI; got {}",
                    src.source_id
                );
            }
            for id in &row.known_divergence_ids {
                assert!(
                    divergence_by_id(id).is_some(),
                    "row {} references unregistered divergence id {id}",
                    row.branch_index
                );
            }
            by_index[idx] = Some(row.into());
        }
        by_index
            .into_iter()
            .enumerate()
            .map(|(i, opt)| {
                opt.unwrap_or_else(|| {
                    panic!("branch-channel corpus missing index {i}")
                })
            })
            .collect()
    })
}

/// Pure lookup: given a local civil hour and minute, return the matching
/// [`BranchChannelAssociation`] row.
///
/// Delegates the boundary math to
/// [`crate::almanac::hour_pillar::resolve_hour_branch_slot`] — never
/// redefines it. The four pinned boundary cases
/// (`22:59 → Hợi`, `23:00 → Tý`, `00:59 → Tý`, `01:00 → Sửu`) are locked
/// by `tests/branch_channel_integration.rs`.
pub fn resolve_hour_branch_association(
    local_hour: u8,
    local_minute: u8,
) -> Option<BranchChannelAssociation> {
    let slot = hour_pillar::resolve_hour_branch_slot(local_hour, local_minute)?;
    let corpus = load_corpus();
    corpus
        .iter()
        .find(|row| row.branch_index as usize == slot.branch_index)
        .cloned()
}

/// Build a [`TraditionalWellnessContext`] for the given local civil time.
/// The function is total from valid inputs (`local_hour <= 23` and
/// `local_minute <= 59`); the wrapped `Option<hour_branch>` distinguishes
/// "out of range" from a successful resolution with a recorded association.
///
/// This is the standalone lookup path used by plan 01-01 tests. Plan
/// 01-02 wraps it in `enrich_day_snapshot_with_branch_channel_association`
/// (signature will simplify once the `DaySnapshot.traditional_wellness`
/// field exists).
pub fn resolve_traditional_wellness_context(
    local_hour: u8,
    local_minute: u8,
) -> TraditionalWellnessContext {
    let hour_branch = resolve_hour_branch_association(local_hour, local_minute);
    let review_state = hour_branch
        .as_ref()
        .map(|row| row.reviewer.clone())
        .unwrap_or(ExternalReviewState::ExternalReviewPending {
            reason: "branch_channel_lookup_out_of_range".to_string(),
            expected_review_date: "YYYY-MM-DD".to_string(),
            assigned_to: "classical_chinese_reviewer".to_string(),
        });
    TraditionalWellnessContext {
        hour_branch,
        disclaimer: cultural_information_disclaimer(),
        review_state,
        time_basis: TimeBasis::LocalCivilHourBranch,
        evidence: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_corpus_returns_exactly_12_rows_in_branch_index_order() {
        let corpus = load_corpus();
        assert_eq!(corpus.len(), 12);
        for (i, row) in corpus.iter().enumerate() {
            assert_eq!(row.branch_index as usize, i);
        }
    }

    #[test]
    fn every_row_uses_only_shi_er_jing_na_di_zhi() {
        let corpus = load_corpus();
        for row in corpus {
            assert!(!row.sources.is_empty());
            for src in &row.sources {
                assert_eq!(src.source_id, SOURCE_SHI_ER_JING_NA_DI_ZHI);
            }
        }
    }

    #[test]
    fn every_row_references_lh_div_02() {
        let corpus = load_corpus();
        for row in corpus {
            assert!(
                row.known_divergence_ids.iter().any(|id| id == "LH-DIV-02"),
                "row {} must reference LH-DIV-02",
                row.branch_index
            );
        }
    }

    #[test]
    fn chinese_channel_names_are_preserved_verbatim() {
        let corpus = load_corpus();
        let expected = [
            "足少陽膽",
            "足厥陰肝",
            "手太陰肺",
            "手陽明大腸",
            "足陽明胃",
            "足太陰脾",
            "手少陰心",
            "手太陽小腸",
            "足太陽膀胱",
            "足少陰腎",
            "手厥陰心包",
            "手少陽三焦",
        ];
        for (row, expected_zh) in corpus.iter().zip(expected.iter()) {
            assert_eq!(&row.channel_zh, expected_zh);
        }
    }

    #[test]
    fn tam_tieu_and_tam_bao_are_preserved_as_traditional_names() {
        // Per LH-DIV-06 / LUNAR_HEALTH_RESEARCH.md:186, 心包 (Tâm bào /
        // Pericardium) and 三焦 (Tam tiêu / Triple Burner) MUST NOT be
        // biomedicalized. They are preserved verbatim as channel names.
        let corpus = load_corpus();
        let tuat = corpus
            .iter()
            .find(|r| r.branch_index == 10)
            .expect("Tuất row");
        assert_eq!(tuat.channel_vi, "Tâm bào");
        assert_eq!(tuat.channel_en, "Pericardium");
        assert_eq!(tuat.channel_zh, "手厥陰心包");
        let hoi = corpus
            .iter()
            .find(|r| r.branch_index == 11)
            .expect("Hợi row");
        assert_eq!(hoi.channel_vi, "Tam tiêu");
        assert_eq!(hoi.channel_en, "Triple Burner");
        assert_eq!(hoi.channel_zh, "手少陽三焦");
    }

    #[test]
    fn resolve_picks_branch_by_index_not_by_branch_name() {
        // Sanity: 00:30 (the middle of Tý) maps to branch_index 0, not
        // to "Sửu" because of any string-comparison accident.
        let row = resolve_hour_branch_association(0, 30).expect("Tý lookup");
        assert_eq!(row.branch_index, 0);
        assert_eq!(row.branch_vi, "Tý");
    }

#[test]
    fn provenance_entries_use_almanac_rule_family() {
        let row = resolve_hour_branch_association(3, 30).expect("Dần lookup");
        let entries = row.provenance_entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].source_id, SOURCE_SHI_ER_JING_NA_DI_ZHI);
        assert!(
            entries[0].method.starts_with("branch_channel_lookup:"),
            "method must be prefixed with branch_channel_lookup:; got {:?}",
            entries[0].method
        );
        assert!(
            entries[0].method.ends_with(":Dần"),
            "method must end with :<branch_vi> for Dần; got {:?}",
            entries[0].method
        );
    }

    #[test]
    fn reasoning_evidence_uses_almanac_rule_family() {
        let row = resolve_hour_branch_association(3, 30).expect("Dần lookup");
        let evidence = row.reasoning_evidence();
        assert_eq!(evidence.len(), 1);
        assert_eq!(
            evidence[0].source_family,
            ReasoningEvidenceSourceFamily::AlmanacRule
        );
    }

    #[test]
    fn out_of_range_inputs_produce_no_association() {
        assert!(resolve_hour_branch_association(24, 0).is_none());
        assert!(resolve_hour_branch_association(12, 60).is_none());
    }

    #[test]
    fn resolve_traditional_wellness_context_carries_disclaimer_and_basis() {
        let ctx = resolve_traditional_wellness_context(23, 30);
        let hb = ctx.hour_branch.expect("Tý must resolve");
        assert_eq!(hb.branch_index, 0);
        assert_eq!(ctx.disclaimer.id.as_str(), "cultural_information_v1");
        assert!(!ctx.disclaimer.vi.is_empty());
        assert!(!ctx.disclaimer.en.is_empty());
        assert_eq!(ctx.time_basis, TimeBasis::LocalCivilHourBranch);
        assert!(matches!(
            ctx.review_state,
            ExternalReviewState::ExternalReviewPending { .. }
        ));
        assert!(ctx.evidence.is_empty());
    }
}
