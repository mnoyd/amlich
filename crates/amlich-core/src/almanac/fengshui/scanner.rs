//! TietKhiScanner — injection wrapper over `get_all_tiet_khi_for_year`.
//!
//! Provides a stable struct that downstream Phi Tinh functions accept as
//! `&TietKhiScanner`, enabling test substitution and encapsulating the
//! timezone default (ICT = +7.0).
//!
//! The wrapper exists so that `period.rs`, `annual.rs`, `monthly.rs` can
//! all share the same `&TietKhiScanner` signature (FS-01, FS-06, FS-07).

use crate::tietkhi::{get_all_tiet_khi_for_year, SolarTermWithDate};

/// Injection wrapper over `get_all_tiet_khi_for_year`.
///
/// Callers construct one instance and pass `&TietKhiScanner` wherever
/// a solar-term lookup is needed.  The default time zone is ICT (+7.0).
#[derive(Debug, Clone)]
pub struct TietKhiScanner {
    time_zone: f64,
}

impl TietKhiScanner {
    /// Construct with the default ICT time zone (+7.0).
    pub fn new() -> Self {
        Self { time_zone: 7.0 }
    }

    /// Construct with an explicit time zone offset (hours east of UTC).
    pub fn with_time_zone(tz: f64) -> Self {
        Self { time_zone: tz }
    }

    /// Return the Julian Day Number of Lập Xuân for `year`.
    ///
    /// Panics if Lập Xuân is absent in the scanned year (should never
    /// happen for valid Gregorian years in range).
    pub fn lap_xuan_jd(&self, year: i32) -> i32 {
        let terms = get_all_tiet_khi_for_year(year, self.time_zone);
        terms
            .into_iter()
            .find(|t| t.name == "Lập Xuân")
            .map(|t| t.jd)
            .unwrap_or_else(|| panic!("Lập Xuân not found for year {year}"))
    }

    /// Return all solar terms for `year` (delegates to the free function).
    ///
    /// Annual and monthly Phi Tinh plans reuse this to locate arbitrary
    /// solar term boundaries without re-scanning independently.
    pub fn terms_for_year(&self, year: i32) -> Vec<SolarTermWithDate> {
        get_all_tiet_khi_for_year(year, self.time_zone)
    }
}

impl Default for TietKhiScanner {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// TietKhiScanner::new() constructs without panicking.
    #[test]
    fn test_scanner_new() {
        let scanner = TietKhiScanner::new();
        assert_eq!(scanner.time_zone, 7.0);
    }

    /// with_time_zone stores the given offset.
    #[test]
    fn test_scanner_with_time_zone() {
        let scanner = TietKhiScanner::with_time_zone(8.0);
        assert_eq!(scanner.time_zone, 8.0);
    }

    /// lap_xuan_jd(2024) returns a JD that maps to February 2024
    /// (Lập Xuân 2024 falls around Feb 4).
    #[test]
    fn test_lap_xuan_2024_is_in_february() {
        let scanner = TietKhiScanner::new();
        let jd = scanner.lap_xuan_jd(2024);
        let (day, month, year) = crate::julian::jd_to_date(jd);
        assert_eq!(year, 2024, "Lập Xuân 2024 should be in year 2024");
        assert_eq!(month, 2, "Lập Xuân 2024 should be in February");
        // Lập Xuân 2024 is Feb 4
        assert!(
            (3..=6).contains(&day),
            "Lập Xuân 2024 day expected 3-6, got {day}"
        );
    }

    /// terms_for_year delegates and returns at least 24 terms.
    #[test]
    fn test_terms_for_year_returns_terms() {
        let scanner = TietKhiScanner::new();
        let terms = scanner.terms_for_year(2024);
        assert!(
            terms.len() >= 24,
            "expected >= 24 terms, got {}",
            terms.len()
        );
        // Lập Xuân should be present
        let has_lap_xuan = terms.iter().any(|t| t.name == "Lập Xuân");
        assert!(has_lap_xuan, "Lập Xuân not found in 2024 terms");
    }
}
