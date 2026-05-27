//! Monthly Phi Tinh — Nguyệt Tử Bạch (FS-07).
//!
//! Implements `compute_monthly_flying_stars` which returns a 9-palace monthly
//! flying-star layout.  The monthly center star derives from the year's Earth
//! Branch (chi) group (8/5/2 rule, ADR-0002) and descends mod-9 per solar month.
//!
//! Solar month numbering (ADR-0002):
//!   month 1 = Dần, opens at Lập Xuân (315°)
//!   month 2 = Mão, opens at Kinh Trập
//!   ...
//!   month 12 = Sửu, opens at Tiểu Hàn
//!
//! Year-branch group rule:
//!   - Group 8: Dần/Tỵ/Thân/Hợi years (chi_index 2,5,8,11) → month-1 center = 8
//!   - Group 5: Tý/Mão/Ngọ/Dậu years  (chi_index 0,3,6,9)  → month-1 center = 5
//!   - Group 2: Thìn/Mùi/Tuất/Sửu years (chi_index 4,7,10,1) → month-1 center = 2
//!
//! Direction follows the SAME year-polarity rule as annual Phi Tinh (ADR-0003):
//!   dương year → nghịch (descending), âm year → thuận (ascending).
//! Implemented by reusing `year_is_ascending` from `annual.rs`.
//!
//! `fill_palaces` and `FLYING_PATH` are shared from `annual.rs` (pub(crate)).

use crate::almanac::fengshui::{
    annual::{fill_palaces, year_is_ascending},
    scanner::TietKhiScanner,
    stars::flying_star_from_u8,
    types::{FlyingStarLayout, FlyingStarPeriod},
};
use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};
use crate::sources::SOURCE_HUYEN_KHONG;

// ---------------------------------------------------------------------------
// Year-branch group rule (ADR-0002)
// ---------------------------------------------------------------------------

/// Return the month-1 (Dần month) center star for `year` based on its Earth Branch.
///
/// chi_index (0-based): 0=Tý, 1=Sửu, 2=Dần, 3=Mão, 4=Thìn, 5=Tỵ,
///                      6=Ngọ, 7=Mùi, 8=Thân, 9=Dậu, 10=Tuất, 11=Hợi
pub fn month_group(year: i32) -> u8 {
    let cc = crate::canchi::get_year_canchi(year);
    match cc.chi_index {
        // Group 8: Dần=2, Tỵ=5, Thân=8, Hợi=11
        2 | 5 | 8 | 11 => 8,
        // Group 5: Tý=0, Mão=3, Ngọ=6, Dậu=9
        0 | 3 | 6 | 9 => 5,
        // Group 2: Sửu=1, Thìn=4, Mùi=7, Tuất=10
        1 | 4 | 7 | 10 => 2,
        other => panic!("chi_index {other} is out of range 0..=11"),
    }
}

// ---------------------------------------------------------------------------
// Monthly center star computation
// ---------------------------------------------------------------------------

