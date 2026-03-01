---
phase: 03-validator-harness-and-divergence-inventory
plan: 02
subsystem: testing
tags: [rust, integration-tests, golden-dataset, divergence-report, almanac]

# Dependency graph
requires:
  - phase: 02-golden-dataset-and-loader
    provides: load_golden_dataset() API with 233-entry GoldenDataset
  - phase: 03-validator-harness-and-divergence-inventory
    provides: Plan 01 established khcbppt_taboos.rs pattern

provides:
  - khcbppt_deity.rs with validate_deity_against_golden and DayDeityClassification enum-to-string helper
  - khcbppt_truc.rs with validate_truc_against_golden comparing name, index, and quality
  - khcbppt_xung_hop.rs with validate_xung_hop_against_golden using sorted Vec comparison

affects: [04-corrections]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "collect-then-assert divergence pattern for complete mismatch inventory"
    - "DayDeityClassification enum-to-str helper for golden dataset comparison"
    - "sorted Vec comparison for order-independent tam_hop and tu_hanh_xung"

key-files:
  created:
    - crates/amlich-core/tests/khcbppt_deity.rs
    - crates/amlich-core/tests/khcbppt_truc.rs
    - crates/amlich-core/tests/khcbppt_xung_hop.rs
  modified: []

key-decisions:
  - "Deity validator handles Option<DayDeity> as mismatch (expected X, got NONE) rather than panicking"
  - "Xung hop Vec comparison uses sort-then-compare (not HashSet) to preserve duplicate detection while being order-independent"

patterns-established:
  - "classification_to_str helper: match on DayDeityClassification enum, return &'static str for golden comparison"
  - "Sorted Vec pattern: clone both sides, sort both, compare -- preserves duplicates unlike HashSet"

requirements-completed: [DEI-01, DEI-02, TRC-01, XH-01]

# Metrics
duration: 2min
completed: 2026-03-01
---

# Phase 3 Plan 02: Deity, Truc, and Xung Hop Validators Summary

**Three integration test validators (deity with enum-to-str helper, truc with 3-field comparison, xung hop with sorted Vec) covering 4 requirement IDs, all producing zero-divergence reports across 233 golden entries.**

## Performance

- **Duration:** ~2 min
- **Started:** 2026-03-01T16:08:01Z
- **Completed:** 2026-03-01T16:09:07Z
- **Tasks:** 2
- **Files modified:** 3 (all created)

## Accomplishments
- khcbppt_deity.rs: validate_deity_against_golden with DayDeityClassification enum-to-string helper; handles Option<DayDeity> None as mismatch (DEI-01, DEI-02)
- khcbppt_truc.rs: validate_truc_against_golden comparing name, index, and quality per entry using collect-then-assert pattern (TRC-01)
- khcbppt_xung_hop.rs: validate_xung_hop_against_golden with luc_xung direct comparison and sorted Vec comparison for tam_hop and tu_hanh_xung (XH-01)
- All three produce complete divergence reports and pass with 0 mismatches across all 233 golden entries

## Task Commits

Each task was committed atomically:

1. **Task 1: Create khcbppt_deity.rs and khcbppt_truc.rs validators** - `77f927c` (feat)
2. **Task 2: Create khcbppt_xung_hop.rs validator** - `a2f11ff` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified
- `crates/amlich-core/tests/khcbppt_deity.rs` - Day deity validator with classification_to_str helper; Option<DayDeity> handling; covers DEI-01 and DEI-02
- `crates/amlich-core/tests/khcbppt_truc.rs` - Truc duty-star validator comparing name, index, quality; canonical collect-then-assert example (TRC-01)
- `crates/amlich-core/tests/khcbppt_xung_hop.rs` - Xung hop validator with sorted Vec comparison for tam_hop and tu_hanh_xung (XH-01)

## Decisions Made
- Deity Option<None> handled as mismatch string ("expected X, got NONE") rather than panicking -- consistent with collect-then-assert pattern requiring all 233 entries to process regardless of missing data
- Sorted Vec used for tam_hop/tu_hanh_xung comparison instead of HashSet -- preserves duplicate detection capability while remaining order-independent, as specified in plan

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered
None.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- All 4 requirements (DEI-01, DEI-02, TRC-01, XH-01) now have active validators producing zero divergences
- Plan 03 (stars, than_huong, na_am) is ready to proceed
- Pattern library is complete: enum-to-str helper, sorted Vec, collect-then-assert, Option handling

---
*Phase: 03-validator-harness-and-divergence-inventory*
*Completed: 2026-03-01*

## Self-Check: PASSED

- FOUND: crates/amlich-core/tests/khcbppt_deity.rs
- FOUND: crates/amlich-core/tests/khcbppt_truc.rs
- FOUND: crates/amlich-core/tests/khcbppt_xung_hop.rs
- FOUND commit: 77f927c (deity and truc validators)
- FOUND commit: a2f11ff (xung hop validator)
