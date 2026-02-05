/**
 * Can Chi (干支 / Can-Chi) Calculations
 *
 * Calculates Heavenly Stem and Earthly Branch for:
 * - Day (based on Julian Day Number)
 * - Month (based on lunar month and year stem)
 * - Year (based on lunar year)
 *
 * References:
 * - Day Can Chi: JD-based formula with verified offset
 * - Month Can Chi: Fixed branch by lunar month, stem by year
 * - Year Can Chi: Standard formula from lunar year
 */
use crate::types::{normalize_index, CanChi};

/// Get Can Chi for a given day
///
/// Formula verified against known dates:
/// - 2024-02-10 (JD 2460351) should be Giáp Thìn (0, 4)
/// - 2025-01-29 (JD 2460705) should be Mậu Tuất (4, 10)
///
/// Standard formula:
/// - Can: (JD + 9) % 10
/// - Chi: (JD + 1) % 12
///
/// # Arguments
/// * `jd` - Julian Day Number
///
/// # Returns
/// Day Can Chi information
pub fn get_day_canchi(jd: i32) -> CanChi {
    // JD offset formulas (verified against multiple sources)
    let can_index = normalize_index(jd + 9, 10);
    let chi_index = normalize_index(jd + 1, 12);

    CanChi::new(can_index, chi_index)
}

/// Get Can Chi for a lunar month
///
/// Branch (Chi): Fixed by lunar month
/// - Month 1 = Dần (index 2)
/// - Month 2 = Mão (index 3)
/// - Month 3 = Thìn (index 4)
/// - ... wraps at 12
///
/// Stem (Can): Determined by year stem using table:
/// Year Can | Month 1 (Dần) starts with
/// ---------|---------------------------
/// Giáp/Kỷ  → Bính (index 2)
/// Ất/Canh  → Mậu (index 4)
/// Bính/Tân → Canh (index 6)
/// Đinh/Nhâm → Nhâm (index 8)
/// Mậu/Quý  → Giáp (index 0)
///
/// # Arguments
/// * `lunar_month` - Lunar month (1-12)
/// * `lunar_year` - Lunar year
/// * `is_leap_month` - Is leap month (false or true)
///
/// # Returns
/// Month Can Chi information
pub fn get_month_canchi(lunar_month: i32, lunar_year: i32, is_leap_month: bool) -> CanChi {
    // Month branch is fixed: Month 1 = Dần (2), Month 2 = Mão (3), etc.
    let chi_index = normalize_index(lunar_month + 1, 12);

    // Get year stem to determine first month stem
    let year_can_index = normalize_index(lunar_year + 6, 10);

    // Year stem to first month (Dần) stem mapping
    let first_month_can_table = [2, 4, 6, 8, 0, 2, 4, 6, 8, 0]; // Giáp→Bính, Ất→Mậu, etc.
    let first_month_can = first_month_can_table[year_can_index];

    // Calculate current month stem (offset from month 1)
    let can_index = normalize_index(first_month_can as i32 + (lunar_month - 1), 10);

    let mut result = CanChi::new(can_index, chi_index);

    // Add leap month indicator
    if is_leap_month {
        result.full = format!("{} (nhuận)", result.full);
    }

    result
}

/// Get Can Chi for a lunar year
///
/// Standard formula verified against known years:
/// - 2024 lunar = Giáp Thìn (0, 4)
/// - 2025 lunar = Ất Tỵ (1, 5)
/// - 2023 lunar = Quý Mão (9, 3)
///
/// Formula:
/// - Can: (year + 6) % 10
/// - Chi: (year + 8) % 12
///
/// # Arguments
/// * `lunar_year` - Lunar year
///
/// # Returns
/// Year Can Chi information
pub fn get_year_canchi(lunar_year: i32) -> CanChi {
    let can_index = normalize_index(lunar_year + 6, 10);
    let chi_index = normalize_index(lunar_year + 8, 12);

    CanChi::new(can_index, chi_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_canchi_tet_2024() {
        // Tết 2024: February 10, 2024 (JD 2460351) = Giáp Thìn
        let cc = get_day_canchi(2460351);
        assert_eq!(cc.can, "Giáp");
        assert_eq!(cc.chi, "Thìn");
        assert_eq!(cc.full, "Giáp Thìn");
    }

    #[test]
    fn test_day_canchi_tet_2025() {
        // Tết 2025: January 29, 2025 (JD 2460705) = Mậu Tuất
        let cc = get_day_canchi(2460705);
        assert_eq!(cc.can, "Mậu");
        assert_eq!(cc.chi, "Tuất");
    }

    #[test]
    fn test_year_canchi_2024() {
        // 2024 lunar year = Giáp Thìn (Year of the Dragon)
        let cc = get_year_canchi(2024);
        assert_eq!(cc.can, "Giáp");
        assert_eq!(cc.chi, "Thìn");
        assert_eq!(cc.con_giap, "Thìn (Rồng)");
    }

    #[test]
    fn test_year_canchi_2025() {
        // 2025 lunar year = Ất Tỵ (Year of the Snake)
        let cc = get_year_canchi(2025);
        assert_eq!(cc.can, "Ất");
        assert_eq!(cc.chi, "Tỵ");
        assert_eq!(cc.con_giap, "Tỵ (Rắn)");
    }

    #[test]
    fn test_year_canchi_2023() {
        // 2023 lunar year = Quý Mão (Year of the Cat)
        let cc = get_year_canchi(2023);
        assert_eq!(cc.can, "Quý");
        assert_eq!(cc.chi, "Mão");
        assert_eq!(cc.con_giap, "Mão (Mèo)");
    }

    #[test]
    fn test_month_canchi_basic() {
        // Month 1 of year 2024 (Giáp year)
        // Giáp year → Month 1 starts with Bính (index 2)
        // Month 1 branch = Dần (index 2)
        let cc = get_month_canchi(1, 2024, false);
        assert_eq!(cc.can, "Bính");
        assert_eq!(cc.chi, "Dần");
    }

    #[test]
    fn test_month_canchi_leap() {
        // Test leap month indicator
        let cc = get_month_canchi(4, 2023, true);
        assert!(cc.full.contains("(nhuận)"));
    }

    #[test]
    fn test_month_canchi_progression() {
        // Test that months progress correctly
        let month1 = get_month_canchi(1, 2024, false);
        let month2 = get_month_canchi(2, 2024, false);

        // Stems should progress by 1
        assert_eq!((month1.can_index + 1) % 10, month2.can_index);
        // Branches should progress by 1
        assert_eq!((month1.chi_index + 1) % 12, month2.chi_index);
    }
}
