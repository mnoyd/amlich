# Phase 2 Beads - Day Hoang Dao and Hac Dao

## R-2001 - Finalize day deity mapping table for v1

## Meta
- ID: `R-2001`
- Type: research
- Status: TODO
- Priority: P0
- Phase: 2
- Depends on: `R-0002`, `I-1004`

## Context
We need a canonical day-level hoang dao/hac dao mapping for `vn_baseline_v1` before implementing the resolver.

## Deliverables
- Chosen v1 day-deity mapping table and naming set
- Variant notes (if alternate mappings exist)
- At least 12 worked examples across lunar month branch groups

## Acceptance Criteria
1. Table is explicit and complete for all 12 offsets.
2. Day deity names and hoang/hac classification are frozen for v1.
3. Source metadata and variant caveats are documented.

## Research Notes
- Questions to answer:
  - Exact start-position mapping by lunar month branch group
  - Naming variants (e.g. transliteration differences)
  - Whether any app-facing aliases are needed
- Candidate sources:
  - Existing project research notes
  - Chosen v1 canonical source(s)

## Handoff Notes
- Next recommended bead: `I-2002`

---

## I-2002 - Implement day deity resolver

## Meta
- ID: `I-2002`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 2
- Depends on: `R-2001`

## Context
Implement day-level "than tri nhat" calculation from lunar month branch + day branch.

## Deliverables
- Resolver function returning day deity and hoang/hac classification
- Ruleset-backed table lookup (not hard-coded one-off values)

## Acceptance Criteria
1. Resolver covers all 12 day branches across all month branch groups.
2. Output includes deity name and `HOANG_DAO | HAC_DAO` label (or equivalent typed enum).
3. Resolver is ruleset-driven and testable.

## Implementation Notes
- Candidate files:
  - `crates/amlich-core/src/almanac/*` (new day-hoang-dao module or within `calc.rs`)
  - `crates/amlich-core/src/almanac/types.rs`
  - ruleset data files under `crates/amlich-core/data/almanac/`

## Test Plan
- Unit tests covering each month-branch group and selected day branches.

## Handoff Notes
- Next recommended bead: `I-2003`

---

## I-2003 - Add day-level output fields

## Meta
- ID: `I-2003`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 2
- Depends on: `I-2002`

## Context
Expose day hoang dao/hac dao data in `DayFortune` and parent day info outputs.

## Deliverables
- New typed fields in almanac output model
- Serialization support in API-facing structs (if applicable in same slice)

## Acceptance Criteria
1. Output includes day deity name and classification.
2. Output schema is backward compatible (new fields additive).
3. Default day-info path returns populated values when ruleset supports it.

## Test Plan
- Serialization tests and day-info integration tests.

## Handoff Notes
- Next recommended bead: `I-2004`

---

## I-2004 - Integrate evidence metadata for day hoang dao/hac dao

## Meta
- ID: `I-2004`
- Type: implementation
- Status: TODO
- Priority: P1
- Phase: 2
- Depends on: `I-2003`

## Context
All rule-derived outputs should expose provenance metadata for trust and debugging.

## Deliverables
- Evidence metadata wiring (`source_id`, `method`, `profile`, ruleset version if available)

## Acceptance Criteria
1. Day hoang dao/hac dao fields include evidence metadata.
2. Metadata matches selected ruleset and source family.

## Test Plan
- Unit tests asserting evidence is present and non-empty.

## Handoff Notes
- Next recommended bead: `T-2005`

---

## T-2005 - Golden date tests by lunar month and day branch

## Meta
- ID: `T-2005`
- Type: test
- Status: TODO
- Priority: P0
- Phase: 2
- Depends on: `I-2004`

## Context
Day-level hoang dao/hac dao is table-sensitive and easy to regress silently.

## Deliverables
- Golden fixtures covering representative dates across lunar months
- Tests validating deity + classification outputs

## Acceptance Criteria
1. Fixtures cover all month-branch groups.
2. Tests fail on mapping drift.
3. Ruleset id/version are pinned in fixture metadata.

## Handoff Notes
- Next recommended bead: `D-2006`

---

## D-2006 - Formula and variant documentation

## Meta
- ID: `D-2006`
- Type: docs
- Status: TODO
- Priority: P1
- Phase: 2
- Depends on: `T-2005`

## Context
Contributors and users need to understand why results may differ across rulesets/apps.

## Deliverables
- `docs/almanac/day-hoangdao-hacdao.md`
- Source notes + variant caveats + examples

## Acceptance Criteria
1. Documents chosen v1 mapping and source lineage.
2. Documents known variant behavior and future extension path.
3. Links to relevant decision log entries.
