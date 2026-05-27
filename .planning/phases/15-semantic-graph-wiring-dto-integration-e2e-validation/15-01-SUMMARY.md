---
phase: 15-semantic-graph-wiring-dto-integration-e2e-validation
plan: 01
subsystem: api
tags: [serde, dto, daysnapshot, fengshui, rituals, rust]

# Dependency graph
requires:
  - phase: 14-phi-tinh-81-cell-aspects-safety-hints
    provides: compute_combined_overlay, CombinedFlyingStarLayout, TietKhiScanner
  - phase: 11-van-khan-module-lookup-apis
    provides: find_van_khan_for_snapshot, RitualEntry with ritual_id
provides:
  - Serialize/Deserialize on full DaySnapshot type chain (SolarDate, CanChiSet, DayContext, DaySnapshot, CanChi, NguHanh, LunarDate, SolarTerm, StarType, HourInfo, GioHoangDao)
  - FlyingStarsSummary DTO slim type
  - flying_stars: Option<FlyingStarsSummary> on DaySnapshot (additive, skip_serializing_if)
  - applicable_rituals: Option<Vec<String>> on DaySnapshot (additive, skip_serializing_if)
  - Both fields populated by default in calculate_day_snapshot_internal
affects: [15-02, 15-03, 15-04, int-01, int-02, int-05]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "serde derive sweep: add Serialize+Deserialize to type chain bottom-up (leaf types first)"
    - "additive DTO fields: Option<T> + #[serde(default, skip_serializing_if = Option::is_none)] for backward-compat"
    - "build-then-mutate snapshot pattern for two-pass population (flying_stars then applicable_rituals)"

key-files:
  created: []
  modified:
    - crates/amlich-core/src/lib.rs
    - crates/amlich-core/src/types.rs
    - crates/amlich-core/src/lunar.rs
    - crates/amlich-core/src/tietkhi.rs
    - crates/amlich-core/src/gio_hoang_dao.rs

key-decisions:
  - "FlyingStarsSummary DTO uses crate::almanac::fengshui::types::FlyingStar path — FlyingStar is not re-exported from fengshui mod.rs directly, must use types sub-module path"
  - "center_star taken from annual_layout.center_star (Nien layer), not van_layout — annual layer is the year-specific Nien Phi Tinh, most relevant for a day snapshot"
  - "van extracted from van_layout.period via FlyingStarPeriod::Van { van } destructure; fallback to 1 (unreachable in practice)"
  - "Build-then-mutate pattern: DaySnapshot constructed with None fields first, then applicable_rituals populated by passing snap to find_van_khan_for_snapshot, avoiding circular dependency"
  - "No deny_unknown_fields added anywhere — backward compat (INT-05) requires lenient deserialization"

patterns-established:
  - "Additive DTO fields: use Option<T> + skip_serializing_if to preserve JSON backward compat"
  - "serde::Serialize, serde::Deserialize in derive without use import (path-qualified) works for leaf modules that don't want to pollute namespace"

requirements-completed: [INT-01, INT-02]

# Metrics
duration: 5min
completed: 2026-05-28
---

# Phase 15 Plan 01: Serde Foundation + Additive DaySnapshot Fields Summary

**Full serde derive sweep on DaySnapshot type chain plus FlyingStarsSummary DTO and two backward-compatible additive fields (flying_stars, applicable_rituals) populated by default in calculate_day_snapshot_internal**

## Performance

- **Duration:** 5 min
- **Started:** 2026-05-27T19:03:05Z
- **Completed:** 2026-05-28T19:08:00Z
- **Tasks:** 2
- **Files modified:** 5

## Accomplishments

- Added Serialize+Deserialize to all 9 types in the DaySnapshot chain (SolarDate, CanChiSet, DayContext, DaySnapshot in lib.rs; NguHanh, CanChi in types.rs; LunarDate in lunar.rs; SolarTerm in tietkhi.rs; StarType, HourInfo, GioHoangDao in gio_hoang_dao.rs)
- Defined FlyingStarsSummary DTO (van, year, month, center_star, palace_overlays) as slim serializable scalar type
- Added flying_stars and applicable_rituals as additive optional fields with skip_serializing_if — absent from JSON when None, no backward-compat break
- Both fields populated by default in calculate_day_snapshot_internal via compute_combined_overlay and find_van_khan_for_snapshot
- All 694 lib tests pass; new day_snapshot_serde_round_trip and day_snapshot_populates_additive_surfaces tests green

## Task Commits

Each task was committed atomically:

1. **Task 1 RED: Serde round-trip failing test** - `8d904a3` (test)
2. **Task 1 GREEN: Serde-derive sweep on type chain** - `5f960b1` (feat)
3. **Task 2 RED: Additive fields failing test** - `3c75c16` (test)
4. **Task 2 GREEN: FlyingStarsSummary DTO + field population** - `9d8f0ef` (feat)

_Note: TDD tasks have separate RED and GREEN commits._

## Files Created/Modified

- `crates/amlich-core/src/lib.rs` - Added serde import + derives on SolarDate/CanChiSet/DayContext/DaySnapshot; defined FlyingStarsSummary DTO; added flying_stars and applicable_rituals fields; updated calculate_day_snapshot_internal to populate both; added two new tests
- `crates/amlich-core/src/types.rs` - Added serde import + Serialize/Deserialize to NguHanh and CanChi
- `crates/amlich-core/src/lunar.rs` - Added serde::Serialize/serde::Deserialize to LunarDate
- `crates/amlich-core/src/tietkhi.rs` - Added serde::Serialize/serde::Deserialize to SolarTerm
- `crates/amlich-core/src/gio_hoang_dao.rs` - Added serde::Serialize/serde::Deserialize to StarType, HourInfo, GioHoangDao

## Decisions Made

- **FlyingStarsSummary uses crate::almanac::fengshui::types::FlyingStar** — FlyingStar is not re-exported from fengshui/mod.rs; must use the types sub-module path directly
- **center_star from annual_layout** — the Nien (annual) layer's center star is most meaningful for a day snapshot; van_layout center_star is the base period number
- **van extracted via FlyingStarPeriod::Van destructure** with fallback 1 (unreachable since compute_period_for_year always returns a Van period for a given year)
- **Build-then-mutate pattern** — snapshot constructed first with None fields; then applicable_rituals populated by passing `&snap` to find_van_khan_for_snapshot avoiding circular borrows
- **No deny_unknown_fields** — kept lenient per INT-05 requirement for additive forward compat

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- FlyingStar path resolution: the plan suggested `crate::almanac::fengshui::FlyingStar` but that path is not re-exported in fengshui/mod.rs; corrected to `crate::almanac::fengshui::types::FlyingStar`. Build error caught immediately.

## Next Phase Readiness

- DaySnapshot is now fully serde-serializable — unblocks INT-01, INT-02, INT-05 DTO integration tests
- FlyingStarsSummary DTO is public and available for E2E smoke tests (INT-06)
- applicable_rituals Vec<String> provides the slim ritual surface for JSON consumers
- Phases 15-02 through 15-04 can proceed on this foundation

## Self-Check: PASSED

All created files and commits verified present.

---
*Phase: 15-semantic-graph-wiring-dto-integration-e2e-validation*
*Completed: 2026-05-28*
