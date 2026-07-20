---
phase: 14-phi-tinh-81-cell-aspects-safety-hints
verified: 2026-05-28T00:00:00Z
status: passed
score: 5/5 must-haves verified
re_verification: false
---

# Phase 14: Phi Tinh 81-Cell Aspects + Safety Hints Verification Report

**Phase Goal:** User can look up the digitized 2-star aspect for any of the 81 ordered star pairs and receive citation-bearing advisory hints (danger predicate + Ngũ-Hành element hint) — never product names.
**Verified:** 2026-05-28
**Status:** PASSED
**Re-verification:** No — initial verification

---

## Goal Achievement

### Observable Truths

| #   | Truth | Status | Evidence |
| --- | ----- | ------ | -------- |
| 1 | Caller can invoke `lookup_star_pair_aspect(star_a, star_b)` for any of the 81 ordered pairs and receive a `StarPairAspect { name, ngu_hanh_relation, auspice, original_citation }` | VERIFIED | `aspects.rs` L187-201; JSON corpus has 81 ordered pairs (Python validation: missing pairs = NONE); `all_81_pairs_lookup_ordered` integration test passes |
| 2 | Every `StarPairAspect` carries `source_id: "huyen-khong"` plus an `original_citation` pointing to a chapter of Thẩm Thị Huyền Không Học, with a `confidence` tier | VERIFIED | Corpus validator (L143-150) enforces SOURCE_HUYEN_KHONG at load time; all 81 rows have non-empty citation titles (0 empty); 6 primaries with chapter-level page citations; `aspect_provenance_discipline` integration test passes |
| 3 | Caller can invoke `compute_palace_aspects(year, month, &term_scanner)` and receive `[StarPairAspect; 9]` derived from the combined overlay | VERIFIED | `aspects.rs` L210-220 delegates to `compute_combined_overlay` then `std::array::from_fn`; `compute_palace_aspects_matches_overlay` integration test confirms 9-element array matches `overlay.palace_overlays[i]` for all 9 palaces with year=2024, month=1 |
| 4 | Caller can call `is_danger_palace(star)` on `FlyingStar` and receive `true` exactly for Ngũ Hoàng (5) and Nhị Hắc (2) per classical tradition | VERIFIED | `safety.rs` L178-180: `matches!(star, FlyingStar::NguHoang \| FlyingStar::NhiHac)`; `danger_palace_predicate` integration test asserts true for 2 stars, false for all remaining 7 |
| 5 | Caller can call `element_hint_for_palace(star)` and receive an `Option<RemedyHint>` referencing Ngũ-Hành with a classical citation; test suite verifies no product names appear anywhere in the hint corpus | VERIFIED | `safety.rs` L193-201; 4-row JSON corpus with stars 2/3/5/7; `element_hint_present_for_danger_stars` confirms Some for NguHoang/NhiHac, None for NhatBach; `no_product_names_in_corpora` scans all 9 hints + 81 aspect names against FORBIDDEN_PRODUCT_TERMS — 0 violations |

**Score:** 5/5 truths verified

---

### Required Artifacts

| Artifact | Expected | Status | Details |
| -------- | -------- | ------ | ------- |
| `crates/amlich-core/src/almanac/fengshui/aspects.rs` | StarPairAspect, FsCitation, FsConfidenceTier types; corpus loader + validator; lookup_star_pair_aspect; compute_palace_aspects | VERIFIED | 355 lines (min 120); contains `pub fn compute_palace_aspects` at L210; OnceLock+include_str! loader; seen-grid validator; 8 unit tests |
| `crates/amlich-core/data/almanac/flying_star_aspects.json` | 81-cell ordered star-pair aspect corpus | VERIFIED | schema_version "aspects-v1" present; 81 aspects (all 9x9 pairs, 0 missing); all source_id == "huyen-khong"; 0 empty citation titles; 6 primary + 75 synthesized entries |
| `crates/amlich-core/src/almanac/fengshui/mod.rs` | pub mod aspects + re-exports of aspects public API | VERIFIED | `pub mod aspects;` at L14; re-exports at L26: compute_palace_aspects, lookup_star_pair_aspect, FsCitation, FsConfidenceTier, StarPairAspect |
| `crates/amlich-core/src/almanac/fengshui/safety.rs` | is_danger_palace predicate; RemedyHint type; element_hint_for_palace loader+lookup | VERIFIED | 301 lines (min 70); `pub fn is_danger_palace` at L178; RemedyHint at L56; OnceLock loader; 6 unit tests |
| `crates/amlich-core/data/almanac/flying_stars_safety.json` | Per-star Ngũ-Hành mitigation hints | VERIFIED | schema_version "safety-v1" present; 4 hint rows for stars 2/3/5/7; all source_id == "huyen-khong"; all citation titles non-empty |
| `crates/amlich-core/tests/fengshui_aspects.rs` | Black-box integration tests for FS-11..FS-15 + no-product-names guard | VERIFIED | 329 lines (min 70); contains `no_product_names` (2 occurrences); FORBIDDEN_PRODUCT_TERMS (3 occurrences); `use amlich_core::` (3 occurrences); 6 tests |

