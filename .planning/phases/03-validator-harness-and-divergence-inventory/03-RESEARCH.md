# Phase 3: Validator Harness and Divergence Inventory - Research

**Researched:** 2026-03-01
**Domain:** Rust test harness design, per-subsystem validation, divergence inventory reporting
**Confidence:** HIGH

## Summary

Phase 3 builds 7 validator test files (`khcbppt_taboos.rs`, `khcbppt_deity.rs`, `khcbppt_truc.rs`, `khcbppt_stars.rs`, `khcbppt_xung_hop.rs`, `khcbppt_than_huong.rs`, `khcbppt_na_am.rs`) as Rust integration tests in `crates/amlich-core/tests/`. Each validator iterates over the 233-entry golden dataset (loaded via `load_golden_dataset()`), calls `get_day_info()` for each entry's solar date, compares the implementation output against the golden expected values, and accumulates all mismatches into a divergence report. The critical design constraint is that `cargo test --package amlich-core` must surface EVERY mismatch, not just the first failure -- this rules out naive `assert_eq!` per comparison and instead requires a "collect mismatches then assert at end" pattern within each test function.

The technical approach is straightforward: each validator is a single integration test file with one `#[test]` function per validation dimension. Within each test function, mismatches are collected into a `Vec<String>`, printed with `eprintln!`, and the test asserts `mismatches.is_empty()` at the end -- showing the full divergence report in the test output. The 28-star JD epoch must be verified first (Success Criteria #3), so `khcbppt_stars.rs` has a dedicated `test_jd_epoch_against_khcbppt_dated_entries` test that runs before bulk star validation. No corrections are applied to baseline.json or source constants during this phase (Success Criteria #4) -- this is purely inventory.

The project already has 182 passing tests and a well-established pattern for integration tests in `crates/amlich-core/tests/`. No new dependencies are needed. The existing golden loader, `get_day_info()` API, and DayFortune type structures provide everything required. The main engineering challenge is designing the divergence report format to be readable and actionable for Phase 4 corrections.

**Primary recommendation:** Write 7 integration test files using the "collect-then-assert" pattern, where each test accumulates mismatches across all 233 golden entries and fails with the complete divergence list. Start with `khcbppt_stars.rs` to verify the JD epoch before other subsystems.

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| TAB-01 | Tam Nuong lunar day list cross-referenced against KHCBPPT | `khcbppt_taboos.rs` validator compares `expected_taboos` containing "tam_nuong" against `resolve_day_taboos()` output for all 233 entries |
| TAB-02 | Nguyet Ky lunar day list cross-referenced against KHCBPPT | `khcbppt_taboos.rs` validator compares `expected_taboos` containing "nguyet_ky" against `resolve_day_taboos()` output |
| TAB-03 | Sat Chu 12-month chi map cross-referenced against KHCBPPT | `khcbppt_taboos.rs` validator compares `expected_taboos` containing "sat_chu" against `resolve_day_taboos()` output across all 12 months |
| TAB-04 | Tho Tu 12-month chi map cross-referenced against KHCBPPT | `khcbppt_taboos.rs` validator compares `expected_taboos` containing "tho_tu" against `resolve_day_taboos()` output across all 12 months |
| DEI-01 | 12-deity cycle order and classification cross-referenced | `khcbppt_deity.rs` validator compares `expected_day_deity_name` and `expected_day_deity_classification` against `day_fortune.day_deity` |
| DEI-02 | 12 month-start offsets cross-referenced | `khcbppt_deity.rs` validator implicitly tests offsets by comparing deity output for entries across all 12 lunar months |
| TRC-01 | All 12 truc quality assignments cross-referenced against KHCBPPT | `khcbppt_truc.rs` validator compares `expected_truc_name`, `expected_truc_index`, `expected_truc_quality` against `day_fortune.truc` |
| STR-01 | FixedByChi star assignments cross-referenced against KHCBPPT | `khcbppt_stars.rs` validator compares star lists per chi across entries covering all 12 chi values |
| STR-02 | 28-star JD epoch alignment verified (3+ dated entries) | `khcbppt_stars.rs` dedicated epoch verification test with 3+ manually-identified KHCBPPT-dated star entries |
| STR-03 | 28-star quality assignments cross-referenced | `khcbppt_stars.rs` validator compares `expected_star_name` and `expected_star_quality` against `day_fortune.stars.day_star` |
| THH-01 | 10 stems x 3 directions cross-referenced against KHCBPPT | `khcbppt_than_huong.rs` validator compares `expected_xuat_hanh`, `expected_tai_than`, `expected_hy_than` against `day_fortune.travel` |
| XH-01 | Luc Xung, Tam Hop, Tu Hanh Xung formula basis verified | `khcbppt_xung_hop.rs` validator compares `expected_luc_xung`, `expected_tam_hop`, `expected_tu_hanh_xung` against `day_fortune.xung_hop` |
| NAM-01 | 30 nap am pairs cross-referenced against source | `khcbppt_na_am.rs` validator compares `expected_na_am` and `expected_element` against `day_fortune.day_element` |
</phase_requirements>

## Standard Stack

### Core
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| Rust built-in test framework | stable | `#[test]` functions, `cargo test` runner | Already in use; 182 tests passing; no external test framework needed |
| amlich-core (self) | workspace | `get_day_info()`, `load_golden_dataset()`, almanac types | All subsystem APIs already public and tested |
| serde / serde_json | 1.0 (workspace) | Golden dataset deserialization | Already a dependency via golden_loader.rs |

### Supporting
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| std::collections::HashSet | stable | Set comparison for taboo rule ID lists | Comparing expected vs actual taboo sets where order does not matter |
| std::fmt::Write | stable | Building formatted divergence report strings | Alternative to `format!` for accumulating mismatch descriptions |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| Collect-then-assert pattern | One `#[test]` per golden entry | 233 x 7 = 1631 test functions -- too noisy; one test per subsystem with mismatch collection is cleaner |
| eprintln! for divergence output | Custom reporter struct | eprintln! is simpler and `cargo test -- --nocapture` shows it; custom reporter adds complexity without benefit |
| Integration tests (tests/) | Unit tests (#[cfg(test)] in src/) | Integration tests are the right choice -- validators are external consumers of the public API, matching existing almanac_golden.rs pattern |

**Installation:**
No new dependencies needed. All required crates are already in `Cargo.toml`.

## Architecture Patterns

### Recommended Project Structure
```
crates/amlich-core/
  tests/
    almanac_golden.rs          # existing (preserved)
    generate_golden.rs         # existing (preserved)
    golden_dataset_coverage.rs # existing (preserved)
    ruleset_determinism.rs     # existing (preserved)
    taboo_boundary.rs          # existing (preserved)
    khcbppt_taboos.rs          # NEW: TAB-01..TAB-04
    khcbppt_deity.rs           # NEW: DEI-01, DEI-02
    khcbppt_truc.rs            # NEW: TRC-01
    khcbppt_stars.rs           # NEW: STR-01, STR-02, STR-03
    khcbppt_xung_hop.rs        # NEW: XH-01
    khcbppt_than_huong.rs      # NEW: THH-01
    khcbppt_na_am.rs           # NEW: NAM-01
```

### Pattern 1: Collect-Then-Assert Divergence Report
**What:** Each validator test function iterates all 233 golden entries, collects mismatches into a Vec, then asserts the Vec is empty -- displaying ALL divergences, not just the first.
**When to use:** Every `#[test]` function in every `khcbppt_*.rs` validator file
**Example:**
```rust
// Source: project convention adapted for Phase 3
use amlich_core::almanac::golden_loader::load_golden_dataset;
use amlich_core::get_day_info;

#[test]
fn validate_truc_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let info = get_day_info(entry.solar_day, entry.solar_month, entry.solar_year);
        let fortune = &info.day_fortune;

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
        eprintln!("\n=== TRUC DIVERGENCE REPORT ({} mismatches) ===", mismatches.len());
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END TRUC REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} truc divergence(s) -- see report above",
        mismatches.len()
    );
}
```

### Pattern 2: Taboo Set Comparison
**What:** Taboo validation compares sets of rule_ids (order-independent) rather than exact Vec comparison
**When to use:** `khcbppt_taboos.rs` validator
**Example:**
```rust
// Source: project convention
use std::collections::HashSet;

fn compare_taboo_sets(
    solar_date: &str,
    expected: &[String],
    actual: &[amlich_core::almanac::types::DayTaboo],
    mismatches: &mut Vec<String>,
) {
    let expected_set: HashSet<&str> = expected.iter().map(|s| s.as_str()).collect();
    let actual_set: HashSet<&str> = actual.iter().map(|t| t.rule_id.as_str()).collect();

    let missing: Vec<&&str> = expected_set.difference(&actual_set).collect();
    let extra: Vec<&&str> = actual_set.difference(&expected_set).collect();

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
```

### Pattern 3: Star JD Epoch Verification (Must Run First)
**What:** Dedicated test that verifies the 28-star JD epoch against 3+ real KHCBPPT dated entries before bulk star validation
**When to use:** `khcbppt_stars.rs` -- this test is architecturally prerequisite
**Example:**
```rust
// Source: Phase 3 success criteria #3 + Phase 1 finding (stars.md)
#[test]
fn verify_jd_epoch_against_khcbppt_dated_entries() {
    // These dates have known star assignments from KHCBPPT text
    // Each tuple: (solar_day, solar_month, solar_year, expected_star_name)
    // Values must come from KHCBPPT Cong Quy section entries
    let khcbppt_dated_stars: Vec<(i32, i32, i32, &str)> = vec![
        // At least 3 entries required by success criteria
        // These must be populated from KHCBPPT reference text
        // during implementation
    ];

    let mut mismatches: Vec<String> = Vec::new();
    for (day, month, year, expected_star) in &khcbppt_dated_stars {
        let info = get_day_info(*day, *month, *year);
        let actual_star = info.day_fortune.stars.day_star
            .as_ref()
            .map(|s| s.name.as_str())
            .unwrap_or("NONE");
        if actual_star != *expected_star {
            mismatches.push(format!(
                "[{year}-{month:02}-{day:02}] star: expected '{expected_star}', got '{actual_star}'"
            ));
        }
    }

    if !mismatches.is_empty() {
        eprintln!("\n=== JD EPOCH DIVERGENCE ({} mismatches) ===", mismatches.len());
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END JD EPOCH REPORT ===");
        eprintln!("WARNING: JD epoch offset may be incorrect. All star validations are suspect.");
    }
    assert!(
        mismatches.is_empty(),
        "JD epoch verification failed: {} mismatch(es). Star validation cannot proceed reliably.",
        mismatches.len()
    );
}
```

### Pattern 4: DayDeityClassification String Comparison
**What:** The golden dataset stores deity classification as string ("hoang_dao"/"hac_dao") but the implementation returns `DayDeityClassification` enum
**When to use:** `khcbppt_deity.rs` validator needs a helper to convert
**Example:**
```rust
fn classification_to_string(c: &amlich_core::almanac::types::DayDeityClassification) -> &'static str {
    match c {
        amlich_core::almanac::types::DayDeityClassification::HoangDao => "hoang_dao",
        amlich_core::almanac::types::DayDeityClassification::HacDao => "hac_dao",
    }
}
```

### Anti-Patterns to Avoid
- **Using `assert_eq!` directly per comparison:** This panics on the first mismatch, hiding all subsequent divergences. The whole point of this phase is to see the COMPLETE inventory.
- **One test function per golden entry:** 233 entries x 7 subsystems = 1631 test functions. This is unnecessarily noisy. Use one test per subsystem that iterates all entries.
- **Applying corrections during this phase:** Success Criteria #4 explicitly forbids corrections to baseline.json or source constants. The output is a divergence report, not a fix.
- **Skipping star validation if epoch is wrong:** Do NOT skip bulk star validation -- run it anyway to see the full picture, but ensure the JD epoch test runs and reports separately so the Phase 4 team knows whether epoch correction is needed before star values.
- **Ignoring star rule sparsity:** The golden dataset was generated from `get_day_info()` which already applies all star rules including contextual ones. If a contextual bucket has only 1 seed entry (Phase 1 finding), the validator should note entries where no contextual star rules fire -- this is absence detection, not just mismatch detection.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Golden dataset loading | Custom JSON parser | `load_golden_dataset()` from golden_loader.rs | Already built in Phase 2, validated, OnceLock-cached |
| Day computation | Manual JD/lunar/canchi calculation | `get_day_info(day, month, year)` | Single API call returns all subsystem outputs; the implementation being tested |
| Taboo resolution | Manual rule checking | `day_fortune.taboos` from get_day_info() output | DayFortune already contains resolved taboo list |
| Star quality mapping | String parsing | `StarQuality` enum comparison | Already handled in DayStar struct |
| Test framework | External test runner | Rust built-in `#[test]` + `cargo test` | 182 existing tests already use this; no reason to change |

**Key insight:** Phase 3 validators are thin comparison layers -- they call `get_day_info()`, extract the relevant subsystem field from the DayFortune output, and compare it to the corresponding `expected_*` field in the golden entry. The heavy lifting is already done by the existing implementation and the Phase 2 golden dataset.

## Common Pitfalls

### Pitfall 1: First-Failure Panic Hides Remaining Divergences
**What goes wrong:** Using `assert_eq!` inside the entry iteration loop causes the test to panic on the first mismatch, hiding all subsequent divergences from the report
**Why it happens:** Natural Rust instinct is to `assert_eq!` everywhere
**How to avoid:** Use the collect-then-assert pattern: push mismatches to a `Vec<String>`, print the full list, then `assert!(mismatches.is_empty())`
**Warning signs:** A validator test that fails but only shows one entry's divergence

### Pitfall 2: Cargo Test Output Suppression
**What goes wrong:** `cargo test` captures stdout/stderr by default, so `eprintln!` divergence reports are invisible on success and barely visible on failure
**Why it happens:** Default cargo test behavior
**How to avoid:** Run with `cargo test -- --nocapture` or `cargo test -- --show-output` when reading divergence reports. Document this in the validator file header comments.
**Warning signs:** Test fails but the only output is "assertion failed" with no divergence details

### Pitfall 3: Taboo Set Order Sensitivity
**What goes wrong:** Comparing `expected_taboos` as a Vec against actual taboo rule_ids fails due to ordering differences, even though the same rules fired
**Why it happens:** Golden dataset may list taboos in a different order than `resolve_day_taboos()` returns them
**How to avoid:** Convert both sides to `HashSet<&str>` before comparison. Use set difference to report MISSING and EXTRA separately.
**Warning signs:** Tests fail claiming taboos don't match, but the same rules appear in both lists

### Pitfall 4: DayDeityClassification Enum vs String
**What goes wrong:** Golden dataset stores classification as "hoang_dao"/"hac_dao" strings, but `DayDeity.classification` is a `DayDeityClassification` enum
**Why it happens:** JSON serialization uses lowercase snake_case strings for enum variants
**How to avoid:** Write a `classification_to_string()` helper function, or serialize the enum to string for comparison
**Warning signs:** All deity classification comparisons fail despite correct values

### Pitfall 5: Star Quality Enum vs String Comparison
**What goes wrong:** Golden dataset stores star quality as "cat"/"hung"/"binh" strings, but `DayStar.quality` is a `StarQuality` enum
**Why it happens:** Same enum-to-string mismatch as deity classification
**How to avoid:** Write a `star_quality_to_string()` helper or match-and-compare
**Warning signs:** All star quality comparisons fail

### Pitfall 6: Missing Day Star (Option<DayStar>)
**What goes wrong:** `day_fortune.stars.day_star` is `Option<DayStar>` -- validator code that unwraps without checking will panic
**Why it happens:** The implementation wraps star in Option even though it should always be present for the golden dataset entries
**How to avoid:** Handle `None` as a mismatch ("expected star X, got NONE") rather than panicking
**Warning signs:** Validator panics with "called Option::unwrap() on None"

### Pitfall 7: Confusing Inventory with Verification
**What goes wrong:** Implementer tries to fix divergences during this phase, modifying baseline.json or source constants
**Why it happens:** Natural instinct to fix bugs when found
**How to avoid:** Success Criteria #4 explicitly forbids corrections. Divergences are Phase 4 work. The ONLY output of Phase 3 is the divergence inventory itself.
**Warning signs:** Commits modifying baseline.json or truc.rs constants

## Code Examples

### Complete Validator File Structure
```rust
// Source: project convention for Phase 3
//! KHCBPPT validator: [subsystem name]
//!
//! Compares golden dataset expected values against implementation output
//! for all 233 entries. Run with `cargo test -- --nocapture` to see
//! full divergence reports.
//!
//! This is INVENTORY ONLY -- no corrections are applied.

use amlich_core::almanac::golden_loader::load_golden_dataset;
use amlich_core::get_day_info;

#[test]
fn validate_[subsystem]_against_golden() {
    let dataset = load_golden_dataset();
    let mut mismatches: Vec<String> = Vec::new();

    for entry in &dataset.entries {
        let info = get_day_info(entry.solar_day, entry.solar_month, entry.solar_year);
        let fortune = &info.day_fortune;

        // Compare expected vs actual for this subsystem
        // Push to mismatches Vec instead of asserting
    }

    if !mismatches.is_empty() {
        eprintln!(
            "\n=== [SUBSYSTEM] DIVERGENCE REPORT ({} mismatches across {} entries) ===",
            mismatches.len(),
            dataset.entries.len()
        );
        for m in &mismatches {
            eprintln!("  {m}");
        }
        eprintln!("=== END [SUBSYSTEM] REPORT ===\n");
    }
    assert!(
        mismatches.is_empty(),
        "Found {} [subsystem] divergence(s) across {} golden entries. Run with --nocapture for details.",
        mismatches.len(),
        dataset.entries.len()
    );
}
```

### Than Huong Direction Comparison
```rust
// Source: golden_loader.rs field names + than_huong.rs API
fn validate_than_huong_entry(
    entry: &amlich_core::almanac::golden_loader::GoldenEntry,
    fortune: &amlich_core::almanac::types::DayFortune,
    mismatches: &mut Vec<String>,
) {
    if fortune.travel.xuat_hanh_huong != entry.expected_xuat_hanh {
        mismatches.push(format!(
            "[{}] xuat_hanh: expected '{}', got '{}'",
            entry.solar_date, entry.expected_xuat_hanh, fortune.travel.xuat_hanh_huong
        ));
    }
    if fortune.travel.tai_than != entry.expected_tai_than {
        mismatches.push(format!(
            "[{}] tai_than: expected '{}', got '{}'",
            entry.solar_date, entry.expected_tai_than, fortune.travel.tai_than
        ));
    }
    if fortune.travel.hy_than != entry.expected_hy_than {
        mismatches.push(format!(
            "[{}] hy_than: expected '{}', got '{}'",
            entry.solar_date, entry.expected_hy_than, fortune.travel.hy_than
        ));
    }
}
```

### Na Am Comparison
```rust
// Source: golden_loader.rs field names + types.rs DayElement
fn validate_na_am_entry(
    entry: &amlich_core::almanac::golden_loader::GoldenEntry,
    fortune: &amlich_core::almanac::types::DayFortune,
    mismatches: &mut Vec<String>,
) {
    if fortune.day_element.na_am != entry.expected_na_am {
        mismatches.push(format!(
            "[{}] na_am: expected '{}', got '{}'",
            entry.solar_date, entry.expected_na_am, fortune.day_element.na_am
        ));
    }
    if fortune.day_element.element != entry.expected_element {
        mismatches.push(format!(
            "[{}] element: expected '{}', got '{}'",
            entry.solar_date, entry.expected_element, fortune.day_element.element
        ));
    }
}
```

### Xung Hop Vec Comparison
```rust
// Source: golden_loader.rs field names + types.rs XungHopResult
fn validate_xung_hop_entry(
    entry: &amlich_core::almanac::golden_loader::GoldenEntry,
    fortune: &amlich_core::almanac::types::DayFortune,
    mismatches: &mut Vec<String>,
) {
    if fortune.xung_hop.luc_xung != entry.expected_luc_xung {
        mismatches.push(format!(
            "[{}] luc_xung: expected '{}', got '{}'",
            entry.solar_date, entry.expected_luc_xung, fortune.xung_hop.luc_xung
        ));
    }

    // Tam hop: compare as sorted sets
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

    // Tu hanh xung: compare as sorted sets
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
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Manual spot-checking (almanac_golden.rs, 7 tests) | Systematic bulk validation (7 validator files, 233 entries each) | Phase 3 (now) | Every subsystem fully inventoried, not just a few spot checks |
| Single assert_eq per comparison | Collect-then-assert divergence pattern | Phase 3 (now) | Complete divergence picture instead of first-failure-only |
| Implicit "implementation is correct" assumption | Explicit golden dataset comparison with KHCBPPT citations | Phase 2-3 (now) | Divergences between implementation and KHCBPPT are visible |

**Preserved existing tests:**
- `almanac_golden.rs` -- 7 manual golden tests (stays; Phase 3 adds bulk coverage alongside)
- `ruleset_determinism.rs` -- 5 tests for ruleset loading determinism (stays; unrelated)
- `taboo_boundary.rs` -- 5 boundary tests for taboo rules (stays; unrelated)
- `golden_dataset_coverage.rs` -- 9 coverage validation tests (stays; validates dataset integrity)

## Open Questions

1. **JD epoch verification source data**
   - What we know: JD epoch (JD 0 = Giac/index 0) is Ho Ngoc Duc implementation artifact, not KHCBPPT-defined (Phase 1 finding, MEDIUM confidence). Success Criteria #3 requires verification against 3+ real KHCBPPT dated entries.
   - What's unclear: Which specific KHCBPPT dated entries pair a known solar/lunar date with a named 28-star? The Phase 1 `stars.md` reference file should contain this data, but the validator needs concrete date-to-star mappings.
   - Recommendation: During implementation, consult `docs/reference/khcbppt/stars.md` for dated entries. If fewer than 3 dated entries exist in the reference file, document this as a research gap -- the JD epoch verification is limited by available reference data.

2. **Star rule sparsity detection**
   - What we know: Contextual star buckets (FixedByCanChi, ByYear, ByMonth, ByTietKhi) have only 1 seed entry each in baseline.json (Phase 1 finding)
   - What's unclear: How should the validator report "absence" -- entries where contextual rules should fire but don't because the baseline.json bucket is incomplete?
   - Recommendation: Add a supplementary test in `khcbppt_stars.rs` that counts how many golden entries have zero contextual star rules (FixedByCanChi/ByYear/ByMonth/ByTietKhi). If the count is high, report it as a coverage gap for Phase 4 to investigate.

3. **Expected divergence count**
   - What we know: Phase 1 verified all taboo values, truc qualities, deity values, than huong directions, na am pairs, and xung hop formulas as matching KHCBPPT. The golden dataset was generated from `get_day_info()` output for confirmed-correct subsystems.
   - What's unclear: If the golden dataset and implementation were both generated from the same code, should the divergence count be exactly zero for all subsystems except stars?
   - Recommendation: Yes -- for subsystems confirmed correct in Phase 1, the validator should produce zero divergences (since golden values = implementation output). This is a tautological check, but it establishes the harness infrastructure. Any non-zero divergence would indicate a bug in the golden dataset generator or a code change between Phase 2 and Phase 3. The star subsystem may show divergences due to JD epoch uncertainty.

## Sources

### Primary (HIGH confidence)
- Project source: `crates/amlich-core/src/almanac/golden_loader.rs` -- GoldenDataset/GoldenEntry struct definitions, load_golden_dataset() API
- Project source: `crates/amlich-core/src/almanac/types.rs` -- DayFortune, TrucInfo, XungHopResult, DayDeity, DayTaboo, DayStar, TravelDirection, DayElement struct definitions
- Project source: `crates/amlich-core/src/almanac/calc.rs` -- calculate_day_fortune() function showing how all subsystems are wired together
- Project source: `crates/amlich-core/src/lib.rs` -- get_day_info() public API returning DayInfo with DayFortune
- Project source: `crates/amlich-core/tests/almanac_golden.rs` -- existing golden test patterns (7 tests)
- Project source: `crates/amlich-core/src/almanac/taboo.rs` -- resolve_day_taboos() returning Vec<TabooHit>
- Project source: `crates/amlich-core/src/almanac/truc.rs` -- TRUC_NAMES, TRUC_QUALITY constants, get_truc() API
- Project source: `crates/amlich-core/src/almanac/day_deity.rs` -- resolve_day_deity() API
- Project source: `crates/amlich-core/src/almanac/xung_hop.rs` -- get_xung_hop() API
- Project source: `crates/amlich-core/src/almanac/than_huong.rs` -- get_than_huong() API
- Project source: `crates/amlich-core/src/almanac/star.rs` -- StarCategory, resolve_rules() precedence engine
- Project source: `crates/amlich-core/data/almanac/khcbppt-golden.json` -- 233-entry golden dataset with per-subsystem expected values

### Secondary (MEDIUM confidence)
- [Rust Book: Controlling How Tests Are Run](https://doc.rust-lang.org/book/ch11-02-running-tests.html) -- verified that `cargo test` runs all tests in a binary even when some fail; each `#[test]` function is independent
- [Rust users forum: Non-panicking assertions](https://users.rust-lang.org/t/non-panicking-assertions/75766) -- confirmed that the collect-then-assert pattern is the idiomatic Rust approach for accumulating multiple assertion failures

### Tertiary (LOW confidence)
- 28-star JD epoch correctness -- documented as Ho Ngoc Duc implementation artifact in Phase 1; awaiting Phase 3 verification against dated KHCBPPT entries (the validator itself will resolve this question)

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH -- no new dependencies; all APIs already exist and are tested
- Architecture: HIGH -- collect-then-assert pattern is well-established in Rust; validator file structure mirrors existing test file conventions
- Pitfalls: HIGH -- identified from direct codebase inspection of type mismatches (enum vs string), Option handling, and output suppression behavior
- Star validation: MEDIUM -- JD epoch verification depends on available KHCBPPT reference data from Phase 1; may have fewer than 3 verifiable entries

**Research date:** 2026-03-01
**Valid until:** 2026-04-01 (stable -- no external dependencies; all patterns are internal to this project)
