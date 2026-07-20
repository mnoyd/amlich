---
phase: 19-recommends-offering-semantic-graph-node
plan: 02
subsystem: semantic-graph
tags: [offering, ritual, semantic-graph, dual-source-provenance, serde-json-payload, huyen-khong, vn-folk-ritual, recommendsoffering]

# Dependency graph
requires:
  - phase: 19-recommends-offering-semantic-graph-node
    plan: 01
    provides: "OfferingRef struct + SourceId alias + DaySnapshot.offering_refs/offerings fields (schema-lock-before-builder)"
provides:
  - "NodeConcept::Offering + EdgeConcept::RecommendsOffering across all 6 ontology slice locations"
  - "Additive payload: Option<serde_json::Value> field on SemanticNode (INT-08 SC#2 literal)"
  - "RitualMetadata + CrossSourceCure schema structs (INT-09 corpus augmentation)"
  - "Annotated van-khan-tet-day-du ritual entry with huyen-khong cross_source_curing"
  - "add_offering_facts helper emitting Offering nodes + RecommendsOffering edges with dual-source provenance"
  - "Payload population on aggregate Ritual node (INT-08 SC#2 literal interpretation)"
  - "New nodes_mut() accessor on SemanticGraph for post-population of additive fields"
affects:
  - "Phase 19 plan 19-03 — external-crate black-box tests"
  - "Any future semantic-graph consumers querying Offering nodes or RecommendsOffering edges"
  - "Future corpus-augmentation patterns building on cross_source_curing discipline"

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Additive Option<T> + #[serde(default, skip_serializing_if = 'Option::is_none')] for v1.6 schema-lock discipline"
    - "Two-call track_provenance pattern for INT-09 dual-source edge provenance"
    - "HashSet<(String, String)> edge-dedup with NO parallel provenance-dedup helper"
    - "Post-population of additive fields via new nodes_mut() accessor"
    - "Generic serde_json::Value payload (Option B from 19-RESEARCH.md) over typed enum"
    - "extend locked test pattern: v16_concepts_present_in_ontology_slices sibling of v15 test"

key-files:
  created: []
  modified:
    - "crates/amlich-core/src/semantic_graph/ontology.rs"
    - "crates/amlich-core/src/semantic_graph/node.rs"
    - "crates/amlich-core/src/semantic_graph/graph.rs"
    - "crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs"
    - "crates/amlich-core/src/semantic_graph/views/helpers.rs"
    - "crates/amlich-core/src/semantic_graph/views/visualization.rs"
    - "crates/amlich-core/src/rituals/schema.rs"
    - "crates/amlich-core/data/rituals/tet-nguyen-dan.json"

key-decisions:
  - "Generic serde_json::Value payload on SemanticNode (Option B) over typed RitualNodePayload enum (Option C) — avoids the cost of a typed enum and matches the v1.5 discipline of additive Option<T> fields"
  - "CrossSourceCure.element_cure_for stays as free-form String (Vietnamese like 'Kim') for now — the existing Element enum at almanac/fengshui/types.rs uses lowercase English ('metal') and a typed enum would couple unrelated domains"
  - "post-population via new nodes_mut() accessor (additive) over constructing the Ritual node with payload (which would require knowing offering_refs at SemanticNode::new() time) — preserves the established builder pattern"
  - "Two track_provenance calls (one per source) reuses the v1.5 multi-source append-pattern — NO parallel dedup helper introduced"
  - "Rationale carried on the EDGE provenance note (Blocker 4 fix) — not just on the Offering node provenance"
  - "Annotated van-khan-tet-day-du (5 offerings) as the canonical Tết entry most likely to surface in any Tết-day applicable_rituals — maximizes test exposure"

patterns-established:
  - "INT-09 dual-source provenance pattern: corpus entry has metadata.cross_source_curing → builder emits 1 extra track_provenance call per cure annotation on the same edge"
  - "INT-08 SC#2 literal payload: aggregate Ritual node carries {offering_refs, offerings} JSON value derived from DaySnapshot fields"
  - "Edge dedup via HashSet<(from_id, to_id)> when iterating a parent's children — keys on edge endpoints, NOT on provenance"

requirements-completed: [INT-07, INT-08, INT-09]

# Metrics
duration: 6min
completed: 2026-07-15
---

# Phase 19 Plan 2: Offering node + RecommendsOffering edge + INT-09 dual-source provenance Summary

**Wired NodeConcept::Offering + EdgeConcept::RecommendsOffering across the ontology, populated SemanticNode::payload with INT-08 SC#2 literal ritual+offerings JSON, and implemented INT-09 dual-source edge provenance via corpus-augmentation of `van-khan-tet-day-du` with a `huyen-khong` cross_source_curing annotation.**

