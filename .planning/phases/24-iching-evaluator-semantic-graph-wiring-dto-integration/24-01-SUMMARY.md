---
phase: 24-iching-evaluator-semantic-graph-wiring-dto-integration
plan: 01
subsystem: iching
tags: [iching, evaluator, kinh-dich, mai-hoa-dich-so, the-dung, day-snapshot, additive-dto, crit-3, crit-6, mod-7, wasm-safe, adr-0006, richard-whincup-free, nfc]

# Dependency graph
requires:
  - phase: 22-mai-hoa-casting-bien-que-the-dung
    provides: MaiHoaCast struct + cast_mai_hoa + MaiHoaCast::dong_hao + MaiHoaCast::chu_que; BienQue + derive_bien_que; TheDungClassification + classify_the_dung + TheDungRelation + CatHung + trigram_element; mai_hoa_golden.json 12-case cross-source dataset (Phase 22 SC4)
  - phase: 21-iching-corpus-loader
    provides: OnceLock-cached 64-hexagram Ngô Tất Tố corpus (HEXAGRAMS_JSON via include_str!) + get_hexagram(KingWenHexagram) + NFC-normalised vi_name / thoai_tu / hao_tu / cat_hung fields + hao_tu length invariant enforcement at load
  - phase: 20-foundation-schema-lock-source-ids-adrs-ontology
    provides: SOURCE_KINH_DICH + SOURCE_MAI_HOA_DICH_SO consts (FND-09); three CRIT-3-isolating newtypes (TienThienTrigram / HauThienTrigram / KingWenHexagram) with NO cross-From impls; ReasoningEvidenceSourceFamily::IChing + ActionId::IChing variants (FND-12)
  - phase: 23-th-i-tu-tam-s-t-phi-tinh-cross-link
    provides: enrich_day_snapshot_with_direction_cross_link immutable clone-and-attach helper pattern at crate root (mirrored by enrich_day_snapshot_with_iching); DaySnapshot.direction_cross_link additive field discipline; AGENTS.md / 23-03 SUMMARY.md for the pattern
provides:
  - "ICH-05 closed: IChingEvaluator + IChingQuery + IChingEvaluation + IChingCastSummary + HexagramEntryProjection shipped"
  - "Per-step evidence envelope ordering (3 primitive envelopes: cast_mai_hoa + derive_bien_que carrying SOURCE_MAI_HOA_DICH_SO; corpus_lookup carrying SOURCE_KINH_DICH) + 1 composite envelope carrying COMPOSITE_ICHING_CONSULTATION in Derived family (CRIT-6)"
  - "Tier-0 ActionEvaluator adapter: ActionEvaluator::evaluate returns Ok(empty ActionEvaluation) ignoring personal_input (MOD-7)"
  - "DaySnapshot.iching_cast: Option<IChingCastSummary> additive field with serde(default, skip_serializing_if = 'Option::is_none') discipline (partial INT-12)"
  - "enrich_day_snapshot_with_iching(&DaySnapshot, IChingQuery) -> Result<DaySnapshot, String> at the crate root: immutable clone-and-attach helper, input never mutated"
  - "ProvenanceSource::IChing variant + iching() constructor helper + ReasonEvidenceSourceFamily::IChing mapping via to_reasoning_evidence (extends semantic-graph provenance)"
  - "Reachable as both amlich_core::iching::enrich_day_snapshot_with_iching and amlich_core::enrich_day_snapshot_with_iching (mirrors planned Phase 24-03 consumer import path)"
affects:
  - 24-iching-evaluator-semantic-graph-wiring-dto-integration (future plans 24-02 + 24-03)
  - 25-e2e-validation-golden-cross-source-verification (closes INT-13)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "SIBLING-NEWTYPE query discipline: IChingQuery is a NEW newtype, NOT a ConsultationIntent::IChing variant. Mirrors the v1.6 DailyFlyingStarLayout sibling-to-FlyingStarLayout precedent and the v1.7 DirectionCrossLinkSummary sibling-to-DirectionCrossLink precedent"
    - "TDD RED → GREEN → integration-suite three-commit discipline: RED `563cc27` (15 inline tests fail with 'not implemented: RED phase: ...'); GREEN `4ecd343` (full implementation passes all 19 inline tests + 2 provenance tests); GREEN Task 2 `8ea2373` (17 black-box integration tests pass on first run + 1 bonus source_id_guard self-check)"
    - "Per-step evidence envelope construction (CRIT-6): 3 primitive envelopes (cast_mai_hoa + derive_bien_que carrying SOURCE_MAI_HOA_DICH_SO; corpus_lookup carrying SOURCE_KINH_DICH) + 1 composite envelope with source_id=COMPOSITE_ICHING_CONSULTATION and source_family=ReasoningEvidenceSourceFamily::Derived. The composite does NOT collapse the primitives"
    - "Owned DTO projection: HexagramEntryProjection + IChingCastSummary own all string fields (no &'static corpus refs) so DaySnapshot.iching_cast can be cloned/serialised independently of the OnceLock-cached corpus"
    - "ActionEvaluator trait-shape adapter as a thin no-op mapper: returns Ok(empty ActionEvaluation) for the IChing baseline; the rich IChingEvaluation lives behind evaluate_consultation, NOT collapsed into the generic trait shape (per 24-CONTEXT.md Claude's Discretion §1)"
    - "Immutable clone-and-attach enrichment at the crate root: input snapshot is never mutated; ordinary calculate_day_snapshot calls leave additive fields as None. Mirrors Phase 23-03's enrich_day_snapshot_with_direction_cross_link"
    - "TienThienTrigram-only evaluator (no HauThienTrigram internal access): cast carries TienThienTrigram identities (same newtype as the Phase 22 surface); corpus projection is a projection via KingWenHexagram index only — the HauThien arrangement is never named internally (CRIT-3 isolation at the trigram identity boundary)"

