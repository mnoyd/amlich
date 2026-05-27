---
phase: 14-phi-tinh-81-cell-aspects-safety-hints
plan: 02
subsystem: almanac
tags: [fengshui, phi-tinh, flying-stars, safety, ngu-hanh, serde, json, onceLock]

# Dependency graph
requires:
  - phase: 14-phi-tinh-81-cell-aspects-safety-hints
    plan: 01
    provides: FsCitation struct in aspects.rs (intra-pillar reuse for RemedyHint)
  - phase: 10-foundation-schema-lock-adrs-source-id-registration
    provides: SOURCE_HUYEN_KHONG constant, source_id_guard test

provides:
  - is_danger_palace(star) predicate — true exactly for NguHoang(5) and NhiHac(2)
  - RemedyHint struct (element, hint_text_vi, source_id, original_citation) with serde support
  - element_hint_for_palace(star) — Option<RemedyHint> with classical Ngu Hanh mitigation
  - flying_stars_safety.json — 4 inauspicious star rows (2, 3, 5, 7) with Vietnamese classical hints
  - safety module registered in fengshui/mod.rs with full re-exports

affects:
  - 14-03 (no-product-names test will validate hint_text_vi corpus)
  - 15-semantic-graph-wiring (DTO integration may expose RemedyHint advisory surface)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - OnceLock+include_str! corpus loader with per-row validation (star range/uniqueness, SOURCE_HUYEN_KHONG, allowed elements, non-empty fields)
    - Free function predicate (is_danger_palace) — NOT a method on the frozen FlyingStar enum in types.rs
    - Internal SafetyHintRow deserialization struct keeping public RemedyHint free of star field
    - Intra-pillar FsCitation reuse from aspects.rs (not cross-pillar rituals coupling)

key-files:
  created:
    - crates/amlich-core/src/almanac/fengshui/safety.rs
    - crates/amlich-core/data/almanac/flying_stars_safety.json
  modified:
    - crates/amlich-core/src/almanac/fengshui/mod.rs

key-decisions:
  - "is_danger_palace is a FREE function (not a method on FlyingStar) — types.rs is FROZEN per plan PITFALLS Pitfall 1; no edits to types.rs"
  - "RemedyHint reuses FsCitation from aspects.rs — both are within the fengshui module (intra-pillar); this is NOT the rituals cross-pillar coupling PITFALLS Pitfall 4 forbids"
  - "Internal SafetyHintRow deserialization struct maps star+fields into RemedyHint — public type stays clean without star field"
  - "4 inauspicious star rows: 2 (kim), 3 (hoa), 5 (kim), 7 (thuy) — classical Ngu Hanh drain-weaken logic; auspicious stars 1/4/6/8/9 have no row so element_hint_for_palace returns None"
  - "No bare huyen-khong literal in safety.rs — all source_id comparisons use SOURCE_HUYEN_KHONG constant; JSON data values allowed per plan spec"

patterns-established:
  - "Free function predicate over frozen enum: use matches!(star, FlyingStar::X | FlyingStar::Y) not methods on types.rs"
  - "Internal row struct pattern: SafetyHintRow{star,fields} -> map to public RemedyHint without star field in public type"

requirements-completed: [FS-14, FS-15]

# Metrics
duration: 2min
completed: 2026-05-28
---

# Phase 14 Plan 02: Safety Hints Summary

**Danger-palace predicate (is_danger_palace) and Ngu Hanh element-hint lookup (element_hint_for_palace) with 4-star classical mitigation corpus and no product names**

## Performance

- **Duration:** 2 min
- **Started:** 2026-05-27T18:33:14Z
- **Completed:** 2026-05-27T18:36:08Z
- **Tasks:** 2 (Task 1 TDD RED + Task 2 TDD GREEN)
- **Files modified:** 3

## Accomplishments

- Implemented `is_danger_palace(star) -> bool` as a free function using `matches!(star, FlyingStar::NguHoang | FlyingStar::NhiHac)` — types.rs untouched (FROZEN)
- Declared `RemedyHint` struct reusing `FsCitation` from `aspects.rs` (intra-pillar, not cross-pillar)
- Authored `flying_stars_safety.json` with 4 rows for inauspicious stars 2/3/5/7; classical Ngu Hanh drain logic; Vietnamese hint text with no product names
- All 6 safety unit tests pass; source_id_guard CI green; 691 lib tests total, zero failures

## Task Commits

Each task was committed atomically:

1. **Task 1: is_danger_palace + RemedyHint + element_hint_for_palace loader (TDD RED)** - `f6fefc7` (test)
2. **Task 2: Author flying_stars_safety.json corpus (TDD GREEN)** - `1677a66` (feat)

## Files Created/Modified

- `crates/amlich-core/src/almanac/fengshui/safety.rs` - is_danger_palace predicate; RemedyHint type; SafetyHintRow internal struct; OnceLock corpus loader with full validation; element_hint_for_palace lookup; 6 unit tests
- `crates/amlich-core/data/almanac/flying_stars_safety.json` - 4 inauspicious star rows (schema-v1, source: Tham Thi Huyen Khong Hoc)
- `crates/amlich-core/src/almanac/fengshui/mod.rs` - Added `pub mod safety;` and re-exports (is_danger_palace, element_hint_for_palace, RemedyHint)

## Decisions Made

- **Free function predicate** — `is_danger_palace` declared as a free function in `safety.rs`, not as a method on `FlyingStar`. `types.rs` is FROZEN per PITFALLS Pitfall 1; adding methods there would require a superseding ADR.
- **Intra-pillar FsCitation reuse** — `RemedyHint` imports `FsCitation` from `crate::almanac::fengshui::aspects`. Both modules are within the `fengshui` pillar; this is explicitly NOT the cross-pillar rituals coupling forbidden by PITFALLS Pitfall 4.
- **Internal row struct** — `SafetyHintRow { star: u8, element, hint_text_vi, source_id, original_citation }` (private, Deserialize-only) keeps `star` off the public `RemedyHint` type. Lookup is keyed by function argument, not stored in the hint.
- **4-row corpus** — Stars 2 (earth->kim), 3 (wood->hoa), 5 (earth->kim), 7 (metal->thuy). Classical logic: drain/weaken by generating element. Stars 1/4/6/8/9 auspicious; no rows; element_hint_for_palace returns None.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- FS-14: `is_danger_palace` true exactly for NguHoang(5) and NhiHac(2) — complete.
- FS-15: `element_hint_for_palace` returns `Option<RemedyHint>` with Ngu Hanh element + classical citation; None for auspicious stars; no product names in corpus — complete.
- `types.rs` untouched; no import from `crate::rituals` or `crate::interaction` — compliant.
- Phase 14-03 (no-product-names validation tests) can now test hint_text_vi fields from flying_stars_safety.json.

## Self-Check: PASSED

All created files exist on disk and all task commits are present in git log.

---
*Phase: 14-phi-tinh-81-cell-aspects-safety-hints*
*Completed: 2026-05-28*
