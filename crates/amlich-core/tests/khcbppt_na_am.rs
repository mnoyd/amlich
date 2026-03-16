//! KHCBPPT validator: na am (sexagenary sound / ngu hanh element)
//!
//! Compares golden dataset expected values against implementation output
//! for all 233 entries. Run with `cargo test -- --nocapture` to see
//! full divergence reports.
//!
//! This is INVENTORY ONLY -- no corrections are applied.
//! Requirements: NAM-01 (30 nap am pairs cross-referenced against KHCBPPT).

mod support;

use amlich_core::almanac::golden_loader::load_golden_dataset;
use support::day_snapshot;

/// NAM-01: Validate na am (sexagenary sound) and element fields for all 233 golden entries.
///
/// Compares two fields per entry (all direct string comparison):
/// - na_am (sexagenary sound name, e.g. "Hai Trung Kim")
/// - element (ngu hanh element, e.g. "Kim")
#[test]
fn validate_na_am_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let snapshot = day_snapshot(entry.solar_day, entry.solar_month, entry.solar_year);
        let day_element = &snapshot.day_fortune.day_element;

        if day_element.na_am != entry.expected_na_am {
            mismatches.push(format!(
                "[{}] na_am: expected '{}', got '{}'",
                entry.solar_date, entry.expected_na_am, day_element.na_am
            ));
        }
        if day_element.element != entry.expected_element {
            mismatches.push(format!(
                "[{}] element: expected '{}', got '{}'",
                entry.solar_date, entry.expected_element, day_element.element
            ));
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== NA AM DIVERGENCE REPORT ({} mismatches across {} entries) ===",
            mismatches.len(),
            dataset.entries.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END NA AM REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} na am divergence(s) across {} golden entries. Run with --nocapture for details.",
        mismatches.len(),
        dataset.entries.len()
    );
}
