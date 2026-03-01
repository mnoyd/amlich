---
phase: 03-validator-harness-and-divergence-inventory
plan: 03
subsystem: testing
tags: [rust, cargo-test, golden-dataset, divergence-inventory, than-huong, na-am, khcbppt]

# Dependency graph
requires:
  - phase: 02-golden-dataset-and-loader
    provides: load_golden_dataset(), GoldenEntry with expected_xuat_hanh/expected_tai_than/expected_hy_than/expected_na_am/expected_element fields
  - phase: 03-validator-harness-and-divergence-inventory-plan-01
    provides: khcbppt_stars.rs, khcbppt_taboos.rs validators (created in this plan as deviation)
  - phase: 03-validator-harness-and-divergence-inventory-plan-02
    provides: khcbppt_deity.rs, khcbppt_truc.rs, khcbppt_xung_hop.rs validators
provides:
  - khcbppt_than_huong.rs: THH-01 validator comparing 3 travel direction fields across 233 entries
  - khcbppt_na_am.rs: NAM-01 validator comparing na_am and element across 233 entries
  - khcbppt_stars.rs: STR-01..03 validator with JD epoch verification, bulk stars, and sparsity report
  - khcbppt_taboos.rs: TAB-01..04 validator with set-based taboo comparison
  - Complete Phase 3 divergence inventory — all 7 khcbppt_*.rs validators operational
affects: [04-corrections, phase-4]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Collect-then-assert pattern: iterate all 233 entries, push to Vec<String> mismatches, assert mismatches.is_empty() with eprintln! report"
    - "Set-based taboo comparison using HashSet to avoid order sensitivity"
    - "Enum-to-string helpers for StarQuality and DayDeityClassification"
    - "Sorted Vec comparison for order-independent xung hop fields"

key-files:
  created:
    - crates/amlich-core/tests/khcbppt_than_huong.rs
    - crates/amlich-core/tests/khcbppt_na_am.rs
    - crates/amlich-core/tests/khcbppt_stars.rs
    - crates/amlich-core/tests/khcbppt_taboos.rs
  modified: []

key-decisions:
  - "Zero divergences expected for all non-star subsystems since golden dataset was generated from get_day_info() — validator confirms implementation consistency, not KHCBPPT correctness (Phase 4 work)"
  - "Stars and taboos validators created in this plan as auto-fix deviation (plan 01 files were not yet committed)"
  - "Collect-then-assert is the canonical Phase 3 pattern — all 7 validators use it"

patterns-established:
  - "Than huong: 3 direct string comparisons (xuat_hanh_huong, tai_than, hy_than) with no enum conversion needed"
  - "Na am: 2 direct string comparisons (na_am, element) — simplest validator pattern in the suite"
  - "Phase 3 validators are inventory tools, not correctness gates — divergences are reported, not fixed"

requirements-completed: [THH-01, NAM-01, STR-01, STR-02, STR-03, TAB-01, TAB-02, TAB-03, TAB-04]

# Metrics
duration: 12min
completed: 2026-03-01
---

# Phase 3 Plan 03: Than Huong and Na Am Validators Summary

**Two travel-direction and na-am validators complete all 7 khcbppt_*.rs Phase 3 divergence inventory files; 192 tests pass with zero divergences across all subsystems**

## Performance

- **Duration:** ~12 min
- **Started:** 2026-03-01T00:00:00Z
- **Completed:** 2026-03-01T00:12:00Z
- **Tasks:** 2
- **Files modified:** 4 (2 planned + 2 auto-fix deviation)

## Accomplishments

- Created `khcbppt_than_huong.rs` with 1 test comparing xuat_hanh_huong, tai_than, and hy_than for all 233 golden entries (THH-01)
- Created `khcbppt_na_am.rs` with 1 test comparing na_am and element for all 233 golden entries (NAM-01)
- Created `khcbppt_stars.rs` and `khcbppt_taboos.rs` (auto-fix: plan 01 files missing from previous execution)
- All 7 khcbppt_*.rs validators operational; `cargo test --package amlich-core` runs 192 tests with 0 failures

## Task Commits

Each task was committed atomically:

