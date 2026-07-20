---
phase: 22-mai-hoa-casting-bien-que-the-dung
verified: 2026-07-16T04:14:52Z
status: passed
score: 7/7 must-haves verified
gaps: []
---

# Phase 22: Mai Hoa Casting + Biến Quẻ + Thể/Dụng Verification Report

**Phase Goal:** From ROADMAP.md — Phase 22: Mai Hoa Casting + Biến Quẻ + Thể/Dụng — closes ICH-02, ICH-03, ICH-04
**Verified:** 2026-07-16T04:14:52Z
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| #   | Truth                                                                                                                                                  | Status     | Evidence                                                                                                          |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------- | ----------------------------------------------------------------------------------------------------------------- |
| 1   | `cast_mai_hoa(year_branch, month, day, hour) -> MaiHoaCast` exists, is deterministic, and the (8,8,8,8) boundary yields Khôn/#2 (CRIT-2 boundary test passes) | ✓ VERIFIED | `crates/amlich-core/src/iching/mai_hoa.rs:104` (`cast_mai_hoa`); CRIT-2 inline test at line 152 + integration test at `mai_hoa_casting_integration.rs:47` (both pass; explicit rejection of #1 Kiền) |
| 2   | `derive_bien_que(&MaiHoaCast) -> BienQue` exists, flips exactly the động hào line, and the 384-case exhaustive contract test passes (CRIT-4)             | ✓ VERIFIED | `crates/amlich-core/src/iching/bien_que.rs:105` (`derive_bien_que`); inline 384-case test at line 246 + integration test at `mai_hoa_casting_integration.rs:127` (both pass); worked (8,8,8,8)→#7 Sư verified with explicit rejection of #8 Tỷ trigram-order inversion |
| 3   | `classify_the_dung(&MaiHoaCast) -> TheDungClassification` exposes Thể/Dụng classification + Ngũ Hành sinh/khắc + Cát/Hùng/Bình verdict               | ✓ VERIFIED | `crates/amlich-core/src/iching/the_dung.rs:223` (`classify_the_dung`); 5-way `TheDungRelation` enum (line 127), `CatHung` enum (line 162), 10 inline tests + 11 black-box integration tests all pass (5 verdict cases each verified) |
| 4   | Cross-source golden dataset at `crates/amlich-core/data/iching/mai_hoa_golden.json` has ≥10 cases, each with ≥2 sources, divergences logged as `KnownDivergence` reusing v1.6 fengshui/golden.rs patterns | ✓ VERIFIED | `jq '.cases \| length'` = 12 (≥10); `jq '.cases \| map(.sources \| length) \| min'` = 2 (≥2); `jq '.known_divergences \| length'` = 2 (≥1); `DeferralMarker` + `GoldenConfidence` reused verbatim from `crate::almanac::fengshui::golden` |
| 5   | CRIT-3 type isolation preserved: NO `impl From<...>` between `TienThienTrigram`, `HauThienTrigram`, `KingWenHexagram`                                  | ✓ VERIFIED | `rg "impl From<(TienThienTrigram\|HauThienTrigram\|KingWenHexagram)> for"` returns ZERO matches across `crates/amlich-core/src/iching/`; only doc-comment mentions in `schema.rs` documenting the absence; runtime-built grep-needle guards in all 4 new modules + 2 integration tests |
| 6   | Full crate test suite green (`cargo test -p amlich-core`)                                                                                              | ✓ VERIFIED | 990 tests passed, 0 failures across 18 test binaries (lib + integration suites + doc tests); matches SUMMARY claim of 990/0 |
| 7   | Cross-reference requirement IDs from PLAN frontmatter against REQUIREMENTS.md — ICH-02, ICH-03, ICH-04 all accounted for (closed or in-progress correctly) | ✓ VERIFIED | ICH-02 (line 21), ICH-03 (line 22), ICH-04 (line 23) all marked `[x]` Complete; Phase mapping (line 80) shows Phase 22 owns all three; status table (lines 60-62) confirms Complete with plan attribution (22-01 for ICH-02/03, 22-02 for ICH-04) |

**Score:** 7/7 must-haves verified

### Required Artifacts

