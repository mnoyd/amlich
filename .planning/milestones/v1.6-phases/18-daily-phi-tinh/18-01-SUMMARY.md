---
phase: 18-daily-phi-tinh
plan: 01
subsystem: fengshui
tags: [phi-tinh, daily, lua-nhat, adr-0004, schema-lock, serde]

# Dependency graph
requires:
  - phase: 13-foundation-phi-tinh
    provides: FlyingStar/FlyingStarLayout/FlyingStarPeriod types + locked field set + fill_palaces helper
  - phase: 16-foundation-adr-0003-confidence-closure
    provides: deferral-discipline precedent (ADR-0003a §4) for the page-citation deferral in ADR-0004
  - phase: 17-van-khan-reviewer-closure
    provides: v1.5/v1.6 additive DTO pattern + cross-phase boundary discipline
provides:
  - ADR-0004 daily Phi Tinh starting-star convention (6-pivot table + Dương-thuận/Âm-nghịch + Giáp-Tý-seed + chapter+verse citation + 3 alternatives)
  - Additive FlyingStarPeriod::Daily { date: (i32, u32, u32) } variant on the locked enum
  - Sibling DailyFlyingStarLayout struct mirroring the locked FlyingStarLayout's 4-field shape (period, palaces, center_star, evidence)
  - Re-export of DailyFlyingStarLayout (and locked types) from almanac::fengshui::mod.rs for external-crate test consumers
  - Extended test_flying_star_period_serde_round_trip test (4 cases) + new test_daily_flying_star_layout_period_serde test (passing)
affects:
  - phase: 18-daily-phi-tinh/18-02 (compute_daily_flying_stars algorithm builds against the locked schema)
  - phase: 18-daily-phi-tinh/18-03 (golden dataset uses the additive "daily" kind + the ADR-0004 6-pivot table)
  - phase: 18-daily-phi-tinh/18-04 (DaySnapshot.daily_flying_stars additive field uses the sibling struct)

# Tech tracking
tech-stack:
  added: []  # No new dependencies; mirrors existing serde derive macros + companion surface for DailyFlyingStarLayout
  patterns:
    - additive-only DTO extension on a locked enum (FlyingStarPeriod gains a 4th variant, no mutation of existing 3)
    - sibling struct pattern when the locked field set must remain frozen (DailyFlyingStarLayout duplicates the 4 fields rather than mutating FlyingStarLayout)
    - serde tag = "kind" + rename_all = "snake_case" on the period enum keeps Daily { date: ... } JSON shaped as {"kind": "daily", "date": [y, m, d]}
    - i32 year / u32 month, day tuple mirrors SolarDate day/month/year style at lib.rs:110-115 (zero new chrono deps)

key-files:
  created:
    - .planning/adrs/0004-daily-phi-tinh-starting-star-convention.md
  modified:
    - crates/amlich-core/src/almanac/fengshui/types.rs
    - crates/amlich-core/src/almanac/fengshui/mod.rs

key-decisions:
  - "Locked FlyingStarLayout field set is FROZEN — the daily layer introduces a sibling DailyFlyingStarLayout struct rather than mutating the locked field set (per types.rs:106-118 lock and CONTEXT.md discipline)"
  - "6 Trung Khí pivots partition the year (Đông Chí, Vũ Thuỷ, Cốc Vũ, Hạ Chí, Xử Thử, Sương Giáng) — winter pivots are Dương→thuận (forward, mod 9 wrapping 1↔9); summer pivots are Âm→nghịch (descending, mod 9 wrapping 1↔9)"
  - "Dương/Âm direction rule is the OPPOSITE of the ADR-0003 annual layer — daily rule uses Tiết-Khí pivot polarity, annual rule uses year-stem polarity; the two are not interchangeable"
  - "Giáp Tý-as-seed-day mechanic (Pitfall P-7): the pivot seed 'kicks in' at the FIRST Giáp Tý (Can=0, Chi=0) with JD >= pivot_jd, not at the pivot instant itself"
  - "i32-tuple date shape (i32, u32, u32) mirrors SolarDate's day/month/year style at lib.rs:110-115 — no new chrono::NaiveDate dependency introduced"
  - "Classic citation trails to *Thẩm Thị Huyền Không Học* chapter 三元日白訣 / Tam Nguyên Nhật Bạch Quyết only; exact page-level citation is deferred per Phase 16 deferral discipline (mirrors the 1960 case in ADR-0003a §4) — chapter + verse name achievable from open references, exact page awaits a numbered-edition lookup"