1. **Task 1: Create khcbppt_than_huong.rs and khcbppt_na_am.rs (+ deviation: stars and taboos)** - `633a10d` (feat)
2. **Task 2: Verify complete Phase 3 test suite** - verification only, no new commit needed

**Plan metadata:** (docs commit below)

## Files Created/Modified

- `crates/amlich-core/tests/khcbppt_than_huong.rs` - 1 test, 3 travel direction field comparisons (THH-01)
- `crates/amlich-core/tests/khcbppt_na_am.rs` - 1 test, 2 na am field comparisons (NAM-01)
- `crates/amlich-core/tests/khcbppt_stars.rs` - 3 tests: JD epoch verification, bulk star validation, sparsity report (STR-01..03)
- `crates/amlich-core/tests/khcbppt_taboos.rs` - 2 tests: set-based taboo comparison, coverage by rule (TAB-01..04)

## Decisions Made

- Zero divergences expected for all subsystems: golden dataset was generated from `get_day_info()` output, so implementation matches itself by construction. This is a tautological check establishing the harness infrastructure for Phase 4 corrections.
- khcbppt_stars.rs uses first 5 golden entries as JD epoch anchors (not hand-selected KHCBPPT dates) since the golden dataset was generated from the implementation itself — epoch is self-consistent.
- All validators use collect-then-assert pattern for complete divergence reporting (no first-failure panic).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Created khcbppt_stars.rs and khcbppt_taboos.rs (plan 01 files missing)**
- **Found during:** Task 1 (checking prerequisite files)
- **Issue:** Plan 03-01 files (khcbppt_stars.rs, khcbppt_taboos.rs) were not committed despite plan 03-02 files being present. Without them, the Task 2 "all 7 validators" verification would fail.
- **Fix:** Created both files per plan 03-01 spec (star_quality_to_str helper, JD epoch test, bulk star validation, sparsity report, compare_taboo_sets helper, taboo validation, coverage report).
- **Files modified:** crates/amlich-core/tests/khcbppt_stars.rs, crates/amlich-core/tests/khcbppt_taboos.rs
- **Verification:** `cargo test --package amlich-core --test khcbppt_stars --test khcbppt_taboos` passes
- **Committed in:** 633a10d (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (Rule 3 - blocking prerequisite)
**Impact on plan:** Auto-fix was necessary to achieve the plan's success criteria ("all 7 khcbppt_*.rs validator files exist"). No scope creep — files created exactly to plan 03-01 spec.

## Issues Encountered

None beyond the missing prerequisite files (handled as deviation above).

## Phase 3 Divergence Inventory Summary

All 7 validators ran across 233 golden entries with **zero divergences** in all subsystems:

| Validator | Subsystem | Tests | Result | Divergences |
|-----------|-----------|-------|--------|-------------|
| khcbppt_stars.rs | 28-star cycle | 3 | PASS | 0 |
| khcbppt_taboos.rs | Taboo rules | 2 | PASS | 0 |
| khcbppt_deity.rs | 12-deity cycle | 1 | PASS | 0 |
| khcbppt_truc.rs | Truc duty-star | 1 | PASS | 0 |
| khcbppt_xung_hop.rs | Xung/Hop | 1 | PASS | 0 |
| khcbppt_than_huong.rs | Travel directions | 1 | PASS | 0 |
| khcbppt_na_am.rs | Na am / element | 1 | PASS | 0 |

**Total test count:** 192 passed, 1 ignored (generate_golden #[ignore] test), 0 failed.

Zero divergences are expected because the golden dataset was generated from `get_day_info()` output. Phase 4 corrections will compare against the actual KHCBPPT reference tables, at which point divergences will appear and require fixes.

## Next Phase Readiness

- Phase 4 (corrections) can now run all 7 validators as a regression suite
- Any correction to baseline.json or source constants will immediately surface in the appropriate validator
- Star corrections should begin with JD epoch investigation (khcbppt_stars.rs already has the epoch test infrastructure)
- No source or data files were modified during Phase 3 (Success Criteria #4 confirmed via `git diff --name-only`)

---
*Phase: 03-validator-harness-and-divergence-inventory*
*Completed: 2026-03-01*
