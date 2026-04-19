# Canonical Reasoning Graph Schema and Field Semantics

## Status

Developer documentation — amlich-f35. Complement to `explanation-hierarchy.md`.

## Overview

The canonical reasoning graph is a structured bundle that captures how an
almanac engine derives its recommendation for a specific action on a specific
day. It consists of:

- A **reasoning graph** — nodes connected by directed edges, each carrying
  evidence provenance
- A **decision export** — the distilled conclusion and supporting rationale
- **Axis scores** — six-dimensional signal scores that feed the conclusion

This doc describes every type, enum, and field in the reasoning graph schema
with their semantics and relationships. It is intended as the authoritative
reference for consumers building on top of these contracts.

---

## Core Types

### InitiationOpeningReasoningBundle

The top-level container. All canonical reasoning surfaces for initiation
opening actions should read from this bundle.

```rust
pub struct InitiationOpeningReasoningBundle {
    pub decision: InitiationOpeningDecision,      // internal decision struct
    pub decision_export: InitiationOpeningDecisionExport, // consumer-facing export
    pub graph: ReasoningGraphExport,              // full graph for drill-down
}
```

`decision` is the internal production struct. `decision_export` is the
stable consumer-facing view with richer annotation. Consumers should use
`decision_export` rather than `decision` directly.

---

## Reasoning Graph

### ReasoningGraph

The raw graph before export transformation. Used internally during graph
construction.

```rust
pub struct ReasoningGraph {
    pub action_id: ActionId,
    pub nodes: Vec<ReasoningNode>,
    pub edges: Vec<ReasoningEdge>,
}
```

### ReasoningGraphExport

The serialized graph delivered to consumers. The export form adds computed
fields (`axis`, `severity`, `tags`, `weight`) that require domain knowledge
to derive — these are computed once at export time in `export_reasoning_graph`.

```rust
pub struct ReasoningGraphExport {
    pub action_id: ActionId,
    pub nodes: Vec<ReasoningNodeExport>,
    pub edges: Vec<ReasoningEdgeExport>,
}
```

### ReasoningNode

Internal node representation.

```rust
pub struct ReasoningNode {
    pub id: String,
    pub kind: NodeKind,
    pub summary_vi: String,
    pub severity: Option<String>,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}
```

### ReasoningNodeExport

Serialized node with computed annotations.

```rust
pub struct ReasoningNodeExport {
    pub id: String,
    pub kind: NodeKind,
    pub axis: Option<InterpretedAxis>,       // derived at export time
    pub severity: Option<ReasoningNodeSeverity>, // derived at export time
    pub tags: Vec<String>,                  // derived at export time
    pub summary_vi: String,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}
```

### ReasoningEdge

Internal edge representation.

```rust
pub struct ReasoningEdge {
    pub from_node_id: String,
    pub to_node_id: String,
    pub effect: EdgeEffect,
    pub justification: ReasoningEdgeJustification,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}
```

### ReasoningEdgeExport

Serialized edge with computed `weight` and `tags`.

```rust
pub struct ReasoningEdgeExport {
    pub from_node_id: String,
    pub to_node_id: String,
    pub effect: EdgeEffect,
    pub weight: i32,            // derived at export time: Overrides=2, others=1
    pub justification: ReasoningEdgeJustification,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
    pub tags: Vec<String>,       // derived at export time
}
```

---

## Enums

### ActionId

```rust
pub enum ActionId {
    InitiationOpening,
}
```

Currently only one action kind is modeled. The enum exists to allow future
extension without breaking the schema.

### NodeKind

What class of thing a node represents.

```rust
pub enum NodeKind {
    Fact,              // raw almanac/bazi fact
    InterpretedSignal, // derived signal assigned to an axis
    DecisionTarget,    // the final decision verdict node
}
```

