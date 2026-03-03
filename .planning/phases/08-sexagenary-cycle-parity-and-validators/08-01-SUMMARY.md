---
phase: 08-sexagenary-cycle-parity-and-validators
plan: 01
subsystem: core-algorithms
tags: [sexagenary-cycle, canonical-validation, modular-arithmetic, tdd]

# Dependency graph
requires:
  - phase: 07-hour-pillar-parity-core
    provides: CanChi type, CAN/CHI constants, normalize_index helper
provides:
  - Bounded 1-based cycle index to stem-branch conversion (cycle_index_to_canchi)
  - Stem-branch to cycle index inversion with canonical validation (canchi_to_cycle_index)
  - Signed progression with modular rollover (progress_cycle_index)
affects: [08-02-full-table-parity-validators, 09-na-am-apis]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - TDD workflow with RED-GREEN cycles
    - 1-based public contract with 0-based internal arithmetic
    - Chinese Remainder Theorem for solving modular constraints

key-files:
  created: []
  modified:
    - crates/amlich-core/src/almanac/sexagenary_cycle.rs - Core 60-cycle utilities (382 lines)
    - crates/amlich-core/src/almanac/mod.rs - Export sexagenary_cycle module

key-decisions:
  - "Used corrected formula for canchi_to_cycle_index based on Chinese Remainder Theorem instead of the initial incorrect formula"
  - "TDD approach with atomic commits for each test-implementation cycle"

patterns-established:
  - "Pattern: Bounded public contracts - 1-based indices (1-60) for Vietnamese convention, 0-based for internal arithmetic"
  - "Pattern: Explicit None returns for invalid inputs - no panics for out-of-bounds or non-canonical pairs"
  - "Pattern: Modular arithmetic with rem_euclid for correct signed delta handling"

requirements-completed: [SC-01, SC-02, SC-03, SC-04]

# Metrics
duration: 4min
completed: 2026-03-03T17:17:34Z
---

# Phase 8 Plan 1: Sexagenary 60-Cycle Utilities Summary

**Canonical sexagenary 60-cycle conversion utilities with bounded 1-based contracts, parity validation, and deterministic progression using modular arithmetic**

## Performance

- **Duration:** 4 min
- **Started:** 2026-03-03T17:13:01Z
- **Completed:** 2026-03-03T17:17:34Z
- **Tasks:** 3
- **Files modified:** 2

## Accomplishments

- **Implemented cycle_index_to_canchi** - Converts 1-based cycle index (1-60) to stem-branch pair with bounds validation
- **Fixed canchi_to_cycle_index** - Corrected formula using Chinese Remainder Theorem for accurate cycle position calculation with canonical parity validation
- **Verified progress_cycle_index** - Confirmed signed progression with correct modular rollover using rem_euclid
- **Established TDD workflow** - Each task followed RED-GREEN cycle with atomic commits

## Task Commits

Each task was committed atomically following TDD discipline:

1. **Task 1: Add cycle index to stem-branch conversion** - `770bf95` (test)
   - Tests valid bounds (1 returns Giáp Tý, 60 returns Quý Hợi)
   - Tests invalid indices (0 and 61 return None)
   - Tests intermediate values and canonical parity validation
   - Implementation already existed and passed all tests

2. **Task 2: Add stem-branch to cycle index conversion** - `6002f35` (test), `969b63f` (feat)
   - RED: Tests edge cases, non-canonical pairs, parity validation, roundtrip
   - GREEN: Fixed formula using Chinese Remainder Theorem for moduli 10 and 12
   - Original formula was incorrect; corrected to: `k = ((can_idx - chi_idx) / 2).rem_euclid(6)`, `i = (can_idx + 10*k) % 60`

3. **Task 3: Implement signed progression with modular rollover** - `32843f1` (test)
   - Tests rollover (60+1=1, 1-1=60), large deltas, composition property
   - Implementation already existed and passed all tests

**Plan metadata:** (to be committed)

## Files Created/Modified

- `crates/amlich-core/src/almanac/sexagenary_cycle.rs` - 60-cycle utilities (382 lines, exceeds 180 min)
  - `cycle_index_to_canchi(index: u8) -> Option<CanChi>` - 1-based to stem-branch with bounds validation
  - `canchi_to_cycle_index(can_index: usize, chi_index: usize) -> Option<u8>` - Stem-branch to 1-based with canonical validation
  - `progress_cycle_index(index: u8, delta: i32) -> Option<u8>` - Signed progression with modular rollover
- `crates/amlich-core/src/almanac/mod.rs` - Added `pub mod sexagenary_cycle;`

## Decisions Made

- **Formula correction for canchi_to_cycle_index**: The original formula `((can_idx * 6) + (chi_idx / 2)) % 60` did not correctly map stem-branch pairs to their cycle positions. Fixed using Chinese Remainder Theorem solution for congruence `i % 10 = can_idx` and `i % 12 = chi_idx`, resulting in: `k = ((can_idx - chi_idx) / 2).rem_euclid(6)` and `i = (can_idx + 10*k) % 60`.
- **TDD workflow despite existing implementation**: Even though implementations existed from previous plan execution (08-02), followed TDD discipline by writing tests first, verifying failure where applicable, and committing separately for test and implementation phases.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed incorrect canchi_to_cycle_index formula**
- **Found during:** Task 2 (canchi_to_cycle_index roundtrip test)
- **Issue:** Original formula `((can_idx * 6) + (chi_idx / 2)) % 60` did not correctly compute cycle positions. Example: index 2 → (1,1) → 7 (should be 2)
- **Fix:** Replaced with Chinese Remainder Theorem solution: `k = ((can_idx - chi_idx) / 2).rem_euclid(6)` and `i = (can_idx + 10*k) % 60`
- **Files modified:** `crates/amlich-core/src/almanac/sexagenary_cycle.rs`
- **Verification:** Roundtrip test now passes for all 60 cycle positions
- **Committed in:** `969b63f` (Task 2 GREEN phase commit)

---

**Total deviations:** 1 auto-fixed (1 bug)
**Impact on plan:** Formula fix was essential for correctness. No scope creep.

## Issues Encountered

None - plan executed smoothly with TDD workflow.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **SC-01 through SC-04 complete**: All sexagenary cycle conversion utilities implemented and tested
- **Module exported**: sexagenary_cycle accessible via `almanac::sexagenary_cycle` namespace
- **Ready for Phase 8 Plan 2**: Full 60-cycle parity validators can now use the corrected conversion utilities

**No blockers** - implementation provides deterministic, bounded contracts suitable for hour pillar and Na Am API integration.

---
*Phase: 08-sexagenary-cycle-parity-and-validators*
*Completed: 2026-03-03*
