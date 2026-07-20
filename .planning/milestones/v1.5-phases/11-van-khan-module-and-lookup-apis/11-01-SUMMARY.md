---
phase: 11-van-khan-module-and-lookup-apis
plan: "01"
subsystem: infra
tags: [rituals, fixtures, nfc, unicode-normalization, ci-guard, han-character, json-corpus]

# Dependency graph
requires:
  - phase: 10-foundation-schema-lock-and-source-id-registration
    provides: "ADR-0001 RitualEntry schema (rituals/schema.rs); SOURCE_VN_FOLK_RITUAL constant; deny_unknown_fields lock on RitualFile envelope"
provides:
  - "unicode-normalization 0.1.25 declared in amlich-core/Cargo.toml [dependencies]"
  - "crates/amlich-core/data/rituals/fixtures.json with 6 stub RitualEntry entries wrapped in {$schema_version: rituals-v1, entries: [...]} envelope"
  - "crates/amlich-core/tests/ritual_han_guard.rs — CI integration test rejecting any CJK Unified Ideograph in data/rituals/*.json"
affects:
  - "11-02-corpus-loader: can now `include_str!(\"../data/rituals/fixtures.json\")` and define RitualFile envelope deserialization"
  - "11-03-matcher: matcher unit tests can rely on 6 stub entries covering HolidayId, LunarDate (Mùng-1 + Mùng-15), SolarTerm, LifeEvent, Always, explicit LeapPolicy paths"
  - "11-04-integration-tests: 6 ritual_ids (van-khan-tet-don-gian, van-khan-ram-thang-gieng, van-khan-thanh-minh, van-khan-dong-tho, van-khan-gia-tien-hang-ngay, van-khan-doan-ngo) referenced by name"

# Tech tracking
tech-stack:
  added:
    - "unicode-normalization 0.1.25 — NFC normalization library for ritual corpus (RIT-08 NFC-at-load preparation)"
  patterns:
    - "Inline char-range Hán detection (matches!) — no external regex/unicode-blocks dep, mirrors source_id_guard.rs file-scan structure"
    - "RitualFile JSON envelope: {$schema_version: \"rituals-v1\", entries: Vec<RitualEntry>} — discriminated by schema_version literal for future migrations"
    - "Corpus CI guards live in tests/ as integration tests, scan data/rituals/ via CARGO_MANIFEST_DIR — no source-code coupling to loader module"
    - "No-op-on-missing-dir pattern for wave-1 ordering safety (tests file can land before fixtures.json without breaking CI)"

key-files:
  created:
    - "crates/amlich-core/data/rituals/fixtures.json (131 lines, 6 RitualEntry entries)"
    - "crates/amlich-core/tests/ritual_han_guard.rs (60 lines, integration test)"
  modified:
    - "crates/amlich-core/Cargo.toml (+1 line: unicode-normalization = \"0.1.25\")"

key-decisions:
  - "Inline char-range Hán detection — chose `matches!(c, '\\u{4E00}'..='\\u{9FFF}' | ...)` over external crates per 11-RESEARCH.md §Don't Hand-Roll; covers 4 CJK blocks (base + Ext-A + Ext-B + Compatibility) with zero new transitive deps"
  - "No manifest.json file at this wave — research Q2 deferred manifest until Phase 12 corpus authoring; single fixtures.json suffices for stub corpus"
  - "Hán guard no-ops on missing data/rituals/ dir — Phase 11 plans land in order but wave-1 ordering safety lets the test file commit before fixtures.json (it can't, in this plan, but the pattern is preserved for Phase 12 file additions)"
  - "TDD validated by ephemeral RED check — injected `{\"han_test\":\"中文\"}` fixture; guard correctly panicked with `2 Hán code points found` diagnostic before fixture removed for GREEN"
  - "unicode-normalization dep landed before loader (11-02) — Cargo accepts unused deps; declaring early prevents transient build break in wave-2 parallel execution"

patterns-established:
  - "Hán-character CI guard pattern: file-scan tests/ritual_han_guard.rs reads CARGO_MANIFEST_DIR/data/rituals/*.json, counts Hán code points via inline char-range, panics with per-file diagnostic listing"
  - "RitualEntry fixture authoring pattern: NFC-normalized Vietnamese, source_id=\"vn-folk-ritual\" everywhere, ADR-0001 conformant (deny_unknown_fields tested in schema.rs unit tests), Optional fields (body_en, notes) omitted entirely from JSON (skip_serializing_if)"
  - "Wave-1 unblock pattern: ship the dep + fixtures + guard in one plan so downstream waves can include_str! and run unit tests against real data immediately"

requirements-completed:
  - RIT-08

# Metrics
duration: 18min
completed: 2026-05-26
---

# Phase 11 Plan 01: Wave-1 Ritual Corpus Foundation Summary

**Unblocks Phase 11 waves 2-4 by landing unicode-normalization 0.1.25, 6 NFC-normalized stub ritual entries in data/rituals/fixtures.json, and a Hán-character CI guard mirroring source_id_guard.rs.**

## Performance

- **Duration:** ~18 min
- **Started:** 2026-05-26T16:16:00Z
- **Completed:** 2026-05-26T16:34:42Z
- **Tasks:** 3
- **Files modified:** 3 (1 modified, 2 created)

## Accomplishments