/// Compute the center star for solar month `solar_month` (1=Dần..12=Sửu).
///
/// Descends one step per month from `group_leader`, wrapping 1→9:
///   `center(m) = ((group_leader - 1 - (m - 1)) mod 9 + 9) mod 9 + 1`
///
/// Examples (group_leader=2):
///   m=1 → 2, m=2 → 1, m=3 → 9 (wrap), m=4 → 8, ...
pub fn monthly_center(group_leader: u8, solar_month: u8) -> u8 {
    let steps = (solar_month - 1) as i32;
    let raw = ((group_leader as i32 - 1 - steps).rem_euclid(9)) + 1;
    raw as u8
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute the 9-palace monthly Phi Tinh layout for solar month `month` of `year`.
///
/// `month` is 1-based solar month index where 1 = Dần (opens at Lập Xuân),
/// 2 = Mão, ..., 12 = Sửu.  Panics on out-of-range month (0 or >12).
///
/// The `_scanner` parameter is part of the FS-07 API signature (solar-term month
/// boundary resolution is a caller concern; the group rule here is arithmetic).
///
/// # Evidence
/// - method: "phi_tinh.nguyet"
/// - note: "year={year};month={month};group={group};center={center}"
pub fn compute_monthly_flying_stars(
    year: i32,
    month: u8,
    _scanner: &TietKhiScanner,
) -> FlyingStarLayout {
    assert!(
        (1..=12).contains(&month),
        "month {month} is out of range 1..=12"
    );

    let leader = month_group(year);
    let center = monthly_center(leader, month);
    let ascending = year_is_ascending(year);
    let palaces = fill_palaces(center, ascending);

    let note = format!(
        "year={year};month={month};group={leader};center={center}"
    );
    let evidence = ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: SOURCE_HUYEN_KHONG.to_string(),
        method: "phi_tinh.nguyet".to_string(),
        note: Some(note),
    };

    FlyingStarLayout {
        period: FlyingStarPeriod::Monthly { year, month },
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
    // month_group
    // -----------------------------------------------------------------------

    /// 2024 = Giáp Thìn, chi_index=4 (Thìn) => group 2.
    #[test]
    fn test_month_group_2024_thìn_is_2() {
        assert_eq!(month_group(2024), 2,
            "2024 Thìn (chi_index 4) should be group 2");
    }

    /// 2025 = Ất Tỵ, chi_index=5 (Tỵ) => group 8.
    #[test]
    fn test_month_group_2025_tỵ_is_8() {
        assert_eq!(month_group(2025), 8,
            "2025 Tỵ (chi_index 5) should be group 8");
    }

    /// Tý year: 2020 = Canh Tý, chi_index=0 => group 5.
    #[test]
    fn test_month_group_ty_year_is_5() {
        // 2020: can=(2020+6)%10=6 (Canh), chi=(2020+8)%12=0 (Tý)
        assert_eq!(month_group(2020), 5, "Tý year should be group 5");
    }

    /// Mão year: 2023 = Quý Mão, chi_index=3 => group 5.
    #[test]
    fn test_month_group_mao_year_is_5() {
        assert_eq!(month_group(2023), 5, "Mão year (chi_index 3) should be group 5");
    }

    /// Dần year => group 8.
    #[test]
    fn test_month_group_dan_year_is_8() {
        // 2022 = Nhâm Dần, chi_index=2 (Dần)
        assert_eq!(month_group(2022), 8, "Dần year (chi_index 2) should be group 8");
    }

    /// Sửu year => group 2.
    #[test]
    fn test_month_group_suu_year_is_2() {
        // 2021 = Tân Sửu, chi_index=1 (Sửu)
        assert_eq!(month_group(2021), 2, "Sửu year (chi_index 1) should be group 2");
    }

    // -----------------------------------------------------------------------
    // monthly_center
    // -----------------------------------------------------------------------

    /// Group-leader=2, month=1 => center=2.
    #[test]
    fn test_monthly_center_group2_month1() {
        assert_eq!(monthly_center(2, 1), 2);
    }

    /// Group-leader=2, month=2 => center=1.
    #[test]
    fn test_monthly_center_group2_month2() {
        assert_eq!(monthly_center(2, 2), 1);
    }

    /// Group-leader=2, month=3 => center=9 (wrap from 1 down to 9).
    #[test]
    fn test_monthly_center_group2_month3_wraps() {
        assert_eq!(monthly_center(2, 3), 9,
            "Descending from 2: m=1→2, m=2→1, m=3→9 (wrap)");
    }

    /// Group-leader=2, month=4 => center=8.
    #[test]
    fn test_monthly_center_group2_month4() {
        assert_eq!(monthly_center(2, 4), 8);
    }

    /// Group-leader=8, month=1 => center=8.
    #[test]
    fn test_monthly_center_group8_month1() {
        assert_eq!(monthly_center(8, 1), 8);
    }

    /// Group-leader=5, month=1 => center=5.
    #[test]
    fn test_monthly_center_group5_month1() {
        assert_eq!(monthly_center(5, 1), 5);
    }

    // -----------------------------------------------------------------------
    // compute_monthly_flying_stars
    // -----------------------------------------------------------------------

    /// 2024, month=1: center=2 (Thìn year => group 2 => month-1 center = 2).
    #[test]
    fn test_compute_monthly_2024_m1_center_is_2() {
        let layout = compute_monthly_flying_stars(2024, 1, &scanner());
        assert_eq!(layout.center_star as u8, 2,
            "2024 Thìn year, month 1: expected center=2");
    }

    /// period is Monthly { year: 2024, month: 1 }.
    #[test]
    fn test_compute_monthly_period_variant() {
        let layout = compute_monthly_flying_stars(2024, 1, &scanner());
        if let crate::almanac::fengshui::types::FlyingStarPeriod::Monthly { year, month } = layout.period {
            assert_eq!(year, 2024);
            assert_eq!(month, 1);
        } else {
            panic!("Expected Monthly period");
        }
    }

    /// Palaces are a permutation of 1..=9 for 2024 month 1.
    #[test]
    fn test_compute_monthly_2024_m1_palaces_permutation() {
        let layout = compute_monthly_flying_stars(2024, 1, &scanner());
        let mut seen = [false; 10];
        for &s in &layout.palaces {
            let n = s as u8;
            assert!(n >= 1 && n <= 9, "star {n} out of range");
            assert!(!seen[n as usize], "duplicate star {n}");
            seen[n as usize] = true;
        }
    }

    /// Evidence method is "phi_tinh.nguyet".
    #[test]
    fn test_compute_monthly_evidence_method() {
        let layout = compute_monthly_flying_stars(2024, 1, &scanner());
        assert_eq!(layout.evidence.method, "phi_tinh.nguyet");
        assert_eq!(layout.evidence.source_id, crate::sources::SOURCE_HUYEN_KHONG);
    }

    /// Month=0 panics (out of range).
    #[test]
    #[should_panic(expected = "out of range")]
    fn test_compute_monthly_month_zero_panics() {
        compute_monthly_flying_stars(2024, 0, &scanner());
    }

    /// Month=13 panics (out of range).
    #[test]
    #[should_panic(expected = "out of range")]
    fn test_compute_monthly_month_13_panics() {
        compute_monthly_flying_stars(2024, 13, &scanner());
    }

    /// Verify all 12 months for 2024 produce permutations.
    #[test]
    fn test_compute_monthly_all_months_2024_permutations() {
        for month in 1u8..=12 {
            let layout = compute_monthly_flying_stars(2024, month, &scanner());
            let mut seen = [false; 10];
            for &s in &layout.palaces {
                let n = s as u8;
                assert!(n >= 1 && n <= 9, "month {month}: star {n} out of range");
                assert!(!seen[n as usize], "month {month}: duplicate star {n}");
                seen[n as usize] = true;
            }
        }
    }

    /// Center descends from month 1 to month 2 for 2024 (group 2: m1=2, m2=1).
    #[test]
    fn test_compute_monthly_2024_m2_center_is_1() {
        let layout = compute_monthly_flying_stars(2024, 2, &scanner());
        assert_eq!(layout.center_star as u8, 1,
            "2024 group=2, month=2: center should be 1");
    }
}
