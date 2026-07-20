---
phase: 15-semantic-graph-wiring-dto-integration-e2e-validation
plan: 03
subsystem: semantic-graph
tags: [rust, semantic-graph, flying-stars, rituals, provenance, ontology, day-snapshot]

# Dependency graph
requires:
  - phase: 15-01
    provides: FlyingStarsSummary DTO + applicable_rituals field on DaySnapshot
  - phase: 15-02
    provides: NodeConcept::FlyingStar, NodeConcept::Ritual, EdgeConcept::OccupiesPalace, EdgeConcept::PrescribedFor ontology entries
provides:
  - FlyingStar semantic node with source_id huyen-khong in DaySnapshotGraphBuilder
  - Ritual semantic node with source_id vn-folk-ritual in DaySnapshotGraphBuilder
  - Direction node carrying dual provenance (khcbppt-family + huyen-khong, len==2)
  - PrescribedFor edge (ritual -> day_root) exercising new edge concept
  - OccupiesPalace edge (flying_star -> direction) exercising new edge concept
affects: [15-04, INT-04]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "Multi-source Direction node: append second ProvenanceEntry to same node at construction time (NOT a second add_node which would overwrite)"
    - "Source-disjoint pillar nodes: FlyingStar uses only SOURCE_HUYEN_KHONG, Ritual uses only SOURCE_VN_FOLK_RITUAL"
    - "Guard pattern: let Some(x) = field else { return; } for optional DaySnapshot fields"

key-files:
  created: []
  modified:
    - crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs

key-decisions:
  - "Dual provenance appended to SAME Direction node at construction — second add_node would silently overwrite via HashMap insert"
  - "OccupiesPalace edge wired from FlyingStar node to Direction node id (always present in this builder), no conditional guard needed"
  - "PrescribedFor edge direction is ritual -> day_root (ritual prescribed FOR the day), not day_root -> ritual"
  - "Test uses calculate_day_snapshot(17, 2, 2026) (Tet 2026) to guarantee both flying_stars and applicable_rituals are populated"

patterns-established:
  - "Multi-source provenance: use .with_provenance() chaining to accumulate entries on the same node"
  - "Source-ID discipline: SOURCE_* constants only in production code; bare string literals permitted in test assertions only"

requirements-completed: [INT-04]

# Metrics
duration: 2min
completed: 2026-05-28
---

# Phase 15 Plan 03: Semantic Graph Wiring (FlyingStar + Ritual + dual-provenance Direction) Summary

**FlyingStar node (huyen-khong), Ritual node (vn-folk-ritual), and dual-provenance Direction node (khcbppt + huyen-khong) wired into DaySnapshotGraphBuilder with OccupiesPalace and PrescribedFor edges exercising all INT-04 ontology requirements**

## Performance

- **Duration:** 2 min
- **Started:** 2026-05-27T19:11:06Z
- **Completed:** 2026-05-27T19:13:55Z
- **Tasks:** 2
- **Files modified:** 1

## Accomplishments

- Direction node now carries 2 provenance entries: existing khcbppt-family entry plus new `ProvenanceEntry::almanac_rule(SOURCE_HUYEN_KHONG, "phi_tinh.direction_overlap")` — single node, no HashMap overwrite risk
- `add_flying_star_facts` creates FlyingStar node from `snapshot.flying_stars` with source_id huyen-khong only, connected via Composes (to day_root) and OccupiesPalace (to Direction node)
- `add_ritual_facts` creates Ritual node from `snapshot.applicable_rituals` with source_id vn-folk-ritual only, connected via PrescribedFor to day_root; no-ops on None/empty
- `v15_pillar_nodes_carry_disjoint_source_ids_and_direction_is_multi_source` unit test asserts all INT-04 invariants using Tet 2026 snapshot
- `source_id_guard` CI test remains green — zero bare string literals in production code

## Task Commits

Each task was committed atomically:

1. **Task 1: Append huyen-khong provenance to shared Direction node** - `200b9af` (feat)
2. **Task 2: Add FlyingStar + Ritual builder methods + register + test** - `4173368` (feat)

## Files Created/Modified

- `crates/amlich-core/src/semantic_graph/builders/day_snapshot.rs` — Added SOURCE_HUYEN_KHONG/SOURCE_VN_FOLK_RITUAL imports; dual-provenance in add_travel_direction_fact; add_flying_star_facts; add_ritual_facts; two new unit tests

## Decisions Made

- **Dual provenance via .with_provenance() chaining**: The plan explicitly warned that a second `add_node` with the same id would overwrite the Direction node (HashMap semantics). The second `ProvenanceEntry` is appended to the same node builder chain before the single `add_node` call.
- **OccupiesPalace edge always fires**: The Direction node is always created in `add_travel_direction_fact` (which runs before `add_flying_star_facts`), so no conditional guard is needed for the OccupiesPalace edge.
- **PrescribedFor direction is ritual -> day_root**: Matches plan specification — the ritual is prescribed FOR the day, not the day owning the ritual.
- **Test date Tet 2026 (17 Feb 2026)**: Chosen per plan specification to guarantee `flying_stars: Some(_)` and `applicable_rituals: Some(non-empty)` are both populated.

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- INT-04 requirements fully satisfied: source-disjoint pillar nodes, multi-source Direction node, new edge concepts (PrescribedFor, OccupiesPalace) exercised
- `direction_merge.rs` untouched throughout — PITFALLS CRIT-3 constraint honored
- Ready for Phase 15 Plan 04 (E2E validation / integration tests)

---
*Phase: 15-semantic-graph-wiring-dto-integration-e2e-validation*
*Completed: 2026-05-28*
