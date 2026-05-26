//! Integration tests for Phase 11 ritual lookup APIs.
//!
//! These tests treat amlich-core as an EXTERNAL crate (via `amlich_core::...`)
//! to confirm the Phase 11 module's public re-exports work end-to-end. Inline
//! `#[cfg(test)] mod tests` in `matcher.rs` and `corpus.rs` cover white-box
//! invariants; this file covers black-box behavior matching ROADMAP §Phase 11
//! Success Criteria #1, #4, #5.

use amlich_core::holidays::get_vietnamese_holidays;
use amlich_core::rituals::{
    all_rituals, find_van_khan_for_event, find_van_khan_for_snapshot, LeapPolicy,
    RitualEventKey,
};
use amlich_core::{calculate_day_snapshot, DaySnapshot};

// ─── Test 1: RIT-01 Tết snapshot wiring ──────────────────────────────────────
#[test]
fn tet_nguyen_dan_2024_snapshot_returns_tet_ritual() {
    // Tết 2024 = solar 2024-02-10, lunar 1/1.
    let snapshot: DaySnapshot = calculate_day_snapshot(10, 2, 2024);
    let hits = find_van_khan_for_snapshot(&snapshot);
    assert!(!hits.is_empty(), "Tết 2024 should return ≥ 1 ritual");

    let has_tet_holiday_key = hits.iter().any(|entry| {
        entry.event_keys.iter().any(|k| matches!(
            k,
            RitualEventKey::HolidayId { value } if value == "tet-nguyen-dan"
        ))
    });
    assert!(
        has_tet_holiday_key,
        "expected at least one returned ritual to carry HolidayId{{\"tet-nguyen-dan\"}}; got: {:?}",
        hits.iter().map(|r| &r.ritual_id).collect::<Vec<_>>()
    );
}

// ─── Test 2: Sóc/Vọng snapshot path — falsifiable via van-khan-ram-thang-gieng ─
#[test]
fn vong_snapshot_returns_ram_thang_gieng_via_snapshot_path() {
    // 2024-02-24 = lunar 1/15 = Tết Nguyên Tiêu (Vọng of month 1).
    let snapshot = calculate_day_snapshot(24, 2, 2024);

    // Anchor sanity: the conversion really did land on lunar day 15. If this
    // assertion fires, the lunar-conversion contract drifted and the rest of
    // the test is meaningless — surface that as the root cause.
    assert_eq!(
        snapshot.context.lunar.day, 15,
        "anchor date 2024-02-24 must be lunar day 15; lunar conversion drifted"
    );

    let hits = find_van_khan_for_snapshot(&snapshot);

    // FALSIFIABLE: the 11-01 fixture `van-khan-ram-thang-gieng` carries
    //   event_keys: [HolidayId{tet-nguyen-tieu}, LunarDate{m=1,d=15}]
    // Either of those needles must fire from `derive_event_keys` on this
    // snapshot. If hits is empty (or missing the fixture), the snapshot-path
    // Sóc/Vọng plumbing is broken — fail loudly.
    assert!(
        hits.iter().any(|r| r.ritual_id == "van-khan-ram-thang-gieng"),
        "Vọng 2024-02-24 must return 'van-khan-ram-thang-gieng' via the snapshot path; \
         got: {:?}",
        hits.iter().map(|r| &r.ritual_id).collect::<Vec<_>>()
    );

    // Honesty check: every hit must trace to a snapshot-derivable key.
    // `derive_event_keys` emits: HolidayId (per matching real holiday),
    // LunarDate (current month/day + policy), SolarTerm (current tiết khí),
    // Always. A hit with none of those is a matcher bug.
    for entry in &hits {
        let via_day_15 = entry.event_keys.iter().any(|k| matches!(
            k,
            RitualEventKey::LunarDate { day: 15, .. }
        ));
        let via_holiday = entry.event_keys.iter().any(|k| matches!(
            k,
            RitualEventKey::HolidayId { .. }
        ));
        let via_solar_term = entry.event_keys.iter().any(|k| matches!(
            k,
            RitualEventKey::SolarTerm { .. }
        ));
        let via_always = entry.event_keys.iter().any(|k| matches!(k, RitualEventKey::Always));
        assert!(
            via_day_15 || via_holiday || via_solar_term || via_always,
            "ritual {} fired on Vọng 2024-02-24 but has no day-15/holiday/solar-term/always event key",
            entry.ritual_id
        );
    }
}

