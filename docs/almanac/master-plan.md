# AmLich Almanac Expansion - Master Plan

## Purpose

Build a reliable, explainable, versioned Vietnamese almanac engine on top of `amlich-core`.

The architecture is split into 3 layers:

1. Deterministic calendar and astronomy core
2. Rule-based almanac engine
3. Person and event evaluation engine

## Guiding Principles

- Keep existing lunar conversion behavior stable unless explicitly versioned.
- Keep deterministic math separate from cultural rule tables.
- Encode cultural rules in data packs (JSON), not hard-coded constants.
- Include evidence metadata in outputs (`source_id`, `method`, `profile`, `ruleset_version`).
- Add features in vertical slices with tests and docs per slice.

## Delivery Phases

### Phase 0 - Scope Freeze and Contracts

Goals:
- Freeze v1 boundaries.
- Freeze terminology and output contracts.

Beads:
- R-0001: Glossary normalization
- R-0002: v1 ruleset scope (`vn_baseline_v1`)
- I-0003: Define `RuleSetDescriptor` schema
- D-0004: Document known differences policy

Exit criteria:
- Team agrees on v1 includes/excludes.
- Ruleset contract is stable.

### Phase 1 - Ruleset Infrastructure

Goals:
- Move from baseline-only loading to multi-ruleset loading.
- Make ruleset and version explicit in compute context.

Beads:
- R-1001: Catalog current almanac dataset fields
- I-1002: Implement ruleset registry and loader
- I-1003: Add schema and token validators
- I-1004: Wire `ruleset_id` into day computation
- T-1005: Determinism tests across rulesets
- D-1006: Loader and fallback documentation

Exit criteria:
- Core computes with explicit `ruleset_id`.
- Output includes ruleset profile/version provenance.

### Phase 2 - Day Hoang Dao and Hac Dao

Goals:
- Add day-level hoang dao/hac dao (not only hour-level).

Beads:
- R-2001: Finalize day deity mapping table for v1
- I-2002: Implement day deity resolver
- I-2003: Add day-level output fields
- I-2004: Integrate evidence metadata
- T-2005: Golden date tests by lunar month and day branch
- D-2006: Formula and variant documentation

Exit criteria:
- `DayFortune` includes day-level classification and explanation.

### Phase 3 - Core Taboo Rules

Goals:
- Ship common VN taboo indicators users expect.

Beads:
- R-3001: Normalize tables for Tam Nuong, Nguyet Ky, Sat Chu, Tho Tu
- I-3002: Add taboo rule categories in ruleset schema
- I-3003: Implement taboo resolver with severity (`hard`/`soft`)
- I-3004: Emit structured `taboos[]` with reason/evidence
- T-3005: Boundary tests around lunar month transitions
- D-3006: Explanation conventions

Exit criteria:
- Day output includes standardized taboo flags.

### Phase 4 - Person Rule Foundation

Goals:
- Add person-aware computation model and core personal rules.

Beads:
- R-4001: Normalize age policy (`tuoi_mu`, Tet boundary)
- I-4002: Add `PersonProfile` schema
- I-4003: Implement tuoi xung, tam tai, kim lau, hoang oc
- T-4004: Birth-near-Tet test cases
- D-4005: Age policy documentation

Exit criteria:
- Personal warnings computed deterministically.

### Phase 5 - Cuu Dieu and Yearly Han

Goals:
- Add yearly star and yearly han outputs by age and gender.

Beads:
- R-5001: Freeze cửu diệu and han mapping tables for v1
- I-5002: Add star and han tables to ruleset schema
- I-5003: Implement yearly resolver
- T-5004: Cross-check known examples
- D-5005: Variant caveat documentation

Exit criteria:
- Person yearly output includes `cuu_dieu` and `han` with evidence metadata.

### Phase 6 - Event Evaluation Engine

Goals:
- Evaluate and rank dates for event types with explainable scoring.

Beads:
- R-6001: Define event taxonomy (`cuoi_hoi`, `dong_tho`, `xuat_hanh`, ...)
- R-6002: Define hard filters and scoring weights
- I-6003: Implement evaluation pipeline (hard -> soft -> score)
- I-6004: Add date-range evaluation API surface
- T-6005: Deterministic ranking tests
- D-6006: Scoring transparency documentation

Exit criteria:
- Engine returns ranked days with reasons and violated rules.

### Phase 7 - Solar Term Accuracy Track (Optional)

Goals:
- Keep current fast mode and add optional high-accuracy occurrences.

Beads:
- R-7001: Choose reference strategy for exact term instants
- I-7002: Add occurrence model (`instant_utc`, `instant_local`, `type`)
- I-7003: Add precompute/cache by year and tz
- T-7004: Compare with published term tables
- D-7005: Fast vs accurate mode documentation

Exit criteria:
- Accurate mode available and validated without regressing fast mode.

### Phase 8 - Locale and Variant Presentation

Goals:
- Decouple symbolic display choices from core branch identity.

Beads:
- R-8001: Define locale symbol policy (e.g. Mao -> Meo/Tho)
- I-8002: Move animal labels to locale layer
- I-8003: Add direction variant tables (VN/CN)
- T-8004: Verify locale-only changes do not alter core results
- D-8005: Locale behavior guarantees

Exit criteria:
- Same computed branch can render locale-specific labels safely.

### Phase 9 - OSS Hardening and Release

Goals:
- Stabilize contracts, fixtures, and contributor workflow.

Beads:
- R-9001: Build canonical golden corpus
- I-9002: Finalize API contract versioning
- T-9003: Full parity suite (core/api/ui)
- D-9004: Contributor guide for adding rulesets
- D-9005: Release notes with known differences

Exit criteria:
- Behavior is reproducible and documented by ruleset version.

