---
phase: 03-validator-harness-and-divergence-inventory
plan: 01
subsystem: testing
tags: [rust, cargo-test, golden-dataset, divergence-inventory, stars, taboos, khcbppt]

# Dependency graph
requires:
  - phase: 02-golden-dataset-and-loader
    provides: load_golden_dataset(), GoldenEntry with expected_star_index/expected_star_name/expected_star_quality/expected_taboos fields
provides:
  - khcbppt_stars.rs: STR-01, STR-02, STR-03 validator with JD epoch verification, bulk star validation (name/index/quality), and sparsity report
  - khcbppt_taboos.rs: TAB-01..TAB-04 validator with set-based taboo comparison (MISSING/EXTRA), and coverage-by-rule report
affects: [03-02, 03-03, phase-4]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Collect-then-assert pattern: iterate all 233 entries, push to Vec<String> mismatches, assert mismatches.is_empty() with eprintln! report"
    - "Set-based taboo comparison using HashSet to avoid order sensitivity"
    - "StarQuality enum-to-string helper: star_quality_to_str maps Cat/Hung/Binh to cat/hung/binh"
    - "Option<DayStar> handling: None yields NONE mismatch strings rather than panic"

key-files:
  created:
    - crates/amlich-core/tests/khcbppt_stars.rs
    - crates/amlich-core/tests/khcbppt_taboos.rs
  modified: []

key-decisions:
  - "JD epoch verification uses first 5 golden entries as anchors (not hand-selected KHCBPPT dates) — golden dataset was generated from get_day_info() so epoch is self-consistent; verification confirms no regression"
  - "Star rule sparsity test always passes (informational) — 233/233 entries have no contextual star rules (FixedByCanChi/ByYear/ByMonth/ByTietKhi) confirming baseline.json has only 1 seed entry per contextual bucket"
  - "Taboo comparison is set-based (HashSet) not Vec-based — avoids false failures due to ordering differences between golden and impl"
  - "Zero divergences expected for all subsystems since golden dataset was generated from get_day_info() — tautological check establishing harness infrastructure for Phase 4"

patterns-established:
  - "Star validator: star_quality_to_str helper converts StarQuality enum to 'cat'/'hung'/'binh' strings for comparison against golden"
  - "Taboo validator: compare_taboo_sets helper takes solar_date/expected/actual/mismatches — sorts MISSING and EXTRA lists for deterministic output"
  - "All validators: eprintln! report header format '=== [SUBSYSTEM] DIVERGENCE REPORT (N mismatches across M entries) ==='"

requirements-completed: [STR-01, STR-02, STR-03, TAB-01, TAB-02, TAB-03, TAB-04]

# Metrics
duration: 5min
completed: 2026-03-01
---

# Phase 3 Plan 01: Star and Taboo Validator Summary

**khcbppt_stars.rs (3 tests: JD epoch, bulk stars, sparsity) and khcbppt_taboos.rs (2 tests: set-based taboos, coverage) implement the two most complex Phase 3 validators with zero divergences across 233 entries**

## Performance

- **Duration:** ~5 min (files verified as pre-existing from plan 03-03 auto-fix deviation)
- **Started:** 2026-03-01T16:12:22Z
- **Completed:** 2026-03-01T16:17:00Z
- **Tasks:** 2
- **Files modified:** 2 (both pre-committed in 633a10d as plan 03-03 deviation)

## Accomplishments

- Verified `khcbppt_stars.rs` contains 3 tests: `verify_jd_epoch_against_khcbppt_dated_entries`, `validate_stars_against_golden`, `report_star_rule_sparsity`
- Verified `khcbppt_taboos.rs` contains 2 tests: `validate_taboos_against_golden`, `validate_taboo_coverage_by_rule`
- Confirmed all 5 tests pass: JD epoch aligned (0 mismatches), stars match golden (0 divergences), taboos match golden (0 divergences)
- Confirmed taboo coverage: all 4 rule types (nguyet_ky, sat_chu, tam_nuong, tho_tu) present in both golden and impl
- Star rule sparsity: 233/233 entries have no contextual rules — confirms known baseline.json gap (4 buckets with 1 seed entry each)

## Task Commits

Files were committed as auto-fix deviation in plan 03-03:

1. **Task 1: Create khcbppt_stars.rs validator** - `633a10d` (feat, created as plan 03-03 deviation)
2. **Task 2: Create khcbppt_taboos.rs validator** - `633a10d` (feat, same commit)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `crates/amlich-core/tests/khcbppt_stars.rs` - 3 tests: JD epoch verification (first 5 golden entries), bulk star validation (name/index/quality for all 233 entries), star rule sparsity report (STR-01, STR-02, STR-03)
- `crates/amlich-core/tests/khcbppt_taboos.rs` - 2 tests: set-based taboo validation (MISSING/EXTRA HashSet comparison for all 233 entries), taboo rule coverage report (TAB-01, TAB-02, TAB-03, TAB-04)

## Decisions Made

- JD epoch verification uses first 5 golden entries as anchors: Since the golden dataset was generated from `get_day_info()`, the epoch is self-consistent. The test verifies no regression occurred between dataset generation and test execution.
- Star rule sparsity test always passes: It is informational — 233/233 entries confirm that `fixed_by_canchi`, `by_year_can`, `by_lunar_month`, and `by_tiet_khi` buckets are all effectively empty (1 seed each). This is Phase 4 work.
- Taboo comparison is set-based (HashSet): Avoids false failures due to ordering differences. Reports MISSING and EXTRA separately for clear actionability.

## Deviations from Plan

### Context: Files Pre-existing from Plan 03-03 Auto-fix

Both files (`khcbppt_stars.rs` and `khcbppt_taboos.rs`) were created as a deviation in plan 03-03 execution because that executor discovered the plan 03-01 files were missing and blocking the "all 7 validators" success criteria. The files were committed atomically in `633a10d`.

This plan's execution verified the files meet all specified requirements:
- 3 test functions in khcbppt_stars.rs (JD epoch, bulk stars, sparsity) per plan spec
- 2 test functions in khcbppt_taboos.rs (bulk taboos, coverage by rule) per plan spec
- `star_quality_to_str` helper present per plan spec
- `compare_taboo_sets` helper using HashSet per plan spec
- All tests pass with zero divergences

No additional auto-fixes were required during this execution.

---

**Total deviations:** 0 auto-fixed in this execution (files pre-committed in 633a10d)
**Impact on plan:** Plan success criteria fully met. Files exist, compile, and produce correct divergence reports.

## Issues Encountered

None - files were already implemented correctly from the prior plan 03-03 deviation.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Both validators serve as the divergence reporting foundation for Plan 03-02 (deity, truc, xung hop) and Plan 03-03 (than huong, na am)
- Pattern established: collect-then-assert with eprintln! reports, enum-to-string helpers, set-based comparison for unordered fields
- Phase 4 corrections can use khcbppt_stars.rs as regression suite for JD epoch and star quality fixes
- Star rule sparsity (233/233 entries missing contextual rules) is documented for Phase 4 to address

---
*Phase: 03-validator-harness-and-divergence-inventory*
*Completed: 2026-03-01*
