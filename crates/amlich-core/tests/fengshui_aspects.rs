//! Black-box integration tests for Phase 14 public API — external-consumer perspective.
//!
//! Tests cover:
//!   FS-11 — all 81 ordered pairs accessible; ordering is non-symmetric
//!   FS-12 — source_id discipline + non-empty citation title
//!   FS-13 — compute_palace_aspects output consistent with compute_combined_overlay
//!   FS-14 — is_danger_palace predicate truth table
//!   FS-15 — element_hint_for_palace returns Some for danger stars, None for auspicious
//!   Guard — no product/commercial terms in any hint_text_vi or aspect name corpus
//!
//! Imports via `use amlich_core::...` as an external consumer would.

use amlich_core::almanac::fengshui::{
    compute_combined_overlay, compute_palace_aspects, element_hint_for_palace, is_danger_palace,
    lookup_star_pair_aspect, TietKhiScanner,
};
use amlich_core::almanac::fengshui::types::FlyingStar;

/// All 9 FlyingStar variants in order for reuse across tests.
const ALL_STARS: [FlyingStar; 9] = [
    FlyingStar::NhatBach,
    FlyingStar::NhiHac,
    FlyingStar::TamBich,
    FlyingStar::TuLuc,
    FlyingStar::NguHoang,
    FlyingStar::LucBach,
    FlyingStar::ThatXich,
    FlyingStar::BatBach,
    FlyingStar::CuuTu,
];

// ---------------------------------------------------------------------------
// FS-11 — All 81 ordered pairs accessible; order-sensitive (not symmetric)
// ---------------------------------------------------------------------------

/// FS-11: loop star_a × star_b across all 9 FlyingStar variants;
/// assert returned aspect has star_a == a and star_b == b.
/// Assert that (NhatBach, CuuTu) and (CuuTu, NhatBach) differ in ordering.
#[test]
fn all_81_pairs_lookup_ordered() {
    for &a in &ALL_STARS {
        for &b in &ALL_STARS {
            let asp = lookup_star_pair_aspect(a, b);
            assert_eq!(
                asp.star_a,
                a as u8,
                "star_a mismatch for pair ({:?}, {:?}): expected {}, got {}",
                a,
                b,
                a as u8,
                asp.star_a
            );
            assert_eq!(
                asp.star_b,
                b as u8,
                "star_b mismatch for pair ({:?}, {:?}): expected {}, got {}",
                a,
                b,
                b as u8,
                asp.star_b
            );
        }
    }

    // Order-sensitivity: (NhatBach, CuuTu) and (CuuTu, NhatBach) are distinct rows
    let asp_1_9 = lookup_star_pair_aspect(FlyingStar::NhatBach, FlyingStar::CuuTu);
    let asp_9_1 = lookup_star_pair_aspect(FlyingStar::CuuTu, FlyingStar::NhatBach);
    assert_eq!(asp_1_9.star_a, 1, "NhatBach->CuuTu: star_a must be 1");
    assert_eq!(asp_1_9.star_b, 9, "NhatBach->CuuTu: star_b must be 9");
    assert_eq!(asp_9_1.star_a, 9, "CuuTu->NhatBach: star_a must be 9");
    assert_eq!(asp_9_1.star_b, 1, "CuuTu->NhatBach: star_b must be 1");
}

// ---------------------------------------------------------------------------
// FS-12 — source_id discipline and non-empty citation title
// ---------------------------------------------------------------------------

