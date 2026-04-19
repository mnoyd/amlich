# Semantic Graph v1 Spec

## Purpose

Define the first concrete version of the new `semantic_graph` substrate for `amlich-core`.

This spec is intentionally narrow. It is the smallest stable semantic layer needed to:

- graphify day-level facts
- graphify bazi/profile facts
- support future matrix graphification
- support visualization experiments
- support LLM-ready structured slices
- avoid a big-bang rewrite

## v1 Scope

### Included
- `DaySnapshot`
- `DayFortune`
- `BaziChart`
- `BaziAnalysisReport`

### Excluded from v1
- recommendation policy resolution
- graph-backed action evaluators
- matrix graphification beyond extension hooks
- decision synthesis from graph
- UI-specific layout logic
- narrative generation rules

## Design Principles

1. Stable identity over display text
2. Structured semantics over flattened summaries
3. Provenance is first-class
4. Additive introduction
5. Composable graph slices
6. LLM-readiness without prompt coupling

## Module Layout

```text
crates/amlich-core/src/
  semantic_graph/
    mod.rs
    ontology.rs
    ids.rs
    provenance.rs
    node.rs
    edge.rs
    graph.rs
    builders/
      mod.rs
      day_snapshot.rs
      bazi.rs
    views/
      mod.rs
      subgraph.rs
      visualization.rs
      llm.rs
```

## Core Types

### `EntityKind`

```rust
pub enum EntityKind {
    Day,
    SolarTerm,
    Truc,
    DayDeity,
    DayStar,
    Taboo,
    XungHop,
    TravelDirection,
    HoangDaoHours,
    BaziProfile,
    Pillar,
    DayMasterStrength,
    ElementDistribution,
    TenGodDistribution,
    ChartInteraction,
    Matrix,
    Signal,
}
```

### `RelationKind`

```rust
pub enum RelationKind {
    HasFact,
    HasComponent,
    AppliesTo,
    DerivedFrom,
    Summarizes,
    Supports,
    Weakens,
    Overrides,
    ConflictsWith,
    HarmonizesWith,
    ClashesWith,
    Generates,
    Controls,
}
```

### `GraphSourceFamily`

```rust
pub enum GraphSourceFamily {
    Snapshot,
    AlmanacRule,
    Bazi,
    Interaction,
    Derived,
}
```

### `GraphProvenance`

```rust
pub struct GraphProvenance {
    pub source_family: GraphSourceFamily,
    pub source_id: String,
    pub method: String,
    pub ruleset_id: Option<String>,
    pub ruleset_version: Option<String>,
    pub profile: Option<String>,
    pub note: Option<String>,
}
```

### `GraphNode`

```rust
pub struct GraphNode {
    pub id: String,
    pub kind: EntityKind,
    pub label_vi: String,
    pub tags: Vec<String>,
    pub attributes: BTreeMap<String, serde_json::Value>,
    pub provenance: Vec<GraphProvenance>,
}
```

### `GraphEdge`

```rust
pub struct GraphEdge {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub kind: RelationKind,
    pub weight: Option<f32>,
    pub tags: Vec<String>,
    pub attributes: BTreeMap<String, serde_json::Value>,
    pub provenance: Vec<GraphProvenance>,
}
```

### `SemanticGraph`

```rust
pub struct SemanticGraph {
    pub root_ids: Vec<String>,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}
```

Required helpers:

```rust
impl SemanticGraph {
    pub fn new() -> Self;
    pub fn add_node(&mut self, node: GraphNode) -> Result<(), GraphMergeError>;
    pub fn add_edge(&mut self, edge: GraphEdge) -> Result<(), GraphMergeError>;
    pub fn merge(&mut self, other: SemanticGraph) -> Result<(), GraphMergeError>;
}
```

## ID Format Rules

### Day root
```text
day:YYYY-MM-DD:tz+7
```

### Day child facts
```text
solar_term:day:2024-05-13:tz+7
truc:day:2024-05-13:tz+7
day_deity:day:2024-05-13:tz+7
xung_hop:day:2024-05-13:tz+7
travel:day:2024-05-13:tz+7
hoang_dao_hours:day:2024-05-13:tz+7
taboo:day:2024-05-13:tz+7:tam_nuong
```

### Bazi root
```text
bazi_profile:1990-01-01T09:30:tz+7
```

### Bazi child facts
```text
pillar:bazi_profile:...:year
pillar:bazi_profile:...:month
pillar:bazi_profile:...:day
pillar:bazi_profile:...:hour
element_distribution:bazi_profile:...
ten_god_distribution:bazi_profile:...
day_master_strength:bazi_profile:...
chart_interaction:bazi_profile:...:0
```

Rules:
- deterministic
- readable
- type-prefixed
- no localized display strings as identity
- no random IDs

## Builder APIs

### DaySnapshot builder

```rust
pub fn build_day_snapshot_graph(snapshot: &DaySnapshot) -> Result<SemanticGraph, String>;
```

Responsibilities:
- create day root
- attach solar term, truc, deity, taboo, stars, xung_hop, travel, hoang dao hours
- preserve `RuleEvidence` and ruleset provenance

### Bazi builder

```rust
pub fn build_bazi_profile_graph(
    chart: &BaziChart,
    analysis: &BaziAnalysisReport,
) -> Result<SemanticGraph, String>;
```

Responsibilities:
- create profile root
- attach pillars
- attach element distribution, ten god distribution, day master strength, chart interactions

## Merge Rules

1. same node ID + same kind -> merge provenance uniquely
2. same node ID + different kind -> error
3. same edge ID + same payload -> merge provenance uniquely
4. same edge ID + different payload -> error

## View APIs

### Subgraph extraction

```rust
pub fn extract_subgraph(
    graph: &SemanticGraph,
    root_ids: &[&str],
    depth: usize,
) -> Result<SemanticGraph, String>;
```

### Visualization export

```rust
pub struct VisualizationGraph {
    pub nodes: Vec<VisualizationNode>,
    pub edges: Vec<VisualizationEdge>,
}
```

Initial cluster mapping:
- day-side facts -> `day-core`
- bazi-side facts -> `bazi-core`

### LLM slice

```rust
pub struct LlmGraphSlice {
    pub root_ids: Vec<String>,
    pub node_refs: Vec<String>,
    pub edge_refs: Vec<String>,
    pub summary_points: Vec<String>,
}
```

Rules:
- summary points are derived
- node/edge refs remain traceable to canonical graph IDs

## First Slice Execution Plan

### Slice 1 — foundation types
- add `semantic_graph/` core types
- add deterministic ID helpers
- add merge API and tests

### Slice 2 — day graph builder
- implement `build_day_snapshot_graph(snapshot)`
- cover `DayFortune` facts
- add provenance and determinism tests

### Slice 3 — bazi graph builder
- implement `build_bazi_profile_graph(chart, analysis)`
- cover pillars and analysis facts
- add mergeability tests

### Slice 4 — lightweight views
- subgraph extraction
- visualization export
- llm slice export

## Exit Criteria

Semantic Graph v1 is complete when:
- the substrate compiles as a stable internal module
- day graphs build from `DaySnapshot`
- bazi graphs build from `BaziChart` + `BaziAnalysisReport`
- graphs merge safely
- subgraph and lightweight visualization/llm views export
- tests prove determinism and provenance preservation
