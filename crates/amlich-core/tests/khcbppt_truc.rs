//! KHCBPPT truc (12 duty-stars) validator
//!
//! Compares golden dataset expected values against implementation output
//! for all 233 entries. Run with `cargo test -- --nocapture` to see
//! full divergence reports.
//!
//! This is INVENTORY ONLY -- no corrections are applied.
//! Requirements: TRC-01 (all 12 truc quality assignments cross-referenced against KHCBPPT).

mod support;

use amlich_core::almanac::golden_loader::load_golden_dataset;
use support::day_snapshot;

#[test]
fn validate_truc_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let snapshot = day_snapshot(entry.solar_day, entry.solar_month, entry.solar_year);
        let fortune = &snapshot.day_fortune;

        if fortune.truc.name != entry.expected_truc_name {
            mismatches.push(format!(
                "[{}] truc name: expected '{}', got '{}'",
                entry.solar_date, entry.expected_truc_name, fortune.truc.name
            ));
        }
        if fortune.truc.index != entry.expected_truc_index {
            mismatches.push(format!(
                "[{}] truc index: expected {}, got {}",
                entry.solar_date, entry.expected_truc_index, fortune.truc.index
            ));
        }
        if fortune.truc.quality != entry.expected_truc_quality {
            mismatches.push(format!(
                "[{}] truc quality: expected '{}', got '{}'",
                entry.solar_date, entry.expected_truc_quality, fortune.truc.quality
            ));
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== TRUC DIVERGENCE REPORT ({} mismatches across {} entries) ===",
            mismatches.len(),
            dataset.entries.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END TRUC REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} truc divergence(s) across {} golden entries. Run with --nocapture for details.",
        mismatches.len(),
        dataset.entries.len()
    );
}
