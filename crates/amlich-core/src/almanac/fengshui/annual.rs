//! Annual Phi Tinh — Niên Tử Bạch (FS-06).
//!
//! Implements `compute_yearly_flying_stars` which returns a 9-palace annual
//! flying-star layout.  The annual center star descends mod-9 from the
//! anchor (2024 → 4, Giáp Thìn, Hạ Nguyên — contemporary cross-checked value).
//!
//! Direction follows ADR-0003 (polarity matrix, not a bare bool flag):
//!   - Dương year (can_index even) → nghịch hành (descending, false in fill_palaces)
//!   - Âm year (can_index odd)  → thuận hành (ascending, true in fill_palaces)
//!
//! `fill_palaces` is `pub(crate)` so monthly.rs reuses the same implementation.
//!
//! Flying-path constant follows the Lo Shu Thuận walk:
//!   Center(4) → NW(5) → W(6) → NE(7) → S(8) → N(0) → SW(1) → E(2) → SE(3)
//! which is the same path used by the Van base-table construction in period.rs.

use crate::almanac::fengshui::{
    scanner::TietKhiScanner,
    stars::flying_star_from_u8,
    types::{FlyingStar, FlyingStarLayout, FlyingStarPeriod},
};
use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};
use crate::sources::SOURCE_HUYEN_KHONG;

// ---------------------------------------------------------------------------
// Year polarity (ADR-0003)
// ---------------------------------------------------------------------------

/// Year polarity derived from Thiên Can (Heavenly Stem) index.
///
/// ADR-0003 §3: dương stems are Giáp/Bính/Mậu/Canh/Nhâm (can_index 0/2/4/6/8 —
/// even 0-based).  Âm stems are Ất/Đinh/Kỷ/Tân/Quý (can_index 1/3/5/7/9 —
/// odd 0-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearPolarity {
    /// Dương year (even can_index): nghịch hành (descending).
    Duong,
    /// Âm year (odd can_index): thuận hành (ascending).
    Am,
}

/// Return the polarity of `year` from its Heavenly Stem index.
pub fn year_polarity(year: i32) -> YearPolarity {
    let cc = crate::canchi::get_year_canchi(year);
    if cc.can_index % 2 == 0 {
        YearPolarity::Duong
    } else {
        YearPolarity::Am
    }
}

/// Return `true` if `year` uses ascending (thuận) spiral fill.
///
/// Dương=descending(false), Âm=ascending(true).
/// Exported as `pub(crate)` so `monthly.rs` can import without duplication.
pub(crate) fn year_is_ascending(year: i32) -> bool {
    matches!(year_polarity(year), YearPolarity::Am)
}

// ---------------------------------------------------------------------------
// Annual center star (Niên Tử Bạch)
// ---------------------------------------------------------------------------

/// Compute the annual center star for `year`.
///
/// Anchor: nien_center(2024) == 4 (Giáp Thìn, Hạ Nguyên — contemporary
/// cross-checked value per ADR-0003).
///
/// The sequence descends by 1 per year (mod 9, wrapping 1→9):
///   2024→4, 2025→3, 2026→2, 2023→5, 2016→3 (for test coverage).
pub fn nien_center(year: i32) -> u8 {
    // Steps after 2024 — positive means future (descending: subtract steps).
    // rem_euclid ensures we always get a non-negative offset even for past years.
    // For year < 2024: offset = (2024-year) mod 9 means we ADD offset going back.
    // Equivalently: center = 4 - (year-2024) mod 9, wrapping in 1..=9.
    //
    // Using signed arithmetic carefully:
    //   delta = year - 2024
    //   raw = ((4 - 1 - delta.rem_euclid(9) as i32) % 9 + 9) % 9 + 1
    let delta = year - 2024;
    let offset = delta.rem_euclid(9) as i32; // 0..=8 for any direction
    // For delta=0 => offset=0 => raw = 4  ✓
    // For delta=1 => offset=1 => raw = 3  ✓ (2025)
    // For delta=-1 => offset=8 => raw = (4-1-8)%9+9)%9+1 = (-5%9+9)%9+1 = 4+1=5 ✓ (2023)
    let raw = ((4_i32 - 1 - offset).rem_euclid(9)) + 1;
    raw as u8
}

// ---------------------------------------------------------------------------
// Yuan annotation (for evidence confidence)
// ---------------------------------------------------------------------------

/// Return the Tam Nguyên label and whether confidence is MEDIUM.
///
/// - 1864–1983 → Thượng/Trung Nguyên → MEDIUM confidence (ADR-0003 §4).
/// - 1984–2043 → Hạ Nguyên → HIGH confidence.
fn yuan_of_year(year: i32) -> (&'static str, bool /* is_medium */) {
    if year < 1984 {
        ("pre-1984", true)
    } else {
        ("ha_nguyen", false)
    }
}

