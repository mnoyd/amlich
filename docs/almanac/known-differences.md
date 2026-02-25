# Known Differences Policy (`D-0004`)

## Purpose

Define how the project documents and ships behavior differences across rulesets, versions, and sources.

This policy exists to prevent silent output drift and to make differences explainable to users, contributors, and downstream clients.

## Core Policy (Short Version)

1. **No silent behavior changes** for an existing `(ruleset_id, ruleset_version)`.
2. **Document known differences** whenever a rule family is variant-sensitive.
3. **Version bump on breaking behavior changes**.
4. **Prefer additive metadata** over implicit assumptions.
5. **Keep deterministic evidence** so differences can be traced.

## What Counts as a "Known Difference"

A known difference is any repeatable output difference caused by one or more of:

- different source tables (e.g. `sat_chu`, `tho_tu`, day-deity variants)
- different ruleset version or variant
- timezone/default settings affecting local-day results
- deterministic algorithm mode differences (e.g. fast vs accurate in future phases)
- display/localization policy (when presentation changes are intentional and documented)

## No-Silent-Change Rule (Required)

For any published ruleset version:

- Do **not** change rule tables, formula conventions, token meanings, or ordering semantics in-place if the change alters emitted behavior.
- Do **not** replace variant-sensitive tables under the same ruleset version without documenting a migration and bumping version where required.
- Do **not** merge behavior changes hidden inside unrelated refactors.

If behavior must change:

- create a new ruleset version (or variant ruleset)
- document the difference in release/docs
- add/update tests that prove the changed behavior is intentional

## Version Bump Requirements

### Requires ruleset version bump

Bump `ruleset_version` when any change alters observable behavior for the same inputs.

Examples:

- changing taboo month->chi tables (`sat_chu`, `tho_tu`)
- changing day-deity cycle order or month-group starts
- changing star precedence behavior that changes final `cat_tinh`/`sat_tinh`
- changing severity defaults that affect emitted `taboos[]`
- changing canonical source selection for a variant-sensitive family

### Usually does NOT require ruleset version bump

Examples (assuming behavior is unchanged):

- docs clarifications
- internal refactors with preserved outputs
- additive metadata fields with stable defaults
- stricter validation that rejects only previously invalid data
- display-only aliases added outside core output semantics

## How to Document Differences

When introducing or freezing a difference-prone rule family, document all of the following:

1. **Canonical selection** (which mapping/table/formula was chosen)
2. **Variant caveats** (what alternatives exist)
3. **Output impact** (which fields may differ)
4. **Versioning rule** (what requires version bump)
5. **Tests/fixtures** (how drift is detected)

Recommended locations:

- family-specific docs (`docs/almanac/*.md`)
- `docs/almanac/decision-log.md`
- `docs/almanac/ruleset-v1-scope.md`
- future release notes / migration docs

## Evidence and Traceability Requirements

When possible, outputs should include provenance metadata to support difference analysis:

- `ruleset_id`
- `ruleset_version`
- field/rule `evidence` (`source_id`, `method`, `profile`)

This allows downstream clients to diagnose why two results differ without guessing.

## Differences by Category (v1 Guidance)

### 1) Source-table differences (variant-sensitive)

Examples:

- `sat_chu` / `tho_tu`
- day hoang dao/hac dao mapping tables
- future `hoang_oc`, `cuu_dieu`, yearly `han`

Policy:

- Freeze one canonical mapping per ruleset version
- Record caveats and lineage in docs
- Ship alternates as new version/variant only

### 2) Deterministic mode differences

Examples (future):

- fast vs accurate tiết khí mode
- official vs astronomical/historic mode

Policy:

- Modes must be explicit in API/compute context (not hidden global switches)
- Differences must be documented with expected impact ranges

### 3) Locale/presentation differences

Examples:

- label spelling variants (`Huyền Vũ` vs `Nguyên Vũ`)
- locale animal labels (presentation-only)

Policy:

- Prefer keeping core symbolic identity stable
- Presentation changes should not alter logic outputs unless explicitly versioned as ruleset behavior

## Contributor Workflow for Behavior Changes

Before merging a change that may alter outputs:

1. Identify affected rule family and fields.
2. Decide whether change is behavior-changing or metadata/docs-only.
3. If behavior-changing:
   - bump ruleset version / create variant
   - update ruleset docs + decision log
   - update golden/determinism tests
4. If not behavior-changing:
   - document why outputs remain stable
5. Add a changelog/release note entry (future release process)

## Examples (Applied to Current Docs)

- Day deity mapping freeze (`DEC-0006`) is documented as canonical v1 mapping; alternate slot assignments require new version/variant.
- Taboo rule freeze (`DEC-0008`) documents fixed v1 `sat_chu`/`tho_tu` tables and severities; replacing those tables in-place is not allowed.
- Taboo explanation templates (`DEC-0007`) are deterministic output text conventions; changing templates may require versioning if clients depend on exact strings.

## Related Documents

- `docs/almanac/ruleset-v1-scope.md`
- `docs/almanac/decision-log.md`
- `docs/almanac/ruleset-loader.md`
- `docs/almanac/day-hoangdao-hacdao.md`
- `docs/almanac/taboo-rules.md`
