//! compute_period, Period type, base palace table loader, and Lo Shu validator.
//!
//! # Design
//!
//! `compute_period` uses the Tiết Khí scanner to find the exact Lập Xuân
//! Julian Day for the year containing the input JD.  A JD that falls
//! *before* Lập Xuân of its calendar year is still in the *previous*
//! solar-Vận year — this is the CRIT-2 fix.  Never use `year >= 2024`.
//!
//! # Vận Tam Nguyên ranges (1864–2043, anchored at Lập Xuân)
//!
//! | Vận | Solar years (effective from Lập Xuân) |
//! |-----|---------------------------------------|
//! |  1  | 1864–1883 |
//! |  2  | 1884–1903 |
//! |  3  | 1904–1923 |
//! |  4  | 1924–1943 |
//! |  5  | 1944–1963 |
//! |  6  | 1964–1983 |
//! |  7  | 1984–2003 |
//! |  8  | 2004–2023 |
//! |  9  | 2024–2043 |

use std::sync::OnceLock;

use serde::Deserialize;

use crate::almanac::fengshui::{
    stars::flying_star_from_u8,
    types::{FlyingStar, FlyingStarLayout, FlyingStarPeriod},
};
use crate::julian::{jd_from_date, jd_to_date};
use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};
use crate::sources::SOURCE_HUYEN_KHONG;

use super::scanner::TietKhiScanner;

// ---------------------------------------------------------------------------
// JSON loader types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct VanTable {
    van: u8,
    palaces: [u8; 9],
}

#[derive(Debug, Deserialize)]
pub struct FlyingStarsBaseTable {
    tables: Vec<VanTable>,
}

const FLYING_STARS_BASE_JSON: &str =
    include_str!("../../../data/almanac/flying_stars_base.json");

static FLYING_STARS_BASE: OnceLock<FlyingStarsBaseTable> = OnceLock::new();

// ---------------------------------------------------------------------------
// Lo Shu validator
// ---------------------------------------------------------------------------

/// Validate that a palace array satisfies Lo Shu invariants for `van`:
/// - sum of all 9 values == 45
/// - each value 1..=9 appears exactly once
/// - center palace (index 4) == van number
///
/// Panics immediately on any violation — used at load time so a typo in
/// the JSON is caught during startup / test run (PITFALLS CRIT-4).
pub fn validate_van_table(van: u8, palaces: &[u8; 9]) {
    // Sum invariant
    let sum: u32 = palaces.iter().map(|&v| v as u32).sum();
    assert_eq!(
        sum, 45,
        "Lo Shu sum violation for Vận {van}: expected 45, got {sum}"
    );

    // Each 1..=9 exactly once
    let mut seen = [false; 10];
    for &v in palaces {
        assert!(
            v >= 1 && v <= 9,
            "Lo Shu range violation for Vận {van}: value {v} is outside 1..=9"
        );
        assert!(
            !seen[v as usize],
            "Lo Shu duplicate violation for Vận {van}: value {v} appears more than once"
        );
        seen[v as usize] = true;
    }

    // Center palace (index 4 in Palace::ALL order) must == van
    assert_eq!(
        palaces[4], van,
        "Lo Shu center violation for Vận {van}: center palace is {}, expected {van}",
        palaces[4]
    );
}

// ---------------------------------------------------------------------------
// Loader
// ---------------------------------------------------------------------------

/// Load the base palace table, validating every Vận row at first access.
///
/// Returns a `&'static` reference backed by `OnceLock`.
pub fn load_flying_stars_base() -> &'static FlyingStarsBaseTable {
    FLYING_STARS_BASE.get_or_init(|| {
        let table: FlyingStarsBaseTable =
            serde_json::from_str(FLYING_STARS_BASE_JSON)
                .expect("Failed to parse flying_stars_base.json");

        // Validate all 9 Vận tables.
        assert_eq!(
            table.tables.len(),
            9,
            "flying_stars_base.json must contain exactly 9 Vận tables, got {}",
            table.tables.len()
        );
        for row in &table.tables {
            validate_van_table(row.van, &row.palaces);
        }

        table
    })
}

/// Return the base palace layout for `van` as an array of `FlyingStar`.
///
/// The array is indexed in `Palace::ALL` order: [N, SW, E, SE, Center, NW, W, NE, S].
pub fn base_palaces_for_van(van: u8) -> [FlyingStar; 9] {
    let base = load_flying_stars_base();
    let row = base
        .tables
        .iter()
        .find(|r| r.van == van)
        .unwrap_or_else(|| panic!("No base table for Vận {van}"));
    row.palaces.map(flying_star_from_u8)
}

