//! Civil-time boundary and cross-day spillover goldens for the v1.11
//! point-opening engine (bead `amlich-xlag.2.2.4`).
//!
//! Pins that the civil-time connection reuses Amlich's existing
//! calendar contracts — never redefining them:
//!
//!   1. the four pinned boundary moments (`22:59 → Hợi`,
//!      `23:00 → Tý`, `00:59 → Tý`, `01:00 → Sửu`) delegate to
//!      `resolve_hour_branch_slot` exactly (the same four cases locked
//!      by `tests/branch_channel_integration.rs` and
//!      `tests/hour_pillar_parity.rs`);
//!   2. the 23:00 day transition and the frozen cross-day spillover
//!      rule follow the corpus `day_attribution_rule` / TNLC-DIV-03:
//!      the 23:00–01:00 Tý block belongs to the civil date containing
//!      its 00:00–01:00 half — REVIEWER-PACK §A.3 pins the 甲→乙 (yang)
//!      and a yin 亥/子 boundary;
//!   3. every resolved moment's hour pillar equals the established
//!      `compute_hour_pillar` 五鼠遁 engine seeded from the same day
//!      stem;
//!   4. every result discloses `time_basis = local_civil_hour_branch`
//!      and the TNLC-DIV-03 divergence.

use amlich_core::almanac::hour_pillar::{compute_hour_pillar, resolve_hour_branch_slot};
use amlich_core::almanac::types::{HeavenlyStem, Polarity};
use amlich_core::canchi::get_day_canchi;
use amlich_core::point_opening::{
    resolve_frozen_point_opening_at_local_civil_time, LocalCivilPointOpening, PointOpeningSlotState,
};
use amlich_core::traditional_wellness::divergence::TimeBasis;

/// 2024-02-10 — the verified Giáp (甲, yang) Thìn day fixture from
/// `canchi.rs` (JD 2460351).
const JD_GIAP_DAY: i32 = 2460351;
/// 2024-02-11 — Ất (乙, yin) day, the Giáp day's civil successor.
const JD_AT_DAY: i32 = 2460352;
/// 2024-02-19 — Quý (癸, yin) day: the kidney-table gap day.
const JD_QUY_DAY: i32 = 2460360;

const STEMS_ZH: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const BRANCHES_ZH: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

fn resolve(jd: i32, hour: u8, minute: u8) -> LocalCivilPointOpening {
    resolve_frozen_point_opening_at_local_civil_time(jd, hour, minute)
        .unwrap_or_else(|| panic!("{jd} {hour}:{minute:02} must resolve"))
}

fn pillar_zh(can_index: usize, chi_index: usize) -> String {
    format!("{}{}", STEMS_ZH[can_index], BRANCHES_ZH[chi_index])
}

/// The disclosure contract riding every result (TNLC-DIV-03).
fn assert_disclosed(result: &LocalCivilPointOpening) {
    assert_eq!(result.time_basis(), &TimeBasis::LocalCivilHourBranch);
    assert!(
        result
            .known_divergence_ids()
            .iter()
            .any(|id| id == "TNLC-DIV-03"),
        "every grid-cell result must disclose TNLC-DIV-03"
    );
}

/// The frozen hour pillar must equal the established 五鼠遁 engine
/// seeded from the same slot day stem.
fn assert_hour_pillar_matches_engine(result: &LocalCivilPointOpening, hour: u8, minute: u8) {
    let stem = HeavenlyStem::ALL[result.slot_day_canchi.can_index];
    let pillar = compute_hour_pillar(stem, hour, minute)
        .unwrap_or_else(|| panic!("{hour}:{minute:02} must compute a pillar"));
    assert_eq!(
        result.record.hour_pillar_zh,
        pillar_zh(pillar.can_chi.can_index, pillar.can_chi.chi_index),
        "frozen hour pillar must match compute_hour_pillar at {hour}:{minute:02}"
    );
    assert_eq!(result.hour_slot.branch_index, pillar.slot.branch_index);
}

// ---------------------------------------------------------------------------
// The four pinned boundary moments
// ---------------------------------------------------------------------------

