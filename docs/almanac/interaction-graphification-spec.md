# Interaction Graphification Spec

> **Doc status:** Active — canonical spec for Epic 3 (Interaction Matrices as Graph Projections).
> **When to use:** Reference this when graphifying any of the five interaction matrices, defining matrix ontology extensions, or building matrix subgraph extraction helpers.
> **Depends on:** [`semantic-graph-v1-spec.md`](semantic-graph-v1-spec.md) (Epic 1 foundation types)
> **Parent roadmap:** [`hybrid-semantic-graph-plan.md`](hybrid-semantic-graph-plan.md)

## Purpose

Define how `interaction/` outputs become graph-backed structures in the hybrid `semantic_graph` architecture.

This spec covers all five current matrices:

1. `DayPersonMatrix`
2. `PersonalHourMatrix`
3. `ElementResonanceMatrix`
4. `DirectionMergeMatrix`
5. `DomainDayBoostMatrix`

## Goals

1. represent matrix semantics as structured subgraphs
2. preserve current matrix outputs as compatibility projections
3. make interaction data visualizable
4. prepare these relations for future reasoning evaluators
5. keep migration additive and parity-testable

## Shared Rules

- graph builders must not recompute matrix formulas
- every matrix graph has one deterministic root node
- current matrix structs remain compatibility contracts until graph-backed parity is proven
- provenance from matrix evidence must survive graphification

## Ontology Extensions

Suggested additions:

```rust
pub enum EntityKind {
    ...
    DayPersonMatrix,
    PersonalHourMatrix,
    ElementResonanceMatrix,
    DirectionMergeMatrix,
    DomainDayBoostMatrix,
    InteractionRow,
    PillarRelation,
    HourSlot,
    TenGodRelation,
    BranchRelation,
    ElementRelation,
}
```

```rust
pub enum RelationKind {
    ...
    HasMatrix,
    HasRow,
    RelatesTo,
    Evaluates,
    InteractsWith,
    HasTenGodRelation,
    HasBranchRelation,
    HasElementRelation,
    BestFor,
}
```

## DayPersonMatrix

### Root
```text
matrix:day_person:<day-id>:<profile-id>
```

### Rows
```text
matrix_row:day_person:<day-id>:<profile-id>:year
matrix_row:day_person:<day-id>:<profile-id>:month
matrix_row:day_person:<day-id>:<profile-id>:day
matrix_row:day_person:<day-id>:<profile-id>:hour
```

### Relation nodes
- `ten_god_relation:day_person:<...>:year`
- `branch_relation:day_person:<...>:year`
- `element_relation:day_person:<...>:year`

### Builder
```rust
pub fn build_day_person_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &DayPersonMatrix,
) -> Result<SemanticGraph, String>;
```

### Projection
```rust
pub fn project_day_person_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<DayPersonMatrix, String>;
```

### Required tests
- one matrix root
- one row per pillar
- relation nodes per row
- projection parity
- stable IDs

## PersonalHourMatrix

### Root
```text
matrix:personal_hour:<day-id>:<profile-id>
```

### Rows
```text
matrix_row:personal_hour:<day-id>:<profile-id>:0
...
matrix_row:personal_hour:<day-id>:<profile-id>:11
```

### Relation nodes
- `ten_god_relation:personal_hour:<...>:0`
- `branch_relation:personal_hour:<...>:0`
- `element_relation:personal_hour:<...>:0`

### Builder
```rust
pub fn build_personal_hour_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &PersonalHourMatrix,
) -> Result<SemanticGraph, String>;
```

### Projection
```rust
pub fn project_personal_hour_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<PersonalHourMatrix, String>;
```

### Required tests
- one root and 12 rows
- row scores preserved
- `is_hoang_dao` preserved
- projection parity
- best-scoring hour unchanged

## ElementResonanceMatrix

### Root
```text
matrix:element_resonance:<day-id>:<profile-id>
```

### Rows
```text
matrix_row:element_resonance:<day-id>:<profile-id>:moc
...
matrix_row:element_resonance:<day-id>:<profile-id>:thuy
```

### Builder
```rust
pub fn build_element_resonance_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &ElementResonanceMatrix,
) -> Result<SemanticGraph, String>;
```

### Projection
```rust
pub fn project_element_resonance_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<ElementResonanceMatrix, String>;
```

### Required tests
- one root and 5 rows
- `personal_score`, `effective_resonance`, deficit flags preserved
- net resonance unchanged
- projection parity

## DirectionMergeMatrix

### Root
```text
matrix:direction_merge:<day-id>:<profile-id>
```

### Rows
```text
matrix_row:direction_merge:<day-id>:<profile-id>:bac
...
matrix_row:direction_merge:<day-id>:<profile-id>:tay_bac
```

### Signal nodes
Examples:
- `direction_signal:direction_merge:<...>:bac:kua_favorable`
- `direction_signal:direction_merge:<...>:bac:tai_than`
- `direction_signal:direction_merge:<...>:bac:sat_phuong`

### Builder
```rust
pub fn build_direction_merge_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &DirectionMergeMatrix,
) -> Result<SemanticGraph, String>;
```

### Projection
```rust
pub fn project_direction_merge_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<DirectionMergeMatrix, String>;
```

### Required tests
- one root and 8 rows
- counts and net score preserved
- active signals become signal nodes
- polarity preserved
- projection parity

## DomainDayBoostMatrix

### Root
```text
matrix:domain_day_boost:<day-id>:<profile-id>
```

### Rows
```text
matrix_row:domain_day_boost:<day-id>:<profile-id>:career
...
matrix_row:domain_day_boost:<day-id>:<profile-id>:timing
```

### Builder
```rust
pub fn build_domain_day_boost_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &DomainDayBoostMatrix,
) -> Result<SemanticGraph, String>;
```

### Projection
```rust
pub fn project_domain_day_boost_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<DomainDayBoostMatrix, String>;
```

### Required tests
- one root and 5 rows
- base/day/han/boosted fields preserved
- domain ordering preserved
- projection parity

## Focused Subgraph Helpers

```rust
pub fn extract_day_person_focus(
    graph: &SemanticGraph,
    matrix_id: &str,
    pillar_kind: Option<&str>,
) -> Result<SemanticGraph, String>;
```

```rust
pub fn extract_personal_hour_focus(
    graph: &SemanticGraph,
    matrix_id: &str,
    min_score: Option<u8>,
) -> Result<SemanticGraph, String>;
```

Additional helpers after all matrices land:
- interaction-only graph extraction
- top-row filtering by score / polarity / deficit / domain order

## Execution Order

1. shared interaction graph builder framework
2. `DayPersonMatrix`
3. `PersonalHourMatrix`
4. `ElementResonanceMatrix`
5. `DirectionMergeMatrix`
6. `DomainDayBoostMatrix`
7. unified interaction subgraph helpers

## Exit Criteria

Epic 3 is complete when:
- all five interaction matrices can be graphified
- all five can be projected back to current structs
- parity tests pass
- visualization helpers can operate on all five
- LLM slice helpers can summarize all five
- the interaction layer is ready to feed graph-backed reasoning
