# Explanation Hierarchy for Canonical Reasoning Outputs

## Status

Design document — amlich-g3j.

## Goal

Define a stable presentation model that translates canonical engine outputs
(`decision_export`, reasoning graph, matrix outputs) into user-facing
explanation layers. This lets all consumer surfaces share the same layered
interpretation contract without duplicating derivation logic.

## Non-goals

- This doc does not mandate specific UI component shapes.
- This doc does not change product behavior or scoring semantics.
- This doc does not cover TTY/TUI layout details (those belong to the surfaces
  that consume the layers).

---

## Layered Explanation Model

Every canonical reasoning output is designed to be consumed through four
presentation layers:

| Layer | Purpose | Typical content | Surfaces |
|-------|---------|----------------|----------|
| **Headline** | One-line verdict | `primary_conclusion`, `recommendation_bucket`, severity badge | TTY line 1, notification, badge |
| **Rationale** | Why the verdict is what it is | `strongest_supports`, `strongest_resistances`, axis scores | Main body text, cards |
| **Cautions** | Warnings, hard constraints, override factors | `override_factors`, `conflict_notes`, `warnings` from bazi/matrix | Alerts, footnotes, banners |
| **Drill-down** | Full provenance, graph internals, evidence chains | Raw `graph` nodes/edges, `evidence` envelopes, matrix rows | Detail panels, logs, "explain more" links |

### Headline

The headline distills the entire reasoning bundle into a single actionable
statement. It uses:

- `decision_export.primary_conclusion` — the synthesized verdict string
- `decision_export.recommendation_bucket` — one of `avoid | cautious | mixed | favorable`
- `decision_export.confidence` — `low | medium | high`
- `decision_export.semantic` — the conclusion semantic variant
  (`OverrideAvoid`, `FavorableClear`, etc.)

Renderers should present the bucket prominently (color, icon, badge) and
the confidence as a secondary qualifier. The `semantic` field drives
presentation variant logic (e.g., "override" badges look different from
"favorable" badges).

#### Headline fields

```
headline
├── primary_conclusion     String        — verdict text (Vietnamese source, renderers localize)
├── recommendation_bucket  Bucket        — avoid|cautious|mixed|favorable
├── confidence            Confidence    — low|medium|high
├── semantic              Semantic      — conclusion variant
├── suggested_hours       Vec<String>   — top hour picks (optional)
├── suggested_directions  Vec<String>   — top direction picks (optional)
└── severity              String        — legacy alias for bucket in some surfaces
```

### Rationale

The rationale explains the "why". It draws from graph axis scores and
the `strongest_*` / `override_*` / `conflict_*` arrays in `decision_export`.

The reasoning graph has six core axes. Each axis has a score (0.0–1.0) and
a `strongest_node_id` pointing to the graph node that drove that score.
Consumers display axis scores as a summary bar or radar chart, then
detail each axis by rendering the linked node's `summary_vi` and its
`evidence` envelope.

#### Rationale fields

```
rationale
├── axis_scores              Vec<AxisScore>     — six axes with score + strongest_node
│   ├── axis                 InterpretedAxis   — support|resistance|stability|...
│   ├── score                f32               — 0.0–1.0
│   └── strongest_summary_vi String?           — linked node verdict
├── strongest_supports       Vec<ReasoningNote> — favorable node summaries
├── strongest_resistances    Vec<ReasoningNote> — unfavorable node summaries
└── context_is_clear        bool               — whether context signal is clear
```

Rendering note: axis scores are the primary drill-down entry point. A surface
showing a radar chart or bar chart should map each bar to one
`InterpretedAxis`. The `strongest_summary_vi` of the top-scoring node for
each axis is the text label for that bar.

### Cautions

Cautions surface hard constraints and warning-level signals. They come from:

- `override_factors` in `decision_export` — nodes with `EdgeEffect::Overrides`
- `conflict_notes` in `decision_export` — nodes with `EdgeEffect::ConflictsWith`
- `warnings` in `BaziAdvisoryReport`
- `hard_taboo` / `soft_taboo` tagged nodes in the graph
- Matrix `han_penalty` scores (when active)

Each caution carries its own `node_id`, `summary_vi`, and `tags` so
renderers can group by tag (`override`, `taboo`, `conflict`) and apply
appropriate visual treatment.

#### Cautions fields

