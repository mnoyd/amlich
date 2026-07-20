---
phase: 13-phi-tinh-primitives-period-annual-monthly
plan: 02
subsystem: almanac
tags: [phi-tinh, flying-stars, fengshui, rust, lo-shu, nien-tu-bach, nguyet-tu-bach]

# Dependency graph
requires:
  - phase: 13-phi-tinh-primitives-period-annual-monthly
    plan: 01
    provides: TietKhiScanner, flying_star_from_u8, FlyingStarLayout/Period/FlyingStar types, fill infrastructure
provides:
  - compute_yearly_flying_stars: 9-palace annual grid, center star via nien_center, ADR-0003 polarity matrix
  - compute_monthly_flying_stars: 9-palace monthly grid, 8/5/2 branch-group rule, solar-term month index
  - fill_palaces(center, ascending) pub(crate): shared Lo Shu spiral fill (single implementation)
  - year_is_ascending(year) pub(crate): direction helper shared by annual + monthly
  - YearPolarity enum: Duong/Am from can_index parity
affects: [13-03-combined, 14-aspects, 15-integration]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "nien_center anchored to 2024→4 via rem_euclid offset formula"
    - "fill_palaces pub(crate) in annual.rs, imported by monthly.rs (single implementation)"
    - "FLYING_PATH constant: Center→NW→W→NE→S→N→SW→E→SE (Lo Shu thuận walk, indices 4,5,6,7,8,0,1,2,3)"
    - "year_is_ascending pub(crate) for direction sharing without duplication"
    - "Evidence note encodes year/center/polarity/confidence for traceability"

key-files:
  created:
    - crates/amlich-core/src/almanac/fengshui/annual.rs
    - crates/amlich-core/src/almanac/fengshui/monthly.rs
  modified:
    - crates/amlich-core/src/almanac/fengshui/mod.rs

key-decisions:
  - "fill_palaces lives in annual.rs as pub(crate), imported by monthly.rs — single Lo Shu spiral implementation shared by all time layers"
  - "year_is_ascending(year) pub(crate) in annual.rs eliminates polarity logic duplication in monthly.rs"
  - "nien_center formula: delta=year-2024; offset=delta.rem_euclid(9); raw=((4-1-offset).rem_euclid(9))+1 — produces 2024→4, 2025→3, 2023→5 correctly"
  - "YearPolarity enum (not a bool flag) per ADR-0003; direction is a derived property, never stored as is_retrograde"
  - "Pre-1984 years annotated confidence=medium in evidence note; post-1984 confidence=high"

patterns-established:
  - "pub(crate) helpers for shared spiral logic: annual.rs is the home of fill_palaces + FLYING_PATH + year_is_ascending"
  - "Evidence method naming: phi_tinh.nien for annual, phi_tinh.nguyet for monthly"
  - "Assert on month range at function entry: (1..=12).contains(&month)"

requirements-completed: [FS-06, FS-07, FS-09]

# Metrics
duration: 3min
completed: 2026-05-27
---

# Phase 13 Plan 02: Annual + Monthly Flying Stars Summary

**Annual Niên Tử Bạch (descend-mod-9 anchored 2024→4) and monthly Nguyệt Tử Bạch (8/5/2 year-branch group + descend-mod-9) with shared Lo Shu spiral fill and per-layer evidence envelopes**

## Performance

- **Duration:** 3 min
- **Started:** 2026-05-27T17:06:48Z
- **Completed:** 2026-05-27T17:09:48Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments

- `annual.rs`: `YearPolarity` enum, `year_polarity()`, `nien_center()` anchored 2024→4, `fill_palaces()` shared spiral, `compute_yearly_flying_stars()` with ADR-0003 polarity matrix — 16 tests all green
- `monthly.rs`: `month_group()` maps 3 chi-branch families to groups 8/5/2, `monthly_center()` descends mod-9 from group leader, `compute_monthly_flying_stars()` reusing `fill_palaces` and `year_is_ascending` from annual.rs — 20 tests all green
- `mod.rs` re-exports both functions; `fill_palaces` and `year_is_ascending` are `pub(crate)` (single implementation, no copy-paste divergence); no bare `"huyen-khong"` strings; no `use crate::interaction`

## Task Commits

Each task was committed atomically:

1. **Task 1: compute_yearly_flying_stars + Niên polarity matrix** - `2a311d1` (feat)
2. **Task 2: compute_monthly_flying_stars + solar-term month resolver + 8/5/2 group rule** - `b79c230` (feat)

**Plan metadata:** (docs commit follows)

## Files Created/Modified

- `crates/amlich-core/src/almanac/fengshui/annual.rs` - YearPolarity, year_polarity, nien_center, fill_palaces, FLYING_PATH, year_is_ascending, compute_yearly_flying_stars + 16 tests
- `crates/amlich-core/src/almanac/fengshui/monthly.rs` - month_group, monthly_center, compute_monthly_flying_stars + 20 tests
- `crates/amlich-core/src/almanac/fengshui/mod.rs` - pub mod annual/monthly; pub use both functions + YearPolarity

## Decisions Made

- `fill_palaces` placed in `annual.rs` as `pub(crate)` and imported by `monthly.rs` — single Lo Shu spiral implementation; no divergence risk between annual/monthly/van layers
- `year_is_ascending` extracted as `pub(crate)` helper to avoid duplicating polarity logic in monthly.rs
- `YearPolarity` is an enum (not a bool) per ADR-0003 — direction derived at call site, never stored as `is_retrograde: bool`
- `nien_center` formula uses `rem_euclid(9)` to handle both past and future years correctly from the 2024=4 anchor
- Pre-1984 years (Thượng/Trung Nguyên) annotated `confidence=medium` in evidence note per ADR-0003 §4

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- `compute_yearly_flying_stars` and `compute_monthly_flying_stars` are ready for 13-03 combined overlay
- `fill_palaces` shared infrastructure means 13-03 can combine annual + monthly layouts without re-implementing the spiral
- `TietKhiScanner` parameter is already part of both signatures; 13-03 will pass it for solar-term date resolution

---
*Phase: 13-phi-tinh-primitives-period-annual-monthly*
*Completed: 2026-05-27*
