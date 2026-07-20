---
phase: 23-th-i-tu-tam-s-t-phi-tinh-cross-link
plan: 02
subsystem: reasoning
tags: [serde, dto, additive-snapshot, contracts, crit3-isolation, rust]

# Dependency graph
requires:
  - phase: 19-offering-refs-and-semantic-graph-wiring
    provides: additive Option<T> serde default/skip-if-none discipline on DaySnapshot
  - phase: 20-foundation-schema-lock-source-ids-adrs-ontology
    provides: FND-12 ontology reservation for Hexagram/LocatedAt/Transforms + ReasoningNodeSeverity/ReasoningEvidenceEnvelope types
provides:
  - "Public serializable DirectionCrossLink / DirectionCrossLinkSummary contracts (DTO-only, no lifetimes)"
  - "Agreement enum with five variants (Agreement, BothSilent, KhcbpptOnly, HuyenKhongOnly, Conflict)"
  - "DirectionCell / DirectionalTaboo / DirectionalThaiTue / HuyenKhongCell owned field shapes"
  - "Locked DIRECTION_ORDER ([Direction; 8]) constant matching the existing interaction-layer directional convention"
  - "DATE_ONLY_BIRTH_CHI_INDEX (usize::MAX) sentinel for the date-only variant"
  - "COMPOSITE_DIRECTION_CROSS_LINK named const for the composite rule identifier"
  - "FlyingStarsSummary.palace_safety_hints additive transport (Option<[Option<String>; 9]>)"
  - "DaySnapshot.direction_cross_link additive field (Option<DirectionCrossLinkSummary>), default None"
affects:
  - 23-03-direction-cross-link-implementation
  - 24-iching-evaluator-semantic-graph-wiring-dto-integration

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Additive Option<T> serde default/skip-if-none DTO discipline (extends Phase 18/19 pattern)"
    - "std::array::from_fn for initializing non-Copy fixed-size arrays (8 DirectionCell slots)"
    - "Sentinel-value pattern (usize::MAX) for variant discrimination without a wrapper type"
    - "Named-const composite rule identifier (single audit point for a rule.composite.* source_id)"
    - "DTO-boundary pre-baking: lower-layer data is lifted to plain owned DTO fields so the upper layer consumes only the snapshot (CRIT-3-safe)"

key-files:
  created:
    - crates/amlich-core/src/reasoning/direction_composite.rs
  modified:
    - crates/amlich-core/src/reasoning/mod.rs
    - crates/amlich-core/src/lib.rs

key-decisions:
  - "birth_chi_index kept as usize (not u8 or Option<usize>) on both DirectionCrossLink and DirectionCrossLinkSummary; DATE_ONLY_BIRTH_CHI_INDEX == usize::MAX is the documented sentinel — no wrapper type minted"
  - "HuyenKhongCell.annual_star/monthly_star stored as DTO projections (u8), NOT lower-level palace-layout types — preserves CRIT-3 isolation at the cross-link layer"
  - "palace_safety_hints pre-baked into FlyingStarsSummary at calculate_day_snapshot_internal time via element_hint_for_palace + std::array::from_fn — the cross-link consumes only snapshot.flying_stars"
  - "direction_cross_link initialized to None in the DaySnapshot constructor; no calculation path auto-populates it — explicit enrichment helper deferred to Plan 23-03"
  - "Selective pub use re-export from reasoning/mod.rs exposes the contract types/constants only; build_* function re-exports land with Plan 23-03"
  - "Rule 1 deviation: scrubbed the test placeholder cross_link_kind string to avoid the future CRIT-3 substring scan tripping on phi_tinh inside a longer identifier"

patterns-established:
  - "Sentinel-value + doc-comment for variant discrimination on a primitive field (usize::MAX) instead of an Option or newtype"
  - "DTO-boundary pre-baking: lower-layer safety text lifted into an additive Option<[Option<String>; N]> field so upper layers stay pure-DTO consumers"
  - "Contract module that declares only types/constants + an inline contract test, with function signatures reserved as API-comments for the implementation plan"

requirements-completed: [XLK-03]

# Metrics
duration: 12min
completed: 2026-07-16
---

# Phase 23 Plan 02: Thái Tuế / Tam Sát ⇄ Phi Tinh Cross-Link Contracts Summary

**Public serializable cross-link DTO contracts (DirectionCrossLink/Summary + 8 rich cells + Agreement enum + usize::MAX date-only sentinel) plus the additive palace_safety_hints transport and the default-None DaySnapshot.direction_cross_link field, all CRIT-3-safe at the reasoning boundary**

## Performance

- **Duration:** 12 min
- **Started:** 2026-07-16T15:16:48Z
- **Completed:** 2026-07-16T15:28:33Z
- **Tasks:** 2 (both TDD: RED → GREEN, no REFACTOR needed)
- **Files modified:** 3 (1 new, 2 existing)

