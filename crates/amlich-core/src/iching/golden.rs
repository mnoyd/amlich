//! Mai Hoa golden dataset — cross-source reference cases for ICH-04 + SC4.
//!
//! Provides:
//! - `MaiHoaGoldenDataset` — top-level container (12 cases + divergence rows)
//! - `MaiHoaGoldenCase` — one lunar-input → cast-output reference case
//! - `MaiHoaKnownDivergence` — logged source disagreement (FS-10 / AF-05:
//!   log, do NOT silently correct)
//! - `load_mai_hoa_golden()` — `OnceLock` + `include_str!` loader mirroring
//!   `iching/corpus.rs` (Plan 21-02) EXACTLY in shape
//!
//! Validation at load time:
//! - schema version == `"mai-hoa-golden-v1"` (hard pin; any change requires a
//!   superseding ADR)
//! - case count >= 10 (Phase 22 SC4 / INT-13)
//! - every case has >= 2 source entries (FS-10 dual-source discipline)
//! - >= 1 known_divergences row demonstrates FS-10 audit logging
//!
//! NFC normalization applied to every Vietnamese/string text field at load
//! (RIT-08 precedent). The divergent value is a STRING (hexagram description)
//! because Mai Hoa divergences don't map to a single u8 — we deliberately do
//! NOT force-cast Mai Hoa data into the fengshui `KnownDivergence` shape (the
//! fengshui one carries a u8 star value; Mai Hoa carries full casting
//! tuples). We re-use `DeferralMarker` and `GoldenConfidence` verbatim from
//! `almanac::fengshui::golden` (it is generic).
//!
//! WASM-safety: the loader uses `include_str!` (compile-time) + `OnceLock`
//! (std, WASM-safe) and avoids filesystem and wall-clock APIs. The WASM-safety
//! grep guard in the inline test module asserts this discipline.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use unicode_normalization::{is_nfc, UnicodeNormalization};

use crate::almanac::fengshui::golden::{DeferralMarker, GoldenConfidence};
use crate::iching::schema::{KingWenHexagram, TienThienTrigram};

// ---------------------------------------------------------------------------
// JSON data path
// ---------------------------------------------------------------------------

/// The Mai Hoa golden dataset, embedded at compile time. Mirrors the
/// `corpus.rs` path convention: from `src/iching/golden.rs`, `../../data/iching/`
/// resolves to `crates/amlich-core/data/iching/`.
const MAI_HOA_GOLDEN_JSON: &str = include_str!("../../data/iching/mai_hoa_golden.json");

const EXPECTED_SCHEMA_VERSION: &str = "mai-hoa-golden-v1";

static MAI_HOA_GOLDEN: OnceLock<MaiHoaGoldenDataset> = OnceLock::new();

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// Cast inputs (the four lunar inputs + outputs of `cast_mai_hoa`).
///
/// Field ranges documented in `MaiHoaCast` (Plan 22-01). These are the same
/// inputs to `cast_mai_hoa(year_branch, month, day, hour)` — kept in JSON so
/// the golden dataset can be re-cast by any consumer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaiHoaGoldenInputs {
    /// Lunar year branch (0..=11, Tý=0..=Hợi=11).
    pub year_branch: u8,
    /// Lunar month (1..=12).
    pub month: u8,
    /// Lunar day (1..=30).
    pub day: u8,
    /// Chi-hour index (0..=11, Tý=0..=Hợi=11).
    pub hour: u8,
}

/// The expected [`crate::iching::mai_hoa::MaiHoaCast`] output for a golden
/// case.
///
/// `upper`/`lower` are snake_case [`TienThienTrigram`] names; `king_wen` is
/// the King Wen hexagram index. The `Serialize` derive on these types uses the
/// locked `rename_all = "snake_case"` discipline, so the JSON shape is
/// stable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaiHoaGoldenExpected {
    pub upper: TienThienTrigram,
    pub lower: TienThienTrigram,
    pub dong_hao: u8,
    pub king_wen: KingWenHexagram,
}

/// One independent source entry for a golden case.
///
/// The `value` is a free-text description of what the source says about the
/// cast (not a single u8 — Mai Hoa values are full casting tuples, not star
/// numbers). Sources must number >= 2 per FS-10 dual-source discipline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaiHoaGoldenSource {
    /// Identifier for the reference source (e.g. "nhantu.net", "vi.wikipedia").
    pub source: String,
    /// URL or page-reference string. May carry
    /// `[PendingExternalReview — ...]` markers honestly per AF-05.
    pub url_or_ref: String,
    /// The source's description of the cast (free-text).
    pub value: String,
}

