---
phase: 11-van-khan-module-and-lookup-apis
plan: "02"
subsystem: rituals
tags: [rituals, corpus-loader, once-lock, include-str, nfc, source-id-discipline, tdd]

# Dependency graph
requires:
  - phase: 11-van-khan-module-and-lookup-apis
    plan: "01"
    provides: "data/rituals/fixtures.json (6 NFC entries, $schema_version=rituals-v1); unicode-normalization 0.1.25 in Cargo.toml; Hán-character CI guard"
  - phase: 10-foundation-schema-lock-and-source-id-registration
    provides: "ADR-0001 RitualEntry schema (rituals::schema); SOURCE_VN_FOLK_RITUAL constant in crate::sources"
provides:
  - "crate::rituals::corpus::all_rituals() -> &'static [RitualEntry] — OnceLock-cached, NFC-normalized, source_id-validated"
  - "NFC-at-load invariant (RIT-08): every text field on every RitualEntry returned by all_rituals() passes is_nfc()"
  - "source_id discipline-at-load invariant: panics at first access if any entry has source_id != SOURCE_VN_FOLK_RITUAL"
  - "schema_version assertion at load: panics if $schema_version != \"rituals-v1\" (ADR-0001 enforcement)"
  - "mod corpus; registered as private submodule in crates/amlich-core/src/rituals/mod.rs"
affects:
  - "11-03 (matcher): can consume all_rituals() via `pub use corpus::all_rituals` re-export (additive line, no conflict)"
  - "11-04 (integration tests): can assert NFC + source_id invariants from outside the module by calling crate::rituals::corpus::all_rituals (after 11-03 re-export)"

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "OnceLock<Vec<T>> + include_str! + .expect() panic-on-init: mirrors holiday_data.rs:117-138 and golden_loader.rs; canonical lazy-init triad for compile-embedded JSON corpora in this crate"
    - "normalize_and_validate(entry) -> RitualEntry: per-entry assert_eq! on source_id (against constant, not literal) + recursive NFC walk over every text field; private helper consumed only by get_or_init closure"
    - "nfc(&str) -> String short-circuit via is_nfc(): canonical input bypasses the .nfc().collect() pipeline for the common case (fixtures pre-normalized by plan 11-01)"
    - "Schema-version-pin in loader: $schema_version literal compared against EXPECTED_SCHEMA_VERSION const; future v2 corpus requires superseding ADR + const bump (ADR-0001 discipline)"
    - "Private submodule + future-additive re-export: `mod corpus;` (no pub); plan 11-03 will append `pub use corpus::all_rituals;` to mod.rs — different line, zero conflict"

key-files:
  created:
    - "crates/amlich-core/src/rituals/corpus.rs (172 lines: 110 implementation, 62 inline tests)"
  modified:
    - "crates/amlich-core/src/rituals/mod.rs (+1 line: `mod corpus;` below existing `pub mod schema;`)"

key-decisions:
  - "TDD round-trip: RED commit shipped corpus.rs with todo!() stub + 5 failing tests + the mod.rs registration line (registration required for tests to compile); GREEN commit replaced only corpus.rs body with the full loader, leaving mod.rs intact. Two-commit atomicity preserved despite Task 2's mod.rs edit landing in the RED commit."
  - "RitualFile envelope kept private — `struct RitualFile { schema_version, entries }` is an implementation detail of the loader; only `all_rituals()` is `pub`. Helpers `normalize_and_validate`, `nfc`, and the `RITUAL_FIXTURES_JSON`/`EXPECTED_SCHEMA_VERSION` consts stay module-private."
  - "include_str! path verified: `../../data/rituals/fixtures.json` resolves correctly from `src/rituals/corpus.rs` (2 levels up to crate root, then into data/rituals/) — single attempt, no path adjustment needed."
  - "source_id assert compares against constant: `assert_eq!(entry.source_id, SOURCE_VN_FOLK_RITUAL, ...)` — bare literal would have tripped source_id_guard.rs CI. Confirmed green post-merge."
  - "Dead-code warnings expected: `all_rituals`, `normalize_and_validate`, `nfc` flagged unused at lib build because no caller exists yet outside the test module. Resolves automatically when plan 11-03 lands the matcher and the `pub use` re-export."

