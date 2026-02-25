# Phase 1 Beads - Ruleset Infrastructure

## R-1001 - Catalog current almanac dataset fields

## Meta
- ID: `R-1001`
- Type: research
- Status: TODO
- Priority: P0
- Phase: 1
- Depends on: `R-0002`

## Context
Current `baseline.json` and loader already contain useful structure, but we need a field inventory before schema refactor.

## Deliverables
- Field catalog of current dataset and loader responsibilities.
- List of extension points for future rule families.

## Acceptance Criteria
1. Every top-level key in current dataset is listed.
2. Loader validation responsibilities are documented.
3. Extension points and schema risks are identified.

## Implementation Notes
- Primary references:
  - `crates/amlich-core/data/almanac/baseline.json`
  - `crates/amlich-core/src/almanac/data.rs`

---

## I-1002 - Implement ruleset registry and loader

## Meta
- ID: `I-1002`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 1
- Depends on: `R-1001`, `I-0003`

## Context
We need to move from one hard-coded baseline loader to a ruleset registry that can load multiple packs.

## Deliverables
- Ruleset registry abstraction in `amlich-core`.
- Baseline ruleset registered as `vn_baseline_v1` (or transitional alias + version metadata).

## Acceptance Criteria
1. Core can resolve a ruleset by id.
2. Existing baseline behavior remains available.
3. Errors are explicit for unknown ruleset ids.

## Implementation Notes
- Start with in-binary static registry using `include_str!`.
- Defer dynamic external loading unless needed.

## Test Plan
- Unit tests for known/unknown ruleset lookup.
- Regression test proving baseline outputs unchanged.

---

## I-1003 - Add schema and token validators

## Meta
- ID: `I-1003`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 1
- Depends on: `I-1002`

## Context
Multi-ruleset support increases the risk of silent bad data.

## Deliverables
- Validation hooks for ruleset metadata and family-specific tokens.
- Validation tests with invalid fixtures (unit-level).

## Acceptance Criteria
1. Invalid ids/tokens fail with clear error messages.
2. Required metadata fields are enforced.
3. Existing baseline pack passes validation.

## Test Plan
- Add negative tests in `almanac::data`.

---

## I-1004 - Wire ruleset_id into day computation

## Meta
- ID: `I-1004`
- Type: implementation
- Status: TODO
- Priority: P0
- Phase: 1
- Depends on: `I-1002`, `I-1003`

## Context
Ruleset selection must be part of compute context and output provenance.

## Deliverables
- New day-info path or parameterized function accepting `ruleset_id`.
- `DayFortune` (or parent output) carries ruleset version metadata.

## Acceptance Criteria
1. Ruleset id can be passed to core day computation.
2. Output exposes selected ruleset id/version.
3. Existing default path still works (defaulting to baseline).

## Test Plan
- Unit/integration tests for default + explicit ruleset path.

---

## T-1005 - Determinism tests across rulesets

## Meta
- ID: `T-1005`
- Type: test
- Status: TODO
- Priority: P1
- Phase: 1
- Depends on: `I-1004`

## Context
Ruleset versioning is only safe if outputs are reproducible.

## Deliverables
- Determinism tests over a fixed date sample and explicit ruleset ids.

## Acceptance Criteria
1. Same input + same ruleset -> same output.
2. Unknown ruleset -> explicit error.
3. Default ruleset path remains stable.

---

## D-1006 - Loader and fallback documentation

## Meta
- ID: `D-1006`
- Type: docs
- Status: TODO
- Priority: P1
- Phase: 1
- Depends on: `I-1004`, `T-1005`

## Context
Contributors need to know how rulesets are loaded and how default behavior is chosen.

## Deliverables
- `docs/almanac/ruleset-loader.md`
- Example of adding a new ruleset pack safely

## Acceptance Criteria
1. Documents default/fallback behavior.
2. Documents validation expectations.
3. Documents versioning requirements for behavior changes.