#[test]
fn four_boundary_cases_reuse_existing_calendar_contracts() {
    // (jd, h, m, branch_vi, rollover, cell stem zh, cell branch zh)
    let cases = [
        (JD_GIAP_DAY, 22, 59, "Hợi", false, "甲", "亥"),
        (JD_GIAP_DAY, 23, 0, "Tý", true, "乙", "子"),
        (JD_AT_DAY, 0, 59, "Tý", false, "乙", "子"),
        (JD_AT_DAY, 1, 0, "Sửu", false, "乙", "丑"),
    ];

    for (jd, hour, minute, branch_vi, rollover, stem_zh, branch_zh) in cases {
        let result = resolve(jd, hour, minute);

        // Hour-branch contract reused exactly, never redefined.
        let slot = resolve_hour_branch_slot(hour, minute)
            .unwrap_or_else(|| panic!("{hour}:{minute:02} must resolve a slot"));
        assert_eq!(result.hour_slot, slot, "hour slot at {hour}:{minute:02}");
        assert_eq!(result.hour_slot.branch, branch_vi);

        // Day-pillar contract: the slot day pillar is the existing
        // get_day_canchi over jd (+1 exactly at the 23:00 transition).
        assert_eq!(
            result.late_night_day_transition, rollover,
            "rollover flag at {hour}:{minute:02}"
        );
        assert_eq!(
            result.slot_day_canchi,
            get_day_canchi(jd + i32::from(rollover)),
            "slot day pillar at {hour}:{minute:02}"
        );
        assert_eq!(result.civil_day_canchi, get_day_canchi(jd));

        // Exactly one frozen cell selected by those two conventions.
        assert_eq!(result.record.day_stem_zh, stem_zh);
        assert_eq!(result.record.hour_branch_zh, branch_zh);

        assert_hour_pillar_matches_engine(&result, hour, minute);
        assert_disclosed(&result);
    }
}

// ---------------------------------------------------------------------------
// 23:00 day transition and cross-day spillover (TNLC-DIV-03)
// ---------------------------------------------------------------------------

#[test]
fn late_ty_block_belongs_to_one_civil_day_across_midnight() {
    // 23:30 on the Giáp day and 00:30 on the following Ất day are the
    // same Tý block: the date containing its 00:00–01:00 half owns it.
    let late = resolve(JD_GIAP_DAY, 23, 30);
    let early = resolve(JD_AT_DAY, 0, 30);

    assert!(late.late_night_day_transition);
    assert!(!early.late_night_day_transition);
    assert_eq!(late.civil_day_canchi.can, "Giáp");
    assert_eq!(early.civil_day_canchi.can, "Ất");
    assert_eq!(late.slot_day_canchi, early.slot_day_canchi);
    assert_eq!(late.record, early.record);
}

#[test]
fn yang_cross_day_spillover_pin_giap_day_late_ty() {
    // REVIEWER-PACK §A.3 pins the 甲→乙 boundary: at 23:00–23:59 on a
    // Giáp (yang) day the Tý cell is 乙/子, which the frozen corpus
    // fills from the previous 甲 day's table (row 2, 前谷) — a
    // cross-day spillover whose source table is yang.
    let result = resolve(JD_GIAP_DAY, 23, 30);

    assert_eq!(result.civil_day_canchi.can, "Giáp");
    assert_eq!(result.slot_day_canchi.can, "Ất");
    assert!(result.late_night_day_transition);

    let record = result.record;
    assert_eq!(record.day_stem_zh, "乙");
    assert_eq!(record.hour_branch_zh, "子");
    assert_eq!(record.hour_pillar_zh, "丙子");
    assert!(record.cross_day_spillover);

    let PointOpeningSlotState::Open {
        slot_class_zh_as_printed,
        points,
        ..
    } = &record.context.state
    else {
        panic!("乙/子 must be frozen open");
    };
    assert_eq!(slot_class_zh_as_printed, "榮");
    assert_eq!(points[0].xue_ming_zh, "前谷");
    assert_eq!(points[0].huyet_danh_vi, "Tiền cốc");
    assert_eq!(points[0].standard_code_gloss, "SI2");

    // The spilling source table belongs to the previous day stem 甲 —
    // yang polarity.
    let source_stem = HeavenlyStem::ALL[(result.slot_day_canchi.can_index + 9) % 10];
    assert_eq!(source_stem, HeavenlyStem::Giap);
    assert_eq!(source_stem.polarity(), Polarity::Duong);

    assert_disclosed(&result);
}

