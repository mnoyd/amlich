---
phase: 24
name: iching-evaluator-semantic-graph-wiring-dto-integration
status: passed
score: 4/4
verified_at: 2026-07-19T12:30:00Z
re_verification: false
---

# Phase 24: IChing Evaluator + Semantic-Graph Wiring + DTO Integration — Verification Report

**Phase Goal (from ROADMAP.md):**
> A caller can run an `IChingQuery` through an `IChingEvaluator` that emits per-step `ReasoningEvidenceEnvelope` instances (distinct source_ids + one composite), and a semantic-graph reader can find Hexagram nodes (chủ quẻ + biến quẻ) wired via `LocatedAt`/`Transforms` edges plus a composite cross-link fact node — both surfaced additively on `DaySnapshot` with v1.6→v1.7 backward-compat preserved.

**Verified:** 2026-07-19T12:30:00Z
**Status:** PASSED
**Re-verification:** No — initial verification.

---

## Must-Haves Verified

| # | Success Criterion | Evidence Path | Status |
|---|-------------------|---------------|--------|
| 1 | A caller can construct an `IChingQuery` (sibling newtype, NOT `ConsultationIntent::IChing`) + run through `IChingEvaluator` emitting per-step envelopes with distinct source_ids (`mai-hoa-dich-so` + `kinh-dich`) + 1 composite (`rule.composite.iching_consultation`); works at Tier 0 (no birth data) | `crates/amlich-core/src/iching/evaluator.rs:73-185` (`IChingQuery` struct + `from_snapshot` + `from_lunar_inputs`); `evaluator.rs:300-358` (`evaluate_consultation` delegating to Phase 22 surface); `evaluator.rs:418-465` (`build_evidence` emits exactly 4 envelopes: 2× `SOURCE_MAI_HOA_DICH_SO` + 1× `SOURCE_KINH_DICH` + 1× `COMPOSITE_ICHING_CONSULTATION` in `Derived` family); `evaluator.rs:467-495` (ActionEvaluator impl returns `Ok(ActionEvaluation::empty(ActionId::IChing))`, ignoring `personal_input` — MOD-7); `tests/iching_evaluator_integration.rs` 18/18 tests green including `iching_evaluator_emits_at_least_two_primitive_source_ids_plus_one_composite` + `iching_evaluator_works_at_tier_0_with_no_birth_data` | ✓ VERIFIED |
| 2 | A reader of the semantic graph can find Hexagram nodes (chủ quẻ + biến quẻ) wired via `LocatedAt`/`Transforms` edges, plus a composite cross-link fact node — emitted by additive `add_iching_facts()` + `add_direction_composite_facts()` builder methods | `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs:48-49` (dispatch wiring of both methods in `DaySnapshotGraphBuilder::new`); `day_snapshot.rs:780-883` (`add_iching_facts` emits 2 distinct Hexagram nodes with role-bearing stable keys + 1 Transforms edge chu→bien + 2 LocatedAt edges to day_root + CRIT-6 dual-source provenance); `day_snapshot.rs:899-966` (`add_direction_composite_facts` emits 1 Direction composite node with 3-provenance); `crates/amlich-core/src/semantic_graph/ids.rs:251-256` (`SemanticId::iching_hexagram(role, king_wen, date, tz)` → `"hexagram:iching:{role}:{king_wen}:{date}:{tz}"`); `tests/semantic_graph_iching_integration.rs` 13/13 tests green | ✓ VERIFIED |
| 3 | A reader of `DaySnapshot` can find additive `iching_cast: Option<IChingCastSummary>` and `direction_cross_link: Option<DirectionCrossLinkSummary>` fields (`#[serde(default, skip_serializing_if = "Option::is_none")]`) | `crates/amlich-core/src/lib.rs:195-201` (`direction_cross_link: Option<crate::reasoning::DirectionCrossLinkSummary>` with serde attrs — Phase 23-02 shipped the field before Plan 24-02); `lib.rs:202-210` (`iching_cast: Option<crate::iching::IChingCastSummary>` with identical serde attrs — Plan 24-01); `lib.rs:350-359` (`enrich_day_snapshot_with_iching` immutable clone-and-attach helper at crate root); ordinary `calculate_day_snapshot_internal` initialises both to `None` (no implicit population) | ✓ VERIFIED |
| 4 | A caller can deserialize a v1.6 `DaySnapshot` JSON (no `iching_cast` / `direction_cross_link` fields) into the v1.7 struct and re-serialize without unexpected fields (combined-strip v1.6→v1.7 round-trip) | `crates/amlich-core/tests/day_snapshot_v14_compat.rs:373-514` (Test 10 `v16_json_without_v17_iching_fields_deserializes_and_round_trips` — strips BOTH v1.7 fields together, deserialises, re-serialises, asserts semantic equality via `serde_json::Value` + no new keys + no `null`); `day_snapshot_v14_compat.rs:516-715` (Test 11 `v17_iching_cast_and_direction_cross_link_byte_equal_round_trip` — byte-equal round-trip with BOTH fields populated + CRIT-6 evidence contract + cross-link 8-cell shape); `day_snapshot_v14_compat.rs:725-748` (Test 12 `v17_iching_fields_absent_when_none` — both fields absent in JSON when None); 12/12 tests in file green | ✓ VERIFIED |

