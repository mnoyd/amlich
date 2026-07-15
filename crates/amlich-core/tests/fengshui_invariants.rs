//! Black-box fengshui invariants integration tests — external-consumer perspective.
//!
//! Tests cover:
//!   A — Lo Shu invariants (FS-04): all 9 base Van tables
//!   B — Vận boundary correctness (FS-05): CRIT-2 fix, no naïve year>=2024
//!   C — Golden annual coverage drive (FS-10): >=10 cases/Van per golden dataset
//!   D — Golden monthly + period cases (FS-10/FS-05)
//!   E — Combined overlay smoke (FS-08 cross-check)
//!
//! Imports via `use amlich_core::...` as an external consumer would.

use amlich_core::almanac::fengshui::{
    base_palaces_for_van, compute_combined_overlay, compute_monthly_flying_stars,
    compute_period, compute_yearly_flying_stars, load_flying_stars_golden,
    GoldenConfidence, TietKhiScanner,
};
use amlich_core::almanac::fengshui::types::FlyingStarPeriod;
use amlich_core::julian::jd_from_date;

// ---------------------------------------------------------------------------
// Test A — Lo Shu invariants (FS-04)
// ---------------------------------------------------------------------------

/// For each Van 1..=9:
/// - sum of all 9 palace values == 45
/// - each 1..=9 appears exactly once
/// - center palace (index 4 in Palace::ALL order) == van number
#[test]
fn test_a_lo_shu_invariants_all_vans() {
    for van in 1u8..=9 {
        let palaces = base_palaces_for_van(van);
        let values: Vec<u8> = palaces.iter().map(|&s| s as u8).collect();

        // Sum == 45
        let sum: u32 = values.iter().map(|&v| v as u32).sum();
        assert_eq!(
            sum, 45,
            "Van {van}: Lo Shu sum should be 45, got {sum}"
        );

        // Each 1..=9 exactly once
        let mut seen = [false; 10];
        for &v in &values {
            assert!(
                v >= 1 && v <= 9,
                "Van {van}: star value {v} is outside 1..=9"
            );
            assert!(
                !seen[v as usize],
                "Van {van}: star {v} appears more than once (Lo Shu duplicate violation)"
            );
            seen[v as usize] = true;
        }
        for n in 1usize..=9 {
            assert!(seen[n], "Van {van}: star {n} is missing from the palace layout");
        }

        // Center palace (index 4 = Center in Palace::ALL) == van
        assert_eq!(
            palaces[4] as u8,
            van,
            "Van {van}: center palace should be {van}, got {} (Lo Shu center violation)",
            palaces[4] as u8
        );
    }
}

// ---------------------------------------------------------------------------
// Test B — Vận boundary correctness (FS-05, CRIT-2 fix)
// ---------------------------------------------------------------------------

/// Proves the boundary is scanned via TietKhiScanner — NOT naïve year >= 2024.
///
/// - Jan 15, 2024 (before Lap Xuan 2024-02-04) => Van 8
/// - Feb  5, 2024 (after  Lap Xuan 2024-02-04) => Van 9
/// - Jul  1, 2010 (mid Van 8)                  => Van 8
/// - Jul  1, 1990 (mid Van 7)                  => Van 7
#[test]
fn test_b_van_boundary_lap_xuan_2024() {
    let scanner = TietKhiScanner::new();

    // Before Lap Xuan 2024 => Van 8
    let before = jd_from_date(15, 1, 2024);
    let period = compute_period(before, &scanner);
    assert_eq!(
        period.van, 8,
        "Jan 15 2024 is before Lap Xuan => should be Van 8, got Van {}",
        period.van
    );

    // After Lap Xuan 2024 => Van 9
    let after = jd_from_date(5, 2, 2024);
    let period = compute_period(after, &scanner);
    assert_eq!(
        period.van, 9,
        "Feb 5 2024 is after Lap Xuan => should be Van 9, got Van {}",
        period.van
    );
}

#[test]
fn test_b_van_boundary_mid_van8_and_van7() {
    let scanner = TietKhiScanner::new();

    // Mid-year Van 8: Jul 1 2010 => Van 8
    let mid_van8 = jd_from_date(1, 7, 2010);
    let period = compute_period(mid_van8, &scanner);
    assert_eq!(
        period.van, 8,
        "Jul 1 2010 should be Van 8 (not a boundary edge case), got Van {}",
        period.van
    );

    // Mid-year Van 7: Jul 1 1990 => Van 7
    let mid_van7 = jd_from_date(1, 7, 1990);
    let period = compute_period(mid_van7, &scanner);
    assert_eq!(
        period.van, 7,
        "Jul 1 1990 should be Van 7, got Van {}",
        period.van
    );
}