#[test]
fn yin_cross_day_spillover_pin_giap_day_early_suu() {
    // The complementary yin-source spillover, also on the Giáp day:
    // the 01:00–02:59 Sửu cell 甲/丑 stays on the civil date and the
    // frozen corpus fills it from the previous Quý (yin) day's table
    // (row 2, 行間 / Hành gian).
    let result = resolve(JD_GIAP_DAY, 1, 30);

    assert!(!result.late_night_day_transition);
    assert_eq!(result.civil_day_canchi, result.slot_day_canchi);
    assert_eq!(result.slot_day_canchi.can, "Giáp");

    let record = result.record;
    assert_eq!(record.day_stem_zh, "甲");
    assert_eq!(record.hour_branch_zh, "丑");
    assert_eq!(record.hour_pillar_zh, "乙丑");
    assert!(record.cross_day_spillover);

    let PointOpeningSlotState::Open { points, .. } = &record.context.state else {
        panic!("甲/丑 must be frozen open");
    };
    assert_eq!(points[0].xue_ming_zh, "行間");
    assert_eq!(points[0].huyet_danh_vi, "Hành gian");
    assert_eq!(points[0].standard_code_gloss, "LR2");

    // The spilling source table belongs to the previous day stem Quý —
    // yin polarity.
    let source_stem = HeavenlyStem::ALL[(result.slot_day_canchi.can_index + 9) % 10];
    assert_eq!(source_stem, HeavenlyStem::Quy);
    assert_eq!(source_stem.polarity(), Polarity::Am);

    assert_disclosed(&result);
}

#[test]
fn quy_day_hoi_ty_boundary_pins_the_kidney_table_gap() {
    // The yin-day 亥/子 boundary of §A.3: on the Quý day the Hợi block
    // is the kidney table's own opening row (湧泉), while 23:00 rolls
    // to the upcoming Giáp day's Tý cell, which the Xu-style tables
    // leave explicitly closed (閉穴) — never filled.
    let hoi = resolve(JD_QUY_DAY, 21, 30);
    assert_eq!(hoi.record.day_stem_zh, "癸");
    assert_eq!(hoi.record.hour_branch_zh, "亥");
    assert!(!hoi.record.cross_day_spillover);
    let PointOpeningSlotState::Open { points, .. } = &hoi.record.context.state else {
        panic!("癸/亥 must be frozen open");
    };
    assert_eq!(points[0].xue_ming_zh, "湧泉");
    assert_eq!(points[0].channel_vi, "Thận");

    let ty = resolve(JD_QUY_DAY, 23, 30);
    assert!(ty.late_night_day_transition);
    assert_eq!(ty.civil_day_canchi.can, "Quý");
    assert_eq!(ty.slot_day_canchi.can, "Giáp");
    assert_eq!(ty.record.day_stem_zh, "甲");
    assert_eq!(ty.record.hour_branch_zh, "子");
    assert!(matches!(
        ty.record.context.state,
        PointOpeningSlotState::Closed { .. }
    ));

    assert_disclosed(&hoi);
    assert_disclosed(&ty);
}

// ---------------------------------------------------------------------------
// Agreement with the established pillar/hour engines, exhaustively
// ---------------------------------------------------------------------------

#[test]
fn every_moment_of_ten_consecutive_days_matches_the_engines_and_discloses() {
    // Ten consecutive days cover every day stem. Sampling the first
    // and last minute of every hour block proves: the attribution rule
    // (rollover exactly at 23:00–23:59), the hour-pillar engine
    // agreement (五鼠遁 from the slot day stem), and the disclosure
    // contract on every result.
    for offset in 0..10 {
        let jd = JD_GIAP_DAY + offset;
        for hour in 0u8..=23 {
            for &minute in &[0u8, 59u8] {
                let result = resolve(jd, hour, minute);

                let expected_rollover = hour == 23;
                assert_eq!(
                    result.late_night_day_transition, expected_rollover,
                    "at {jd} {hour}:{minute:02}"
                );
                assert_eq!(
                    result.slot_day_canchi,
                    get_day_canchi(jd + i32::from(expected_rollover)),
                    "slot day pillar at {jd} {hour}:{minute:02}"
                );
                assert_eq!(
                    result.hour_slot,
                    resolve_hour_branch_slot(hour, minute).unwrap()
                );
                assert_eq!(
                    result.record.day_stem_zh, STEMS_ZH[result.slot_day_canchi.can_index],
                    "cell stem at {jd} {hour}:{minute:02}"
                );
                assert_eq!(
                    result.record.hour_branch_zh, BRANCHES_ZH[result.hour_slot.branch_index],
                    "cell branch at {jd} {hour}:{minute:02}"
                );

                assert_hour_pillar_matches_engine(&result, hour, minute);
                assert_disclosed(&result);
            }
        }
    }
}
