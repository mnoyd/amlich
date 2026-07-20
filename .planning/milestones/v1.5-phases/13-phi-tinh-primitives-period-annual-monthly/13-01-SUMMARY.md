---
phase: 13-phi-tinh-primitives-period-annual-monthly
plan: 01
subsystem: almanac
tags: [fengshui, flying-stars, phi-tinh, lo-shu, van, tiet-khi, ocalock, include_str, serde_json]

# Dependency graph
requires:
  - phase: 10-foundation
    provides: frozen FlyingStar/Palace/FlyingStarLayout types in fengshui/types.rs, SOURCE_HUYEN_KHONG constant
  - phase: existing
    provides: get_all_tiet_khi_for_year, jd_from_date/jd_to_date in tietkhi.rs/julian.rs

provides:
  - TietKhiScanner injection wrapper (scanner.rs) — stable &TietKhiScanner signature for all Phi Tinh plans
  - FlyingStar metadata loader via OnceLock + include_str! (stars.rs) — element/polarity/auspice for stars 1-9
  - flying_star_from_u8() u8->FlyingStar converter used by all downstream table loaders
  - compute_period() using Lập Xuân boundary scan (CRIT-2 fix — never year>=2024)
  - Period struct with base_layout() building FlyingStarLayout with Van evidence
  - load_flying_stars_base() OnceLock loader with Lo Shu invariant validation at load time
  - base_palaces_for_van() [FlyingStar; 9] lookup
  - flying_stars.json 9 star metadata rows
  - flying_stars_base.json 9 Vận palace tables (Vận 1-9, each verified sum=45)

affects:
  - 13-02 (annual Niên Tử Bạch — imports TietKhiScanner, compute_period)
  - 13-03 (monthly Nguyệt Tử Bạch — imports TietKhiScanner, compute_period)
  - 13-04 (golden/integration tests — verifies compute_period at 2024 boundary)
  - 14-01 (81-cell aspects — imports FlyingStarLayout from base_layout)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - OnceLock + include_str! JSON loader pattern with load-time panic validation (from golden_loader.rs)
    - fengshui/../../../data/almanac/ relative path depth (fengshui/ is 3 dirs from crate root data/)
    - validate_van_table() Lo Shu enforcer — sum=45, each 1-9 once, center=van
    - SOURCE_HUYEN_KHONG constant in ReasoningEvidenceEnvelope (source_id_guard.rs CI compliance)
    - compute_period() CRIT-2 pattern: jd_to_date -> lap_xuan_jd -> compare -> effective_year

key-files:
  created:
    - crates/amlich-core/src/almanac/fengshui/scanner.rs
    - crates/amlich-core/src/almanac/fengshui/stars.rs
    - crates/amlich-core/src/almanac/fengshui/period.rs
    - crates/amlich-core/data/almanac/flying_stars.json
    - crates/amlich-core/data/almanac/flying_stars_base.json
  modified:
    - crates/amlich-core/src/almanac/fengshui/mod.rs

key-decisions:
  - "Lập Xuân boundary via scanner.lap_xuan_jd(year): jd < lap_xuan -> effective_year = year-1; else year. CRIT-2 fix."
  - "Lo Shu invariants enforced at load time via validate_van_table() panic — sum=45, each 1-9, center=van (CRIT-4)."
  - "van_for_solar_year_after_lap_xuan: formula ((y-1864)/20)+1 clamped 1..=9; Vận 8=2004-2023, Vận 9=2024-2043."
  - "SOURCE_HUYEN_KHONG used for all evidence envelopes — no bare huyen-khong literals (source_id_guard.rs)."
  - "Base palace table computed: Lo Shu thuận path Center->NW->W->NE->S->N->SW->E->SE with star N at center."

patterns-established:
  - "fengshui/ include_str! path depth: ../../../data/almanac/ (3 up from fengshui/)"
  - "validate_van_table() panic pattern for any loader that needs Lo Shu correctness"
  - "TietKhiScanner::new() default ICT +7.0; all downstream FS functions accept &TietKhiScanner"

requirements-completed: [FS-01, FS-02, FS-03, FS-04, FS-05]

# Metrics
duration: 4min
completed: 2026-05-27
---

# Phase 13 Plan 01: Phi Tinh Primitives Summary

**TietKhiScanner injection wrapper, FlyingStar metadata loader, Lo Shu-validated Vận base tables, and Lập Xuân-anchored compute_period — primitives foundation for all Phi Tinh algorithm plans**