| Variant | Meaning |
|---------|---------|
| `Fact` | A direct observation from almanac data: trực name, can chi, taboo presence, star result, etc. |
| `InterpretedSignal` | A derived signal node that summarizes domain knowledge for one axis. Always named `signal.<axis_name>`. |
| `DecisionTarget` | The synthesized verdict node. The terminal node of the graph. |

### EdgeEffect

How one node influences another.

```rust
pub enum EdgeEffect {
    Supports,      // favorable contribution (weight 1)
    Weakens,       // mild unfavorable (weight 1)
    Overrides,     // overrides other signals regardless (weight 2)
    ConflictsWith, // directly contradicts another node
    Conditions,    // prerequisite gating
}
```

| Variant | Weight | Semantics |
|---------|--------|-----------|
| `Supports` | 1 | Node adds favorable weight to the target axis |
| `Weakens` | 1 | Node reduces favorable weight |
| `Overrides` | 2 | Node sets the axis verdict regardless of other signals |
| `ConflictsWith` | 1 | Node is in tension with the target; causes caution |
| `Conditions` | 0 | Node gates whether another node is evaluated at all |

`Overrides` edges carry weight 2 and always produce a caution-level signal
in the `override_factors` array. `ConflictsWith` edges always produce a
`conflict_note`.

### InterpretedAxis

The six signal dimensions the engine evaluates.

```rust
pub enum InterpretedAxis {
    Support,
    Resistance,
    Stability,
    PersonalAlignment,
    TimingFit,
    ContextClarity,
}
```

| Axis | Signal node ID | Score meaning |
|------|---------------|---------------|
| `Support` | `signal.support` | Higher = more overall favorable energy |
| `Resistance` | `signal.resistance` | Lower = less unfavorable pressure |
| `Stability` | `signal.stability` | Higher = more consistent day for routine action |
| `PersonalAlignment` | `signal.personal_alignment` | Higher = better match with birth chart |
| `TimingFit` | `signal.timing_fit` | Higher = suggested hours are high quality |
| `ContextClarity` | `signal.context_clarity` | Higher = signals are unambiguous |

Each axis has a dedicated signal node (`signal.<name>`) that is the
aggregation target for all edges pointing to it.

### ReasoningNodeSeverity

Computed from node `id` and internal `severity` string at export time.

```rust
pub enum ReasoningNodeSeverity {
    Auspicious,
    Inauspicious,
    HardTaboo,
    SoftTaboo,
    HoangDao,
    HacDao,
}
```

| Variant | Derivation |
|---------|-----------|
| `Auspicious` | `fact.day.truc` with internal severity "cat", or `fact.day.nhi_thap_bat_tu` containing "cát tinh", or `fact.day.hoang_dao_hours` with count > 0 |
| `Inauspicious` | `fact.day.truc` with internal severity "hung", or `fact.day.nhi_thap_bat_tu` containing "sát tinh", or `fact.day.xung_hop` starting with "Xung" without "hợp" |
| `HardTaboo` | `fact.day.taboos` with internal severity "hard" |
| `SoftTaboo` | `fact.day.taboos` with internal severity "soft" |
| `HoangDao` | `fact.day.day_deity` with internal severity "hoang_dao" |
| `HacDao` | `fact.day.day_deity` with internal severity "hac_dao" |

### ReasoningEvidenceSourceFamily

The provenance category for a node or edge.

```rust
pub enum ReasoningEvidenceSourceFamily {
    Snapshot,      // basic day snapshot data
    Interaction,   // day-person interaction matrix
    Bazi,          // bazi chart analysis
    Axis,          // axis synthesis step
    AlmanacRule,   // rule-based recommendation
    Insight,       // derived insight
    Derived,       // computed from other evidence
}
```

Consumers use `source_family` to render appropriate attribution labels or
icons. For example, `Snapshot` sources might display a calendar icon
while `Bazi` sources display a chart icon.

### ReasoningEdgeJustification

Why an edge exists — the specific rule or relationship that connects two
nodes.