key-files:
  created:
    - crates/amlich-core/src/iching/evaluator.rs
    - crates/amlich-core/tests/iching_evaluator_integration.rs
  modified:
    - crates/amlich-core/src/iching/mod.rs
    - crates/amlich-core/src/semantic_graph/provenance.rs
    - crates/amlich-core/src/lib.rs

key-decisions:
  - "SIBLING-NEWTYPE query (ICH-05): IChingQuery is a NEW struct carrying chi_hour_index + question_vi + lunar_year_branch + lunar_month + lunar_day. NOT a ConsultationIntent::IChing variant (per 24-CONTEXT.md locked decision + the ~25-43 call-site Copy-break churn rationale from .planning/research/ARCHITECTURE.md:436-440). Mirrors the v1.6 DailyFlyingStarLayout sibling-to-FlyingStarLayout + v1.7 DirectionCrossLinkSummary sibling-to-DirectionCrossLink precedent."
  - "Locked four-evidence-vector shape: 3 primitive envelopes (cast_mai_hoa + derive_bien_que carrying SOURCE_MAI_HOA_DICH_SO; corpus_lookup carrying SOURCE_KINH_DICH) + 1 composite envelope (source_id=COMPOSITE_ICHING_CONSULTATION, source_family=ReasoningEvidenceSourceFamily::Derived, method='iching_consultation'). The composite does NOT collapse the primitives — every step in the derivation remains individually traceable. CRIT-6 source-id discipline: every production call-site uses the registered SOURCE_* consts + the COMPOSITE_ICHING_CONSULTATION named const; no bare string literals"
  - "Tier-0 ActionEvaluator adapter: ActionEvaluator::evaluate returns Ok(ActionEvaluation::empty(ActionId::IChing)) unconditionally, ignoring personal_input. The rich IChingEvaluation lives behind IChingEvaluator::evaluate_consultation (NOT collapsed into the generic trait shape per 24-CONTEXT.md Claude's Discretion §1: 'Prefer clear domain types and a compound result struct over ambiguous primitive/boolean parameters. The rich I Ching result must not be reduced to the generic ActionEvaluation shape')"
  - "select_subgraph returns the full graph clone (no subgraph filtering). The IChing facts are added by DaySnapshotGraphBuilder (out of scope for this plan, planned for Plan 24-02) and consumed wholesale by the evaluator. Mirrors InitiationOpeningEvaluator::select_subgraph"
  - "Owned DTO projection everywhere: HexagramEntryProjection owns vi_name / thoai_tu / hao_tu / cat_hung strings; IChingCastSummary owns chu_hexagram_vi_name / chu_hexagram_thoai_tu / bien_hexagram_vi_name / bien_hexagram_thoai_tu / question_vi / evidence. No &'static HexagramEntry references — the snapshot stays self-contained once cloned, even if the OnceLock-cached corpus is dropped/refreshed"
  - "Inline nfc() helper (mirrors corpus.rs:163-169 + rituals/corpus.rs byte shape): keeps the evaluator module independent of the corpus loader while staying byte-identical. RIT-08 NFC normalisation applied to question_vi at IChingQuery::from_snapshot construction time. Whitespace-only question normalised to None"
  - "cat_hung_str helper: stable lowercase string projection of CatHung ('cat' / 'binh' / 'hung') for the IChingCastSummary.cat_hung_summary field. The cast already carries CatHung enum; the projection lives next to it"
  - "Crate-root + iching-namespace dual re-export: enrich_day_snapshot_with_iching lives at lib.rs (canonical site) and is re-exported from iching/mod.rs so callers using the planned Phase 24-03 import path 'use amlich_core::iching::{enrich_day_snapshot_with_iching, IChingQuery}' resolve cleanly. Both surfaces coexist (mirrors 23-03's direction_cross_link pattern)"
  - "ProvenanceSource::IChing added as additive-safe variant (between Insight and Derived — preserves existing match-arm order from Phase 15; compiler-enforced). Only constructed (never matched at the public graph surface); to_reasoning_evidence() maps it to ReasoningEvidenceSourceFamily::IChing via a new explicit match arm. The ProvenanceEntry::iching() constructor helper mirrors the existing almanac_rule / derived / snapshot / interaction / bazi helpers"
  - "Validation: chi_hour_index 0..=11 + question_vi whitespace-strip + NFC normalisation in IChingQuery::from_snapshot AND IChingQuery::from_lunar_inputs. The latter validates all four input ranges (year_branch 0..=11, month 1..=12, day 1..=30, hour 0..=11). Out-of-range values return Err with a message that names the field"
  - "Missing hexagram guard: get_hexagram returns Option<&'static HexagramEntry>; if None the evaluator returns Err('missing hexagram entry: <index> (contract violation)'). Unreachable per bijectivity, but contract-violation-safe (mirrors compose()'s 'Unreachable' panic shape)"

patterns-established:
  - "Per-step evidence envelope construction (CRIT-6): name the method dispatch strings ('cast_mai_hoa' / 'derive_bien_que' / 'corpus_lookup' / 'iching_consultation'), thread inputs into the note field, use named consts for source_ids — never bare string literals at production call-sites"
  - "TienThienTrigram-only evaluator (CRIT-3 at the trigram-identity boundary): never name HauThienTrigram inside the evaluator; cast carries TienThienTrigram identities; corpus projection is reached only via the KingWen index — the HauThien arrangement is the corpus's display concern, not the evaluator's"
  - "ActionEvaluator as a thin no-op mapper for additive-safe new variants (MOD-7 + the 24-CONTEXT.md 'prefer clear domain types' rule): the trait-shape adapter handles the wiring (so dispatching code stays uniform); the rich path lives behind a domain-named method (so the rich result is reachable in full)"
  - "Immutable clone-and-attach enrichment: input snapshot is never mutated; ordinary calculate_day_snapshot calls leave additive fields as None; only an explicit enrichment helper call populates the field. Mirrors Phase 23-03's enrich_day_snapshot_with_direction_cross_link"

requirements-completed: [ICH-05]

# Metrics
duration: 10min
completed: 2026-07-16
---
# Phase 24 Plan 01: IChing Evaluator + DaySnapshot integration Summary

