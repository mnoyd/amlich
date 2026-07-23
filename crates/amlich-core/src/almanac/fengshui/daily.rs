//! Daily Phi Tinh — Lưu Nhật / 日紫白 (FS-16).
//!
//! Implements `compute_daily_flying_stars` per ADR-0004 (2026-07-15). The daily
//! center star derives from the **6 Trung Khí pivots** (Đông Chí, Vũ Thuỷ, Cốc Vũ,
//! Hạ Chí, Xử Thử, Sương Giáng) found via the v1.1.2 `TietKhiScanner` (no naïve
//! calendar arithmetic — inherits CRIT-2 / ADR-0002 boundary discipline).
//!
//! Each pivot "kicks in" at the FIRST Giáp Tý (Can=0, Chi=0) day with JD >= pivot_jd
//! (Pitfall P-7 — Giáp-Tý-as-seed-day mechanic). Days BEFORE that Giáp Tý use the
//! PREVIOUS pivot's seed and direction.
//!
//! Direction rule: Dương pivot → thuận (forward, +1 per Giáp Tý cycle); Âm pivot →
//! nghịch (descending, -1 per Giáp Tý cycle). This is the OPPOSITE of the annual
//! layer (ADR-0003 §4: dương year = nghịch, âm year = thuận).
//!
//! The 9-palace fill reuses `fill_palaces(center, ascending)` from annual.rs
//! (pub(crate)) — no duplicate walking logic, no silent Lo Shu drift.

use crate::almanac::fengshui::{
    annual::fill_palaces,
    scanner::TietKhiScanner,
    stars::flying_star_from_u8,
    types::{DailyFlyingStarLayout, FlyingStarPeriod},
};
use crate::canchi::get_day_canchi;
use crate::julian::jd_from_date;
use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};
use crate::sources::SOURCE_HUYEN_KHONG;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PivotKind {
    DuongPivot,
    AmPivot,
}

fn pivot_kind(name: &str) -> PivotKind {
    match name {
        "Đông Chí" | "Vũ Thuỷ" | "Vũ Thủy" | "Cốc Vũ" => PivotKind::DuongPivot,
        "Hạ Chí" | "Xử Thử" | "Sương Giáng" => PivotKind::AmPivot,
        _ => panic!("not a daily-pivot Trung Khí: {name}"),
    }
}

fn pivot_starting_star(name: &str) -> u8 {
    match name {
        "Đông Chí" => 1,            // Nhất Bạch
        "Vũ Thuỷ" | "Vũ Thủy" => 7, // Thất Xích
        "Cốc Vũ" => 4,              // Tứ Lục
        "Hạ Chí" => 9,              // Cửu Tử
        "Xử Thử" => 3,              // Tam Bích
        "Sương Giáng" => 6,         // Lục Bạch
        _ => panic!("not a daily-pivot Trung Khí: {name}"),
    }
}

fn daily_pivots_for_year(scanner: &TietKhiScanner, year: i32) -> Vec<(String, i32)> {
    const NAMES: &[&str] = &[
        "Đông Chí",
        "Vũ Thuỷ",
        "Vũ Thủy",
        "Cốc Vũ",
        "Hạ Chí",
        "Xử Thử",
        "Sương Giáng",
    ];
    let mut result: Vec<(String, i32)> = Vec::with_capacity(18);
    for y in [year - 1, year, year + 1] {
        for t in scanner.terms_for_year(y) {
            if NAMES.contains(&t.name.as_str()) {
                result.push((t.name.clone(), t.jd));
            }
        }
    }
    result.sort_by_key(|(_, jd)| *jd);
    result
}