// ---------------------------------------------------------------------------
// Shared spiral-fill (Lo Shu flying path)
// ---------------------------------------------------------------------------

/// Lo Shu flying path — indices into a `[FlyingStar; 9]` palace array.
///
/// The array is indexed in `Palace::ALL` order:
///   index 0=N, 1=SW, 2=E, 3=SE, 4=Center, 5=NW, 6=W, 7=NE, 8=S
///
/// Thuận walk (ascending): Center → NW → W → NE → S → N → SW → E → SE
/// which visits palace indices [4, 5, 6, 7, 8, 0, 1, 2, 3] in order.
/// Starting position (index 4 = Center) receives the center star first, then
/// each subsequent position receives center ± 1 per step.
///
/// This is the same path baked into the Vận base tables in period.rs / JSON.
pub(crate) const FLYING_PATH: [usize; 9] = [4, 5, 6, 7, 8, 0, 1, 2, 3];

/// Fill the 9-palace array by spiraling from `center` along the Lo Shu path.
///
/// `ascending=true`  → each step adds 1 (mod 9, wrap 9→1) — thuận hành.
/// `ascending=false` → each step subtracts 1 (mod 9, wrap 1→9) — nghịch hành.
///
/// The returned array is indexed in `Palace::ALL` order.
pub(crate) fn fill_palaces(center: u8, ascending: bool) -> [FlyingStar; 9] {
    let mut palaces = [FlyingStar::NhatBach; 9];
    let mut current = center as i32;
    for &idx in &FLYING_PATH {
        // Clamp current into 1..=9
        let star_n = (((current - 1).rem_euclid(9)) + 1) as u8;
        palaces[idx] = flying_star_from_u8(star_n);
        if ascending {
            current += 1;
        } else {
            current -= 1;
        }
    }
    palaces
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute the 9-palace annual Phi Tinh layout for `year`.
///
/// The `_scanner` parameter is part of the FS-06 API signature for symmetry
/// with `compute_period` and future jd-based overloads.  The Niên year
/// boundary is implicit (integer `year` passed by caller).
///
/// # Evidence
/// - method: "phi_tinh.nien"
/// - note: "year={year};center={center};polarity={polarity:?};confidence=high|medium"
pub fn compute_yearly_flying_stars(year: i32, _scanner: &TietKhiScanner) -> FlyingStarLayout {
    let center = nien_center(year);
    let polarity = year_polarity(year);
    let ascending = matches!(polarity, YearPolarity::Am);
    let palaces = fill_palaces(center, ascending);

    let (_yuan_label, is_medium) = yuan_of_year(year);
    let conf = if is_medium {
        "confidence=medium"
    } else {
        "confidence=high"
    };
    let note = format!("year={year};center={center};polarity={polarity:?};{conf}");

    let evidence = ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: SOURCE_HUYEN_KHONG.to_string(),
        method: "phi_tinh.nien".to_string(),
        note: Some(note),
    };

    FlyingStarLayout {
        period: FlyingStarPeriod::Yearly { year },
        palaces,
        center_star: flying_star_from_u8(center),
        evidence,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn scanner() -> TietKhiScanner {
        TietKhiScanner::new()
    }

    // -----------------------------------------------------------------------
    // year_polarity
    // -----------------------------------------------------------------------

    /// Giáp year (can_index 0, even) => Duong.
    #[test]
    fn test_year_polarity_2024_duong() {
        assert_eq!(year_polarity(2024), YearPolarity::Duong,
            "2024 is Giáp Thìn, can_index 0 (even) => Duong");
    }

    /// Ất year (can_index 1, odd) => Am.
    #[test]
    fn test_year_polarity_2025_am() {
        assert_eq!(year_polarity(2025), YearPolarity::Am,
            "2025 is Ất Tỵ, can_index 1 (odd) => Am");
    }

    /// year_is_ascending: 2024 => false (Duong = descending), 2025 => true.
    #[test]
    fn test_year_is_ascending() {
        assert!(!year_is_ascending(2024), "2024 Duong => not ascending");
        assert!(year_is_ascending(2025), "2025 Am => ascending");
    }

    // -----------------------------------------------------------------------
    // nien_center
    // -----------------------------------------------------------------------

    /// Anchor: 2024 => 4.
    #[test]
    fn test_nien_center_2024_is_4() {
        assert_eq!(nien_center(2024), 4);
    }

    /// 2025 => 3 (descend by 1).
    #[test]
    fn test_nien_center_2025_is_3() {
        assert_eq!(nien_center(2025), 3);
    }

    /// 2026 => 2.
    #[test]
    fn test_nien_center_2026_is_2() {
        assert_eq!(nien_center(2026), 2);
    }

    /// 2023 => 5 (go back one from 4, ascending past 2024).
    #[test]
    fn test_nien_center_2023_is_5() {
        assert_eq!(nien_center(2023), 5);
    }

    /// Wrap: 2032 should be 2024+8 => 4-8 mod9 => check: delta=8, offset=8,
    /// raw = ((4-1-8) rem_euclid 9)+1 = ((-5) rem_euclid 9)+1 = 4+1=5... wait
    /// Actually 2032 = 2024+8: center should descend 8 steps: 4,3,2,1,9,8,7,6,5 => 5 for 2031? No:
    /// 2025=3, 2026=2, 2027=1, 2028=9 (wrap), 2029=8, 2030=7, 2031=6, 2032=5.
    #[test]
    fn test_nien_center_2032_is_5() {
        assert_eq!(nien_center(2032), 5);
    }

    /// 2033 = 2024+9 => mod9=0 => same as 2024 => 4.
    #[test]
    fn test_nien_center_cycle_of_9() {
        // 9-year cycle: 2024 and 2033 should give same center
        assert_eq!(nien_center(2024), nien_center(2033));
    }

    // -----------------------------------------------------------------------
    // fill_palaces
    // -----------------------------------------------------------------------

    /// fill_palaces always produces a permutation of 1..=9.
    #[test]
    fn test_fill_palaces_is_permutation() {
        for center in 1u8..=9 {
            for &ascending in &[true, false] {
                let palaces = fill_palaces(center, ascending);
                let mut seen = [false; 10];
                for &s in &palaces {
                    let n = s as u8;
                    assert!(n >= 1 && n <= 9, "star {n} out of range for center={center}");
                    assert!(!seen[n as usize], "duplicate star {n} for center={center}, ascending={ascending}");
                    seen[n as usize] = true;
                }
                // Center palace is at index 4 in Palace::ALL order.
                assert_eq!(palaces[4] as u8, center,
                    "center palace mismatch: expected {center}, got {:?}", palaces[4]);
            }
        }
    }

    // -----------------------------------------------------------------------
    // compute_yearly_flying_stars
    // -----------------------------------------------------------------------

    /// 2024 annual: center=4, period=Yearly{2024}, palaces permutation.
    #[test]
    fn test_compute_yearly_2024_center_is_4() {
        let layout = compute_yearly_flying_stars(2024, &scanner());
        assert_eq!(layout.center_star as u8, 4,
            "2024 annual center should be 4 (Tứ Lục)");
        assert_eq!(layout.palaces.len(), 9);
        if let FlyingStarPeriod::Yearly { year } = layout.period {
            assert_eq!(year, 2024);
        } else {
            panic!("Expected Yearly period");
        }
    }

    /// 2025 annual: center=3.
    #[test]
    fn test_compute_yearly_2025_center_is_3() {
        let layout = compute_yearly_flying_stars(2025, &scanner());
        assert_eq!(layout.center_star as u8, 3);
    }

    /// Palaces are a permutation of 1..=9 for 2024 (Duong, descending).
    #[test]
    fn test_compute_yearly_2024_palaces_permutation() {
        let layout = compute_yearly_flying_stars(2024, &scanner());
        let mut seen = [false; 10];
        for &s in &layout.palaces {
            let n = s as u8;
            assert!(n >= 1 && n <= 9);
            assert!(!seen[n as usize], "duplicate star {n}");
            seen[n as usize] = true;
        }
    }

    /// Evidence method is "phi_tinh.nien".
    #[test]
    fn test_compute_yearly_evidence_method() {
        let layout = compute_yearly_flying_stars(2024, &scanner());
        assert_eq!(layout.evidence.method, "phi_tinh.nien");
        assert_eq!(layout.evidence.source_id, crate::sources::SOURCE_HUYEN_KHONG);
    }

    /// Pre-1984 year (e.g. 1960) has "confidence=medium" in evidence note.
    #[test]
    fn test_compute_yearly_pre_1984_medium_confidence() {
        let layout = compute_yearly_flying_stars(1960, &scanner());
        let note = layout.evidence.note.as_deref().unwrap_or("");
        assert!(note.contains("confidence=medium"),
            "Pre-1984 evidence should contain 'confidence=medium', got: {note:?}");
    }

    /// Post-1984 year (2024) has "confidence=high" in evidence note.
    #[test]
    fn test_compute_yearly_post_1984_high_confidence() {
        let layout = compute_yearly_flying_stars(2024, &scanner());
        let note = layout.evidence.note.as_deref().unwrap_or("");
        assert!(note.contains("confidence=high"),
            "Post-1984 evidence should contain 'confidence=high', got: {note:?}");
    }
}
