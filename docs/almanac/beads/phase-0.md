# Phase 0 Beads - Scope Freeze and Contracts

## R-0001 - Glossary normalization

## Meta
- ID: `R-0001`
- Type: research
- Status: TODO
- Priority: P0
- Phase: 0
- Depends on: none

## Context
We need a single glossary so team members and agents do not mix terms across different traditions.

## Deliverables
- `docs/almanac/glossary.md` with normalized Vietnamese + internal English token names.
- Mapping table for aliases/synonyms.

## Acceptance Criteria
1. Core terms are defined (can-chi, tiet khi, truc, taboos, person rules, scoring terms).
2. Each term has one canonical internal token.
3. Ambiguous terms are marked with notes and non-goals.

## Test Plan
- Review by at least one implementer and one reviewer.

---

## R-0002 - v1 ruleset scope freeze

## Meta
- ID: `R-0002`
- Type: research
- Status: TODO
- Priority: P0
- Phase: 0
- Depends on: `R-0001`

## Context
Avoid scope creep by freezing what `vn_baseline_v1` includes and excludes.

## Deliverables
- Confirm and finalize `docs/almanac/ruleset-v1-scope.md`.
- Add explicit deferred list and rationale.

## Acceptance Criteria
1. Included families are final for v1.
2. Deferred families are explicit.
3. Any unresolved family is moved to backlog with owner.

## Test Plan
- Scope review against `docs/almanac/master-plan.md`.

---

## I-0003 - RuleSetDescriptor schema

## Meta
- ID: `I-0003`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 0
- Depends on: `R-0002`

## Context
Core needs a first-class ruleset descriptor to support versioned data packs.

## Deliverables
- Rust type definitions for ruleset descriptor.
- JSON schema notes for ruleset metadata.

## Acceptance Criteria
1. Descriptor includes id, version, region, defaults, source notes.
2. Loader can parse descriptor without breaking existing baseline path.
3. Unit tests verify required fields.

## Implementation Notes
- Candidate files:
  - `crates/amlich-core/src/almanac/types.rs`
  - `crates/amlich-core/src/almanac/data.rs`

## Test Plan
- `cargo test -p amlich-core` targeted module tests.

---

## D-0004 - Known differences policy

## Meta
- ID: `D-0004`
- Type: docs
- Status: TODO
- Priority: P1
- Phase: 0
- Depends on: `R-0002`

## Context
We need clear policy on variant behavior and output differences.

## Deliverables
- `docs/almanac/known-differences.md`
- Link from `ruleset-v1-scope.md` and `decision-log.md`.

## Acceptance Criteria
1. Policy defines how variant differences are communicated.
2. Policy defines version bump requirements for behavior changes.
3. Policy defines no-silent-change rule.
