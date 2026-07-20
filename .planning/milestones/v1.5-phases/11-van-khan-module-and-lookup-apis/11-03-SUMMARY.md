---
phase: 11-van-khan-module-and-lookup-apis
plan: "03"
subsystem: rituals
tags: [matcher, lookup-api, rituals, ritual-event-key, leap-policy, day-snapshot, holiday-id]

# Dependency graph
requires:
  - phase: 11-02
    provides: "corpus.rs OnceLock loader exposing all_rituals() &'static [RitualEntry]; mod corpus; registered in rituals/mod.rs"
  - phase: 10-03
    provides: "Locked RitualEventKey enum (5 variants — HolidayId/LunarDate/SolarTerm/LifeEvent/Always), LeapPolicy enum, LifeEventKind enum, RitualEntry struct (ADR-0001)"
  - phase: 10-02
    provides: "Holiday.id: Option<String> field on data/holidays/lunar-festivals.json entries; get_vietnamese_holidays(year) -> Vec<Holiday>"
provides:
  - "crate::rituals::find_van_khan_for_snapshot(&DaySnapshot) -> Vec<&'static RitualEntry> (RIT-01)"
  - "crate::rituals::find_van_khan_for_event(&RitualEventKey) -> Vec<&'static RitualEntry> (RIT-02)"
  - "crate::rituals::find_van_khan_for_life_event(LifeEventKind) -> Vec<&'static RitualEntry> (RIT-03)"
  - "crate::rituals::get_ritual_by_id(&str) -> Option<&'static RitualEntry> (RIT-04)"
  - "crate::rituals::all_rituals re-export at module root (RIT-05 surface)"
  - "crate::rituals::* pub use schema::* re-exports (RitualEventKey, LeapPolicy, LifeEventKind, RitualEntry, etc.)"
  - "Leap-aware event_key_matches helper (private) with closed 5-variant exhaustive coverage plus _ => false cross-variant collapse"
  - "derive_event_keys helper (private) — snapshot → HolidayId/LunarDate/SolarTerm/Always needles (NO LifeEvent — caller intent only)"
affects: [11-04, 15-dto-integration, 12-corpus-authoring]

# Tech tracking
tech-stack:
  added: []  # zero new dependencies — pure stdlib + existing types
  patterns:
    - "Linear-scan filter over &'static [RitualEntry] returning Vec<&'static RitualEntry> (Vec<reference> not Vec<owned>)"
    - "Symmetric leap-policy reconciliation: Either matches any; otherwise policies must equal"
    - "Snapshot → event-key derivation as caller-intent-free projection: day properties only (research Q4)"
    - "Closed-enum exhaustive match + _ => false collapse arm gated by ADR superseding rule"

key-files:
  created:
    - "crates/amlich-core/src/rituals/matcher.rs (231 lines including 9 inline tests)"
  modified:
    - "crates/amlich-core/src/rituals/mod.rs (added mod matcher; + pub use schema::*; + pub use corpus::all_rituals; + pub use matcher::{four APIs})"

key-decisions:
  - "Closed-enum + _ => false wildcard intentionally preserved over fully-exhaustive cross-variant arms — variant set is locked by ADR-0001; doc-comment on event_key_matches flags the superseding-ADR requirement for any 6th variant"
  - "derive_event_keys does NOT emit LifeEvent needles — life events are caller intent (Động thổ, Cưới, Khai Trương, etc.) not day properties; only find_van_khan_for_life_event wraps them"
  - "Matcher returns Vec<&'static RitualEntry> (reference type) so callers can hold results indefinitely without cloning; cheap since corpus is OnceLock-backed and &'static"
  - "Holiday id needles built via clone (h.id.clone()) — get_vietnamese_holidays returns owned Vec<Holiday>, no &Holiday lifetime held across the loop"
  - "Plain `as u8` casts for lunar month/day — safe per domain invariants (1..=12 / 1..=30); no try_into ceremony needed"

patterns-established:
  - "Same-variant exhaustive coverage + Always-sentinel cross-variant case + single _ => false fallback: a stable shape for closed-enum matchers in this codebase"
  - "Module surface: schema::* re-exported (callers don't need ::schema:: segment); helpers stay private; only named functions cross the mod boundary"
  - "Test 'always_sentinel_matches_anything' uses both direct event_key_matches invocations AND end-to-end find_van_khan_for_event(Always) — validates fixture wiring + matcher logic together"

requirements-completed: [RIT-01, RIT-02, RIT-03, RIT-04, RIT-06, RIT-07]

# Metrics
duration: 2min
completed: 2026-05-26
---

# Phase 11 Plan 03: Văn Khấn Matcher + Public API Surface Summary

**Four ritual-lookup APIs (RIT-01..04) wired to corpus via leap-aware event_key_matches + snapshot-to-needle derivation; rituals module re-exports the full surface for external consumers.**

## Performance

- **Duration:** 2 min
- **Started:** 2026-05-26T16:44:36Z
- **Completed:** 2026-05-26T16:46:46Z
- **Tasks:** 2
- **Files modified:** 2 (1 created, 1 edited)

## Accomplishments

