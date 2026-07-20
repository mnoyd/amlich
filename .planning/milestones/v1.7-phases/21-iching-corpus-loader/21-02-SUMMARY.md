---
phase: 21-iching-corpus-loader
plan: 02
subsystem: database
tags: [iching, kinh-dich, corpus-loader, oncereentrant, hao-tu-invariant, nfc-normalization, adr-0005, wasm-safe, black-box-tests]

# Dependency graph
requires:
  - phase: 21-iching-corpus-loader
    provides: 64-entry hexagrams.json corpus (Plan 21-01 — the DATA half of ICH-01; envelope {$schema_version: iching-v1}) + 64-row provenance_audit.md ledger
  - phase: 20-foundation-schema-lock-source-ids-adrs-ontology
    provides: Locked HexagramEntry schema (ADR-0005) + three CRIT-3-isolating newtypes (KingWenHexagram, HauThienTrigram, TienThienTrigram) + bijective COMPOSITION_TABLE
  - phase: 11-van-khan-rituals-corpora
    provides: OnceLock + include_str! + FileEnvelope + nfc() loader pattern (the template mirrored exactly by rituals/corpus.rs)
provides:
  - "OnceLock-cached IChing corpus loader: `all_hexagrams() -> &'static [HexagramEntry]` + `get_hexagram(KingWenHexagram) -> Option<&'static HexagramEntry>`"
  - "ADR-0005 §2 `hao_tu` length invariant enforcement at load (7 for #1/#2, 6 for #3..=64 — panic on violation, fail-fast)"
  - "RIT-08 NFC normalization of every Vietnamese text field at load (vi_name, thoai_tu, cat_hung, every hao_tu line + reserved *_en if Some)"
  - "$schema_version == 'iching-v1' load-time assertion (ADR enforcement — panics on mismatch)"
  - "Black-box integration test suite (8 tests) verifying ICH-01 success criteria 1-4 from the external crate path"
affects:
  - 22-mai-hoa-casting-bien-que-the-dung (Mai Hoa cast -> King Wen index -> corpus lookup via get_hexagram)
  - 24-iching-evaluator-semantic-graph-wiring-dto (semantic-graph Hexagram nodes sourced from this corpus)
  - 25-e2e-validation-golden-cross-source (golden cross-source verification consuming the corpus)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "OnceLock + include_str! + serde envelope corpus loader (3rd corpus milestone mirroring rituals/corpus.rs — pattern now applied across v1.5/v1.6/v1.7)"
    - "Loader-enforced length invariant (hao_tu 6-vs-7 rule) that serde cannot encode (Vec<String> has no length-dependent-on-other-field derive)"
    - "Serde-NAME identity cross-check between two distinct newtypes sharing rename_all=snake_case (CRIT-3-safe: compare serialized names, never convert types)"
    - "Integration-test grep guard for WASM-safety (no std::fs::, no Utc::now) mirroring v1.6 fengshui_crit3_isolation.rs"

key-files:
  created:
    - crates/amlich-core/src/iching/corpus.rs
    - crates/amlich-core/tests/iching_corpus_integration.rs
  modified:
    - crates/amlich-core/src/iching/mod.rs

key-decisions:
  - "Loader mirrors rituals/corpus.rs exactly (OnceLock + include_str! + HexagramFile envelope + nfc() helper) — the pattern is now proven across three corpus milestones (rituals v1.5, golden v1.6, iching v1.7)"
  - "hao_tu length invariant is enforced at LOAD (in normalize_and_validate) via assert_eq!, NOT via serde. Panic on violation because the corpus is compile-embedded — a parse failure is a build-time bug, not a runtime condition"
  - "get_hexagram uses a 64-entry linear scan (mirrors compose()'s scan decision in schema.rs:261-269) — premature to pre-compute a reverse lookup map for 64 entries accessed rarely"
  - "Trigram identity cross-check in test #3 compares SERDE NAMES (e.g. \\\"kien\\\"), NOT discriminants — CRIT-3 isolation preserved because we never convert between TienThienTrigram and HauThienTrigram"
  - "WASM-safety grep guard anchored on `std::fs::` / `use std::fs;` / `Utc::now` (actual usage patterns), NOT the bare strings — bare strings false-positive on doc comments mentioning the rule"

patterns-established:
  - "Loader-enforced structural invariant (not serde): use when the constraint depends on another field's value (e.g. Vec length depends on enum variant). Panic = fail-fast; corpus is compile-embedded so failure is build-time."
  - "Grep guards for WASM-safety (or any 'forbidden API' rule) must anchor on actual USAGE patterns (`std::fs::`, `use std::fs;`), not bare substrings — doc comments legitimately mention the rule."