patterns-established:
  - "Ritual-corpus OnceLock pattern: include_str! the JSON, deserialize into a private File-envelope struct, assert $schema_version, map .into_iter() through normalize_and_validate, collect into Vec<T>, OnceLock-cache. Reusable template for any future tier-0 corpus (calendars, deity-day registries, etc.)."
  - "NFC-at-load helper template: nfc(&str) -> String with is_nfc() short-circuit + per-Option<String>/per-Vec<_> walk in normalize_and_validate. Mechanically extensible to any new String field via additional if-let branches."

requirements-completed:
  - RIT-05
  - RIT-08

# Metrics
duration: 3min
completed: 2026-05-26
---

# Phase 11 Plan 02: Ritual Corpus Loader (OnceLock + NFC + source_id discipline) Summary

**Lands `crate::rituals::corpus::all_rituals()` — the OnceLock-backed, NFC-normalized, source_id-validated entry point for plan 11-03's matcher. Completes RIT-05 and the NFC-at-load half of RIT-08.**

## Performance

- **Duration:** ~3 min
- **Started:** 2026-05-26T16:38:15Z
- **Completed:** 2026-05-26T16:40:42Z
- **Tasks:** 2 (RED + GREEN, single TDD round-trip)
- **Files modified:** 2 (1 created, 1 modified)

## Accomplishments

- Created `crates/amlich-core/src/rituals/corpus.rs` (172 lines): private `RitualFile` envelope (`$schema_version` + `entries: Vec<RitualEntry>`), `OnceLock<Vec<RitualEntry>>` static, `pub fn all_rituals() -> &'static [RitualEntry]` get-or-init entry point, `normalize_and_validate(entry)` per-entry source_id assert + recursive NFC walk, `nfc(&str)` short-circuit helper
- Embedded `data/rituals/fixtures.json` at compile time via `include_str!("../../data/rituals/fixtures.json")` — path verified resolving from `src/rituals/corpus.rs` upward to crate root, then down into `data/rituals/`; single-attempt success
- Enforced ADR-0001 schema lock at load via `assert_eq!(file.schema_version, "rituals-v1", ...)` — any future corpus version bump requires both a superseding ADR and a const change in the loader
- Enforced RIT-08 NFC invariant at load: every text field (title_vi, title_en, body_en, invocation_text_vi, Offering.name_vi/name_en/quantity/notes, PreparationStep.description_vi/description_en, every notes[i]) passes through `nfc()` which short-circuits via `is_nfc()` for already-canonical input
- Enforced source_id discipline at load: `assert_eq!(entry.source_id, SOURCE_VN_FOLK_RITUAL, ...)` against the constant (not the bare literal) — source_id_guard.rs CI test remains green post-merge
- Registered `mod corpus;` (private) below existing `pub mod schema;` in `rituals/mod.rs` — plan 11-03 will additively append `pub use corpus::all_rituals;` on a different line
- TDD round-trip verified: RED commit (e5c0102) had 5 failing tests via `todo!()` stub; GREEN commit (3f7e2ed) flipped all 5 to passing in a single pass
- Full crate regression check: `cargo test -p amlich-core` — 588 lib tests pass (was 583 + 5 new), zero failures, zero new test infrastructure required; source_id_guard and ritual_han_guard integration tests still green

## Task Commits

Each TDD phase was committed atomically:

1. **RED phase: `test(11-02): add failing tests for ritual corpus loader`** — `e5c0102`
   - Created corpus.rs with `todo!()` stub + 5 inline tests
   - Registered `mod corpus;` in rituals/mod.rs (needed for tests to compile)
   - Confirmed 5 tests fail with `not yet implemented` panic
2. **GREEN phase: `feat(11-02): implement OnceLock ritual corpus loader`** — `3f7e2ed`
   - Replaced corpus.rs body with full include_str! + OnceLock + normalize_and_validate + nfc helper
   - All 5 inline tests pass; full crate suite green (588 lib tests)

**Plan metadata commit:** _(forthcoming, includes this SUMMARY.md + STATE.md + ROADMAP.md + REQUIREMENTS.md updates)_

## Files Created/Modified

- **Created** `crates/amlich-core/src/rituals/corpus.rs` (172 lines)
  - 110 lines of implementation (RitualFile envelope, OnceLock static, all_rituals, normalize_and_validate, nfc)
  - 62 lines of inline tests (≥5 entries, source_id discipline, NFC invariant, OnceLock idempotency, known-id presence)