| Artifact                                                                            | Expected                                                                                                                  | Status      | Details                                                                                                                                             |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/amlich-core/src/iching/mai_hoa.rs`                                           | `MaiHoaCast` struct + `cast_mai_hoa()` deterministic casting (ADR-0006 convention); ≥90 lines                              | ✓ VERIFIED  | 279 lines; `MaiHoaCast` struct at line 60, `cast_mai_hoa` at line 104, `mai_hoa_remainder` CRIT-2 helper at line 80; 5 inline tests pass            |
| `crates/amlich-core/src/iching/bien_que.rs`                                          | `derive_bien_que()` transformation + trigram↔3-line bit mapping; ≥80 lines                                               | ✓ VERIFIED  | 311 lines; `derive_bien_que` at line 105, `trigram_lines` at line 62, `lines_to_trigram` at line 80; 4 inline tests pass incl. CRIT-4 384-case       |
| `crates/amlich-core/src/iching/the_dung.rs`                                         | trigram→Ngũ Hành mapping + `TheDungClassification` + `TheDungRelation` sinh/khắc + `CatHung` verdict; ≥110 lines          | ✓ VERIFIED  | 532 lines; `trigram_element` at line 70, `generates`+`controls` at 94/108, `TheDungRelation` at 127, `CatHung` at 162, `TheDungClassification` at 182, `classify_the_dung` at 223; 10 inline tests pass |
| `crates/amlich-core/src/iching/golden.rs`                                           | `MaiHoaGoldenCase` + `MaiHoaGoldenDataset` + `load_mai_hoa_golden()` OnceLock loader; ≥120 lines                            | ✓ VERIFIED  | 392 lines; types at 105/126/146, `load_mai_hoa_golden` at line 169, NFC normalization + schema-version assertion + FS-10/SC4 validation at 211-246; 7 inline tests pass |
| `crates/amlich-core/data/iching/mai_hoa_golden.json`                                 | ≥10 cross-source golden casting cases; "mai-hoa-golden-v1" schema                                                         | ✓ VERIFIED  | 281 lines; 12 cases (`$schema_version = "mai-hoa-golden-v1"`); all 12 with exactly 2 sources (FS-10 met); 2 `KnownDivergence` rows w/ `DeferralMarker` |
| `crates/amlich-core/tests/mai_hoa_casting_integration.rs`                           | Black-box integration tests for ICH-02 + ICH-03; ≥180 lines; contains "384"                                                | ✓ VERIFIED  | 414 lines; 6 black-box tests pass (CRIT-2 boundary + CRIT-4 384-case + determinism + range sweep + worked #7 Sư + CRIT-3 grep)                    |
| `crates/amlich-core/tests/mai_hoa_the_dung_integration.rs`                           | Black-box tests for ICH-04 + golden-dataset integrity; ≥160 lines                                                          | ✓ VERIFIED  | 398 lines; 11 black-box tests pass (5 verdict cases + golden dataset integrity + cross-source verification + CRIT-3 + WASM-safety)              |
| `crates/amlich-core/src/iching/mod.rs`                                              | Module registration + re-exports for all Phase 22 types                                                                    | ✓ VERIFIED  | 27 lines; registers `pub mod mai_hoa; bien_que; the_dung; golden;`; re-exports `cast_mai_hoa, MaiHoaCast, derive_bien_que, BienQue, classify_the_dung, TheDungClassification, TheDungRelation, CatHung, trigram_element, load_mai_hoa_golden, MaiHoaGoldenCase, MaiHoaGoldenDataset` |

### Key Link Verification

| From                                                | To                                                          | Via                                                              | Status     | Details                                                                                                                 |
| --------------------------------------------------- | ----------------------------------------------------------- | ---------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------------------------- |
| `crates/amlich-core/src/iching/mai_hoa.rs`          | `crates/amlich-core/src/iching/schema.rs::compose`          | `compose(upper, lower) -> KingWenHexagram` (line 120)            | ✓ WIRED    | Pattern `compose(` confirmed at line 120; `use crate::iching::schema::compose` in source                                |
| `crates/amlich-core/src/iching/bien_que.rs`         | `crates/amlich-core/src/iching/schema.rs::COMPOSITION_TABLE` | `compose(new_upper, new_lower)` for biến quẻ re-composition (line 133) | ✓ WIRED    | Pattern `compose(` confirmed at line 133; uses the locked schema bridge                                                 |
| `crates/amlich-core/src/iching/mod.rs`              | `mai_hoa + bien_que + the_dung + golden modules`            | `pub mod` + `pub use` re-exports                                  | ✓ WIRED    | All 4 new modules registered (lines 13-18); 12 new types re-exported at module root (lines 20-27)                       |
| `crates/amlich-core/src/iching/the_dung.rs`         | `crates/amlich-core/src/almanac/types.rs::FiveElement`      | `trigram_element(t) -> FiveElement` plain fn (line 70)            | ✓ WIRED    | Pattern `FiveElement` confirmed (5 matches); `use crate::almanac::types::FiveElement` import at line 34                 |
| `crates/amlich-core/src/iching/golden.rs`           | `crates/amlich-core/src/almanac/fengshui/golden.rs::KnownDivergence`/`DeferralMarker` | `use crate::almanac::fengshui::golden::{DeferralMarker, GoldenConfidence};` (line 35) | ✓ WIRED    | Pattern `KnownDivergence`/`DeferralMarker`/`GoldenConfidence` confirmed; v1.6 fengshui/golden types reused verbatim       |
| `crates/amlich-core/src/iching/golden.rs`           | `data/iching/mai_hoa_golden.json`                            | `include_str!("../../data/iching/mai_hoa_golden.json")`           | ✓ WIRED    | Pattern `include_str!` confirmed at line ~88; OnceLock-cached loader mirrors `corpus.rs` discipline                     |

### Requirements Coverage

| Requirement | Source Plan | Description                                                                                                                                                       | Status      | Evidence                                                                                                                                                       |
| ----------- | ---------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| ICH-02      | 22-01       | `cast_mai_hoa(year_branch, month, day, hour) -> MaiHoaCast` exists, deterministic, honours `((n-1)%k)+1` (CRIT-2 boundary test passes — (8,8,8,8)→Khôn/#2)         | ✓ SATISFIED | REQUIREMENTS.md line 21 `[x]`, status table line 60 "Complete"; inline test `crit2_all_eights_yields_khon` passes; integration test `crit2_all_eights_boundary_yields_khon_not_kien` passes; 51,840-cast range sweep passes |
| ICH-03      | 22-01       | `derive_bien_que(&MaiHoaCast) -> BienQue` exists, flips exactly the động hào line, 384-case (64×6) exhaustive contract test passes (CRIT-4)                       | ✓ SATISFIED | REQUIREMENTS.md line 22 `[x]`, status table line 61 "Complete"; inline test `crit4_bien_que_384_case_exhaustive_contract_inline` passes; integration test `crit4_bien_que_384_case_exhaustive_contract` passes; worked (8,8,8,8)→#7 Sư verified |
| ICH-04      | 22-02       | `classify_the_dung(&MaiHoaCast) -> TheDungClassification` exposes Thể/Dụng + Ngũ Hành sinh/khắc + Cát/Hùng/Bình verdict                                              | ✓ SATISFIED | REQUIREMENTS.md line 23 `[x]`, status table line 62 "Complete"; 10 inline tests + 11 black-box integration tests pass; 5 verdict cases each verified              |

### Anti-Patterns Found

| File                                                                       | Line | Pattern                | Severity | Impact                                                                                                                                              |
| -------------------------------------------------------------------------- | ---- | ---------------------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
| `crates/amlich-core/src/iching/golden.rs`                                  | 359  | Test name `wasm_safety_no_fs_no_utc_no_rand_inline` contains literal "fs", "utc", "rand" substrings | ℹ️ Info  | Test name is intentional; the runtime-built needle (`format!("rand{}", "::")`) at line 372 + `!SRC.contains(rand_colon.as_str())` at line 387 is the correct pattern (mirrors corpus.rs WASM-safety test discipline). Not an actual `rand::`/`std::fs::`/`Utc::now` API usage. |

No TODO/FIXME/XXX/HACK/PLACEHOLDER markers found. No empty implementations. No console.log-only stubs. All `unimplemented!()` RED-phase stubs were properly replaced in GREEN commits per SUMMARY deviation documentation.

### Human Verification Required

None. All must-haves are programmatically verifiable via the test suite, and all automated checks pass:
- CRIT-2 boundary (8,8,8,8)→Khôn/#2 is asserted with explicit rejection of #1 regression — purely arithmetic, no human judgment required
- CRIT-4 384-case exhaustive contract is verified by automated nested-loop test — exhaustive, no sampling needed
- Trigram→Ngũ Hành mapping + 5-way sinh/khắc + 5 verdict cases are pure lookup-table logic — automated tests cover all paths
- Golden dataset cross-source verification is mechanical (algorithm reproduces expected outputs) — automated
- CRIT-3 isolation is structural (grep guard for forbidden patterns) — automated
- Full crate suite green (990 tests, 0 failures) — automated

The Vietnamese-language content (cát/hùng/bình verdicts, classical Bát Quái names, Thiệu Khang Tiết citations) is the algorithmic output of well-defined lookup tables; no subjective interpretation is involved at this layer (that comes in Phase 24's semantic-graph layer).

### Gaps Summary

No gaps. All 7 must-haves verified against the actual codebase with three-level verification (exists, substantive, wired):

1. ✓ `cast_mai_hoa` (CRIT-2 boundary test passes; (8,8,8,8)→Khôn/#2 explicitly asserted, #1 Kiền explicitly rejected)
2. ✓ `derive_bien_que` (CRIT-4 384-case exhaustive contract passes; 64×6 cases verified)
3. ✓ `classify_the_dung` (5-way relation + 5 verdict cases each verified)
4. ✓ Golden dataset (12 cases, all with 2 sources, 2 `KnownDivergence` rows with `DeferralMarker` reuse)
5. ✓ CRIT-3 isolation preserved (zero `impl From<...>` definitions between the three newtypes)
6. ✓ Full crate test suite green (990 tests, 0 failures, 0 regressions)
7. ✓ ICH-02 + ICH-03 + ICH-04 all marked Complete in REQUIREMENTS.md

Phase 22 is fully closed. All three IChing-pillar requirements (ICH-02, ICH-03, ICH-04) achieved. Ready for Phase 24 (IChing Evaluator + Semantic-Graph Wiring) and Phase 25 (E2E Validation + Golden Cross-Source), which both consume `cast_mai_hoa` + `derive_bien_que` + `classify_the_dung` + `load_mai_hoa_golden` as built by Phase 22.

---

_Verified: 2026-07-16T04:14:52Z_
_Verifier: Claude (gsd-verifier)_