requirements-completed: [ICH-01]

# Metrics
duration: 9 min
completed: 2026-07-16
---

# Phase 21 Plan 02: IChing Corpus Loader Summary

**OnceLock-backed IChing corpus loader + get_hexagram/all_hexagrams lookup API mirroring the v1.5 rituals/corpus.rs pattern, with ADR-0005 §2 hao_tu length-invariant enforcement at load, RIT-08 NFC normalization on every text field, and 8 black-box integration tests closing ICH-01 across all 4 success criteria**

## Performance

- **Duration:** 9 min (578 s)
- **Started:** 2026-07-16T02:11:45Z
- **Completed:** 2026-07-16T02:21:23Z
- **Tasks:** 2 (Task 1 = TDD red→green; Task 2 = `type="auto"`)
- **Files created:** 2 (corpus.rs, iching_corpus_integration.rs)
- **Files modified:** 1 (iching/mod.rs)

## Accomplishments

- **`crates/amlich-core/src/iching/corpus.rs` (233 lines)** implements the OnceLock-cached corpus loader mirroring `rituals/corpus.rs` exactly: `HEXAGRAMS_JSON` constant via `include_str!("../../data/iching/hexagrams.json")`, `EXPECTED_SCHEMA_VERSION = "iching-v1"` asserted at load (panics on mismatch — ADR enforcement), `HexagramFile` envelope struct with `#[serde(rename = "$schema_version")]`, `all_hexagrams() -> &'static [HexagramEntry]` (OnceLock-init parse + schema-version assertion + per-entry normalize_and_validate), `get_hexagram(KingWenHexagram) -> Option<&'static HexagramEntry>` (64-entry linear scan), `normalize_and_validate()` (NFC + hao_tu invariant), and the `nfc()` helper.
- **ADR-0005 §2 `hao_tu` length invariant enforced at load**: `normalize_and_validate()` asserts `hao_tu.len() == 7` for King Wen #1/#2 (dụng cửu / dụng lục seventh line) and `== 6` for #3..=64. Panic on violation — corpus is compile-embedded so a parse failure is a build-time bug, not a runtime condition. (Cannot be a serde constraint: `Vec<String>` has no length-dependent-on-other-field derive.)
- **RIT-08 NFC normalization** applied to every Vietnamese text field at load: `vi_name`, `thoai_tu`, `cat_hung`, every line of `hao_tu`, plus the reserved `*_en` Option fields if `Some` (None in v1.7 but normalized for forward-safety). `nfc()` helper is byte-identical to `rituals/corpus.rs:163-169`.
- **`crates/amlich-core/src/iching/mod.rs` updated** with `pub mod corpus;` + `pub use corpus::{all_hexagrams, get_hexagram};` re-export alongside the existing schema re-export.
- **`crates/amlich-core/tests/iching_corpus_integration.rs` (316 lines)** carries 8 black-box integration tests verifying ICH-01 success criteria 1-4 from the external crate path (`use amlich_core::iching::{...}`):
  1. **SC1** `lookup_all_64_indices_succeed` — every King Wen index 1..=64 looks up to Some(entry) with non-empty vi_name, thoai_tu, cat_hung, hao_tu, and serializable upper/lower trigrams.
  2. **SC1** `hao_tu_length_rule_honored` — #1/#2 carry 7 entries; #3/#10/#33/#64 carry 6 entries (ADR-0005 §2).
  3. **`corpus_trigram_identity_matches_composition_table`** — authoring-error catcher comparing the serde-serialized NAME identity (NOT discriminants) of `entry.upper_trigram`/`lower_trigram` against `COMPOSITION_TABLE[i]` for all 64. CRIT-3 isolation preserved: never converts between `TienThienTrigram` and `HauThienTrigram`.
  4. **SC2** `every_entry_carries_reviewer_signature` — `reviewer` contains "ExternalReviewPending" + "external-kinh-dich-reviewer" + "2026-12-31"; typed `pending_review` DeferralMarker consistent (same assigned_to + expected_review_date).
  5. **SC3** `every_text_field_is_nfc_normalized` — `is_nfc` on vi_name/thoai_tu/cat_hung/each hao_tu line + reserved *_en.
  6. **SC4** `load_is_lazy_and_idempotent` — `as_ptr()` equality on two `all_hexagrams()` calls (OnceLock idempotency).
  7. **SC3** `provenance_ledger_has_64_rows_all_pending` — embeds `provenance_audit.md` via `include_str!`, parses 64 data rows (`| <number> |`), asserts all carry "ExternalReviewPending".
  8. **SC4** `wasm_safety_no_fs_no_utc` — grep guard on `corpus.rs` source asserting no `std::fs::` / `use std::fs;` / `Utc::now` (mirrors v1.6 `fengshui_crit3_isolation.rs`).
