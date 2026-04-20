# Audit: Orphaned Semantic Graph Migration Surfaces (amlich-mzc)

**Date:** 2026-04-20
**Scope:** `crates/amlich-core/src/semantic_graph/`, `reasoning/`, `almanac/recommendation/`
**Evidence:** `cargo build -p amlich-core` produced 60 warnings; all files read and cross-referenced.

## Executive Summary

The codebase maintains **two parallel reasoning pipelines** that compute nearly identical results:

1. **Old path (production):** `reasoning::build_fact_graph` → `derive_interpreted_signals` → `assemble_action_vector` → `build_initiation_opening_reasoning_bundle` — operates on the flat `ReasoningGraph` (vec-based).
2. **New path (test-only):** `semantic_graph::build_reasoning_input_graph` → `InitiationOpeningEvaluator::evaluate` — operates on the rich `SemanticGraph`.

The semantic-graph-era code was built as a richer substrate to replace the old reasoning graph, but the migration was never completed. The old path remains the production consumer. The new path, views, selectors, and projection helpers are only exercised by tests.

---

## 1. Confirmed Orphaned / Likely Removable

### 1.1 Interaction projection layer (35+ dead functions)

**File:** `semantic_graph/builders/interaction.rs:742-1291`
**Symbols:** `project_day_person_matrix`, `project_personal_hour_matrix`, `project_element_resonance_matrix`, `project_direction_merge_matrix`, `project_domain_day_boost_matrix`, and ~30 private helpers (`matrix_root_node`, `row_nodes`, `required_tag`, `has_tag`, `parse_bool_tag`, `parse_u8_tag`, `parse_u16_tag`, `parse_usize_tag`, `parse_i8_tag`, `parse_f32_tag`, `row_key`, `parse_pillar_kind`, `parse_five_element`, `parse_five_element_relation`, `parse_element_interaction`, `parse_direction_signal`, `direction_signal_order`, `pillar_order`, `five_element_order`, `direction_order`, `domain_order`, `project_day_person_pillar`, `project_personal_hour_entry`, `project_element_resonance_entry`, `project_direction_entry`, `project_domain_day_boost_entry`, `relation_child_node`, `thap_than_from_relation_node`, `thap_than_from_matrix_root`, `branch_relation_from_node`, `element_interaction_from_node`, `rule_evidence_from_tags`)
**Usage evidence:** Build produces 35 warnings — "function is never used." These functions extracted typed structs back from the graph (graph→struct round-trip). The original typed structs are already computed directly by the `interaction` module, so the round-trip is never needed.
**Classification:** **remove** — The projection layer has no consumer. The `build_*_matrix_graph` functions that write into the graph are still used by `merge.rs`, but the read-back projections are dead.

### 1.2 `build_element_resonance_matrix_graph` and `build_domain_day_boost_matrix_graph`

**File:** `semantic_graph/builders/interaction.rs:529-712`
**Usage evidence:** Build warns "function is never used." Not called in `merge.rs`, not called in any test, not re-exported.
**Classification:** **remove** — These matrix-to-graph builders have zero consumers. The underlying typed matrices (`ElementResonanceMatrix`, `DomainDayBoostMatrix`) are computed and consumed directly without graph intermediation.

### 1.3 `project_reasoning_graph_export`

**File:** `reasoning/projection.rs:60-183`
**Usage evidence:** Defined, exported via `reasoning/mod.rs`, but never called anywhere — not even in tests. The function was meant to bridge `SemanticGraph` → `ReasoningGraphExport` but no consumer invokes it.
**Classification:** **remove**

### 1.4 Unused builder surface methods/fields

**File:** `semantic_graph/builders/interaction.rs`
**Symbols:** `InteractionGraphBuilder::with_ruleset()`, `InteractionGraphBuilder::graph()`
**Usage evidence:** Build warns "methods are never used." Only `.new()`, `.add_*()`, and `.build()` are called.
**Classification:** **remove** — These were extension points that never got consumers.

**File:** `semantic_graph/builders/day_snapshot.rs`
**Symbols:** `DaySnapshotGraphBuilder` fields `ruleset_id`, `ruleset_version` (warned as never read). The struct is never exported; only `build_day_snapshot_graph()` is called.
**Classification:** **remove** the dead fields.