// ---------------------------------------------------------------------------
// Vận period computation
// ---------------------------------------------------------------------------

/// Return the Vận number for a solar year that has already passed Lập Xuân.
///
/// Uses the Tam Nguyên 20-year cycle anchored at 1864.
/// Supported range: 1864–2043 (Vận 1–9).  Values outside this range are
/// clamped to 1 or 9 respectively.
pub fn van_for_solar_year_after_lap_xuan(y: i32) -> u8 {
    // CRIT-2: caller guarantees the JD is on/after Lập Xuân for `y`.
    // Formula: Vận = floor((y - 1864) / 20) + 1, clamped 1..=9.
    let van_raw = (y - 1864) / 20 + 1;
    van_raw.clamp(1, 9) as u8
}

/// The Vận-period context for a given instant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    /// Active Vận number (1–9).
    pub van: u8,
    /// Effective solar year (may differ from calendar year for dates before Lập Xuân).
    pub year: i32,
    /// Julian Day Number of that year's Lập Xuân.
    pub lap_xuan_jd: i32,
}

impl Period {
    /// Build the base `FlyingStarLayout` for this Vận.
    pub fn base_layout(&self) -> FlyingStarLayout {
        FlyingStarLayout {
            period: FlyingStarPeriod::Van { van: self.van },
            palaces: base_palaces_for_van(self.van),
            center_star: flying_star_from_u8(self.van),
            evidence: van_evidence(self.van),
        }
    }
}

fn van_evidence(van: u8) -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: SOURCE_HUYEN_KHONG.to_string(),
        method: "phi_tinh.van".to_string(),
        note: Some(format!("van={van}")),
    }
}

/// Compute the `Period` (Vận) active at Julian Day `jd`.
///
/// # CRIT-2 fix
///
/// A naive `year >= 2024` check would be wrong for dates in Jan/early Feb of
/// a transition year.  Instead, we find the Lập Xuân JD for the calendar
/// year of `jd`:
/// - If `jd < lap_xuan`, the effective Vận year is `year - 1`.
/// - If `jd >= lap_xuan`, the effective Vận year is `year`.
pub fn compute_period(jd: i32, scanner: &TietKhiScanner) -> Period {
    let (_, _, year) = jd_to_date(jd);
    let lap_xuan = scanner.lap_xuan_jd(year);
    let effective_year = if jd < lap_xuan { year - 1 } else { year };
    let van = van_for_solar_year_after_lap_xuan(effective_year);
    Period {
        van,
        year: effective_year,
        lap_xuan_jd: lap_xuan,
    }
}