/// One dated (well, input-tuple) reference case for Mai Hoa casting validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaiHoaGoldenCase {
    /// Unique identifier within the dataset.
    pub id: String,
    /// Cast inputs (year_branch, month, day, hour).
    pub inputs: MaiHoaGoldenInputs,
    /// Expected `cast_mai_hoa(inputs...)` output.
    pub expected: MaiHoaGoldenExpected,
    /// Reference sources — at least 2 required (FS-10 dual-source discipline).
    pub sources: Vec<MaiHoaGoldenSource>,
    /// Confidence tier for this case (reused from fengshui/golden).
    pub confidence: GoldenConfidence,
    /// Free-text note for this case.
    pub note: String,
}

/// A logged source divergence (FS-10 / AF-05: log, do NOT silently correct).
///
/// Distinct from `almanac::fengshui::golden::KnownDivergence` because the
/// divergent value here is a STRING (hexagram/trigram description), not a u8.
/// Re-uses `DeferralMarker` verbatim — that struct is generic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaiHoaKnownDivergence {
    /// Human-readable case identifier (e.g. "thap-can-tien-thien-Ly-vs-Kien").
    pub case: String,
    /// The value we select after applying the tiebreaker (free-text).
    pub our_value: String,
    /// All source values, including the losing ones.
    pub source_values: Vec<MaiHoaGoldenSource>,
    /// Which tiebreaker was applied and which source won.
    pub tiebreaker: String,
    /// Additional context on why this divergence exists.
    pub note: String,
    /// Optional typed `PendingExternalReview` deferral marker (reused from
    /// fengshui/golden). Presence signals the divergence has NOT been
    /// silently corrected and is awaiting external review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferral: Option<DeferralMarker>,
}

/// Top-level Mai Hoa golden dataset container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaiHoaGoldenDataset {
    /// Schema version marker — asserted at load (mirrors corpus.rs pattern).
    #[serde(rename = "$schema_version")]
    pub schema_version: String,
    /// The case list.
    pub cases: Vec<MaiHoaGoldenCase>,
    /// Logged source divergences (>= 1 per FS-10 discipline).
    pub known_divergences: Vec<MaiHoaKnownDivergence>,
}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