```
cautions
├── override_factors  Vec<ReasoningNote> — edges/nodes with override effect
├── conflict_notes   Vec<ReasoningNote> — edges/nodes with conflict effect
├── graph_taboo_nodes Vec<ReasoningNodeExport> — nodes tagged hard_taboo|soft_taboo
├── bazi_warnings    Vec<String>       — from BaziAdvisoryReport.warnings
└── han_penalty_note String?           — when DomainDayBoostMatrix has active hạn
```

### Drill-down

Drill-down exposes the full graph and evidence provenance. This layer is
intended for:

- "Show full reasoning" UI affordances
- Debug/export flows
- API consumers who want to re-interpret the graph

The raw `graph` (`ReasoningGraphExport`) and matrix structs are the
canonical data for this layer. Renderers should not need to re-derive
anything — they walk the graph nodes/edges and matrix rows directly.

#### Drill-down fields

```
drill_down
├── graph
│   ├── action_id   ActionId
│   ├── nodes       Vec<ReasoningNodeExport>
│   │   ├── id, kind, axis, severity, tags, summary_vi
│   │   └── evidence: Vec<ReasoningEvidenceEnvelope>
│   └── edges       Vec<ReasoningEdgeExport>
│       ├── from_node_id, to_node_id, effect, weight
│       ├── justification, tags
│       └── evidence: Vec<ReasoningEvidenceEnvelope>
├── matrices
│   ├── day_person_matrix   DayPersonMatrix?
│   ├── personal_hour_matrix PersonalHourMatrix?
│   ├── element_resonance_matrix ElementResonanceMatrix?
│   ├── direction_merge_matrix DirectionMergeMatrix?
│   └── domain_day_boost_matrix DomainDayBoostMatrix?
└── evidence_envelopes
    └── Vec<EvidenceEnvelope>  — top-level evidence for the bundle
```

---

## Decision Export Semantics

`InitiationOpeningDecisionExport` is the canonical top-level reasoning
summary. All headline-layer consumers should read from this structure
before falling back to legacy advisory fields.

### Field semantics

| Field | Type | Meaning |
|-------|------|---------|
| `primary_conclusion` | String | Synthesized Vietnamese verdict. Renderers localize. |
| `recommendation_bucket` | Bucket | `avoid \| cautious \| mixed \| favorable` — top-level suitability |
| `confidence` | Confidence | `low \| medium \| high` — overall certainty |
| `semantic` | Semantic | Conclusion variant encoding override/conflict patterns |
| `context_is_clear` | bool | True when `signal.context_clarity` axis is strong |
| `strongest_supports` | Vec<ReasoningNote> | Top supporting node summaries |
| `strongest_resistances` | Vec<ReasoningNote> | Top resisting node summaries |
| `override_factors` | Vec<ReasoningNote> | Nodes/edges with override weight — shown as cautions |
| `conflict_notes` | Vec<ReasoningNote> | Nodes/edges with conflict weight — shown as cautions |
| `suggested_hours` | Vec<String> | Top-3 chi names for the day's favorable hours |
| `suggested_directions` | Vec<String> | Top-3 direction names |
| `axis_scores` | Vec<ReasoningAxisScore> | Six-axis scores keyed by `InterpretedAxis` |

### Axis score semantics

`InterpretedAxis` is the primary ordering dimension for rationale
presentation:

| Axis | Signal node ID | Meaning |
|------|---------------|---------|
| `Support` | `signal.support` | Overall favorable energy for the action |
| `Resistance` | `signal.resistance` | Overall unfavorable energy |
| `Stability` | `signal.stability` | Consistency of the day for routine vs novel action |
| `PersonalAlignment` | `signal.personal_alignment` | Match with personal birth chart (requires birth data) |
| `TimingFit` | `signal.timing_fit` | Quality of suggested hours and time windows |
| `ContextClarity` | `signal.context_clarity` | How unambiguous the signals are |

Each `ReasoningAxisScore` carries a `score` (0.0–1.0) where higher is
better for `Support`, `PersonalAlignment`, `TimingFit`, `ContextClarity`;
and lower is better for `Resistance` (display inverted). `Stability` is
bidirectional.

---

## Matrix Outputs and Their Layer Mapping

Matrix outputs are first-class reasoning data, not supplementary. They
populate all four layers depending on which field is referenced.

