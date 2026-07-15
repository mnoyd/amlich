---
phase: 18-daily-phi-tinh
plan: 04
subsystem: fengshui
tags: [phi-tinh, daily, lua-nhat, fs-19, daysnapshot, serde-additive, crit-3-isolation, grep-guard]

# Dependency graph
requires:
  - phase: 18-daily-phi-tinh/18-01
    provides: ADR-0004 daily starting-star convention + locked DailyFlyingStarLayout sibling struct (the field type used on DaySnapshot)
  - phase: 18-daily-phi-tinh/18-02
    provides: compute_daily_flying_stars(date, scanner) -> DailyFlyingStarLayout public API (auto-populated into DaySnapshot.daily_flying_stars)
  - phase: 18-daily-phi-tinh/18-03
    provides: golden validator + load_daily_flying_stars_golden loader (the field is validated against this dataset transitively)
provides:
  - Additive daily_flying_stars: Option<DailyFlyingStarLayout> field on DaySnapshot with the EXACT serde pattern as flying_stars / applicable_rituals (#[serde(default, skip_serializing_if = "Option::is_none")])
  - calculate_day_snapshot_internal auto-populates snap.daily_flying_stars = Some(compute_daily_flying_stars(...)) via a dedicated block between the flying_stars and applicable_rituals populate blocks
  - v1.5 -> v1.6 backward-compatibility proven by 3 new round-trip tests in tests/day_snapshot_v14_compat.rs (missing key deserializes to None; byte-equal round-trip; absent in JSON when None)
  - tests/fengshui_crit3_isolation.rs grep guard asserting direction_merge.rs does NOT contain any of 6 forbidden Phi Tinh patterns (CRIT-3 / Pitfall P-1 isolation discipline)
affects:
  - phase: 19-recommends-offering (v1.5 -> v1.6 backward-compat round-trip test extends the daily_flying_stars surface; the 2026 E2E smoke can exercise the daily field)
  - future phases touching DaySnapshot (any v1.7+ additive field follows the same serde additive pattern)

# Tech tracking
tech-stack:
  added: []  # No new dependencies; pure additive Option<T> field with serde derive
  patterns:
    - additive Option<T> field on a locked DTO with #[serde(default, skip_serializing_if = "Option::is_none")] (mirrors the v1.4 -> v1.5 flying_stars / applicable_rituals precedent)
    - populate-block pattern between existing populate blocks (flying_stars -> daily_flying_stars -> applicable_rituals) — keeps compute_day_snapshot_internal readable and minimizes diff blast radius
    - file-based grep guard for cross-module type isolation (tests/fengshui_crit3_isolation.rs mirrors tests/source_id_guard.rs but targets Phi Tinh type names instead of source_id string literals)

key-files:
  created:
    - crates/amlich-core/tests/fengshui_crit3_isolation.rs
  modified:
    - crates/amlich-core/src/lib.rs
    - crates/amlich-core/tests/day_snapshot_v14_compat.rs

key-decisions:
  - "daily_flying_stars uses the EXACT serde pattern as flying_stars / applicable_rituals (#[serde(default, skip_serializing_if = \"Option::is_none\")]) — additive Option<T> on a locked DTO, no existing field mutated, no removals in the diff."
  - "The daily_flying_stars populate block sits BETWEEN the flying_stars and applicable_rituals populate blocks (not at the end of calculate_day_snapshot_internal) — keeps the three additive surfaces grouped for readability."
  - "Solar day/month/year extracted from snap.context.solar (the v1.5 DayContext shape) rather than from the raw (day, month, year) i32 args — guarantees the daily layout matches the snapshot's own DayContext, including any timezone correction in compute_day_context."
  - "tests/fengshui_crit3_isolation.rs is semantically DISTINCT from tests/source_id_guard.rs: source_id_guard forbids bare source_id STRING LITERALS; crit3_isolation forbids Phi Tinh TYPE NAMES / MODULE PATHS leaking into direction_merge.rs. Both guards are needed and complementary."
  - "6 forbidden patterns chosen to cover every leak vector: type import (FlyingStar, DailyFlyingStar, DailyFlyingStarLayout), module path (almanac::fengshui, phi_tinh), function name (compute_daily_flying_stars)."

patterns-established:
  - "Pattern: additive Option<T> field with #[serde(default, skip_serializing_if = \"Option::is_none\")] on a locked DTO — the v1.4 -> v1.5 -> v1.6 evolution demonstrates that this pattern keeps backward-compat deserialization trivial (missing key -> None) and round-trip byte-equal."
  - "Pattern: dedicated cross-module isolation grep guard — when a type boundary discipline (e.g. CRIT-3 disjoint between khcbppt directions and huyen-khong palace layouts) must be enforced long-term, a tests/<feature>_isolation.rs file with a forbidden-patterns list and a single assert! provides a compile-time-stable guard independent of lint config."
  - "Pattern: populate-block grouping in builders — additive surfaces on a builder fn are grouped in dedicated blocks (flying_stars / daily_flying_stars / applicable_rituals) rather than inlined into the constructor, keeping diff blast radius minimal and the builder readable."

requirements-completed: [FS-19]

# Metrics
duration: 3min
completed: 2026-07-15
---

# Phase 18 Plan 04: DaySnapshot.daily_flying_stars Additive Field Summary

**Additive `daily_flying_stars: Option<DailyFlyingStarLayout>` field on `DaySnapshot` with the exact serde additive pattern as `flying_stars` / `applicable_rituals`, auto-populated in `calculate_day_snapshot_internal` via `compute_daily_flying_stars`, plus 3 new v1.5→v1.6 round-trip tests and a `tests/fengshui_crit3_isolation.rs` grep guard enforcing 6 forbidden Phi Tinh patterns stay out of `interaction/direction_merge.rs` (FS-19 closed; Phase 18 complete).**

## Performance

- **Duration:** ~3 min
- **Started:** 2026-07-15T14:01:12Z
- **Completed:** 2026-07-15T14:04:45Z
- **Tasks:** 2
- **Files modified:** 3 (1 source file modified, 1 test file modified, 1 test file created)

## Accomplishments

- **`DaySnapshot.daily_flying_stars: Option<crate::almanac::fengshui::types::DailyFlyingStarLayout>`** added as the 4th additive surface on the DTO, carrying the daily Phi Tinh (Lưu Nhật / 日紫白) overlay from Plan 18-02's algorithm. The field uses `#[serde(default, skip_serializing_if = "Option::is_none")]` — EXACT mirror of the existing `flying_stars` (line 163-164) and `applicable_rituals` (line 166-167) attributes — so absent in JSON when None and lenient on deserialize when missing.
- **`calculate_day_snapshot_internal` auto-population**: a dedicated block BETWEEN the existing `flying_stars` and `applicable_rituals` populate blocks runs `compute_daily_flying_stars((solar_year, solar_month, solar_day), &TietKhiScanner::new())` and assigns the result to `snap.daily_flying_stars`. The solar Y/M/D are extracted from `snap.context.solar` (the v1.5 `DayContext` shape), guaranteeing the daily layout matches the snapshot's own context. No other `DaySnapshot` field is mutated.
- **Diff is additive-only**: `git diff crates/amlich-core/src/lib.rs` shows exactly 3 hunks — field decl, `daily_flying_stars: None` constructor init, populate block — with zero removals (verified before commit).
- **3 new round-trip tests** in `tests/day_snapshot_v14_compat.rs` (now 6 total):
  - `v15_json_without_daily_flying_stars_deserializes` — a v1.5-shaped JSON (flying_stars + applicable_rituals populated, daily_flying_stars absent) deserializes cleanly into the v1.6 struct with daily_flying_stars defaulting to None. Proves v1.5 → v1.6 backward compatibility.
  - `daily_flying_stars_byte_equal_round_trip` — serialize → deserialize → re-serialize yields byte-equal JSON, and `daily_flying_stars.center_star` survives the round-trip.
  - `daily_flying_stars_absent_when_none` — when `daily_flying_stars` is None, the key does NOT appear in the serialized JSON (skip_serializing_if honored).
- **`tests/fengshui_crit3_isolation.rs`** authored with 1 grep guard test (`direction_merge_does_not_import_flying_star_or_daily_flying_star`). Uses `env!("CARGO_MANIFEST_DIR")` to locate `src/interaction/direction_merge.rs` at compile time and asserts NONE of the 6 forbidden patterns appear in its contents: `FlyingStar`, `DailyFlyingStar`, `DailyFlyingStarLayout`, `almanac::fengshui`, `phi_tinh`, `compute_daily_flying_stars`. Each pattern targets a different leak vector (type name, sibling struct name, module path, snake-case module path, function name). Failure message names the offending pattern and cites Pitfalls P-1 / CRIT-3.
- **FS-19 closed** — the final Phase 18 requirement is satisfied: a user-of-`DaySnapshot` can find the additive `daily_flying_stars: Option<DailyFlyingStarLayout>` field with the locked serde attributes, and v1.5 fixtures round-trip cleanly through the field absent.

## Task Commits

Each task was committed atomically:

1. **Task 1: DaySnapshot field + auto-populate** - `defe59e` (feat) — *feat(phase-18): add DaySnapshot.daily_flying_stars additive field + auto-populate (FS-19)* — lib.rs +19 lines, 0 removals
2. **Task 2: 3 round-trip tests + CRIT-3 grep guard** - `e655140` (feat) — *feat(phase-18): add daily_flying_stars round-trip tests + CRIT-3 grep guard (FS-19)* — day_snapshot_v14_compat.rs +73 lines, fengshui_crit3_isolation.rs +42 lines (new file)

## Files Created/Modified

- `crates/amlich-core/src/lib.rs` — Modified. 3 additive hunks on the `DaySnapshot` struct and `calculate_day_snapshot_internal`: (a) new `daily_flying_stars` field decl with `#[serde(default, skip_serializing_if = "Option::is_none")]` matching the `flying_stars` / `applicable_rituals` pattern; (b) `daily_flying_stars: None` constructor init; (c) new populate block calling `compute_daily_flying_stars((year, month, day), &TietKhiScanner::new())` between the `flying_stars` and `applicable_rituals` populate blocks. No other field modified, zero removals.
- `crates/amlich-core/tests/day_snapshot_v14_compat.rs` — Modified. 3 new test functions appended after Test 3 (`v14_json_without_new_fields_deserializes`): `v15_json_without_daily_flying_stars_deserializes`, `daily_flying_stars_byte_equal_round_trip`, `daily_flying_stars_absent_when_none`. Test count 3 → 6.
- `crates/amlich-core/tests/fengshui_crit3_isolation.rs` — Created. ~42 lines. Module docstring explaining the khcbppt/huyen-khong boundary discipline (DEC-0015/0016, CRIT-3, Pitfall P-1). `FORBIDDEN_TYPE_NAMES` constant with the 6 patterns. One `#[test]` function reading `src/interaction/direction_merge.rs` via `env!("CARGO_MANIFEST_DIR")` and asserting none of the forbidden patterns appear in the file contents.

## Decisions Made

- **Serde attributes EXACTLY mirror existing surfaces** — `daily_flying_stars` uses the same `#[serde(default, skip_serializing_if = "Option::is_none")]` pair as `flying_stars` (line 163-164) and `applicable_rituals` (line 166-167). No new attribute, no new serde derive. This is the v1.4 → v1.5 → v1.6 additive-Option pattern; it keeps backward-compat deserialization trivial (missing key → None) and round-trip byte-equal.
- **Populate block placement** — the new `daily_flying_stars` block sits BETWEEN the `flying_stars` and `applicable_rituals` populate blocks rather than at the end of `calculate_day_snapshot_internal`. This groups the three additive surfaces together for readability and keeps the diff blast radius minimal (no reorder of existing blocks).
- **Solar Y/M/D extracted from `snap.context.solar`** — the daily layout uses the snapshot's own `DayContext.solar.year / month / day` rather than the raw `(day, month, year)` i32 args to `calculate_day_snapshot_internal`. This guarantees the daily layout matches the snapshot's context, including any timezone correction in `compute_day_context`. Casts are explicit (`month as u32`, `day as u32`) because `DayContext.solar.month/day` are `i32` but `compute_daily_flying_stars` takes `(i32, u32, u32)`.
- **`fengshui_crit3_isolation.rs` is a SEPARATE file from `source_id_guard.rs`** — `source_id_guard.rs` guards against bare source_id STRING LITERALS; `fengshui_crit3_isolation.rs` guards against Phi Tinh TYPE NAMES / MODULE PATHS leaking into `direction_merge.rs`. Both guards are needed and complementary. The new file's module docstring explicitly cites the disjoint concern.
- **6 forbidden patterns** — chosen to cover every leak vector: type import (`FlyingStar`, `DailyFlyingStar`, `DailyFlyingStarLayout`), module path (`almanac::fengshui`, `phi_tinh`), function name (`compute_daily_flying_stars`). A future regression that adds any one of these to `direction_merge.rs` will fail the test loudly with the offending pattern cited.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None — the lib.rs change compiled cleanly on first build, the 3 new round-trip tests passed on first run, the CRIT-3 grep guard passed on first run (the current `direction_merge.rs` contains none of the 6 forbidden patterns), and the full-crate gate (`cargo test -p amlich-core`) reported zero regressions (709 lib tests + all integration tests passing).

## User Setup Required

None - no external service configuration required. All work is local Rust.

## Next Phase Readiness

- **Phase 18 COMPLETE (4/4 plans executed, FS-16/17/18/19 all closed)** — the daily Phi Tinh layer is now end-to-end: ADR-0004 convention (18-01) + algorithm (18-02) + golden dataset (18-03) + DaySnapshot field (18-04). Phase verification can be run.
- **Phase 19 (RecommendsOffering + v1.6 Integration) ready to proceed**: the v1.5 → v1.6 backward-compat round-trip test pattern (Plan 19-03 INT-10) can extend the surface proven here; the 2026 E2E smoke test (Plan 19-03) can sample days that exercise BOTH the annual/monthly `flying_stars` field AND the new `daily_flying_stars` field together.
- **No blockers**, no architectural decisions deferred. The `daily_flying_stars` field is the last v1.6 DTO addition needed before Phase 19's RecommendsOffering work.

---
*Phase: 18-daily-phi-tinh*
*Completed: 2026-07-15*

## Self-Check: PASSED

All claimed files exist on disk and the claimed behavior is verified:
- `crates/amlich-core/src/lib.rs` ✓ (additive `daily_flying_stars: Option<crate::almanac::fengshui::types::DailyFlyingStarLayout>` field with `#[serde(default, skip_serializing_if = "Option::is_none")]`; `daily_flying_stars: None` constructor init; populate block calling `compute_daily_flying_stars` between the `flying_stars` and `applicable_rituals` blocks; zero removals in diff)
- `crates/amlich-core/tests/day_snapshot_v14_compat.rs` ✓ (3 new test functions: `v15_json_without_daily_flying_stars_deserializes`, `daily_flying_stars_byte_equal_round_trip`, `daily_flying_stars_absent_when_none`)
- `crates/amlich-core/tests/fengshui_crit3_isolation.rs` ✓ (new file; 6 forbidden patterns; `direction_merge_does_not_import_flying_star_or_daily_flying_star` test)
- Both task commits present on branch: `defe59e` (feat lib.rs) and `e655140` (feat tests)
- Test status at write time: `cargo build -p amlich-core --quiet` EXIT 0; `cargo test -p amlich-core --test day_snapshot_v14_compat` reports 6/6 passing (3 pre-existing + 3 new); `cargo test -p amlich-core --test fengshui_crit3_isolation` reports 1/1 passing; `cargo test -p amlich-core` full crate reports 709 lib tests + all integration tests passing (zero regressions vs the 18-03 baseline)
