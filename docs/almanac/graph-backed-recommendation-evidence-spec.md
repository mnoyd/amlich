# Graph-backed Recommendation Evidence Spec

> **Doc status:** Active — canonical spec for Epic 5 (Graph-backed Recommendation Evidence).
> **When to use:** Reference this when graphifying recommendation hits, activity nodes, and evidence aggregation, or connecting recommendation evidence to shared semantic facts.
> **Depends on:** [`semantic-graph-v1-spec.md`](semantic-graph-v1-spec.md) (Epic 1), [`interaction-graphification-spec.md`](interaction-graphification-spec.md) (Epic 3), [`graph-backed-reasoning-migration-spec.md`](graph-backed-reasoning-migration-spec.md) (Epic 4)
> **Parent roadmap:** [`hybrid-semantic-graph-plan.md`](hybrid-semantic-graph-plan.md)
> **Contains:** Reasoning–Recommendation Convergence Spec (appendix below — describes the long-term convergence strategy for Epics 4 and 5)

## Purpose

Define how the recommendation layer in `amlich-core` migrates from a procedural hit-merging pipeline toward graph-backed recommendation evidence built on the shared `semantic_graph` substrate.

This spec does **not** require immediate replacement of recommendation policy resolution. It focuses first on graphifying recommendation evidence so that:

- recommendation reasoning becomes inspectable and visualizable
- activity recommendations can be connected to the same semantic facts used by reasoning
- future convergence between reasoning and recommendation becomes possible without a rewrite

## Current State

Current recommendation flow is centered in `almanac/recommendation/synthesize.rs`:

- collect base hits from `Trực`
- add modifier hits from stars, day deity, taboos, xung/hop, travel, tiết khí, giờ hoàng đạo
- optionally add event kind layers and pack layers
- merge hits into per-activity recommendation outcomes
- apply policy to determine buckets and summaries

This works well, but recommendation evidence currently exists mainly as procedural intermediate state rather than canonical graph data.

## Migration Goals

1. preserve current recommendation outputs while graphifying evidence
2. represent activities and recommendation hits as semantic graph structures
3. connect recommendation evidence to shared day/person facts where possible
4. make recommendation evidence visualizable and LLM-ready
5. defer policy replacement until evidence graph is stable

## Non-goals

This spec does not yet require:
- replacing `synthesize_daily_recommendations_with_layers`
- rewriting recommendation policy in graph form
- changing public recommendation DTOs immediately
- merging all recommendation and reasoning semantics into one evaluator model in the first pass

## Target Architecture

```text
domain producers + semantic facts
  -> recommendation evidence graph builders
    -> activity evidence subgraphs
      -> compatibility recommendation projection
      -> visualization views
      -> llm-ready recommendation slices
```

## Core Recommendation Graph Concepts

## New / extended entity kinds

```rust
pub enum EntityKind {
    ...
    Activity,
    RecommendationHit,
    RecommendationLayer,
    RecommendationSummary,
    RecommendationPolicy,
}
```

## New / extended relation kinds

```rust
pub enum RelationKind {
    ...
    Recommends,
    AdvisesAgainst,
    ContributesTo,
    OriginatesFrom,
    TargetsActivity,
    ProducedByLayer,
    Aggregates,
}
```

These names can still be refined, but recommendation graphification needs stable identity for activities, hits, layers, and aggregated outcomes.

## Evidence Graph Shape

## Activity nodes
Every activity in the recommendation system should become a stable graph entity.

Examples:
```text
activity:opening_start
activity:contract_agreement
activity:travel
activity:wedding_engagement
```

Attributes:
```json
{
  "activity_id": "opening_start",
  "label_vi": "khai trương / mở việc"
}
```

## Recommendation hit nodes
Each collected hit should become a graph node instead of remaining procedural-only.

Example IDs:
```text
recommendation_hit:opening_start:truc:kien:favor
recommendation_hit:opening_start:taboo:tam_nuong:avoid
recommendation_hit:travel:deity:thanh_long:favor
```

Attributes should preserve current hit fields:
```json
{
  "source": "truc",
  "source_code": "kien",
  "direction": "favor",
  "summary_vi": "Trực Kiến hợp mở việc",
  "summary_en": "Truc Kien supports opening start",
  "severity": "normal",
  "hard_stop": false
}
```

