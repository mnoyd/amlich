# Personalized Recommendation Layer (v1 Boundary Design)

## Purpose

Define the boundary for profile-aware recommendation logic so personalized signals can be added without contaminating the default date-only engine.

This design covers:

- `xung tuoi` and birth-year driven logic
- event-kind-aware advice
- future user profile inputs

## Policy Baseline

Default v1 recommendations remain:

- date-only
- deterministic
- safe to compute without user profile data

Personalized logic must be a separate layer, not an invisible extension of baseline output.

## Why Separation Is Required

- profile data is optional and consent-sensitive
- many rule families depend on birth-year or event context
- mixing personalized logic into date-only output would make results harder to explain and test
- consumers need to know whether a result is general advice or user-specific advice

## Layer Shape

Recommended execution order:

1. compute baseline daily recommendations
2. apply enabled recommendation packs
3. apply profile-aware recommendation layers
4. apply final presentation wording

This keeps personalized changes interpretable as additions or overrides on top of a visible baseline.

## Proposed Profile Model

```rust
struct RecommendationProfile {
    profile_id: String,
    birth_year: Option<i32>,
    gender: Option<String>,
    event_kind: Option<String>,
    locale: Option<String>,
}
```

Future fields may include:

- birth date if a rule family truly requires it
- relationship role for marriage-related advice
- household or construction context

No field should be collected unless a concrete recommendation family needs it.

## Personalization Categories

### Birth-year compatibility

Examples:

- `xung tuoi`
- age-cycle warnings
- event-role matching

Policy:

- require explicit user input
- must be attributable to a named profile-aware family

### Event-kind-aware logic

Examples:

- marriage-specific hard filters
- construction-specific person rules
- travel-specific personal advisories

Policy:

- event kind should be explicit, not guessed from UI context
- recommendations should remain scoped to that event

### Consent-sensitive enrichment

Examples:

- persistent saved profiles
- household preferences

Policy:

- keep outside default API responses unless requested

## Contract Direction

Recommended request model:

- baseline request may omit profile entirely
- personalized request includes `profile_id` or inline profile payload

Recommended response model:

- keep `daily_recommendations` for baseline
- add `profile_recommendations` or explicit profile-layer metadata when profile logic runs

Do not silently mix profile-only reasons into baseline reason lists without a visible marker.

## RecommendationLayer Mapping

The current `RecommendationLayer` interface is already suitable for this boundary.

Recommended implementation direction:

- one layer per profile-aware family or family group
- layer ids such as `profile.xung_tuoi` or `profile.event.marriage`
- explicit provenance notes marking the output as profile-derived

## Conflict Rules

When personalized logic conflicts with baseline:

1. baseline hard-stop taboo and safety-policy rules remain in force
2. profile-aware hard filters may demote baseline favorable activities for the specific event/profile
3. personalized output must clearly state that the change is profile-dependent

This avoids accidental interpretation of a user-specific warning as a universal day verdict.

## Testing Requirements

Every personalized layer needs:

1. profile-free baseline regression tests
2. profile-on vs profile-off comparison tests
3. explicit fixture cases near age or calendar boundaries
4. API parity tests for profile fields and output metadata

## Non-Goals for v1

- automatic profile inference
- mandatory profile collection
- embedding profile-sensitive logic in the baseline recommendation corpus

## Related Documents

- `docs/almanac/recommendation-safety-policy.md`
- `docs/almanac/recommendation-pack-architecture.md`
- `docs/almanac/recommendation-conflict-triage.md`
- `docs/almanac/recommendation-research-reconciliation.md`