## Accomplishments

- Locked the eight-element `DIRECTION_ORDER` constant in the exact existing interaction-layer order (North, Northeast, East, Southeast, South, Southwest, West, Northwest) so the cross-link cell indexing is stable from day one.
- Authored the full public contract surface (`DirectionCrossLink`, `DirectionCrossLinkSummary`, `DirectionCell`, `DirectionalTaboo`, `DirectionalThaiTue`, `HuyenKhongCell`, `Agreement`) as owned, serde-compatible types — no lifetimes, no graph edges, no new directional enum.
- Held `birth_chi_index: usize` on both rich and summary forms with `DATE_ONLY_BIRTH_CHI_INDEX == usize::MAX` as the documented date-only sentinel — no wrapper type, one public type identity for the Phase 24 consumer.
- Added `FlyingStarsSummary.palace_safety_hints: Option<[Option<String>; 9]>` populated at the snapshot boundary via `element_hint_for_palace + std::array::from_fn`, so the reasoning cross-link consumes only `snapshot.flying_stars` (CRIT-3-safe transport).
- Added `DaySnapshot.direction_cross_link: Option<crate::reasoning::DirectionCrossLinkSummary>` initialized to `None` in the constructor — no auto-population, absent from JSON when None, v1.6 → v1.7 round-trip stays byte-equal.
- Verified the parallel-execution discipline: the contract module contains zero of the future CRIT-3 forbidden substrings (`almanac::fengshui`, `phi_tinh`, `compute_*`, `TietKhiScanner`, `FlyingStarPeriod`).

## Task Commits

Each task was committed atomically (TDD RED → GREEN per task):

1. **Task 1 RED — direction cross-link contract test** — `401b248` (test)
2. **Task 1 GREEN — implement direction cross-link contracts** — `9ff695f` (feat)
3. **Task 2 RED — additive snapshot transport tests** — `6f0d73d` (test)
4. **Task 2 GREEN — palace safety-hint transport + direction_cross_link field** — `bf680e0` (feat)

**Plan metadata:** _pending final docs commit (SUMMARY + STATE + ROADMAP)._

## Files Created/Modified

- `crates/amlich-core/src/reasoning/direction_composite.rs` — NEW (~240 lines). All public contract types, the three named constants (`COMPOSITE_DIRECTION_CROSS_LINK`, `DATE_ONLY_BIRTH_CHI_INDEX`, `DIRECTION_ORDER`), and six inline contract tests. Function signatures reserved as API-comments for Plan 23-03.
- `crates/amlich-core/src/reasoning/mod.rs` — MODIFIED. Added `mod direction_composite;` plus a selective `pub use direction_composite::{Agreement, DirectionCell, DirectionCrossLink, DirectionCrossLinkSummary, DirectionalTaboo, DirectionalThaiTue, HuyenKhongCell, COMPOSITE_DIRECTION_CROSS_LINK, DATE_ONLY_BIRTH_CHI_INDEX, DIRECTION_ORDER}` re-export.
- `crates/amlich-core/src/lib.rs` — MODIFIED. Extended `FlyingStarsSummary` with the additive `palace_safety_hints` field + pre-baking logic in `calculate_day_snapshot_internal`; extended `DaySnapshot` with the additive `direction_cross_link` field initialized to `None`; three new inline tests for the additive transport.

## Decisions Made