## Performance

- **Duration:** 6 min
- **Started:** 2026-07-15T16:59:39Z
- **Completed:** 2026-07-15T17:05:20Z
- **Tasks:** 2 (Task 1 + Task 2)
- **Files modified:** 8 (4 source + 1 view helper + 1 view visualization + 1 schema + 1 corpus JSON)

## Accomplishments

- **INT-07 closed** — `NodeConcept::Offering` + `EdgeConcept::RecommendsOffering` added to all 6 ontology slice locations in `ontology.rs` (enum + label() match + ConceptLabel enum + as_str() match + node_concepts()/edge_concepts() static slices + extended locked test). New sibling test `v16_concepts_present_in_ontology_slices` asserts both concepts are present in the static slices AND label round-trips work.
- **INT-08 SC#2 literal closed** (Blocker 2 fix) — additive `payload: Option<serde_json::Value>` field on `SemanticNode` (Option B from 19-RESEARCH.md — generic payload via serde_json::Value, NOT a typed `RitualNodePayload` enum). `add_ritual_facts` populates the payload on the aggregate Ritual node with `serde_json::json!({offering_refs: [...], offerings: [...]})` derived from `snapshot.offering_refs`. New `pub fn nodes_mut(&mut self) -> &mut HashMap<String, SemanticNode>` accessor on `SemanticGraph` enables post-population (the additive companion to `nodes()`).
- **INT-09 closed** (Blocker 1 fix — supersedes Q2 Option C deferral) — `RitualMetadata { cross_source_curing: Option<Vec<CrossSourceCure>> }` + `CrossSourceCure { element_cure_for: String, source_id: SourceId, rationale_vi: String }` structs added to `rituals/schema.rs`; additive `metadata: Option<RitualMetadata>` field on `RitualEntry` with `#[serde(default, skip_serializing_if = "Option::is_none")]`. The `van-khan-tet-day-du` ritual entry (5 offerings — canonical Tết full-variant) annotated with one `cross_source_curing` entry whose `source_id = "huyen-khong"`.
- **Builder wiring complete** — new `add_offering_facts` helper on `DaySnapshotGraphBuilder` called from `new()` immediately AFTER `add_ritual_facts`. The helper iterates `snapshot.applicable_rituals`, calls `get_ritual_by_id`, inspects `entry.metadata.cross_source_curing`, builds `OfferingRef::new(...)` for each `entry.offerings[idx]`, emits `SemanticNode`s with `NodeConcept::Offering` + `SemanticEdge`s with `EdgeConcept::RecommendsOffering`, calls `track_provenance` ONCE for `SOURCE_VN_FOLK_RITUAL` (always) + ONE EXTRA call per `cross_source_curing` annotation (dual-source pattern). Edge dedup via `HashSet<(ritual_node_id, offering_node_id)>`. Rationale carried on the edge provenance note (Blocker 4 fix).
- **3 new tests added** — `ritual_metadata_and_cross_source_cure_serde_round_trip` (schema lock), `recommends_offering_edge_carries_dual_source_provenance` (Blocker 1+4+6 verification), `phase19_offering_wiring_endpoint_and_flying_star_components` (Blocker 6+7 verification).
- **View helper updates** — `cluster_for_node_id` in `views/helpers.rs` and `shape_hint_for_node` in `views/visualization.rs` updated to handle `NodeConcept::Offering` (compiler-enforced exhaustiveness forced these updates).

## Task Commits

Each task was committed atomically:

1. **Task 1: Add NodeConcept::Offering + EdgeConcept::RecommendsOffering across all 6 ontology slice locations + additive payload field on SemanticNode + extended test** — `3dce869` (feat)
2. **Task 2: Add RitualMetadata + CrossSourceCure schema + corpus annotation + dual-source add_offering_facts builder + focused dual-source test** — `18028b3` (feat)

## Files Created/Modified

