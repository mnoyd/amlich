//! Phi Tinh golden dataset — multi-source reference cases for FS-10.
//!
//! Provides:
//! - `PhiTinhGoldenDataset` — top-level container
//! - `PhiTinhGoldenCase` — one dated reference case (annual / monthly / period)
//! - `KnownDivergence` — logged source disagreement (FS-10: log, do NOT silently correct)
//! - `GoldenConfidence` — typed per-case confidence tier (HIGH | MEDIUM | LOW)
//! - `load_flying_stars_golden()` — OnceLock + include_str! loader with coverage validation
//!
//! Validation at load time (mirrors `golden_loader.rs` pattern):
//! - case_count metadata matches actual vec length
//! - every annual/monthly case has >= 2 sources (FS-10)
//! - every case has a non-empty tiebreaker
//! - per-Vận coverage: >= 10 annual cases each for Vận 7, Vận 8, Vận 9 (FS-05/FS-10)
//!
//! ADR-0003 §4: matrix structure, Tam Nguyên ranges, year polarity rule (unchanged).
//! ADR-0003a (2026-07-15, supersedes §6): pre-1984 Thượng/Trung Nguyên polarity rows
//! promoted to HIGH confidence after dual-source independent secondary modern
//! verification (phongthuycaivan.org + lasotuvi.com / phongthuyso.vn); the
//! >=10-per-Vận gate applies only to Vận 7/8/9.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// JSON data path
// ---------------------------------------------------------------------------

const FLYING_STARS_GOLDEN_JSON: &str =
    include_str!("../../../data/almanac/flying_stars_golden.json");

static FLYING_STARS_GOLDEN: OnceLock<PhiTinhGoldenDataset> = OnceLock::new();

// ---------------------------------------------------------------------------
// Types
// ---------------------------------------------------------------------------

/// A single source's value for a given case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceValue {
    /// Identifier for the reference source (e.g. "phongthuycaivan.org").
    pub source: String,
    /// The star number reported by this source (1..=9).
    pub value: u8,
}

/// Confidence tier for a golden-dataset case.
///
/// ADR-0003a (2026-07-15) supersedes ADR-0003 §6: pre-1984 Thượng/Trung Nguyên
/// polarity rows are HIGH after dual-source independent secondary modern
/// verification (phongthuycaivan.org + lasotuvi.com / phongthuyso.vn). The
/// `Medium` default is a compatibility shim for legacy JSON that omitted the
/// field; canonical current cases MUST set `"confidence": "high"` explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum GoldenConfidence {
    /// Two-source independent secondary modern verification + classical tiebreaker
    /// (e.g. *Thẩm Thị Huyền Không Học*).
    High,
    /// Compatibility default — used only for legacy JSON that omitted the field.
    #[default]
    Medium,
    /// Single-source only; reserved for future use.
    Low,
}

/// Structured marker for a deferred `KnownDivergence`.
///
/// The presence of this marker (via `KnownDivergence.deferral: Option<DeferralMarker>`)
/// is the typed `PendingExternalReview` disposition — the divergence has NOT been
/// resolved by the polarity-row confidence upgrade (see ADR-0003a §4 for the
/// 1960 Trung Nguyên case). The provisional tiebreaker value remains in
/// `KnownDivergence.our_value` while review is pending.
///
/// Backward compatibility: the field on `KnownDivergence` is `Option<DeferralMarker>`
/// with `#[serde(default, skip_serializing_if = "Option::is_none")]` so legacy
/// payloads that omit `deferral` deserialize unchanged.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeferralMarker {
    /// Reason the divergence is deferred (e.g., independent sources disagree
    /// on the center-star encoding and classical cross-check does not settle it).
    pub reason: String,
    /// ISO 8601 date when review is expected (e.g., `"2026-12-31"`).
    pub expected_review_date: String,
    /// Who/what will perform the review. `None` is acceptable for purely
    /// external (unassigned) deferrals; canonical current deferrals set it.
    pub assigned_to: Option<String>,
}

