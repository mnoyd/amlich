//! Phi Tinh advisory safety surface — danger-palace predicate + Ngũ-Hành element hints
//! (FS-14, FS-15).
//!
//! # Classical background
//!
//! In Huyền Không (玄空) Phi Tinh tradition, stars 2 (Nhị Hắc, 二黑) and 5 (Ngũ Hoàng, 五黃)
//! are the two principal danger stars — inauspicious earth stars that require Ngũ-Hành
//! elemental mitigation. Stars 3 and 7 are classically inauspicious but less severe.
//! Stars 1, 4, 6, 8, 9 are considered auspicious; no classical mitigation is prescribed.
//!
//! # Public API
//!
//! - [`is_danger_palace`] — predicate: true exactly for stars 5 and 2 (FS-14).
//! - [`element_hint_for_palace`] — returns `Some(RemedyHint)` for inauspicious stars
//!   that have a classical Ngũ-Hành mitigation; `None` for auspicious stars (FS-15).
//! - [`RemedyHint`] — classical mitigation hint with element, Vietnamese advisory text,
//!   source_id, and bibliographic citation (no product names).
//!
//! # Source: Thẩm Thị Huyền Không Học
//!
//! All hints carry `source_id = SOURCE_HUYEN_KHONG` and a non-empty citation title.
//! The JSON corpus is at `data/almanac/flying_stars_safety.json`.

use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::almanac::fengshui::aspects::FsCitation;
use crate::almanac::fengshui::types::FlyingStar;

// ---------------------------------------------------------------------------
// Corpus asset (embedded at compile time)
// ---------------------------------------------------------------------------

// Path depth: fengshui/ -> almanac/ -> src/ -> crate root -> data/
const SAFETY_JSON: &str = include_str!("../../../data/almanac/flying_stars_safety.json");

static SAFETY_CORPUS: OnceLock<SafetyCorpus> = OnceLock::new();

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Classical Ngũ-Hành mitigation hint for an inauspicious Phi Tinh palace star.
///
/// `element` is one of: "kim" | "mộc" | "thủy" | "hỏa" | "thổ"
/// `hint_text_vi` is a Vietnamese classical advisory referencing element CATEGORIES
/// only — it MUST NOT contain product names, brand names, prices, or commercial
/// calls-to-action (enforced by the no-product-names test in plan 14-03).
///
/// Reuses [`FsCitation`] from `aspects.rs` (intra-pillar reuse — both are within
/// the `fengshui` module; this is NOT the forbidden cross-pillar rituals coupling
/// described in PITFALLS Pitfall 4).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemedyHint {
    /// Mitigating Ngũ-Hành element. One of: "kim" | "mộc" | "thủy" | "hỏa" | "thổ".
    pub element: String,
    /// Classical Ngũ-Hành advisory in Vietnamese. No product/brand names.
    pub hint_text_vi: String,
    /// Canonical source ID. Always equals `SOURCE_HUYEN_KHONG`.
    pub source_id: String,
    /// Bibliographic citation (title required non-empty).
    pub original_citation: FsCitation,
}

// ---------------------------------------------------------------------------
// Internal deserialization shapes
// ---------------------------------------------------------------------------

/// Internal row struct used during deserialization. The public `RemedyHint` type
/// does not expose `star` (lookup is keyed by the function argument, not stored).
#[derive(Debug, Deserialize)]
struct SafetyHintRow {
    star: u8,
    element: String,
    hint_text_vi: String,
    source_id: String,
    original_citation: FsCitation,
}

/// Top-level corpus envelope from `flying_stars_safety.json`.
#[derive(Debug, Deserialize)]
struct SafetyCorpus {
    #[allow(dead_code)]
    schema_version: String,
    #[allow(dead_code)]
    source: String,
    hints: Vec<SafetyHintRow>,
}

// ---------------------------------------------------------------------------
// Allowed Ngũ-Hành element values
// ---------------------------------------------------------------------------

const ALLOWED_ELEMENTS: &[&str] = &["kim", "mộc", "thủy", "hỏa", "thổ"];

