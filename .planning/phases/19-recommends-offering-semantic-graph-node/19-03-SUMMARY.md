---
phase: 19-recommends-offering-semantic-graph-node
plan: 03
subsystem: testing
tags: [rust, cargo, serde, backward-compat, e2e-smoke, semantic-graph, integration-tests, Offering, RecommendsOffering]

# Dependency graph
requires:
  - phase: 19-01
    provides: "OfferingRef struct + SourceId alias + DaySnapshot.offering_refs + DaySnapshot.offerings additive fields"
  - phase: 19-02
    provides: "NodeConcept::Offering + EdgeConcept::RecommendsOffering across all 6 ontology slices + SemanticNode payload + INT-09 dual-source provenance + build_day_snapshot_graph builder"
provides:
  - "3 v1.5→v1.6 backward-compat round-trip tests for offering_refs + offerings additive fields (BLOCKER 5 FIX strips daily_flying_stars + offering_refs + offerings together)"
  - "1 E2E 2026 smoke test exercising Offering wiring on >=5 representative dates with daily_flying_stars + offering_refs + semantic-graph Offering nodes + RecommendsOffering edges"
  - "BLOCKER 6 endpoint verification (from_node=Ritual, to_node=Offering) + INT-09 dual-source provenance verification (vn-folk-ritual + huyen-khong)"
  - "BLOCKER 7 annual + monthly FlyingStar component verification on palace_overlays tuples"
  - "Public re-export of build_day_snapshot_graph at semantic_graph crate root (mirrors build_reasoning_input_graph pattern)"
affects: [v1.6-release, integration-tests, semantic-graph-api]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Private-module re-export at crate root (pub use) — mirrors existing pattern for build_reasoning_input_graph when the deep path traverses private modules"
    - "Combined-strip round-trip: remove ALL v1.6-new additive fields together to simulate v1.5 fixture shape, then assert byte-equal re-serialization (extends Phase 18-04 single-strip pattern)"

key-files:
  created: []
  modified:
    - "crates/amlich-core/tests/day_snapshot_v14_compat.rs (+167 lines: 3 new round-trip tests + 2 imports)"
    - "crates/amlich-core/tests/integration_2026_smoke.rs (+241 lines: 1 new E2E smoke test + 2 imports)"
    - "crates/amlich-core/src/semantic_graph/mod.rs (+1/-1 line: build_day_snapshot_graph added to pub use re-exports)"

key-decisions:
  - "Re-export build_day_snapshot_graph at the semantic_graph crate root instead of making builders + day_snapshot modules public. The plan's literal import path (amlich_core::semantic_graph::builders::day_snapshot::build_day_snapshot_graph) is unreachable because both builders and day_snapshot are private mod. The re-export is the minimal, idiomatic fix and mirrors the existing build_reasoning_input_graph re-export pattern."
  - "Task 2 import path adjusted from plan-specified amlich_core::semantic_graph::builders::day_snapshot::build_day_snapshot_graph to amlich_core::semantic_graph::build_day_snapshot_graph (consumes the new re-export). The other import (EdgeConcept, NodeConcept) is unchanged from the plan."

patterns-established:
  - "Pattern: when a builder function lives under a private module subtree, expose it via pub use at the crate-root semantic_graph module rather than flipping private modules to pub — keeps the public API surface tight"
  - "Pattern: combined-strip v1.5→v1.6 round-trip test (strip ALL v1.6-new additive fields in one assertion) is the gold standard for additive DTO discipline going forward"

requirements-completed: [INT-10]

# Metrics
duration: 18 min
completed: 2026-07-16
---

# Phase 19 Plan 03: INT-10 Backward-Compat Round-Trip + 2026 E2E Offering Wiring Summary

**3 new v1.5→v1.6 round-trip tests for offering_refs + offerings (BLOCKER 5 strips all 3 v1.6 fields together) + 1 E2E 2026 smoke test exercising Offering node + RecommendsOffering edge wiring on >=5 representative dates with endpoint + INT-09 dual-source + annual/monthly FlyingStar component verification.**