/// A known source divergence — logged, never silently corrected (FS-10).
///
/// When two or more authoritative references disagree on a value, the
/// disagreement is recorded here.  `our_value` is the tiebreaker selection
/// per *Thẩm Thị Huyền Không Học*; the losing source values are preserved
/// for audit.
///
/// A `deferral` marker (if present) signals that the divergence disposition is
/// `PendingExternalReview` — the provisional tiebreaker in `our_value` is
/// retained while review is pending, NOT silently corrected (FS-10 / ADR-0003a §4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownDivergence {
    /// Human-readable case identifier (e.g. "annual 1960").
    pub case: String,
    /// The value we select after applying the tiebreaker.
    pub our_value: u8,
    /// All source values, including the losing ones.
    pub source_values: Vec<SourceValue>,
    /// Which tiebreaker was applied and which source won.
    pub tiebreaker: String,
    /// Additional context on why this divergence exists.
    pub note: String,
    /// Optional typed `PendingExternalReview` deferral marker. The presence
    /// of this field is the machine-readable signal that the divergence has
    /// NOT been silently corrected and is awaiting external review. Backward
    /// compatible: absent markers deserialize as `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferral: Option<DeferralMarker>,
}

/// One dated reference case for Phi Tinh validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiTinhGoldenCase {
    /// Unique identifier within the dataset.
    pub id: String,
    /// Case type: "annual" | "monthly" | "period".
    pub kind: String,
    /// Solar year for the case.
    pub year: i32,
    /// Solar month (1-based, ADR-0002 convention). Present for `kind="monthly"`.
    #[serde(default)]
    pub month: Option<u8>,
    /// Julian Day Number. Present for `kind="period"` boundary tests.
    #[serde(default)]
    pub jd: Option<i32>,
    /// Expected active Vận number (1..=9).
    pub van: u8,
    /// Expected center star for the layer under test (1..=9).
    ///
    /// For "annual": the Niên Tử Bạch center star.
    /// For "monthly": the Nguyệt Tử Bạch center star.
    /// For "period": equals `van` (tests `compute_period().van`).
    pub expected_center: u8,
    /// Reference sources — at least 2 required for annual/monthly cases (FS-10).
    pub sources: Vec<SourceValue>,
    /// Tiebreaker note citing *Thẩm Thị Huyền Không Học*.
    pub tiebreaker: String,
    /// Free-text note for this case.
    pub note: String,
    /// Confidence tier for this case. Defaults to `Medium` for legacy JSON;
    /// canonical current cases set `"confidence": "high"` explicitly.
    ///
    /// ADR-0003a supersedes ADR-0003 §6: pre-1984 Thượng/Trung Nguyên rows are
    /// HIGH after dual-source independent secondary modern verification.
    #[serde(default)]
    pub confidence: GoldenConfidence,
}

/// Top-level Phi Tinh golden dataset container.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhiTinhGoldenDataset {
    pub metadata: GoldenMeta,
    pub cases: Vec<PhiTinhGoldenCase>,
    /// Logged source divergences. May be empty if all sources agree.
    #[serde(default)]
    pub known_divergences: Vec<KnownDivergence>,
}

/// Dataset-level metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenMeta {
    /// Human-readable description of the dataset.
    pub description: String,
    /// Number of cases (must equal `cases.len()` at load time).
    pub case_count: usize,
}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