**Tier-0 I Ching evaluator (`IChingQuery` sibling newtype + `IChingEvaluator` + compound `IChingEvaluation` + slim owned `IChingCastSummary` DTO + per-step `ReasoningEvidenceEnvelope` provenance: 3 primitive envelopes carrying `SOURCE_MAI_HOA_DICH_SO` + `SOURCE_KINH_DICH` plus 1 composite envelope `rule.composite.iching_consultation` in the Derived family) wired into the existing Phase 22 cast + biến-quẻ + thể-dụng surface, with the additive `DaySnapshot.iching_cast` field + immutable `enrich_day_snapshot_with_iching` helper. Closes ICH-05 + lays the partial INT-12 surface (one of two additive `DaySnapshot` fields; the `direction_cross_link` companion lands in Plan 24-03).**

## Performance

- **Duration:** 10 min 16s (616s)
- **Started:** 2026-07-16T17:31:21Z
- **Completed:** 2026-07-16T17:41:37Z
- **Tasks:** 2
- **Task commits:** 3 (RED `563cc27` + GREEN `4ecd343` + GREEN task 2 `8ea2373`)
- **Files created:** 2 (`evaluator.rs`, `iching_evaluator_integration.rs`)
- **Files modified:** 3 (`iching/mod.rs`, `lib.rs`, `semantic_graph/provenance.rs`)
- **Net tests added:** 41 (19 inline + 18 integration + 2 + 2 new provenance tests → wait, recount: 21 inline tests in evaluator.rs (19 main + 2 inherited) + 18 integration tests + 2 provenance tests = 41 net additions)
- **Crate test suite:** 1101 passing tests across 48 test groups, 0 failures, 0 regressions vs Phase 23-03's 1062 baseline (+39 net additions)

## Accomplishments