- **TDD discipline observed on Task 1**: RED commit (`e227a66` — 7 inline tests fail with "not implemented"), GREEN commit (`8b67850` — implementation passes all 7). Task 2 committed separately as `6b708ef`.
- **Zero regressions** across the crate: 729 lib tests + 7 doc tests + all integration suites pass. Pre-existing `unused_import` warnings in unrelated files (`semantic_graph/views/helpers.rs:115` etc.) are out of scope and logged to deferred-items.md.

## Task Commits

Each task was committed atomically (TDD on Task 1 produced the conventional RED→GREEN pair):

1. **Task 1 RED: failing tests for IChing corpus loader** — `e227a66` (test)
   - `crates/amlich-core/src/iching/corpus.rs` (176 lines) — stub `unimplemented!()` impls + full `#[cfg(test)] mod tests` (7 tests) + HexagramFile envelope + OnceLock static + HEXAGRAMS_JSON include_str!
   - `crates/amlich-core/src/iching/mod.rs` — `pub mod corpus;` + `pub use corpus::{all_hexagrams, get_hexagram};`
   - All 7 tests fail with "not implemented: RED phase"
2. **Task 1 GREEN: implement OnceLock IChing corpus loader + lookup API** — `8b67850` (feat)
   - `crates/amlich-core/src/iching/corpus.rs` — replace stubs with full impl: `all_hexagrams()`, `get_hexagram()`, `normalize_and_validate()` (hao_tu invariant + NFC), `nfc()` helper
   - All 7 inline tests pass
3. **Task 2: 8 black-box IChing corpus integration tests** — `6b708ef` (test)
   - `crates/amlich-core/tests/iching_corpus_integration.rs` (316 lines) — 8 tests covering SC1-4
   - `crates/amlich-core/src/iching/corpus.rs` doc reworded to avoid literal `Utc::now` call syntax (Rule 1 deviation — see below)
   - All 8 integration tests pass

**Plan metadata:** (pending final docs commit below)

## Files Created/Modified

- `crates/amlich-core/src/iching/corpus.rs` (created, 233 lines) — OnceLock-cached IChing corpus loader + lookup API. `HEXAGRAMS_JSON` include_str!, `EXPECTED_SCHEMA_VERSION = "iching-v1"` assertion, `HexagramFile` envelope, `all_hexagrams()` / `get_hexagram()`, `normalize_and_validate()` (NFC + hao_tu length invariant per ADR-0005 §2), `nfc()` helper.
- `crates/amlich-core/tests/iching_corpus_integration.rs` (created, 316 lines) — 8 black-box tests for ICH-01 SC1-4 + trigram identity cross-check + WASM-safety grep guard.
- `crates/amlich-core/src/iching/mod.rs` (modified, 19 lines) — registers `pub mod corpus;` + re-exports `all_hexagrams`, `get_hexagram`.

## Decisions Made

- **Loader mirrors rituals/corpus.rs exactly** — the v1.5 rituals pattern (OnceLock + include_str! + HexagramFile envelope + nfc() helper) is now proven across three corpus milestones (rituals v1.5, golden v1.6, iching v1.7). No structural divergence; only the corpus-specific invariants (hao_tu length rule, schema_version string) are new.
- **`hao_tu` length invariant is loader-enforced, NOT serde-enforced** — Rust's `Vec<String>` cannot encode "6 or 7 depending on enum value". Asserting at load (`normalize_and_validate`) is fail-fast: corpus is compile-embedded, so a violation is caught by `cargo test` before release, not at runtime. Panic message cites ADR-0005 §2.
- **`get_hexagram` uses a 64-entry linear scan** — mirrors `compose()`'s scan decision in `schema.rs:261-269`. A pre-computed reverse lookup map is premature optimization for 64 entries accessed rarely (Mai Hoa casting will look up one hexagram per cast, not in a hot loop).
- **Trigram identity cross-check compares SERDE NAMES, not discriminants** — `TienThienTrigram::Kien` and `HauThienTrigram::Kien` both serialize to `"kien"` because both enums carry `#[serde(rename_all = "snake_case")]`. Comparing serialized strings preserves CRIT-3 isolation: we never construct a value of one type from another; we just compare the JSON name identity. This catches any authoring error where the corpus's trigram variant doesn't match the composition table.
- **WASM-safety grep guard anchored on USAGE patterns, not bare substrings** — the guard checks for `std::fs::`, `use std::fs;`, and `Utc::now`. Bare-substring checks (e.g. `contains("std::fs")`) false-positive on doc comments that legitimately mention the rule (see Deviation #1).

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] WASM-safety grep guard initially false-positive on its own doc comment**
- **Found during:** Task 2 (running the 8-test suite)
- **Issue:** The first iteration of `wasm_safety_no_fs_no_utc` checked `!CORPUS_SRC.contains("std::fs")` and `!CORPUS_SRC.contains("Utc::now")`. The corpus.rs module doc comment legitimately contained the strings `"std::fs"` and `"chrono::Utc::now()"` (explaining the WASM-safety guarantee) — so the test failed on its own documentation. A grep guard that fires on its own rationale text is broken.
- **Fix:** (a) Tightened the test patterns to match actual USAGE (`std::fs::`, `use std::fs;`, `Utc::now`) — the qualifier `::` distinguishes path usage from doc mention. (b) Reworded the corpus.rs doc to use the form "no filesystem I/O (`std::fs`) or wall-clock access (`chrono::Utc`)" — avoiding the literal `Utc::now` call syntax while keeping the documentation meaningful.
- **Files modified:** `crates/amlich-core/tests/iching_corpus_integration.rs`, `crates/amlich-core/src/iching/corpus.rs`
- **Verification:** `cargo test -p amlich-core --test iching_corpus_integration wasm_safety_no_fs_no_utc` passes; the guard still triggers if a real `std::fs::read_to_string` or `Utc::now()` call is added (manually verified by reasoning about the matcher).
- **Committed in:** `6b708ef` (Task 2 commit)