**File:** `semantic_graph/builders/merge.rs`
**Symbols:** `ReasoningInputGraph::day_root_id`, `profile_root_id` — warned "associated functions are never used."
**Classification:** **remove** or **wire** — These would be useful if the evaluator path became production, but currently dead.

---

## 2. Compatibility-Only but Still Justified

### 2.1 Old reasoning pipeline (`reasoning/` module)

**Files:** `reasoning/facts.rs`, `signals.rs`, `vector.rs`, `synthesis.rs`, `export.rs`, `types.rs`
**Usage evidence:** Actively called by `lib.rs::build_initiation_opening_reasoning()` and `build_initiation_opening_reasoning_bundle()`. External tests in `tests/reasoning_graph_*.rs` (11 test files) exercise this pipeline.
**Classification:** **keep** — This is the production reasoning path. It must stay until the semantic-graph evaluator fully replaces it.

### 2.2 `reasoning::export_reasoning_graph`

**File:** `reasoning/export.rs`
**Usage evidence:** Called by `synthesis.rs:152` to produce `ReasoningGraphExport` in the bundle.
**Classification:** **keep** — Active production consumer.

### 2.3 `build_day_snapshot_graph`, `build_bazi_profile_graph`, `build_day_person_matrix_graph`, `build_personal_hour_matrix_graph`, `build_direction_merge_matrix_graph`

**Files:** `semantic_graph/builders/day_snapshot.rs`, `bazi.rs`, `interaction.rs`
**Usage evidence:** All are actively called by `merge.rs::ReasoningInputGraph::from_day_and_bazi()` and/or by tests. They construct the semantic graph from typed data.
**Classification:** **keep** — These are the graph construction layer. Used by the test-only evaluator but represent the intended future direction.

---

## 3. Planned Extension Surfaces with No Real Consumer Yet

### 3.1 `InitiationOpeningEvaluator` + `ActionEvaluator` trait

**File:** `reasoning/initiation_opening_evaluator.rs`, `reasoning/action_evaluator.rs`
**Usage evidence:** Only used in `tests/reasoning_graph_parity.rs` (5 test functions). Not called from any production code. Re-exported via `reasoning/mod.rs` and `lib.rs`.
**Classification:** **wire** — This evaluator was built to work on `SemanticGraph` and is the intended replacement for the old `synthesis.rs` pipeline. It contains duplicated synthesis logic (semantic classification, conclusion generation, hour/direction refinement). Should be wired as the production path once parity is confirmed.

### 3.2 `EvidenceSelectors`

**File:** `semantic_graph/selectors.rs`
**Usage evidence:** All 15+ selector methods are only called in the module's own tests. Exported via `semantic_graph/mod.rs` → `lib.rs` but no external consumer.
**Classification:** **wire** — Intended as query API for graph consumers. Keep as public extension surface but acknowledge it has no production consumer yet.

### 3.3 View types (`VisualizationGraph`, `LlmGraphSlice`, `LlmConvergenceSlice`, `ConvergenceView`, `RecommendationEvidenceGraphView`, `LlmRecommendationSlice`, `SubgraphView`)

**Files:** `semantic_graph/views/*.rs`
**Usage evidence:** All usage is within the view modules' own test blocks. No app or package imports these.
**Classification:** **wire** — These are serialization/presentation layers designed for LLM consumers and visualization. They're the right abstraction but need wiring into an actual API surface.

### 3.4 Recommendation evidence graph builders

**File:** `semantic_graph/builders/recommendation.rs`
**Symbols:** `RecommendationEvidenceGraphBuilder`, `build_recommendation_evidence_graph`, `build_recommendation_evidence_graph_connected`, `build_recommendation_evidence_graph_with_layers`
**Usage evidence:** Build warns all 4 symbols are never constructed/used. All usage is in the module's own tests. Exported through `builders/mod.rs` → `semantic_graph/mod.rs`.
**Classification:** **wire** — These produce recommendation evidence graphs with proper provenance. They should be wired into the day snapshot or advisory pipeline.

### 3.5 `build_reasoning_input_graph` / `ReasoningInputGraph`

