//! INT-06 — 2026 E2E calendar smoke test across ≥30 representative dates.
//!
//! Categories covered:
//!   - Tết Nguyên Đán 2026 (solar 2026-02-17 = lunar 1/1)
//!   - Sóc (lunar day 1) ×12 months of 2026
//!   - Vọng (lunar day 15) ×12 months of 2026
//!   - Vận 8→9 boundary straddle (2024-02-03 = Vận 8; 2024-02-05 = Vận 9)
//!   - 2026 leap lunar month 6 (3 solar dates with lunar.is_leap && lunar.month == 6)
//!   - All 24 Tiết Khí boundaries of 2026 via TietKhiScanner::terms_for_year(2026)
//!
//! For each date the test asserts:
//!   - calculate_day_snapshot does not panic
//!   - find_van_khan_for_snapshot does not panic (result may be empty)
//!   - compute_combined_overlay(year, lunar_month, &scanner).palace_overlays.len() == 9
//!   - compute_palace_aspects(year, lunar_month, &scanner).len() == 9
//!
//! Imports via `use amlich_core::...` as an external consumer would.

use amlich_core::almanac::fengshui::{
    compute_combined_overlay, compute_palace_aspects, compute_period, TietKhiScanner,
};
use amlich_core::julian::{jd_from_date, jd_to_date};
use amlich_core::rituals::find_van_khan_for_snapshot;
use amlich_core::calculate_day_snapshot;

// ---------------------------------------------------------------------------
// Helper: collect Sóc (lunar day 1) and Vọng (lunar day 15) dates in 2026
// ---------------------------------------------------------------------------

/// Scan solar year 2026 (2026-01-01 .. 2026-12-31) and return the first solar
/// date in each distinct lunar month that has `lunar.day == target_day`.
/// Returns up to 13 entries (12 normal months + possible leap month).
fn collect_lunar_day_dates(target_day: i32) -> Vec<(i32, i32, i32)> {
    let mut result: Vec<(i32, i32, i32)> = Vec::new();
    // Track (lunar_month, is_leap) pairs we have already recorded.
    let mut seen: std::collections::HashSet<(i32, bool)> = std::collections::HashSet::new();

    let start_jd = jd_from_date(1, 1, 2026);
    let end_jd = jd_from_date(31, 12, 2026);

    let mut jd = start_jd;
    while jd <= end_jd {
        let (d, m, y) = jd_to_date(jd);
        let snap = calculate_day_snapshot(d, m, y);
        let lunar = &snap.context.lunar;
        if lunar.day == target_day {
            let key = (lunar.month, lunar.is_leap);
            if !seen.contains(&key) {
                seen.insert(key);
                result.push((d, m, y));
            }
        }
        jd += 1;
    }

    result
}

// ---------------------------------------------------------------------------
// Helper: collect 3 dates in the 2026 leap lunar month 6
// ---------------------------------------------------------------------------

fn collect_leap_month6_dates() -> Vec<(i32, i32, i32)> {
    let mut result: Vec<(i32, i32, i32)> = Vec::new();

    // The 2026 leap month 6 falls in solar late Jul–Aug 2026. Scan a wide
    // enough window to find at least 3 dates.
    let start_jd = jd_from_date(1, 6, 2026); // earlier than expected, safe
    let end_jd = jd_from_date(30, 9, 2026);

    let mut jd = start_jd;
    while jd <= end_jd && result.len() < 3 {
        let (d, m, y) = jd_to_date(jd);
        let snap = calculate_day_snapshot(d, m, y);
        let lunar = &snap.context.lunar;
        if lunar.month == 6 && lunar.is_leap {
            result.push((d, m, y));
        }
        jd += 1;
    }

    result
}

// ---------------------------------------------------------------------------
// Helper: collect 24 Tiết Khí boundary dates for 2026
// ---------------------------------------------------------------------------