/// Load and validate the Phi Tinh golden dataset, returning a `&'static` ref.
///
/// Parses the embedded JSON on first call, validates coverage invariants, then
/// caches via `OnceLock`.  Panics on any invariant violation — this is a test
/// oracle, not user-facing data.
pub fn load_flying_stars_golden() -> &'static PhiTinhGoldenDataset {
    FLYING_STARS_GOLDEN.get_or_init(|| {
        let dataset: PhiTinhGoldenDataset = serde_json::from_str(FLYING_STARS_GOLDEN_JSON)
            .expect("Failed to parse flying_stars_golden.json");
        validate_phi_tinh_golden(&dataset);
        dataset
    })
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_phi_tinh_golden(dataset: &PhiTinhGoldenDataset) {
    // 1. Non-empty and metadata consistency.
    assert!(
        !dataset.cases.is_empty(),
        "flying_stars_golden.json: cases must not be empty"
    );
    assert_eq!(
        dataset.metadata.case_count,
        dataset.cases.len(),
        "flying_stars_golden.json: metadata.case_count ({}) must match cases.len() ({})",
        dataset.metadata.case_count,
        dataset.cases.len()
    );

    // 2. Per-case invariants for annual/monthly cases.
    for case in &dataset.cases {
        if case.kind == "annual" || case.kind == "monthly" {
            assert!(
                case.sources.len() >= 2,
                "case '{}' ({}): must have >= 2 sources, got {}",
                case.id,
                case.kind,
                case.sources.len()
            );
        }
        assert!(
            !case.tiebreaker.trim().is_empty(),
            "case '{}': tiebreaker must not be empty",
            case.id
        );
    }

    // 3. Per-Vận coverage: >= 10 annual cases each for Vận 7, 8, 9.
    let van7_count = dataset
        .cases
        .iter()
        .filter(|c| c.kind == "annual" && c.van == 7)
        .count();
    let van8_count = dataset
        .cases
        .iter()
        .filter(|c| c.kind == "annual" && c.van == 8)
        .count();
    let van9_count = dataset
        .cases
        .iter()
        .filter(|c| c.kind == "annual" && c.van == 9)
        .count();

    assert!(
        van7_count >= 10,
        "flying_stars_golden.json: need >= 10 annual cases for Vận 7, got {}",
        van7_count
    );
    assert!(
        van8_count >= 10,
        "flying_stars_golden.json: need >= 10 annual cases for Vận 8, got {}",
        van8_count
    );
    assert!(
        van9_count >= 10,
        "flying_stars_golden.json: need >= 10 annual cases for Vận 9, got {}",
        van9_count
    );
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn golden_dataset_loads_and_validates() {
        let ds = load_flying_stars_golden();
        assert!(!ds.cases.is_empty());
        assert_eq!(ds.metadata.case_count, ds.cases.len());
    }

    #[test]
    fn golden_dataset_van7_coverage() {
        let ds = load_flying_stars_golden();
        let count = ds.cases.iter().filter(|c| c.kind == "annual" && c.van == 7).count();
        assert!(count >= 10, "expected >= 10 Van 7 annual cases, got {count}");
    }

    #[test]
    fn golden_dataset_van8_coverage() {
        let ds = load_flying_stars_golden();
        let count = ds.cases.iter().filter(|c| c.kind == "annual" && c.van == 8).count();
        assert!(count >= 10, "expected >= 10 Van 8 annual cases, got {count}");
    }

    #[test]
    fn golden_dataset_van9_coverage() {
        let ds = load_flying_stars_golden();
        let count = ds.cases.iter().filter(|c| c.kind == "annual" && c.van == 9).count();
        assert!(count >= 10, "expected >= 10 Van 9 annual cases, got {count}");
    }

    #[test]
    fn golden_dataset_has_known_divergences() {
        let ds = load_flying_stars_golden();
        assert!(
            !ds.known_divergences.is_empty(),
            "expected at least one KnownDivergence entry"
        );
    }

    #[test]
    fn golden_dataset_all_cases_have_tiebreaker() {
        let ds = load_flying_stars_golden();
        for case in &ds.cases {
            assert!(
                !case.tiebreaker.trim().is_empty(),
                "case '{}' has empty tiebreaker",
                case.id
            );
        }
    }

    #[test]
    fn golden_dataset_annual_monthly_cases_have_two_sources() {
        let ds = load_flying_stars_golden();
        for case in &ds.cases {
            if case.kind == "annual" || case.kind == "monthly" {
                assert!(
                    case.sources.len() >= 2,
                    "case '{}' ({}): need >= 2 sources, got {}",
                    case.id,
                    case.kind,
                    case.sources.len()
                );
            }
        }
    }

    #[test]
    fn golden_dataset_period_cases_exist() {
        let ds = load_flying_stars_golden();
        let count = ds.cases.iter().filter(|c| c.kind == "period").count();
        assert!(count >= 2, "expected >= 2 period boundary cases, got {count}");
    }

    #[test]
    fn golden_dataset_cross_validation_cases_exist() {
        let ds = load_flying_stars_golden();
        // ADR-0003 open question #3: Thuong/Trung Nguyen cross-validation
        let pre_1984 = ds.cases.iter().filter(|c| c.kind == "annual" && c.year < 1984).count();
        assert!(pre_1984 >= 2, "expected >= 2 pre-1984 cross-validation cases, got {pre_1984}");
    }
}
