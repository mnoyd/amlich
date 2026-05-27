//! Phi Tinh (Flying Stars) 81-cell star-pair aspect corpus (FS-11, FS-12, FS-13).
//!
//! Loads the complete 81 ordered-pair aspect table for Huyền Không from
//! `data/almanac/flying_star_aspects.json` via `OnceLock` + `include_str!`.
//!
//! Validates that:
//! - exactly 81 ordered (star_a, star_b) pairs are present (each 1..=9 × 1..=9)
//! - no pair is duplicated
//! - every entry has a non-empty `original_citation.title`
//! - every entry's `source_id` equals `SOURCE_HUYEN_KHONG`
//!
//! Public API:
//! - `lookup_star_pair_aspect(a, b)` — O(81) scan; panics if pair absent (corpus invariant).
//! - `compute_palace_aspects(year, month, scanner)` — delegates to `compute_combined_overlay`
//!   then calls `lookup_star_pair_aspect` per palace.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::almanac::fengshui::combined::compute_combined_overlay;
use crate::almanac::fengshui::scanner::TietKhiScanner;
use crate::almanac::fengshui::types::FlyingStar;

// Path depth: fengshui/ -> almanac/ -> src/ -> crate root -> data/
const ASPECTS_JSON: &str = include_str!("../../../data/almanac/flying_star_aspects.json");

static ASPECTS_CORPUS: OnceLock<StarPairAspectsCorpus> = OnceLock::new();

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Bibliographic citation for a star-pair aspect row.
///
/// Mirrors `SourceCitation` shape but declared locally to keep the
/// fengshui and rituals pillars decoupled (PITFALLS Pitfall 4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FsCitation {
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
}

/// Confidence tier for a star-pair aspect row.
///
/// Mirrors `RitualConfidenceTier` naming but declared locally to avoid
/// cross-pillar import (PITFALLS Pitfall 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FsConfidenceTier {
    Primary,
    RegionalVariant,
    Synthesized,
}

/// A single ordered (star_a, star_b) aspect row from the 81-cell corpus.
///
/// `star_a` is the "host" (annual) star and `star_b` is the "visiting" (monthly) star,
/// though the lookup is purely positional — (1,9) and (9,1) are distinct entries
/// with potentially different `auspice` and `name` values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StarPairAspect {
    pub star_a: u8,
    pub star_b: u8,
    pub name: String,
    pub ngu_hanh_relation: String,
    pub auspice: String,
    pub source_id: String,
    pub original_citation: FsCitation,
    pub confidence: FsConfidenceTier,
}

// ---------------------------------------------------------------------------
// Internal deserialization envelope
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct StarPairAspectsCorpus {
    schema_version: String,
    #[allow(dead_code)]
    source: String,
    aspects: Vec<StarPairAspect>,
}

// ---------------------------------------------------------------------------
// Loader + validator
// ---------------------------------------------------------------------------

fn load_aspects_inner() -> StarPairAspectsCorpus {
    let corpus: StarPairAspectsCorpus =
        serde_json::from_str(ASPECTS_JSON).expect("Failed to parse flying_star_aspects.json");
    validate_aspects_corpus(&corpus);
    corpus
}