patterns-established:
  - "Pattern: locked-field-set + sibling-struct — when a schema is frozen, additive sibling structs (rather than field-set mutations) carry time-layer-specific period variants while sharing the same shape"
  - "Pattern: additive enum variants on discriminator enums — FlyingStarPeriod gains the 4th variant without mutating Van / Yearly / Monthly, preserving all existing serde round-trips verbatim"

requirements-completed: [FS-17]

# Metrics
duration: 5min
completed: 2026-07-15
---

# Phase 18 Plan 01: Daily Phi Tinh Schema-Lock Summary

**ADR-0004 daily starting-star convention with 6-pivot Dương-thuận/Âm-nghịch table, Giáp-Tý-seed mechanic, and chapter+verse citation in *Thẩm Thị Huyền Không Học*; additive `FlyingStarPeriod::Daily { date: (i32, u32, u32) }` variant and sibling `DailyFlyingStarLayout` struct on the frozen v1 type contract (FS-17 closed).**

## Performance

- **Duration:** 5 min
- **Started:** 2026-07-15T13:09:25Z
- **Completed:** 2026-07-15T13:14:48Z
- **Tasks:** 2
- **Files modified:** 3 (ADR-0004 created, types.rs + mod.rs modified)

## Accomplishments

- **ADR-0004 authored** at `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` (~131 lines), with: §1 boundary semantics (6 Trung Khí pivots via `TietKhiScanner`, NOT naïve calendar), §2 the 6-pivot table (starting star + classical name + polarity + direction per pivot), §3 the Dương-thuận/Âm-nghịch direction rule (with explicit acknowledgment that this is OPPOSITE the ADR-0003 annual rule), §4 the Giáp Tý-as-seed-day mechanic with a worked example (Đông Chí 2021 / Quý Mão / 22-day window before Giáp Tý on 11/1/2022), §5 the *Thẩm Thị Huyền Không Học* chapter+verse citation with explicit page-deferral note per Phase 16 deferral discipline, §6 three alternative conventions explicitly REJECTED with full rationale (naïve calendar, annual-seed descent, lunar-month bounded pivots), §7 the adopted convention, plus `## Consequences` and `## References` sections.
- **Additive `Daily { date: (i32, u32, u32) }` variant** added to the locked `FlyingStarPeriod` enum after `Monthly { … }` — existing 3 variants untouched, serde tag + snake_case rename preserved.
- **Sibling `DailyFlyingStarLayout` struct** added immediately after the locked `FlyingStarLayout` (which remains at exactly 4 `pub` fields, verified by grep). The sibling has the 4 fields mirroring the locked shape: `period, palaces, center_star, evidence`.
- **Tests extended and added**: `test_flying_star_period_serde_round_trip` extended with the `Daily { date: (2024, 12, 25) }` case (now 4 cases); new test `test_daily_flying_star_layout_period_serde` exercises the sibling struct's serde round-trip and asserts the `Daily` period variant unpacks correctly.
- **Re-export added** in `crates/amlich-core/src/almanac/fengshui/mod.rs` for `DailyFlyingStarLayout` (plus `FlyingStar`, `FlyingStarLayout`, `FlyingStarPeriod`, `Palace`, `minimal_evidence`) so external-crate test consumers (Plan 18-03 and 18-04) can import the new sibling struct without reaching into `types` directly.
- **FS-17 closed at the schema-lock step** (the algorithm + golden dataset + DaySnapshot field land in Plans 18-02 / 18-03 / 18-04).

## Task Commits

Each task was committed atomically:

1. **Task 1: Author ADR-0004** - `b2265eb` (docs) — *docs(phase-18): author ADR-0004 daily Phi Tinh starting-star convention with 6-pivot table and Tam Nguyen Nhat Bach Quyet citation (FS-17)*
2. **Task 2: Schema extension** - `a593a13` (feat) — *feat(phase-18): add DailyFlyingStarLayout sibling struct + FlyingStarPeriod::Daily variant (FS-17 schema lock)*

## Files Created/Modified

- `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` — ADR-0004 (Context, Decision with 7 sections including 3 REJECTED alternatives + chapter+verse citation + page-deferral note, Consequences with FS-16/17/18/19 forward pointers, References to 3 Vietnamese sources + classical text + 3 in-repo ADRs)
- `crates/amlich-core/src/almanac/fengshui/types.rs` — additive `Daily` variant on `FlyingStarPeriod` enum; new sibling `DailyFlyingStarLayout` struct mirroring the locked 4-field shape; doc comment on the enum updated with the `Daily` row; one new test function + one extended test case
- `crates/amlich-core/src/almanac/fengshui/mod.rs` — re-export block extended to surface `DailyFlyingStarLayout` (and locked types) for external-crate test consumers

## Decisions Made