- **`crates/amlich-core/src/iching/evaluator.rs`** (~470 lines) — Tier-0 I Ching evaluator module:
  - `pub const COMPOSITE_ICHING_CONSULTATION: &str = "rule.composite.iching_consultation"` — named const (single audit point for CRIT-6; mirrors `COMPOSITE_DIRECTION_CROSS_LINK`'s discipline)
  - `pub struct IChingQuery { pub chi_hour_index: u8, pub question_vi: Option<String>, pub lunar_year_branch: u8, pub lunar_month: u8, pub lunar_day: u8 }` — sibling newtype (NOT a `ConsultationIntent::IChing` variant; ~25-43 call-site `Copy`-break churn rationale from `.planning/research/ARCHITECTURE.md:436-440`)
  - `IChingQuery::from_snapshot(&DaySnapshot, Option<String>, u8)` — derives lunar inputs from `snapshot.context.canchi.year.chi_index` + `snapshot.context.lunar.{month,day}`; validates `chi_hour_index` ∈ `0..=11`; NFC-normalises `question_vi`; normalises whitespace-only to `None`
  - `IChingQuery::from_lunar_inputs(...)` — direct constructor for golden tests / boundary checks; validates all four input ranges (`year_branch` ∈ `0..=11`, `month` ∈ `1..=12`, `day` ∈ `1..=30`, `hour` ∈ `0..=11`); returns `Err` on out-of-range
  - `pub struct HexagramEntryProjection { pub king_wen_index: KingWenHexagram, pub vi_name: String, pub thoai_tu: String, pub hao_tu: Vec<String>, pub cat_hung: String }` — owned DTO (no `&'static HexagramEntry` lifetime entanglement)
  - `pub struct IChingEvaluation { query, cast, bien_que, the_dung, chu_hexagram, bien_hexagram, evidence }` — compound rich result; carries every intermediate
  - `pub struct IChingCastSummary { cast, bien_que, the_dung, chu_hexagram_vi_name, chu_hexagram_thoai_tu, bien_hexagram_vi_name, bien_hexagram_thoai_tu, cat_hung_summary, moving_line, question_vi, evidence }` — slim owned DTO for `DaySnapshot.iching_cast`
  - `pub struct IChingEvaluator { query: IChingQuery }` — Tier-0 evaluator (NO RNG, NO wall-clock, NO filesystem, NO Bazi, NO birth data)
  - `IChingEvaluator::evaluate_consultation(&DaySnapshot) -> Result<IChingEvaluation, String>` — composes Phase 22 surface (`cast_mai_hoa` + `derive_bien_que` + `classify_the_dung`) + Phase 21 corpus lookup (`get_hexagram` × 2); projects corpus entries into owned `HexagramEntryProjection`s; builds the 4-envelope evidence vector (3 primitive + 1 composite); returns `Err` on missing hexagram entry (contract-violation guard)
  - `IChingEvaluator::to_summary(&IChingEvaluation) -> IChingCastSummary` — projects the rich result into the slim owned DTO; owned strings throughout
  - `IChingEvaluator::evaluate(&DaySnapshot) -> Result<IChingCastSummary, String>` — convenience: evaluate + project to summary
  - `impl ActionEvaluator for IChingEvaluator` — trait-shape adapter: `action_id()` returns `ActionId::IChing`; `select_subgraph` returns the full graph clone (no subgraph filtering; mirrors `InitiationOpeningEvaluator`); `evaluate` returns `Ok(ActionEvaluation::empty(ActionId::IChing))` ignoring `personal_input` (Tier-0; MOD-7)
  - `fn build_evidence(...)` — locked 4-envelope construction (CRIT-6 source-id discipline): `cast_mai_hoa` + `derive_bien_que` carrying `SOURCE_MAI_HOA_DICH_SO`; `corpus_lookup` carrying `SOURCE_KINH_DICH`; composite carrying `COMPOSITE_ICHING_CONSULTATION` in `Derived` family. The composite does NOT collapse the primitives — every step in the derivation remains individually traceable
  - 19 inline tests (4 const-marker/guard tests + 15 behaviour tests covering all 5 must-have truths)
- **`crates/amlich-core/src/iching/mod.rs`** (modified) — registers `pub mod evaluator;` + re-exports `IChingCastSummary`, `IChingEvaluation`, `IChingEvaluator`, `IChingQuery`, `HexagramEntryProjection`, `COMPOSITE_ICHING_CONSULTATION`, and the `enrich_day_snapshot_with_iching` re-export from the crate root
- **`crates/amlich-core/src/semantic_graph/provenance.rs`** (modified) — extends the graph provenance surface:
  - `pub enum ProvenanceSource { ... IChing }` (additive-safe variant between `Insight` and `Derived`; preserves the existing match-arm order from Phase 15; compiler-enforced)
  - `impl ProvenanceEntry { pub fn iching(source_id, method) -> Self { ... } }` — constructor helper (mirrors `almanac_rule` / `derived` / `snapshot` / etc.)
  - `ProvenanceEntry::to_reasoning_evidence()` — adds the `ProvenanceSource::IChing => ReasoningEvidenceSourceFamily::IChing` match arm
  - 2 inline tests: `to_reasoning_evidence_maps_iching_to_iching_family` + `to_reasoning_evidence_preserves_all_existing_match_arms` (the second test pins the locked mapping for every existing variant as well, guarding against silent drift)
- **`crates/amlich-core/src/lib.rs`** (modified) — DaySnapshot + enrichment helper:
  - `pub struct DaySnapshot { ... }` — additive `pub iching_cast: Option<crate::iching::IChingCastSummary>` field with `#[serde(default, skip_serializing_if = "Option::is_none")]` discipline (partial INT-12; full close in Plan 24-03)
  - `calculate_day_snapshot_internal` — initialises `iching_cast: None` in the constructor literal (mirrors how `flying_stars` / `applicable_rituals` / `daily_flying_stars` / `offering_refs` / `offerings` / `direction_cross_link` are initialised — no auto-population from any almanac surface)
  - `pub fn enrich_day_snapshot_with_iching(snapshot: &DaySnapshot, query: crate::iching::IChingQuery) -> Result<DaySnapshot, String>` — immutable clone-and-attach helper at the crate root: clones the snapshot, runs `IChingEvaluator::evaluate`, attaches the resulting summary to the new snapshot, returns the new snapshot. The input snapshot is never mutated. Mirrors `enrich_day_snapshot_with_direction_cross_link`'s discipline
- **`crates/amlich-core/tests/iching_evaluator_integration.rs`** (NEW, 18 tests) — black-box integration tests from the external crate path:
  - 1-4: IChingQuery construction (`from_snapshot_derives_lunar_inputs`, `rejects_invalid_hour_index`, `nfc_normalises_question`, `rejects_whitespace_only_question`)
  - 5-8: IChingEvaluator rich path (`emits_at_least_two_primitive_source_ids_plus_one_composite`, `uses_phase_22_surface`, `is_deterministic`, `works_at_tier_0_with_no_birth_data`)
  - 9-14: DaySnapshot + enrichment (`does_not_mutate_input`, `populates_owned_summary`, `ordinary_day_snapshot_has_iching_cast_none`, `ordinary_day_snapshot_does_not_serialize_iching_cast_key`, `iching_cast_byte_equal_round_trip`, `iching_cast_absent_in_json_when_none`)
  - 15: `iching_provenance_source_maps_to_iching_family` (external-crate path test for the new `ProvenanceSource::IChing` mapping)
  - 16-17: `crit3_isolation_no_cross_newtype_from_impls_in_evaluator` + `wasm_safety_no_fs_no_utc_no_rand_in_evaluator` (runtime-built needle grep guards; CRIT-3 + WASM-safety preservation)
  - 18: bonus `source_id_guard_passes_for_new_evaluator_module` (verifies the new evaluator respects the bare-literal source-id discipline via the same pattern as `tests/source_id_guard.rs`)
- **TDD discipline observed:** RED commit `563cc27` (15 inline tests fail with "RED phase: not implemented" + 4 invariant tests pass + 2 provenance tests pass); GREEN commit `4ecd343` (full implementation + re-exports pass all 19 inline tests + 2 provenance tests); GREEN task 2 commit `8ea2373` (DaySnapshot field + enrichment helper + 18 integration tests pass on first run + 0 source_id_guard regressions). Three commits in order, 10 min 16 s total
- **CRIT-3 isolation preserved across the new module:** `rg "impl From<(Tien|Hau|KingWen)" crates/amlich-core/src/iching/evaluator.rs` returns zero matches; `rg "FlyingStar|direction_merge" crates/amlich-core/src/iching/evaluator.rs` returns zero matches; the doc-comments deliberately avoid mentioning TienThienTrigram / HauThienTrigram by their full names to keep the runtime-built needle grep guard from self-tripping (mirrors the 22-01 / 22-02 discipline codified across `corpus.rs` / `mai_hoa.rs` / `bien_que.rs` / `the_dung.rs` / `golden.rs`)
- **WASM-safety + determinism discipline preserved:** `rg "rand::|Utc::now|std::fs::" crates/amlich-core/src/iching/evaluator.rs` returns zero matches; the doc-comments deliberately avoid mentioning these patterns in literal form to keep the runtime-built needle grep guard from self-tripping. The initial doc-comment that mentioned these patterns was scrubbed during RED-phase test passes (a single Rule 1 deviation, fixed before the RED commit shipped)
- **Bare source-id literal discipline preserved:** `tests/source_id_guard.rs` still passes — the only literal `"kinh-dich"` / `"mai-hoa-dich-so"` mentions outside `sources.rs` are inside `#[cfg(test)]` blocks (test assertions + the new `ProvenanceEntry::iching(...)` test fixture in `provenance.rs`). The production evaluation code uses the registered `SOURCE_KINH_DICH` / `SOURCE_MAI_HOA_DICH_SO` / `COMPOSITE_ICHING_CONSULTATION` consts
- **No new crate dependencies:** existing `serde` + `serde_json` + `chrono` + `unicode-normalization` cover every need; `Cargo.toml` is unchanged
- **Full crate test result:** 1101 passing tests across 48 test groups, zero failures, zero regressions vs Phase 23-03's 1062-test baseline (+39 net additions = 21 inline + 18 integration). `cargo tree -p amlich-core --depth 1` shows no new dependencies
- **ICH-05 closed** in REQUIREMENTS.md. INT-12 is partially closed here (the `iching_cast` field + the `enrich_day_snapshot_with_iching` helper are in); the full INT-12 close-out — combined-strip v1.6→v1.7 round-trip including `direction_cross_link` — ships in Plan 24-03

## Task Commits

Each task was committed atomically (TDD on Task 1 produced the conventional RED → GREEN pair):

1. **Task 1 RED — failing inline tests for IChingQuery + IChingEvaluator + ProvenanceSource::IChing** — `563cc27` (test)
   - `crates/amlich-core/src/iching/evaluator.rs` (created, ~430 lines) — full type definitions (IChingQuery + IChingEvaluator + IChingEvaluation + IChingCastSummary + HexagramEntryProjection + COMPOSITE_ICHING_CONSULTATION const) + ActionEvaluator trait-shape adapter returning empty ActionEvaluation for the personal-input no-op path + 19 inline tests with stub methods returning `unimplemented!("RED phase: ...")`
   - `crates/amlich-core/src/iching/mod.rs` — registers `pub mod evaluator;` (no re-exports yet; land in GREEN)
   - `crates/amlich-core/src/semantic_graph/provenance.rs` — adds `ProvenanceSource::IChing` variant (between Insight and Derived — preserves the existing match-arm order) + `to_reasoning_evidence()` arm mapping IChing → IChing + `ProvenanceEntry::iching(source_id, method)` constructor helper + 2 inline tests (mapping + existing-arms preservation)
   - 15 of 19 inline tests fail with "not implemented: RED phase: from_snapshot / from_lunar_inputs / evaluate_consultation / to_summary / evaluate"; 4 invariant tests pass (const declared, CRIT-3 grep guard clean, WASM-safety grep guard clean after one Rule 1 doc-comment scrub, `cat_hung_str` projection); the 2 new provenance tests pass (mapping exists in RED because the variant + arm are added at the same time per the additive-safe discipline)
2. **Task 1 GREEN — implement IChingQuery + IChingEvaluator with per-step evidence envelopes** — `4ecd343` (feat)
   - `crates/amlich-core/src/iching/evaluator.rs` — full implementation: `IChingQuery::from_snapshot` + `from_lunar_inputs` (validation + NFC normalisation), `IChingEvaluator::evaluate_consultation` (composes cast_mai_hoa + derive_bien_que + classify_the_dung + 2× get_hexagram + project_hexagram + build_evidence), `IChingEvaluator::to_summary` (rich→slim projection), `IChingEvaluator::evaluate` (convenience), `build_evidence` (4-envelope construction)
   - `crates/amlich-core/src/iching/mod.rs` — registers the re-exports for the new public surface
   - 19/19 inline tests pass; CRIT-3 grep guard clean; WASM-safety grep guard clean. One test (`iching_query_nfc_normalises_question`) was simplified during GREEN to use a single-grave-mark NFD input that clearly verifies U+00EC precomposed ì is present after NFC (the original two-grave + precomposed-ê + combining-circumflex case relies on Unicode composition quirks out of scope for the contract test)
3. **Task 2 — DaySnapshot.iching_cast + enrich_day_snapshot_with_iching + 18-test black-box integration suite** — `8ea2373` (feat)
   - `crates/amlich-core/src/lib.rs` — `DaySnapshot.iching_cast: Option<crate::iching::IChingCastSummary>` additive field with serde(default, skip_serializing_if = "Option::is_none") discipline; initialised to None in `calculate_day_snapshot_internal`; `enrich_day_snapshot_with_iching(snapshot, query) -> Result<DaySnapshot, String>` at the crate root (immutable clone-and-attach)
   - `crates/amlich-core/src/iching/mod.rs` — re-exports `enrich_day_snapshot_with_iching` from the crate root so callers using the planned Phase 24-03 import path `use amlich_core::iching::{enrich_day_snapshot_with_iching, IChingQuery}` resolve cleanly
   - `crates/amlich-core/tests/iching_evaluator_integration.rs` (created, ~470 lines) — 18 black-box integration tests (17 from the plan + 1 bonus `source_id_guard_passes_for_new_evaluator_module`)
   - All 18 integration tests pass; full crate suite green; zero regressions

## Files Created/Modified

- `crates/amlich-core/src/iching/evaluator.rs` (created, ~470 lines) — full evaluator module + 19 inline tests
- `crates/amlich-core/tests/iching_evaluator_integration.rs` (created, ~470 lines) — 18 black-box integration tests
- `crates/amlich-core/src/iching/mod.rs` (modified, 27 → 33 lines) — registers `pub mod evaluator;` + re-exports the new public surface + re-exports `enrich_day_snapshot_with_iching` from the crate root
- `crates/amlich-core/src/semantic_graph/provenance.rs` (modified, 153 → 230 lines) — `ProvenanceSource::IChing` variant + `iching()` constructor + `to_reasoning_evidence()` arm + 2 inline tests
- `crates/amlich-core/src/lib.rs` (modified, 703 → 743 lines) — `DaySnapshot.iching_cast` additive field + `calculate_day_snapshot_internal` initialiser + `enrich_day_snapshot_with_iching` crate-root helper

## Decisions Made

- **Sibling-newtype query (locked, non-negotiable per 24-CONTEXT.md):** `IChingQuery` is a NEW struct carrying `chi_hour_index` + optional `question_vi` (NFC-normalised) + `lunar_year_branch` + `lunar_month` + `lunar_day`. NOT a `ConsultationIntent::IChing` variant. Adding the variant would force ~25-43 call-site `Copy`-break churn across the codebase (per `.planning/research/ARCHITECTURE.md:436-440` rationale). Mirrors the v1.6 `DailyFlyingStarLayout` sibling-to-`FlyingStarLayout` precedent + the v1.7 `DirectionCrossLinkSummary` sibling-to-`DirectionCrossLink` precedent
- **Locked 4-envelope evidence vector (CRIT-6):** 3 primitive envelopes carrying `SOURCE_MAI_HOA_DICH_SO` (cast_mai_hoa + derive_bien_que) and `SOURCE_KINH_DICH` (corpus_lookup) + 1 composite envelope with `source_id = COMPOSITE_ICHING_CONSULTATION` and `source_family = ReasoningEvidenceSourceFamily::Derived`. The composite does NOT collapse the primitives — every step in the derivation remains individually traceable. Method strings (`cast_mai_hoa` / `derive_bien_que` / `corpus_lookup` / `iching_consultation`) name the dispatch step + thread inputs into the `note` field (`"lunar_year_branch=X;month=Y;day=Z;hour=W"` etc.). Every production call-site uses the named const `COMPOSITE_ICHING_CONSULTATION` — never the bare literal
- **ActionEvaluator trait-shape adapter as a thin no-op mapper (24-CONTEXT.md Claude's Discretion §1 + MOD-7):** `ActionEvaluator::evaluate` returns `Ok(ActionEvaluation::empty(ActionId::IChing))` ignoring `personal_input`. The rich `IChingEvaluation` lives behind `IChingEvaluator::evaluate_consultation`, NOT collapsed into the generic trait shape. Test #8 (`iching_evaluator_works_at_tier_0_with_no_birth_data`) constructs an `IChingEvaluator`, calls `evaluate(graph, snap, None)` AND `evaluate(graph, snap, Some(&personal_input))`, and asserts both calls return `Ok(empty ActionEvaluation)` with identical `action_id` / `bucket` / `primary_conclusion` fields — proving `personal_input` is a no-op
- **`select_subgraph` returns the full graph clone:** the IChing facts are added by `DaySnapshotGraphBuilder` (out of scope for this plan, planned for Plan 24-02) and consumed wholesale by the evaluator. Mirrors `InitiationOpeningEvaluator::select_subgraph` (no subgraph filtering)
- **Owned DTO projection everywhere:** `HexagramEntryProjection` owns `vi_name` / `thoai_tu` / `hao_tu` / `cat_hung` strings; `IChingCastSummary` owns `chu_hexagram_vi_name` / `chu_hexagram_thoai_tu` / `bien_hexagram_vi_name` / `bien_hexagram_thoai_tu` / `question_vi` / `evidence`. No `&'static HexagramEntry` references — the snapshot stays self-contained once cloned, even if the `OnceLock`-cached corpus is dropped/refreshed. This is the "Owned DTO data" constraint from 24-01-PLAN.md's `<critical_constraints>`
- **Inline `nfc()` helper (mirrors `corpus.rs:163-169` byte shape):** keeps the evaluator module independent of the corpus loader while staying byte-identical to the proven RIT-08 NFC normalisation. RIT-08 NFC normalisation applied to `question_vi` at `IChingQuery::from_snapshot` construction time. Whitespace-only question normalised to `None` (consistent with the "whitespace-only signals intent to ignore" semantic the plan locks)
- **`cat_hung_str` helper (module-pub-crate):** stable lowercase string projection of `CatHung` (`'cat'` / `'binh'` / `'hung'`) for the `IChingCastSummary.cat_hung_summary` field. The cast already carries `CatHung` enum; the projection is a small bridge that keeps the DTO self-contained
- **Crate-root + iching-namespace dual re-export:** `enrich_day_snapshot_with_iching` lives at `lib.rs` (canonical public-API site, mirrors `enrich_day_snapshot_with_direction_cross_link`'s placement at lines 304-320). Re-exported from `iching/mod.rs` so callers using the planned Phase 24-03 import path (`use amlich_core::iching::{enrich_day_snapshot_with_iching, IChingQuery}`) resolve cleanly. Both surfaces coexist (mirrors 23-03's `direction_cross_link` pattern where the helper sits at the crate root but the type lives behind `reasoning::`)
- **`ProvenanceSource::IChing` as additive-safe variant (between `Insight` and `Derived` — preserves the existing match-arm order from Phase 15; compiler-enforced):** only constructed at the public graph surface (never matched elsewhere); `to_reasoning_evidence()` maps it to `ReasoningEvidenceSourceFamily::IChing` via a new explicit match arm. The `ProvenanceEntry::iching(source_id, method)` constructor helper mirrors the existing `almanac_rule` / `derived` / `snapshot` / `interaction` / `bazi` helpers. The `to_reasoning_evidence_preserves_all_existing_match_arms` test pins the locked mapping for every existing variant as well (defensive contract test against silent drift)
- **Input validation policy:** `chi_hour_index` ∈ `0..=11` required, `question_vi` whitespace-stripped + NFC-normalised at construction. `IChingQuery::from_lunar_inputs` validates all four input ranges (`year_branch` ∈ `0..=11`, `month` ∈ `1..=12`, `day` ∈ `1..=30`, `hour` ∈ `0..=11`); out-of-range returns `Err` with a message naming the field. `IChingQuery::from_snapshot` validates `chi_hour_index` (the lunar fields are derived from the snapshot and presumed valid by construction per the Vietnamese lunar conversion in `almanac::lunar`)
- **Missing hexagram guard:** `get_hexagram` returns `Option<&'static HexagramEntry>`; if `None` the evaluator returns `Err("missing hexagram entry: <index> (contract violation)")`. Unreachable per bijectivity (the corpus covers all 64 King Wen indices), but contract-violation-safe (mirrors `compose()`'s "Unreachable" panic shape)
- **Source-id naming discipline:** the `note` field of each evidence envelope embeds the inputs in a stable textual shape (`"lunar_year_branch=X;month=Y;day=Z;hour=W"` / `"dong_hao=N;bien_que_king_wen=M"` / `"chu_king_wen=N;bien_king_wen=M;verdict=Cat"`). DOWNSTREAM readers can reconstruct the call-site context from the envelope alone (no separate log lookup)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Doc-comments contained literal `std::fs::` / `rand::` / `Utc::now` substrings that the runtime-built-needle WASM-safety grep guard found**

- **Found during:** Task 1 RED compilation verification (`cargo test -p amlich-core --lib iching::evaluator`)
- **Issue:** Initial doc-comments used the literal patterns `rand::`, `Utc::now`, `std::fs::` (with backticks) when describing what the guard forbids. The runtime-built needle `String::from("std::f"); fs.push('s');` builds `"std::fs"` and includes the source file's text via `include_str!("evaluator.rs")` — the doc-comments' `std::fs::` literal substring matched the needle. Same trap as Phase 22-02 (golden.rs) — bare-substring grep self-trips on rationale text
- **Fix:** Rewrote the doc-comments to use phrase-level names ("no filesystem access, no wall-clock, no RNG" / "forbidden patterns by NAME") rather than the literal forbidden strings. Mirrors the 22-02 / corpus.rs WASM-safety scrub discipline
- **Files modified:** `crates/amlich-core/src/iching/evaluator.rs`
- **Verification:** CRIT-3 grep test passes, WASM-safety grep test passes; full crate suite green
- **Committed in:** `563cc27` (RED commit) — the fix landed BEFORE the RED commit shipped

**2. [Rule 1 - Bug] `iching_query_nfc_normalises_question` original input relied on a Unicode composition quirk**

- **Found during:** Task 1 GREEN verification
- **Issue:** Initial test input `"vi\u{0300}e\u{0302} công vi\u{00ea}\u{0302}c"` decomposed to NFD form expecting NFC normalisation to `"viê công viếc"`. But the actual NFC result is `"vìê công viê̂c"` because `vi\u{00ea}\u{0302}c` (precomposed `ê` followed by combining circumflex) has no standard precomposed form (`ê` + circumflex would be `ế` U+1EC5 which is `e` + circumflex + acute, NOT `e` + circumflex + circumflex). The test was asserting composition behaviour that depends on quirky Unicode character ranges
- **Fix:** Simplified to a single-grave-mark NFD case (`"vi\u{0300} công việc"` → `"vì công việc"`) and asserted on the preselected-presence invariant (`stored.contains('\u{00EC}')`). Trims the assertion to the core contract (NFC recomposition) without depending on Unicode composition specifics out of scope for the contract test
- **Files modified:** `crates/amlich-core/src/iching/evaluator.rs`
- **Verification:** Test passes; full crate suite green; preserves the original intent (NFD input is recomposed to NFC, verified via `is_nfc()` + `contains('\u{00EC}')`)
- **Committed in:** `4ecd343` (GREEN commit)

**3. [Rule 2 - Missing Critical] `enrich_day_snapshot_with_iching` re-export added to `iching/mod.rs` despite being declared in `lib.rs`**

- **Found during:** Task 2 GREEN compilation (`cargo build -p amlich-core`)
- **Issue:** The Phase 24 plan-checker warning `d63639f` explicitly pins the contract that Plan 24-03 callers will import via `use amlich_core::iching::{enrich_day_snapshot_with_iching, IChingQuery}`. Without an explicit `pub use crate::enrich_day_snapshot_with_iching` in `iching/mod.rs`, the helper would be reachable only via `amlich_core::enrich_day_snapshot_with_iching` (the canonical site at lib.rs), forcing future Phase 24-03 callers to change their import path
- **Fix:** Added `pub use crate::enrich_day_snapshot_with_iching;` line in `iching/mod.rs` (one-line re-export; the canonical placement stays at `lib.rs` for the public-API surface, but the helper is reachable via both `amlich_core::enrich_day_snapshot_with_iching` and `amlich_core::iching::enrich_day_snapshot_with_iching`). Mirrors the 23-02/23-03 pattern where `enrich_day_snapshot_with_direction_cross_link` is similarly re-exported
- **Files modified:** `crates/amlich-core/src/iching/mod.rs`
- **Verification:** Build clean; integration test `iching_cast_byte_equal_round_trip` exercises the enrichment helper path
- **Committed in:** `8ea2373` (Task 2 commit)

---

**Total deviations:** 3 auto-fixed (2 Rule 1 — bug fixes for false-positive grep guard + Unicode composition edge case; 1 Rule 2 — missing critical re-export per the plan-checker's pinned `d63639f` contract).

**Impact on plan:** All three fixes are necessary to keep the plan's own verification gates passing and to honor the Phase 24 plan-checker's locked import path. No scope creep; no behavior change to the locked contracts (locked 4-envelope evidence vector; sibling-newtype query; Tier-0 / no-birth-data ActionEvaluator adapter; immutable clone-and-attach enrichment; owned DTO projection; CRIT-3 + WASM-safety + bare-literal source-id discipline).

## Issues Encountered

None beyond the three Rule 1/2 deviations documented above.

## Authentication Gates

None — no external services, no credentials, no CLI deployments. Pure Rust algorithm + DTO + immutable enrichment + integration tests against the already-shipped Phase 20/21/22 types + Phase 23 immutable enrichment helper pattern.

## User Setup Required

None — no external service configuration required. This plan is pure Rust algorithm + DTO + integration tests against already-shipped Phase 20 schema + Phase 21 corpus + Phase 22 cast/biến-quẻ/thể-dụng types + Phase 23 immutable enrichment pattern. No new dependencies, no environment variables, no dashboards.

## Next Phase Readiness

- **ICH-05 is fully closed.** `IChingEvaluator` ships with per-step evidence envelopes (≥2 primitive distinct source_ids including `mai-hoa-dich-so` AND `kinh-dich`, plus exactly 1 composite `rule.composite.iching_consultation`), works at Tier-0 with no birth data, and is exposed via the explicit immutable `enrich_day_snapshot_with_iching` helper. The evaluator never re-implements the Phase 22 surface — every arithmetic step delegates to `cast_mai_hoa` / `derive_bien_que` / `classify_the_dung` directly (verified by the `iching_evaluator_uses_phase_22_surface` integration test re-deriving the expected values via the Phase 22 surface and asserting equality)
- **INT-12 is partially closed here.** `DaySnapshot.iching_cast: Option<IChingCastSummary>` is in place with the additive-`Option<T>` + `#[serde(default, skip_serializing_if = "Option::is_none")]` discipline. The full INT-12 close-out — combined-strip v1.6→v1.7 round-trip including `direction_cross_link` — ships in Plan 24-03 (Phase 24 plan-checker contract pinned in commit `d63639f`)
- **CRIT-3 isolation preserved across the new module.** `rg "impl From<(Tien|Hau|KingWen)" crates/amlich-core/src/iching/evaluator.rs` returns zero matches; `rg "FlyingStar|direction_merge" crates/amlich-core/src/iching/evaluator.rs` returns zero matches. The CRIT-3 + WASM-safety grep guards use runtime-built needles (mirrors the v1.6/v1.7 discipline codified across `corpus.rs`, `mai_hoa.rs`, `bien_que.rs`, `the_dung.rs`, `golden.rs`)
- **WASM-safety + determinism discipline preserved.** `rg "rand::|Utc::now|std::fs::"` returns zero matches across `evaluator.rs`. File system-free, wall-clock-free, RNG-free
- **Bare source-id literal discipline preserved.** `tests/source_id_guard.rs` still passes; the only literal `"kinh-dich"` / `"mai-hoa-dich-so"` mentions outside `sources.rs` are inside `#[cfg(test)]` blocks (test assertions + the new `ProvenanceEntry::iching(...)` test fixture in `provenance.rs`)
- **No new crate dependencies.** `cargo tree -p amlich-core --depth 1` shows the existing `chrono` + `serde` + `serde_json` + `unicode-normalization` set unchanged
- **Ready for Plan 24-02** (IChing semantic-graph wiring — `add_iching_facts()` method on `DaySnapshotGraphBuilder` to add Hexagram nodes + `LocatedAt` / `Transforms` edges from `iching_cast` summary; published via `semantic_graph/builders/day_snapshot.rs`)
- **Ready for Plan 24-03** (additive `DaySnapshot.direction_cross_link` is already shipped via Phase 23-03's `enrich_day_snapshot_with_direction_cross_link` helper; Plan 24-03's deliverable is the combined-strip v1.6→v1.7 round-trip integration test + the REQUIREMENTS.md INT-12 close — the `enrich_day_snapshot_with_iching` import path `use amlich_core::iching::{enrich_day_snapshot_with_iching, IChingQuery}` resolves cleanly via the `iching/mod.rs` re-export this plan added)
- **Ready for Phase 25** (E2E Validation + Golden Cross-Source Verification) — INT-13's combined-strip cross-source gate is met by Phase 22-02's golden dataset + Phase 24-02's `add_iching_facts` builder + the combined-strip round-trip test landed in Plan 24-03
- **No blockers.** Phase 24 has 2 more plans (24-02 semantic-graph wiring + 24-03 combined-strip round-trip) + Phase 25 E2E; the IChing half of INT-11/INT-12 ships here, leaving Phase 23's cross-link half already shipped

---
*Phase: 24-iching-evaluator-semantic-graph-wiring-dto-integration*
*Completed: 2026-07-16*

## Self-Check: PASSED

- All 2 declared `key-files.created` exist on disk:
  - `crates/amlich-core/src/iching/evaluator.rs`
  - `crates/amlich-core/tests/iching_evaluator_integration.rs`
- All 3 declared `key-files.modified` exist on disk:
  - `crates/amlich-core/src/iching/mod.rs`
  - `crates/amlich-core/src/semantic_graph/provenance.rs`
  - `crates/amlich-core/src/lib.rs`
- All 3 task commit hashes (`563cc27` RED, `4ecd343` GREEN Task 1, `8ea2373` GREEN Task 2) are present in `git log`
- Plan-level verification gates green:
  - `cargo test -p amlich-core --lib iching::evaluator` — 19/19 inline tests pass
  - `cargo test -p amlich-core --lib semantic_graph::provenance` — 2/2 new tests pass + existing tests still pass
  - `cargo test -p amlich-core --test iching_evaluator_integration` — 18/18 black-box tests pass
  - `cargo test -p amlich-core --test source_id_guard` — 1/1 passes (no bare source-id literals introduced)
  - `cargo test -p amlich-core --test fengshui_crit3_isolation` — 1/1 passes (existing grep guard unaffected)
  - `cargo build -p amlich-core` — clean
  - `cargo tree -p amlich-core --depth 1` — no new dependency (chrono + serde + serde_json + unicode-normalization)
- Full crate test result: 1101 passing tests across 48 test groups, 0 failures, 0 regressions vs Phase 23-03's 1062 baseline (+39 net additions = 19 inline + 18 integration + 2 provenance)
- `rg "impl From<(Tien|Hau|KingWen)" crates/amlich-core/src/iching/evaluator.rs` returns ZERO — CRIT-3 isolation preserved
- `rg "FlyingStar|direction_merge" crates/amlich-core/src/iching/evaluator.rs` returns ZERO — no premature Phase 23 cross-link import
- `rg "rand::|Utc::now|std::fs::" crates/amlich-core/src/iching/evaluator.rs` returns ZERO — WASM-safety + determinism preserved
- `rg "rule.composite.iching_consultation" crates/amlich-core/src/` returns 4 (1 const declaration + 2 doc-comment mentions + 1 test assertion) — all legitimate uses of the named const
- Ordinary `calculate_day_snapshot(...)` snapshots serialize WITHOUT `"iching_cast"` key (verified by `ordinary_day_snapshot_does_not_serialize_iching_cast_key` test)
- Enriched snapshots serialize WITH `"iching_cast"` AND round-trip byte-equally (verified by `iching_cast_byte_equal_round_trip` test)
- `tests/source_id_guard.rs` still passes — no bare source-id literals introduced (verified by the bonus `source_id_guard_passes_for_new_evaluator_module` integration test)
- ICH-05 marked Complete in REQUIREMENTS.md (INT-12 partial close here; full close in Plan 24-03)
