//! KHCBPPT validator: taboo rules (Tam Nuong, Nguyet Ky, Sat Chu, Tho Tu)
//!
//! Compares golden dataset expected taboo rule_ids against implementation output
//! for all 233 entries. Comparison is order-independent (set-based).
//! Run with `cargo test -- --nocapture` to see full divergence reports.
//!
//! This is INVENTORY ONLY -- no corrections are applied.
//! Requirements: TAB-01, TAB-02, TAB-03, TAB-04

use amlich_core::almanac::golden_loader::load_golden_dataset;
use amlich_core::almanac::types::DayTaboo;
use amlich_core::get_day_info;
use std::collections::HashSet;

/// Compare expected taboo rule_id sets against actual DayTaboo lists.
///
/// Reports MISSING (in golden but not in impl) and EXTRA (in impl but not in golden) separately.
/// Comparison is order-independent using HashSet.
fn compare_taboo_sets(
    solar_date: &str,
    expected: &[String],
    actual: &[DayTaboo],
    mismatches: &mut Vec<String>,
) {
    let expected_set: HashSet<&str> = expected.iter().map(|s| s.as_str()).collect();
    let actual_set: HashSet<&str> = actual.iter().map(|t| t.rule_id.as_str()).collect();

    let mut missing: Vec<&&str> = expected_set.difference(&actual_set).collect();
    missing.sort();
    let mut extra: Vec<&&str> = actual_set.difference(&expected_set).collect();
    extra.sort();

    if !missing.is_empty() {
        mismatches.push(format!(
            "[{solar_date}] taboos MISSING (in golden, not in impl): {:?}",
            missing
        ));
    }
    if !extra.is_empty() {
        mismatches.push(format!(
            "[{solar_date}] taboos EXTRA (in impl, not in golden): {:?}",
            extra
        ));
    }
}

/// TAB-01, TAB-02, TAB-03, TAB-04: Validate taboo rules for all 233 golden entries.
///
/// Uses set-based comparison (order-independent). Covers all 4 taboo rule types:
/// - TAB-01: tam_nuong (lunar days 3, 7, 13, 18, 22, 27)
/// - TAB-02: nguyet_ky (lunar days 5, 14, 23)
/// - TAB-03: sat_chu (month-chi map)
/// - TAB-04: tho_tu (month-chi map)
#[test]
fn validate_taboos_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let info = get_day_info(entry.solar_day, entry.solar_month, entry.solar_year);
        compare_taboo_sets(
            &entry.solar_date,
            &entry.expected_taboos,
            &info.day_fortune.taboos,
            &mut mismatches,
        );
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== TABOO DIVERGENCE REPORT ({} mismatches across {} entries) ===",
            mismatches.len(),
            dataset.entries.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END TABOO REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} taboo divergence(s) across {} golden entries. Run with --nocapture for details.",
        mismatches.len(),
        dataset.entries.len()
    );
}

/// Supplementary: Report which taboo rule_ids appear in the golden dataset and implementation.
///
/// This test always passes -- it is informational, verifying that all 4 taboo types
/// (tam_nuong, nguyet_ky, sat_chu, tho_tu) are represented in the dataset.
#[test]
fn validate_taboo_coverage_by_rule() {
    let dataset = load_golden_dataset();

    let mut golden_rules: HashSet<String> = HashSet::new();
    let mut impl_rules: HashSet<String> = HashSet::new();

    for entry in &dataset.entries {
        let info = get_day_info(entry.solar_day, entry.solar_month, entry.solar_year);
        for rule_id in &entry.expected_taboos {
            golden_rules.insert(rule_id.clone());
        }
        for taboo in &info.day_fortune.taboos {
            impl_rules.insert(taboo.rule_id.clone());
        }
    }

    let mut golden_sorted: Vec<&String> = golden_rules.iter().collect();
    golden_sorted.sort();
    let mut impl_sorted: Vec<&String> = impl_rules.iter().collect();
    impl_sorted.sort();

    eprintln!(
        "\n=== TABOO RULE COVERAGE REPORT ===\n  Rules in golden: {:?}\n  Rules in impl:   {:?}\n=== END COVERAGE REPORT ===\n",
        golden_sorted, impl_sorted
    );

    // Always passes -- informational only
    assert!(
        true,
        "coverage report is informational, this assertion never fails"
    );
}