// ---------------------------------------------------------------------------
// Loader + validator (mirrors stars.rs and aspects.rs discipline)
// ---------------------------------------------------------------------------

fn load_safety_inner() -> SafetyCorpus {
    let corpus: SafetyCorpus =
        serde_json::from_str(SAFETY_JSON).expect("Failed to parse flying_stars_safety.json");
    validate_safety_corpus(&corpus);
    corpus
}

fn validate_safety_corpus(c: &SafetyCorpus) {
    let mut seen = [false; 10usize]; // index by star number 1..=9

    for row in &c.hints {
        assert!(
            row.star >= 1 && row.star <= 9,
            "flying_stars_safety.json: star={} is out of range 1..=9",
            row.star
        );
        assert!(
            !seen[row.star as usize],
            "flying_stars_safety.json: duplicate star={} entry",
            row.star
        );
        seen[row.star as usize] = true;

        assert_eq!(
            row.source_id,
            crate::sources::SOURCE_HUYEN_KHONG,
            "flying_stars_safety.json: star={} has source_id {:?}, expected SOURCE_HUYEN_KHONG",
            row.star,
            row.source_id
        );

        assert!(
            !row.original_citation.title.is_empty(),
            "flying_stars_safety.json: star={} has empty original_citation.title",
            row.star
        );

        assert!(
            ALLOWED_ELEMENTS.contains(&row.element.as_str()),
            "flying_stars_safety.json: star={} has element {:?}, must be one of {:?}",
            row.star,
            row.element,
            ALLOWED_ELEMENTS
        );

        assert!(
            !row.hint_text_vi.is_empty(),
            "flying_stars_safety.json: star={} has empty hint_text_vi",
            row.star
        );
    }

    assert_eq!(
        c.schema_version.is_empty(),
        false,
        "schema_version must be non-empty in flying_stars_safety.json"
    );
}

fn safety_corpus() -> &'static SafetyCorpus {
    SAFETY_CORPUS.get_or_init(load_safety_inner)
}

// ---------------------------------------------------------------------------
// Public API (FS-14, FS-15)
// ---------------------------------------------------------------------------

/// Return `true` if `star` is one of the two principal danger stars in classical
/// Huyền Không Phi Tinh (FS-14).
///
/// Per *Thẩm Thị Huyền Không Học* and the wider Huyền Không tradition, stars 5 (Ngũ Hoàng,
/// 五黃) and 2 (Nhị Hắc, 二黑) are the two principal earth danger stars. Star 5 is the most
/// malignant; star 2 brings sickness and obstruction. Neither has any auspicious application
/// in the classical texts.
///
/// Returns `false` for all other seven stars (1, 3, 4, 6, 7, 8, 9).
pub fn is_danger_palace(star: FlyingStar) -> bool {
    matches!(star, FlyingStar::NguHoang | FlyingStar::NhiHac)
}