- `crates/amlich-core/src/semantic_graph/ontology.rs` — 6 slice locations extended; new sibling test `v16_concepts_present_in_ontology_slices`
- `crates/amlich-core/src/semantic_graph/node.rs` — additive `payload: Option<serde_json::Value>` field + constructor init
- `crates/amlich-core/src/semantic_graph/graph.rs` — new `pub fn nodes_mut(&mut self) -> &mut HashMap<String, SemanticNode>` accessor (additive companion to `nodes()`)
- `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` — `add_ritual_facts` populates payload; new `add_offering_facts` helper with dual-source provenance + edge dedup; called from `new()` after `add_ritual_facts`; 2 new focused tests
- `crates/amlich-core/src/semantic_graph/views/helpers.rs` — `cluster_for_node_id` updated for Offering (compiler-enforced)
- `crates/amlich-core/src/semantic_graph/views/visualization.rs` — `shape_hint_for_node` updated for Offering (compiler-enforced)
- `crates/amlich-core/src/rituals/schema.rs` — `CrossSourceCure` + `RitualMetadata` structs + additive `metadata` field on `RitualEntry` + new test `ritual_metadata_and_cross_source_cure_serde_round_trip`
- `crates/amlich-core/data/rituals/tet-nguyen-dan.json` — `van-khan-tet-day-du` entry annotated with `metadata.cross_source_curing` (one `huyen-khong` cure for element "Kim")

## Decisions Made

- **Generic `serde_json::Value` payload over typed enum** — Per 19-RESEARCH.md Option B. Avoids the cost of a typed `RitualNodePayload` enum (Option C) and matches the v1.5 additive `Option<T>` discipline. The aggregate Ritual node carries `{offering_refs, offerings}` JSON; other concepts can use the same field for concept-specific structured data.
- **`CrossSourceCure.element_cure_for` stays free-form String (Vietnamese)** — The existing `Element` enum at `almanac/fengshui/types.rs` uses lowercase English ("metal", "wood"), while corpus annotations use Vietnamese ("Kim", "Mộc"). A typed enum would couple unrelated domains; Phase 19 keeps the annotation human-readable.
- **Post-population via new `nodes_mut()` accessor** — Rather than constructing the Ritual node with the payload at `SemanticNode::new()` time (which would require pre-computing `offering_refs` outside the builder), `add_ritual_facts` constructs the node first, then mutates via `nodes_mut()`. Additive accessor preserves the established builder pattern.
- **Two-call `track_provenance` reuses v1.5 multi-source append-pattern** — No parallel dedup helper. The `ProvenanceTracker::track()` method already appends, and the v1.5 Direction-node precedent (line 829 of `day_snapshot.rs` before this plan) establishes the pattern.
- **Rationale on the EDGE provenance note (Blocker 4 fix)** — Not just on the Offering node. The dual-source rationale `"lễ vật của nghi lễ, hỗ trợ chữa trị ngũ hành tương ứng"` is carried in the vn-folk-ritual entry's note via `rationale=...` substring, ensuring any consumer querying the edge can recover the rationale without a node lookup.
- **Annotated `van-khan-tet-day-du` (full-variant, 5 offerings)** — Chosen as the canonical Tết entry most likely to surface in any Tết-day `applicable_rituals`, maximizing test exposure. The other 3 Tết entries (don-gian, phat-giao, dan-gian) remain un-annotated — only the cross-source-cure-flagged entry triggers dual-source emission.

## Deviations from Plan

None - plan executed exactly as written. All 6 ontology slice locations updated, additive payload field added, schema structs added (BLOCKER 1 + 2 fixes), corpus annotated, builder wired (with rationale on edge — BLOCKER 4 fix), and both focused tests added.

## Issues Encountered

None.

## Next Phase Readiness

- INT-07 closed (Offering + RecommendsOffering concepts + SourceId discipline).
- INT-08 fully closed (DaySnapshot fields from 19-01 + SemanticNode::payload literal interpretation from 19-02).
- INT-09 closed (dual-source edge provenance via corpus-augmentation, NOT deferred — Q2 Option A implemented).
- Phase 19 plan 19-03 next: external-crate black-box tests for the full RecommendsOffering pipeline.
- Phase 19 plan 19-03's primary test surface: load `tet-nguyen-dan.json`, build a Tết 2026 `DaySnapshot`, assert the graph contains RecommendsOffering edges with dual-source provenance.

---

*Phase: 19-recommends-offering-semantic-graph-node*
*Completed: 2026-07-15*

## Self-Check: PASSED

- 19-02-SUMMARY.md exists
- Commits `3dce869` (Task 1) and `18028b3` (Task 2) found in git log
- All 5 key files exist on disk (ontology.rs, node.rs, day_snapshot.rs, schema.rs, tet-nguyen-dan.json)
- All 8 required content markers present:
  - `NodeConcept::Offering` in ontology.rs
  - `EdgeConcept::RecommendsOffering` in ontology.rs
  - `pub payload` field in node.rs
  - `RitualMetadata` struct
  - `CrossSourceCure` struct
  - `cross_source_curing` in tet-nguyen-dan.json corpus
  - `add_offering_facts` helper
  - `recommends_offering_edge_carries_dual_source_provenance` test
