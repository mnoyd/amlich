---
phase: 20-foundation-schema-lock-source-ids-adrs-ontology
plan: 03
subsystem: ontology
tags: [semantic-graph, ontology, iching, hexagram, rust, enums, exhaustive-match]

# Dependency graph
requires:
  - phase: 19-foundation (INT-07 Offering/RecommendsOffering precedent)
    provides: 6-slice ontology extension discipline + views exhaustiveness precedent
provides:
  - "NodeConcept::Hexagram variant across all 6 ontology slices (compiler-enforced exhaustive)"
  - "EdgeConcept::LocatedAt + EdgeConcept::Transforms edge variants across all edge slices"
  - "ConceptLabel::Hexagram/LocatedAt/Transforms with snake_case as_str labels (hexagram/located_at/transforms)"
  - "ActionId::IChing variant (Tier-0 reasoning action id, serializes to i_ching)"
  - "ReasoningEvidenceSourceFamily::IChing variant (distinct Tier-0 evidence family, serializes to i_ching)"
  - "Updated cluster_for_node_id + shape_hint_for_node exhaustive arms for Hexagram"
  - "v17_concepts_present_in_ontology_slices test (presence + label round-trips)"
affects: [24-iching-evaluator-semantic-graph-wiring-dto, 21-iching-corpus-loader]

# Tech tracking
tech-stack:
  added: []
  patterns:
    - "6-slice ontology extension discipline (enum + label() + ConceptLabel + as_str() + node_concepts() + edge_concepts()) preserved from Phase 19 INT-07"
    - "Additive enum extension with zero #[non_exhaustive] escape (FND-12 locks compiler-enforced exhaustiveness)"
    - "Additive-safe variant addition when no exhaustive match blocks exist (ActionId, ReasoningEvidenceSourceFamily)"

key-files:
  created: []
  modified:
    - "crates/amlich-core/src/semantic_graph/ontology.rs (Hexagram node + LocatedAt/Transforms edges across all 6 slices + v1.7 test)"
    - "crates/amlich-core/src/semantic_graph/views/helpers.rs (cluster_for_node_id arm extended for Hexagram)"
    - "crates/amlich-core/src/semantic_graph/views/visualization.rs (shape_hint_for_node arm extended for Hexagram)"
    - "crates/amlich-core/src/reasoning/types.rs (ActionId::IChing + ReasoningEvidenceSourceFamily::IChing)"

key-decisions:
  - "Hexagram joins the Ritual/FlyingStar/Offering cluster family in views (day-core cluster, box shape hint) — mirrors the corpus-node placement established in Phase 19 for Offering."
  - "IChing added as a distinct Tier-0 ReasoningEvidenceSourceFamily variant (NOT a reuse of AlmanacRule) per v1.7 roadmap requirement — IChing deserves its own evidence-family namespace."
  - "Additive-safe variant addition on ActionId + ReasoningEvidenceSourceFamily required zero call-site churn — rg confirmed no exhaustive match blocks exist on either enum (only constructed, never matched)."

patterns-established:
  - "6-slice extension template now covers 3 successive v1.x bumps (v1.5 Ritual/FlyingStar, v1.6 Offering, v1.7 Hexagram) — discipline is repeatable and compiler-enforced."

requirements-completed: [FND-12]

# Metrics
duration: 5min
completed: 2026-07-15
---

# Phase 20 Plan 03: v1.7 Ontology (Hexagram + LocatedAt/Transforms + IChing Variants) Summary

**Extended the 6-slice semantic-graph ontology with NodeConcept::Hexagram + EdgeConcept::LocatedAt/Transforms and added ActionId::IChing + ReasoningEvidenceSourceFamily::IChing Tier-0 variants — all compiler-enforced exhaustive with no #[non_exhaustive] escape.**

## Performance

- **Duration:** 5 min
- **Started:** 2026-07-15T19:40:32Z
- **Completed:** 2026-07-15T19:45:50Z
- **Tasks:** 2 (Task 1 was TDD with RED → GREEN)
- **Files modified:** 4

## Accomplishments

- All 6 ontology slices extended for `NodeConcept::Hexagram` (enum, `label()` match, `ConceptLabel` enum, `as_str()` match, `node_concepts()` slice).
- Symmetric `EdgeConcept` slices extended for `EdgeConcept::LocatedAt` + `EdgeConcept::Transforms` (enum + `label()` match + `ConceptLabel` + `as_str()` + `edge_concepts()` slice).
- Two exhaustive-match views updated for `Hexagram` arm extension (`cluster_for_node_id` → `day-core`, `shape_hint_for_node` → `box`).
- v1.7 ontology test asserts Hexagram/LocatedAt/Transforms presence + label round-trips (`hexagram`/`located_at`/`transforms`).
- `ActionId::IChing` + `ReasoningEvidenceSourceFamily::IChing` added — both verified to serialize to `"i_ching"` via round-trip test.
- Zero `#[non_exhaustive]` escapes introduced (FND-12 enforced).

## Task Commits