- **Modified** `crates/amlich-core/src/rituals/mod.rs` (+1 line: `mod corpus;` below `pub mod schema;`)

## Decisions Made

- **TDD round-trip in 2 commits** — RED commit included Task 1's failing test-only corpus.rs **plus** Task 2's `mod corpus;` registration line, because the tests cannot compile without registration. GREEN commit then narrowed to the implementation swap. The plan's nominal 2-task structure was preserved as RED-then-GREEN.
- **RitualFile envelope kept private** — `struct RitualFile { schema_version, entries }` is loader-internal; only `all_rituals()` crosses the module boundary. Helpers (`normalize_and_validate`, `nfc`, the JSON const, the version const, the OnceLock) all stay module-private per plan constraint #5.
- **Constant-only source_id comparison** — `assert_eq!(entry.source_id, SOURCE_VN_FOLK_RITUAL, ...)` imports `use crate::sources::SOURCE_VN_FOLK_RITUAL;` and compares against the symbol. Zero bare `"vn-folk-ritual"` literals in corpus.rs — verified via `grep -E '"vn-folk-ritual"'` returning empty, and via `cargo test --test source_id_guard` remaining green.
- **NFC short-circuit via `is_nfc()`** — `nfc(s) { if is_nfc(s) { s.to_string() } else { s.nfc().collect() } }` — saves the `.nfc().collect::<String>()` decomposition pipeline for the common case where fixtures.json (already NFC-validated by plan 11-01) is being loaded.
- **Dead-code warnings deferred to plan 11-03** — `all_rituals`, `normalize_and_validate`, `nfc` are flagged unused at `cargo build -p amlich-core` because no production caller exists yet (corpus is private and not re-exported). Tests use them, build succeeds, and 11-03's `pub use corpus::all_rituals;` resolves the warnings without re-touching this file.

## Deviations from Plan

None — plan executed exactly as written. The plan's prescriptive code block was typed verbatim; all 5 inline tests passed on first GREEN attempt; the `include_str!` path resolved on the first try (no `../../../` adjustment needed); the `unicode-normalization` import resolved cleanly (Cargo's hyphen↔underscore translation worked as expected); the full crate test suite (588 tests) passed with zero regressions.

---

**Total deviations:** 0
**Auto-fix attempts:** 0
**Impact on plan:** Plan was thoroughly specified down to the test-name level; executor only had to apply the prescribed RED-then-GREEN sequencing and verify gates.

## Issues Encountered

None. No authentication gates (this is a pure-Rust local-build plan with no external services). No build breaks. No transient test failures.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- **11-03 (matcher) unblocked:** `corpus::all_rituals()` is the canonical input source; plan 11-03 will append `pub use corpus::all_rituals;` to `rituals/mod.rs` (additive line, no conflict with the `mod corpus;` line landed here) and implement the lookup APIs against the returned `&'static [RitualEntry]`
- **11-04 (integration tests) unblocked:** once 11-03 re-exports `all_rituals`, integration tests can assert NFC + source_id invariants from outside the module (the inline tests in corpus.rs already verify these from inside)
- **No blockers.** All Wave 2 success criteria green; Wave 3 (matcher) may begin immediately.

## Self-Check: PASSED

- `crates/amlich-core/src/rituals/corpus.rs` created — FOUND (172 lines)
- `crates/amlich-core/src/rituals/mod.rs` modified — FOUND (`mod corpus;` line present below `pub mod schema;`)
- `pub fn all_rituals` present in corpus.rs — FOUND
- `SOURCE_VN_FOLK_RITUAL` referenced (no bare literal) in corpus.rs — FOUND (constant), no bare literal
- `cargo test -p amlich-core --lib rituals::corpus` — 5 passed, 0 failed
- `cargo test -p amlich-core --test source_id_guard` — 1 passed, 0 failed (green)
- `cargo test -p amlich-core --test ritual_han_guard` — 1 passed, 0 failed (green)
- `cargo test -p amlich-core` — 0 failed across all suites (588 lib + integration tests)
- Commit `e5c0102` (RED: test) — FOUND
- Commit `3f7e2ed` (GREEN: feat) — FOUND

---
*Phase: 11-van-khan-module-and-lookup-apis*
*Completed: 2026-05-26*