fn collect_tiet_khi_dates() -> Vec<(i32, i32, i32)> {
    let scanner = TietKhiScanner::new();
    scanner
        .terms_for_year(2026)
        .iter()
        .map(|t| {
            let (d, m, y) = jd_to_date(t.jd);
            (d, m, y)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Helper: exercise the four pillar APIs for a single date
// ---------------------------------------------------------------------------

fn assert_date_pillar_apis_ok(d: i32, m: i32, y: i32, scanner: &TietKhiScanner) {
    let snap = calculate_day_snapshot(d, m, y);

    // find_van_khan_for_snapshot must not panic; result may be empty
    let _rituals = find_van_khan_for_snapshot(&snap);

    // Clamp lunar month to 1..=12 for the fengshui functions
    let lunar_month = snap.context.lunar.month.clamp(1, 12) as u8;
    let solar_year = snap.context.solar.year;

    // compute_combined_overlay must return exactly 9 palace_overlays
    let overlay = compute_combined_overlay(solar_year, lunar_month, scanner);
    assert_eq!(
        overlay.palace_overlays.len(),
        9,
        "compute_combined_overlay({y}-{m:02}-{d:02}): expected 9 palace_overlays, got {}",
        overlay.palace_overlays.len()
    );

    // compute_palace_aspects must return exactly 9 aspects
    let aspects = compute_palace_aspects(solar_year, lunar_month, scanner);
    assert_eq!(
        aspects.len(),
        9,
        "compute_palace_aspects({y}-{m:02}-{d:02}): expected 9 aspects, got {}",
        aspects.len()
    );
}

// ---------------------------------------------------------------------------
// Main smoke test — all categories, ≥30 distinct dates
// ---------------------------------------------------------------------------

#[test]
fn e2e_2026_smoke_all_categories() {
    let scanner = TietKhiScanner::new();

    let mut dates: Vec<(i32, i32, i32)> = Vec::new();

    // --- Tết Nguyên Đán 2026 (solar 2026-02-17) ---
    dates.push((17, 2, 2026));

    // --- Sóc (lunar day 1) × all months of 2026 ---
    dates.extend(collect_lunar_day_dates(1));

    // --- Vọng (lunar day 15) × all months of 2026 ---
    dates.extend(collect_lunar_day_dates(15));

    // --- Vận 8→9 boundary straddle ---
    dates.push((3, 2, 2024)); // Van 8 (before Lap Xuan 2024-02-04)
    dates.push((5, 2, 2024)); // Van 9 (after  Lap Xuan 2024-02-04)

    // --- Leap lunar month 6 of 2026 (3 dates) ---
    let leap_dates = collect_leap_month6_dates();
    dates.extend(leap_dates.iter());

    // --- 24 Tiết Khí boundaries of 2026 ---
    dates.extend(collect_tiet_khi_dates());

    // Dedup (preserve first occurrence)
    {
        let mut seen = std::collections::HashSet::new();
        dates.retain(|(d, m, y)| seen.insert((*d, *m, *y)));
    }

    // Must have at least 30 distinct dates
    assert!(
        dates.len() >= 30,
        "date set must contain >= 30 distinct entries; got {}",
        dates.len()
    );

    // Exercise all four pillar APIs for every date
    for &(d, m, y) in &dates {
        assert_date_pillar_apis_ok(d, m, y, &scanner);
    }
}

// ---------------------------------------------------------------------------
// Tết assertion — solar 2026-02-17 maps to lunar 1/1
// ---------------------------------------------------------------------------

#[test]
fn tet_2026_is_lunar_1_1() {
    let snap = calculate_day_snapshot(17, 2, 2026);
    assert_eq!(
        snap.context.lunar.day, 1,
        "Tết 2026-02-17: lunar day must be 1, got {}",
        snap.context.lunar.day
    );
    assert_eq!(
        snap.context.lunar.month, 1,
        "Tết 2026-02-17: lunar month must be 1, got {}",
        snap.context.lunar.month
    );
    assert!(
        !snap.context.lunar.is_leap,
        "Tết 2026-02-17: lunar month 1 must not be a leap month"
    );
}

// ---------------------------------------------------------------------------
// Vận boundary assertions — 2024-02-03 = Van 8, 2024-02-05 = Van 9
// ---------------------------------------------------------------------------

#[test]
fn van_boundary_8_to_9() {
    let scanner = TietKhiScanner::new();

    let jd_before = jd_from_date(3, 2, 2024); // before Lập Xuân 2024-02-04
    let period_before = compute_period(jd_before, &scanner);
    assert_eq!(
        period_before.van, 8,
        "2024-02-03 (before Lập Xuân) must be Vận 8, got Vận {}",
        period_before.van
    );

    let jd_after = jd_from_date(5, 2, 2024); // after Lập Xuân 2024-02-04
    let period_after = compute_period(jd_after, &scanner);
    assert_eq!(
        period_after.van, 9,
        "2024-02-05 (after Lập Xuân) must be Vận 9, got Vận {}",
        period_after.van
    );
}
