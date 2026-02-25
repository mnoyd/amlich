# Ruleset Loader and Fallback Guide (`D-1006`)

## Purpose

Explain how almanac rulesets are resolved, what fallback behavior is guaranteed, and what validation/versioning rules contributors must follow.

Primary implementation reference:

- `crates/amlich-core/src/almanac/data.rs`

## Current Loader Model

The current model is an in-binary static registry with embedded JSON data.

### Data source

- Baseline payload is embedded with `include_str!` from:
  - `crates/amlich-core/data/almanac/baseline.json`

### Initialization

- Parsed and normalized once through `OnceLock<AlmanacData>`.
- The baseline loader path is `baseline_data()`.

### Registry

- Canonical ruleset id: `vn_baseline_v1`
- Transitional alias: `baseline`
- Registry entries are resolved via:
  - `get_ruleset(ruleset_id)`
  - `get_ruleset_data(ruleset_id)`
- Unknown id behavior:
  - returns `RulesetLookupError::UnknownRulesetId(...)`

## Default and Fallback Behavior

### Default behavior

- `default_ruleset()` returns the first (and currently only) registry entry.
- Default ruleset id is `vn_baseline_v1`.

### Compatibility fallback

- Legacy paths that call `baseline_data()` remain valid.
- `baseline_data()` is treated as a compatibility shim for the default baseline ruleset data.

### Alias behavior

- `baseline` resolves to the same registry entry/data as `vn_baseline_v1`.
- Alias exists for migration convenience; new integrations should prefer canonical id.

## Validation Expectations

Ruleset loading currently enforces validation before data is used.

### 1) Descriptor-level expectations

When building the descriptor doc (`get_ruleset_descriptor_doc`), these are enforced:

- non-empty `id`, `version`, `profile`
- allowed `region` (currently `vn`)
- `defaults.tz_offset` in `-12..=14`
- `schema_version` must match `ruleset-descriptor/v1`
- source notes must be unique by family and non-empty

### 2) Family metadata expectations

- Each `SourceMeta` must include:
  - non-empty `source_id`
  - valid `method` token (`table-lookup`, `bai-quyet`, `jd-cycle`)

### 3) Data-family schema expectations (baseline)

Examples enforced today:

- `travel_by_can` has exactly 10 can keys
- `conflict_by_chi` has exactly 12 chi keys
- direction tokens are from allowed compass labels
- `nhi_thap_bat_tu` has 28 entries with valid quality tokens
- star rule maps validate key domains and duplicate star constraints
- day deity rule set validates 12-cycle + full month-group key coverage
- taboo rule sets validate rule ids, severity tokens, and month/day constraints

## Error and Failure Semantics

Two error modes currently exist:

1. **Lookup errors** (recoverable):
   - unknown ruleset id -> `RulesetLookupError`

2. **Validation failures** (panic/assert path):
   - invalid embedded schema/tokens currently fail fast via `assert!`/`expect!`

Contributor note:

- This is acceptable for embedded baseline development, but multi-pack external loading should evolve toward typed validation errors with field-path context.

## Versioning Rules for Loader/Data Changes

### Required rules

1. Do not silently replace behavior for an existing `(ruleset_id, ruleset_version)`.
2. Breaking behavior changes require a new `ruleset_version` (and usually new fixture snapshots).
3. Keep canonical id stable (`vn_baseline_v1`) for current baseline unless a planned migration is documented.
4. Alias changes must be additive/migration-safe.

### Practical examples

- **Allowed without version bump:**
  - docs clarifications
  - additional non-breaking metadata fields with defaults
  - stricter validation that only catches previously invalid data

- **Requires version bump:**
  - changing taboo month->chi mapping values
  - changing day deity cycle ordering/classification
  - changing deterministic precedence that alters emitted results

## Adding a New Ruleset Safely (Workflow)

1. Define new identity/version and scope in docs.
2. Add a new data pack JSON (do not overwrite existing baseline behavior).
3. Register new entry in static registry.
4. Reuse/extend validators for all enabled families.
5. Add deterministic tests for known dates under explicit ruleset id.
6. Document known differences and migration/fallback behavior.

## Consumer Guidance

- Prefer explicit `ruleset_id` usage in call paths as Phase 1 evolves.
- Treat missing/unknown ruleset ids as explicit errors.
- Keep output provenance (`ruleset_id`, `ruleset_version`, evidence) visible in higher-level contracts.

## Related Docs

- `docs/almanac/ruleset-v1-scope.md`
- `docs/almanac/ruleset-baseline-field-catalog.md`
- `docs/almanac/master-plan.md`
- `docs/almanac/decision-log.md`