---

**Total deviations:** 1 auto-fixed (1 bug — false-positive grep guard).
**Impact on plan:** Fix was necessary for the plan's own verification gate (test #8) to pass. No scope creep; no behaviour change to the loader itself. The patterns-established entry documents this for future grep-guard authors.

## Issues Encountered

None beyond the Rule 1 deviation above.

## User Setup Required

None — no external service configuration required. This plan is pure Rust loader + tests against an already-shipped compile-embedded corpus (Plan 21-01). No new dependencies, no environment variables, no dashboards.

## Next Phase Readiness

- **ICH-01 is fully closed.** Both halves now shipped: 21-01 (DATA — 64-entry corpus JSON + 64-row provenance ledger) + 21-02 (CODE — OnceLock loader + lookup API + integration tests). All four ICH-01 success criteria are test-backed from the external crate path.
- **Phase 21 is complete (2/2 plans).** Ready for the parallel track: `/gsd-plan-phase 22` (Mai Hoa casting + Biến Quẻ + Thể/Dụng — `cast_mai_hoa(...) -> MaiHoaCast` consuming `get_hexagram` + `compose()`) OR `/gsd-plan-phase 23` (Thái Tuế / Tam Sát ⇄ Phi Tinh cross-link).
- **`get_hexagram(KingWenHexagram)` is the public API Mai Hoa casting (Phase 22) will call** after composing a Tiên Thiên pair via `compose()` to a King Wen index. Linear scan over 64 entries is adequate for one lookup per cast.
- **CRIT-3 isolation verified by `corpus_trigram_identity_matches_composition_table`** — the corpus's Hậu Thiên trigram identities match the composition table's Tiên Thiên identities by NAME; no `From` impl exists or is needed.
- **No blockers.**

---
*Phase: 21-iching-corpus-loader*
*Completed: 2026-07-16*

## Self-Check: PASSED

- All 3 created/modified files exist on disk: `corpus.rs` (233 lines, ≥ 90), `iching_corpus_integration.rs` (316 lines, ≥ 120), `iching/mod.rs` (19 lines).
- All 3 task commits exist: `e227a66` (test RED), `8b67850` (feat GREEN), `6b708ef` (test integration suite).
- `corpus.rs` contains the required patterns: `include_str!("../../data/iching/hexagrams.json")` (1 match), OnceLock static, HexagramFile envelope, `EXPECTED_SCHEMA_VERSION = "iching-v1"`, `all_hexagrams()` + `get_hexagram()` public API, `normalize_and_validate()` (NFC + hao_tu length invariant), `nfc()` helper.
- `mod.rs` registers `pub mod corpus;` + `pub use corpus::{all_hexagrams, get_hexagram};`.
- `cargo test -p amlich-core --lib iching::corpus` → 7/7 inline tests pass.
- `cargo test -p amlich-core --test iching_corpus_integration` → 8/8 black-box tests pass.
- `cargo test -p amlich-core` (full crate) → 729 lib + 7 doc + all integration suites pass; zero regressions.
- Pre-existing `unused_import` warnings in `semantic_graph/views/helpers.rs:115` are out of scope — logged to `deferred-items.md`.
- ICH-01 ready to be marked Closed in REQUIREMENTS.md (both halves shipped: 21-01 DATA + 21-02 CODE).
