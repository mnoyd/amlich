# Amlich-core Hybrid Semantic Graph Plan

> **Doc status:** Active — source-of-truth architecture roadmap for the semantic graph track.
> **When to use:** Read this first to understand the overall hybrid `2.5` architecture vision, epic sequencing, and dependency order. Then consult the detailed specs for each epic.

## Purpose

Define the hybrid `2.5` architecture direction for `amlich-core` where existing domain engines remain authoritative for computation while a new `semantic_graph` substrate becomes the long-term integration layer for facts, matrices, reasoning, recommendation evidence, visualization, and LLM-ready outputs.

## Vision

Build `amlich-core` toward a semantic, visualizable, LLM-ready engine where:

- almanac facts
- bazi/person facts
- interaction matrices
- reasoning
- recommendation evidence
- visualization exports
- LLM-ready semantic views

all converge through a shared graph substrate instead of remaining isolated feature-specific islands.

The goal is not a big-bang rewrite. The goal is to create a safe migration path from the current feature-sliced architecture to a graph-backed, explainable, extensible core.

## Current Shape

Today the system is split across several productive but partially disconnected layers:

- `almanac/` produces day-level structured outputs
- `bazi/` produces chart, analysis, metrics, and advisory outputs
- `interaction/` produces personal matrices
- `reasoning/` produces an action-specific reasoning graph and decision export
- `almanac/recommendation/` synthesizes activity recommendations through a procedural hit-merging pipeline

These layers already contain much of the right data, but they do not yet share one canonical semantic substrate.

## Target Architecture

```text
domain producers
  -> semantic_graph
    -> reasoning evaluators
    -> matrix projections
    -> recommendation projections
    -> visualization exports
    -> llm-ready exports
```

### Key principle

Existing modules continue to compute domain facts. The new `semantic_graph` layer becomes the place where those facts are normalized into:

- stable entities
- typed relationships
- provenance-aware evidence
- subgraphs/views for downstream consumers

This avoids a big-bang rewrite while still creating a durable long-term architecture.

## Core Design Goals

1. **Keep current feature velocity where possible**
   - Do not require full replacement of `reasoning/` or `recommendation/` before value appears.
2. **Introduce one canonical semantic model**
   - Facts, interactions, and explanations should become comparable and composable.
3. **Support visualization as a first-class concern**
   - The system should export graph-friendly structures with enough metadata to render causal and relational views.
4. **Be LLM-ready by construction**
   - The substrate should expose stable IDs, typed relations, provenance, ambiguity, and compact slices.
5. **Treat current outputs as projections**
   - `decision_export`, matrix tables, recommendation lists, and UI summaries should progressively become views over shared graph data.

## Proposed Module Direction

Add a new module in `crates/amlich-core/src/`:

```text
semantic_graph/
  mod.rs
  ontology.rs
  ids.rs
  provenance.rs
  node.rs
  edge.rs
  graph.rs
  builders/
    day_snapshot.rs
    bazi.rs
    interaction.rs
    recommendation.rs
  views/
    reasoning.rs
    matrix.rs
    visualization.rs
    llm.rs
```

## Epic Plan

### Epic 1 — Semantic Graph Foundation
- create `semantic_graph/`
- define node/edge/provenance types
- define ontology v1
- define stable ID conventions
- define graph assembly conventions

### Epic 2 — Domain Fact Graphification
- map `DaySnapshot`, `DayFortune`, `BaziChart`, `BaziAnalysisReport`
- preserve provenance and stable identity
- make day/person facts mergeable into one graph

### Epic 3 — Interaction Matrices as Graph Projections
- graphify `DayPersonMatrix`, `PersonalHourMatrix`, `ElementResonanceMatrix`, `DirectionMergeMatrix`, `DomainDayBoostMatrix`
- keep current matrix structs as compatibility projections
- add visualization and LLM slices over interaction subgraphs

### Epic 4 — Graph-backed Reasoning
- migrate `InitiationOpening` first
- introduce reusable action evaluator model
- project decision exports and reasoning graph views from semantic graph

### Epic 5 — Graph-backed Recommendation Evidence
- map recommendation hits/layers into semantic graph evidence
- connect activity recommendations to shared graph facts
- prepare gradual convergence between reasoning and recommendation

### Epic 6 — Visualization Contract
- define graph-library-friendly exports
- add cluster/island metadata
- support filtered subgraph views by action, matrix, and profile completeness

### Epic 7 — LLM-ready Semantic Views
- define compact graph slices
- top causal chains
- ambiguity, conflict, and missing-input summaries
- preserve traceability back to graph IDs and provenance

### Epic 8 — Migration, Compatibility, and Validation
- dual-path validation where needed
- parity tests for old/new projections
- contract locks for graph exports and compatibility surfaces
- explicit retirement plans for legacy adapters

## Dependency Order

1. Epic 1 — foundation
2. Epic 2 — fact graphification
3. Epic 3 — interaction graphification
4. Epic 4 — graph-backed reasoning
5. Epic 5 — graph-backed recommendation evidence
6. Epic 6 — visualization contract
7. Epic 7 — LLM-ready views
8. Epic 8 — runs throughout and formalizes rollout discipline

## First Execution Slice

1. add `semantic_graph/` core types
2. build graph from `DaySnapshot` + `DayFortune`
3. graphify `DayPersonMatrix`
4. expose one visualization-friendly subgraph view
5. compare graph-backed output with existing matrix behavior

## Main Risks

1. ontology becomes too broad too early
2. graph layer becomes a second ad-hoc abstraction
3. matrix/reasoning/recommendation drift apart during migration
4. visualization needs distort core design
5. LLM-ready exports become flattened convenience blobs

## Related Specs

This roadmap is the architectural overview. The following specs detail each epic:

| Spec | Epic | Purpose |
|------|------|---------|
| [`semantic-graph-v1-spec.md`](semantic-graph-v1-spec.md) | 1 — Foundation | Core types, IDs, builders, merge rules |
| [`interaction-graphification-spec.md`](interaction-graphification-spec.md) | 3 — Interaction Graphs | Five matrix graphification, projections |
| [`graph-backed-reasoning-migration-spec.md`](graph-backed-reasoning-migration-spec.md) | 4 — Reasoning | `ActionEvaluator` trait, `InitiationOpening` migration |
| [`graph-backed-recommendation-evidence-spec.md`](graph-backed-recommendation-evidence-spec.md) | 5 — Recommendation | Recommendation evidence graph, convergence strategy |

Epics 2, 6, 7, and 8 do not yet have standalone specs — they are described inline in this plan until dedicated specs are needed.
