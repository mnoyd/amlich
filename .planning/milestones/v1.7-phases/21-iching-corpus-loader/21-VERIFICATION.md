---
phase: 21-iching-corpus-loader
verified: 2026-07-16T09:35:00Z
status: passed
score: 14/14 must-haves verified
re_verification: no
---

# Phase 21: IChing Corpus + Loader Verification Report

**Phase Goal:** User-of-corpus can load the 64-hexagram Ngô Tất Tố corpus via a lazy `OnceLock` loader and look up any hexagram by King Wen index, with every entry reviewer-signed and any Ngô Tất Tố source gaps surfaced as `PendingExternalReview` rather than silently filled from another translator. (Closes ICH-01.)
**Verified:** 2026-07-16T09:35:00Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

Must-haves sourced from the two PLAN frontmatters (6 from 21-01, 8 from 21-02) plus the 4 ROADMAP success criteria. Truths 1-6 = Plan 21-01 (DATA); 7-14 = Plan 21-02 (CODE). ROADMAP SC1-4 map onto truths 7,4,10+6,11+12 respectively.

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | A reader of `hexagrams.json` finds exactly 64 HexagramEntry records, one per King Wen index 1..=64, each appearing exactly once | ✓ VERIFIED | Python check: `entries=64`, `indices==list(range(1,65))`, 0 issues. Inline test `all_hexagrams_has_64_entries` + `entries_are_in_ascending_king_wen_order` pass. |
| 2 | Every entry's upper/lower_trigram identity matches COMPOSITION_TABLE pair at that index | ✓ VERIFIED | Integration test `corpus_trigram_identity_matches_composition_table` (compares serialized serde NAMES across 64 entries — CRIT-3-safe) passes. Python: `#1 upper=kien/lower=kien`, `#2 upper=khon/lower=khon` match composition table. |
| 3 | #1 & #2 have 7 hao_tu; #3..=64 have 6 hao_tu | ✓ VERIFIED | Python invariant check on all 64 entries: 0 violations. Inline test `hao_tu_length_invariant_at_load` + integration `hao_tu_length_rule_honored` pass. |
| 4 | Every entry carries pending_review: Some(DeferralMarker) + reviewer ExternalReviewPending marker (AF-05) | ✓ VERIFIED | Python: all 64 entries have `pending_review != None` and `reviewer` contains `ExternalReviewPending`. Integration test `every_entry_carries_reviewer_signature` validates typed `DeferralMarker` consistency (assigned_to + 2026-12-31). |
| 5 | Envelope `{$schema_version: "iching-v1", entries: [...]}` | ✓ VERIFIED | Python: `$schema_version == "iching-v1"`. Loader asserts equality at load (`corpus.rs:65-69`). |
| 6 | `provenance_audit.md` has exactly 64 ledger rows, all ExternalReviewPending | ✓ VERIFIED | `rg -c "^\| [0-9]+ \|" == 64`. Integration test `provenance_ledger_has_64_rows_all_pending` parses 64 rows from `include_str!` of the ledger, asserts every row contains `ExternalReviewPending`. |
| 7 | Caller can look up any of 64 King Wen indices via `get_hexagram` and receive populated entry | ✓ VERIFIED | Inline `every_index_lookup_succeeds` + integration `lookup_all_64_indices_succeed` — every kw 1..=64 returns Some with non-empty vi_name/thoai_tu/cat_hung/hao_tu + valid trigram variants. |
| 8 | Caller can retrieve all 64 entries via `all_hexagrams() -> &'static [HexagramEntry]` | ✓ VERIFIED | `corpus.rs:59-73` returns `&'static [HexagramEntry]`. Inline test asserts `len()==64`. |
| 9 | hao_tu length rule enforced at load (panic on violation) | ✓ VERIFIED | `corpus.rs:97-106` `assert_eq!(entry.hao_tu.len(), expected, ...)` in `normalize_and_validate()`. Cannot be serde — `Vec<String>` has no length-dependent-on-other-field derive. |
| 10 | Every Vietnamese text field is NFC-normalized | ✓ VERIFIED | Python: 0 NFC violations across all 64 entries × all text fields. Loader normalizes via `nfc()` helper (`corpus.rs:133-139`). Inline `every_text_field_is_nfc` + integration `every_text_field_is_nfc_normalized` pass. |
| 11 | Corpus load is lazy (OnceLock — first call triggers parse, subsequent return same slice) | ✓ VERIFIED | `corpus.rs:44` `static HEXAGRAMS: OnceLock<Vec<HexagramEntry>>`. Integration `load_is_lazy_and_idempotent` asserts `as_ptr()` equality on two calls. |
| 12 | WASM-safe: no std::fs, no Utc::now anywhere in iching module | ✓ VERIFIED | `rg 'std::fs::|use std::fs|Utc::now\(' src/iching/` returns 0 matches. Integration `wasm_safety_no_fs_no_utc` grep guard passes (anchored on `std::fs::` / `use std::fs;` / `Utc::now` — actual usage patterns, not doc text). |
| 13 | Loader asserts `$schema_version == "iching-v1"` at load (ADR enforcement) | ✓ VERIFIED | `corpus.rs:33` `EXPECTED_SCHEMA_VERSION = "iching-v1"` + `corpus.rs:65-69` `assert_eq!` panics on mismatch. |
| 14 | Provenance ledger test-verified 64 rows all ExternalReviewPending | ✓ VERIFIED | Truth 6 evidence; ledger-driven test prevents audit/test drift (RIT-14 pattern). |