fn validate_aspects_corpus(c: &StarPairAspectsCorpus) {
    assert_eq!(
        c.aspects.len(),
        81,
        "flying_star_aspects.json must contain exactly 81 aspects, got {}",
        c.aspects.len()
    );

    // Build seen grid: seen[a][b] tracks whether (a, b) has been encountered.
    // Dimensions 10×10 so we can index directly with star numbers 1..=9.
    let mut seen = [[false; 10usize]; 10usize];

    for asp in &c.aspects {
        let a = asp.star_a as usize;
        let b = asp.star_b as usize;

        assert!(
            a >= 1 && a <= 9,
            "aspect star_a={} is out of range 1..=9",
            asp.star_a
        );
        assert!(
            b >= 1 && b <= 9,
            "aspect star_b={} is out of range 1..=9",
            asp.star_b
        );
        assert!(
            !seen[a][b],
            "duplicate ordered pair ({},{}) in flying_star_aspects.json",
            asp.star_a,
            asp.star_b
        );
        seen[a][b] = true;

        assert!(
            !asp.original_citation.title.is_empty(),
            "empty original_citation.title for aspect ({},{})",
            asp.star_a,
            asp.star_b
        );
        assert_eq!(
            asp.source_id,
            crate::sources::SOURCE_HUYEN_KHONG,
            "aspect ({},{}) has source_id {:?}, expected SOURCE_HUYEN_KHONG",
            asp.star_a,
            asp.star_b,
            asp.source_id
        );
    }

    // Assert all 81 ordered pairs are present.
    for a in 1..=9usize {
        for b in 1..=9usize {
            assert!(
                seen[a][b],
                "ordered pair ({},{}) is missing from flying_star_aspects.json",
                a,
                b
            );
        }
    }

    // Assert schema_version is non-empty.
    assert!(
        !c.schema_version.is_empty(),
        "schema_version must be non-empty in flying_star_aspects.json"
    );
}

fn aspects_corpus() -> &'static StarPairAspectsCorpus {
    ASPECTS_CORPUS.get_or_init(load_aspects_inner)
}

// ---------------------------------------------------------------------------
// Public API (FS-11, FS-13)
// ---------------------------------------------------------------------------

/// Return the aspect for the ordered pair `(star_a, star_b)`.
///
/// The lookup is order-sensitive: `(NhatBach, CuuTu)` and `(CuuTu, NhatBach)` resolve to
/// distinct corpus rows. Returns an owned `StarPairAspect` (corpus is `'static`).
///
/// Panics if the pair is not found — this indicates a corpus invariant violation since
/// the validator guarantees all 81 ordered pairs are present.
pub fn lookup_star_pair_aspect(star_a: FlyingStar, star_b: FlyingStar) -> StarPairAspect {
    let a = star_a as u8;
    let b = star_b as u8;
    aspects_corpus()
        .aspects
        .iter()
        .find(|asp| asp.star_a == a && asp.star_b == b)
        .cloned()
        .unwrap_or_else(|| {
            panic!(
                "lookup_star_pair_aspect: ({},{}) not found — corpus invariant broken",
                a, b
            )
        })
}

