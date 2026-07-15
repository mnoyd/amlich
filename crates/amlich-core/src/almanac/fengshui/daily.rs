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
        "Đông Chí" | "Vũ Thuỷ" | "Cốc Vũ" => PivotKind::DuongPivot,
        "Hạ Chí" | "Xử Thử" | "Sương Giáng" => PivotKind::AmPivot,
        _ => panic!("not a daily-pivot Trung Khí: {name}"),
    }
}

fn pivot_starting_star(name: &str) -> u8 {
    match name {
        "Đông Chí" => 1,    // Nhất Bạch
        "Vũ Thuỷ" => 7,    // Thất Xích
        "Cốc Vũ" => 4,     // Tứ Lục
        "Hạ Chí" => 9,     // Cửu Tử
        "Xử Thử" => 3,     // Tam Bích
        "Sương Giáng" => 6, // Lục Bạch
        _ => panic!("not a daily-pivot Trung Khí: {name}"),
    }
}

fn daily_pivots_for_year(scanner: &TietKhiScanner, year: i32) -> Vec<(String, i32)> {
    const NAMES: &[&str] = &[
        "Đông Chí",
        "Vũ Thuỷ",
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