| Matrix | Headline use | Rationale use | Cautions use | Drill-down use |
|--------|-------------|---------------|--------------|----------------|
| `DayPersonMatrix` | — | Per-pillar interaction summary | Conflict flags (xung/tương hại) | Full 4-row pillar table |
| `PersonalHourMatrix` | `suggested_hours` | Top hour entry detail | Low-score hour warnings | All 12 hour rows |
| `ElementResonanceMatrix` | — | `net_resonance` as axis backdrop | `day_helps_deficit` alerts | All 5 element rows |
| `DirectionMergeMatrix` | `suggested_directions` | Per-direction signal breakdown | SatPhuong / KuaUnfavorable | Full 8-direction table |
| `DomainDayBoostMatrix` | Domain boost highlights | Per-domain boosted scores | Active `han_penalty` rows | All 5 domain rows |

---

## Graph Node and Edge Semantics

### Node kinds

| `NodeKind` | Meaning |
|------------|---------|
| `Fact` | Raw almanac or bazi fact (trực, can chi, taboo, star, etc.) |
| `InterpretedSignal` | A derived signal after axis assignment (e.g. `signal.support`) |
| `DecisionTarget` | The target verdict node (`target.decision`) |

### Edge effects

| `EdgeEffect` | Meaning |
|-------------|---------|
| `Supports` | Node contributes favorably to the target axis |
| `Weakens` | Node has mild unfavorable effect |
| `ConflictsWith` | Node directly contradicts another |
| `Conditions` | Node is a prerequisite gating another |
| `Overrides` | Node overrides other signals regardless of score |

`Overrides` edges carry weight 2; all others carry weight 1.

### Evidence envelope

`ReasoningEvidenceEnvelope` is the provenance record for every node and
edge:

```
source_family: Snapshot | Interaction | Bazi | Axis | AlmanacRule | Insight | Derived
source_id:    String   — identifies the source fact or computation
method:       String   — the computation or lookup method used
note:         String?  — optional human-readable annotation
```

Renderers use `source_family` to display an appropriate icon or label
(e.g., "Bazi" for bazi-sourced nodes, "AlmanacRule" for rule-based nodes).

---

## Consumer Guidance

### Layer access order

Surfaces that need only a quick verdict should read only **Headline**
fields. Surfaces that need to explain the verdict should additionally read
**Rationale**. Surfaces that need full provenance or want to support
"show full reasoning" should additionally read **Drill-down**.

**Cautions** should be shown whenever `override_factors` or
`conflict_notes` is non-empty, regardless of what other layers are
displayed.

### Field aliasing

Some legacy surfaces expose `advisory.summary`, `advisory.severity`,
`advisory.top_signals` as aliases for headline fields. These are
maintained for backward compatibility but are not the canonical source.
Canonical consumers should read from `decision_export` directly.

### Matrix presence

Matrix fields are gated. When a surface does not request matrix data
(via `include=matrix` or equivalent), matrix fields are absent from the
response. Renderers must handle absent matrices gracefully — do not
render matrix sections when the data is missing.

### Localization

`primary_conclusion` and all `summary_vi` fields are Vietnamese source
text. Renderers are responsible for localizing to the user's locale.
The engine does not currently provide `summary_en` for the initiation
opening bundle (unlike some other surfaces that provide both `_vi` and
`_en`).

---

## Next bead: amlich-f35

This document is the foundation for **amlich-f35** ("Document canonical
reasoning graph schema and field semantics"). Where this doc defines the
*presentation model*, amlich-f35 will document the *schema internals*:
full type definitions for `ReasoningGraph`, `ReasoningNode`, `ReasoningEdge`,
all enums, and the matrix type definitions, with field-level semantics
and relationships.

---

## Summary

The explanation hierarchy maps canonical engine outputs to four layers:

1. **Headline** — `decision_export.primary_conclusion` + bucket + confidence
2. **Rationale** — axis scores + `strongest_supports/resistances`
3. **Cautions** — `override_factors` + `conflict_notes` + warnings
4. **Drill-down** — raw graph + matrix rows + evidence envelopes

Consumer surfaces read from the top down, stopping when the information
needs of the surface are satisfied. The schema for layers 3–4 is defined
by `InitiationOpeningReasoningBundle` and the matrix types in
`amlich-core/src/interaction/types.rs`.