## Recommendation summary / aggregate nodes
For each activity, add one aggregate node representing merged recommendation evidence before final bucket projection.

Example:
```text
recommendation_summary:opening_start:day:2024-05-13:tz+7
```

Attributes:
```json
{
  "favor_sources": 2,
  "strong_avoid_sources": 1,
  "supporting_avoid_sources": 0,
  "saw_hard_stop": false
}
```

This does not replace policy yet; it gives the graph a stable place to attach per-activity evidence.

## Layer nodes
Optional for first pass, but recommended where pack/layer attribution matters.

Examples:
```text
recommendation_layer:core_policy
recommendation_layer:event_kind
recommendation_layer:pack:nhi_thap_bat_tu
```

## Edge Layout

### Activity evidence edges
- `recommendation_hit -> activity` with `TargetsActivity`
- or `activity <- recommendation_hit` with `ContributesTo`

### Layer attribution edges
- `recommendation_hit -> recommendation_layer` with `ProducedByLayer`

### Fact-origin edges
Where recommendation hits are derived from known graph facts, connect them explicitly.

Examples:
- `truc node -> recommendation_hit` with `OriginatesFrom`
- `taboo node -> recommendation_hit` with `OriginatesFrom`
- `day_deity node -> recommendation_hit` with `OriginatesFrom`
- `travel direction node -> recommendation_hit` with `OriginatesFrom`

This is the key bridge between recommendation evidence and the shared semantic graph.

### Aggregate edges
- `recommendation_hit -> recommendation_summary` with `ContributesTo`
- `recommendation_summary -> activity` with `Summarizes`

### Policy/result edges (later-compatible)
After evidence graph is stable, future graph policy can add:
- `recommendation_summary -> activity` with `Recommends`
- `recommendation_summary -> activity` with `AdvisesAgainst`

For the first pass, keep final bucket/result in projected compatibility output.

## Builder APIs

Add in `semantic_graph/builders/recommendation.rs`:

```rust
pub fn build_recommendation_evidence_graph(
    context: &RecommendationSynthesisContext<'_>,
    recommendations: &DailyRecommendations,
) -> Result<SemanticGraph, String>;
```

This first-pass API may need access to internal collected hits rather than only final recommendations. If so, introduce an internal intermediary:

```rust
pub fn build_recommendation_evidence_graph_from_hits(
    context: &RecommendationSynthesisContext<'_>,
    hits: &[RecommendationLayerHit],
) -> Result<SemanticGraph, String>;
```

Recommendation: separate hit collection from final merge so graphification can occur before aggregation is lost.

## Suggested Internal Refactor

Current pipeline can evolve toward:

1. collect raw hits
2. graphify hits
3. merge hits into activity summaries
4. project existing `DailyRecommendations`

This keeps policy behavior stable while exposing recommendation evidence structurally.

## Compatibility Projection

Public outputs remain current structs for now.

Needed helper:

```rust
pub fn project_daily_recommendations_from_graph(
    graph: &SemanticGraph,
) -> Result<DailyRecommendations, String>;
```

In the first migration step, this may remain partial or test-only. The more realistic first goal is:
- graph evidence is built alongside existing projection
- parity asserts that graph-backed aggregation matches the existing summary state

## Provenance Rules

Recommendation hits must preserve:
- source family (`truc`, `taboo`, `star`, `deity`, `travel`, `hour`, `pack`, etc.)
- source code
- whether hit came from core policy or extension layer
- ruleset metadata from `DayFortune`
- pack/layer identifiers when relevant

This should integrate with `GraphProvenance` rather than invent a parallel provenance model.

## Recommendation Fact Connectivity

A recommendation graph is only valuable if it connects back to real facts.

### Required first-pass origin mappings
- `Truc` -> recommendation hits collected from `collect_truc_hits`
- `DayDeity` -> deity modifier hits
- `Taboo` -> taboo modifier hits
- `XungHop` -> xung/hop modifier hits
- `TravelDirection` -> travel modifier hits
- `HoangDaoHours` or hour nodes -> hour modifier hits

### Deferred mappings
- pack-specific stars or richer star-rule nodes where current graph identity is still evolving
- person-specific recommendation layers if/when they become active in recommendation policy

## Visualization Guidance

Recommendation evidence is especially suited to graph UI.

