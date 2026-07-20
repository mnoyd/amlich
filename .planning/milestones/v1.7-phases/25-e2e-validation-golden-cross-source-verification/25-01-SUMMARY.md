---
phase: 25-e2e-validation-golden-cross-source-verification
plan: 01
subsystem: testing
tags: [e2e-smoke, baseline-guard, iching, semantic-graph, golden-dataset, int-13, v1.7-milestone-closure]

# Dependency graph
requires:
  - phase: 24-iching-evaluator-semantic-graph-wiring-dto-integration
    provides: IChingEvaluator + IChingCastSummary + enrich_day_snapshot_with_iching + DaySnapshotGraphBuilder::add_iching_facts + add_direction_composite_facts + DaySnapshot additive fields (iching_cast + direction_cross_link)
  - phase: 23-th-i-tu-tam-s-t-phi-tinh-cross-link
    provides: DirectionCrossLinkSummary + enrich_day_snapshot_with_direction_cross_link + DATE_ONLY_BIRTH_CHI_INDEX sentinel + dual-source provenance (KHCBPPT + HUYEN_KHONG + rule.composite.direction_cross_link)
  - phase: 22-mai-hoa-casting-bien-que-the-dung
    provides: cast_mai_hoa + derive_bien_que + classify_the_dung + load_mai_hoa_golden + MaiHoaGoldenDataset
provides:
  - e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates (Phase 25 SC2 — unified v1.7 surface E2E test)
  - cargo_dependency_tree_unchanged_from_v16 (Phase 25 SC4 — runtime-enforced dep-tree lock)
  - int13_golden_dataset_cross_source_discipline_holds (Phase 25 SC1 — INT-13 cross-source golden discipline sentinel)
affects: [v1.8+, gsd-complete-milestone]

# Tech tracking
tech-stack:
  added: []  # Phase 25 adds NO new deps — pure validation phase
  patterns:
    - "include_str! + ad-hoc parser for runtime Cargo.toml dependency-tree guard (faster + cargo-version-robust vs invoking `cargo tree` at test time)"
    - "Function-body `use` block for additive test functions (preserves existing module-level imports when appending to a shared test file)"
    - "Compare-against-base-snapshot pattern for semantic-graph wiring tests (proves enrichment adds nodes/edges, not pre-existing)"
    - "Sort-both-sides comparison for SET-shaped invariants (locks dep name set without enforcing declaration order)"

key-files:
  created:
    - crates/amlich-core/tests/v17_baseline_guards.rs
  modified:
    - crates/amlich-core/tests/integration_2026_smoke.rs

key-decisions:
  - "Phase 25 closure: SC1 (golden discipline) + SC2 (unified E2E smoke) + SC3 (full suite green) + SC4 (dep-tree lock) all met by 2 task commits. INT-13 closed; v1.7 milestone ready for /gsd-complete-milestone."
  - "include_str!-based Cargo.toml parser over `cargo tree` CLI invocation: faster + cargo-version-robust + locks the same dep-name SET. Sorts both sides so declaration order is not enforced."
  - "Test 2 re-asserts INT-13's discipline explicitly even though the Phase 22-02 loader already validates the same invariants — formal Phase 25 closure sentinel so a future weakening of the loader would still trip this test."
  - "Function-body `use` blocks inside the new test preserve the file's existing module-level imports (purely additive — no changes to lines 1-30 of integration_2026_smoke.rs)."

patterns-established:
  - "Phase 25 milestone-closure discipline: pure validation phase (NO new production code, NO new deps); runtime-invariant baseline guards lock the v1.6 baseline + INT-13 golden discipline so v1.8+ cannot silently drift."
  - "Sort-both-sides comparison: for SET-shaped invariants (e.g. dep names), sort both actual + expected vectors before assert_eq! — locks the set without enforcing declaration order in the source file."

requirements-completed: [INT-13]

# Metrics
duration: 7 min
completed: 2026-07-19
---

# Phase 25 Plan 01: E2E Validation + Golden Cross-Source Verification Summary

**v1.7 milestone closure plan — unified IChing + cross-link E2E smoke on representative 2026 dates + runtime baseline guards locking the dep-tree shape (SC4) and the INT-13 cross-source golden discipline (SC1). Phase 25 adds NO production code; pure validation.**

