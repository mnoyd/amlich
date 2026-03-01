//! KHCBPPT xung hop validator (Luc Xung, Tam Hop, Tu Hanh Xung)
//!
//! Compares golden dataset expected values against implementation output
//! for all 233 entries. Run with `cargo test -- --nocapture` to see
//! full divergence reports.
//!
//! This is INVENTORY ONLY -- no corrections are applied.
//! Requirements: XH-01 (Luc Xung, Tam Hop, Tu Hanh Xung formula basis verified).
//!
//! Tam hop and tu hanh xung are compared as sorted vectors (order-independent).

use amlich_core::almanac::golden_loader::load_golden_dataset;
use amlich_core::get_day_info;

#[test]
fn validate_xung_hop_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let info = get_day_info(entry.solar_day, entry.solar_month, entry.solar_year);
        let fortune = &info.day_fortune;

        // Luc xung: direct string comparison
        if fortune.xung_hop.luc_xung != entry.expected_luc_xung {
            mismatches.push(format!(
                "[{}] luc_xung: expected '{}', got '{}'",
                entry.solar_date, entry.expected_luc_xung, fortune.xung_hop.luc_xung
            ));
        }

        // Tam hop: order-independent comparison (sort both sides)
        let mut expected_tam = entry.expected_tam_hop.clone();
        expected_tam.sort();
        let mut actual_tam = fortune.xung_hop.tam_hop.clone();
        actual_tam.sort();
        if expected_tam != actual_tam {
            mismatches.push(format!(
                "[{}] tam_hop: expected {:?}, got {:?}",
                entry.solar_date, expected_tam, actual_tam
            ));
        }

        // Tu hanh xung: order-independent comparison (sort both sides)
        let mut expected_thx = entry.expected_tu_hanh_xung.clone();
        expected_thx.sort();
        let mut actual_thx = fortune.xung_hop.tu_hanh_xung.clone();
        actual_thx.sort();
        if expected_thx != actual_thx {
            mismatches.push(format!(
                "[{}] tu_hanh_xung: expected {:?}, got {:?}",
                entry.solar_date, expected_thx, actual_thx
            ));
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== XUNG HOP DIVERGENCE REPORT ({} mismatches across {} entries) ===",
            mismatches.len(),
            dataset.entries.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END XUNG HOP REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} xung hop divergence(s) across {} golden entries. Run with --nocapture for details.",
        mismatches.len(),
        dataset.entries.len()
    );
}