Suggested clusters:
- day facts -> `day-core`
- recommendation hits -> `recommendation-evidence`
- activity nodes -> `recommendation-activities`
- aggregate summary nodes -> `recommendation-summary`
- extension layers/packs -> `recommendation-layers`

Useful visual patterns:
- one activity node with multiple incoming supporting/avoiding hits
- shared fact nodes feeding multiple activities
- clear separation between evidence and final recommendation state

## LLM-ready Guidance

Recommendation LLM slices should include:
- activity IDs
- strongest favor hits
- strongest avoid hits
- whether hard stop exists
- source fact references
- pack/layer origin where relevant

Example summary points:
- "`opening_start` is supported by Trực Kiến and day deity support but pressured by Tam Nương taboo"
- "`travel` remains favorable because no hard-stop taboo targets it and receives supporting deity/travel signals"

These summaries must remain traceable back to hit nodes and source facts.

## Test Strategy

### Required tests
1. activity nodes created for recommended activities
2. recommendation hits preserved as graph nodes
3. hit origin/source metadata preserved
4. fact-origin edges connect hits to source facts where available
5. per-activity aggregate nodes preserve summary counts / hard-stop state
6. graph-backed evidence remains materially consistent with existing `DailyRecommendations`

### Strongly recommended
- one test for pack/layer attribution
- one test for hard-stop taboo evidence
- one test where one fact contributes to multiple activities

## Recommended Execution Order

### Slice 1 — internal hit graph model
- expose or capture recommendation hits before aggregation is flattened away
- define stable activity and hit IDs

### Slice 2 — graphify core recommendation evidence
- truc, taboo, deity, xung/hop, travel, hour modifiers
- activity nodes
- hit nodes
- aggregate summary nodes

### Slice 3 — fact-origin connectivity
- connect recommendation hits back to semantic fact nodes

### Slice 4 — visualization and llm slices
- top-activity evidence extraction
- support/avoid split for consumers

### Slice 5 — compatibility/parity tests
- ensure graph evidence materially matches existing recommendation summaries

## Risks

1. hit identity becomes unstable if source code normalization is inconsistent
2. policy and evidence are migrated together too early
3. graph does not connect back to shared facts, reducing value
4. packs/layers introduce complex attribution before the core path is stable

Mitigations:
- keep policy projection stable initially
- graphify evidence first
- make origin edges mandatory for core evidence types
- defer pack-specific richness until core evidence graph is correct

## Exit Criteria

Graph-backed recommendation evidence is complete for the first migration stage when:
- recommendation activities and hits exist as graph entities
- core evidence types are graphified
- graph hits connect back to shared semantic facts where possible
- per-activity summaries can be visualized and queried
- current recommendation outputs remain stable
- parity tests show graph evidence is materially consistent with existing output

## Recommended Next Spec After This

After this, the next useful spec should be:

**Reasoning–Recommendation Convergence Spec**

focused on:
- which evidence is shared between action evaluators and activity recommendations
- where reasoning and recommendation should remain separate
- whether a common evaluator/policy substrate should exist later

---
# Reasoning–Recommendation Convergence Spec

> **Note:** This convergence spec is co-located here because it directly extends both the recommendation evidence model (above) and the graph-backed reasoning model. It describes the long-term convergence strategy rather than an independent implementation spec.

## Purpose

Define how graph-backed action reasoning and graph-backed recommendation evidence should converge over time without collapsing two distinct concerns into one prematurely.

This spec exists because the hybrid architecture introduces two related but non-identical systems:

- **action reasoning**: evaluates whether a specific action such as `InitiationOpening` is favorable, cautious, or avoid-worthy under a given day/profile context
- **activity recommendations**: produces lists of recommended/avoidable activities using day-level evidence and policy-driven aggregation

These systems should share semantic facts and graph evidence where possible, but they should not be forced into one identical evaluator model too early.

## Why convergence matters

Today the codebase has separate pathways:

- `reasoning/` for action-specific graph and decision synthesis
- `almanac/recommendation/` for activity-centric hit collection and aggregation

The new `semantic_graph` substrate makes it possible to:

- share fact identity
- share evidence provenance
- share matrix and day-profile interaction subgraphs
- visualize both action reasoning and activity recommendations over the same graph
- expose more coherent LLM-ready slices