**Score:** 4/4 success criteria verified.

---

## Requirements Traceability

| Requirement | Source Plan | Status in REQUIREMENTS.md | Evidence |
|-------------|-------------|---------------------------|----------|
| **ICH-05** | 24-01-PLAN.md | Complete (line 24, traceability line 63) | `IChingQuery` sibling newtype + `IChingEvaluator` rich path + 4-envelope CRIT-6 evidence vector + Tier-0 ActionEvaluator adapter (MOD-7) + immutable `enrich_day_snapshot_with_iching` helper. 19 inline + 18 black-box integration tests green. Verified directly in `evaluator.rs`. |
| **INT-11** | 24-02-PLAN.md | Complete (line 34, traceability line 67) | `add_iching_facts` ships 2 distinct Hexagram nodes with role-bearing stable keys + 1 Transforms edge + 2 LocatedAt edges + dual-source provenance. `add_direction_composite_facts` ships Direction composite node with KHCBPPT + HUYEN_KHONG + composite envelopes. 13 black-box integration tests green. |
| **INT-12** | 24-01 + 24-03-PLAN.md | Complete (line 35, traceability line 68) | `iching_cast` additive field + helper landed in 24-01. `direction_cross_link` additive field landed earlier in Phase 23-02. Combined-strip v1.6→v1.7 round-trip tests (Tests 10-12) in 24-03 close the verification half. 12/12 `day_snapshot_v14_compat.rs` tests green. |

**Orphan requirements:** None. REQUIREMENTS.md explicitly maps ICH-05, INT-11, INT-12 to Phase 24 — all three accounted for and marked Complete.

---

## Test Suite Status

