---
phase: 14-phi-tinh-81-cell-aspects-safety-hints
plan: 03
subsystem: almanac
tags: [fengshui, phi-tinh, flying-stars, aspects, safety, integration-tests, no-product-names]

# Dependency graph
requires:
  - phase: 14-phi-tinh-81-cell-aspects-safety-hints
    plan: 01
    provides: lookup_star_pair_aspect, compute_palace_aspects, StarPairAspect, FsConfidenceTier, FsCitation
  - phase: 14-phi-tinh-81-cell-aspects-safety-hints
    plan: 02
    provides: is_danger_palace, element_hint_for_palace, RemedyHint
  - phase: 13-phi-tinh-primitives-period-annual-monthly
    provides: compute_combined_overlay, TietKhiScanner, FlyingStar

provides:
  - Black-box external-consumer integration tests for FS-11..FS-15 requirements
  - no_product_names_in_corpora corpus guard scanning all 9 hints + 81 aspect names
  - CI regression gate: turns RED if commercial terms appear in corpora

affects:
  - 15-semantic-graph-wiring (DTO integration can trust the public API surface is verified)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - External-crate black-box tests via `use amlich_core::...` (established in Phase 11)
    - Runtime corpus scan guard for forbidden terms (FORBIDDEN_PRODUCT_TERMS const array)
    - Collect violations into Vec, assert empty with descriptive message listing offenders

key-files:
  created:
    - crates/amlich-core/tests/fengshui_aspects.rs
  modified: []

key-decisions:
  - "Unused type imports (FsConfidenceTier, RemedyHint, StarPairAspect, flying_star_from_u8) removed — tests verify behavior via returned values, not type constructors; no need to name the types in imports"
  - "Both Task 1 (FS-11..15 tests) and Task 2 (no-product-names guard) committed in a single atomic commit — same file, both verified together, no benefit to splitting"
  - "ALL_STARS const array defined once and reused across tests — avoids repeating 9-variant list; mirrors plan guidance 'build the 9-variant array once and reuse'"

patterns-established:
  - "FORBIDDEN_PRODUCT_TERMS corpus guard: const &[&str] of commercial patterns; lowercase().contains() scan; collect violations; assert empty with offenders listed"
  - "External-consumer import pattern: use amlich_core::almanac::fengshui::{...} and use amlich_core::almanac::fengshui::types::FlyingStar"

requirements-completed: [FS-11, FS-12, FS-13, FS-14, FS-15]

# Metrics
duration: 2min
completed: 2026-05-28
---

# Phase 14 Plan 03: Black-box Integration Tests Summary

**External-consumer integration tests for all 5 Phase 14 requirements (FS-11..FS-15) plus a standing CI corpus guard that turns RED if commercial product terms appear in hint_text_vi or aspect names**

## Performance

- **Duration:** 2 min
- **Started:** 2026-05-27T18:38:40Z
- **Completed:** 2026-05-27T18:40:11Z
- **Tasks:** 2 (Task 1: FS-11..15 tests + Task 2: no-product-names guard)
- **Files modified:** 1

## Accomplishments

- Created `tests/fengshui_aspects.rs` with 6 black-box tests imported via `use amlich_core::...` (external-consumer perspective)
- `all_81_pairs_lookup_ordered` (FS-11): loops all 81 ordered pairs, verifies star_a/star_b match, proves asymmetry for NhatBach↔CuuTu
- `aspect_provenance_discipline` (FS-12): 6-pair sample asserts source_id == "huyen-khong" and non-empty citation title
- `compute_palace_aspects_matches_overlay` (FS-13): 2024-01 aspects[i].star_a/star_b verified against overlay.palace_overlays[i] for all 9 palaces
- `danger_palace_predicate` (FS-14): NguHoang+NhiHac return true; all other 7 stars return false
- `element_hint_present_for_danger_stars` (FS-15): Some for NguHoang/NhiHac with full field validation; None for NhatBach
- `no_product_names_in_corpora`: runtime corpus guard scanning all 9 hints + 81 aspect names against FORBIDDEN_PRODUCT_TERMS; standing CI regression gate; 691 lib tests + 6 new integration tests all green

## Task Commits

Each task was committed atomically:

1. **Tasks 1+2: FS-11..15 integration tests + no-product-names corpus guard** - `80130de` (feat)

**Plan metadata:** (pending)

## Files Created/Modified

- `crates/amlich-core/tests/fengshui_aspects.rs` — 329-line external-consumer test file with 6 tests: all_81_pairs_lookup_ordered, aspect_provenance_discipline, compute_palace_aspects_matches_overlay, danger_palace_predicate, element_hint_present_for_danger_stars, no_product_names_in_corpora

## Decisions Made

- **Unused type imports removed** — FsConfidenceTier, RemedyHint, StarPairAspect are in the re-export surface (plan specified them) but the tests verify behavior via returned values without naming the types explicitly; removing avoids Rust compiler warnings without reducing coverage.
- **Single atomic commit for both tasks** — Tasks 1 and 2 both modify the same file; both verified together in one test run; no reason to split the commit and add complexity.
- **ALL_STARS const array** — defined once at file scope, reused in multiple tests (FS-11, FS-15, no-product-names); matches plan guidance to "build the 9-variant array once and reuse".

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- FS-11: `lookup_star_pair_aspect` verified for all 81 ordered pairs with correct star_a/star_b and asymmetry proof — complete.
- FS-12: source_id and citation provenance verified across 6 representative pairs — complete.
- FS-13: `compute_palace_aspects` output proven consistent with `compute_combined_overlay` for a real date (2024-01) — complete.
- FS-14: `is_danger_palace` truth table verified for all 9 stars — complete.
- FS-15: `element_hint_for_palace` Some/None behavior verified; field constraints enforced — complete.
- no-product-names corpus guard is a standing CI regression gate — future corpus edits that introduce commercial terms will fail the test suite.
- Phase 14 is now COMPLETE (3/3 plans done). Phase 15 (Semantic Graph Wiring + DTO Integration) can proceed.

## Self-Check: PASSED

- `crates/amlich-core/tests/fengshui_aspects.rs` exists on disk (329 lines, > min 70)
- Contains `no_product_names` (2 occurrences) and `FORBIDDEN_PRODUCT_TERMS` (3 occurrences) and `use amlich_core::` (3 occurrences)
- Commit `80130de` exists in git log
- All 6 integration tests pass; full crate suite (691 lib + all integration tests) green

---
*Phase: 14-phi-tinh-81-cell-aspects-safety-hints*
*Completed: 2026-05-28*