Each task was committed atomically (TDD plan: Task 1 split into RED → GREEN):

1. **Task 1 RED: v1.7 failing ontology test** — `7f4c562` (test)
2. **Task 1 GREEN: Hexagram/LocatedAt/Transforms across 6 slices + views arm fixes** — `668dcbc` (feat)
3. **Task 2: ActionId::IChing + ReasoningEvidenceSourceFamily::IChing variants** — `06cb209` (feat)

**Plan metadata:** `lmn012o` (docs: complete plan) — to be added in final commit.

## Files Created/Modified

- `crates/amlich-core/src/semantic_graph/ontology.rs` — `NodeConcept::Hexagram` + `EdgeConcept::LocatedAt` + `EdgeConcept::Transforms` across all 6 slices (enum, `label()`, `ConceptLabel`, `as_str()`, `node_concepts()`, `edge_concepts()`); plus v1.7 test inside `#[cfg(test)] mod tests`.
- `crates/amlich-core/src/semantic_graph/views/helpers.rs` — `cluster_for_node_id` exhaustive arm extended: `Ritual | FlyingStar | Offering | Hexagram => "day-core"`.
- `crates/amlich-core/src/semantic_graph/views/visualization.rs` — `shape_hint_for_node` exhaustive arm extended: `Ritual | FlyingStar | Offering | Hexagram => Some("box")`.
- `crates/amlich-core/src/reasoning/types.rs` — `ActionId::IChing` (line 7) + `ReasoningEvidenceSourceFamily::IChing` (line 143); both enums already derive `#[serde(rename_all = "snake_case")]` so they serialize to `"i_ching"`.

## Decisions Made

- **Hexagram joins the corpus-node cluster family** — added to the existing `Ritual | FlyingStar | Offering` arm in both `cluster_for_node_id` (→ `day-core`) and `shape_hint_for_node` (→ `box`). Hexagram nodes will sit in the `day-core` cluster alongside other classical-corpus nodes; `box` shape matches the other noun-style corpus nodes. This mirrors the Phase 19 precedent for Offering.
- **`IChing` is its own `ReasoningEvidenceSourceFamily` variant** — explicitly NOT a reuse of `AlmanacRule`. IChing is a distinct Tier-0 evidence family per the v1.7 roadmap (separate source-ids `kinh-dich` / `mai-hoa-dich-so`), so it deserves its own family namespace. `AlmanacRule` remains reserved for KHCBPPT-derived fengshui rules.
- **Additive-safe enum extensions on `ActionId` + `ReasoningEvidenceSourceFamily`** — research-confirmed (and execution-verified) that no exhaustive match blocks exist on either enum anywhere in `src/`. Both enums are only constructed, never matched, so variant addition required zero call-site churn.

## Deviations from Plan

None - plan executed exactly as written. The TDD RED-GREEN cycle for Task 1 produced exactly the 3 commits the plan's structure implies (test → feat → feat). All exhaustive-match sites flagged by the compiler matched the 2 sites the plan predicted (`views/helpers.rs` + `views/visualization.rs`); no additional match sites surfaced.

## Issues Encountered

- **Pre-existing failing test surfaced during final full-suite verification** — `iching::schema::tests::composition_table_is_bijective` fails with `duplicate pair at King Wen #2: (1, 1)`. This traces to commit `99efa74 test(20-02): add failing bijectivity test for iching composition table` (Plan 20-02's TDD RED phase, intentionally failing and awaiting its GREEN implementation). NOT caused by Plan 20-03 — all Plan 20-03 changes compile cleanly and pass their own tests. Logged to `deferred-items.md` per SCOPE BOUNDARY rules.

## User Setup Required

None - no external service configuration required.

## Next Phase Readiness

- The v1.7 type-level ontology scaffolding is complete: `NodeConcept::Hexagram` + `EdgeConcept::LocatedAt` + `EdgeConcept::Transforms` are now reserved in the semantic-graph vocabulary, ready for Phase 24's `add_iching_facts()` builder to populate (Hexagram nodes wired via LocatedAt/Transforms edges).
- The `ActionId::IChing` + `ReasoningEvidenceSourceFamily::IChing` variants mark IChing as a distinct Tier-0 evidence family + action id — Phase 22's Mai Hoa casting + Phase 24's IChingEvaluator will construct these directly.
- Combined with Plans 01 (source-id registration) and 02 (IChing schema), Plan 03 completes the v1.7 foundation type-scaffolding that Phases 21-25 build upon.
- **Blocker on phase completion:** Plan 20-02's failing bijectivity test must be resolved during Plan 20-02's GREEN execution before the Phase 20 suite can be considered fully green.

---
*Phase: 20-foundation-schema-lock-source-ids-adrs-ontology*
*Completed: 2026-07-15*

## Self-Check: PASSED

- All 4 modified source files exist on disk.
- All 3 task commits present in git history (`7f4c562`, `668dcbc`, `06cb209`).
- SUMMARY.md + deferred-items.md created at expected paths.
- v1.7 ontology test passes; crate builds clean; no `#[non_exhaustive]` introduced.

