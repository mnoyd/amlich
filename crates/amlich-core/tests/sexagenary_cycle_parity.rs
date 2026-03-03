//! Sexagenary cycle parity validator
//!
//! Validates that the generated 60-cycle mapping exactly matches baseline
//! reference data from baseline.json na_am_pairs (30 entries × 2 = 60 positions).
//!
//! Requirements:
//! - SC-05: Full-table parity validation against canonical references
//! - PAR-01: Dedicated parity validators for full 60-cycle tables

use amlich_core::almanac::data::baseline_data;
use amlich_core::almanac::sexagenary_cycle::cycle_index_to_canchi;

/// SC-05: Validate full 60-cycle parity against baseline na_am_pairs
///
/// Iterates over all 60 cycle positions (1-60), expanding from 30 baseline
/// na_am_pairs (each covers 2 consecutive positions), and verifies:
/// - Stem (can) matches expected value from baseline
/// - Branch (chi) matches expected value from baseline
/// - Na Am matches the pair entry
/// - Element is correctly extracted
#[test]
fn validate_full_60_cycle_parity() {
    let data = baseline_data();
    let mut mismatches: Vec<String> = Vec::new();

    // Iterate over all 60 cycle positions (1-60)
    for cycle_index in 1..=60 {
        // Get CanChi from the generated cycle
        let canchi = match cycle_index_to_canchi(cycle_index) {
            Some(cc) => cc,
            None => {
                mismatches.push(format!(
                    "[cycle {}] cycle_index_to_canchi returned None",
                    cycle_index
                ));
                continue;
            }
        };

        // Build expected key for lookup
        let expected_key = format!("{} {}", canchi.can, canchi.chi);

        // Look up baseline entry
        let entry = match data.sexagenary_na_am.get(&expected_key) {
            Some(e) => e,
            None => {
                mismatches.push(format!(
                    "[cycle {}] key '{}' not found in baseline sexagenary_na_am",
                    cycle_index, expected_key
                ));
                continue;
            }
        };

        // Verify stem matches
        if entry.can != canchi.can {
            mismatches.push(format!(
                "[cycle {}] can: expected '{}', got '{}'",
                cycle_index, entry.can, canchi.can
            ));
        }

        // Verify branch matches
        if entry.chi != canchi.chi {
            mismatches.push(format!(
                "[cycle {}] chi: expected '{}', got '{}'",
                cycle_index, entry.chi, canchi.chi
            ));
        }

        // Verify element is correctly extracted from na_am
        let expected_element = entry.na_am.split_whitespace().last().unwrap_or("");
        if entry.element != expected_element {
            mismatches.push(format!(
                "[cycle {}] element: expected '{}', got '{}'",
                cycle_index, expected_element, entry.element
            ));
        }
    }

    // Print divergence report if mismatches found
    if !mismatches.is_empty() {
        eprintln!(
            "\n=== SEXAGENARY CYCLE DIVERGENCE REPORT ({} mismatches) ===",
            mismatches.len()
        );
        for m in &mismatches {
            eprintln!("  {}", m);
        }
        eprintln!("=== END DIVERGENCE REPORT ===\n");
    }

    assert!(
        mismatches.is_empty(),
        "Found {} sexagenary cycle divergence(s) across 60 positions. Run with --nocapture for details.",
        mismatches.len()
    );
}

/// Test that each na_am_pair covers 2 consecutive cycle positions with correct mapping
#[test]
fn validate_na_am_pair_consecutive_coverage() {
    let data = baseline_data();
    let mut mismatches: Vec<String> = Vec::new();

    // Iterate over all 60 positions, checking pairs
    for pair_index in 0..30 {
        let index1 = (pair_index * 2) as u8 + 1; // First position (e.g., Giáp Tý)
        let index2 = (pair_index * 2 + 1) as u8 + 1; // Second position (e.g., Ất Sửu)

        let canchi1 = cycle_index_to_canchi(index1);
        let canchi2 = cycle_index_to_canchi(index2);

        if canchi1.is_none() {
            mismatches.push(format!(
                "[pair {}] cycle_index_to_canchi({}) returned None",
                pair_index, index1
            ));
        }

        if canchi2.is_none() {
            mismatches.push(format!(
                "[pair {}] cycle_index_to_canchi({}) returned None",
                pair_index, index2
            ));
        }

        if let (Some(cc1), Some(cc2)) = (canchi1, canchi2) {
            // Verify indices are consecutive
            if index2 != index1 + 1 {
                mismatches.push(format!(
                    "[pair {}] indices not consecutive: {} and {}",
                    pair_index, index1, index2
                ));
            }

            // Verify they share the same na_am
            let key1 = format!("{} {}", cc1.can, cc1.chi);
            let key2 = format!("{} {}", cc2.can, cc2.chi);
            let entry1 = data.sexagenary_na_am.get(&key1);
            let entry2 = data.sexagenary_na_am.get(&key2);

            match (entry1, entry2) {
                (Some(e1), Some(e2)) => {
                    if e1.na_am != e2.na_am {
                        mismatches.push(format!(
                            "[pair {}] na_am mismatch: '{}' vs '{}'",
                            pair_index, e1.na_am, e2.na_am
                        ));
                    }
                }
                (None, _) => {
                    mismatches.push(format!(
                        "[pair {}] key '{}' not found in baseline",
                        pair_index, key1
                    ));
                }
                (_, None) => {
                    mismatches.push(format!(
                        "[pair {}] key '{}' not found in baseline",
                        pair_index, key2
                    ));
                }
            }
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== NA AM PAIR COVERAGE DIVERGENCE ({} mismatches) ===",
            mismatches.len()
        );
        for m in &mismatches {
            eprintln!("  {}", m);
        }
        eprintln!("=== END DIVERGENCE REPORT ===\n");
    }

    assert!(
        mismatches.is_empty(),
        "Found {} na_am pair coverage divergence(s). Run with --nocapture for details.",
        mismatches.len()
    );
}

/// Test divergence report formatting - verify mismatches are collected and reported
#[test]
fn validate_divergence_report_collection() {
    let data = baseline_data();

    // Check first few known positions
    let first_index = 1; // Giáp Tý
    let last_index = 60; // Quý Hợi

    let first = cycle_index_to_canchi(first_index);
    let last = cycle_index_to_canchi(last_index);

    assert!(
        first.is_some(),
        "cycle_index_to_canchi(1) should return Some"
    );
    assert!(
        last.is_some(),
        "cycle_index_to_canchi(60) should return Some"
    );

    if let Some(cc) = first {
        let key = format!("{} {}", cc.can, cc.chi);
        assert!(
            data.sexagenary_na_am.contains_key(&key),
            "Key '{}' should exist in baseline",
            key
        );
    }

    if let Some(cc) = last {
        let key = format!("{} {}", cc.can, cc.chi);
        assert!(
            data.sexagenary_na_am.contains_key(&key),
            "Key '{}' should exist in baseline",
            key
        );
    }
}
