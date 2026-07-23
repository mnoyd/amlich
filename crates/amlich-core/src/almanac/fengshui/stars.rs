//! FlyingStar metadata loader.
//!
//! Loads star metadata (element / polarity / auspice) from
//! `data/almanac/flying_stars.json` via `OnceLock` + `include_str!`.
//! Validates that exactly 9 stars are present with numbers 1..=9 each
//! appearing once.

use std::sync::OnceLock;

use serde::Deserialize;

use crate::almanac::fengshui::types::FlyingStar;

// Path depth: fengshui/ -> almanac/ -> src/ -> crate root -> data/
const FLYING_STARS_JSON: &str = include_str!("../../../data/almanac/flying_stars.json");

static FLYING_STARS_META: OnceLock<FlyingStarsMeta> = OnceLock::new();

// ---------------------------------------------------------------------------
// Internal deserialization types
// ---------------------------------------------------------------------------

/// Metadata for a single flying star (private — only the loader reads this).
#[derive(Debug, Deserialize)]
pub struct StarMeta {
    pub number: u8,
    pub slug: String,
    pub name_vi: String,
    pub element: String,
    pub polarity: String,
    pub auspice: String,
}

/// JSON envelope wrapping all nine `StarMeta` rows.
#[derive(Debug, Deserialize)]
struct FlyingStarsMeta {
    stars: Vec<StarMeta>,
}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

fn load_stars_inner() -> FlyingStarsMeta {
    let meta: FlyingStarsMeta =
        serde_json::from_str(FLYING_STARS_JSON).expect("Failed to parse flying_stars.json");

    // Validate: exactly 9 stars, numbers 1..=9 each once.
    assert_eq!(
        meta.stars.len(),
        9,
        "flying_stars.json must contain exactly 9 star entries, got {}",
        meta.stars.len()
    );
    let mut seen = [false; 10]; // index 1-9
    for star in &meta.stars {
        let n = star.number;
        assert!(
            (1..=9).contains(&n),
            "star number {n} is out of range 1..=9"
        );
        assert!(
            !seen[n as usize],
            "star number {n} appears more than once in flying_stars.json"
        );
        seen[n as usize] = true;
    }
    for (n, &present) in seen.iter().enumerate().skip(1) {
        assert!(present, "star number {n} is missing from flying_stars.json");
    }

    meta
}

fn stars_meta() -> &'static FlyingStarsMeta {
    FLYING_STARS_META.get_or_init(load_stars_inner)
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Return the classical metadata for `star`.
///
/// The returned reference is `'static` (backed by `OnceLock`).
pub fn star_metadata(star: FlyingStar) -> &'static StarMeta {
    let n = star as u8;
    stars_meta()
        .stars
        .iter()
        .find(|s| s.number == n)
        .unwrap_or_else(|| panic!("star number {n} not found — flying_stars.json invariant broken"))
}

/// Convert a raw star number (1..=9) to a `FlyingStar` variant.
///
/// Panics on out-of-range input (0 or ≥10).
/// Used by base-table loaders that store `u8` in JSON.
pub fn flying_star_from_u8(n: u8) -> FlyingStar {
    match n {
        1 => FlyingStar::NhatBach,
        2 => FlyingStar::NhiHac,
        3 => FlyingStar::TamBich,
        4 => FlyingStar::TuLuc,
        5 => FlyingStar::NguHoang,
        6 => FlyingStar::LucBach,
        7 => FlyingStar::ThatXich,
        8 => FlyingStar::BatBach,
        9 => FlyingStar::CuuTu,
        _ => panic!("flying_star_from_u8: {n} is out of range 1..=9"),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// star_metadata(NguHoang) returns element="earth", polarity="neutral", auspice="inauspicious".
    #[test]
    fn test_ngu_hoang_metadata() {
        let meta = star_metadata(FlyingStar::NguHoang);
        assert_eq!(meta.number, 5);
        assert_eq!(meta.slug, "ngu_hoang");
        assert_eq!(meta.element, "earth");
        assert_eq!(meta.polarity, "neutral");
        assert_eq!(meta.auspice, "inauspicious");
    }

    /// All 9 star numbers 1..=9 are accessible via star_metadata.
    #[test]
    fn test_all_9_stars_present() {
        let stars = [
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
        for star in stars {
            let n = star as u8;
            let meta = star_metadata(star);
            assert_eq!(meta.number, n, "number mismatch for star {n}");
            assert!(!meta.element.is_empty(), "empty element for star {n}");
            assert!(!meta.polarity.is_empty(), "empty polarity for star {n}");
            assert!(!meta.auspice.is_empty(), "empty auspice for star {n}");
        }
    }

    /// flying_star_from_u8 maps 5 -> NguHoang.
    #[test]
    fn test_flying_star_from_u8_five() {
        assert_eq!(flying_star_from_u8(5), FlyingStar::NguHoang);
    }

    /// flying_star_from_u8 round-trips for all 1..=9.
    #[test]
    fn test_flying_star_from_u8_round_trip() {
        for n in 1u8..=9 {
            let star = flying_star_from_u8(n);
            assert_eq!(star as u8, n, "round-trip failed for n={n}");
        }
    }

    /// flying_star_from_u8(0) panics.
    #[test]
    #[should_panic(expected = "out of range")]
    fn test_flying_star_from_u8_zero_panics() {
        flying_star_from_u8(0);
    }

    /// flying_star_from_u8(10) panics.
    #[test]
    #[should_panic(expected = "out of range")]
    fn test_flying_star_from_u8_ten_panics() {
        flying_star_from_u8(10);
    }
}