| Test Target | Result |
|-------------|--------|
| `cargo test -p amlich-core` (full crate) | **1117 passed, 0 failed, 7 ignored** (the 7 ignored are pre-existing doc-tests — `na_am`, `sexagenary_cycle` — unrelated to Phase 24) |
| `cargo test -p amlich-core --test iching_evaluator_integration` (24-01) | **18/18 pass** |
| `cargo test -p amlich-core --test semantic_graph_iching_integration` (24-02) | **13/13 pass** (incl. `direction_composite_facts_wires_populated_state` — ACTIVE, not `#[ignore]`'d, because Phase 23 shipped before Plan 24-02) |
| `cargo test -p amlich-core --test day_snapshot_v14_compat` (24-03) | **12/12 pass** (Tests 1-9 pre-existing + Tests 10-12 new for Phase 24-03) |
| `cargo test -p amlich-core --test source_id_guard` | **1/1 pass** |
| `cargo test -p amlich-core --test thai_tue_cross_link_crit3` | **3/3 pass** (CRIT-3 sibling guard unaffected) |

**Baseline comparison:**
- Plan 24-01 baseline: 1101 tests (claimed in 24-01-SUMMARY.md)
- Plan 24-02 baseline: 1114 tests (claimed in 24-02-SUMMARY.md) — Plan 24-01's 1101 + 13 new
- Plan 24-03 final: 1117 tests (claimed in 24-03-SUMMARY.md) — Plan 24-02's 1114 + 3 new
- Verified actual: 1117 passing tests — matches the claimed baseline progression (+16 net additions across Phase 24; 0 regressions).

---

## Discipline Checks

### CRIT-3 isolation (no `FlyingStar` in iching/ or cross-link code paths)

```bash
$ rg "FlyingStar" crates/amlich-core/src/iching/
# (zero matches)
```

✓ **CLEAN.** `crates/amlich-core/src/iching/` contains zero `FlyingStar` references. The grep guard test `crit3_isolation_no_cross_newtype_from_impls_in_evaluator` (inline + integration) also passes. `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` references `FlyingStar` ONLY in `add_flying_star_facts` (lines 499, 509, 519) + inline tests (lines 1189+) — never inside `add_iching_facts` (line 780) or `add_direction_composite_facts` (line 899), verified by direct file inspection.

### CRIT-6 source-id discipline (no bare literals at production call-sites)

```bash
$ cargo test -p amlich-core --test source_id_guard 2>&1 | tail -5
running 1 test
test no_bare_source_id_literals_in_production_src ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

✓ **CLEAN.** All production call-sites use the registered `SOURCE_KINH_DICH` + `SOURCE_MAI_HOA_DICH_SO` + `SOURCE_KHCBPPT` + `SOURCE_HUYEN_KHONG` + `COMPOSITE_ICHING_CONSULTATION` + `COMPOSITE_DIRECTION_CROSS_LINK` consts. The only literal `"kinh-dich"` / `"mai-hoa-dich-so"` mentions outside `sources.rs` are inside `#[cfg(test)]` blocks (test assertions + `ProvenanceEntry::iching(...)` test fixtures).

### WASM-safety + determinism (no RNG / wall-clock / filesystem)

```bash
$ rg "rand::|Utc::now|std::fs::" crates/amlich-core/src/iching/
# (zero matches)
```

✓ **CLEAN.** `crates/amlich-core/src/iching/` contains zero matches for `rand::`, `Utc::now`, or `std::fs::`. The grep guard test `wasm_safety_no_fs_no_utc_no_rand_in_evaluator` (inline + integration) also passes. Evaluator is deterministic — `iching_evaluator_is_deterministic` asserts two consecutive `evaluate_consultation` calls return equal values.

### No new crate dependencies

```bash
$ cargo tree -p amlich-core --depth 1
amlich-core v0.1.4 (/home/noy/work/amlich/crates/amlich-core)
├── chrono v0.4.44
├── serde v1.0.228
├── serde_json v1.0.149
└── unicode-normalization v0.1.25
```

✓ **UNCHANGED.** Same 4 dependencies as v1.6 baseline (chrono + serde + serde_json + unicode-normalization). No new crates introduced across Phase 24.

### No `#[ignore]`'d Phase 24 tests

```bash
$ rg "#\[ignore" crates/amlich-core/tests/
crates/amlich-core/tests/generate_golden.rs:#[ignore]
```

✓ **CLEAN.** Only 1 `#[ignore]` test exists in the entire crate — the pre-existing `generate_golden.rs` (unrelated to Phase 24). Critically, the plan-anticipated `#[ignore = "Phase 23 must ship first"]` for `direction_composite_facts_wires_populated_state` was NEVER ADDED because Phase 23 shipped before Plan 24-02 executed — the test is ACTIVE and passes (confirmed in the 13/13 `semantic_graph_iching_integration` run).

---

## Required Artifacts — All Verified (3 Levels)

| Artifact | Exists | Substantive | Wired | Status |
|----------|--------|-------------|-------|--------|
| `crates/amlich-core/src/iching/evaluator.rs` | ✓ (1083 lines) | ✓ All declared types present: `IChingQuery`, `IChingEvaluator`, `IChingEvaluation`, `IChingCastSummary`, `HexagramEntryProjection`, `COMPOSITE_ICHING_CONSULTATION` const + 19 inline tests + working `evaluate_consultation` + `build_evidence` | ✓ Phase 22 surface (`cast_mai_hoa`, `derive_bien_que`, `classify_the_dung`, `get_hexagram`) wired; ActionEvaluator trait impl present; immutable enrichment helper at crate root re-exports through `iching/mod.rs` | ✓ VERIFIED |
| `crates/amlich-core/src/iching/mod.rs` | ✓ | ✓ `pub mod evaluator;` + re-exports (`IChingQuery`, `IChingEvaluator`, `IChingCastSummary`, `IChingEvaluation`, `HexagramEntryProjection`, `COMPOSITE_ICHING_CONSULTATION`, `enrich_day_snapshot_with_iching`) | ✓ Reachable via `amlich_core::iching::*` import paths (Test 11 + iching_evaluator_integration confirm) | ✓ VERIFIED |
| `crates/amlich-core/src/lib.rs` | ✓ | ✓ `DaySnapshot.iching_cast` field with correct serde attrs (line 209-210); `DaySnapshot.direction_cross_link` field (line 200-201); `enrich_day_snapshot_with_iching` helper at crate root (line 350-359) | ✓ `calculate_day_snapshot_internal` initialises both fields to `None`; enrichment helpers immutable (test #9 asserts) | ✓ VERIFIED |
| `crates/amlich-core/src/semantic_graph/provenance.rs` | ✓ | ✓ `ProvenanceSource::IChing` variant + `ProvenanceEntry::iching()` constructor + `to_reasoning_evidence()` arm mapping to `Family::IChing` + 2 inline tests | ✓ Mapping exercised by `iching_provenance_source_maps_to_iching_family` integration test | ✓ VERIFIED |
| `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` | ✓ (1543 lines) | ✓ `add_iching_facts` (line 780) + `add_direction_composite_facts` (line 899) + dispatch wiring (lines 48-49) — both methods substantive with the full dual-source provenance + edge-insertion-order discipline | ✓ Builder consumes `iching_cast` + `direction_cross_link` additive fields; downstream tests verify Hexagram + Direction nodes appear in graph | ✓ VERIFIED |
| `crates/amlich-core/src/semantic_graph/ids.rs` | ✓ (263 lines) | ✓ `SemanticId::iching_hexagram(role, king_wen, date, tz)` constructor at line 251 produces role-bearing stable keys `"hexagram:iching:{role}:{king_wen}:{date}:{tz}"` | ✓ Used by `add_iching_facts` to build the two distinct Hexagram node ids; verified distinct by `iching_graph_hexagram_stable_keys_are_role_bearing` test | ✓ VERIFIED |
| `crates/amlich-core/tests/iching_evaluator_integration.rs` | ✓ (601 lines) | ✓ 18 black-box tests covering all 5 must-have truths for 24-01: query construction + NFC + Tier-0 + dual-source evidence + immutability + round-trip | ✓ All 18 tests pass | ✓ VERIFIED |
| `crates/amlich-core/tests/semantic_graph_iching_integration.rs` | ✓ (519 lines) | ✓ 13 black-box tests covering INT-11 success criteria: 2 Hexagram nodes + Transforms + LocatedAt edges + dual-source provenance + role-bearing keys + no implicit wiring + directional composite wiring | ✓ All 13 tests pass | ✓ VERIFIED |
| `crates/amlich-core/tests/day_snapshot_v14_compat.rs` | ✓ (748 lines) | ✓ 12 tests total — Tests 1-9 pre-existing (Phase 15 + 18-04 + 19-03) + Tests 10-12 NEW for Phase 24-03 (combined-strip v1.6→v1.7 round-trip) | ✓ All 12 tests pass | ✓ VERIFIED |

---

## Key Link Verification

| From | To | Via | Status |
|------|----|----|--------|
| `evaluator.rs` | `mai_hoa.rs::cast_mai_hoa` | Phase-22 cast surface (no reimplementation) | ✓ WIRED — `evaluator.rs:39, 306` |
| `evaluator.rs` | `bien_que.rs::derive_bien_que` | Phase-22 bien-que surface | ✓ WIRED — `evaluator.rs:39, 314` |
| `evaluator.rs` | `the_dung.rs::classify_the_dung` | Phase-22 the-dung surface | ✓ WIRED — `evaluator.rs:38, 316` |
| `evaluator.rs` | `corpus.rs::get_hexagram` | Phase-21 corpus lookup | ✓ WIRED — `evaluator.rs:39, 320, 326` |
| `evaluator.rs` | `sources.rs::SOURCE_*` consts | CRIT-6 source-id discipline | ✓ WIRED — `evaluator.rs:46, 430, 438, 447` |
| `evaluator.rs` | `provenance.rs::ProvenanceSource::IChing` | Graph provenance conversion | ✓ WIRED — `provenance.rs:22, 124`; test `iching_provenance_source_maps_to_iching_family` passes |
| `lib.rs::enrich_day_snapshot_with_iching` | `evaluator.rs::IChingEvaluator::evaluate` | Immutable clone-and-attach helper | ✓ WIRED — `lib.rs:354-355` |
| `iching/mod.rs` | `evaluator.rs` public surface | Re-exports for `amlich_core::iching::*` | ✓ WIRED — `iching/mod.rs` re-exports (Test 11 imports confirm) |
| `day_snapshot.rs::add_iching_facts` | `IChingCastSummary::chu_king_wen_index/bien_king_wen_index` | Builder consumes evaluator output (no recompute) | ✓ WIRED — `day_snapshot.rs:792-793` |
| `day_snapshot.rs::add_direction_composite_facts` | `DaySnapshot::direction_cross_link` | Phase 23 dependency | ✓ WIRED — `day_snapshot.rs:900`; test `direction_composite_facts_wires_populated_state` passes |
| `day_snapshot.rs::add_*_facts` | `sources.rs::SOURCE_*` consts | CRIT-6 source-id discipline | ✓ WIRED — `SOURCE_MAI_HOA_DICH_SO`/`SOURCE_KINH_DICH` (line 802, 810, 816, 824); `SOURCE_KHCBPPT`/`SOURCE_HUYEN_KHONG` (line 922, 930) |
| `day_snapshot_v14_compat.rs` Tests 10-12 | `DaySnapshot::iching_cast` + `direction_cross_link` | Combined-strip v1.6→v1.7 round-trip | ✓ WIRED — both fields exercised in Test 10 (strip + recover), Test 11 (populated byte-equal), Test 12 (None → absent) |

---

## Anti-Patterns Scan

| File | Pattern | Severity | Impact |
|------|---------|----------|--------|
| `evaluator.rs` | None — no TODO/FIXME/XXX/HACK/PLACEHOLDER | n/a | n/a |
| `day_snapshot.rs` builders | None — no TODO/FIXME/XXX/HACK/placeholder | n/a | n/a |
| `provenance.rs` | None | n/a | n/a |
| `lib.rs` (enrichment helper + DaySnapshot fields) | None | n/a | n/a |
| All test files | None — no `=> {}` empty handlers, no `return null` stubs, no `console.log`-only paths (Rust idiom-equivalent check) | n/a | n/a |

✓ **No anti-patterns found across Phase 24 files.**

---

## Commit Verification

All claimed Phase 24 commits present in `git log`:

| Commit | Type | Plan | Description |
|--------|------|------|-------------|
| `563cc27` | test | 24-01 RED | Failing tests for IChingQuery + IChingEvaluator + ProvenanceSource::IChing |
| `4ecd343` | feat | 24-01 GREEN | Implement IChingQuery + IChingEvaluator with per-step evidence envelopes |
| `8ea2373` | feat | 24-01 Task 2 | Wire DaySnapshot.iching_cast + immutable enrichment helper + integration tests |
| `278f4a5` | test | 24-02 RED | Failing tests for IChing semantic-graph wiring |
| `46ad421` | feat | 24-02 GREEN | Wire Hexagram semantic-graph facts for IChing (merged Task 1 + Task 2) |
| `8263348` | feat | 24-03 | v1.7 combined-strip round-trip tests for iching_cast and direction_cross_link |

---

## Human Verification Needed

None required. All success criteria are fully verifiable via:
- Cargo test invocations (green)
- Static `rg` checks (CRIT-3, CRIT-6, WASM-safety, no-new-deps all clean)
- Direct code inspection of the additive DTO fields, the immutable enrichment helper, the evidence-vector construction, and the builder methods

The Phase 24 surface is pure Rust algorithm + DTO + integration tests — no external services, no UI, no real-time behaviour to eyeball.

---

## Gaps

**None.** All 4 success criteria verified, all 3 requirements (ICH-05, INT-11, INT-12) marked Complete in REQUIREMENTS.md with matching evidence, all discipline checks (CRIT-3, CRIT-6, WASM-safety, determinism, no new deps, no ignored Phase 24 tests) clean, all 1117 crate tests pass with zero regressions vs the claimed Plan 24-02 baseline of 1114 (+3 = Plan 24-03's new round-trip tests).

---

## Summary

Phase 24 is **fully complete and verified**. The goal-backward audit confirms every claimed deliverable exists in the actual codebase (not just in the SUMMARYs):

- **Evaluator layer:** `IChingQuery` sibling newtype + `IChingEvaluator` rich path emitting the locked 4-envelope evidence vector (CRIT-6) + `ActionEvaluator` trait-shape adapter returning empty `ActionEvaluation` (MOD-7 Tier-0) — all in `crates/amlich-core/src/iching/evaluator.rs` (1083 lines).
- **Semantic-graph wiring:** `add_iching_facts` (2 Hexagram nodes + Transforms + LocatedAt edges + dual-source provenance) + `add_direction_composite_facts` (1 Direction composite node with KHCBPPT + HUYEN_KHONG + composite envelopes) — both wired into `DaySnapshotGraphBuilder::new` dispatch.
- **DTO integration:** Additive `DaySnapshot.iching_cast` + `direction_cross_link` fields with `#[serde(default, skip_serializing_if = "Option::is_none")]` discipline; immutable `enrich_day_snapshot_with_iching` helper at the crate root; combined-strip v1.6→v1.7 round-trip tests (Tests 10-12 in `day_snapshot_v14_compat.rs`) proving absence-preserving serialisation across both new fields.

All three Phase 24 requirements (ICH-05, INT-11, INT-12) are marked Complete in `.planning/REQUIREMENTS.md` with matching code evidence. Phase 25 (E2E Validation + INT-13) is unblocked.

---

_Verified: 2026-07-19T12:30:00Z_
_Verifier: Claude (gsd-verifier)_
_Methodology: Goal-backward verification — read SUMMARYs, then independently verified code at claimed paths + ran the full test suite + grep discipline checks. Did NOT trust SUMMARY claims._