/// Return the 9 star-pair aspects for each palace, derived from the combined overlay.
///
/// The returned array is indexed in `Palace::ALL` order (index 0 = N, index 4 = Center,
/// index 8 = S). Each element represents `(annual_star=host, monthly_star=visiting)`.
///
/// Delegates to `compute_combined_overlay` then looks up each palace's
/// `(annual_star, monthly_star)` pair via `lookup_star_pair_aspect`.
pub fn compute_palace_aspects(
    year: i32,
    month: u8,
    scanner: &TietKhiScanner,
) -> [StarPairAspect; 9] {
    let overlay = compute_combined_overlay(year, month, scanner);
    std::array::from_fn(|i| {
        let (annual, monthly) = overlay.palace_overlays[i];
        lookup_star_pair_aspect(annual, monthly)
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::fengshui::stars::flying_star_from_u8;

    /// Corpus has exactly 81 entries.
    #[test]
    fn test_corpus_has_81_entries() {
        assert_eq!(aspects_corpus().aspects.len(), 81);
    }

    /// StarPairAspect round-trips through serde (deny_unknown_fields rejects an extra field).
    #[test]
    fn test_star_pair_aspect_serde_round_trip() {
        let asp = lookup_star_pair_aspect(FlyingStar::NhatBach, FlyingStar::LucBach);
        let json = serde_json::to_string(&asp).expect("serialization failed");
        let roundtripped: StarPairAspect =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(roundtripped, asp);
    }

    /// deny_unknown_fields rejects an extra field on StarPairAspect.
    #[test]
    fn test_star_pair_aspect_deny_unknown_fields() {
        // Build the JSON string using the constant to avoid a bare literal in .rs
        // (source_id_guard.rs scans all .rs files outside #[cfg(test)] blocks and
        //  this test is in cfg(test), but we follow the discipline everywhere).
        let source_val = crate::sources::SOURCE_HUYEN_KHONG;
        let bad_json = format!(
            r#"{{
            "star_a": 1,
            "star_b": 6,
            "name": "test",
            "ngu_hanh_relation": "sinh",
            "auspice": "auspicious",
            "source_id": "{source_val}",
            "original_citation": {{"title": "Test"}},
            "confidence": "primary",
            "extra_field": "should_fail"
        }}"#
        );
        let result: Result<StarPairAspect, _> = serde_json::from_str(&bad_json);
        assert!(result.is_err(), "unknown field should be rejected");
    }

    /// FsConfidenceTier serializes Primary -> "primary", RegionalVariant -> "regional-variant",
    /// Synthesized -> "synthesized".
    #[test]
    fn test_fs_confidence_tier_kebab_serialization() {
        let primary = serde_json::to_string(&FsConfidenceTier::Primary).unwrap();
        assert_eq!(primary, "\"primary\"");
        let regional = serde_json::to_string(&FsConfidenceTier::RegionalVariant).unwrap();
        assert_eq!(regional, "\"regional-variant\"");
        let synthesized = serde_json::to_string(&FsConfidenceTier::Synthesized).unwrap();
        assert_eq!(synthesized, "\"synthesized\"");
    }

    /// All 81 ordered pairs are accessible via lookup_star_pair_aspect.
    #[test]
    fn test_all_81_pairs_accessible() {
        for a in 1u8..=9 {
            for b in 1u8..=9 {
                let star_a = flying_star_from_u8(a);
                let star_b = flying_star_from_u8(b);
                let asp = lookup_star_pair_aspect(star_a, star_b);
                assert_eq!(asp.star_a, a, "star_a mismatch for ({},{})", a, b);
                assert_eq!(asp.star_b, b, "star_b mismatch for ({},{})", a, b);
            }
        }
    }

    /// source_id discipline: every looked-up aspect.source_id == SOURCE_HUYEN_KHONG.
    #[test]
    fn test_source_id_discipline() {
        for a in 1u8..=9 {
            for b in 1u8..=9 {
                let asp = lookup_star_pair_aspect(flying_star_from_u8(a), flying_star_from_u8(b));
                assert_eq!(
                    asp.source_id,
                    crate::sources::SOURCE_HUYEN_KHONG,
                    "source_id mismatch for ({},{})",
                    a,
                    b
                );
                assert!(
                    !asp.original_citation.title.is_empty(),
                    "empty citation title for ({},{})",
                    a,
                    b
                );
            }
        }
    }

    /// Order-sensitivity: (1,9) and (9,1) return rows with swapped star_a/star_b.
    #[test]
    fn test_order_sensitivity() {
        let asp_1_9 = lookup_star_pair_aspect(FlyingStar::NhatBach, FlyingStar::CuuTu);
        let asp_9_1 = lookup_star_pair_aspect(FlyingStar::CuuTu, FlyingStar::NhatBach);
        assert_eq!(asp_1_9.star_a, 1);
        assert_eq!(asp_1_9.star_b, 9);
        assert_eq!(asp_9_1.star_a, 9);
        assert_eq!(asp_9_1.star_b, 1);
    }

    /// compute_palace_aspects(2024, 1, &scanner) returns 9 aspects whose (star_a, star_b)
    /// match the combined overlay's palace_overlays.
    #[test]
    fn test_compute_palace_aspects_matches_combined_overlay() {
        let scanner = TietKhiScanner::new();
        let aspects = compute_palace_aspects(2024, 1, &scanner);
        assert_eq!(aspects.len(), 9);

        // Cross-check with the combined overlay directly.
        let overlay = crate::almanac::fengshui::combined::compute_combined_overlay(2024, 1, &scanner);
        for i in 0..9 {
            let (annual, monthly) = overlay.palace_overlays[i];
            assert_eq!(
                aspects[i].star_a,
                annual as u8,
                "palace {i} star_a mismatch"
            );
            assert_eq!(
                aspects[i].star_b,
                monthly as u8,
                "palace {i} star_b mismatch"
            );
        }
    }
}