## Performance

- **Duration:** ~18 min (Task 1 commit c0a37c0 at 00:11+07 → Task 2 commit bd5aeda at 00:28+07)
- **Started:** 2026-07-15T17:11:17Z (prior attempt; resumed on 2026-07-16)
- **Completed:** 2026-07-15T17:28:57Z
- **Tasks:** 2
- **Files modified:** 3 (2 test files + 1 source file for re-export)

## Accomplishments
- INT-10 fully closed: both sub-criteria satisfied — v1.5→v1.6 backward-compat round-trip AND >=5-date 2026 E2E smoke exercising BOTH annual/monthly fields AND new daily field with semantic-graph wiring verified.
- 9/9 day_snapshot_v14_compat tests pass (3 pre-existing v1.4→v1.5 + 3 Phase 18 v1.5→v1.6 daily_flying_stars + 3 NEW Phase 19 v1.5→v1.6 offering_refs).
- 4/4 integration_2026_smoke tests pass (3 pre-existing + 1 NEW offering-wiring smoke).
- BLOCKER 5 FIX: Test 7 strips daily_flying_stars + offering_refs + offerings together (v1.5 fixture shape: has flying_stars, no v1.6 fields) and re-serializes the recovered v1.6 value to assert byte-equal round-trip + no unexpected fields.
- BLOCKER 6 FIX: Every RecommendsOffering edge verified to have from_node_id pointing to a Ritual node + to_node_id pointing to an Offering node, AND at least one date in the set (Tết 2026) exercises the INT-09 dual-source pattern (edge provenance contains BOTH vn-folk-ritual + huyen-khong).
- BLOCKER 7 FIX: Each flying_stars.palace_overlays[i] tuple's annual + monthly components asserted as valid FlyingStar variants via matches! against all 9 variants.
- Multi-source surface coexistence gate passed: flying_stars + applicable_rituals + daily_flying_stars + offering_refs + offerings all coexist on DaySnapshot without conflicts.
- 716 lib tests + all integration tests pass — zero regressions vs Phase 19-02 baseline.

## Task Commits

Each task was committed atomically:

1. **Task 1: Add 3 offering_refs round-trip tests to tests/day_snapshot_v14_compat.rs** — `c0a37c0` (test)
2. **Task 2: Add E2E 2026 smoke test for Offering wiring on representative dates** — `bd5aeda` (feat)

_Note: Task 1 was committed in a prior (cancelled) attempt; this execution resumed from the partial state, finished Task 2, verified both tasks compile + pass together, and produced the SUMMARY/STATE/ROADMAP metadata._

## Files Created/Modified
- `crates/amlich-core/tests/day_snapshot_v14_compat.rs` — 3 NEW round-trip tests (Test 7 v15_json_without_v16_fields_deserializes_and_round_trips, Test 8 offering_refs_byte_equal_round_trip, Test 9 offering_refs_absent_when_none) + 2 imports (OfferingRef + SOURCE_VN_FOLK_RITUAL)
- `crates/amlich-core/tests/integration_2026_smoke.rs` — 1 NEW E2E smoke test (e2e_2026_smoke_offering_wiring_on_representative_dates) + 2 imports (build_day_snapshot_graph + EdgeConcept/NodeConcept)
- `crates/amlich-core/src/semantic_graph/mod.rs` — added build_day_snapshot_graph to the pub use re-export at the crate root (Rule 3 deviation, see below)