/// FS-12: for a sample of pairs, assert source_id == "huyen-khong" and non-empty
/// original_citation.title. Checks pairs (1,6), (6,1), (5,2), (2,5), (8,9), (9,8).
#[test]
fn aspect_provenance_discipline() {
    let sample_pairs = [
        (FlyingStar::NhatBach, FlyingStar::LucBach),
        (FlyingStar::LucBach, FlyingStar::NhatBach),
        (FlyingStar::NguHoang, FlyingStar::NhiHac),
        (FlyingStar::NhiHac, FlyingStar::NguHoang),
        (FlyingStar::BatBach, FlyingStar::CuuTu),
        (FlyingStar::CuuTu, FlyingStar::BatBach),
    ];

    for (a, b) in sample_pairs {
        let asp = lookup_star_pair_aspect(a, b);
        assert_eq!(
            asp.source_id,
            "huyen-khong",
            "pair ({:?},{:?}): expected source_id 'huyen-khong', got '{}'",
            a,
            b,
            asp.source_id
        );
        assert!(
            !asp.original_citation.title.is_empty(),
            "pair ({:?},{:?}): original_citation.title must not be empty",
            a,
            b
        );
    }
}

// ---------------------------------------------------------------------------
// FS-13 — compute_palace_aspects output consistent with compute_combined_overlay
// ---------------------------------------------------------------------------

/// FS-13: compute_palace_aspects(2024, 1, &scanner) returns 9 aspects whose
/// (star_a, star_b) match the combined overlay's palace_overlays for every palace.
#[test]
fn compute_palace_aspects_matches_overlay() {
    let scanner = TietKhiScanner::new();
    let overlay = compute_combined_overlay(2024, 1, &scanner);
    let aspects = compute_palace_aspects(2024, 1, &scanner);

    assert_eq!(
        aspects.len(),
        9,
        "compute_palace_aspects must return exactly 9 aspects, got {}",
        aspects.len()
    );

    for i in 0..9 {
        let (annual_star, monthly_star) = overlay.palace_overlays[i];
        assert_eq!(
            aspects[i].star_a,
            annual_star as u8,
            "palace {i}: aspects.star_a {} != overlay annual star {}",
            aspects[i].star_a,
            annual_star as u8
        );
        assert_eq!(
            aspects[i].star_b,
            monthly_star as u8,
            "palace {i}: aspects.star_b {} != overlay monthly star {}",
            aspects[i].star_b,
            monthly_star as u8
        );
    }
}

// ---------------------------------------------------------------------------
// FS-14 — is_danger_palace predicate truth table
// ---------------------------------------------------------------------------

/// FS-14: is_danger_palace is true exactly for NguHoang (5) and NhiHac (2).
/// Assert the remaining 7 stars all return false.
#[test]
fn danger_palace_predicate() {
    // Danger stars
    assert!(
        is_danger_palace(FlyingStar::NguHoang),
        "NguHoang (5) must be a danger palace"
    );
    assert!(
        is_danger_palace(FlyingStar::NhiHac),
        "NhiHac (2) must be a danger palace"
    );

    // Non-danger stars — all 7 remaining
    let non_danger = [
        FlyingStar::NhatBach,
        FlyingStar::TamBich,
        FlyingStar::TuLuc,
        FlyingStar::LucBach,
        FlyingStar::ThatXich,
        FlyingStar::BatBach,
        FlyingStar::CuuTu,
    ];
    for star in non_danger {
        assert!(
            !is_danger_palace(star),
            "star {:?} ({}) must NOT be a danger palace",
            star,
            star as u8
        );
    }
}

// ---------------------------------------------------------------------------
// FS-15 — element_hint_for_palace: Some for danger stars, None for auspicious
// ---------------------------------------------------------------------------

