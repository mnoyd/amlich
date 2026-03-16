//! Golden dataset coverage tests.
//!
//! These tests validate that khcbppt-golden.json meets all dimensional
//! coverage requirements defined in the Phase 2 plan.

use std::collections::HashSet;

use amlich_core::almanac::golden_loader::GoldenDataset;

const GOLDEN_JSON: &str = include_str!("../data/almanac/khcbppt-golden.json");

fn load_golden() -> GoldenDataset {
    serde_json::from_str(GOLDEN_JSON).expect("Failed to parse khcbppt-golden.json")
}

#[test]
fn golden_dataset_has_180_to_240_entries() {
    let dataset = load_golden();
    let count = dataset.entries.len();
    assert!(
        (180..=240).contains(&count),
        "Expected 180-240 entries, got {count}"
    );
    assert_eq!(
        dataset.metadata.entry_count, count,
        "metadata.entry_count must match actual entry count"
    );
}

#[test]
fn golden_dataset_covers_all_12_chi() {
    let dataset = load_golden();
    let chi_set: HashSet<&str> = dataset.entries.iter().map(|e| e.day_chi.as_str()).collect();
    assert_eq!(chi_set.len(), 12, "Must cover all 12 chi, got: {chi_set:?}");

    // Each chi must appear at least 7 times
    for chi in &chi_set {
        let count = dataset.entries.iter().filter(|e| e.day_chi == *chi).count();
        assert!(
            count >= 7,
            "Chi '{chi}' appears only {count} times, need at least 7"
        );
    }
}

#[test]
fn golden_dataset_covers_all_10_can() {
    let dataset = load_golden();
    let can_set: HashSet<&str> = dataset.entries.iter().map(|e| e.day_can.as_str()).collect();
    assert_eq!(can_set.len(), 10, "Must cover all 10 can, got: {can_set:?}");

    // Each can must appear at least 5 times
    for can in &can_set {
        let count = dataset.entries.iter().filter(|e| e.day_can == *can).count();
        assert!(
            count >= 5,
            "Can '{can}' appears only {count} times, need at least 5"
        );
    }
}

#[test]
fn golden_dataset_covers_all_12_lunar_months() {
    let dataset = load_golden();
    let month_set: HashSet<i32> = dataset.entries.iter().map(|e| e.lunar_month).collect();
    assert_eq!(
        month_set.len(),
        12,
        "Must cover all 12 lunar months, got: {month_set:?}"
    );

    // Each month must appear at least 3 times
    for month in 1..=12 {
        let count = dataset
            .entries
            .iter()
            .filter(|e| e.lunar_month == month)
            .count();
        assert!(
            count >= 3,
            "Lunar month {month} appears only {count} times, need at least 3"
        );
    }
}

#[test]
fn golden_dataset_covers_all_28_star_positions() {
    let dataset = load_golden();
    let star_set: HashSet<usize> = dataset
        .entries
        .iter()
        .map(|e| e.expected_star_index)
        .collect();
    assert_eq!(
        star_set.len(),
        28,
        "Must cover all 28 JD-cycle star positions, got {} unique",
        star_set.len()
    );

    // Each position must appear at least 2 times
    for pos in 0..28 {
        let count = dataset
            .entries
            .iter()
            .filter(|e| e.expected_star_index == pos)
            .count();
        assert!(
            count >= 2,
            "Star position {pos} appears only {count} times, need at least 2"
        );
    }
}

#[test]
fn golden_dataset_has_at_least_2_leap_month_entries() {
    let dataset = load_golden();
    let leap_count = dataset.entries.iter().filter(|e| e.is_leap_month).count();
    assert!(
        leap_count >= 2,
        "Need at least 2 leap month entries, got {leap_count}"
    );
}

#[test]
fn golden_dataset_all_dates_in_2020_2030_range() {
    let dataset = load_golden();
    for entry in &dataset.entries {
        assert!(
            (2020..=2030).contains(&entry.solar_year),
            "solar_year {} out of 2020-2030 range for date {}",
            entry.solar_year,
            entry.solar_date
        );
    }
}

#[test]
fn golden_dataset_all_entries_have_nonempty_citations() {
    let dataset = load_golden();
    for (i, entry) in dataset.entries.iter().enumerate() {
        let ref_fields = [
            ("entry_note", &entry.khcbppt_ref.entry_note),
            ("truc", &entry.khcbppt_ref.truc),
            ("day_deity", &entry.khcbppt_ref.day_deity),
            ("taboos", &entry.khcbppt_ref.taboos),
            ("stars", &entry.khcbppt_ref.stars),
            ("xung_hop", &entry.khcbppt_ref.xung_hop),
            ("than_huong", &entry.khcbppt_ref.than_huong),
            ("na_am", &entry.khcbppt_ref.na_am),
        ];
        for (field_name, value) in &ref_fields {
            assert!(
                !value.trim().is_empty(),
                "Entry {i} ({}) has empty khcbppt_ref.{field_name}",
                entry.solar_date
            );
        }
    }
}

mod support;

#[test]
fn golden_dataset_values_match_day_snapshot() {
    let dataset = load_golden();
    // Spot-check a sample of entries to verify golden values match day_snapshot output
    let sample_size = dataset.entries.len().min(20);
    let step = dataset.entries.len() / sample_size;

    for i in (0..dataset.entries.len()).step_by(step) {
        let entry = &dataset.entries[i];
        let snapshot = support::day_snapshot(entry.solar_day, entry.solar_month, entry.solar_year);

        assert_eq!(
            snapshot.context.canchi.day.full, entry.day_canchi,
            "day_canchi mismatch for {}",
            entry.solar_date
        );
        assert_eq!(
            snapshot.context.canchi.day.can, entry.day_can,
            "day_can mismatch for {}",
            entry.solar_date
        );
        assert_eq!(
            snapshot.context.canchi.day.chi, entry.day_chi,
            "day_chi mismatch for {}",
            entry.solar_date
        );
        assert_eq!(
            snapshot.context.lunar.month, entry.lunar_month,
            "lunar_month mismatch for {}",
            entry.solar_date
        );
        assert_eq!(
            snapshot.day_fortune.truc.name, entry.expected_truc_name,
            "truc_name mismatch for {}",
            entry.solar_date
        );
    }
}