Without a convergence strategy, the system risks replacing one set of islands with two new islands: a graph-backed reasoning island and a graph-backed recommendation island.

## Design goals

1. Share fact and evidence identity between reasoning and recommendation.
2. Keep action evaluators and activity policy aggregation distinct until their overlap is proven.
3. Reuse graph evidence rather than duplicating derivation logic.
4. Preserve compatibility with current decision and recommendation contracts.
5. Make future unification possible, but not mandatory at the start.

## Non-goals

This spec does not require:

- immediate replacement of current recommendation policy
- immediate merging of `ActionEvaluator` and recommendation policy into one trait
- forcing all activities to behave like action-level reasoning evaluators
- rewriting public DTOs in one pass

## Shared substrate model

Both systems should converge first at the **evidence substrate** layer.

### Shared layers

```text
domain facts
  -> semantic_graph facts
  -> interaction subgraphs
  -> recommendation evidence nodes
  -> action evaluator subgraphs
```

The order of convergence should be:

1. shared fact graph
2. shared interaction graph
3. shared recommendation evidence graph
4. shared selection rules where overlap is real
5. only then consider shared evaluator/policy abstractions

## What should be shared now

### 1. Canonical entity identity
The following must be shared between reasoning and recommendation:

- day nodes
- taboo nodes
- truc nodes
- day deity nodes
- star nodes or star aggregates
- xung/hop nodes
- travel direction nodes
- hoàng đạo hour nodes
- bazi/profile nodes when relevant
- interaction matrix nodes when relevant
- activity IDs
- action IDs

Reasoning and recommendation should never mint parallel identities for the same day fact.

### 2. Provenance
Both systems should use the same provenance model:

- `source_family`
- `source_id`
- `method`
- `ruleset_id`
- `ruleset_version`
- `profile`
- `note`

### 3. Fact-origin connectivity
Recommendation hits and reasoning notes should be traceable back to the same fact nodes where possible.

Examples:
- `taboo:tam_nuong` may contribute to both `InitiationOpening` reasoning and `opening_start` recommendation evidence
- `truc:Kien` may support both the action evaluator and one or more recommendation activities
- `direction_merge` rows may drive `suggested_directions` in reasoning and later directional recommendation refinement

## What should stay separate for now

### 1. Decision shape vs recommendation shape
Reasoning currently answers:
- what is the verdict for one action?
- what supports or resists it?
- what refinements or cautions matter?

Recommendation currently answers:
- which activities are favored or avoided today?
- what evidence contributed to each activity bucket?

These are related but not identical products.

### 2. Evaluator vs policy aggregator
`ActionEvaluator` should stay separate from recommendation policy in the first convergence phase.

Reason:
- action reasoning is narrow, action-specific, and can inspect personal graphs deeply
- recommendation policy is broad, activity-centric, and currently rule-aggregation oriented

Attempting to unify them too early would likely overfit one to the other's constraints.

### 3. Output contracts
Keep these distinct initially:
- `InitiationOpeningDecisionExport`
- `DailyRecommendations`
- recommendation evidence graph views
- reasoning graph views

They may later share projection infrastructure, but should not be collapsed prematurely.

## Recommended convergence model

## Phase A — Shared facts only
Completed by:
- Semantic Graph v1
- interaction graphification

At this phase:
- reasoning and recommendation still evaluate separately
- both consume the same day/profile fact graph

## Phase B — Shared evidence links
Implemented by graph-backed recommendation evidence and graph-backed reasoning migration.

At this phase:
- recommendation hits point back to fact nodes
- reasoning notes point back to fact and matrix nodes
- both can be visualized in one graph
- both can produce LLM slices with shared references

## Phase C — Shared evidence selectors
Once both sides are graph-backed, identify reusable selectors.

Examples:
- taboo pressure selector
- favorable truc selector
- supportive day deity selector
- hoàng đạo timing selector
- clash-heavy xung/hop selector

These selectors should live below evaluator/policy level, e.g. as graph-query helpers or evidence-selection utilities.

This is the most realistic first point of meaningful convergence.

## Phase D — Optional shared scoring primitives
Only after selectors are stable, consider shared scoring primitives such as:
- hard-stop detection
- support/resistance weighting
- conflict density
- direction refinement extraction
- top-hour extraction