/// FS-15: element_hint_for_palace returns Some for NguHoang and NhiHac;
/// each RemedyHint has non-empty hint_text_vi, source_id == "huyen-khong",
/// non-empty citation title, and element in the allowed Ngu Hanh set.
/// element_hint_for_palace(NhatBach) returns None.
#[test]
fn element_hint_present_for_danger_stars() {
    let allowed_elements = ["kim", "mộc", "thủy", "hỏa", "thổ"];

    // NguHoang — must have a hint
    let hint_ngu_hoang = element_hint_for_palace(FlyingStar::NguHoang);
    assert!(
        hint_ngu_hoang.is_some(),
        "element_hint_for_palace(NguHoang) must return Some(RemedyHint)"
    );
    let h = hint_ngu_hoang.unwrap();
    assert!(
        !h.hint_text_vi.is_empty(),
        "NguHoang RemedyHint.hint_text_vi must not be empty"
    );
    assert_eq!(
        h.source_id,
        "huyen-khong",
        "NguHoang RemedyHint.source_id must be 'huyen-khong', got '{}'",
        h.source_id
    );
    assert!(
        !h.original_citation.title.is_empty(),
        "NguHoang RemedyHint.original_citation.title must not be empty"
    );
    assert!(
        allowed_elements.contains(&h.element.as_str()),
        "NguHoang RemedyHint.element '{}' must be in {:?}",
        h.element,
        allowed_elements
    );

    // NhiHac — must have a hint
    let hint_nhi_hac = element_hint_for_palace(FlyingStar::NhiHac);
    assert!(
        hint_nhi_hac.is_some(),
        "element_hint_for_palace(NhiHac) must return Some(RemedyHint)"
    );
    let h = hint_nhi_hac.unwrap();
    assert!(
        !h.hint_text_vi.is_empty(),
        "NhiHac RemedyHint.hint_text_vi must not be empty"
    );
    assert_eq!(
        h.source_id,
        "huyen-khong",
        "NhiHac RemedyHint.source_id must be 'huyen-khong', got '{}'",
        h.source_id
    );
    assert!(
        !h.original_citation.title.is_empty(),
        "NhiHac RemedyHint.original_citation.title must not be empty"
    );
    assert!(
        allowed_elements.contains(&h.element.as_str()),
        "NhiHac RemedyHint.element '{}' must be in {:?}",
        h.element,
        allowed_elements
    );

    // NhatBach — auspicious star must return None
    assert!(
        element_hint_for_palace(FlyingStar::NhatBach).is_none(),
        "element_hint_for_palace(NhatBach) must return None (auspicious star)"
    );
}

// ---------------------------------------------------------------------------
// No-product-names corpus guard (Pattern 9 — runtime corpus scan)
// ---------------------------------------------------------------------------

/// Corpus guard: no commercial or product-related terms may appear in any
/// RemedyHint.hint_text_vi or StarPairAspect.name in the runtime corpora.
///
/// This test is a standing CI regression gate — if any future corpus edit
/// introduces a commercial/product reference, this turns RED.
#[test]
fn no_product_names_in_corpora() {
    const FORBIDDEN_PRODUCT_TERMS: &[&str] = &[
        "đặt mua",
        "mua ngay",
        "click",
        "sản phẩm",
        "giá",
        "khuyến mãi",
        "http://",
        "https://",
        "www.",
    ];

    let mut violations: Vec<String> = Vec::new();

    // --- Walk every RemedyHint: all 9 FlyingStar variants ---
    for &star in &ALL_STARS {
        if let Some(hint) = element_hint_for_palace(star) {
            let lower = hint.hint_text_vi.to_lowercase();
            for &term in FORBIDDEN_PRODUCT_TERMS {
                if lower.contains(term) {
                    violations.push(format!(
                        "RemedyHint for star {:?} ({}) hint_text_vi contains forbidden term '{}': '{}'",
                        star,
                        star as u8,
                        term,
                        hint.hint_text_vi
                    ));
                }
            }
        }
    }

    // --- Walk every StarPairAspect name: all 81 ordered pairs ---
    for &a in &ALL_STARS {
        for &b in &ALL_STARS {
            let asp = lookup_star_pair_aspect(a, b);
            let lower = asp.name.to_lowercase();
            for &term in FORBIDDEN_PRODUCT_TERMS {
                if lower.contains(term) {
                    violations.push(format!(
                        "StarPairAspect ({},{}) name contains forbidden term '{}': '{}'",
                        asp.star_a,
                        asp.star_b,
                        term,
                        asp.name
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "no_product_names_in_corpora: {} violation(s) found:\n{}",
        violations.len(),
        violations.join("\n")
    );
}
