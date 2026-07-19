---
phase: 25-e2e-validation-golden-cross-source-verification
verified: 2026-07-20T00:00:00Z
status: passed
score: 6/6 must-haves verified
re_verification: false
---

# Phase 25: E2E Validation + Golden Cross-Source Verification — Verification Report

**Phase Goal (from ROADMAP.md):** User-of-validation can find ≥10 IChing golden casting cases cross-checked against ≥2 independent sources, a 2026 E2E smoke exercising the full new surface (IChing casting + biến quẻ + Thái Tuế cross-link + semantic graph + DaySnapshot), and the full crate test suite green with zero regressions.

**Verified:** 2026-07-20
**Status:** passed
**Re-verification:** No — initial verification

## Goal Achievement

### Observable Truths

| # | Truth | Status | Evidence |
| --- | --- | --- | --- |
| 1 | A reader can find ≥10 IChing golden casting cases (in `data/iching/` golden dataset), each cross-checked against ≥2 independent sources with divergences logged as `KnownDivergence` (SC1) | ✓ VERIFIED | `crates/amlich-core/data/iching/mai_hoa_golden.json` carries 12 cases (boundary-cr2-all-eights, non-boundary-one-one-one, nhantu-001..010) × 2 sources each + 2 KnownDivergence rows; `schema_version = "mai-hoa-golden-v1"`; locked at runtime by `int13_golden_dataset_cross_source_discipline_holds` in `v17_baseline_guards.rs:166` (passes) |
| 2 | A reader can find a 2026 E2E smoke extending `tests/integration_2026_smoke.rs` exercising IChing casting + biến quẻ + Thái Tuế cross-link + semantic-graph wiring + DaySnapshot fields together (SC2) | ✓ VERIFIED | `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates` at `integration_2026_smoke.rs:514` (test passes; 5 representative 2026 dates — Tết + 4 Sóc dates spanning solar months 3/6/9/12) |
| 3 | A caller running `cargo test --package amlich-core` observes the full test suite green with zero regressions on v1.6 baseline (SC3) | ✓ VERIFIED | `cargo test --package amlich-core` → every test result line shows `ok`; 1120 passed, 0 failed, 7 ignored (6 doc-tests in almanac modules + 1 pre-existing ignored) — matches SUMMARY's claim of +3 vs the 1117-test Phase 24-end baseline |
| 4 | A reader can confirm the cargo dependency tree is unchanged from v1.6 (exactly 4 deps: serde + serde_json + chrono + unicode-normalization) (SC4) | ✓ VERIFIED | `cargo tree -p amlich-core --depth 1` shows exactly 4 deps (chrono v0.4.44, serde v1.0.228, serde_json v1.0.149, unicode-normalization v0.1.25); locked at runtime by `cargo_dependency_tree_unchanged_from_v16` in `v17_baseline_guards.rs:58` (passes via `include_str!("../Cargo.toml")` parser) |
| 5 | The v1.7 E2E smoke asserts the immutable-enrichment contract on both helpers AND the CRIT-6 4-envelope evidence contract on `IChingCastSummary` AND the 8-cell + dual-source contract on `DirectionCrossLinkSummary` | ✓ VERIFIED | Test body at `integration_2026_smoke.rs:514-886` asserts: `snap.iching_cast.is_none()` after enrichment (line 636); `enriched_iching.direction_cross_link.is_none()` after cross-link enrichment (line 742); `enriched_both.iching_cast.is_some()` (line 749); exactly 4 evidence envelopes with 2 SOURCE_MAI_HOA_DICH_SO + 1 SOURCE_KINH_DICH + 1 composite (lines 647-690); exactly 8 cells (line 760); ≥3 envelopes with KHCBPPT + HUYEN_KHONG + composite (lines 768-794) |
| 6 | The v1.7 E2E smoke verifies the semantic-graph wiring: enriched snapshot yields STRICTLY more Hexagram + Direction nodes + Transforms edges than the un-enriched base snapshot | ✓ VERIFIED | Test body at `integration_2026_smoke.rs:808-884` asserts `hex_count > base_hex_count` (line 828), `transforms_count > base_transforms_count` (line 853), `direction_count > base_direction_count` (line 878) — proves enrichment adds facts, not pre-existing |

**Score:** 6/6 truths verified

### Required Artifacts

| Artifact | Expected | Status | Details |
| --- | --- | --- | --- |
| `crates/amlich-core/tests/integration_2026_smoke.rs` | Appended `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates` exercising ALL FIVE v1.7 surfaces together on Tết 2026 + ≥4 Sóc dates; ≥130 lines | ✓ VERIFIED | File exists, 886 lines total (470 pre-existing + 416 new); test fn at line 514; pre-existing 4 tests untouched; exercises cast_mai_hoa + derive_bien_que + classify_the_dung + enrich_day_snapshot_with_iching + enrich_day_snapshot_with_direction_cross_link + build_day_snapshot_graph |
| `crates/amlich-core/tests/v17_baseline_guards.rs` | NEW file with 2 tests: `cargo_dependency_tree_unchanged_from_v16` (SC4) + `int13_golden_dataset_cross_source_discipline_holds` (SC1); ≥80 lines | ✓ VERIFIED | File exists, 236 lines; Test 1 at line 58 parses `Cargo.toml` via `include_str!` and asserts exactly 4 deps with locked names; Test 2 at line 166 asserts ≥10 cases + ≥2 sources per case + nhantu.net in ≥1 + ≥1 KnownDivergence with non-empty fields + schema pin |

### Key Link Verification