---

### Key Link Verification

| From | To | Via | Status | Details |
| ---- | -- | --- | ------ | ------- |
| `aspects.rs` | `flying_star_aspects.json` | `include_str!` + OnceLock::get_or_init + serde_json::from_str | WIRED | L26: `const ASPECTS_JSON: &str = include_str!("../../../data/almanac/flying_star_aspects.json")` |
| `compute_palace_aspects` | `compute_combined_overlay` | delegation then per-palace lookup | WIRED | L215: `let overlay = compute_combined_overlay(year, month, scanner)` + `std::array::from_fn` |
| `StarPairAspect.source_id` assignments | `crate::sources::SOURCE_HUYEN_KHONG` | constant, no bare literal in .rs | WIRED | Validator L145: `crate::sources::SOURCE_HUYEN_KHONG`; source_id_guard CI passes |
| `safety.rs` | `flying_stars_safety.json` | `include_str!` + OnceLock::get_or_init | WIRED | L36: `const SAFETY_JSON: &str = include_str!("../../../data/almanac/flying_stars_safety.json")` |
| `safety.rs RemedyHint` | `aspects::FsCitation` | `use crate::almanac::fengshui::aspects::FsCitation` | WIRED | L28: `use crate::almanac::fengshui::aspects::FsCitation` |
| `is_danger_palace` | `FlyingStar::NguHoang \| FlyingStar::NhiHac` | `matches!` pattern | WIRED | L179: `matches!(star, FlyingStar::NguHoang \| FlyingStar::NhiHac)` |
| `tests/fengshui_aspects.rs` | `amlich_core` public API | `use amlich_core::...` (external-consumer import) | WIRED | L13-17: imports all 6 public functions via `use amlich_core::almanac::fengshui::{...}` |
| `no_product_names_in_corpora` | `element_hint_for_palace` + `lookup_star_pair_aspect` corpora | runtime scan against FORBIDDEN_PRODUCT_TERMS | WIRED | L272-329: const array of 9 forbidden terms; scans all 9 hints + all 81 aspect names |

---

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| ----------- | ----------- | ----------- | ------ | -------- |
| FS-11 | 14-01, 14-03 | `lookup_star_pair_aspect(star_a, star_b) -> StarPairAspect` for all 81 ordered pairs | SATISFIED | aspects.rs L187-201; JSON corpus complete; `all_81_pairs_lookup_ordered` test passes |
| FS-12 | 14-01, 14-03 | `StarPairAspect` carries source_id "huyen-khong", original_citation to Thẩm Thị Huyền Không Học, confidence tier | SATISFIED | Corpus validator enforces at load time; `aspect_provenance_discipline` test verifies 6-pair sample |
| FS-13 | 14-01, 14-03 | `compute_palace_aspects(year, month, term_scanner) -> [StarPairAspect; 9]` from combined overlay | SATISFIED | aspects.rs L210-220; `compute_palace_aspects_matches_overlay` test verifies 9 palaces for 2024-01 |
| FS-14 | 14-02, 14-03 | `is_danger_palace()` predicate true for Ngũ Hoàng and Nhị Hắc | SATISFIED | safety.rs L178-180; `danger_palace_predicate` test asserts truth table for all 9 stars |
| FS-15 | 14-02, 14-03 | `element_hint_for_palace(star) -> Option<RemedyHint>` with classical citation, never product names | SATISFIED | safety.rs L193-201; JSON corpus; `element_hint_present_for_danger_stars` + `no_product_names_in_corpora` tests pass |

No orphaned requirements — all 5 Phase 14 requirements (FS-11..FS-15) claimed by plans 14-01/14-02/14-03 and marked Complete in REQUIREMENTS.md.

---

### Anti-Patterns Found

No anti-patterns found in any of the 3 new/modified source files or the test file. No TODO/FIXME/XXX/HACK/PLACEHOLDER comments; no stub return values (`return null`, `return {}`, `return []`); no console.log-only implementations.

The one notable item — a bare `"huyen-khong"` string at safety.rs L191 — is in a doc comment (`///` line), not executable code, and the source_id_guard CI test passes confirming it is not flagged.

---

### Human Verification Required

None. All five success criteria are fully automatable:
- All 81 pair lookups are verified programmatically.
- Citation and source_id discipline is enforced at corpus load time and by integration tests.
- Palace aspect derivation is numerically cross-checked against compute_combined_overlay.
- The danger predicate truth table is exhaustively tested (all 9 variants).
- The no-product-names guard scans the entire runtime corpus against a defined forbidden-terms list.

---

### Gaps Summary

No gaps. All 5 observable truths pass, all 6 required artifacts are substantive and wired, all 8 key links are confirmed in source, all 5 requirements are satisfied, and the full amlich-core test suite (691 lib tests + 6 new integration tests + all other integration suites) is green with 0 failures.

---

_Verified: 2026-05-28_
_Verifier: Claude (gsd-verifier)_
