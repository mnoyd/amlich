//! KHCBPPT validator: 28-star (Nhi Thap Bat Tu) system
//!
//! Compares golden dataset expected values against implementation output
//! for all 233 entries. Run with `cargo test -- --nocapture` to see
//! full divergence reports.
//!
//! This is INVENTORY ONLY -- no corrections are applied.
//! Requirements: STR-01, STR-02, STR-03

mod support;

use amlich_core::almanac::golden_loader::load_golden_dataset;
use amlich_core::almanac::types::StarQuality;
use support::day_snapshot;

fn star_quality_to_str(q: &StarQuality) -> &'static str {
    match q {
        StarQuality::Cat => "cat",
        StarQuality::Hung => "hung",
        StarQuality::Binh => "binh",
    }
}

/// STR-02: Verify JD epoch alignment against golden dataset entries.
///
/// The 28-star JD cycle uses jd.rem_euclid(28) -- this test verifies the epoch
/// is correctly aligned by checking the first 5 golden entries (which have known
/// star assignments from get_day_info at dataset generation time).
#[test]
fn verify_jd_epoch_against_khcbppt_dated_entries() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    // Use the first 5 entries as epoch verification anchors.
    // These entries have known star assignments from the golden dataset.
    let epoch_entries = dataset.entries.iter().take(5);

    for entry in epoch_entries {
        let snapshot = day_snapshot(entry.solar_day, entry.solar_month, entry.solar_year);
        match &snapshot.day_fortune.stars.day_star {
            None => {
                mismatches.push(format!(
                    "[{}] star: expected '{}' (index {}), got NONE",
                    entry.solar_date, entry.expected_star_name, entry.expected_star_index
                ));
            }
            Some(day_star) => {
                if day_star.name != entry.expected_star_name {
                    mismatches.push(format!(
                        "[{}] star name: expected '{}', got '{}'",
                        entry.solar_date, entry.expected_star_name, day_star.name
                    ));
                }
                if day_star.index != entry.expected_star_index {
                    mismatches.push(format!(
                        "[{}] star index: expected {}, got {}",
                        entry.solar_date, entry.expected_star_index, day_star.index
                    ));
                }
            }
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== JD EPOCH DIVERGENCE REPORT ({} mismatches) ===",
            mismatches.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END JD EPOCH REPORT ===");
        eprintln!("WARNING: JD epoch offset may be incorrect. All star validations are suspect.\n");
    }
    assert!(
        mismatches.is_empty(),
        "JD epoch verification failed: {} mismatch(es). Star validation cannot proceed reliably.",
        mismatches.len()
    );
}

/// STR-01, STR-03: Validate all 233 golden entries for star name, index, and quality.
///
/// Covers FixedByChi star assignments via dataset coverage (all 12 chi are present).
/// Star quality uses StarQuality enum converted to string for comparison.
#[test]
fn validate_stars_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let snapshot = day_snapshot(entry.solar_day, entry.solar_month, entry.solar_year);
        match &snapshot.day_fortune.stars.day_star {
            None => {
                mismatches.push(format!(
                    "[{}] star name: expected '{}', got NONE",
                    entry.solar_date, entry.expected_star_name
                ));
                mismatches.push(format!(
                    "[{}] star index: expected {}, got NONE",
                    entry.solar_date, entry.expected_star_index
                ));
                mismatches.push(format!(
                    "[{}] star quality: expected '{}', got NONE",
                    entry.solar_date, entry.expected_star_quality
                ));
            }
            Some(day_star) => {
                if day_star.name != entry.expected_star_name {
                    mismatches.push(format!(
                        "[{}] star name: expected '{}', got '{}'",
                        entry.solar_date, entry.expected_star_name, day_star.name
                    ));
                }
                if day_star.index != entry.expected_star_index {
                    mismatches.push(format!(
                        "[{}] star index: expected {}, got {}",
                        entry.solar_date, entry.expected_star_index, day_star.index
                    ));
                }
                let actual_quality = star_quality_to_str(&day_star.quality);
                if actual_quality != entry.expected_star_quality {
                    mismatches.push(format!(
                        "[{}] star quality: expected '{}', got '{}'",
                        entry.solar_date, entry.expected_star_quality, actual_quality
                    ));
                }
            }
        }
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== STAR DIVERGENCE REPORT ({} mismatches across {} entries) ===",
            mismatches.len(),
            dataset.entries.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END STAR REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} star divergence(s) across {} golden entries. Run with --nocapture for details.",
        mismatches.len(),
        dataset.entries.len()
    );
}

/// Supplementary: Report star rule sparsity for contextual categories.
///
/// Counts entries with zero contextual star rules (FixedByCanChi, ByYear, ByMonth, ByTietKhi).
/// This test always passes -- it is informational, documenting the coverage gap for Phase 4.
#[test]
fn report_star_rule_sparsity() {
    let dataset = load_golden_dataset();
    let contextual_categories = ["FixedByCanChi", "ByYear", "ByMonth", "ByTietKhi"];

    let no_contextual_count = dataset
        .entries
        .iter()
        .filter(|entry| {
            let snapshot = day_snapshot(entry.solar_day, entry.solar_month, entry.solar_year);
            let matched = &snapshot.day_fortune.stars.matched_rules;
            !matched
                .iter()
                .any(|r| contextual_categories.contains(&r.category.as_str()))
        })
        .count();

    eprintln!(
        "\n=== STAR RULE SPARSITY REPORT ===\n  {}/{} entries have no contextual star rules (FixedByCanChi/ByYear/ByMonth/ByTietKhi)\n=== END SPARSITY REPORT ===\n",
        no_contextual_count,
        dataset.entries.len()
    );

    // Always passes -- informational only
    assert!(
        true,
        "sparsity report is informational, this assertion never fails"
    );
}