These should still not force one unified evaluator abstraction unless the overlap becomes substantial in practice.

## Shared selector examples

Potential reusable helpers:

```rust
pub fn select_hard_taboo_nodes(graph: &SemanticGraph) -> Vec<String>;
pub fn select_supportive_truc_nodes(graph: &SemanticGraph) -> Vec<String>;
pub fn select_hoang_dao_timing_nodes(graph: &SemanticGraph) -> Vec<String>;
pub fn select_positive_direction_rows(graph: &SemanticGraph) -> Vec<String>;
pub fn select_conflict_heavy_branch_relations(graph: &SemanticGraph) -> Vec<String>;
```

These can feed both:
- `InitiationOpeningEvaluator`
- recommendation evidence aggregation

This gives reuse without prematurely forcing identical top-level contracts.

## Shared graph views

The semantic graph should eventually support combined views such as:

### 1. Action-focused reasoning view
- action evaluator selected evidence
- strongest supports/resistances
- personal refinements

### 2. Activity-focused recommendation view
- activity hit nodes
- aggregate summary nodes
- origin fact connectivity

### 3. Combined convergence view
A filtered graph showing:
- shared fact nodes
- action reasoning nodes
- recommendation hit nodes
- where one source fact influences both surfaces

This combined view is valuable for:
- debugging drift
- validating semantic consistency
- future UI explainability
- LLM-based analysis

## Contract strategy

### Short term
Keep reasoning and recommendation projections separate.

### Medium term
Standardize shared internal metadata:
- IDs
- provenance
- evidence references
- shared selector utilities

### Long term
Consider whether a common internal abstraction should exist, for example:

```rust
pub trait EvidenceConsumer {
    fn evidence_scope(&self) -> EvidenceScope;
    fn select_evidence(&self, graph: &SemanticGraph) -> Result<SemanticGraph, String>;
}
```

But do **not** introduce this until actual duplication appears.

## LLM-ready convergence

LLM-ready exports should be one of the earliest visible benefits of convergence.

A combined LLM slice should be able to answer:
- which facts matter to the action verdict?
- which facts matter to the activity recommendations?
- which facts are shared by both?
- where do reasoning and recommendation diverge?
- is the divergence due to action-specific constraints or aggregation policy?

This is one of the strongest reasons to share semantic graph evidence even before full evaluator unification.

## Risks

### 1. Over-unifying too early
Mitigation:
- converge at evidence and selector layers first
- keep evaluator/policy layers separate until justified

### 2. Shared selectors become hidden policy logic
Mitigation:
- keep selectors descriptive, not verdict-producing
- policy/evaluator layers remain responsible for final meaning

### 3. Drift persists despite shared graph
Mitigation:
- add convergence-focused tests on shared source facts
- verify that the same fact IDs are reused across both systems

### 4. Combined graph views become noisy
Mitigation:
- rely on filtered subgraph extraction and convergence-focused slices

## Test strategy

### Required convergence tests
1. same source fact node IDs reused across reasoning and recommendation graphs
2. recommendation hits correctly point to canonical fact nodes
3. reasoning notes correctly point to canonical fact/matrix nodes
4. shared selector outputs remain stable for representative cases
5. combined convergence view can be extracted without orphan nodes

### Recommended higher-level tests
- one case where taboo pressure affects both action reasoning and activity recommendation
- one case where reasoning and recommendation share supporting evidence but diverge in final interpretation
- one case with personal refinement affecting reasoning more than generic recommendation

## Execution order

1. finish semantic graph v1
2. finish interaction graphification
3. land graph-backed reasoning for `InitiationOpening`
4. land graph-backed recommendation evidence
5. add shared evidence selectors
6. add convergence-focused tests and combined graph views
7. only then reassess whether deeper evaluator/policy convergence is worth the complexity

## Exit Criteria

The first stage of reasoning–recommendation convergence is complete when:
- reasoning and recommendation share canonical fact IDs
- recommendation hits and reasoning notes both connect back to the same semantic graph facts
- shared selector utilities exist for common evidence families
- combined convergence views can be exported
- public reasoning and recommendation contracts remain stable

## Recommended follow-up

After this, the next useful planning artifact would be an **Implementation Slices Plan** that turns Epic 1–5 into concrete coding milestones and sequencing for actual repo work.