| From | To | Via | Status | Details |
| --- | --- | --- | --- | --- |
| `integration_2026_smoke.rs` | `lib.rs::enrich_day_snapshot_with_iching` | Phase 24-01 immutable enrichment helper | ✓ WIRED | Imported + called at line 627 (`enrich_day_snapshot_with_iching(&snap, query)`) |
| `integration_2026_smoke.rs` | `lib.rs::enrich_day_snapshot_with_direction_cross_link` | Phase 23-03 immutable enrichment helper | ✓ WIRED | Imported + called at line 730 with `DATE_ONLY_BIRTH_CHI_INDEX` sentinel |
| `integration_2026_smoke.rs` | `iching/evaluator.rs::IChingQuery::from_snapshot` | Phase 24-01 sibling-newtype query constructor | ✓ WIRED | Imported + called at line 571 |
| `integration_2026_smoke.rs` | `iching/{mai_hoa,bien_que,the_dung}.rs` casting chain | Phase 22 IChing casting chain | ✓ WIRED | `cast_mai_hoa` line 582, `derive_bien_que` line 583, `classify_the_dung` line 584 |
| `integration_2026_smoke.rs` | `semantic_graph/builders/day_snapshot.rs::DaySnapshotGraphBuilder` | Phase 24-02 additive builder dispatch | ✓ WIRED | `build_day_snapshot_graph(&enriched_both)` at line 808 + base comparison at line 809 |
| `v17_baseline_guards.rs` | `crates/amlich-core/Cargo.toml` | `include_str!` + ad-hoc parser | ✓ WIRED | `const CARGO_TOML: &str = include_str!("../Cargo.toml");` at line 62; parser extracts + verifies `[dependencies]` section |
| `v17_baseline_guards.rs` | `iching/golden.rs::load_mai_hoa_golden` | Phase 22-02 golden dataset loader | ✓ WIRED | Imported at line 28; called at line 167; dataset struct fields accessed |

### Requirements Coverage

| Requirement | Source Plan | Description | Status | Evidence |
| --- | --- | --- | --- | --- |
| INT-13 | 25-01-PLAN | User-of-validation can find ≥10 IChing golden casting cases cross-checked against ≥2 independent sources + 2026 E2E smoke + full crate test suite green with zero regressions | ✓ SATISFIED | All 4 SCs met: SC1 via `int13_golden_dataset_cross_source_discipline_holds` + 12-case dataset; SC2 via `e2e_2026_smoke_v17_iching_and_cross_link_wiring_on_representative_dates`; SC3 via `cargo test --package amlich-core` (1120 passed, 0 failed); SC4 via `cargo_dependency_tree_unchanged_from_v16` + `cargo tree -p amlich-core --depth 1`. REQUIREMENTS.md line 36 marks INT-13 as `[x]` (Complete) |

**Orphaned Requirements Check:** Phase 25 maps to exactly one requirement (INT-13) per REQUIREMENTS.md line 83 ("Phase 25 (E2E Validation): INT-13"). No orphaned requirements.

### Anti-Patterns Found

| File | Line | Pattern | Severity | Impact |
| --- | --- | --- | --- | --- |
| (none) | — | — | — | No TODO/FIXME/HACK/PLACEHOLDER/placeholder/coming-soon patterns in either test file; no `return null`/`=> {}` stubs; no console-log-only handlers |

Both test files compile cleanly against the v1.6-baseline 4-dep tree and use only `amlich_core`'s public API + std library.

### Human Verification Required

None. All four success criteria are programmatically verifiable and have been exercised via `cargo test --package amlich-core` + `cargo tree -p amlich-core --depth 1` + direct inspection of the golden dataset JSON + test source files. No UI/UX/external-service surfaces involved (Phase 25 is pure validation).

### Gaps Summary

No gaps. All 6 observable truths verified, both required artifacts pass the three levels (exists, substantive, wired), all 7 key links wired, INT-13 fully satisfied with no orphaned requirements, no anti-patterns.

### Concrete Evidence Appendix

**Test result summary** (`cargo test --package amlich-core`):
- 50+ test binary targets, every result line reports `ok`
- Aggregate: 1120 passed, 0 failed, 7 ignored (6 pre-existing doc-test ignores in `almanac::na_am` + `almanac::sexagenary_cycle` modules + 1 in `integration_2026_smoke`)
- Delta vs Phase 24-end baseline (1117): +3 tests (1 new in `integration_2026_smoke.rs` + 2 new in `v17_baseline_guards.rs`) — matches SUMMARY claim

**Cargo dependency tree** (`cargo tree -p amlich-core --depth 1`):
```
amlich-core v0.1.4 (/home/noy/work/amlich/crates/amlich-core)
├── chrono v0.4.44
├── serde v1.0.228
├── serde_json v1.0.149
└── unicode-normalization v0.1.25
```

**Golden dataset** (`crates/amlich-core/data/iching/mai_hoa_golden.json`):
- `schema_version: "mai-hoa-golden-v1"` ✓
- 12 cases (`boundary-cr2-all-eights`, `non-boundary-one-one-one`, `nhantu-001`..`nhantu-010`) ✓ ≥10
- 28 total `"source":` entries (12 cases × 2 sources + 2 KnownDivergence × 2 sources) ✓ ≥2 per case
- 12 `"nhantu.net"` references ✓ canonical first reference present
- 2 KnownDivergence rows (`thap-can-tien-thien-arrangement-ly-vs-kien` + 1 more) ✓ divergences logged

**Task commits verified present:**
- `71d4c72` — test(25-01): add v1.7 E2E smoke for IChing + cross-link unified wiring
- `4921a1c` — test(25-01): add v17_baseline_guards (SC4 dep tree + SC1 golden dataset)
- `c741826` — docs(25-01): complete v1.7 E2E validation + golden cross-source verification plan

---

_Verified: 2026-07-20_
_Verifier: Claude (gsd-verifier)_
