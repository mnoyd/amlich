# Reasoning Graph Export Replacement Plan

> **Doc status:** Active planning note for `amlich-jih`.
> **Implements next:** `amlich-p16`
> **Primary context:** [`graph-backed-reasoning-migration-spec.md`](graph-backed-reasoning-migration-spec.md), [`legacy-replacement-playbook.md`](legacy-replacement-playbook.md), [`../audit-semantic-graph-migration-surfaces.md`](../audit-semantic-graph-migration-surfaces.md)

## 1. Current Consumers

### Live contract consumers

- `crates/amlich-core/src/reasoning/synthesis.rs`
  - `build_initiation_opening_reasoning_bundle()` still fills `bundle.graph` by calling the private legacy helper `build_legacy_reasoning_graph_export()`, which runs `build_fact_graph()` -> `derive_interpreted_signals()` -> `export_reasoning_graph()`.
- `crates/amlich-core/src/lib.rs`
  - re-exports `InitiationOpeningReasoningBundle`, `ReasoningGraphExport`, `ReasoningGraph`, `ReasoningNode`, and `ReasoningEdge` as public crate API.
- `crates/amlich-api/src/dto.rs`
  - `PersonalDayAnalysisDto.graph` and `PersonalDayReportDto.graph` expose `Option<amlich_core::reasoning::ReasoningGraphExport>`.
- `crates/amlich-api/src/lib.rs`
  - `get_personal_day_analysis()` and `get_personal_day_report()` copy `bundle.graph` directly into API DTOs.
- `apps/desktop/src/lib/insights/types/personal-day-dto.ts`
  - mirrors the Rust `ReasoningGraphExport` DTO shape exactly.
- `apps/desktop/src/lib/components/PersonalDayPanel.svelte`
  - reads `report.graph.nodes.length` and `report.graph.edges.length` for the evidence summary string.

### Contract tests that lock the shape

- `crates/amlich-core/tests/reasoning_graph_contract.rs`
  - asserts `bundle.graph` is present, non-empty, serializes under the `graph` key, and preserves severity/tags/evidence/justification metadata.
- `crates/amlich-core/tests/reasoning_graph_canonical.rs`
  - asserts JSON round-trip stability and checks known exported node IDs such as `fact.day.taboos`, `fact.day.day_deity`, and `fact.day.hoang_dao_hours`.
- `crates/amlich-core/tests/reasoning_graph_public_api.rs`
  - asserts the public bundle API still exposes a non-empty `graph`.
- `crates/amlich-api/tests/personal_day_contract.rs`
  - asserts `graph` is present when reasoning is available, absent when it is not, and equal between `report.graph` and `report.analysis.graph`.
- `apps/desktop/src-tauri/src/lib.rs`
  - asserts the Tauri command still returns non-empty graph nodes and parity between `report.graph` and `report.analysis.graph`.

### Legacy-producer-only consumers

- `crates/amlich-core/tests/reasoning_graph_facts.rs`
- `crates/amlich-core/tests/reasoning_graph_signals.rs`
- `crates/amlich-core/tests/reasoning_graph_vector.rs`
- `crates/amlich-core/tests/reasoning_graph_personal.rs`
- `crates/amlich-core/tests/reasoning_graph_types.rs`

These tests exercise the old `ReasoningGraph` pipeline directly, but they do not prove that downstream app/API consumers need that raw graph representation.

## 2. Replacement Contract Options

### Option A: semantic-native public export

Replace `bundle.graph: ReasoningGraphExport` with a semantic-graph-native DTO.

- Pros: aligns with the target architecture and stops carrying the compatibility shape.
- Cons: breaks `amlich-core` public API, `amlich-api` DTOs, desktop TS types, UI code, and current contract tests in one step.
- Assessment: too broad for `amlich-p16`.

### Option B: compatibility-shaped semantic export

Keep `bundle.graph: ReasoningGraphExport`, but generate it from the semantic graph and evaluator output instead of the legacy `ReasoningGraph` pipeline.

- Pros: removes the legacy producer from the production path without forcing downstream contract changes.
- Pros: matches the replacement playbook: replace substrate first, contract second.
- Cons: requires a semantic-to-compat projection layer and parity tests.
- Assessment: safest default.

### Option C: public dual contract

Keep `graph: ReasoningGraphExport` and add a second semantic-native graph field.

- Pros: gives future consumers a richer graph immediately.
- Cons: expands the public contract without a confirmed consumer; adds rollout and documentation overhead.
- Assessment: only justified if a near-term caller needs semantic-native graph semantics that `ReasoningGraphExport` cannot represent.

## 3. Recommended Contract

Recommend **Option B** for `amlich-p16`: keep the public `ReasoningGraphExport` contract, but make it a **compatibility projection from the semantic graph**.

That means:

- `InitiationOpeningReasoningBundle.graph` stays present and keeps the same type.
- `PersonalDayAnalysisDto.graph`, `PersonalDayReportDto.graph`, and the desktop `ReasoningGraphExportDto` stay unchanged during `amlich-p16`.
- The old flat `ReasoningGraph` stops being the production source for `bundle.graph`.

### Required compatibility guarantees

`amlich-p16` should preserve all of the following:

