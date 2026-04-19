# Canonical Contract Evolution Guidelines

## Status

Developer policy — amlich-606.

## Goal

Define how canonical reasoning and personalization contracts may evolve without fragmenting shapes, meanings, or migration expectations across surfaces.

---

## Scope

This policy applies to consumer-facing reasoning and personalization contracts, especially:

- `decision_export`
- `graph`
- matrix reports such as `personal-day-matrix`
- any canonical DTO or JSON surface derived from those exports

It complements, but does not replace:

- `docs/almanac/reasoning-graph-schema.md` for field semantics
- `docs/almanac/contract-usage-examples.md` for consumer mapping and acceptance examples
- `docs/almanac/known-differences.md` and `docs/almanac/ruleset-loader.md` for ruleset/version behavior

When those docs disagree in practice, prefer this document for contract-shape evolution rules and the schema doc for field semantics.

## Contract invariants

These rules are mandatory unless a documented migration says otherwise.

1. **One canonical shape per concept.**
   - Do not introduce parallel exports that mean the same thing with different field names.
   - Do not keep legacy aliases after active migration is complete.

2. **Canonical consumer surfaces read canonical exports.**
   - UI and app surfaces should read `decision_export`, `graph`, and matrix sections directly.
   - Internal structs may exist, but consumer contracts should not drift back toward ad-hoc flattened views.

3. **Field meaning must stay stable.**
   - Reusing an existing field name for a narrower, broader, or different meaning is a breaking change even if the type stays the same.
   - Changing severity or bucket semantics is a semantic contract change, not a mere wording change.

4. **Optionality is part of the contract.**
   - If a section is optional because input completeness gates it, that optionality must remain explicit.
   - Do not make previously optional sections required without a migration plan.

5. **Headline, rationale, caution, and drill-down layers must stay separable.**
   - New fields should fit the explanation hierarchy rather than collapsing multiple layers into one convenience alias.
   - Keep deep evidence and graph internals distinct from top-level verdict fields.

## Allowed additive changes

These changes are usually allowed without introducing a new contract family, as long as existing consumers remain correct.

### Safe additions

- adding a new optional field with a clear default or omission behavior
- adding a new optional matrix section gated by input completeness
- adding new node tags, edge tags, or evidence metadata that existing consumers can ignore safely
- adding new worked examples, docs, or stronger regression coverage
- adding richer internal production structs when canonical export shape remains unchanged

### Conditions for safe additions

All additive changes must satisfy all of the following:

1. Existing consumers can ignore the new field safely.
2. Existing field meaning does not change.
3. The schema doc and usage doc are updated if consumers should notice the addition.
4. Regression tests prove that representative existing outputs still deserialize and behave as expected.

## Breaking or semantic changes

Treat the following as contract-breaking or semantic changes requiring explicit migration handling.

### Shape-breaking changes

- renaming a published canonical field
- removing a published canonical field that active consumers still rely on
- changing field type (`string` to object, scalar to array, optional to required)
- moving a field to a new nesting location without keeping a documented migration path

### Meaning-breaking changes

- changing what `recommendation_bucket`, `confidence`, `semantic`, or axis scores mean
- changing whether a note belongs to rationale vs caution semantics
- changing ordering expectations when consumers depend on ranked or strongest items
- changing matrix-section presence rules for the same input completeness tier

### Smell-based prohibition

If a proposed change needs phrases like these, stop and redesign:

- “just add another alias”
- “keep both shapes for now” without a removal date
- “the old field is close enough”
- “UI can reconstruct it”
- “consumers can infer the difference”

These usually indicate contract fragmentation rather than evolution.

## Migration rules

When a breaking or semantic change is unavoidable, follow this order.

1. **State the reason clearly.**
   - Explain why the old contract is insufficient.
   - Name which consumers are affected.

2. **Prefer additive transition before removal.**
   - Introduce the new canonical field or section.
   - Migrate active consumers to it.
   - Add tests proving those consumers now use the canonical field.

3. **Remove duplicate legacy adapters once migration is complete.**
   - Do not leave compatibility fields indefinitely.
   - Update docs to stop advertising removed fields.

4. **Document semantic changes as policy changes, not implementation details.**
   - If meaning changes, write it down in docs and decision log / migration notes.
   - If ruleset behavior changes, follow version-bump rules from `known-differences.md` and `ruleset-loader.md`.

5. **Never ship silent drift.**
   - If the same input would now produce different semantics under the same published contract, that requires explicit documentation and usually a versioned path.

## Required tests and docs

Any reasoning/personality contract evolution should update the narrowest relevant combination of these gates.

### Tests

At minimum, consider:

- schema/shape locks for required keys
- parity or representative-case tests for headline/rationale/caution visibility
- API/Tauri or consumer-boundary tests where DTO shape matters
- null-safety tests for optional matrix sections

### Documentation

Update whichever docs are relevant:

- `docs/almanac/reasoning-graph-schema.md` for field semantics or new keys
- `docs/almanac/contract-usage-examples.md` for consumer mapping and acceptance examples
- this document for new evolution policy or migration pattern
- migration/audit docs when legacy fields are removed

### Review checklist

Before considering a contract change complete, verify:

- [ ] no duplicate canonical and legacy shapes remain without a removal plan
- [ ] field meaning is unchanged or explicitly documented as changed
- [ ] optional vs required behavior is documented
- [ ] representative regression coverage exists for the affected layer
- [ ] consumer docs point to the canonical field path

## Worked examples

### Example 1: Adding a new reasoning node type

**Allowed approach:**
- add the new node kind and export it through `graph.nodes`
- add tags/evidence semantics if needed
- keep existing `decision_export` fields stable unless a new top-level summary is truly required
- add tests proving existing consumers still work

**Avoid:**
- adding a second top-level “highlights_v2” array just because one surface wants a shortcut

### Example 2: Refining headline semantics

If the project wants a new headline classification beyond current `recommendation_bucket` semantics:

**Preferred path:**
- add a clearly named new field only if the old one cannot express the distinction
- document exact meaning and consumer expectations
- migrate current surfaces deliberately
- remove any deprecated aliases after migration

**Not allowed:**
- silently changing what `cautious` or `favorable` means while keeping the old field name and docs

### Example 3: Adding a new matrix section

If a future personalization feature introduces a new matrix section:

**Allowed path:**
- add it as an optional section
- define its gating conditions and `unavailable_sections` behavior
- document where it belongs in the explanation hierarchy
- add a regression showing consumers handle omission safely

**Not allowed:**
- assuming all full-profile consumers automatically have the new section without documenting gating and fallback behavior

### Example 4: Removing a migrated legacy field

Once all active consumers use the canonical field:

**Required path:**
- remove the legacy alias from DTOs and presentation code
- tighten tests so only canonical fields are exercised
- update docs so the legacy field stops appearing in examples

This is a cleanup step, not a compatibility regression.