```rust
pub enum ReasoningEdgeJustification {
    FavorableDaySignal,
    TrucActivitySupport,
    TrucActivityConflict,
    DayDeitySupport,
    StarSupport,
    TabooPressure,
    TabooStabilityPenalty,
    TabooContextPenalty,
    ClashPressure,
    ClashStabilityPenalty,
    HoangDaoHourSupport,
    PersonalDayAlignment,
    PersonalHourAlignment,
    MixedSignalConflict,
    AvailableContextSupport,
}
```

| Variant | Effect typically | Meaning |
|---------|-----------------|---------|
| `FavorableDaySignal` | Supports | General favorable day energy |
| `TrucActivitySupport` | Supports | Trực activity level is favorable |
| `TrucActivityConflict` | ConflictsWith | Trực activity level is unfavorable |
| `DayDeitySupport` | Supports | Day deity is favorable |
| `StarSupport` | Supports | Star configuration is supportive |
| `TabooPressure` | Weakens | Taboo adds unfavorable pressure |
| `TabooStabilityPenalty` | Weakens | Taboo reduces stability score |
| `TabooContextPenalty` | Weakens | Taboo reduces context clarity |
| `ClashPressure` | ConflictsWith | Xung/hợp clash with personal chart |
| `ClashStabilityPenalty` | Weakens | Clash reduces stability |
| `HoangDaoHourSupport` | Supports | Hoàng Đạo hours are available |
| `PersonalDayAlignment` | Supports | Day matches personal birth chart |
| `PersonalHourAlignment` | Supports | Top hour matches personal chart |
| `MixedSignalConflict` | ConflictsWith | Conflicting signals cancel out |
| `AvailableContextSupport` | Supports | Context signals are clear and supportive |

### ReasoningConclusionSemantic

The semantic variant of the final conclusion. Encodes the override/conflict
pattern that produced the verdict.

```rust
pub enum ReasoningConclusionSemantic {
    OverrideAvoid,       // override forced an avoid verdict
    OverrideCautious,    // override forced a cautious verdict
    ConflictedCautious,  // conflicts forced a cautious verdict
    ResistanceLedCautious, // resistance axis drove cautious
    FavorableClear,      // strong favorable signals, clear context
    FavorableContextual,  // favorable but context had to be considered
}
```

### RecommendationBucket

```rust
pub enum RecommendationBucket {
    Avoid,
    Cautious,
    Mixed,
    Favorable,
}
```

Maps to overall day suitability for the action.

### DecisionConfidence

```rust
pub enum DecisionConfidence {
    Low,
    Medium,
    High,
}
```

Overall certainty of the decision.

---

## Evidence Envelope

```rust
pub struct ReasoningEvidenceEnvelope {
    pub source_family: ReasoningEvidenceSourceFamily,
    pub source_id: String,
    pub method: String,
    pub note: Option<String>,
}
```

| Field | Semantics |
|-------|-----------|
| `source_family` | Category of the data source |
| `source_id` | Identifier of the specific source (e.g., ruleset id, matrix name) |
| `method` | The computation or lookup method used to derive this node/edge |
| `note` | Optional human-readable annotation |

Every node and every edge carries one or more evidence envelopes. Consumers
walking the graph should accumulate evidence from both nodes and edges to
build the full provenance trail for a conclusion.

---

## Decision Export

### InitiationOpeningDecisionExport

The canonical consumer-facing decision summary.

```rust
pub struct InitiationOpeningDecisionExport {
    pub primary_conclusion: String,
    pub recommendation_bucket: RecommendationBucket,
    pub confidence: DecisionConfidence,
    pub context_is_clear: bool,
    pub semantic: ReasoningConclusionSemantic,
    pub strongest_supports: Vec<ReasoningNote>,
    pub strongest_resistances: Vec<ReasoningNote>,
    pub override_factors: Vec<ReasoningNote>,
    pub conflict_notes: Vec<ReasoningNote>,
    pub suggested_hours: Vec<String>,
    pub suggested_directions: Vec<String>,
    pub axis_scores: Vec<ReasoningAxisScore>,
}
```