/// Compute the 9-palace daily Phi Tinh (Lưu Nhật / 日紫白) layout for `date`.
/// See module-level docstring for the full ADR-0004 algorithm.
pub fn compute_daily_flying_stars(
    date: (i32, u32, u32),
    scanner: &TietKhiScanner,
) -> DailyFlyingStarLayout {
    let (y, m, d) = date;
    let target_jd = jd_from_date(d as i32, m as i32, y);

    // 1. Find the pivot Trung Khí bracketing the target date.
    let pivots = daily_pivots_for_year(scanner, y);
    let (mut pivot_name, mut pivot_jd) = pivots
        .iter()
        .rev()
        .find(|(_, jd)| *jd <= target_jd)
        .cloned()
        .expect("no pivot Trung Khí found before target date — wrap year and retry");

    // 2. Find the first Giáp Tý with JD >= pivot_jd (Pitfall P-7 — Giap-Ty-as-seed-day).
    let mut giap_ty_seed_jd = pivot_jd;
    loop {
        let cc = get_day_canchi(giap_ty_seed_jd);
        if cc.can_index == 0 && cc.chi_index == 0 {
            break;
        }
        giap_ty_seed_jd += 1;
    }

    // 2b. Pitfall P-7 fall-back: if target_jd is BEFORE the first Giap Ty of the
    // new Tiet Khi, the pivot has not "kicked in" yet — use the PRIOR pivot's seed
    // and direction. Example: 2024-12-25 falls between Dong Chi 2024-12-21 and the
    // first Giap Ty in that Tiet Khi (early Jan 2025); algorithm must use the
    // prior pivot (Suong Giang 2024) with seed=6 / nghich direction.
    // Source: phongthuycaivan.org "Cach tra Phi tinh Nien Nguyet Nhat Thoi" cited in
    // 18-RESEARCH.md (Pitfall P-7 narrative + research.md:353-357 trace).
    if target_jd < giap_ty_seed_jd {
        let (prior_name, prior_jd) = pivots
            .iter()
            .rev()
            .find(|(_, jd)| *jd < pivot_jd)
            .cloned()
            .expect("no prior pivot found for pre-Giap-Ty-in-new-Tiet-Khi date");
        pivot_name = prior_name;
        pivot_jd = prior_jd;
        // Recompute giap_ty_seed_jd from the prior pivot.
        giap_ty_seed_jd = pivot_jd;
        loop {
            let cc = get_day_canchi(giap_ty_seed_jd);
            if cc.can_index == 0 && cc.chi_index == 0 {
                break;
            }
            giap_ty_seed_jd += 1;
        }
    }

    // 3. Classify pivot (Dương/Âm) and derive direction + seed.
    let kind = pivot_kind(&pivot_name);
    let ascending = matches!(kind, PivotKind::DuongPivot); // Dương = thuận (forward)
    let seed = pivot_starting_star(&pivot_name);

    // 4. Count Giáp Tý cycles from seed to target (inclusive of target if Giáp Tý).
    let mut n: i32 = 0;
    let mut cur = giap_ty_seed_jd;
    while cur <= target_jd {
        let cc = get_day_canchi(cur);
        if cc.can_index == 0 && cc.chi_index == 0 && cur != giap_ty_seed_jd {
            n += 1;
        }
        cur += 1;
    }

    // 5. Center = seed ± n mod 9 (wrap 1↔9).
    let raw: i32 = if ascending {
        seed as i32 + n
    } else {
        seed as i32 - n
    };
    let center = ((raw - 1_i32).rem_euclid(9)) + 1;

    // 6. Fill palaces via shared fill_palaces.
    let palaces = fill_palaces(center as u8, ascending);

    // 7. Evidence envelope (Pitfall P-8: pivot name in note for audit replay).
    let direction = if ascending { "thuận" } else { "nghịch" };
    let note = format!(
        "date={y}-{m:02}-{d:02};pivot={pivot_name};seed={seed};days_from_seed={n};\
         center={center};direction={direction};confidence=high"
    );
    let evidence = ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: SOURCE_HUYEN_KHONG.to_string(),
        method: "phi_tinh.nhat".to_string(),
        note: Some(note),
    };

    DailyFlyingStarLayout {
        period: FlyingStarPeriod::Daily { date },
        palaces,
        center_star: flying_star_from_u8(center as u8),
        evidence,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::fengshui::annual::nien_center;
    use crate::julian::jd_to_date;

    fn scanner() -> TietKhiScanner {
        TietKhiScanner::new()
    }

    fn first_giap_ty_on_or_after(mut jd: i32) -> i32 {
        loop {
            let cc = get_day_canchi(jd);
            if cc.can_index == 0 && cc.chi_index == 0 {
                return jd;
            }
            jd += 1;
        }
    }

    fn giap_ty_cycles(seed_jd: i32, target_jd: i32) -> i32 {
        let mut n = 0;
        let mut cur = seed_jd;
        while cur <= target_jd {
            let cc = get_day_canchi(cur);
            if cc.can_index == 0 && cc.chi_index == 0 && cur != seed_jd {
                n += 1;
            }
            cur += 1;
        }
        n
    }

    fn wrapped_center(seed: u8, n: i32, ascending: bool) -> u8 {
        let raw = if ascending {
            seed as i32 + n
        } else {
            seed as i32 - n
        };
        (((raw - 1).rem_euclid(9)) + 1) as u8
    }

    fn note(layout: &DailyFlyingStarLayout) -> &str {
        layout
            .evidence
            .note
            .as_deref()
            .expect("daily evidence note")
    }

    fn raw_pivot_for_date(scanner: &TietKhiScanner, date: (i32, u32, u32)) -> (String, i32) {
        let (y, m, d) = date;
        let target_jd = jd_from_date(d as i32, m as i32, y);
        daily_pivots_for_year(scanner, y)
            .iter()
            .rev()
            .find(|(_, jd)| *jd <= target_jd)
            .cloned()
            .expect("raw pivot before date")
    }

    #[test]
    #[should_panic(expected = "not a daily-pivot")]
    fn test_pivot_kind_duong_am_split() {
        assert_eq!(pivot_kind("Đông Chí"), PivotKind::DuongPivot);
        assert_eq!(pivot_kind("Hạ Chí"), PivotKind::AmPivot);
        pivot_kind("Lập Xuân");
    }

    #[test]
    fn test_pivot_starting_star_values() {
        assert_eq!(pivot_starting_star("Đông Chí"), 1);
        assert_eq!(pivot_starting_star("Vũ Thuỷ"), 7);
        assert_eq!(pivot_starting_star("Vũ Thủy"), 7);
        assert_eq!(pivot_starting_star("Cốc Vũ"), 4);
        assert_eq!(pivot_starting_star("Hạ Chí"), 9);
        assert_eq!(pivot_starting_star("Xử Thử"), 3);
        assert_eq!(pivot_starting_star("Sương Giáng"), 6);
    }

    #[test]
    fn test_daily_pivots_for_year_returns_six_pivots() {
        let result = daily_pivots_for_year(&scanner(), 2024);
        assert!(result.len() >= 6, "expected at least the 6 daily pivots");
        for name in [
            "Đông Chí",
            "Vũ Thủy",
            "Cốc Vũ",
            "Hạ Chí",
            "Xử Thử",
            "Sương Giáng",
        ] {
            assert!(
                result.iter().any(|(pivot_name, _)| pivot_name == name),
                "missing pivot {name}"
            );
        }
    }

    #[test]
    fn test_compute_daily_palaces_permutation_invariant() {
        let scanner = scanner();
        for date in [
            (2024, 1, 15),
            (2024, 2, 15),
            (2024, 3, 15),
            (2024, 4, 20),
            (2024, 6, 25),
            (2024, 8, 25),
            (2024, 11, 25),
            (2024, 12, 25),
        ] {
            let layout = compute_daily_flying_stars(date, &scanner);
            let mut seen = [false; 10];
            for star in layout.palaces {
                let n = star as u8;
                assert!((1..=9).contains(&n), "star out of range for {date:?}: {n}");
                assert!(!seen[n as usize], "duplicate star {n} for {date:?}");
                seen[n as usize] = true;
            }
            for (n, &present) in seen.iter().enumerate().skip(1) {
                assert!(present, "missing star {n} for {date:?}");
            }
            assert_eq!(layout.palaces[4], layout.center_star);
        }
    }

    #[test]
    fn test_compute_daily_period_is_daily_variant() {
        let layout = compute_daily_flying_stars((2024, 12, 25), &scanner());
        if let FlyingStarPeriod::Daily { date: (y, m, d) } = layout.period {
            assert_eq!((y, m, d), (2024, 12, 25));
        } else {
            panic!("expected FlyingStarPeriod::Daily");
        }
    }

    #[test]
    fn test_compute_daily_evidence_method_phi_tinh_nhat() {
        let layout = compute_daily_flying_stars((2024, 12, 25), &scanner());
        assert_eq!(layout.evidence.method, "phi_tinh.nhat");
        assert_eq!(layout.evidence.source_id, SOURCE_HUYEN_KHONG);
    }

    #[test]
    fn test_compute_daily_pivot_in_winter_differs_from_nien_center() {
        let layout = compute_daily_flying_stars((2024, 12, 25), &scanner());
        assert_ne!(layout.center_star as u8, nien_center(2024));
    }

    #[test]
    fn test_compute_daily_direction_inversion_duong_vs_am() {
        let scanner = scanner();
        let pivots = daily_pivots_for_year(&scanner, 2024);

        let dong_chi_jd = pivots
            .iter()
            .find(|(name, _)| name == "Đông Chí")
            .map(|(_, jd)| *jd)
            .expect("Đông Chí pivot");
        let dong_seed_jd = first_giap_ty_on_or_after(dong_chi_jd);
        let dong_target_jd = dong_seed_jd + 60;
        let dong_n = giap_ty_cycles(dong_seed_jd, dong_target_jd);
        assert_eq!(wrapped_center(1, dong_n, true), 2, "Dương pivot ascends");

        let ha_chi_jd = pivots
            .iter()
            .find(|(name, _)| name == "Hạ Chí")
            .map(|(_, jd)| *jd)
            .expect("Hạ Chí pivot");
        let ha_seed_jd = first_giap_ty_on_or_after(ha_chi_jd);
        let ha_target_jd = ha_seed_jd + 60;
        let ha_n = giap_ty_cycles(ha_seed_jd, ha_target_jd);
        assert_eq!(wrapped_center(9, ha_n, false), 8, "Âm pivot descends");
    }

    #[test]
    fn test_compute_daily_evidence_note_includes_pivot_name() {
        let scanner = scanner();
        let winter = compute_daily_flying_stars((2024, 12, 25), &scanner);
        assert!(note(&winter).contains("pivot=Sương Giáng"));
        assert!(note(&winter).contains("direction=nghịch"));

        let ha_chi_jd = daily_pivots_for_year(&scanner, 2024)
            .into_iter()
            .find(|(name, _)| name == "Hạ Chí")
            .map(|(_, jd)| jd)
            .expect("Hạ Chí pivot");
        let summer_seed_jd = first_giap_ty_on_or_after(ha_chi_jd);
        let (summer_day, summer_month, summer_year) = jd_to_date(summer_seed_jd);
        let summer = compute_daily_flying_stars(
            (summer_year, summer_month as u32, summer_day as u32),
            &scanner,
        );
        assert!(note(&summer).contains("pivot=Hạ Chí"));
        assert!(note(&summer).contains("direction=nghịch"));
    }

    #[test]
    fn test_compute_daily_giap_ty_seed_mechanic_p7() {
        let scanner = scanner();
        let date = (2024, 12, 25);
        let target_jd = jd_from_date(date.2 as i32, date.1 as i32, date.0);
        let pivots = daily_pivots_for_year(&scanner, date.0);
        let (prior_name, prior_jd) = pivots
            .iter()
            .rev()
            .find(|(name, jd)| name == "Sương Giáng" && *jd <= target_jd)
            .cloned()
            .expect("prior Sương Giáng pivot");
        let seed_jd = first_giap_ty_on_or_after(prior_jd);
        let n = giap_ty_cycles(seed_jd, target_jd);
        let expected_center = wrapped_center(pivot_starting_star(&prior_name), n, false);

        let layout = compute_daily_flying_stars(date, &scanner);
        assert_eq!(layout.center_star as u8, expected_center);
        assert!(note(&layout).contains("pivot=Sương Giáng"));
        assert!(!note(&layout).contains("pivot=Đông Chí"));
    }

    #[test]
    fn test_compute_daily_boundary_discipline_via_tiet_khi_scanner() {
        let scanner = scanner();
        let dong_chi_jd = daily_pivots_for_year(&scanner, 2024)
            .into_iter()
            .find(|(name, _)| name == "Đông Chí")
            .map(|(_, jd)| jd)
            .expect("Đông Chí pivot");
        let (dong_day, dong_month, dong_year) = jd_to_date(dong_chi_jd);
        let (after_name, _) =
            raw_pivot_for_date(&scanner, (dong_year, dong_month as u32, dong_day as u32));
        assert_eq!(after_name, "Đông Chí");

        let mut prior_jd = dong_chi_jd - 1;
        let prior_name = loop {
            let (prior_day, prior_month, prior_year) = jd_to_date(prior_jd);
            let (name, _) =
                raw_pivot_for_date(&scanner, (prior_year, prior_month as u32, prior_day as u32));
            if name != "Đông Chí" {
                break name;
            }
            prior_jd -= 1;
        };
        assert_eq!(prior_name, "Sương Giáng");
    }
}