## Bead Status Model

- TODO
- RESEARCHING
- READY_FOR_IMPL
- IMPLEMENTING
- VERIFYING
- DONE
- BLOCKED

## Definition of Done (Per Bead)

A bead is done only when:

1. Code/data change is complete.
2. Tests pass, including regression scope.
3. Evidence metadata is present in output.
4. Docs/changelog note are updated.
5. No unversioned contract break is introduced.

## Recommendation Roadmap Snapshot (2026-03-09)

The recommendation-system roadmap now has a frozen planning baseline for v1 policy work.

Active planning artifacts:

- `docs/almanac/recommendation-research-reconciliation.md`
- `docs/almanac/recommendation-conflict-triage.md`
- `docs/almanac/recommendation-rule-matrix.json`
- `docs/almanac/recommendation-promotion-order.json`
- `docs/almanac/recommendation-safety-policy.md`
- `docs/almanac/recommendation-pack-architecture.md`
- `docs/almanac/personalized-recommendation-layer.md`

Planning decisions now locked:

- default engine remains precedence-first
- `truc` remains a primary activity-routing signal
- `hoang_dao/hac_dao` remains a bounded modifier
- variant-sensitive rule families should expand through packs or versioned rulesets, not silent source blending
- burial/funeral automation and numeric confidence remain conservative/deferred until explicit policy and test gates are met

Current dependency order for follow-up beads:

1. freeze planning and safety policy artifacts
2. refine default core recommendation logic
3. expand regression corpus and API parity
4. design optional recommendation pack architecture
5. define personalized recommendation layer

## Hybrid Semantic Graph Architecture Track (2026-04-19)

A new hybrid `2.5` architecture track is now active for long-term explainability, visualization, and LLM-ready system design in `amlich-core`. This track does **not** replace the phased delivery plan above; it introduces a migration substrate that existing day, bazi, interaction, reasoning, and recommendation systems can converge onto over time.

### Why this track exists

The current architecture already contains strong structured outputs, but they are split across separate feature slices:

- `almanac/` day facts and rule outputs
- `bazi/` chart and analysis outputs
- `interaction/` personal matrices
- `reasoning/` action-specific explanation graph and decision export
- `almanac/recommendation/` recommendation evidence aggregation

The hybrid semantic-graph track creates a shared substrate so these systems can:

- reuse canonical fact identity
- preserve provenance across surfaces
- graphify matrix and recommendation evidence
- support graph visualization and drill-down
- expose structured LLM-ready semantic slices

### Target architecture

```text
domain producers
  -> semantic_graph
    -> reasoning evaluators
    -> matrix projections
    -> recommendation projections
    -> visualization exports
    -> llm-ready exports
```

### Guiding rule

This is a hybrid migration, not a rewrite:

- existing modules remain authoritative for raw computation in the near term
- `semantic_graph` becomes the shared integration substrate
- current contracts remain available as compatibility projections until parity-backed migration is complete

### Architecture planning artifacts

The semantic graph track is documented in a small, focused set. The table below clarifies the role of each doc and the canonical reading order.

| Doc | Status | Role | Covers |
|-----|--------|------|--------|
| [`hybrid-semantic-graph-plan.md`](hybrid-semantic-graph-plan.md) | Active — source-of-truth | Overall architecture roadmap | Vision, epic map, dependency order, risks |
| [`semantic-graph-v1-spec.md`](semantic-graph-v1-spec.md) | Active — canonical spec | Epic 1 (Foundation) | Core types, IDs, builders, merge rules, views |
| [`interaction-graphification-spec.md`](interaction-graphification-spec.md) | Active — canonical spec | Epic 3 (Interaction Graphs) | Five matrix graphification, ontology extensions, projections |
| [`graph-backed-reasoning-migration-spec.md`](graph-backed-reasoning-migration-spec.md) | Active — canonical spec | Epic 4 (Reasoning) | `ActionEvaluator` trait, `InitiationOpening` migration, axis mapping |
| [`graph-backed-recommendation-evidence-spec.md`](graph-backed-recommendation-evidence-spec.md) | Active — canonical spec | Epic 5 (Recommendation) + convergence | Recommendation evidence graph, activity nodes, and the reasoning–recommendation convergence strategy |

**Reading order for new sessions/agents:** Start with `hybrid-semantic-graph-plan.md` for the big picture, then `semantic-graph-v1-spec.md` for foundation types, then the epic-specific spec you are working on.

Archived planning docs for earlier work are in [`docs/archived/plans/`](../archived/plans/) and are historical reference only — do not treat them as active specs.

### Epic map

1. **Semantic Graph Foundation**
2. **Domain Fact Graphification**
3. **Interaction Matrices as Graph Projections**
4. **Graph-backed Reasoning**
5. **Graph-backed Recommendation Evidence**
6. **Visualization Contract**
7. **LLM-ready Semantic Views**
8. **Migration, Compatibility, and Validation**

### Recommended dependency order

1. semantic graph foundation
2. fact graphification
3. interaction graphification
4. graph-backed reasoning
5. graph-backed recommendation evidence
6. visualization contract
7. llm-ready views
8. compatibility/parity hardening throughout

### Relationship to existing phases

This architecture track overlays the existing phase plan rather than replacing it. In practice:

- Phase 4-6 outputs are key inputs to semantic graphification
- recommendation-system planning artifacts remain valid but can later be graph-backed
- explanation and personalization work should prefer canonical graph-connected identities when expanding contracts

### Expected payoff

If executed successfully, this track should make `amlich-core`:

- more explainable
- easier to visualize
- more coherent across reasoning and recommendation surfaces
- more suitable for structured LLM consumers
- easier to extend without creating new isolated explanation islands