- Shipped `crates/amlich-core/src/rituals/matcher.rs` (231 lines): four public lookup APIs + two private helpers + 9 inline tests.
- Wired `rituals/mod.rs` to register `mod matcher;` and re-export the closed Phase-11 public API surface at `amlich_core::rituals::*`.
- RIT-01 verified end-to-end: `calculate_day_snapshot(10, 2, 2024)` → `find_van_khan_for_snapshot(&snap)` returns the Tết simple-variant entry.
- RIT-07 leap-policy semantics verified: canonical-only entries do NOT fire on leap-month snapshots; `Either` policy reconciles both sides symmetrically.
- 597 lib tests pass (588 previously + 9 new matcher tests, 0 regressions); source_id_guard + ritual_han_guard both green.

## Task Commits

Each task committed atomically:

1. **Task 1: Implement rituals/matcher.rs with 4 lookup APIs + derive_event_keys + leap-aware event_key_matches** — `902879d` (feat)
2. **Task 2: Wire matcher.rs + re-exports in rituals/mod.rs; full crate test pass** — `f3b2d41` (feat)

**Plan metadata:** to be recorded by final docs commit.

## Files Created/Modified

- `crates/amlich-core/src/rituals/matcher.rs` (created) — Four public lookup APIs (`find_van_khan_for_snapshot`, `find_van_khan_for_event`, `find_van_khan_for_life_event`, `get_ritual_by_id`), private helpers (`derive_event_keys`, `event_key_matches`), 9 inline tests covering RIT-01..04 + RIT-06 (Always sentinel) + RIT-07 (leap policy semantics).
- `crates/amlich-core/src/rituals/mod.rs` (modified) — Adds `mod matcher;` declaration and `pub use` re-exports for `schema::*`, `corpus::all_rituals`, and all four matcher APIs. Module doc expanded with full API map and ADR-0001 schema-lock notice.

## Decisions Made

- **Closed-enum + `_ => false` collapse arm preserved** — Fully-exhaustive cross-variant arms would add 12+ false-returning arms with no semantic benefit; the variant set is locked by ADR-0001 and the doc-comment on `event_key_matches` explicitly flags the superseding-ADR requirement should a 6th variant ever land.
- **`derive_event_keys` does NOT emit LifeEvent needles** — Resolved at research Q4: life events are caller intent (Động thổ, Cưới, Khai Trương, etc.), not day properties. Only the explicit `find_van_khan_for_life_event(kind)` path wraps a `LifeEvent` needle.
- **Vec<&'static RitualEntry> return type** — Caller can hold results indefinitely without cloning since corpus is OnceLock-backed. Avoids the API ergonomics tax of `Vec<RitualEntry>` (forced clone) or `impl Iterator<...>` (lifetime gymnastics for callers).
- **Holiday id needles use `h.id.clone()`** — `get_vietnamese_holidays` returns `Vec<Holiday>` (owned); no `&Holiday` reference held across the loop. Clone cost is negligible (≤6 holidays per day, short ids).
- **`as u8` casts for lunar month/day** — Domain invariants guarantee 1..=12 / 1..=30 ranges; no `try_into()` ceremony, matches the project's existing cast discipline at lib.rs callsites.

## Deviations from Plan

None - plan executed exactly as written.

The plan body was unusually prescriptive (full file contents reproduced in the action block), which made execution mechanical: write file, build, test. Both Task 1 (matcher.rs creation) and Task 2 (mod.rs wiring) compiled and tested clean on first attempt. No auto-fix rules triggered.

**Pre-existing warning out of scope:** `unused import: ProvenanceSource` at `crates/amlich-core/src/semantic_graph/views/helpers.rs:111` — landed in commit `a6babaf` (semantic_graph cleanup), pre-dates this plan. Logged here only to confirm it was not caused by this plan.

## Issues Encountered

None. All verification chain steps (cargo build, lib tests, source_id_guard, ritual_han_guard, full crate suite) passed on first execution.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- **11-04 (integration tests) is unblocked** — Public API surface is complete; `amlich_core::rituals::find_van_khan_for_snapshot` and the other three APIs resolve from external test files at `tests/`.
- **Phase 15 (DTO integration) gains a stable consumer surface** — Re-exports cover all schema types + matcher APIs at the module root, no `::schema::` segment needed.
- **Phase 12 (corpus authoring) gains a tested matcher** — Authors writing new fixtures in `data/rituals/` can validate their `event_keys[]` shape against existing matcher tests and the leap-policy semantics.
- **No blockers** — Phase 11 wave 4 is the only remaining work before Phase 11 closes; Phase 13 (Phi Tinh primitives) remains independently executable.

---
*Phase: 11-van-khan-module-and-lookup-apis*
*Completed: 2026-05-26*

## Self-Check: PASSED

Verified:
- FOUND: crates/amlich-core/src/rituals/matcher.rs
- FOUND: crates/amlich-core/src/rituals/mod.rs (contains `mod matcher;` and `pub use matcher::`)
- FOUND commit: 902879d (Task 1 — matcher.rs)
- FOUND commit: f3b2d41 (Task 2 — mod.rs wiring)
- Tests: 597 lib tests pass (9 new matcher tests); source_id_guard + ritual_han_guard green.