/// Return the classical Ngũ-Hành mitigation hint for `star`, if one exists (FS-15).
///
/// Returns `Some(RemedyHint)` for inauspicious stars that have a classical elemental
/// mitigation entry in `flying_stars_safety.json`. Returns `None` for auspicious stars
/// (1 Nhất Bạch, 4 Tứ Lục, 6 Lục Bạch, 8 Bát Bạch, 9 Cửu Tử) that require no mitigation.
///
/// The `RemedyHint` carries:
/// - `element` — the mitigating Ngũ-Hành element.
/// - `hint_text_vi` — a Vietnamese classical advisory (no product names).
/// - `source_id` — always `"huyen-khong"` (via `SOURCE_HUYEN_KHONG` constant).
/// - `original_citation` — bibliographic citation with non-empty `title`.
pub fn element_hint_for_palace(star: FlyingStar) -> Option<RemedyHint> {
    let star_num = star as u8;
    safety_corpus().hints.iter().find(|row| row.star == star_num).map(|row| RemedyHint {
        element: row.element.clone(),
        hint_text_vi: row.hint_text_vi.clone(),
        source_id: row.source_id.clone(),
        original_citation: row.original_citation.clone(),
    })
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::fengshui::types::FlyingStar;

    // --- is_danger_palace truth table ---

    /// Stars 5 and 2 are the danger stars: is_danger_palace returns true for them.
    #[test]
    fn test_is_danger_palace_true_for_ngu_hoang_and_nhi_hac() {
        assert!(is_danger_palace(FlyingStar::NguHoang), "NguHoang (5) must be danger");
        assert!(is_danger_palace(FlyingStar::NhiHac), "NhiHac (2) must be danger");
    }

    /// The remaining 7 stars are NOT danger stars.
    #[test]
    fn test_is_danger_palace_false_for_other_seven_stars() {
        assert!(!is_danger_palace(FlyingStar::NhatBach), "NhatBach (1) is not danger");
        assert!(!is_danger_palace(FlyingStar::TamBich), "TamBich (3) is not danger");
        assert!(!is_danger_palace(FlyingStar::TuLuc), "TuLuc (4) is not danger");
        assert!(!is_danger_palace(FlyingStar::LucBach), "LucBach (6) is not danger");
        assert!(!is_danger_palace(FlyingStar::ThatXich), "ThatXich (7) is not danger");
        assert!(!is_danger_palace(FlyingStar::BatBach), "BatBach (8) is not danger");
        assert!(!is_danger_palace(FlyingStar::CuuTu), "CuuTu (9) is not danger");
    }

    // --- element_hint_for_palace: load-dependent (RED until Task 2 lands JSON) ---

    /// NguHoang (5) and NhiHac (2) — both danger stars — must have a hint.
    #[test]
    fn test_element_hint_some_for_danger_stars() {
        let hint_5 = element_hint_for_palace(FlyingStar::NguHoang);
        assert!(hint_5.is_some(), "NguHoang (5) must have a RemedyHint");
        let hint_2 = element_hint_for_palace(FlyingStar::NhiHac);
        assert!(hint_2.is_some(), "NhiHac (2) must have a RemedyHint");
    }

    /// NhatBach (1) is auspicious — no mitigation hint.
    #[test]
    fn test_element_hint_none_for_nhat_bach() {
        assert!(
            element_hint_for_palace(FlyingStar::NhatBach).is_none(),
            "NhatBach (1) is auspicious and must return None"
        );
    }

    /// Every loaded RemedyHint carries source_id == SOURCE_HUYEN_KHONG and a non-empty title.
    #[test]
    fn test_loaded_hints_carry_source_id_and_non_empty_title() {
        for star_num in 1u8..=9 {
            let star = crate::almanac::fengshui::stars::flying_star_from_u8(star_num);
            if let Some(hint) = element_hint_for_palace(star) {
                assert_eq!(
                    hint.source_id,
                    crate::sources::SOURCE_HUYEN_KHONG,
                    "star={} hint.source_id must equal SOURCE_HUYEN_KHONG",
                    star_num
                );
                assert!(
                    !hint.original_citation.title.is_empty(),
                    "star={} hint.original_citation.title must be non-empty",
                    star_num
                );
                assert!(
                    !hint.hint_text_vi.is_empty(),
                    "star={} hint.hint_text_vi must be non-empty",
                    star_num
                );
                assert!(
                    ALLOWED_ELEMENTS.contains(&hint.element.as_str()),
                    "star={} hint.element {:?} must be in allowed set",
                    star_num,
                    hint.element
                );
            }
        }
    }

    /// RemedyHint serde round-trip with deny_unknown_fields.
    #[test]
    fn test_remedy_hint_deny_unknown_fields() {
        let source_val = crate::sources::SOURCE_HUYEN_KHONG;
        let bad_json = format!(
            r#"{{
            "element": "kim",
            "hint_text_vi": "Dùng vật phẩm thuộc hành Kim",
            "source_id": "{source_val}",
            "original_citation": {{"title": "Test Source"}},
            "unexpected_field": "should_fail"
        }}"#
        );
        let result: Result<RemedyHint, _> = serde_json::from_str(&bad_json);
        assert!(result.is_err(), "deny_unknown_fields should reject extra field");
    }
}
