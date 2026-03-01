//! KHCBPPT day deity validator
//!
//! Compares golden dataset expected values against implementation output
//! for all 233 entries. Run with `cargo test -- --nocapture` to see
//! full divergence reports.
//!
//! This is INVENTORY ONLY -- no corrections are applied.
//! Requirements: DEI-01 (12-deity cycle order and classification),
//!               DEI-02 (12 month-start offsets, implicitly tested via 12 lunar months).

use amlich_core::almanac::golden_loader::load_golden_dataset;
use amlich_core::almanac::types::DayDeityClassification;
use amlich_core::get_day_info;

fn classification_to_str(c: &DayDeityClassification) -> &'static str {
    match c {
        DayDeityClassification::HoangDao => "hoang_dao",
        DayDeityClassification::HacDao => "hac_dao",
    }
}

#[test]
fn validate_deity_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let info = get_day_info(entry.solar_day, entry.solar_month, entry.solar_year);
        let fortune = &info.day_fortune;

        match &fortune.day_deity {
            None => {
                mismatches.push(format!(
                    "[{}] deity: expected '{}' ({}), got NONE",
                    entry.solar_date,
                    entry.expected_day_deity_name,
                    entry.expected_day_deity_classification
                ));
            }
            Some(deity) => {
                if deity.name != entry.expected_day_deity_name {
                    mismatches.push(format!(
                        "[{}] deity name: expected '{}', got '{}'",
                        entry.solar_date, entry.expected_day_deity_name, deity.name
                    ));
                }
                let actual_classification = classification_to_str(&deity.classification);
                if actual_classification != entry.expected_day_deity_classification {
                    mismatches.push(format!(
                        "[{}] deity classification: expected '{}', got '{}'",
                        entry.solar_date,
                        entry.expected_day_deity_classification,
                        actual_classification
                    ));
                }
            }
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== DEITY DIVERGENCE REPORT ({} mismatches across {} entries) ===",
            mismatches.len(),
            dataset.entries.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END DEITY REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} deity divergence(s) across {} golden entries. Run with --nocapture for details.",
        mismatches.len(),
        dataset.entries.len()
    );
}
