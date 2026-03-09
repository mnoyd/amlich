# Recommendation Pack Architecture (v1 Design)

## Purpose

Define how variant-sensitive recommendation logic can be added without destabilizing the default date-only engine.

This design covers:

- optional rule families such as `nhi_thap_bat_tu`
- future folk-taboo expansions
- source-variant or tradition-mode recommendation bundles

It does not promote any new pack into default v1 behavior by itself.

## Design Goals

- keep default core deterministic and stable
- avoid flattening conflicting traditions into one hidden consensus
- make pack activation explicit in API, CLI, and future UI surfaces
- allow per-pack tests, provenance, and versioning

## Core Model

### 1. Default core remains the baseline

Default recommendation synthesis continues to run with:

- frozen baseline rules
- no optional packs enabled
- conservative safety policy from `recommendation-safety-policy.md`

This preserves a stable `date-only` baseline for regression tests and consumer contracts.

### 2. Packs are additive recommendation layers

A recommendation pack should compile to one or more `RecommendationLayer` implementations.

Each pack may:

- add favor signals
- add caution or avoid signals
- add pack-scoped provenance

Each pack may not:

- silently replace baseline rule families
- override safety-policy guardrails
- mutate unrelated baseline outputs in-place

### 3. Packs are explicit selections, not inferred behavior

Pack enablement must come from an explicit request, such as:

- API query parameter
- CLI flag
- persisted profile selection

Never auto-enable a pack because the input date “looks compatible” with a tradition.

## Recommended Data Model

```rust
struct RecommendationPackDescriptor {
    pack_id: String,
    version: String,
    label: String,
    source_family: String,
    mode: RecommendationPackMode,
    compatibility: PackCompatibility,
}
```

Suggested supporting enums:

- `RecommendationPackMode`
  - `Advisory`
  - `TraditionVariant`
  - `Experimental`
- `PackCompatibility`
  - `AdditiveOnly`
  - `ConflictsWithDefaultFamily`
  - `RequiresProfile`

## Pack Categories

### Advisory pack

Use for source families that can add signals without redefining baseline logic.

Examples:

- `nhi_thap_bat_tu` guidance once validated
- additional non-blocking star-derived routing

Behavior:

- may add reasons and bucket pressure
- must preserve baseline provenance

### Tradition-variant pack

Use for source families that represent a real alternative mapping or tradition.

Examples:

- alternate taboo tables
- variant mansion interpretations

Behavior:

- must declare which baseline family it conflicts with
- should not run alongside an incompatible pack unless conflict handling is explicit

### Experimental pack

Use for research-stage layers that are not yet safe for broad promotion.

Behavior:

- disabled by default
- must carry obvious versioning and provenance warnings

## Activation Semantics

Recommended order:

1. baseline core
2. safety-policy guardrails
3. enabled additive packs
4. profile-aware personalization layer

Pack outputs should be inspectable in reasons through source prefixes such as:

- `pack.mansions.*`
- `pack.vn_practice.*`
- `pack.variant.*`

## Conflict Rules

When two packs or a pack and the baseline disagree:

1. hard baseline safety guardrails win
2. explicit hard-stop taboo signals win
3. default-core deterministic overrides win unless the user explicitly selected a conflicting tradition mode
4. additive supporting signals may coexist and produce `co_the`

If a pack represents a conflicting tradition mode, the product must surface that the result is mode-specific rather than baseline-global.

## Initial Pack Targets

### `nhi_thap_bat_tu`

Recommended initial shape:

- pack id: `pack.nhi_thap_bat_tu.v1`
- mode: `Advisory`
- status: not enabled by default

Reason:

- high user expectation
- high consequence if mapped incorrectly
- better to ship as explicit opt-in than to contaminate default outputs

### Future VN-practice taboo expansion

Recommended shape:

- pack id under `pack.vn_practice.*`
- mode depends on whether the family is additive or table-replacing

Reason:

- these families matter in practice
- they should remain explicit and versioned rather than silently universalized

## API / CLI Contract Direction

Recommended request shape:

- `recommendation_packs: ["pack.nhi_thap_bat_tu.v1"]`

Recommended response metadata:

- active pack ids
- pack versions
- pack source family

This keeps consumers aware of why two runs may differ.

## Testing Requirements

Every pack needs:

1. fixture cases with pack off vs pack on
2. deterministic bucket assertions
3. API parity coverage
4. conflict tests if the pack can coexist with others
5. documentation of known differences

## Non-Goals for v1

- shipping multiple packs in default mode
- inferring user tradition automatically
- replacing baseline rule tables without a ruleset/version boundary

## Related Documents

- `docs/almanac/recommendation-conflict-triage.md`
- `docs/almanac/recommendation-research-reconciliation.md`
- `docs/almanac/recommendation-safety-policy.md`
- `docs/almanac/known-differences.md`