// ---------------------------------------------------------------------------
// Test C — Golden annual coverage drive (FS-10)
// ---------------------------------------------------------------------------

/// For every annual case in the golden dataset:
/// - call compute_yearly_flying_stars and assert center_star == expected_center
/// - cases whose id appears in known_divergences are asserted to have a divergence entry
///   (the divergence is logged, not silently failing)
/// - assert >= 10 matched cases per Van 7, 8, 9
#[test]
fn test_c_golden_annual_coverage() {
    let scanner = TietKhiScanner::new();
    let ds = load_flying_stars_golden();

    // Build a lookup from year to divergence for annual cases
    let divergent_years: std::collections::HashMap<i32, &amlich_core::almanac::fengshui::KnownDivergence> = ds
        .known_divergences
        .iter()
        .filter_map(|d| {
            // Parse "annual YYYY" format
            if d.case.starts_with("annual ") {
                d.case["annual ".len()..].parse::<i32>().ok().map(|y| (y, d))
            } else {
                None
            }
        })
        .collect();

    let mut van7_matched = 0u32;
    let mut van8_matched = 0u32;
    let mut van9_matched = 0u32;

    for case in ds.cases.iter().filter(|c| c.kind == "annual") {
        let layout = compute_yearly_flying_stars(case.year, &scanner);
        let computed_center = layout.center_star as u8;

        // Check if this year is a known-divergent case
        let is_divergent = divergent_years.contains_key(&case.year);

        if is_divergent {
            // For divergent cases: assert the divergence is logged (not silently corrected)
            let div_entry = divergent_years[&case.year];
            assert_eq!(
                div_entry.our_value,
                computed_center,
                "divergent case year={}: our_value {} should match computed center {}",
                case.year,
                div_entry.our_value,
                computed_center
            );
            // The expected_center in the golden case should also match our tiebreaker value
            assert_eq!(
                case.expected_center,
                computed_center,
                "divergent case year={}: golden expected_center {} != computed {}",
                case.year,
                case.expected_center,
                computed_center
            );
        } else {
            assert_eq!(
                computed_center,
                case.expected_center,
                "annual case '{}' year={}: expected center={}, got center={}",
                case.id,
                case.year,
                case.expected_center,
                computed_center
            );
        }

        // Count matched cases per Van (only Van 7/8/9 for the coverage gate)
        match case.van {
            7 => van7_matched += 1,
            8 => van8_matched += 1,
            9 => van9_matched += 1,
            _ => {} // pre-1984 cross-validation cases — not counted for coverage gate
        }
    }

    assert!(
        van7_matched >= 10,
        "Test C: expected >= 10 matched annual cases for Van 7, got {van7_matched}"
    );
    assert!(
        van8_matched >= 10,
        "Test C: expected >= 10 matched annual cases for Van 8, got {van8_matched}"
    );
    assert!(
        van9_matched >= 10,
        "Test C: expected >= 10 matched annual cases for Van 9, got {van9_matched}"
    );
}

// ---------------------------------------------------------------------------
// Test D — Golden monthly + period cases (FS-10/FS-05)
// ---------------------------------------------------------------------------

/// For every monthly case: compute_monthly_flying_stars and assert center matches.
#[test]
fn test_d_golden_monthly_cases() {
    let scanner = TietKhiScanner::new();
    let ds = load_flying_stars_golden();

    let monthly_cases: Vec<_> = ds.cases.iter().filter(|c| c.kind == "monthly").collect();
    assert!(
        !monthly_cases.is_empty(),
        "Test D: no monthly cases found in golden dataset"
    );

    for case in monthly_cases {
        let month = case.month.expect("monthly case must have month set");
        let layout = compute_monthly_flying_stars(case.year, month, &scanner);
        assert_eq!(
            layout.center_star as u8,
            case.expected_center,
            "monthly case '{}' year={} month={}: expected center={}, got {}",
            case.id,
            case.year,
            month,
            case.expected_center,
            layout.center_star as u8
        );
    }
}

/// For every period case: compute_period and assert van matches.
#[test]
fn test_d_golden_period_cases() {
    let scanner = TietKhiScanner::new();
    let ds = load_flying_stars_golden();

    let period_cases: Vec<_> = ds.cases.iter().filter(|c| c.kind == "period").collect();
    assert!(
        period_cases.len() >= 2,
        "Test D: expected >= 2 period boundary cases in golden dataset, got {}",
        period_cases.len()
    );

    for case in period_cases {
        let jd = case.jd.expect("period case must have jd set");
        let period = compute_period(jd, &scanner);
        assert_eq!(
            period.van,
            case.van,
            "period case '{}' jd={}: expected van={}, got van={}",
            case.id,
            jd,
            case.van,
            period.van
        );
    }
}