- **`birth_chi_index: usize` (not `u8` / `Option<usize>`).** Keeps the date-only sentinel (`usize::MAX`) representable on the same field without a wrapper type. Mirrors the CONTEXT.md Claude's Discretion recommendation. Documented on both structs.
- **Star numbers as `u8` DTO projections.** `HuyenKhongCell.annual_star`/`monthly_star` hold the raw star number (1–9), NOT the lower-level `FlyingStar` enum or any palace-layout type. The reasoning layer never imports lower-level types.
- **Safety hint pre-baked at the snapshot boundary.** `element_hint_for_palace(overlay.palace_overlays[i].0).map(|h| h.hint_text_vi.clone())` runs inside `calculate_day_snapshot_internal`'s existing `flying_stars` block (where `almanac::fengshui::*` imports are already permitted), storing plain Vietnamese `String` values on the DTO. The cross-link consumes only the snapshot field.
- **No auto-population of `direction_cross_link`.** The constructor sets it to `None`; the explicit immutable enrichment helper is Plan 23-03's responsibility. v1.6 → v1.7 round-trip stays clean because the field is absent from JSON when None.
- **Selective `pub use` re-export.** Only the contract types/constants are re-exported from `reasoning/mod.rs`. The `build_*` function re-exports land with Plan 23-03 (the function bodies do not exist yet).
- **Reserved function signatures as API-comments.** `build_direction_cross_link_personal`, `build_direction_cross_link_date`, and `project_to_summary` are mentioned in the module doc-comment as the implementation plan's deliverables — no stub `todo!()` bodies, no graph edges, no temporary scaffolding.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Scrubbed `phi_tinh` substring from test placeholder**
- **Found during:** Task 1 GREEN verification
- **Issue:** The contract test's placeholder `cross_link_kind` value was `"thai_tue_x_tam_sat_x_phi_tinh"`, which contains the substring `phi_tinh`. Plan 23-03's sibling CRIT-3 grep guard would false-positive on this substring (the guard does a naive `contents.contains(forbidden)` scan over the whole `direction_composite.rs` file).
- **Fix:** Replaced the placeholder with `"composite_kind_contract_probe"` (a pure contract-test identifier with no semantic clash). The real `cross_link_kind` value is Plan 23-03's concern and will be authored to satisfy the same CRIT-3 constraint.
- **Files modified:** `crates/amlich-core/src/reasoning/direction_composite.rs`
- **Verification:** `rg "phi_tinh|almanac::fengshui|compute_daily_flying_stars|compute_combined_overlay|compute_palace_aspects|TietKhiScanner|FlyingStarPeriod" crates/amlich-core/src/reasoning/direction_composite.rs` returns zero matches. All 6 contract tests still pass.
- **Committed in:** `9ff695f` (Task 1 GREEN commit)

---

**Total deviations:** 1 auto-fixed (1 bug / future-RED-prevention).
**Impact on plan:** The fix keeps the contract module clean against the future CRIT-3 scan that Plan 23-03 will install. No scope creep; no behavior change.

## Issues Encountered

- **Parallel-execution build breakage from sibling Plan 23-01.** While my Task 2 RED was on disk, the parallel 23-01 executor had declared `mod tam_sat;` in `almanac/mod.rs` without yet creating the file (E0583). This momentarily broke the lib-test build with an error outside my territory. The breakage self-resolved when 23-01's GREEN commit landed the `tam_sat.rs` file. Per the scope-boundary rule, I did NOT touch any `almanac/*` file — out of my plan's ownership. My own files compiled and tested cleanly throughout.
- **`cargo test -p amlich-core` (full suite) currently has failures in `tests/tam_sat_integration.rs`.** Those are 23-01's in-flight integration tests (9 of 10 failing — 23-01's GREEN implementation hasn't shipped yet at the time my plan completed). My own territory (`reasoning::*`, `lib.rs day_snapshot*`, `tests/day_snapshot_v14_compat.rs`, `tests/source_id_guard.rs`) is fully green (769 lib tests + 9 v14-compat tests + 1 source_id_guard test). This is the expected parallel-wave state; the failures will clear once 23-01 lands its GREEN commit.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- **Ready for Plan 23-03 (implementation).** All public contract types, constants, the DTO transport, and the additive snapshot field are in place. Plan 23-03 authors `build_direction_cross_link_personal`, `build_direction_cross_link_date`, `project_to_summary`, the immutable `enrich_day_snapshot_with_direction_cross_link` helper, the sibling CRIT-3 grep guard at `tests/thai_tue_cross_link_crit3.rs`, and any `PersonalReasoningInput::build_fact_nodes` integration.
- **Ready for Phase 24 (IChing Evaluator + Semantic-Graph Wiring).** The `DirectionCrossLinkSummary` type identity is locked; Phase 24's graph consumer imports `crate::reasoning::DirectionCrossLinkSummary` directly without a placeholder.
- **One follow-up flagged for Plan 23-03:** the eventual `cross_link_kind` value in the real builder must also avoid the CRIT-3 forbidden substrings (not just my test placeholder). The DTO is neutral; the implementation author needs to pick a value that survives the grep guard.

---
*Phase: 23-th-i-tu-tam-s-t-phi-tinh-cross-link*
*Completed: 2026-07-16*

## Self-Check: PASSED

- All declared `key-files.created` / `key-files.modified` exist on disk.
- All four task commit hashes (`401b248`, `9ff695f`, `6f0d73d`, `bf680e0`) are present in `git log`.
- All plan-level verification gates in this plan's territory pass: `cargo test -p amlich-core --lib reasoning::direction_composite` (6/6), `cargo test -p amlich-core --lib day_snapshot*` (23/23), `cargo test -p amlich-core --test day_snapshot_v14_compat` (9/9), `cargo test -p amlich-core --test source_id_guard` (1/1), `cargo build -p amlich-core` clean.
- The full `cargo test -p amlich-core` suite currently shows failures only in `tests/tam_sat_integration.rs`, which belongs to the parallel Plan 23-01 (in-flight at time of completion) and is outside this plan's file ownership.
