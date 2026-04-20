# Amlich-core Legacy Replacement Playbook

> **Doc status:** Active — canonical playbook for Epic 8 (Migration, Compatibility, and Validation).
> **When to use:** Reference this when replacing a legacy `amlich-core` module or pipeline with a `semantic_graph`-backed path, especially when the public contract must remain stable during rollout.
> **Parent roadmap:** [`hybrid-semantic-graph-plan.md`](hybrid-semantic-graph-plan.md)
> **Primary example:** [`graph-backed-reasoning-migration-spec.md`](graph-backed-reasoning-migration-spec.md), [`../audit-semantic-graph-migration-surfaces.md`](../audit-semantic-graph-migration-surfaces.md), and follow-up beads `amlich-58g`, `amlich-zw1`, `amlich-po5`, `amlich-7wj`

## Purpose

Document the migration pattern that has already worked in this repo:

- build the richer `semantic_graph` substrate first
- preserve the old consumer contract while rewiring internals
- prove parity before switching production
- demote compatibility code deliberately instead of carrying it forever

This is a replacement playbook, not a new architecture proposal.

## Migration Principles

1. **Preserve contracts where possible**
   - Keep stable public entrypoints such as `build_initiation_opening_reasoning_bundle()` intact while the internal substrate changes underneath them.
2. **Replace substrate first, consumer contract second**
   - Migrate the data/model path into `semantic_graph` before asking downstream consumers to absorb a new DTO, export shape, or graph type.
3. **Prefer additive migration over big-bang rewrites**
   - Introduce new builders, evaluators, and projections alongside the old path. Remove the old path only after production has moved and residue is audited.
4. **Use parity as a gate, not as an aspiration**
   - New paths do not become production because they seem cleaner. They become production only after targeted parity evidence shows they preserve the existing behavioral contract.
5. **Separate "legacy" from "not yet productized"**
   - A new `semantic_graph` surface with no production consumer is not automatically legacy. It is legacy only if a newer or production path has already replaced its job.

## Lifecycle States

Use these states explicitly when classifying code during migration review:

### 1. Active production
- The surface is on the live production path or public contract.
- Example: the old `reasoning/` synthesis pipeline before `amlich-58g`.

### 2. Dual-run / parity phase
- Old and new paths both exist.
- The new path is exercised by tests, fixtures, or shadow entrypoints while parity gaps are closed.
- Example: `InitiationOpeningEvaluator` and `build_reasoning_input_graph()` before the production rewire.

### 3. Compatibility-only
- Production now uses the new substrate, but an old contract or adapter remains to preserve callers.
- Example: projections that still emit `InitiationOpeningDecision`, `InitiationOpeningDecisionExport`, or `ReasoningGraphExport` from semantic-graph-backed evaluation.

### 4. Deprecated
- The surface is still present but should not gain new consumers.
- New work should target the canonical path instead.

### 5. Removable
- No production use remains, parity and cleanup are complete, and the surface has no justified compatibility role.
- Removal can be scheduled as normal follow-up work.

### 6. Removed
- The code and exports are gone, and any upstream docs/tests now point to the canonical replacement only.

## Standard Migration Workflow

The default sequence for `amlich-core` migrations is:

1. **Build the new substrate/path**
   - Introduce the `semantic_graph` builders, evaluator, or shared helper that should become canonical.
   - Do not delete the old path yet.
2. **Add a compatibility projection**
   - Project the new path back into the current public contract so entrypoints and downstream tests can stay stable.
   - The `amlich-58g` reasoning migration followed this by projecting semantic-graph evaluation back into legacy decision/export types.
3. **Add parity coverage**
   - Lock the current behavior with targeted parity tests around buckets, semantics, supports/resistances, timing/direction refinements, and contract exports.
4. **Rewire the production entrypoint**
   - Move the public entrypoint to the new substrate only after parity is proven.
   - Keep the contract stable unless there is a separate, intentional contract migration.
5. **Demote the old path**
   - Reclassify the previous implementation as compatibility-only or deprecated.
   - Stop adding features there.
