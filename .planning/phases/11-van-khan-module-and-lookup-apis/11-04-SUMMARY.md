---
phase: 11-van-khan-module-and-lookup-apis
plan: "04"
subsystem: testing
tags: [rituals, integration-test, van-khan, public-api, nfc, leap-policy, solar-term]

# Dependency graph
requires:
  - phase: 11-03
    provides: matcher.rs with 4 lookup APIs + leap-aware event_key_matches; rituals/mod.rs public re-exports
  - phase: 11-01
    provides: van-khan-ram-thang-gieng fixture (makes Test 2 falsifiable)
provides:
  - 6 black-box integration tests at crates/amlich-core/tests/rituals_integration.rs
  - external-crate consumer signature verified (use amlich_core::rituals::{...})
  - Falsifiable Sóc/Vọng snapshot path coverage (lunar 1/15 → van-khan-ram-thang-gieng)
  - HolidayId cross-reference guard (every fixture id resolves to real Holiday.id in 2020-2030)
  - NFC byte-equal round-trip coverage on the serde path for every corpus entry
  - Leap-policy semantic test at the public API surface
affects:
  - Phase 15 (E2E validation will extend rituals_integration.rs with snapshot-driven DTO assertions)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "External-crate integration tests for crate-public API surfaces (treat src as black box; consume via `use amlich_core::...`)"
    - "Falsifiable test design — assert non-empty result set against a specifically introduced fixture (van-khan-ram-thang-gieng) to prevent vacuous passes"
    - "Asymmetric matcher semantic — haystack-side Always matches any needle; needle-side Always only matches Always haystack"

key-files:
  created:
    - crates/amlich-core/tests/rituals_integration.rs
  modified:
    - crates/amlich-core/src/rituals/matcher.rs

key-decisions:
  - "Matcher Always semantic switched from symmetric to asymmetric — `(Always, _) => true`, removed `(_, Always) => true`; needle-side Always now only matches Always haystack. Symmetric arm caused every LifeEvent-only entry (e.g. van-khan-dong-tho) to fire on every snapshot because derive_event_keys emits an Always needle. Caught by Test 2's per-hit honesty check."
  - "Inline matcher test `always_sentinel_matches_anything` updated in lockstep to encode the asymmetry (positive: Always haystack matches HolidayId needle; negative: HolidayId haystack does NOT match Always needle; identity: Always ↔ Always matches)."
  - "Single-commit delivery (test file + matcher fix together) chosen over split RED/GREEN because the failure surfaced an upstream matcher bug, not a missing test — the test file IS the failing-then-passing RED→GREEN signal."
  - "HolidayId cross-reference sweep uses years 2020-2030 to cover the project Core Value range and any year-offset edges (some holidays have year_offset ±1)."

patterns-established:
  - "Public-API black-box tests live at crates/<crate>/tests/<feature>_integration.rs and import via the external crate path"
  - "Per-hit honesty assertions inside snapshot integration tests — every returned entry must trace back to a snapshot-derivable event key (HolidayId / LunarDate / SolarTerm / Always haystack)"
  - "Matcher arm asymmetry is documented inline; a future schema variant change must visit both the match arms and this documentation"

requirements-completed:
  - RIT-01
  - RIT-07
  - RIT-08

# Metrics
duration: 2 min
completed: 2026-05-26
---

# Phase 11 Plan 04: Văn khấn Integration Tests Summary

**Six black-box integration tests covering Phase 11 success criteria #1, #4, #5 — and a Rule-1 matcher fix removing the spurious Always-needle symmetry that was making every LifeEvent entry fire on every snapshot.**

## Performance

- **Duration:** 2 min
- **Started:** 2026-05-26T16:50:17Z
- **Completed:** 2026-05-26T16:53:08Z
- **Tasks:** 1 (atomic — integration test file + matcher fix)
- **Files modified:** 2

## Accomplishments

- Six black-box integration tests at `crates/amlich-core/tests/rituals_integration.rs` exercising the Phase 11 public API surface from outside the crate (`use amlich_core::rituals::{...}`):
  - Test 1: Tết Nguyên Đán 2024 snapshot returns a ritual with `HolidayId{tet-nguyen-dan}` (RIT-01 end-to-end).
  - Test 2: Vọng snapshot (lunar 1/15) returns `van-khan-ram-thang-gieng` — falsifiable via per-hit honesty assertion.
  - Test 3: Thanh Minh SolarTerm path — `Holiday.id=None` so only the SolarTerm derivation can fire.
  - Test 4: Every `HolidayId{value}` in fixtures resolves to a real `Holiday.id` from `get_vietnamese_holidays` across 2020-2030 (typo guard).
  - Test 5: Byte-equal NFC round-trip `serialize → deserialize → re-serialize` on every corpus entry.
  - Test 6: Leap-policy semantics at the API surface — `CanonicalMonthOnly` fixture must not match a `LeapMonthOnly` needle and must match a `CanonicalMonthOnly` needle.
