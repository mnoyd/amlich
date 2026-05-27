---
phase: 15-semantic-graph-wiring-dto-integration-e2e-validation
plan: 02
subsystem: semantic-graph
tags: [rust, ontology, semantic-graph, NodeConcept, EdgeConcept, ConceptLabel, GraphOntology, Ritual, FlyingStar]

# Dependency graph
requires:
  - phase: 15-01
    provides: DaySnapshot additive fields (flying_stars, applicable_rituals) DTO foundation
provides:
  - NodeConcept::Ritual and NodeConcept::FlyingStar variants in semantic ontology
  - EdgeConcept::PrescribedFor, OccupiesPalace, CarriesElement variants
  - ConceptLabel mirrors + as_str() snake_case strings for all five new variants
  - GraphOntology::node_concepts() and edge_concepts() static slices updated
  - Completeness test v15_concepts_present_in_ontology_slices guards silent-bug risk
affects:
  - phase 15-03 (uses Ritual/FlyingStar NodeConcept in graph builders)
  - phase 15-04 (E2E validation verifies the full semantic graph including new concepts)

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Six-location atomic update pattern for ontology enums: enum variant, label() arm, ConceptLabel enum, as_str() arm, GraphOntology node_concepts() slice, GraphOntology edge_concepts() slice"
    - "Completeness test pattern: explicit .contains() assertions on static slices to guard hand-maintained slices not enforced by compiler"

key-files:
  created: []
  modified:
    - crates/amlich-core/src/semantic_graph/ontology.rs
    - crates/amlich-core/src/semantic_graph/views/helpers.rs
    - crates/amlich-core/src/semantic_graph/views/visualization.rs

key-decisions:
  - "Ritual and FlyingStar assigned to day-core cluster in helpers.rs and box shape_hint in visualization.rs — both are day-level content nodes analogous to Activity/Recommendation, not matrix or interaction nodes"
  - "TDD RED for Task 2 was structurally blocked by 15-01 RED test in lib.rs (DaySnapshot serde round-trip) which prevents test binary compilation; ontology test verified in isolation via --lib filter; conceptual RED confirmed by running test before GREEN slice additions"

patterns-established:
  - "Silent-bug guard: any hand-maintained static slice (non-compiler-checked) requires an explicit completeness test using .contains()"
  - "New NodeConcept/EdgeConcept variants require exhaustive match updates in views/helpers.rs (cluster_for_node_id) and views/visualization.rs (shape_hint_for_node)"

requirements-completed: [INT-03]

# Metrics
duration: 5min
completed: 2026-05-27
---

# Phase 15 Plan 02: Ontology Extension Summary

**Five new v1.5 semantic concepts (Ritual, FlyingStar, PrescribedFor, OccupiesPalace, CarriesElement) added across all six ontology locations with compiler-enforced exhaustive matches and a completeness test guarding the hand-maintained GraphOntology static slices**

## Performance

- **Duration:** 5 min
- **Started:** 2026-05-27T19:03:10Z
- **Completed:** 2026-05-27T19:08:01Z
- **Tasks:** 2
- **Files modified:** 3

## Accomplishments
- Extended NodeConcept enum with Ritual and FlyingStar; EdgeConcept with PrescribedFor, OccupiesPalace, CarriesElement across all six locations in ontology.rs (enum variants, label() arms, ConceptLabel enum + as_str() arms)
- Added Ritual and FlyingStar to GraphOntology::node_concepts() static slice; PrescribedFor, OccupiesPalace, CarriesElement to edge_concepts() static slice
- Added v15_concepts_present_in_ontology_slices completeness test — explicitly guards against the "silent bug" of missing hand-maintained slice entries; also validates label round-trip strings

## Task Commits

Each task was committed atomically:

1. **Task 1: Extend NodeConcept + EdgeConcept + ConceptLabel across all six ontology locations** - `fc2876e` (feat)
2. **Task 2: Add GraphOntology static-slice entries + completeness test** - `46ebf55` (feat)

_Note: TDD tasks — Task 1 was compiler-verified (exhaustive matches); Task 2 used test-driven RED/GREEN (completeness test run before slice entries added)_

## Files Created/Modified
- `crates/amlich-core/src/semantic_graph/ontology.rs` - All six enum/match/slice locations updated; completeness test added
- `crates/amlich-core/src/semantic_graph/views/helpers.rs` - cluster_for_node_id: Ritual/FlyingStar added to day-core arm
- `crates/amlich-core/src/semantic_graph/views/visualization.rs` - shape_hint_for_node: Ritual/FlyingStar added to box arm

## Decisions Made
- Ritual and FlyingStar assigned to "day-core" cluster and "box" shape hint — consistent with Activity/Recommendation content-node categorization; they represent day-contextual content (rituals that apply to a day, flying stars active that day), not interaction matrices or canchi signals
- Six-location atomic update confirmed as the correct pattern for extending closed enums in ontology.rs; partial updates cause compile errors (enforced)

## Deviations from Plan

### Auto-fixed Issues

**1. [Rule 1 - Bug] Fixed non-exhaustive match arms in helpers.rs and visualization.rs**
- **Found during:** Task 1 (build verification after ontology.rs edits)
- **Issue:** Adding Ritual and FlyingStar to NodeConcept enum caused non-exhaustive pattern errors in `cluster_for_node_id` (helpers.rs line 6) and `shape_hint_for_node` (visualization.rs line 77)
- **Fix:** Added `NodeConcept::Ritual | NodeConcept::FlyingStar => "day-core".to_string()` arm in helpers.rs; added same to box branch in visualization.rs shape_hint function
- **Files modified:** crates/amlich-core/src/semantic_graph/views/helpers.rs, crates/amlich-core/src/semantic_graph/views/visualization.rs
- **Verification:** `cargo build -p amlich-core` passes with no errors
- **Committed in:** fc2876e (Task 1 commit)

---

**Total deviations:** 1 auto-fixed (Rule 1 - non-exhaustive match arms in dependent view files)
**Impact on plan:** Necessary correctness fix; plan's "Edit all six locations atomically" scope did not account for external match sites — the compiler enforced the fix. No scope creep.

## Issues Encountered
- 15-01 RED test (DaySnapshot serde round-trip failing test in lib.rs) blocked `cargo test` compilation for the full lib test binary. Workaround: used `cargo test --lib 'semantic_graph::ontology'` filter to run the completeness test in isolation. The pre-existing 15-01 RED state is intentional and out of scope for 15-02.

## User Setup Required
None - no external service configuration required.

## Next Phase Readiness
- Semantic ontology now includes both v1.5 pillars (Ritual, FlyingStar) as first-class NodeConcept variants
- PrescribedFor, OccupiesPalace, CarriesElement edge concepts ready for use in graph builder wiring (Phase 15-03)
- Completeness test in place as regression guard for future ontology extensions
- No blockers for 15-03 (graph builder wiring for rituals + flying stars)

---
*Phase: 15-semantic-graph-wiring-dto-integration-e2e-validation*
*Completed: 2026-05-27*