6. **Audit residual orphaned/dead surfaces**
   - Inspect warnings, exports, and helper layers for code that became unwired during migration.
   - The audit in [`../audit-semantic-graph-migration-surfaces.md`](../audit-semantic-graph-migration-surfaces.md) is the model for this step.
7. **Remove the old path when safe**
   - Delete dead adapters, duplicate helpers, and unused builders after there is no justified compatibility reason left to keep them.

## Decision Criteria

### Keep legacy code when
- it is still the production path
- it backs a stable public contract that callers still use
- parity for the replacement path is incomplete
- removal would force a broader contract migration than the current bead is meant to handle

### Demote to compatibility-only when
- production has switched to the new substrate
- the old shape is still useful as a compatibility DTO/export
- the code no longer owns real computation and should not receive new logic

### Remove when
- there are zero real consumers
- the surface no longer preserves a needed contract
- tests/build warnings show it is orphaned migration residue rather than a planned extension seam
- a repo audit can explain why deletion is safe

### Retain a public field/helper despite zero current consumers when
- it is part of a deliberate public API or extension seam
- removing it would shrink a documented contract without a separate deprecation decision
- it is a stable access point to canonical graph state rather than a one-off adapter

`amlich-po5` is the model here: dead private helpers were removed, but `ReasoningInputGraph` public fields were intentionally treated as an API boundary question rather than auto-deleted just because build warnings existed.

### Treat an unwired new surface as "not yet productized", not legacy, when
- it is aligned with the target architecture
- it has a plausible future consumer already identified
- it does not duplicate a newer canonical implementation
- the main gap is wiring, not conceptual obsolescence

This was the right classification for selectors, graph views, recommendation evidence builders, and the evaluator path before they were fully wired.

## Validation Gates

### Parity requirements
- Preserve recommendation bucket behavior.
- Preserve semantic classification behavior.
- Preserve materially important support/resistance/override/conflict evidence.
- Preserve personal refinements such as suggested hours and directions when those are already contract-visible.
- Preserve public export shape unless the bead explicitly includes a contract migration.

### Targeted tests
- Add the smallest parity-focused tests around the path being replaced.
- Prefer corpus-style fixtures that cover favorable, cautious, avoid, conflict-heavy, and personalized cases.
- Use the reasoning parity tests as the repo example of this gate.

### Full suite expectations
- Run the directly relevant package/crate suite for the migrated area.
- If the repo already has a stable broader suite for that surface, run it before closing the bead.
- Do not treat "targeted parity passed" as enough if the owning crate/package still regresses.

### Build warnings as migration evidence
- Treat new dead-code or unused-export warnings after a rewire as likely orphaned migration residue.
- Treat warnings as acceptable only when the surface is intentionally public, explicitly planned, or serving as a documented extension seam.
- Do not keep warning-producing compatibility code by default without writing down why it remains.

## Recommended Artifact Location

This playbook lives in `docs/almanac/legacy-replacement-playbook.md` because it is:

- broader than the reasoning migration spec
- narrower and more procedural than the main hybrid architecture roadmap
- directly tied to Epic 8, which the roadmap previously kept inline

Future semantic-graph and legacy-retirement work should link this doc from the relevant migration spec or audit, then keep bead-specific details in the bead or a focused implementation plan.

## Repo Example: Reasoning Migration

The recent reasoning migration established the pattern this playbook formalizes:

1. `semantic_graph` builders and `build_reasoning_input_graph()` assembled the richer substrate.
2. `InitiationOpeningEvaluator` evaluated the new graph while old public reasoning contracts remained intact.
3. parity tests in `crates/amlich-core/tests/reasoning_graph_parity.rs` locked expected behavior.
4. `amlich-58g` rewired the production entrypoint to the semantic-graph-backed path with compatibility projection.
5. `amlich-zw1`, `amlich-po5`, and `amlich-7wj` cleaned up dead projections, unused helpers, and duplicated logic revealed by the migration.

That is the default migration pattern to repeat unless a bead explicitly calls for a contract redesign.
