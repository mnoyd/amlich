---
phase: 18-daily-phi-tinh
plan: 02
subsystem: fengshui
tags: [phi-tinh, daily, lua-nhat, adr-0004, tiet-khi-scanner, giap-ty-seed, pitfall-guards]

# Dependency graph
requires:
  - phase: 18-daily-phi-tinh/18-01
    provides: ADR-0004 daily starting-star convention + locked DailyFlyingStarLayout sibling + FlyingStarPeriod::Daily variant
  - phase: 13-foundation-phi-tinh
    provides: fill_palaces(center, ascending) helper (annual.rs), TietKhiScanner v1.1.2 boundary resolver, flying_star_from_u8 + stars module
  - phase: 16-foundation-adr-0003-confidence-closure
    provides: KnownDivergence deferral discipline (carried into 18-03, not 18-02)
provides:
  - compute_daily_flying_stars(date, scanner) -> DailyFlyingStarLayout public API on the fengshui module re-export surface
  - 6 Trung Khí pivot classification (PivotKind + pivot_kind + pivot_starting_star helpers) with Duong/Am polarity split matching ADR-0004 §2
  - daily_pivots_for_year helper returning a JD-sorted pivot list spanning [year-1, year, year+1] (broader than plan's [year, year+1]) for robust early-January / late-December pivot lookup
  - Giáp-Tý-as-seed-day mechanic with explicit prior-pivot fallback (Pitfall P-7 guard) — pre-Giap-Ty-in-new-Tiet-Khi dates resolve to the PRIOR pivot
  - Evidence envelope carrying method=phi_tinh.nhat, source_id=SOURCE_HUYEN_KHONG (the const import, NOT a bare string), and an audit-replay note containing pivot name + seed + n + center + direction
  - 11 unit tests in daily::tests covering P-3/P-4/P-6/P-7/P-8 pitfalls
affects:
  - phase: 18-daily-phi-tinh/18-03 (daily golden dataset consumes compute_daily_flying_stars as the algorithm ground truth)
  - phase: 18-daily-phi-tinh/18-04 (DaySnapshot.daily_flying_stars additive field populated by compute_daily_flying_stars)
  - phase: 19-recommends-offering (2026 E2E smoke exercises the daily layer)

# Tech tracking
tech-stack:
  added: []  # No new dependencies; pure Rust algorithm reusing v1.5 scanner + annual::fill_palaces
  patterns:
    - public-API algorithm + private helpers pattern mirroring monthly.rs (compute_*_flying_stars + helpers + #[cfg(test)] mod tests)
    - scanner-driven pivot lookup with [year-1, year, year+1] bracket for boundary robustness (broadens the plan's [year, year+1] to cover late-December lookup)
    - explicit prior-pivot fallback when target_jd precedes the first Giáp Tý in the new Tiết Khí (Pitfall P-7 mechanic, deterministic)
    - Unicode NFC/NFD acceptance for "Vũ Thuỷ" vs "Vũ Thủy" (matcher accepts both forms, mirroring the v1.5 source-corpus normalization discipline)
    - shared fill_palaces reuse via pub(crate) import — no duplicate 9-palace walking logic, no silent Lo Shu drift

key-files:
  created:
    - crates/amlich-core/src/almanac/fengshui/daily.rs
  modified:
    - crates/amlich-core/src/almanac/fengshui/mod.rs

key-decisions:
  - "Scanner year bracket widened from [year, year+1] (plan) to [year-1, year, year+1] (committed) — needed for late-December target dates that must walk back past the prior year's Sương Giáng pivot without requiring the caller to retry with year-1"
  - "Unicode NFC/NFD acceptance: pivot_kind and pivot_starting_star accept both 'Vũ Thuỷ' (NFD/legacy) and 'Vũ Thủy' (NFC/preferred) as the same pivot — mirrors the v1.5 source-corpus normalization discipline and prevents a spurious panic on un-pre-normalized input"
  - "Direction rule confirmed opposite of ADR-0003: Dương pivot → thuận (ascending, +1 per Giáp Tý cycle); Âm pivot → nghịch (descending, -1). The annual rule (dương year → nghịch) does NOT apply to the daily layer"
  - "Giáp-Tý seed mechanic with prior-pivot fallback: when target_jd precedes the first Giáp Tý in the new Tiết Khí, the algorithm falls back to the PRIOR pivot (pivot_jd's predecessor in the sorted list) and recomputes seed_jd from there — deterministic and traceable in the evidence note"
  - "Evidence envelope uses the SOURCE_HUYEN_KHONG const import (not a bare 'huyen-khong' string) — verified by the existing tests/source_id_guard.rs at runtime"

patterns-established:
  - "Pattern: scanner-bracket widening for boundary robustness — when an algorithm must walk pivots across a calendar boundary, include year-1/year/year+1 in the lookup rather than forcing the caller to retry"
  - "Pattern: explicit prior-pivot fallback when a seed-day mechanic (Giáp Tý) hasn't kicked in — fall back deterministically rather than synthesizing a wrong seed"
  - "Pattern: NFC/NFD acceptance in classical-name matchers — Vietnamese pivot names accept both uniface (Thuỷ) and composed (Thủy) forms to keep corpus normalization from being a runtime panic risk"

requirements-completed: [FS-16]

# Metrics
duration: 4min
completed: 2026-07-15
---

# Phase 18 Plan 02: Daily Phi Tinh Algorithm Summary

**compute_daily_flying_stars algorithm implemented in a new `daily.rs` module: 6 Trung Khí pivots resolved via v1.1.2 TietKhiScanner, Giáp-Tý-as-seed-day mechanic with prior-pivot fallback (Pitfall P-7), Dương→thuận / Âm→nghịch direction rule (opposite of ADR-0003), shared fill_palaces reuse, and 11 unit tests gating P-3/P-4/P-6/P-7/P-8 pitfalls (FS-16 closed).**

## Performance

- **Duration:** ~4 min (algorithm commit + test commit)
- **Started:** 2026-07-15T13:41:36Z (commit 27ce722)
- **Completed:** 2026-07-15T13:45:04Z (commit 083a483)
- **Tasks:** 2
- **Files modified:** 2 (daily.rs created + mod.rs re-export)

## Accomplishments

- **`compute_daily_flying_stars(date, scanner) -> DailyFlyingStarLayout`** implemented at `crates/amlich-core/src/almanac/fengshui/daily.rs` as the consumer-facing algorithm for FS-16. The 7-step pipeline: (1) resolve the pivot Trung Khí via `daily_pivots_for_year` (JD-sorted list from scanner); (2) find the first Giáp Tý with JD ≥ pivot_jd; (2b) Pitfall P-7 fall-back — if `target_jd` precedes that first Giáp Tý, walk back to the prior pivot and recompute seed; (3) classify pivot (Dương/Âm) + derive direction + seed; (4) count Giáp Tý cycles from seed to target; (5) compute `center = (seed ± n).rem_euclid(9)`; (6) fill palaces via `fill_palaces(center, ascending)`; (7) emit the evidence envelope with `pivot=`/`seed=`/`direction=`/`center=` for audit replay.
- **6 Trung Khí pivots** classified per ADR-0004 §2: Đông Chí / Vũ Thuỷ / Cốc Vũ (Dương → thuận, seeds 1/7/4); Hạ Chí / Xử Thử / Sương Giáng (Âm → nghịch, seeds 9/3/6). Matched via `pivot_kind` + `pivot_starting_star` helpers; "Vũ Thuỷ" and "Vũ Thủy" accepted as the same pivot (Unicode NFC/NFD unification).
- **`daily_pivots_for_year(scanner, year)`** returns JD-sorted pivot list spanning `[year-1, year, year+1]` — broadened from the plan's `[year, year+1]` to handle late-December dates that need to walk back to the prior year's Sương Giáng pivot without caller-side retry.
- **Pitfall P-7 mechanic**: explicit prior-pivot fallback when `target_jd < first_giap_ty_seed_jd_in_new_tiet_khi`. Example: 2024-12-25 falls between Đông Chí 2024-12-21 and the first Giáp Tý in that Tiết Khí (early Jan 2025); the algorithm selects Sương Giáng 2024 (seed=6, Âm/nghịch) rather than wrongly assuming Đông Chí has kicked in.
- **Pitfall P-8 evidence envelope**: every layout carries `method=phi_tinh.nhat`, `source_id=SOURCE_HUYEN_KHONG` (const import, not a bare string), and an audit-replay note containing `date=...;pivot=...;seed=...;days_from_seed=...;center=...;direction=thuận|nghịch;confidence=high`.
- **`mod.rs` wiring**: `pub mod daily;` declared after `pub mod annual;`, and `pub use daily::compute_daily_flying_stars;` re-exported on the public fengshui API surface — ready for the 18-03 external-crate tests and 18-04 DaySnapshot builder to consume.
- **11 unit tests** in `daily::tests`: pivot classification (`test_pivot_kind_duong_am_split` with should_panic for unknown names), seed values for all 6 pivots, `daily_pivots_for_year` coverage, permutation invariant across 8 representative dates, `FlyingStarPeriod::Daily` variant unpack, evidence method/source_id guards, Pitfall P-3 (daily center ≠ annual nien_center for 2024-12-25), Pitfall P-4 (Dương ascends vs Âm descends), Pitfall P-7 (prior-pivot fallback for 2024-12-25), Pitfall P-6 (boundary discipline via scanner — pre/post Đông Chí instant resolves to Sương Giáng vs Đông Chí), Pitfall P-8 (note contains pivot name + direction).
- **FS-16 closed at the algorithm step** (the golden dataset + DaySnapshot field land in Plans 18-03 / 18-04).

## Task Commits

Each task was committed atomically:

1. **Task 1: Algorithm + helpers + mod.rs wiring** - `27ce722` (feat) — *feat(phase-18): implement compute_daily_flying_stars algorithm with 6 Trung Khi pivot and Giap Ty seed mechanic (FS-16)* — daily.rs +178 lines, mod.rs +2 lines
2. **Task 2: 11 unit tests** - `083a483` (feat) — *feat(phase-18): add 11 daily Phi Tinh unit tests covering P-3/P-4/P-6/P-7/P-8 guards (FS-16)* — daily.rs +243/-2 lines

## Files Created/Modified

- `crates/amlich-core/src/almanac/fengshui/daily.rs` — Created. Module docstring + imports; private `PivotKind` enum; `pivot_kind` + `pivot_starting_star` helpers (Unicode-tolerant); `daily_pivots_for_year` scanner helper with 3-year bracket; `compute_daily_flying_stars` 7-step public algorithm; 11-test `#[cfg(test)] mod tests` block with shared helpers (`first_giap_ty_on_or_after`, `giap_ty_cycles`, `wrapped_center`, `note`, `raw_pivot_for_date`). Total 419 lines.
- `crates/amlich-core/src/almanac/fengshui/mod.rs` — `pub mod daily;` declared (line 14) after `pub mod annual;`; `pub use daily::compute_daily_flying_stars;` re-exported (line 27) on the public fengshui API surface for external-crate test consumers.

## Decisions Made

- **Scanner year bracket widened from `[year, year+1]` to `[year-1, year, year+1]`** — the plan's `[year, year+1]` bracket fails for late-December target dates that must walk back past the prior year's Sương Giáng pivot (e.g., 2024-12-25 falls after Đông Chí 2024-12-21 but the Giáp Tý seed lands in early Jan 2025, so the algorithm must consult the 2024 Sương Giáng pivot). The 3-year bracket makes this robust without caller-side retry logic.
- **Unicode NFC/NFD acceptance** — `pivot_kind` and `pivot_starting_star` accept both "Vũ Thuỷ" (NFD/legacy) and "Vũ Thủy" (NFC/preferred) as the same pivot. This mirrors the v1.5 source-corpus normalization discipline and prevents a spurious panic if an un-pre-normalized input reaches the matcher. The `daily_pivots_for_year` helper also accepts both forms in its NAMES list.
- **Direction rule re-confirmed opposite of ADR-0003** — Dương pivot → thuận (forward, +1 per Giáp Tý cycle); Âm pivot → nghịch (descending, -1). This is documented in the module docstring and locked by `test_compute_daily_direction_inversion_duong_vs_am`.
- **Shared `fill_palaces` reuse via `pub(crate)`** — no duplicate 9-palace walking logic introduced; the daily algorithm calls `annual::fill_palaces(center, ascending)` exactly once per compute. No new Lo Shu constants introduced.
- **Evidence envelope uses `SOURCE_HUYEN_KHONG` const import** — not a bare `"huyen-khong"` string. Verified by the existing `tests/source_id_guard.rs` CI guard at runtime.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Scanner year bracket widened from [year, year+1] to [year-1, year, year+1]**
- **Found during:** Task 1 (algorithm implementation)
- **Issue:** The plan's `daily_pivots_for_year` specified `[year, year+1]` lookup for early-January cases. However, for late-December target dates that must walk back to the PRIOR year's Sương Giáng pivot (e.g., 2024-12-25's actual selected pivot is Sương Giáng 2024), the 2-year bracket risks missing the predecessor pivot when the fall-back logic runs.
- **Fix:** Widened the bracket to `[year-1, year, year+1]` and bumped `Vec::with_capacity` from 12 to 18. This makes the prior-pivot lookup deterministic for any target date in the year without caller-side retry.
- **Files modified:** `crates/amlich-core/src/almanac/fengshui/daily.rs` (committed in 27ce722)
- **Verification:** `test_daily_pivots_for_year_returns_six_pivots` confirms ≥6 pivots returned; `test_compute_daily_giap_ty_seed_mechanic_p7` confirms the prior-pivot fall-back for 2024-12-25 selects Sương Giáng with the expected center; both pass.

**2. [Rule 2 - Missing Critical] Unicode NFC/NFD acceptance added to pivot matchers**
- **Found during:** Task 1 (algorithm implementation)
- **Issue:** The plan's `pivot_kind` and `pivot_starting_star` only matched "Vũ Thuỷ" (the legacy uniface form). The scanner corpus may emit "Vũ Thủy" (NFC/preferred Vietnamese form), which would panic at runtime — a correctness regression risk on un-pre-normalized input.
- **Fix:** Extended both matchers and the `NAMES` list in `daily_pivots_for_year` to accept both forms. Mirrors the v1.5 source-corpus normalization discipline.
- **Files modified:** `crates/amlich-core/src/almanac/fengshui/daily.rs` (committed in 27ce722 + 083a483 — `test_pivot_starting_star_values` asserts both spellings yield seed=7)
- **Verification:** `test_pivot_starting_star_values` covers both forms; passes.

---

**Total deviations:** 2 auto-fixed (1 bug, 1 missing critical).
**Impact on plan:** Both auto-fixes harden boundary correctness and Unicode robustness. No scope creep; the public API and ADR-0004 contract are unchanged.

## Issues Encountered

None — the algorithm compiled cleanly on first build, and the 11 tests passed on first run (after the 2 in-flight auto-fixes for the scanner bracket and Unicode acceptance).

## User Setup Required

None — no external service configuration required. All work is local Rust.

## Next Phase Readiness

- **Plan 18-03 (FS-18) ready to proceed**: the `compute_daily_flying_stars(date, scanner) -> DailyFlyingStarLayout` public API is now on the fengshui module re-export surface. The 18-03 golden dataset will use it as the algorithm ground truth, and the external-crate integration tests will import it via `use amlich_core::almanac::fengshui::compute_daily_flying_stars`.
- **Plan 18-04 (FS-19) ready**: `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` additive field will be populated by `compute_daily_flying_stars`; the v1.5→v1.6 fixture round-trip test pattern is unchanged.
- **No blockers**, no architectural decisions deferred. The daily layer is algorithmically complete; only validation corpus + DTO wiring remain.

---
*Phase: 18-daily-phi-tinh*
*Completed: 2026-07-15*

## Self-Check: PASSED

All claimed files exist on disk and the claimed behavior is verified:
- `crates/amlich-core/src/almanac/fengshui/daily.rs` ✓ (419 lines: algorithm + 11 tests; `pub fn compute_daily_flying_stars`, `fn pivot_kind`, `fn pivot_starting_star`, `fn daily_pivots_for_year`, `pivot_kind`, `pivot_starting_star`, `daily_pivots_for_year`, "phi_tinh.nhat", `fill_palaces(center` all present)
- `crates/amlich-core/src/almanac/fengshui/mod.rs` ✓ (`pub mod daily;` at line 14; `pub use daily::compute_daily_flying_stars;` at line 27)
- Both task commits present on branch: `27ce722` (algorithm + mod.rs wiring) and `083a483` (11 tests)
- Test status at write time: `cargo build -p amlich-core --quiet` EXIT 0; `cargo test -p amlich-core --lib almanac::fengshui::daily::` reports 11/11 passing; full fengshui suite `cargo test -p amlich-core --lib almanac::fengshui::` reports 107/107 passing (zero regressions vs the 18-01 baseline)