**File:** `semantic_graph/builders/merge.rs`
**Usage evidence:** Re-exported via `lib.rs`. Used in `tests/reasoning_graph_parity.rs` (5 tests). No app or package consumer.
**Classification:** **wire** — This is the intended entry point for constructing a full semantic graph. Should eventually be called by the advisory/advisor layer.

---

## 4. Duplicated / Consolidation Candidates

### 4.1 Synthesis logic duplication

**Files:** `reasoning/synthesis.rs` vs `reasoning/initiation_opening_evaluator.rs`
**Evidence:** Both contain identical semantic classification logic (`OverrideAvoid`/`OverrideCautious`/`ConflictedCautious`/`ResistanceLedCautious`/`FavorableClear`/`FavorableContextial`), identical conclusion text generation, identical hour/direction refinement logic, identical confidence mapping.
**Classification:** **consolidate** — The evaluator is the richer version operating on `SemanticGraph`. Once it becomes the production path, `synthesis.rs` can be retired.

### 4.2 Source-family counting duplication

**Files:** `semantic_graph/selectors.rs:193-223` (`count_hits_by_source_family`) vs `semantic_graph/views/recommendation.rs:145-171` (`compute_source_breakdown`)
**Evidence:** Both iterate recommendation hit nodes and count by source family using identical string matching logic. They produce slightly different struct types (`SourceFamilyCounts` vs `SourceBreakdown`) with the same fields.
**Classification:** **consolidate** — Unify into one implementation.

### 4.3 Severity interpretation duplication

**Files:** `reasoning/export.rs:50-91` vs `reasoning/initiation_opening_evaluator.rs:144-163`
**Evidence:** Both hard-code severity interpretation by node concept and severity string (e.g., Truc+cat → Auspicious, Taboo+hard → HardTaboo). The evaluator does it inline, export.rs does it in a dedicated function.
**Classification:** **consolidate** — Extract to a shared severity mapping.

### 4.4 Two graph representations

**Files:** `reasoning/types.rs` (`ReasoningGraph`) vs `semantic_graph/graph.rs` (`SemanticGraph`)
**Evidence:** `ReasoningGraph` is a flat `Vec<ReasoningNode>` + `Vec<ReasoningEdge>` without IDs or lookup. `SemanticGraph` is a rich `HashMap<String, SemanticNode>` + `HashMap<String, SemanticEdge>` with full provenance, ontology, and merge support. Both encode the same domain knowledge.
**Classification:** **consolidate** — The semantic graph is the intended successor. The old `ReasoningGraph` should be retired once the evaluator path is production-validated.

---

## 5. Recommended Follow-Up Tasks

| Priority | Task | Rationale |
|----------|------|-----------|
| High | Remove dead interaction projection layer (~550 lines) | 35 build warnings, zero consumers |
| High | Remove dead `build_element_resonance_matrix_graph` and `build_domain_day_boost_matrix_graph` | Zero consumers, not even tests |
| High | Remove dead `project_reasoning_graph_export` | Exported but never called |
| Medium | Wire `InitiationOpeningEvaluator` as production path | Replaces duplicated synthesis logic, operates on richer graph |
| Medium | Consolidate source-family counting (`selectors.rs` ↔ `views/recommendation.rs`) | Identical logic in two places |
| Medium | Wire recommendation evidence graph into advisory pipeline | Builder exists but no production consumer |
| Low | Remove dead builder fields/methods (`with_ruleset`, `graph()`, unused fields) | Minor cleanup |
| Low | Consolidate severity interpretation mapping | Shared logic in export.rs and evaluator |
| Low | Wire view types (VisualizationGraph, LlmGraphSlice, etc.) into API surface | Ready for consumption but no consumer yet |

---

## Build Warning Summary

60 warnings total:
- 35+ "function is never used" (interaction projections + helpers)
- 4 "struct/function is never used" (recommendation builder)
- 2 "unreachable pattern"
- 2 "fields never read" (builder dead fields)
- 2 "associated functions never used" (ReasoningInputGraph)
- 1 "unused import" (various re-exports of dead symbols)
- Misc: 1 unused variable, 1 unnecessary mut, 1 unused import
