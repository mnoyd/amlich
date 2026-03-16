//! KHCBPPT validator: than huong (travel directions)
//!
//! Compares golden dataset expected values against implementation output
//! for all 233 entries. Run with `cargo test -- --nocapture` to see
//! full divergence reports.
//!
//! This is INVENTORY ONLY -- no corrections are applied.
//! Requirements: THH-01 (10 stems x 3 directions: xuat_hanh, tai_than, hy_than).

mod support;

use amlich_core::almanac::golden_loader::load_golden_dataset;
use support::day_snapshot;

/// THH-01: Validate travel direction fields for all 233 golden entries.
///
/// Compares three fields per entry (all direct string comparison):
/// - xuat_hanh_huong (departure direction)
/// - tai_than (Tai Than direction)
/// - hy_than (Hy Than direction)
#[test]
fn validate_than_huong_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let snapshot = day_snapshot(entry.solar_day, entry.solar_month, entry.solar_year);
        let travel = &snapshot.day_fortune.travel;

        if travel.xuat_hanh_huong != entry.expected_xuat_hanh {
            mismatches.push(format!(
                "[{}] xuat_hanh: expected '{}', got '{}'",
                entry.solar_date, entry.expected_xuat_hanh, travel.xuat_hanh_huong
            ));
        }
        if travel.tai_than != entry.expected_tai_than {
            mismatches.push(format!(
                "[{}] tai_than: expected '{}', got '{}'",
                entry.solar_date, entry.expected_tai_than, travel.tai_than
            ));
        }
        if travel.hy_than != entry.expected_hy_than {
            mismatches.push(format!(
                "[{}] hy_than: expected '{}', got '{}'",
                entry.solar_date, entry.expected_hy_than, travel.hy_than
            ));
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== THAN HUONG DIVERGENCE REPORT ({} mismatches across {} entries) ===",
            mismatches.len(),
            dataset.entries.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END THAN HUONG REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} than huong divergence(s) across {} golden entries. Run with --nocapture for details.",
        mismatches.len(),
        dataset.entries.len()
    );
}