// ---------------------------------------------------------------------------
// Test E — Combined overlay smoke (FS-08 cross-check)
// ---------------------------------------------------------------------------

/// compute_combined_overlay(2024, 1, scanner):
/// - palace_overlays len 9
/// - van_layout.period is Van { van: 9 }
/// - evidence.method == "rule.composite.flying_stars"
#[test]
fn test_e_combined_overlay_smoke_2024_m1() {
    let scanner = TietKhiScanner::new();
    let overlay = compute_combined_overlay(2024, 1, &scanner);

    assert_eq!(
        overlay.palace_overlays.len(),
        9,
        "combined overlay should have 9 palace overlays"
    );

    if let FlyingStarPeriod::Van { van } = overlay.van_layout.period {
        assert_eq!(
            van, 9,
            "2024 van_layout should be Van 9, got Van {van}"
        );
    } else {
        panic!(
            "expected van_layout.period to be Van variant, got {:?}",
            overlay.van_layout.period
        );
    }

    assert_eq!(
        overlay.evidence.method,
        "rule.composite.flying_stars",
        "composite evidence method should be 'rule.composite.flying_stars'"
    );
}

/// Annual center for 2024 in the overlay is 4 (center star of the annual layer).
#[test]
fn test_e_combined_overlay_annual_center_2024() {
    let scanner = TietKhiScanner::new();
    let overlay = compute_combined_overlay(2024, 1, &scanner);

    assert_eq!(
        overlay.annual_layout.center_star as u8,
        4,
        "2024 annual center should be 4 (Tu Luc)"
    );
    assert_eq!(
        overlay.monthly_layout.center_star as u8,
        2,
        "2024 month-1 center should be 2 (Nhi Hac, group 2)"
    );
}

/// palace_overlays[i].0 mirrors annual_layout.palaces[i] for all i.
#[test]
fn test_e_combined_overlay_mirrors_components() {
    let scanner = TietKhiScanner::new();
    let overlay = compute_combined_overlay(2024, 3, &scanner);

    for i in 0..9 {
        assert_eq!(
            overlay.palace_overlays[i].0,
            overlay.annual_layout.palaces[i],
            "palace_overlays[{i}].0 should equal annual_layout.palaces[{i}]"
        );
        assert_eq!(
            overlay.palace_overlays[i].1,
            overlay.monthly_layout.palaces[i],
            "palace_overlays[{i}].1 should equal monthly_layout.palaces[{i}]"
        );
    }
}

// ---------------------------------------------------------------------------
// Test F — FND-07 gate: pre-1984 golden cases are HIGH confidence
// ---------------------------------------------------------------------------

/// FND-07 gate: every pre-1984 annual case in the golden dataset carries
/// `confidence: GoldenConfidence::High` after ADR-0003a supersession.
///
/// Pre-1984 Thượng Nguyên (Vận 1–3) and Trung Nguyên (Vận 4–6) cases are
/// reclassified from MEDIUM to HIGH based on dual-source independent secondary
/// modern verification (phongthuycaivan.org + lasotuvi.com / phongthuyso.vn),
/// with *Thẩm Thị Huyền Không Học* retained as classical tiebreaker.
#[test]
fn test_f_golden_pre_1984_confidence_is_high() {
    let ds = load_flying_stars_golden();

    let pre_1984: Vec<_> = ds
        .cases
        .iter()
        .filter(|c| c.kind == "annual" && c.year < 1984)
        .collect();

    assert!(
        !pre_1984.is_empty(),
        "FND-07: expected pre-1984 cross-validation cases in golden dataset, found none"
    );

    // The two canonical pre-1984 cases (1920 + 1960) must both be present.
    let ids: std::collections::HashSet<&str> = pre_1984.iter().map(|c| c.id.as_str()).collect();
    assert!(
        ids.contains("annual-thuong-nguyen-1920"),
        "FND-07: canonical Thượng Nguyên case 'annual-thuong-nguyen-1920' must be present"
    );
    assert!(
        ids.contains("annual-trung-nguyen-1960"),
        "FND-07: canonical Trung Nguyên case 'annual-trung-nguyen-1960' must be present"
    );

    // Every pre-1984 annual case must carry HIGH confidence (post-ADR-0003a).
    for case in &pre_1984 {
        assert_eq!(
            case.confidence,
            GoldenConfidence::High,
            "FND-07: pre-1984 case '{}' (year={}) must be GoldenConfidence::High after ADR-0003a, got {:?}",
            case.id,
            case.year,
            case.confidence,
        );
    }
}