- top-level field presence: `bundle.graph` still exists, and API `graph` fields remain `Some(...)` only when reasoning is available
- parity between `report.graph` and `report.analysis.graph`
- snake_case serialization for enums and fields already asserted by tests
- non-empty `graph.nodes` and `graph.edges` for a populated reasoning bundle
- exported node IDs already treated as canonical in tests: at minimum `fact.day.taboos`, `fact.day.day_deity`, `fact.day.hoang_dao_hours`, and the `signal.*` axis nodes
- computed annotations that current tests and consumers rely on:
  - node `axis`
  - node `severity`
  - node `tags`
  - edge `weight`
  - edge `tags`
  - evidence provenance fields
  - known justifications such as `taboo_pressure`, `personal_day_alignment`, and `personal_hour_alignment`

### Contract boundary to preserve vs. retire

- Preserve for now:
  - `ReasoningGraphExport`
  - `ReasoningNodeExport`
  - `ReasoningEdgeExport`
  - `InitiationOpeningReasoningBundle.graph`
- Retire from the production path:
  - the legacy graph-building pipeline currently used only to synthesize `bundle.graph`

## 4. Safe Rollout Plan

## Step 1: add a semantic-backed compatibility projector

Add a projector that takes:

- the evaluator-selected semantic subgraph
- the `ActionEvaluation`

and returns `ReasoningGraphExport`.

Implementation notes:

- do not rebuild the old `ReasoningGraph`
- derive the existing compatibility annotations directly from semantic nodes/edges
- use evaluator-selected or evaluator-referenced graph slices rather than exporting the whole merged semantic graph
- keep node IDs stable where current tests treat them as canonical

## Step 2: keep the existing bundle/API surface unchanged

Rewire `build_initiation_opening_reasoning_bundle()` so:

- `decision` still comes from `project_initiation_opening_decision(&evaluation)`
- `decision_export` still comes from `project_initiation_opening_decision_export(&evaluation)`
- `graph` now comes from the new semantic-backed compatibility projector

`amlich-p16` should not rename or remove the `graph` field.

## Step 3: add parity coverage before deleting the legacy producer

Add focused tests that compare the new semantic-backed export with the current legacy export across the existing reasoning corpus.

Minimum parity checks:

- top-level action id
- node/edge non-emptiness
- presence of canonical node IDs
- parity for severity/tag/weight derivations
- parity for key evidence source families and edge justifications
- parity for personal cases that currently expose interaction evidence

Allow only intentionally documented diffs. If any diff is accepted, write it down in the test or migration note.

## Step 4: remove only the private production bridge

Once parity passes, remove the private helper in `synthesis.rs` that still calls:

- `build_fact_graph()`
- `derive_interpreted_signals()`
- `export_reasoning_graph()`

This is the safe deletion boundary for `amlich-p16`.

## Step 5: defer broader legacy cleanup to follow-up beads

Do not remove the raw legacy graph API in `amlich-p16`. Audit and deprecate it separately after production no longer depends on it.

## 5. Legacy Retirement Criteria

### Go criteria for deleting the old production graph path

- `build_initiation_opening_reasoning_bundle()` no longer calls the legacy `ReasoningGraph` producer
- all `reasoning_graph_contract`, `reasoning_graph_canonical`, API contract, and Tauri contract tests pass against the semantic-backed projector
- full-profile bundles still expose non-empty `graph.nodes` and `graph.edges`
- incomplete-profile API/report cases still return `graph: None`
- no unresolved parity gaps remain for canonical node IDs, severity/tags/weights, or key edge justifications

### No-go criteria

- any app/API contract still depends on the old producer's exact behavior and the new projector cannot match it yet
- node ID drift breaks current canonical tests or downstream graph lookups
- the new projector would require a public DTO shape change to land safely
- the semantic export path still needs the full merged graph because evaluator subgraph selection/referenced-node tracking is too loose to make the output observable and stable

### What can retire after `amlich-p16` lands

Eligible for deprecation or later removal once direct callers/tests are migrated:

- `reasoning/facts.rs`
- `reasoning/signals.rs`
- `reasoning/vector.rs`
- `reasoning/export.rs`
- raw legacy graph types in `reasoning/types.rs`:
  - `ReasoningGraph`
  - `ReasoningNode`
  - `ReasoningEdge`
- direct tests that only validate the legacy producer internals

### What should remain temporarily

- `ReasoningGraphExport` compatibility DTOs
- `InitiationOpeningReasoningBundle.graph`
- API DTO `graph` fields
- desktop TS DTOs and UI uses
- `ActionEvaluation`, `ActionEvaluator`, `InitiationOpeningEvaluator`
- decision/export projections in `reasoning/projection.rs`

The public compatibility DTO should remain until there is a separate, explicit consumer migration away from `ReasoningGraphExport`.

## 6. Follow-up Work for `amlich-p16`

`amlich-p16` should do exactly this:

1. Introduce a semantic-backed `ReasoningGraphExport` projector.
2. Rewire `build_initiation_opening_reasoning_bundle()` to use that projector for `graph`.
3. Add parity tests proving the semantic-backed export preserves the current compatibility contract.
4. Delete only the private production helper that builds `bundle.graph` through the legacy fact/signal/export pipeline.

`amlich-p16` should not do this:

- remove `ReasoningGraphExport` from public DTOs
- remove `bundle.graph`
- delete raw legacy graph types or their direct tests unless the bead is explicitly expanded
- introduce a new public semantic-native graph DTO unless a real consumer is identified and scoped
