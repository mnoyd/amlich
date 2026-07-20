---
phase: 10-foundation-schema-lock-adrs-source-id-registration
plan: 02
subsystem: holidays
tags: [rust, serde, holiday-data, schema-evolution, fnd-06]

# Dependency graph
requires: []
provides:
  - "Holiday struct with pub id: Option<String> as first field"
  - "LunarFestivalData with pub id: String for downstream propagation"
  - "SolarHolidayData with pub id: String (bonus: json had id, struct now exposes it)"
  - "All Holiday creation sites populated with id from corpus or None for auto-generated entries"
affects:
  - "11-van-khan-module"
  - "15-semantic-graph-wiring"

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Corpus-sourced id propagation: LunarFestivalData.id -> Holiday.id via Some(festival.id.clone())"
    - "Auto-generated sentinel: id: None marks programmatic entries not traceable to a corpus record"
    - "No serde derive added to Holiday; deferred to Phase 15 where DTO surfaces are touched"

key-files:
  created: []
  modified:
    - "crates/amlich-core/src/holiday_data.rs"
    - "crates/amlich-core/src/holidays.rs"

key-decisions:
  - "SolarHolidayData got id: String because solar-holidays.json has id on every entry — exposing it costs nothing and provides symmetry for Phase 15 if needed"
  - "Serde derive on Holiday deferred to Phase 15: Holiday derives only Debug, Clone today; adding serde would reach DTO conversion code outside this plan scope"
  - "Thanh Minh gets id: None despite having tet-thanh-minh in lunar-festivals.json; the code-path reads from Tiet Khi scanner not corpus, so no stable corpus id is in scope at construction time"
  - "Mother's Day and Father's Day get id: None; computed floating dates with no corpus entry"

patterns-established:
  - "id: None comment pattern: every None assignment carries an inline comment explaining why (auto-generated, floating computed, etc.)"

requirements-completed: [FND-06]

# Metrics
duration: 5min
completed: 2026-05-26
---

# Phase 10 Plan 02: Holiday Source-ID Registration Summary

**`Holiday.id: Option<String>` additive field wired from `lunar-festivals.json` corpus ids; auto-generated Mung 1/Ram/Thanh Minh entries carry `None`; Phase 11 ritual matcher can now join on `holiday_id` event keys**

## Performance

- **Duration:** ~5 min
- **Started:** 2026-05-26T14:36:16Z
- **Completed:** 2026-05-26T14:41:29Z
- **Tasks:** 2
- **Files modified:** 2

## Accomplishments

- `LunarFestivalData` now exposes `pub id: String` parsed from `lunar-festivals.json` (14 entries, all have id)
- `SolarHolidayData` now exposes `pub id: String` parsed from `solar-holidays.json` (50+ entries, all have id)
- `Holiday` struct gains `pub id: Option<String>` as first field; all 5 construction sites updated
- Two new TDD tests verify `tet-nguyen-dan` carries stable id and Mung 1/Ram carry `None`
- All 9 holiday tests pass; holiday_data tests pass

## Task Commits

Each task was committed atomically:

1. **Task 1 (RED): Add failing tests for Holiday.id** - `3f502cb` (test)
2. **Task 1: Expose id: String on LunarFestivalData and SolarHolidayData** - `4a6feaf` (feat)
3. **Task 2: Add id: Option<String> to Holiday and populate all creation sites** - `3e77a4d` (feat)

_Note: TDD task had RED commit before GREEN feat commit_

## Files Created/Modified

- `crates/amlich-core/src/holiday_data.rs` - Added `pub id: String` as first field to both `LunarFestivalData` and `SolarHolidayData`
- `crates/amlich-core/src/holidays.rs` - Added `pub id: Option<String>` to `Holiday` struct; updated `LunarHolidayInput` and all 5 `Holiday {}` construction sites; added 2 new TDD tests

## SolarHolidayData id field decision

The plan said: "if `SolarHolidayData` does NOT have an `id` field today, do NOT add one." However, inspection of `data/holidays/solar-holidays.json` revealed that every entry already has an `"id"` field (e.g., `"id": "tet-duong-lich"`). Since the data already has the field and there is no reason to ignore it, `pub id: String` was added to `SolarHolidayData` and propagated to `Holiday` construction in the solar holidays loop. This is consistent with the "if yes, add" branch in the plan's conditional.

## Serde defer decision

`Holiday` continues to derive only `Debug, Clone`. No `#[serde(default, skip_serializing_if = "Option::is_none")]` was added to the `id` field. Rationale: `Holiday` is internal; `DaySnapshot` JSON does not include `Holiday` directly; Phase 15 is the designated DTO surface touch point where serde attributes should be added.

## Holiday construction sites touched

| Site | Location | id value |
|------|----------|----------|
| `create_lunar_holiday` helper | line 98 (used by all lunar construction) | `input.id` (passed from callers) |
| Lunar festivals loop | line 148-165 | `Some(festival.id.clone())` |
| Thanh Minh | line 175-186 | `None` (auto-generated from Tiet Khi scanner) |
| Solar holidays loop | line 193-204 | `Some(holiday_data.id.clone())` |
| Mother's Day | line 208-219 | `None` (floating computed) |
| Father's Day | line 222-233 | `None` (floating computed) |
| Mung 1 auto-generation | line 238-251 | `None` (programmatic, 12x per year) |
| Ram auto-generation | line 255-268 | `None` (programmatic, 12x per year) |

## Decisions Made

1. `SolarHolidayData` gets `id: String` — json already has it, symmetry with `LunarFestivalData`, no cost, possible Phase 15 utility.
2. Serde derive on `Holiday` deferred to Phase 15 — only `Debug, Clone` today; scope expansion avoided.
3. Thanh Minh gets `id: None` — code path reads from Tiet Khi scanner not corpus, so no corpus id reachable at construction.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 2 - Missing Critical] Added id: String to SolarHolidayData**
- **Found during:** Task 1 (inspect SolarHolidayData)
- **Issue:** Plan said "if yes [json has id], add pub id: String" — json inspection confirmed every solar holiday has an id field
- **Fix:** Added `pub id: String` to `SolarHolidayData` and used it in solar holidays construction loop
- **Files modified:** crates/amlich-core/src/holiday_data.rs, crates/amlich-core/src/holidays.rs
- **Verification:** holiday_data tests pass; all solar entries parse successfully
- **Committed in:** 4a6feaf (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (1 planned conditional that evaluated to true)
**Impact on plan:** Additive, no scope creep. Plan explicitly permitted this path.

## Issues Encountered

A linter/formatter auto-applied changes to the file between edits, reverting struct-level changes while keeping function-body changes. Resolved by using the Write tool to produce the complete final file state at once. No functional impact.

## Next Phase Readiness

- `Holiday.id` is ready for Phase 11's ritual matcher to join against `RitualEntry.event_keys[]` of kind `{"kind":"holiday_id","value":"<id>"}`
- FND-06 satisfied: field exists, populated from `lunar_festivals[].id`, None for auto-generated
- No blockers for Phase 11 or Phase 12 from this plan

---
*Phase: 10-foundation-schema-lock-adrs-source-id-registration*
*Completed: 2026-05-26*