- Auto-fixed a matcher correctness bug uncovered by Test 2's honesty check (see Deviations).
- Full crate test suite stays green: 597 lib tests + all integration test binaries pass.

## Task Commits

1. **Task 1: Write 6 integration tests in tests/rituals_integration.rs + fix Always-needle matcher asymmetry** — `e0cb5b4` (test)

**Plan metadata:** (to be appended after this summary commits)

## Files Created/Modified

- `crates/amlich-core/tests/rituals_integration.rs` — Six `#[test]` functions; ~210 LOC; consumes `amlich_core::rituals`, `amlich_core::holidays`, `amlich_core::calculate_day_snapshot` as an external crate. Cargo auto-discovers it as a new test binary `rituals_integration`.
- `crates/amlich-core/src/rituals/matcher.rs` — Changed `event_key_matches` from symmetric `(Always, _) | (_, Always) => true` to asymmetric `(Always, _) => true`. Updated doc-comment to document the new semantic. Updated inline test `always_sentinel_matches_anything` to encode the asymmetry (positive Always-haystack arm, negative Always-needle arm, identity Always↔Always arm).

## Decisions Made

See frontmatter `key-decisions`. Summary:
- Asymmetric `Always` is semantically correct: a query needle of `Always` should find entries tagged Always, not the entire corpus.
- The integration test file and matcher fix shipped in one commit because Test 2 IS the falsifier for the matcher bug.
- HolidayId sweep range 2020-2030 matches the project Core Value statement.

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Always-needle symmetric matcher caused universal match**

- **Found during:** Task 1, running `cargo test -p amlich-core --test rituals_integration`.
- **Issue:** `event_key_matches` had `(Always, _) | (_, Always) => true`. Combined with `derive_event_keys` emitting an `Always` needle for every snapshot, this caused EVERY entry in the corpus to fire on EVERY snapshot — including LifeEvent-only entries like `van-khan-dong-tho`. Test 2's per-hit honesty check (`A hit with none of those is a matcher bug`) flagged it: `ritual van-khan-dong-tho fired on Vọng 2024-02-24 but has no day-15/holiday/solar-term/always event key`. The plan anticipated this exact failure mode and labelled it a matcher bug.
- **Fix:** Removed the `(_, Always) => true` arm. `Always` haystack still matches any needle (daily-fire entries). `Always` needle now only matches an `Always` haystack (queries for daily entries). Updated the matcher doc-comment to document the asymmetry. Updated the inline `always_sentinel_matches_anything` test to assert the new semantic.
- **Files modified:** `crates/amlich-core/src/rituals/matcher.rs` (matcher arm + doc + inline test).
- **Verification:** `cargo test -p amlich-core --test rituals_integration` — 6/6 pass. `cargo test -p amlich-core` — 597 lib tests + all integration binaries pass; 0 failures.
- **Committed in:** `e0cb5b4` (single Task 1 commit, since the test file IS the falsifier).

---

**Total deviations:** 1 auto-fixed (1 bug).
**Impact on plan:** Required by Test 2 falsifiability — without the fix the integration test cannot pass, and the matcher would silently return the entire corpus on every snapshot in production. Scope strictly within the matcher module; no schema changes; no API surface changes; all upstream inline tests still green.

## Issues Encountered

None beyond the auto-fixed Rule-1 bug above.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- Phase 11 (Văn khấn Module + Lookup APIs) closes with this plan. All 4 waves landed (11-01 fixtures + Hán guard, 11-02 corpus loader, 11-03 matcher + lookup APIs, 11-04 integration tests).
- Phase 11 success criteria observable:
  - SC#1 (Tết snapshot → matching entries) — Test 1 + Test 2 + Test 3.
  - SC#2 (five APIs callable from outside crate) — verified by `use amlich_core::rituals::{all_rituals, find_van_khan_for_snapshot, find_van_khan_for_event, ...}` compiling at the test-binary boundary.
  - SC#3 (closed RitualEventKey 5 variants) — compiler-enforced; matcher arm exhaustiveness preserved by Plan 11-03's `_ => false` collapse rule.
  - SC#4 (LunarDateMatch + leap policy default) — Test 6 + ADR-0001.
  - SC#5 (Hán rejected + NFC round-trip) — `ritual_han_guard.rs` (11-01) + Test 5.
- Phase 12 (Văn khấn Corpus Authoring) ready to start. Phase 13 (Phi Tinh Primitives) shares no code paths and may run concurrently.
- No blockers, no concerns.

## Self-Check: PASSED

- File `crates/amlich-core/tests/rituals_integration.rs` exists on disk.
- File `crates/amlich-core/src/rituals/matcher.rs` modified on disk.
- Commit `e0cb5b4` exists in `git log --oneline`.
- `cargo test -p amlich-core --test rituals_integration` — 6 tests, 0 failures.
- `cargo test -p amlich-core` — full suite, 0 failures.

---
*Phase: 11-van-khan-module-and-lookup-apis*
*Completed: 2026-05-26*
