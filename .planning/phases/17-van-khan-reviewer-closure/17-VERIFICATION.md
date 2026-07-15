---
phase: 17-van-khan-reviewer-closure
verified: 2026-07-15T11:43:18Z
status: passed
score: 8/8 must-haves verified
---

# Phase 17: Văn khấn Reviewer Closure Verification Report

**Phase Goal:** User-of-corpus can find every ritual entry carries a `reviewer` field — either an actual reviewer identity (name + date + outcome) or an explicit `ExternalReviewPending` deferral marker with documented reason and expected review date — with a complete audit record and corrected entries re-verified against their cited source.
**Verified:** 2026-07-15T11:43:18Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

The canonical reviewer field is the `reviewer` column in `crates/amlich-core/data/rituals/provenance_audit.md`, not a field in the locked `RitualEntry` JSON schema. This matches the phase's ROADMAP contract and its explicit no-schema-change decision. Independent parsing found exactly 60 unique corpus entries and exactly 60 unique ledger rows with equal ID sets. All 60 current dispositions are valid `ExternalReviewPending(...)` markers; there are no actual-name reviews and no corrected rows in the Phase 17 closure state.

### Observable Truths

| # | Truth | Status | Evidence |
|---|---|---|---|
| 1 | Every one of the 60 ledger entries has either an actual-name reviewer record or an `ExternalReviewPending(...)` marker, with no bare `pending` reviewer cell. | ✓ VERIFIED | Independent audit parsed 60 rows/60 unique IDs; every reviewer starts with `ExternalReviewPending(` and ends with `)`; exact pipe-cell scan found no case-insensitive bare `pending`. Ledger rows are at `provenance_audit.md:18-161`. |
| 2 | Every ledger row has populated `method_of_review`, `date_reviewed`, and `outcome` values using the controlled tokens. | ✓ VERIFIED | All 60 rows use `desk-check`, ISO date `2026-07-15`, and `ExternalReviewPending`; independent count is 0 confirmed / 0 corrected / 0 disputed / 60 ExternalReviewPending. The Rust invariant test validates controlled method/outcome sets and non-empty dates (`rituals_integration.rs:230-259`). |
| 3 | Every deferral marker contains a non-empty reason and expected review date, and its outcome agrees with the deferral disposition. | ✓ VERIFIED | All 60 markers contain a non-empty `reason`, `expected_review_date="2026-12-31"`, and `assigned_to="external-vn-folk-ritual-reviewer"`; the independent audit parsed the date and checked it is not before `date_reviewed`. Every corresponding outcome is `ExternalReviewPending`. Runtime marker validation is wired at `rituals_integration.rs:256-258,481-514`. |
| 4 | All 13 category tables use the same exact eight-column header and separator in the same order. | ✓ VERIFIED | Independent audit counted 13 exact header rows and 13 exact separator rows and confirmed each separator immediately follows its header. Examples begin at `provenance_audit.md:16-17`; the final category header is at lines 149-150. |
| 5 | Ledger and corpus agree 1:1 on the ritual ID set, with no orphans in either direction. | ✓ VERIFIED | Independent JSON/Markdown audit found 13 corpus files, 60 corpus entries, 60 unique corpus IDs, 60 ledger rows, 60 unique ledger IDs, and equal ID sets. The compiled test independently compares `HashSet`s from parsed rows and `all_rituals()` at `rituals_integration.rs:217-228`. |
| 6 | Every row has a valid reviewer disposition and controlled method/outcome tokens. | ✓ VERIFIED | The substantive ledger-driven test parses the compile-time ledger and validates every row (`rituals_integration.rs:217-263`); `cargo test -p amlich-core --test rituals_integration` passed 8/8. |
| 7 | Every corrected ledger ID resolves to the loaded corpus and passes locked-schema/NFC/serde round-trip checks. | ✓ VERIFIED | The ledger contains zero `corrected` rows, which is the explicitly allowed Phase 17 state. The test first asserts that all 60 rows parsed, then derives corrected IDs and contains the lookup/non-empty `invocation_text_vi`/serde round-trip path at `rituals_integration.rs:278-318`. `all_rituals()` deserializes the locked `RitualEntry` schema and NFC-normalizes `invocation_text_vi` at `corpus.rs:94-132`. No corrected source claim exists to re-verify in this phase. |
| 8 | The legacy bare `pending` placeholder is absent from the ledger. | ✓ VERIFIED | Independent exact-cell scan passed; the compiled guard at `rituals_integration.rs:526-545` also passed. Prose references to the historical token and `ExternalReviewPending` are not bare reviewer cells. |

**Score:** 8/8 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
|---|---|---|---|
| `crates/amlich-core/data/rituals/provenance_audit.md` | Canonical 60-row reviewer-audit ledger for RIT-14/RIT-15 | ✓ VERIFIED | Exists and is substantive: 204 lines, 13 category tables, 60 unique rows, eight columns per row, valid marker fields, controlled method/date/outcome values, outcome-count prose, and references. Citation page and confidence cells were independently compared to all 60 JSON entries; source title/publisher references also match after diacritic-insensitive normalization. |
| `crates/amlich-core/tests/rituals_integration.rs` | Ledger parser plus invariant and corrected-entry tests | ✓ VERIFIED | Exists and is substantive. The ledger is embedded with `include_str!` at lines 21-22; tests are at lines 217-319; parser/validators are at lines 328-547. Both tests execute in the existing integration target. |
| `crates/amlich-core/src/rituals/corpus.rs` | Loaded-corpus schema and NFC path used by the corrected-entry gate | ✓ VERIFIED | `all_rituals()` parses all 13 embedded JSON files into `RitualEntry` and calls `normalize_and_validate`; `invocation_text_vi` is NFC-normalized at line 132. The artifact is wired through the public `all_rituals()` call used in both Phase 17 tests. |
| `crates/amlich-core/src/rituals/schema.rs` | Locked `RitualEntry` serde schema | ✓ VERIFIED | `RitualEntry` has `#[serde(deny_unknown_fields)]` at lines 127-150 and uses the required `invocation_text_vi` field at line 139. Corrected-entry test deserializes this exact public type. |