- Added single new dependency `unicode-normalization = "0.1.25"` to amlich-core/Cargo.toml; resolved cleanly via `cargo metadata` without disturbing workspace-level dependencies
- Authored data/rituals/fixtures.json with exactly 6 stub RitualEntry entries that collectively exercise every matcher path Plan 03 will implement: HolidayId (`tet-nguyen-dan`, `tet-nguyen-tieu`, `tet-doan-ngo`), LunarDate Mùng-1 and Mùng-15 (Plan 04 Vọng/Sóc falsifiable target), SolarTerm (`Thanh Minh` — only-key entry, no holiday_id available per Phase 10 plan 10-02), LifeEvent (`dong_tho`), Always sentinel (`van-khan-gia-tien-hang-ngay`), and explicit LeapPolicy::CanonicalMonthOnly (`van-khan-doan-ngo`)
- Created tests/ritual_han_guard.rs integration test: file-scan over CARGO_MANIFEST_DIR/data/rituals/*.json, inline `is_han_char` matching across 4 CJK blocks (U+4E00..U+9FFF base, U+3400..U+4DBF Ext-A, U+20000..U+2A6DF Ext-B, U+F900..U+FAFF Compatibility), panics with per-file diagnostic if any Hán code point found
- TDD validation: temporarily injected `{"han_test":"中文"}` into data/rituals/_red_check.json, confirmed guard failed with `2 Hán code points found`, then removed and confirmed GREEN pass against the real fixtures (`test ritual_corpus_rejects_han_characters ... ok`)
- Full crate regression: `cargo test -p amlich-core` — every test result line shows `0 failed`; source_id_guard still passes (no bare literals leaked into src/)

## Task Commits

Each task was committed atomically:

1. **Task 1: Add unicode-normalization dependency to amlich-core/Cargo.toml** — `fd917a3` (chore)
2. **Task 2: Author data/rituals/fixtures.json with 6 stub entries** — `0f80621` (feat)
3. **Task 3: Add Hán-character CI guard at tests/ritual_han_guard.rs** — `0356509` (test)

**Plan metadata:** _(this commit)_

## Files Created/Modified

- `crates/amlich-core/Cargo.toml` — added `unicode-normalization = "0.1.25"` line under `[dependencies]`
- `crates/amlich-core/data/rituals/fixtures.json` — 6-entry stub corpus with RitualFile envelope; covers all matcher paths Plan 03 and verify gates Plan 04 reference by name
- `crates/amlich-core/tests/ritual_han_guard.rs` — integration test enforcing 0 Hán code points across data/rituals/*.json; no-ops on missing dir

## Decisions Made

- **Inline char-range Hán detection over external crate** — `matches!(c, ...)` with 4 explicit CJK ranges; zero new transitive deps; 60-line test file
- **No manifest.json yet** — per 11-RESEARCH.md Q2, manifest is Phase 12 corpus-authoring concern; fixtures.json alone is sufficient for stub corpus consumption
- **Hán guard no-ops on missing dir** — defensive ordering pattern; not strictly needed in this plan (fixtures.json committed in same wave) but preserved for downstream additions
- **unicode-normalization 0.1.25 (not 0.1.x latest)** — research-verified pinned version; NFC APIs (`nfc()`, `is_nfc()`, `is_nfc_quick()`) stable since 2014, Rust 1.36+ MSRV
- **TDD validated via ephemeral RED check** — instead of writing a Hán-laden test fixture into the corpus and then removing it (which would dirty git history), used a single throwaway _red_check.json file deleted before commit

## Deviations from Plan

None — plan executed exactly as written. All three task verification gates passed on first attempt; no Rule 1-4 deviations triggered.

---

**Total deviations:** 0
**Impact on plan:** Plan was thoroughly specified; all char ranges, JSON entries, and test contents were prescriptive; executor only had to type and verify.

## Issues Encountered

None. Plan was self-contained; no `src/` files touched (per success criteria); no transient build breaks.

## User Setup Required

None — no external service configuration required.

## Next Phase Readiness

- **11-02 (corpus loader) unblocked:** can now `include_str!("../data/rituals/fixtures.json")`; envelope shape locked at `$schema_version=rituals-v1`; unicode-normalization dep available for NFC-at-load enforcement (RIT-08 implementation portion)
- **11-03 (matcher) unblocked:** 6 stub entries cover every matcher path; unit tests can pattern-match on real ritual_ids
- **11-04 (integration tests) unblocked:** all six required ritual_ids (`van-khan-tet-don-gian`, `van-khan-ram-thang-gieng`, `van-khan-thanh-minh`, `van-khan-dong-tho`, `van-khan-gia-tien-hang-ngay`, `van-khan-doan-ngo`) present in fixtures.json by name
- **No blockers.** Waves 2-4 may execute sequentially (or in any topological order consistent with their own depends_on graphs).

## Self-Check: PASSED

- `crates/amlich-core/Cargo.toml` modified — FOUND (`unicode-normalization = "0.1.25"` line present)
- `crates/amlich-core/data/rituals/fixtures.json` created — FOUND (131 lines, 6 entries, NFC, schema_version=rituals-v1)
- `crates/amlich-core/tests/ritual_han_guard.rs` created — FOUND (60 lines, test passes)
- Commit `fd917a3` (Task 1: chore) — FOUND
- Commit `0f80621` (Task 2: feat) — FOUND
- Commit `0356509` (Task 3: test) — FOUND

---
*Phase: 11-van-khan-module-and-lookup-apis*
*Completed: 2026-05-26*