- **Locked-field-set discipline honored** — `FlyingStarLayout` is mutated by zero fields (verified: 4 `pub` fields exactly, before and after). The daily layer introduces a sibling struct (`DailyFlyingStarLayout`) rather than adding a field to the locked shape, per the `types.rs:106` FROZEN comment and the v1.5 schema-lock-before-algorithm discipline.
- **i32-tuple date shape chosen** over `chrono::NaiveDate` to avoid introducing a new transitive dependency. The `(i32 year, u32 month, u32 day)` tuple mirrors the existing `SolarDate { day: i32, month: i32, year: i32 }` style at `lib.rs:110-115` while keeping the month/day as `u32` to match the natural non-negative invariants and prevent accidental negative-day encoding.
- **serde tag preserved** — the existing `#[serde(tag = "kind", rename_all = "snake_case")]` attribute on `FlyingStarPeriod` automatically renders the new variant as `{"kind": "daily", "date": [y, m, d]}` and deserializes back to `Daily { date: (y, m, d) }`. No new attribute or serializer needed; round-trip is byte-equal.
- **Page-citation deferral mirrors ADR-0003a §4** — the *Thẩm Thị Huyền Không Học* citation trails to chapter + verse name only ("三元日白訣 / Tam Nguyên Nhật Bạch Quyết"). The exact page is explicitly deferred per Phase 16 deferral discipline. The audit trail in ADR-0004 §5 names the three Vietnamese-language secondary modern sources that cite the chapter by name and acknowledges the numbered-edition availability gap.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- The plan's literal grep command for the locked `FlyingStarLayout` field count used `grep -A 6 'pub struct FlyingStarLayout {' ... | grep -c 'pub '`, which counts the `pub struct` declaration plus the 4 `pub` fields and therefore reports `5`. Verification was completed with a struct-body-only field counter and confirmed exactly 4 fields remain.
- The plan's `done` criterion referenced "5 pre-existing + 2 new = 7" tests. The actual types-test module contains 5 pre-existing test functions + 1 new test function, with the Daily variant added as an extra case inside an existing test function. `cargo test -p amlich-core --lib almanac::fengshui::types::tests` reports 6/6 pass; the substantive serde and sibling-layout checks are satisfied.

## User Setup Required

None - no external service configuration required. All work is local Rust + Markdown.

## Next Phase Readiness

- **Plan 18-02 (FS-16) ready to proceed**: `compute_daily_flying_stars(date, scanner) -> DailyFlyingStarLayout` algorithm builds against the locked schema. The 6-pivot table from ADR-0004 §2 and the direction rule from §3 are now in a single canonical document. Algorithm imports `pub(crate) fn fill_palaces(center, ascending)` from `annual.rs` (existing pattern), no duplication required.
- **Plan 18-03 (FS-18) ready**: daily golden dataset can adopt the `kind: "daily"` validator extension in `golden.rs:200` and the per-pivot coverage rule derived from the 6-pivot table.
- **Plan 18-04 (FS-19) ready**: `DaySnapshot.daily_flying_stars: Option<DailyFlyingStarLayout>` additive field follows the exact `#[serde(default, skip_serializing_if = "Option::is_none")]` pattern at `lib.rs:163-164`, and the v1.5→v1.6 fixture round-trip test pattern from `tests/day_snapshot_v14_compat.rs:73-128` is unchanged.
- **No blockers**, no architectural decisions deferred to subsequent plans. The v1.5 retrospective pattern (schema-lock-before-algorithm before builder emission) is honored end-to-end.

---
*Phase: 18-daily-phi-tinh*
*Completed: 2026-07-15*

## Self-Check: PASSED

All claimed files exist on disk:
- `.planning/adrs/0004-daily-phi-tinh-starting-star-convention.md` ✓ (131 lines, all 6 pivot names, all 6 classical star names, `Tam Nguyên Nhật Bạch Quyết`, `Thẩm Thị Huyền Không Học`, 8 `REJECTED` literals, page-deferral phrase)
- `crates/amlich-core/src/almanac/fengshui/types.rs` ✓ (additive `Daily { date: (i32, u32, u32) }` variant, sibling `DailyFlyingStarLayout` struct, locked `FlyingStarLayout` field set unchanged at exactly 4 `pub` fields)
- `crates/amlich-core/src/almanac/fengshui/mod.rs` ✓ (`DailyFlyingStarLayout` re-export added)
- Both task commits present on branch: `b2265eb` (docs) and `a593a13` (feat)
- Test status: `cargo build -p amlich-core --quiet` passed; `almanac::fengshui::types::tests` 6/6 pass; zero regressions in the required Phase 18-01 gate
