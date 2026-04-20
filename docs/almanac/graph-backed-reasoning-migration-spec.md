# Graph-backed Reasoning Migration Spec

> **Doc status:** Active — canonical spec for Epic 4 (Graph-backed Reasoning).
> **When to use:** Reference this when migrating `InitiationOpening` (or future actions) to graph-backed evaluation, implementing the `ActionEvaluator` trait, or projecting decision exports from the semantic graph.
> **Depends on:** [`semantic-graph-v1-spec.md`](semantic-graph-v1-spec.md) (Epic 1), [`interaction-graphification-spec.md`](interaction-graphification-spec.md) (Epic 3)
> **Parent roadmap:** [`hybrid-semantic-graph-plan.md`](hybrid-semantic-graph-plan.md)
> **Migration playbook:** [`legacy-replacement-playbook.md`](legacy-replacement-playbook.md)
> **Contract follow-up:** [`reasoning-graph-export-replacement-plan.md`](reasoning-graph-export-replacement-plan.md)

## Purpose

Define how `amlich-core` migrates from the current action-specific reasoning pipeline toward graph-backed reasoning built on the new `semantic_graph` substrate.

This spec focuses on:

- `InitiationOpening` as the first migrated action
- a reusable action evaluator interface
- graph-to-decision projection
- reuse of interaction subgraphs in reasoning logic

## Current State

The current reasoning flow in `reasoning/`:
- `facts.rs` builds a fact graph
- `signals.rs` maps facts to interpreted axes
- `vector.rs` assembles an action vector
- `synthesis.rs` turns that vector into decision exports

Limitations:
1. almost entirely scoped to `InitiationOpening`
2. graph structure is action-local rather than system-global
3. matrix semantics are mostly collapsed into summarized fact nodes
4. reasoning graph is built as its own pipeline rather than projected from a canonical graph substrate

## Migration Goals

1. keep current reasoning output parity while changing internal substrate
2. make `InitiationOpening` the first graph-backed evaluator
3. reuse graphified matrices rather than summarizing them into text-only nodes
4. separate fact graph, evaluator logic, and decision projection
5. create an evaluator model that can later support more actions

## Target Architecture

```text
domain producers
  -> semantic_graph builders
    -> action evaluator
      -> decision projection
      -> reasoning graph projection
      -> llm / visualization slices
```

## Action Evaluator Model

### Evaluator trait

```rust
pub trait ActionEvaluator {
    fn action_id(&self) -> ActionId;

    fn select_subgraph(
        &self,
        graph: &SemanticGraph,
    ) -> Result<SemanticGraph, String>;

    fn evaluate(
        &self,
        graph: &SemanticGraph,
    ) -> Result<ActionEvaluation, String>;
}
```

### `ActionEvaluation`

```rust
pub struct ActionEvaluation {
    pub action_id: ActionId,
    pub bucket: RecommendationBucket,
    pub confidence: DecisionConfidence,
    pub semantic: ReasoningConclusionSemantic,
    pub context_is_clear: bool,
    pub primary_conclusion: String,
    pub strongest_supports: Vec<ReasoningNote>,
    pub strongest_resistances: Vec<ReasoningNote>,
    pub override_factors: Vec<ReasoningNote>,
    pub conflict_notes: Vec<ReasoningNote>,
    pub suggested_hours: Vec<String>,
    pub suggested_directions: Vec<String>,
    pub axis_scores: Vec<ReasoningAxisScore>,
    pub referenced_node_ids: Vec<String>,
    pub referenced_edge_ids: Vec<String>,
}
```

## InitiationOpening as First Graph-backed Evaluator

### Required reasoning input graph
Merged graph should contain:
- day fact graph from `DaySnapshot`
- optional bazi graph
- optional interaction matrix graphs:
  - day-person
  - personal-hour
  - direction-merge
  - later element-resonance/domain-boost if needed

### Initial subgraph selection for `InitiationOpening`
Day-side facts:
- `Truc`
- `DayDeity`
- `Taboo`
- `DayStar`
- `XungHop`
- `HoangDaoHours`
- `TravelDirection`

Personal-side facts:
- profile root
- pillars
- day-person matrix graph
- personal-hour matrix graph
- direction-merge matrix graph

Deferred for later:
- element resonance
- domain day boost

## Evidence Classes