### Key Link Verification

| From | To | Via | Status | Details |
|---|---|---|---|---|
| `provenance_audit.md` | 13 ritual corpus JSON files | `ritual_id`, citation page, and confidence parity | ✓ WIRED | Independent audit loaded `manifest.json`, parsed all 13 files, and proved 60 unique IDs equal the 60 ledger IDs. It also matched every page and confidence value and normalized source-reference text. |
| Eight-column header | Every category sub-table | Exact repeated header + immediately following separator | ✓ WIRED | 13 exact headers and 13 exact separators verified directly. |
| `rituals_integration.rs` | `provenance_audit.md` | Compile-time `include_str!` | ✓ WIRED | `PROVENANCE_AUDIT_MD` embeds `../data/rituals/provenance_audit.md` at lines 21-22 and is consumed by both new tests. |
| Ledger parser | `all_rituals()` | Runtime ID-set equality | ✓ WIRED | `every_ledger_row_passes_invariants` constructs ledger and corpus sets and asserts exact equality at lines 223-228. |
| Corrected ledger rows | `RitualEntry` schema and NFC loader | `all_rituals()` lookup plus serde serialize/deserialize/re-serialize | ✓ WIRED | The path exists at lines 297-318 and uses `invocation_text_vi`. Current ledger has zero corrected rows, so the loop correctly has zero current executions after the 60-row anti-vacuity guard. |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
|---|---|---|---|---|
| RIT-14 | 17-01 | Every ritual has an actual reviewer identity or explicit `ExternalReviewPending` marker with reason and expected date. | ✓ SATISFIED | 60/60 ledger rows have structured markers with non-empty reason and `expected_review_date="2026-12-31"`; exact ledger/corpus ID parity proves one canonical review record per corpus entry. |
| RIT-15 | 17-01 | Per-entry audit record includes reviewer, review method, date, and outcome. | ✓ SATISFIED | All 60 rows have the full eight-column record; controlled values are `desk-check`, `2026-07-15`, and `ExternalReviewPending`. Stable count is 0/0/0/60. |
| RIT-16 | 17-02 | Corrected entries are source-reverified and pass locked schema/NFC guards. | ✓ SATISFIED | There are no corrected rows to source-reverify in the current audit. The ledger-driven test verifies the 60-row parse before deriving the empty corrected set and contains the lookup/schema/NFC/serde gate required for any corrected ID. |

All requirement IDs declared in PLAN frontmatter are accounted for: 17-01 declares RIT-14 and RIT-15; 17-02 declares RIT-16. `.planning/REQUIREMENTS.md:53-55` maps exactly these three IDs to Phase 17. No Phase 17 requirement is orphaned.

### Automated Verification

| Command/check | Result |
|---|---|
| `cargo build -p amlich-core` | PASS |
| `cargo test -p amlich-core --test rituals_integration` | PASS — 8 passed, 0 failed |
| `cargo test -p amlich-core` | PASS — 890 passed in total, 0 failed, 7 ignored; unrelated pre-existing unused-import warnings only |
| Independent manifest/JSON/Markdown audit | PASS — `AUDIT_OK 60 corpus entries 60 ledger rows {'confirmed': 0, 'corrected': 0, 'disputed': 0, 'ExternalReviewPending': 60}` |
| Phase commit/file scope inspection | PASS — implementation commits `1777666`, `57496f7`, and `0c3d483` exist; implementation changes are confined to the ledger and existing ritual integration test target. Working tree was clean before this report was written. |

The repository-local gsd-tools artifact/key-link parser was attempted for both plans but reported that it could not detect the nested frontmatter entries. Artifact and wiring verification was therefore performed manually and with the independent audit above.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
|---|---:|---|---|---|
| `crates/amlich-core/tests/rituals_integration.rs` | 369-451 | The test-only parser does not itself require `header_seen` before accepting a data row and does not prove that a separator is immediately after a header, despite its comments. | ⚠️ Warning | The current ledger is valid because the independent audit directly checked all 13 headers/separators. The in-repo regression test is weaker than its comments for future Markdown-structure drift, but this does not prevent the present phase goal. |
| `crates/amlich-core/tests/rituals_integration.rs` | 287-318 | Phase-state assertion hard-codes `corrected_count == 0` before the forward-compatible corrected-entry loop. | ℹ️ Info | Intentional per Plan 17-02 and correct for the current 0-corrected closure state. A future legitimate correction must update this count expectation before the loop can execute. |

No blocker stubs, TODO/FIXME/HACK markers, empty handlers, placeholder implementations, or bare `pending` ledger cells were found in the phase-modified artifacts.

### Human Verification Required

None for the current Phase 17 closure state. All 60 entries explicitly defer external review, and the phase contract permits that disposition when reason and expected review date are recorded. There are zero `corrected` rows, so there is no claimed human source correction requiring manual spot-checking in this phase. Actual external classical-Vietnamese reviews remain future work by design, due by the recorded `2026-12-31` expected review date.

### Gaps Summary

No goal-blocking gaps found. The corpus has a complete 1:1 canonical audit ledger, every review disposition is explicit and structured, method/date/outcome data is complete, controlled outcome counts are stable, and the corrected-entry schema/NFC gate is wired. The two test-robustness observations above are non-blocking for the current all-deferred state.

---

_Verified: 2026-07-15T11:43:18Z_
_Verifier: Claude (gsd-verifier)_
