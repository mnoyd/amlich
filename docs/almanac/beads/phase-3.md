# Phase 3 Beads - Core Taboo Rules

## R-3001 - Normalize taboo tables for v1

## Meta
- ID: `R-3001`
- Type: research
- Status: TODO
- Priority: P0
- Phase: 3
- Depends on: `R-0002`, `R-2001`

## Context
Taboo rule families are high user-value but variant-heavy. We need one frozen v1 mapping set.

## Deliverables
- Finalized v1 definitions for:
  - Tam Nuong
  - Nguyet Ky
  - Sat Chu
  - Tho Tu
- Severity policy proposal (`hard`/`soft` defaults)

## Acceptance Criteria
1. Each rule family has explicit matching criteria.
2. Sat Chu/Tho Tu month-to-branch tables are frozen for v1.
3. Sources and caveats are recorded in research docs.

## Handoff Notes
- Next recommended bead: `I-3002`

---

## I-3002 - Add taboo categories to ruleset schema

## Meta
- ID: `I-3002`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 3
- Depends on: `R-3001`

## Context
Taboo rules must be represented as data families in ruleset packs.

## Deliverables
- Schema/data model updates for taboo rules
- Validation rules for taboo family data

## Acceptance Criteria
1. Ruleset can encode all v1 taboo families.
2. Invalid taboo entries fail validation clearly.
3. Existing data loads without regressions.

## Implementation Notes
- Candidate files:
  - `crates/amlich-core/src/almanac/data.rs`
  - `crates/amlich-core/src/almanac/types.rs`
  - `crates/amlich-core/data/almanac/*.json`

## Handoff Notes
- Next recommended bead: `I-3003`

---

## I-3003 - Implement taboo resolver with severity

## Meta
- ID: `I-3003`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 3
- Depends on: `I-3002`

## Context
Compute taboo hits for a day using lunar/day-branch context and configured rule tables.

## Deliverables
- Resolver emitting taboo hits with standardized ids and severity

## Acceptance Criteria
1. Resolver supports all v1 taboo families.
2. Each hit includes `rule_id`, display name, severity.
3. Behavior is deterministic by ruleset.

## Test Plan
- Unit tests for each taboo family.

## Handoff Notes
- Next recommended bead: `I-3004`

---

## I-3004 - Emit structured `taboos[]` with reason and evidence

## Meta
- ID: `I-3004`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 3
- Depends on: `I-3003`

## Context
Client layers need structured taboo output for display and scoring gates.

## Deliverables
- `taboos[]` field in day fortune/day info outputs
- Evidence metadata per taboo item

## Acceptance Criteria
1. Output includes all matched taboo entries.
2. Each entry contains reason and provenance metadata.
3. Additive schema change remains backward compatible.

## Handoff Notes
- Next recommended bead: `T-3005`

---

## T-3005 - Boundary tests around lunar transitions

## Meta
- ID: `T-3005`
- Type: test
- Status: TODO
- Priority: P0
- Phase: 3
- Depends on: `I-3004`

## Context
Taboo matching depends on accurate lunar date and branch values.

## Deliverables
- Test matrix around:
  - lunar month boundaries
  - timezone-sensitive conversion boundaries
  - representative months for Sat Chu/Tho Tu

## Acceptance Criteria
1. Tests cover all taboo families.
2. At least one boundary case per family is included.
3. Regressions are easy to diagnose from fixture names.

## Handoff Notes
- Next recommended bead: `D-3006`

---

## D-3006 - Taboo explanation conventions

## Meta
- ID: `D-3006`
- Type: docs
- Status: TODO
- Priority: P1
- Phase: 3
- Depends on: `T-3005`

## Context
Users need understandable explanations; contributors need consistent wording.

## Deliverables
- `docs/almanac/taboo-rules.md`
- Message conventions for each taboo family

## Acceptance Criteria
1. Explanation format is consistent and source-linked.
2. Severity semantics are documented (`hard` vs `soft`).
3. Links to decision log and ruleset scope docs are included.