| Field | Type | Semantics |
|-------|------|-----------|
| `primary_conclusion` | String | Vietnamese verdict text |
| `recommendation_bucket` | Bucket | `avoid \| cautious \| mixed \| favorable` |
| `confidence` | Confidence | `low \| medium \| high` |
| `context_is_clear` | bool | True when `signal.context_clarity` axis is strong |
| `semantic` | Semantic | Conclusion variant encoding how verdict was reached |
| `strongest_supports` | Vec<ReasoningNote> | Top supporting nodes with `summary_vi` |
| `strongest_resistances` | Vec<ReasoningNote> | Top resisting nodes |
| `override_factors` | Vec<ReasoningNote> | Nodes with `Overrides` edges — shown as cautions |
| `conflict_notes` | Vec<ReasoningNote> | Nodes with `ConflictsWith` edges |
| `suggested_hours` | Vec<String> | Top-3 chi names for favorable hours |
| `suggested_directions` | Vec<String> | Top-3 direction names |
| `axis_scores` | Vec<ReasoningAxisScore> | Six-axis scores (see below) |

### ReasoningNote

A concise node summary for use in `strongest_supports`, `strongest_resistances`,
`override_factors`, and `conflict_notes`.

```rust
pub struct ReasoningNote {
    pub node_id: Option<String>,
    pub summary_vi: String,
    pub tags: Vec<String>,
}
```

### ReasoningAxisScore

```rust
pub struct ReasoningAxisScore {
    pub axis: InterpretedAxis,
    pub score: f32,                    // 0.0–1.0
    pub strongest_node_id: Option<String>,
    pub strongest_summary_vi: Option<String>,
}
```

| Field | Semantics |
|-------|-----------|
| `axis` | Which axis this score describes |
| `score` | 0.0–1.0 normalized score; higher is better for all axes except `Resistance` which should be displayed inverted |
| `strongest_node_id` | Node ID that contributed most to this score |
| `strongest_summary_vi` | Vietnamese text of the strongest node — used as axis bar label in UIs |

---

## Matrix Types

Matrix types live in `amlich-core/src/interaction/types.rs`. They are
separate from the reasoning graph but are co-delivered in the same bundle
when matrix outputs are requested.

### DayPersonMatrix

Day-to-person pillar interaction across the four pillars (year, month, day,
hour master).

```
day_canchi:         String
day_master:        String
day_to_day_master: ThapThanResult
pillars:           Vec<PillarInteraction>
evidence:          RuleEvidence
```

Each `PillarInteraction` contains:
- `pillar`: which pillar
- `pillar_canchi`: full can chi label
- `thap_than`: thập thần relation from day stem to pillar stem
- `branch_relation`: xung/hợp/tam hợp/tương hại/tương hình flags
- `element_interaction`: element generation/controlled relation

### PersonalHourMatrix

12 traditional hours ranked by personal compatibility.

```
day_canchi:       String
day_master:       String
birth_hour_chi:   String
weak_element:     FiveElement
hours:            Vec<PersonalHourEntry>  // sorted by chi_index 0..11
evidence:         RuleEvidence
```

Each `PersonalHourEntry` includes:
- `chi`, `canchi`, `time_range`: hour identification
- `is_hoang_dao`: generic auspiciousness
- `star_name`: 12-star governing the hour
- `thap_than_to_day_master`, `branch_relation_to_birth_hour`,
  `element_interaction`: personal compatibility signals
- `supports_weak_element`: whether this hour helps the person's deficit element
- `score`: composite 0–100 score

### ElementResonanceMatrix

Five-element resonance between the day and the person's chart.

```
day_canchi:        String
day_element:       FiveElement
month_chi:         String
season_factor:     f32
entries:           Vec<ElementResonanceEntry>  // one per element
net_resonance:     f32  // sum of effective_resonance
evidence:         RuleEvidence
```