## Performance

- **Duration:** 7 min
- **Started:** 2026-07-19T17:28:36Z
- **Completed:** 2026-07-19T17:35:04Z
- **Tasks:** 2
- **Files modified:** 2 (1 existing test file appended; 1 new test file created)

## Accomplishments

- **SC2 met** — `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates` exercises ALL FIVE v1.7 surfaces together on Tết 2026 + 4 Sóc dates spanning distinct lunar months: Phase 22 casting chain (cast_mai_hoa + derive_bien_que + classify_the_dung with CRIT-4 biến ≠ chủ King Wen invariant), Phase 24-01 immutable IChing enrichment (CRIT-6 4-envelope evidence contract verified — 2 SOURCE_MAI_HOA_DICH_SO + 1 SOURCE_KINH_DICH + 1 rule.composite.iching_consultation; input snapshot NOT mutated), Phase 23-03 immutable direction cross-link enrichment (8-cell + ≥3-envelope dual-source provenance with KHCBPPT + HUYEN_KHONG + rule.composite.*; input snapshot unchanged AND iching_cast preserved across both-field coexistence), Phase 24-02 semantic-graph wiring (≥2 NodeConcept::Hexagram + ≥1 EdgeConcept::Transforms + ≥1 NodeConcept::Direction composite; STRICTLY more than the un-enriched base snapshot's graph).
- **SC4 met** — `cargo_dependency_tree_unchanged_from_v16` parses `crates/amlich-core/Cargo.toml` via `include_str!` + ad-hoc parser, asserts the `[dependencies]` section contains EXACTLY 4 entries with the locked v1.6 names (chrono + serde + serde_json + unicode-normalization). Locks the SET of deps (sorts both sides) so declaration order is not enforced.
- **SC1 met** — `int13_golden_dataset_cross_source_discipline_holds` re-asserts INT-13's full discipline holistically: ≥10 cases, every case has ≥2 sources with `nhantu.net` present in ≥1, ≥1 KnownDivergence row with non-empty fields (our_value + tiebreaker + note), schema_version pinned to `mai-hoa-golden-v1`. Formal Phase 25 closure sentinel — a future weakening of the Phase 22-02 loader would still trip this test.
- **SC3 met** — full crate test suite green: **1120 tests passed, 0 failed, 7 ignored** (was 1117 at Phase 24-end baseline; +1 from Task 1's smoke test, +2 from Task 2's baseline guards). Zero regressions on the v1.6-vintage test files.
- **Dep tree unchanged** — `cargo tree -p amlich-core --depth 1` confirms exactly 4 deps (chrono v0.4.44 + serde v1.0.228 + serde_json v1.0.149 + unicode-normalization v0.1.25); zero new deps despite Phase 25's validation surface.

## Task Commits

Each task was committed atomically:

1. **Task 1: Extend integration_2026_smoke.rs with v1.7 E2E smoke test** — `71d4c72` (test)
2. **Task 2: Add v17_baseline_guards.rs (dep-tree + golden-dataset INT-13 sentinel)** — `4921a1c` (test)

**Plan metadata:** pending (docs: complete plan — will land in final metadata commit)

## Files Created/Modified

- `crates/amlich-core/tests/integration_2026_smoke.rs` — Appended `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates` (+416 lines, total file now 886 lines). Purely additive — existing 4 tests (e2e_2026_smoke_all_categories, tet_2026_is_lunar_1_1, van_boundary_8_to_9, e2e_2026_smoke_offering_wiring_on_representative_dates) untouched. Mirrors Phase 19 INT-10's pattern of appending to the shared INT-06 file. Function-body `use` blocks preserve the file's existing module-level imports.
- `crates/amlich-core/tests/v17_baseline_guards.rs` — NEW file (236 lines) with TWO tests: `cargo_dependency_tree_unchanged_from_v16` (SC4) + `int13_golden_dataset_cross_source_discipline_holds` (SC1). Uses ONLY `amlich_core`'s public API + std library; `include_str!("../Cargo.toml")` embeds the manifest at compile time.

## Test Names + Line Counts

| Test name | File | Lines |
|-----------|------|-------|
| `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates` | `crates/amlich-core/tests/integration_2026_smoke.rs` | ~416 (lines 471-886) |
| `cargo_dependency_tree_unchanged_from_v16` | `crates/amlich-core/tests/v17_baseline_guards.rs` | ~75 (lines 47-121) |
| `int13_golden_dataset_cross_source_discipline_holds` | `crates/amlich-core/tests/v17_baseline_guards.rs` | ~80 (lines 137-233) |

## Decisions Made

- **`include_str!` + ad-hoc parser over `cargo tree` CLI:** The plan's discipline constraint said "Do NOT use `std::process::Command` to invoke `cargo tree` at test time (slow + brittle across cargo versions)". The chosen approach embeds Cargo.toml at compile time via `include_str!("../Cargo.toml")` and parses the `[dependencies]` section by locating the `\n[dependencies]` marker + next `\n[` section start. Sorts both actual + expected dep-name vectors before `assert_eq!` so the SET is locked without enforcing declaration order in the source file. Faster than spawning `cargo tree` + cargo-version-robust.
- **Test 2 is the formal Phase 25 closure sentinel for SC1:** The Phase 22-02 loader (`load_mai_hoa_golden`) already validates ≥10 cases + ≥2 sources per case + ≥1 KnownDivergence at load time. Test 2 re-asserts the same invariants EXPLICITLY at the INT-13 level with INT-13-specific panic messages, so a future regression points back to this milestone closure even if the loader's own assertions are weakened. Mirrors the "defense in depth" discipline from Plan 22-02's `golden_known_divergences_are_logged_not_corrected`.
- **Function-body `use` blocks for Task 1:** The plan said "use `use` inside the function body to avoid touching the file's existing module-level imports" — this preserves the existing INT-06 + INT-10 module-level imports (lines 1-26 of integration_2026_smoke.rs) so the new test is purely additive at the import level too.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Corrected `build_day_snapshot_graph` import path**
- **Found during:** Task 1 (compilation first attempt)
- **Issue:** The plan's `<interfaces>` block listed `build_day_snapshot_graph` under "From `crates/amlich-core/src/lib.rs`" implying it's at the crate root (`amlich_core::build_day_snapshot_graph`). In reality, `build_day_snapshot_graph` is re-exported at `amlich_core::semantic_graph::build_day_snapshot_graph` (line 11 of `semantic_graph/mod.rs`), NOT at the crate root. `lib.rs` lines 84-90 re-export some semantic_graph items at the crate root (`EdgeConcept`, `NodeConcept`, `SemanticGraph`, etc.) but NOT `build_day_snapshot_graph`.
- **Fix:** Changed the function-body `use amlich_core::{build_day_snapshot_graph, ...}` to `use amlich_core::semantic_graph::build_day_snapshot_graph;` (separate `use` statement). Kept the rest of the imports unchanged from the plan. Existing module-level imports at lines 24-26 of the file (which already import `calculate_day_snapshot`, `build_day_snapshot_graph`, `EdgeConcept, NodeConcept` correctly) remain untouched — function-body `use` shadows them locally without conflict.
- **Files modified:** `crates/amlich-core/tests/integration_2026_smoke.rs` (Task 1's `use` block)
- **Verification:** `cargo test --package amlich-core --test integration_2026_smoke e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates -- --exact --nocapture` → `test result: ok. 1 passed`.
- **Committed in:** `71d4c72` (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 blocking — wrong path in plan's interface spec).
**Impact on plan:** Trivial compile-time fix. No behavior change. No scope creep. All assertions, surface checks, and discipline constraints preserved verbatim from the plan.

## Issues Encountered

None — the plan was extremely prescriptive (detailed `<action>` blocks, named assertions, registered SOURCE_* const usage), and both tasks compiled + passed on the first run after the Rule 3 path correction.

## User Setup Required

None — no external service configuration required. Phase 25 is pure validation (test files only).

## Cargo Tree Output Snapshot (SC4 manual verification)

```
amlich-core v0.1.4 (/home/noy/work/amlich/crates/amlich-core)
├── chrono v0.4.44
├── serde v1.0.228
├── serde_json v1.0.149
└── unicode-normalization v0.1.25
```

Exactly 4 production deps. Zero new deps in v1.7 (despite shipping 448-text-field IChing corpus + Mai Hoa casting + biến quẻ + Thể/Dụng + IChingEvaluator + DirectionCrossLink + semantic-graph wiring + additive DaySnapshot fields + v1.6→v1.7 round-trip across Phases 20-24).

## INT-13 Four-SC Closure Cross-References

| SC | Description | Verified by | Status |
|----|-------------|-------------|--------|
| SC1 | ≥10 IChing golden cases cross-checked against ≥2 independent sources; divergences logged as `KnownDivergence` not silently corrected | `int13_golden_dataset_cross_source_discipline_holds` in `v17_baseline_guards.rs` (Test 2) | ✅ Met |
| SC2 | 2026 E2E smoke extending `integration_2026_smoke.rs` exercising IChing casting + biến quẻ + Thái Tuế cross-link + semantic-graph wiring + DaySnapshot fields together | `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates` in `integration_2026_smoke.rs` (Task 1) | ✅ Met |
| SC3 | Full crate test suite green with zero regressions on v1.6 baseline | `cargo test --package amlich-core` → 1120 passed, 0 failed, 7 ignored (was 1117 at Phase 24-end baseline) | ✅ Met |
| SC4 | Cargo deps unchanged (exactly 4: chrono + serde + serde_json + unicode-normalization) | `cargo_dependency_tree_unchanged_from_v16` in `v17_baseline_guards.rs` (Test 1) + manual `cargo tree -p amlich-core --depth 1` verification | ✅ Met |

## Exact Crate Test Count at Plan Completion

**1120 tests passed; 0 failed; 7 ignored.**

Breakdown of the +3 delta vs Phase 24-end baseline (1117):
- +1 from Task 1: `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates` (in `integration_2026_smoke.rs` — file's test count grew 4→5)
- +2 from Task 2: `cargo_dependency_tree_unchanged_from_v16` + `int13_golden_dataset_cross_source_discipline_holds` (new file `v17_baseline_guards.rs`)

7 ignored tests are pre-existing doc-tests in `almanac::na_am` + `almanac::sexagenary_cycle` (marked `ignore` in source; unchanged across Phase 24 → Phase 25).

## Next Phase Readiness

**Phase 25 complete (1/1 plans). INT-13 closed. v1.7 milestone ready for `/gsd-complete-milestone`.**

- All 6 phases of v1.7 milestone (Phases 20-25) complete; all 15/15 requirements closed (FND-09..12 + ICH-01..05 + XLK-01..03 + INT-11..13).
- No new production code in Phase 25 → no surface area to integrate. v1.8+ work unblocks with the locked v1.6 baseline shape + INT-13 cross-source discipline enforced as runtime invariants.
- The new baseline guards (`cargo_dependency_tree_unchanged_from_v16` + `int13_golden_dataset_cross_source_discipline_holds`) will trip CI if v1.8+ inadvertently adds a new production dep OR weakens the INT-13 golden discipline — defense in depth for the locked v1.7 contracts.

---
*Phase: 25-e2e-validation-golden-cross-source-verification*
*Completed: 2026-07-19*

## Self-Check: PASSED

- FOUND: `crates/amlich-core/tests/v17_baseline_guards.rs` (new file, 236 lines)
- FOUND: `crates/amlich-core/tests/integration_2026_smoke.rs` (modified, +416 lines, total 886)
- FOUND: `.planning/phases/25-e2e-validation-golden-cross-source-verification/25-01-SUMMARY.md`
- FOUND commit: `71d4c72` (Task 1 — test: add v1.7 E2E smoke for IChing + cross-link unified wiring)
- FOUND commit: `4921a1c` (Task 2 — test: add v17_baseline_guards SC4 dep tree + SC1 golden dataset)
- Full crate test suite: 1120 passed, 0 failed (verified after both task commits landed).
- `cargo tree -p amlich-core --depth 1`: exactly 4 deps (chrono + serde + serde_json + unicode-normalization).