// ─── Test 3: Thanh Minh via SolarTerm path (Holiday.id=None — only SolarTerm fires) ──
#[test]
fn thanh_minh_snapshot_returns_thanh_minh_ritual_via_solar_term_path() {
    let mut observed_tiet_khi: Vec<String> = Vec::new();
    let mut thanh_minh_day: Option<i32> = None;
    for day in 1..=10 {
        let snap = calculate_day_snapshot(day, 4, 2024);
        observed_tiet_khi.push(format!("{}/04: {}", day, snap.context.tiet_khi.name));
        if snap.context.tiet_khi.name == "Thanh Minh" {
            thanh_minh_day = Some(day);
            break;
        }
    }
    let day = thanh_minh_day.unwrap_or_else(|| {
        panic!(
            "no day in 2024-04-01..2024-04-10 has tiet_khi == \"Thanh Minh\". Observed:\n{}",
            observed_tiet_khi.join("\n")
        )
    });

    let snapshot = calculate_day_snapshot(day, 4, 2024);
    let hits = find_van_khan_for_snapshot(&snapshot);
    assert!(
        hits.iter().any(|r| r.ritual_id == "van-khan-thanh-minh"),
        "Thanh Minh 2024-04-{:02} should return 'van-khan-thanh-minh'; got: {:?}",
        day,
        hits.iter().map(|r| &r.ritual_id).collect::<Vec<_>>()
    );
}

// ─── Test 4: HolidayId cross-reference (no typos in fixtures) ────────────────
#[test]
fn every_holiday_id_in_fixtures_resolves_to_a_real_holiday() {
    use std::collections::HashSet;

    // Collect every HolidayId.value from the entire corpus.
    let mut fixture_ids: HashSet<String> = HashSet::new();
    for entry in all_rituals() {
        for key in &entry.event_keys {
            if let RitualEventKey::HolidayId { value } = key {
                fixture_ids.insert(value.clone());
            }
        }
    }

    // Collect every real Holiday.id across multiple years to cover any year-offset
    // edge cases (some holidays have year_offset = -1 / +1).
    let mut real_ids: HashSet<String> = HashSet::new();
    for year in 2020..=2030 {
        for h in get_vietnamese_holidays(year) {
            if let Some(id) = h.id {
                real_ids.insert(id);
            }
        }
    }

    let missing: Vec<&String> = fixture_ids.difference(&real_ids).collect();
    assert!(
        missing.is_empty(),
        "fixture HolidayId values not found in any 2020-2030 holiday corpus: {:?}",
        missing
    );
}

// ─── Test 5: NFC round-trip byte-equal serialization ─────────────────────────
#[test]
fn every_entry_round_trips_byte_equal_through_serde_json() {
    for entry in all_rituals() {
        let first = serde_json::to_string(entry).expect("serialize");
        let parsed: amlich_core::rituals::RitualEntry =
            serde_json::from_str(&first).expect("deserialize round-trip");
        let second = serde_json::to_string(&parsed).expect("re-serialize");
        assert_eq!(
            first, second,
            "ritual {} did not round-trip byte-equal (NFC drift in serde path?)",
            entry.ritual_id
        );
    }
}

// ─── Test 6: leap-policy semantics at the public API surface ─────────────────
#[test]
fn leap_month_only_needle_does_not_match_canonical_only_entry() {
    // The Đoan Ngọ fixture (plan 11-01) has:
    //   event_keys: [{kind:holiday_id, value:tet-doan-ngo},
    //                {kind:lunar_date, month:5, day:5, leap_month_policy:canonical_month_only}]
    let leap_needle = RitualEventKey::LunarDate {
        month: 5,
        day: 5,
        leap_month_policy: LeapPolicy::LeapMonthOnly,
    };
    let leap_hits = find_van_khan_for_event(&leap_needle);
    assert!(
        !leap_hits.iter().any(|r| r.ritual_id == "van-khan-doan-ngo"),
        "Đoan Ngọ (CanonicalMonthOnly) MUST NOT match a LeapMonthOnly needle"
    );

    let canonical_needle = RitualEventKey::LunarDate {
        month: 5,
        day: 5,
        leap_month_policy: LeapPolicy::CanonicalMonthOnly,
    };
    let canonical_hits = find_van_khan_for_event(&canonical_needle);
    assert!(
        canonical_hits.iter().any(|r| r.ritual_id == "van-khan-doan-ngo"),
        "Đoan Ngọ (CanonicalMonthOnly) MUST match a CanonicalMonthOnly needle"
    );
}