**Score:** 14/14 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `crates/amlich-core/data/iching/hexagrams.json` | 64-entry Ngô Tất Tố corpus; envelope `{$schema_version: iching-v1, entries: [...]}`; contains `$schema_version` | ✓ VERIFIED | 1415 lines, 101 KB, valid JSON, 64 entries, 0 invariant violations, 0 NFC violations. |
| `crates/amlich-core/data/iching/provenance_audit.md` | 64-row ledger mirroring Phase 17 closure template; contains `ExternalReviewPending` | ✓ VERIFIED | 142 lines, 64 data rows (rg-verified), header block + 8-octant sub-headings + References section, reviewer markers byte-identical to `hexagrams.json` (Python check: 0 mismatches). |
| `crates/amlich-core/src/iching/corpus.rs` | OnceLock loader + `all_hexagrams`/`get_hexagram` lookup API; min 90 lines; exports both fns | ✓ VERIFIED | 233 lines (≥90). Exports `all_hexagrams`, `get_hexagram`. Contains `include_str!`, `OnceLock`, `HexagramFile` envelope, `normalize_and_validate` (NFC + hao_tu invariant), `nfc()` helper, 7 inline tests. |
| `crates/amlich-core/src/iching/mod.rs` | Module registration; contains `pub mod corpus` | ✓ VERIFIED | 19 lines. `pub mod corpus;` + `pub use corpus::{all_hexagrams, get_hexagram};` alongside existing `pub use schema::{...}`. |
| `crates/amlich-core/tests/iching_corpus_integration.rs` | Black-box integration tests for ICH-01 SC1-4; min 120 lines | ✓ VERIFIED | 316 lines (≥120). 8 tests covering SC1-4 + authoring-error cross-check + WASM grep guard. External-crate import path `use amlich_core::iching::{...}`. |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `corpus.rs` | `data/iching/hexagrams.json` | `include_str!("../../data/iching/hexagrams.json")` compile-time embedding + serde parse + OnceLock cache | ✓ WIRED | `corpus.rs:29` const embed; `corpus.rs:62` `serde_json::from_str(HEXAGRAMS_JSON)`; compile-time embed verified by `cargo build` + all 15 tests passing. |
| `corpus.rs` | `schema.rs::HexagramEntry` | `Vec<HexagramEntry>` deserialized via envelope struct | ✓ WIRED | `corpus.rs:24` `use crate::iching::schema::{HexagramEntry, KingWenHexagram}`; `corpus.rs:41` `entries: Vec<HexagramEntry>`. |
| `mod.rs` | `corpus.rs` | `pub mod corpus` + `pub use corpus::{all_hexagrams, get_hexagram}` | ✓ WIRED | `mod.rs:13` declares submodule; `mod.rs:16` re-exports both lookup fns. External crate path used by integration tests confirms end-to-end wiring. |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| ICH-01 | 21-01-PLAN, 21-02-PLAN | 64-hexagram lookup via `data/iching/hexagrams.json`, NFC-normalized at load, reviewer-signed; each entry carries `king_wen_index`, `vi_name`, `upper/lower_trigram`, `thoai_tu`, `hao_tu` (6, or 7 for #1/#2), `cat_hung`; loaded via `include_str!` + `OnceLock`; Ngô Tất Tố gaps surfaced as `PendingExternalReview` per AF-05 | ✓ SATISFIED | All 4 ROADMAP success criteria test-backed. 21-01 DATA + 21-02 CODE both shipped. REQUIREMENTS.md line 20 marked `[x]`. Line 59 tracking table: "Complete (21-01 DATA + 21-02 CODE both shipped; 8 black-box integration tests)". |

**Orphaned requirements:** None. Phase 21 maps only ICH-01 (REQUIREMENTS.md line 79) — claimed by both 21-01-PLAN and 21-02-PLAN. No additional unclaimed requirements.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| — | — | — | — | No TODO/FIXME/PLACEHOLDER/unimplemented!/todo!/empty-return anti-patterns in any Phase 21 file (rg scan returned 0 matches). |

**Pre-existing warning (out of scope):** `unused import: ProvenanceSource` in `crates/amlich-core/src/semantic_graph/views/helpers.rs:115`. This is unrelated to Phase 21 scope; logged to `deferred-items.md` for a separate maintenance commit. Not a regression introduced by Phase 21.

### Human Verification Required

None required for goal achievement.

The interpretive text fields (`thoai_tu`, `hao_tu`, `cat_hung`) are intentionally `[PendingExternalReview — ...]` placeholders by design (AF-05 forbids fabricating Ngô Tất Tố source text). Verification that a real external reviewer fills these is tracked separately via the `expected_review_date: "2026-12-31"` deferral marker — out of scope for this phase's goal (the goal was to build the loader + signed corpus, not to source-translate Ngô Tất Tố). The structural completeness, loader mechanics, and signing discipline are fully verified by automated tests.

### Gaps Summary

**No gaps found.** All 14 observable truths verified, all 5 artifacts pass three-level checks (exists + substantive + wired), all 3 key links wired, ICH-01 satisfied with no orphaned requirements, no blocker anti-patterns.

Test execution summary (all run during verification):
- `cargo test -p amlich-core --lib iching::corpus` → **7/7 pass**
- `cargo test -p amlich-core --test iching_corpus_integration` → **8/8 pass**
- `cargo test -p amlich-core` (full crate) → **729 lib + 7 doc + all integration suites pass; 0 failures, 0 regressions**

Commits referenced in SUMMARYs all exist on disk:
- `760b2d9` (feat — corpus JSON), `24e8fbc` (docs — provenance ledger)
- `e227a66` (test RED), `8b67850` (feat GREEN — loader), `6b708ef` (test — integration suite)

The phase goal — "User-of-corpus can load the 64-hexagram Ngô Tất Tố corpus via a lazy OnceLock loader and look up any hexagram by King Wen index, with every entry reviewer-signed and Ngô Tất Tố gaps surfaced as PendingExternalReview" — is fully achieved. ICH-01 is closable (already marked Closed in REQUIREMENTS.md).

---

_Verified: 2026-07-16T09:35:00Z_
_Verifier: Claude (gsd-verifier)_