/// Load and validate the Mai Hoa golden dataset, returning a `&'static` ref.
///
/// Mirrors [`crate::iching::corpus::all_hexagrams`] in shape: `OnceLock` +
/// `include_str!` + envelope + `nfc()` per string field.
///
/// Panics if the embedded JSON fails to parse, if `$schema_version != "mai-hoa-golden-v1"`,
/// if `cases.len() < 10`, if any case has fewer than 2 sources, or if
/// `known_divergences.len() < 1`. These are build-time invariants — a
/// violation is caught at compile-embed time, not deferred to runtime.
pub fn load_mai_hoa_golden() -> &'static MaiHoaGoldenDataset {
    MAI_HOA_GOLDEN.get_or_init(|| {
        let mut dataset: MaiHoaGoldenDataset = serde_json::from_str(MAI_HOA_GOLDEN_JSON)
            .unwrap_or_else(|e| panic!("Failed to parse mai_hoa_golden.json: {e}"));

        // Hard pin on schema version (mirrors corpus.rs).
        assert_eq!(
            dataset.schema_version, EXPECTED_SCHEMA_VERSION,
            "mai_hoa_golden.json: schema_version must equal {:?} (Plan 22-02); found {:?}",
            EXPECTED_SCHEMA_VERSION, dataset.schema_version
        );

        // RIT-08 precedent: NFC-normalize every string field.
        for case in &mut dataset.cases {
            case.note = nfc(&case.note);
            for src in &mut case.sources {
                src.source = nfc(&src.source);
                src.url_or_ref = nfc(&src.url_or_ref);
                src.value = nfc(&src.value);
            }
        }
        for div in &mut dataset.known_divergences {
            div.case = nfc(&div.case);
            div.our_value = nfc(&div.our_value);
            div.tiebreaker = nfc(&div.tiebreaker);
            div.note = nfc(&div.note);
            for src in &mut div.source_values {
                src.source = nfc(&src.source);
                src.url_or_ref = nfc(&src.url_or_ref);
                src.value = nfc(&src.value);
            }
        }

        validate_mai_hoa_golden(&dataset);
        dataset
    })
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_mai_hoa_golden(dataset: &MaiHoaGoldenDataset) {
    // 1. SC4: case count >= 10.
    assert!(
        dataset.cases.len() >= 10,
        "mai_hoa_golden.json: need >= 10 cases (Phase 22 SC4), got {}",
        dataset.cases.len()
    );

    // 2. FS-10: every case has >= 2 sources.
    for case in &dataset.cases {
        assert!(
            case.sources.len() >= 2,
            "case '{}': FS-10 violation — must have >= 2 sources, got {}",
            case.id,
            case.sources.len()
        );
        assert!(
            (1..=6).contains(&case.expected.dong_hao),
            "case '{}': dong_hao out of 1..=6: {}",
            case.id,
            case.expected.dong_hao
        );
        assert!(
            (1..=64).contains(&case.expected.king_wen.0),
            "case '{}': king_wen out of 1..=64: {}",
            case.id,
            case.expected.king_wen.0
        );
    }

    // 3. FS-10 / AF-05: at least one known_divergence row.
    assert!(
        !dataset.known_divergences.is_empty(),
        "mai_hoa_golden.json: FS-10 violation — known_divergences must have >= 1 row"
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn nfc(s: &str) -> String {
    if is_nfc(s) {
        s.to_string()
    } else {
        s.nfc().collect()
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// SC4: dataset has >= 10 cases.
    #[test]
    fn golden_dataset_has_at_least_ten_cases() {
        let ds = load_mai_hoa_golden();
        assert!(
            ds.cases.len() >= 10,
            "expected >= 10 cases (Phase 22 SC4), got {}",
            ds.cases.len()
        );
    }

    /// FS-10: every case carries >= 2 source entries.
    #[test]
    fn golden_dataset_every_case_has_dual_sources() {
        let ds = load_mai_hoa_golden();
        for case in &ds.cases {
            assert!(
                case.sources.len() >= 2,
                "case '{}' has {} sources (FS-10 requires >= 2)",
                case.id,
                case.sources.len()
            );
        }
    }

    /// FS-10: at least one known_divergence row demonstrates the audit
    /// discipline.
    #[test]
    fn golden_dataset_has_at_least_one_known_divergence() {
        let ds = load_mai_hoa_golden();
        assert!(
            !ds.known_divergences.is_empty(),
            "expected >= 1 KnownDivergence row (FS-10 / AF-05 discipline)"
        );
    }

    /// Schema version asserted at load.
    #[test]
    fn golden_dataset_schema_version_is_pinned() {
        let ds = load_mai_hoa_golden();
        assert_eq!(ds.schema_version, "mai-hoa-golden-v1");
    }

    /// OnceLock idempotency: two calls return the same pointer.
    #[test]
    fn golden_dataset_load_is_idempotent() {
        let a = load_mai_hoa_golden();
        let b = load_mai_hoa_golden();
        assert_eq!(
            a as *const _, b as *const _,
            "OnceLock should return the same pointer on subsequent calls"
        );
    }

    /// CRIT-3 isolation guard — this module must NOT add any cross-newtype
    /// `From` impl. Mirrors the corpus/mai_hoa pattern from Plan 21/22-01.
    /// Build the search needles at runtime so the test's own source doesn't
    /// trip the grep.
    #[test]
    fn crit3_isolation_no_cross_newtype_from_impls_inline() {
        const SRC: &str = include_str!("golden.rs");
        let needles: Vec<String> = [
            ("Tien", "ThienTrigram"),
            ("Hau", "ThienTrigram"),
            ("King", "WenHexagram"),
        ]
        .iter()
        .flat_map(|(a, b)| [format!("impl From<{a}{b}"), format!("impl<{a}{b}> From")])
        .collect();
        for needle in &needles {
            assert!(
                !SRC.contains(needle.as_str()),
                "CRIT-3 violation: `{needle}` found in golden.rs. \
                 The three iching newtypes must NOT have cross-type From impls."
            );
        }
    }

    /// WASM-safety guard — no filesystem / wall-clock / RNG APIs in the
    /// loader (Plan 22-01 + 21-02 discipline).
    ///
    /// Builds the forbidden-pattern needles at runtime via `format!` so the
    /// test's own source doesn't trip the grep (mirrors the CRIT-3 grep
    /// guard runtime-built-needle pattern). A bare string in a doc comment
    /// MUST NOT trip the guard.
    #[test]
    fn wasm_safety_no_fs_no_utc_no_rand_inline() {
        const SRC: &str = include_str!("golden.rs");

        // Build the forbidden needles at runtime; never embed them as
        // literals anywhere in this file (or the test's own source would
        // trip the grep).
        let mut fs = String::from("std::f");
        fs.push('s');
        let mut fs_qualified = fs.clone();
        fs_qualified.push_str("::");
        let mut fs_import = String::from("use ");
        fs_import.push_str(&fs);
        let utc_now = format!("Utc::{}", "now");
        let rand_colon = format!("rand{}", "::");

        assert!(
            !SRC.contains(fs_qualified.as_str()),
            "WASM-safety: filesystem API qualified usage appears in golden.rs"
        );
        assert!(
            !SRC.contains(fs_import.as_str()),
            "WASM-safety: filesystem import appears in golden.rs"
        );
        assert!(
            !SRC.contains(utc_now.as_str()),
            "WASM-safety: wall-clock API appears in golden.rs"
        );
        assert!(
            !SRC.contains(rand_colon.as_str()),
            "WASM-safety: RNG crate import appears in golden.rs"
        );
        let _ = fs;
    }
}