/// Compute the `Period` active for most of a calendar year.
///
/// Uses July 1 as the anchor (unambiguously after Lập Xuân, before year-end).
/// Matches the FS-01 spec signature `compute_period(year, scanner)`.
pub fn compute_period_for_year(year: i32, scanner: &TietKhiScanner) -> Period {
    compute_period(jd_from_date(1, 7, year), scanner)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn default_scanner() -> TietKhiScanner {
        TietKhiScanner::new()
    }

    /// validate_van_table panics when center (index 4) != van number.
    #[test]
    #[should_panic(expected = "center violation")]
    fn test_validate_van_table_wrong_center_panics() {
        // A valid Lo Shu row except center is 9 instead of 8.
        // Row sum = 4+5+6+7+9+9+1+2+3 -- this won't sum to 45 either.
        // Use a row that sums to 45, has 1-9, but wrong center.
        // Swap center and NW: palaces[4]=9, palaces[5]=8 (center=9 != van=8).
        let palaces: [u8; 9] = [4, 5, 6, 7, 9, 8, 1, 2, 3]; // center is 9, not 8
        validate_van_table(8, &palaces);
    }

    /// validate_van_table panics when sum != 45.
    #[test]
    #[should_panic(expected = "sum violation")]
    fn test_validate_van_table_wrong_sum_panics() {
        let palaces: [u8; 9] = [1, 2, 3, 4, 8, 9, 1, 2, 3]; // duplicates, wrong sum
        validate_van_table(8, &palaces);
    }

    /// All 9 Vận base tables load and pass Lo Shu invariants.
    #[test]
    fn test_load_flying_stars_base_all_valid() {
        let base = load_flying_stars_base();
        assert_eq!(base.tables.len(), 9, "expected 9 Vận tables");
        let mut van_set = [false; 10];
        for row in &base.tables {
            assert!(row.van >= 1 && row.van <= 9, "van {} out of range", row.van);
            assert!(!van_set[row.van as usize], "duplicate van {}", row.van);
            van_set[row.van as usize] = true;
            // Invariants already checked in loader; this is belt-and-suspenders.
            validate_van_table(row.van, &row.palaces);
        }
    }

    /// compute_period before Lập Xuân 2024 → Vận 8.
    ///
    /// Jan 15, 2024 is before Lập Xuân (approx Feb 4, 2024).
    #[test]
    fn test_compute_period_before_lap_xuan_2024_is_van_8() {
        let scanner = default_scanner();
        let jd = jd_from_date(15, 1, 2024);
        let period = compute_period(jd, &scanner);
        assert_eq!(
            period.van, 8,
            "Jan 15 2024 is before Lập Xuân → should be Vận 8, got {}",
            period.van
        );
    }

    /// compute_period after Lập Xuân 2024 → Vận 9.
    ///
    /// Feb 5, 2024 is after Lập Xuân (approx Feb 4, 2024).
    #[test]
    fn test_compute_period_after_lap_xuan_2024_is_van_9() {
        let scanner = default_scanner();
        let jd = jd_from_date(5, 2, 2024);
        let period = compute_period(jd, &scanner);
        assert_eq!(
            period.van, 9,
            "Feb 5 2024 is after Lập Xuân → should be Vận 9, got {}",
            period.van
        );
    }

    /// Dec 2003 (after Lập Xuân 2003) → Vận 7.
    #[test]
    fn test_compute_period_dec_2003_is_van_7() {
        let scanner = default_scanner();
        let jd = jd_from_date(15, 12, 2003);
        let period = compute_period(jd, &scanner);
        assert_eq!(period.van, 7, "Dec 2003 → Vận 7, got {}", period.van);
    }

    /// Jan 2004 (before Lập Xuân 2004) → Vận 7.
    #[test]
    fn test_compute_period_jan_2004_before_lap_xuan_is_van_7() {
        let scanner = default_scanner();
        let jd = jd_from_date(15, 1, 2004);
        let period = compute_period(jd, &scanner);
        assert_eq!(
            period.van, 7,
            "Jan 15 2004 before Lập Xuân → Vận 7, got {}",
            period.van
        );
    }

    /// After Lập Xuân 2004 → Vận 8.
    #[test]
    fn test_compute_period_after_lap_xuan_2004_is_van_8() {
        let scanner = default_scanner();
        let jd = jd_from_date(10, 2, 2004);
        let period = compute_period(jd, &scanner);
        assert_eq!(
            period.van, 8,
            "Feb 10 2004 after Lập Xuân → Vận 8, got {}",
            period.van
        );
    }

    /// Period::base_layout returns a FlyingStarLayout with correct Van period.
    #[test]
    fn test_period_base_layout_van_8() {
        let scanner = default_scanner();
        // Use mid-2023 for Vận 8 (Lập Xuân 2024 transitions to Vận 9)
        let jd8 = jd_from_date(1, 7, 2023);
        let period = compute_period(jd8, &scanner);
        assert_eq!(period.van, 8);
        let layout = period.base_layout();
        assert_eq!(layout.center_star as u8, 8);
        if let FlyingStarPeriod::Van { van } = layout.period {
            assert_eq!(van, 8);
        } else {
            panic!("Expected Van period");
        }
    }

    /// compute_period_for_year convenience function works.
    #[test]
    fn test_compute_period_for_year_2023_is_van_8() {
        let scanner = default_scanner();
        let period = compute_period_for_year(2023, &scanner);
        assert_eq!(period.van, 8);
    }

    /// van_for_solar_year_after_lap_xuan gives correct Vận for boundary years.
    #[test]
    fn test_van_for_boundary_years() {
        assert_eq!(van_for_solar_year_after_lap_xuan(1984), 7);
        assert_eq!(van_for_solar_year_after_lap_xuan(2003), 7);
        assert_eq!(van_for_solar_year_after_lap_xuan(2004), 8);
        assert_eq!(van_for_solar_year_after_lap_xuan(2023), 8);
        assert_eq!(van_for_solar_year_after_lap_xuan(2024), 9);
        assert_eq!(van_for_solar_year_after_lap_xuan(2043), 9);
    }
}