## Performance

- **Duration:** 4 min
- **Started:** 2026-05-27T17:00:06Z
- **Completed:** 2026-05-27T17:04:12Z
- **Tasks:** 2
- **Files modified:** 6

## Accomplishments

- `TietKhiScanner` struct with `lap_xuan_jd(year)` — stable injection point ensuring all Phi Tinh functions resolve Vận boundaries via real Tiết Khí scan, never naive year arithmetic (CRIT-2)
- `star_metadata()` + `flying_star_from_u8()` backed by `flying_stars.json` (9 rows: element/polarity/auspice for classical Huyền Không metadata); OnceLock + include_str! with load-time validation
- `compute_period()` with Lập Xuân boundary test: Jan 15 2024 → Vận 8, Feb 5 2024 → Vận 9; Jan 2004 → Vận 7, Feb 10 2004 → Vận 8; all 26 fengshui tests green
- `validate_van_table()` enforces Lo Shu invariants (sum=45, each 1-9 once, center=van) at load time; negative test confirms corrupted row panics immediately
- 9 Vận palace tables in `flying_stars_base.json` computed from Lo Shu thuận path; all pass invariants

## Task Commits

1. **Task 1: TietKhiScanner wrapper + FlyingStar metadata loader** - `c0435f1` (feat)
2. **Task 2: compute_period + Period type + base palace table loader** - `1af761d` (feat)

## Files Created/Modified

- `crates/amlich-core/src/almanac/fengshui/scanner.rs` - TietKhiScanner with lap_xuan_jd(), terms_for_year()
- `crates/amlich-core/src/almanac/fengshui/stars.rs` - FlyingStar metadata loader, flying_star_from_u8()
- `crates/amlich-core/src/almanac/fengshui/period.rs` - compute_period, Period, load_flying_stars_base, validate_van_table
- `crates/amlich-core/data/almanac/flying_stars.json` - 9 star rows with element/polarity/auspice
- `crates/amlich-core/data/almanac/flying_stars_base.json` - 9 Vận palace tables (Lo Shu verified)
- `crates/amlich-core/src/almanac/fengshui/mod.rs` - registers scanner/stars/period; re-exports public API

## Decisions Made

- **Lập Xuân CRIT-2 fix**: `compute_period` derives calendar year from JD, gets `lap_xuan = scanner.lap_xuan_jd(year)`, then `effective_year = if jd < lap_xuan { year - 1 } else { year }`. This is the CRIT-2 fix — never `year >= 2024`.
- **Lo Shu invariants at load time**: `validate_van_table(van, palaces)` panics on sum≠45, duplicates, or center≠van. This catches JSON typos at startup/test time, not silently at runtime (CRIT-4).
- **van_for_solar_year formula**: `((y - 1864) / 20) + 1` clamped 1..=9. Vận 7=1984-2003, Vận 8=2004-2023, Vận 9=2024-2043.
- **Base palace table thuận path**: Lo Shu flying sequence Center→NW→W→NE→S→N→SW→E→SE. For Vận N: center=N, then N+1,N+2... ascending (wrap 9→1) following the path. In Palace::ALL index order: palaces[4]=N, palaces[5]=N+1, palaces[6]=N+2, palaces[7]=N+3, palaces[8]=N+4, palaces[0]=N+5, palaces[1]=N+6, palaces[2]=N+7, palaces[3]=N+8.
- **SOURCE_HUYEN_KHONG in all evidence**: `van_evidence(van)` uses `SOURCE_HUYEN_KHONG.to_string()` (never bare `"huyen-khong"`) — passes source_id_guard.rs CI.

## Deviations from Plan

None — plan executed exactly as written. Both tasks implemented with inline TDD tests that passed on first run. No architectural changes or auto-fixes required.

## Issues Encountered

None.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- `TietKhiScanner`, `star_metadata`, `flying_star_from_u8`, `compute_period`, `compute_period_for_year`, `Period`, `base_palaces_for_van`, `load_flying_stars_base` all exported from `fengshui/mod.rs`
- Plans 13-02 (annual Niên Tử Bạch) and 13-03 (monthly Nguyệt Tử Bạch) can now import these primitives
- Plan 13-04 (golden tests) can verify compute_period at 2024 and 2004 boundaries

---
*Phase: 13-phi-tinh-primitives-period-annual-monthly*
*Completed: 2026-05-27*
