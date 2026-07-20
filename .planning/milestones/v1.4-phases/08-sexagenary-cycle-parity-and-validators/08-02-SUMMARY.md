---
phase: 08-sexagenary-cycle-parity-and-validators
plan: 02
subsystem: validation
tags: [sexagenary-cycle, parity-validator, regression-tests, rust]

# Dependency graph
requires:
  - phase: 07-hour-pillar-parity-core
    provides: Hour pillar parity validation patterns
provides:
  - Full-table parity validator confirming exact 60-cycle mapping against baseline
  - Regression guards for index normalization and invalid input handling
  - Bidirectional roundtrip validation for all 60 valid positions
affects: [08-sexagenary-cycle-parity-and-validators]

# Tech tracking
tech-stack:
  added: [sexagenary_cycle module with 1-based cycle utilities]
  patterns: [TDD RED-GREEN-REFACTOR, baseline-driven validation, divergence reporting]

key-files:
  created: [crates/amlich-core/src/almanac/sexagenary_cycle.rs, crates/amlich-core/tests/sexagenary_cycle_parity.rs]
  modified: [crates/amlich-core/src/almanac/mod.rs]

key-decisions:
  - "Chinese Remainder Theorem for canchi_to_cycle_index inversion - ensures correct cycle index recovery"
  - "1-based public API with 0-based internal arithmetic - matches Vietnamese convention"
  - "Chinese Remainder Theorem for inversion uses k = ((can - chi) / 2) % 6 formula"

patterns-established:
  - "Pattern: Full-table parity validation - iterate all 60 positions against baseline reference"
  - "Pattern: Divergence reporting with formatted mismatches and --nocapture output"
  - "Pattern: Regression tests covering bounds, rollover, and roundtrip conversion"

requirements-completed: [SC-05, PAR-01]

# Metrics
duration: 4 min
completed: 2026-03-03
---

# Phase 08: Plan 02 Summary

**Full-table parity validator confirming exact 60-cycle mapping against baseline na_am_pairs with regression guards for boundary cases and invalid inputs.**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-03T17:12:58Z
- **Completed:** 2026-03-03T17:17:50Z
- **Tasks:** 2 (both TDD)
- **Files modified:** 2

## Accomplishments

- Implemented `sexagenary_cycle` module with three public functions for 60-cycle utilities
- Created full-table parity validator iterating over all 60 positions against baseline
- Added regression tests for index normalization, invalid input rejection, and boundary rollover
- All 19 tests pass (7 integration + 12 unit)

## Task Commits

Each task was committed atomically:

1. **Task 1: Implement full 60-cycle parity validator** - `a15b94c` (test → feat)
2. **Task 1 (GREEN): Implement functions** - `94774ff` (feat)
3. **Task 2: Add regression tests** - `d4c4a9b` (feat)

**Plan metadata:** (pending final commit)

_Note: TDD tasks produced 3 commits (test + feat for Task 1, feat for Task 2). REFACTOR phase not needed - implementation is already minimal and clean._

## Files Created/Modified

- `crates/amlich-core/src/almanac/sexagenary_cycle.rs` - 60-cycle conversion and progression utilities with 12 unit tests
- `crates/amlich-core/tests/sexagenary_cycle_parity.rs` - Full-table parity validator with 7 integration tests
- `crates/amlich-core/src/almanac/mod.rs` - Export `pub mod sexagenary_cycle`

## Decisions Made

- **Chinese Remainder Theorem for inversion**: Used CRT-based formula `k = ((can - chi) / 2) % 6` then `cycle_index = (can + 10 * k) % 60` to correctly recover cycle index from stem/branch indices, ensuring bidirectional roundtrip works for all 60 positions.

- **1-based public API**: Public functions use 1-based indices (1-60) to match Vietnamese convention, while internal arithmetic converts to 0-based (0-59) for modulo correctness.

- **Validation-first approach**: `canchi_to_cycle_index` validates canonical combination (same parity) before computing index, returning `None` for non-canonical odd/even mismatches.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 8 SC-05 and PAR-01 requirements satisfied with test-backed validation
- Full 60-cycle parity confirmed against baseline reference
- Regression guards protect against future drift in cycle arithmetic
- Ready for Phase 9 Na Am API implementation

---
*Phase: 08-sexagenary-cycle-parity-and-validators*
*Completed: 2026-03-03*

## Self-Check: PASSED

✓ SUMMARY.md exists at correct path
✓ Test file sexagenary_cycle_parity.rs exists
✓ Commits exist: a15b94c, 94774ff, d4c4a9b
✓ All 19 tests passing (7 integration + 12 unit)