## Decisions Made
- **Re-export build_day_snapshot_graph at semantic_graph crate root (not via pub mod):** The plan's literal import path `amlich_core::semantic_graph::builders::day_snapshot::build_day_snapshot_graph` is unreachable from external consumers because both `builders` and `day_snapshot` are `mod` (private). Two options considered: (a) flip both modules to `pub mod`, or (b) add a single `pub use` re-export at the crate root. Chose (b) — minimal API surface change, mirrors the existing re-export pattern for `build_reasoning_input_graph`, and keeps the builder module subtree private. The test import path was correspondingly simplified to `amlich_core::semantic_graph::build_day_snapshot_graph`.
- **Task 2 import path adjusted to consume the re-export:** Test imports `amlich_core::semantic_graph::build_day_snapshot_graph` instead of the plan's `amlich_core::semantic_graph::builders::day_snapshot::build_day_snapshot_graph`. The other import (`use amlich_core::semantic_graph::{EdgeConcept, NodeConcept};`) is unchanged from the plan.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Re-export build_day_snapshot_graph at semantic_graph crate root**
- **Found during:** Task 2 (E2E 2026 smoke test)
- **Issue:** The plan specified the import path `amlich_core::semantic_graph::builders::day_snapshot::build_day_snapshot_graph` in both the `<action>` block and the `<done>` criteria. This path is unreachable because `crates/amlich-core/src/semantic_graph/builders/mod.rs` declares `mod builders;` (private) and `mod day_snapshot;` (private). Without a public re-export, the test would fail to compile with `error[E0603]: module builders is private`.
- **Fix:** Added `build_day_snapshot_graph` to the existing `pub use builders::{...}` line in `crates/amlich-core/src/semantic_graph/mod.rs` (line 11). This is the idiomatic Rust pattern already used by the sibling `build_reasoning_input_graph` re-export. The test consumes the new path `amlich_core::semantic_graph::build_day_snapshot_graph` instead of the deep unreachable path.
- **Files modified:** `crates/amlich-core/src/semantic_graph/mod.rs` (+1/-1 line)
- **Verification:** `cargo build -p amlich-core` clean; `cargo test -p amlich-core --test integration_2026_smoke` reports 4/4 passing (3 pre-existing + 1 new). The re-export also enables external consumers (e.g., future app/CLI layers) to call the builder without us having to mark the builder subtree public.
- **Committed in:** `bd5aeda` (Task 2 commit — same commit as the new test, since they are mutually required to compile)

---

**Total deviations:** 1 auto-fixed (1 blocking)
**Impact on plan:** The deviation is the minimum required to make Task 2's test compile. It does not change any existing behavior — `build_day_snapshot_graph` was already callable via the private module path internally; the re-export only exposes it externally. No scope creep, no architectural change, no behavioral change to the semantic-graph builder.

## Issues Encountered
- **Resumed from a cancelled prior attempt.** Task 1 was already committed as `c0a37c0`. The working tree contained an in-progress Task 2 (the new E2E smoke test in `integration_2026_smoke.rs` plus the necessary `mod.rs` re-export). After inspecting the uncommitted diff, the work was judged sound and near-complete (build clean, all 4 integration tests pass), so it was finished in place rather than reset. The unrelated `.opencode/package-lock.json` change in the working tree was left untouched and explicitly excluded from the Task 2 commit (staged only the two relevant files).

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness
- Phase 19 COMPLETE (3/3 plans). INT-07 + INT-08 + INT-09 + INT-10 all closed.
- v1.6 Eastern Knowledge Completion milestone: all 12 requirements satisfied (FND-07/08, RIT-14/15/16, FS-16/17/18/19, INT-07/08/09/10).
- All test gates green: 716 lib tests + 9 day_snapshot_v14_compat round-trip tests + 4 integration_2026_smoke tests + 1 source_id_guard test pass with zero regressions.
- No blockers. Ready for the next milestone phase (transition: v1.6 release prep / v1.7 planning).

## Self-Check: PASSED

- FOUND: crates/amlich-core/tests/day_snapshot_v14_compat.rs
- FOUND: crates/amlich-core/tests/integration_2026_smoke.rs
- FOUND: crates/amlich-core/src/semantic_graph/mod.rs
- FOUND: .planning/phases/19-recommends-offering-semantic-graph-node/19-03-SUMMARY.md
- FOUND: both task commits (c0a37c0 Task 1, bd5aeda Task 2)

---
*Phase: 19-recommends-offering-semantic-graph-node*
*Completed: 2026-07-16*