### Support evidence
- favorable `Truc`
- hoàng đạo day deity
- supportive stars
- good hoàng đạo hours
- high-scoring personal hours
- positive direction merge rows

### Resistance evidence
- negative `XungHop`
- unfavorable branch interactions in day-person matrix
- poor personal-hour rows
- negative domain pressure later

### Override evidence
- hard taboo nodes
- future action-blocking matrix patterns if policy introduces them

### Conflict evidence
- simultaneous support and resistance clusters
- mixed day + personal signals

## Reusing Interaction Subgraphs

### DayPersonMatrix
Evaluator may inspect:
- rows with `luc_hop` or `tam_hop` as support
- rows with `luc_xung`, `tuong_hai`, `tuong_hinh` as resistance/conflict

### PersonalHourMatrix
Evaluator may select top hours where:
- `is_hoang_dao = true`
- `score >= threshold`

### DirectionMergeMatrix
Evaluator may:
- choose positive-net directions as suggested directions
- use negative-net directions as caution context later if needed

Rule: evaluator uses matrix outputs as evidence and does **not** recompute matrix formulas.

## Graph-to-Decision Projection

### Compatibility projections

```rust
pub fn project_initiation_opening_decision(
    evaluation: &ActionEvaluation,
) -> InitiationOpeningDecision;
```

```rust
pub fn project_initiation_opening_decision_export(
    evaluation: &ActionEvaluation,
) -> InitiationOpeningDecisionExport;
```

```rust
pub fn project_reasoning_graph_export(
    graph: &SemanticGraph,
    evaluation: &ActionEvaluation,
) -> ReasoningGraphExport;
```

### Projection strategy
- do not export the entire merged graph blindly
- export evaluator-selected subgraph only
- include referenced nodes/edges and tightly connected context
- preserve explanation-relevant graph semantics without flooding consumers

## Axis Mapping Strategy

Preserve current compatibility axes:
- support
- resistance
- stability
- personal_alignment
- timing_fit
- context_clarity

Example mapping:
- favorable `Truc`, hoàng đạo deity, good stars -> `support`
- taboo pressure, clash-heavy relations -> `resistance`
- branch instability, taboo penalties -> `stability`
- day-person matrix + direction merge -> `personal_alignment`
- personal-hour + hoàng đạo hours -> `timing_fit`
- mixed evidence clusters -> `context_clarity`

## Reasoning Input Graph Assembly

```rust
pub fn build_reasoning_input_graph(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<SemanticGraph, String>;
```

Responsibilities:
- build day graph
- optionally build bazi graph
- optionally build matrix graphs
- merge all graphs
- return evaluator-ready semantic graph

## InitiationOpening Evaluator

```rust
pub struct InitiationOpeningEvaluator;
```

```rust
impl ActionEvaluator for InitiationOpeningEvaluator {
    fn action_id(&self) -> ActionId { ... }
    fn select_subgraph(&self, graph: &SemanticGraph) -> Result<SemanticGraph, String> { ... }
    fn evaluate(&self, graph: &SemanticGraph) -> Result<ActionEvaluation, String> { ... }
}
```

## Public Reasoning Entry Point

Public API remains:

```rust
pub fn build_initiation_opening_reasoning_bundle(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<InitiationOpeningReasoningBundle, String>;
```

Internal flow becomes:
1. build merged semantic graph
2. evaluate with `InitiationOpeningEvaluator`
3. project compatibility outputs

## Test Strategy

Required parity tests:
1. bucket parity
2. semantic parity
3. support/resistance material parity
4. personal refinement parity for suggested hours/directions
5. reasoning graph projection sanity

Recommended test sources:
- `reasoning_graph_parity.rs`
- `reasoning_graph_canonical.rs`
- `reasoning_graph_contract.rs`

## Execution Order

1. reasoning input graph assembly
2. `InitiationOpeningEvaluator` skeleton
3. `ActionEvaluation` projection layer
4. parity-backed graph evaluator
5. reasoning graph export projection

## Exit Criteria

Epic 4 for `InitiationOpening` is complete when:
- one merged semantic graph can be built for reasoning input
- `InitiationOpeningEvaluator` can read that graph
- current decision structs are projected from evaluator output
- reasoning graph export is derived from semantic graph
- parity tests pass against the current pipeline
- current public API remains stable