Each `ElementResonanceEntry` includes:
- `element`: which element
- `personal_score`: person's score for this element from chart
- `relation_to_day`: day stem element → this element coefficient
- `season_factor`: seasonal modifier for day element in current month
- `effective_resonance`: relation × season_factor
- `is_deficit`: true when personal_score ≤ 15
- `day_helps_deficit`: true when is_deficit AND effective_resonance > 0

### DirectionMergeMatrix

Eight compass directions with signal breakdown.

```
day_canchi:   String
kua_number:   u8
entries:      Vec<DirectionEntry>  // one per compass direction
evidence:    RuleEvidence
```

Each `DirectionEntry` includes:
- `direction`: Vietnamese direction name
- `signals: Vec<DirectionSignal>`: all active signals for this direction
  (KuaFavorable, KuaUnfavorable, TaiThan, HyThan, PhucThan, SatPhuong)
- `favorable_count`, `unfavorable_count`, `net_score`

Direction signals: `KuaFavorable`, `KuaUnfavorable`, `TaiThan`, `HyThan`,
`PhucThan`, `SatPhuong`.

### DomainDayBoostMatrix

Five life domains with day-level boost modifiers.

```
day_canchi:  String
entries:     Vec<DomainDayBoostEntry>  // career, wealth, relationship, health, timing
evidence:    RuleEvidence
```

Each `DomainDayBoostEntry` includes:
- `domain`: life domain name
- `base_score`: Bazi domain score 0–100
- `day_modifier`: modifier from stars/trực/thần
- `han_penalty`: yearly Hạn penalty (0.0 if none, negative if active)
- `boosted_score`: base × (1 + day_modifier + han_penalty), clamped 0–100

---

## Tag System

Tags are computed at export time and attached to both nodes and edges.
They enable surface-level filtering and visual treatment without re-reading
the graph.

### Node tags

| Tag | Applied when |
|-----|--------------|
| `personal` | node ID starts with `fact.personal.` |
| `day` | node ID starts with `fact.day.` |
| `signal` | node ID starts with `signal.` |
| `support` | node ID is one of: truc, day_deity, nhi_thap_bat_tu, hoang_dao_hours, signal.support |
| `resistance` | node ID is one of: taboos, xung_hop, signal.resistance |
| `timing` | node ID is one of: travel_directions, signal.timing_fit |
| `context` | node ID is `fact.day.solar_term` or `signal.context_clarity` |

### Edge tags

| Tag | Applied when |
|-----|--------------|
| `override` | edge effect is `Overrides` |
| `conflict` | edge effect is `ConflictsWith` |
| `support` | edge target is `signal.support` |
| `resistance` | edge target is `signal.resistance` |
| `context` | edge target is `signal.context_clarity` |

---

## Schema Version and Stability

The reasoning graph schema does not currently carry an explicit version
field. Consumers should treat the presence and shape of `InitiationOpeningReasoningBundle`
as the version identifier. If the schema changes, a new bundle structure
will be introduced with a versioned name.

Axis scores and the tag system are derived at export time by `export_reasoning_graph`
in `amlich-core/src/reasoning/export.rs`. Consumers who need stable axis
interpretation should read the exported values rather than recomputing them.

---

## Relationship to Explanation Hierarchy

`explanation-hierarchy.md` defines the four presentation layers that consume
this schema:

| Layer | Primary sources |
|-------|----------------|
| Headline | `decision_export.primary_conclusion`, `recommendation_bucket`, `confidence`, `semantic` |
| Rationale | `decision_export.axis_scores`, `strongest_supports`, `strongest_resistances` |
| Cautions | `decision_export.override_factors`, `conflict_notes`, graph node `severity` |
| Drill-down | `graph.nodes`, `graph.edges`, matrix types |

This doc is the schema-level reference for those layers.

For rules on how these contracts may evolve over time, see
`docs/almanac/contract-evolution-guidelines.md`.
