# Hour Ranking Policy v1

Status: draft design spec

## Decision summary

`amlich-core` will introduce a first-class hour-ranking policy that shares the assessment policy mechanics used by personal-day scoring, but keeps separate hour-specific feature IDs and vector axes.

The day policy answers whether a date is suitable. The hour policy answers only how to order the twelve traditional hour slots within that already-assessed day. A high-ranked hour must not change an `Avoid` day into a usable day.

```text
day snapshot + optional day assessment
  → hour feature observations
  → hour ranking vector
  → shared weighted aggregation mechanics
  → ranked twelve-hour list with traceable scores
```

## Scope

This spec changes hour selection/ranking only. It does not redesign:

- `PersonalDayAssessment` day verdicts;
- daily recommendation synthesis;
- BaZi chart construction;
- Hoàng Đạo/Hắc Đạo source derivation;
- CLI/API presentation policy.

## Domain boundary

Hour ranking is downstream of day assessment:

- day assessment decides `Favorable`, `Mixed`, `Cautious`, or `Avoid`;
- hour ranking orders all twelve traditional hour slots;
- if the day assessment is `Avoid`, hour ranking may still return results, but those results must carry warning context such as “best available hours on an avoided day”;
- consumers must not present ranked hours as a recommendation that overrides the day verdict.

## Candidate set

The ranker evaluates all twelve traditional hour slots. It does not filter to Hoàng Đạo hours only.

Consumers may filter or visually separate Hắc Đạo hours, but the core policy keeps them in the ranked list so explanations remain visible.

## Hour ranking vector

The initial vector has four semantic axes:

```text
hoang_dao_quality
intent_timing_fit
personal_hour_alignment
day_hour_harmony
```

All axis values are normalized to `0.0..1.0`.

Unavailable is distinct from zero. If an axis cannot be evaluated, its weight is removed from the denominator and the output confidence/context records the missing evidence.

### Hoàng Đạo quality

Binary in v1:

```text
Hoàng Đạo hour → 1.0
Hắc Đạo hour   → 0.0
```

Nuance should come from other axes, not invented Hoàng Đạo subgrades.

### Intent timing fit

Represents source-backed hour-specific support for the requested intent.

If no declared rule exists for the intent, this axis is unavailable and remaining weights are re-normalized. The ranker must not invent a neutral `0.5` fallback just to fill the vector.

### Personal hour alignment

V1 uses birth year Chi only, preserving parity with the current compatibility heuristic. Future BaZi/hour-pillar features may be added later under new feature IDs and a policy version bump.

Missing personal birth facts make this axis unavailable. Missing personal data is not a negative signal.

### Day-hour harmony

V1 uses only the branch relation between the day Chi and the hour Chi. It does not include day Can, hour Can, stars, deities, or other overlays in the first policy version.

## Initial weight profile

The first policy uses one shared weight profile across intents:

```text
hoang_dao_quality       0.45
intent_timing_fit       0.25
personal_hour_alignment 0.20
day_hour_harmony        0.10
```

Intent-specific behavior belongs inside the `intent_timing_fit` axis value. Per-intent weight profiles are deferred until fixtures show that different intents need different axis weights.

When one or more axes are unavailable, the denominator is the sum of available weights only:

```text
rank_score = Σ(axis_score × axis_weight) / Σ(available_axis_weights)
```

The score is clamped to `0.0..1.0`.

## Ranking order

Sort ranked hours by:

1. `rank_score` descending;
2. traditional Chi order for exact ties.

Do not tie-break alphabetically by Vietnamese Chi name.

## Output contract

The first-class policy exposes normalized scores and explanation data:

```rust
pub struct RankedHourV1 {
    pub chi_name: String,
    pub time_range: String,
    pub chi_index: usize,
    pub is_auspicious: bool,
    pub rank_score: f32,
    pub axes: HourRankingAxes,
    pub contributions: Vec<HourRankingContribution>,
    pub warning_context: Option<HourRankingWarning>,
}
```

The exact Rust names may change during implementation, but the semantic contract should remain:

- normalized `rank_score` in `0.0..1.0`;
- per-axis scores with unavailable state;
- traceable contributions/source evidence;
- warning context when a provided day assessment says the day is `Avoid`;
- no hour-level suitability bucket.

Compatibility wrappers may project `rank_score` back to the current integer `0..100` score used by `rank_hours_for_intent`.

## Proposed interface

```rust
pub struct HourRankingPolicy {
    // private policy id, version, axis weights, feature declarations
}

impl HourRankingPolicy {
    pub fn baseline_v1() -> Self;

    pub fn rank(
        &self,
        snapshot: &DaySnapshot,
        intent: ConsultationIntent,
        birth: Option<&BirthInput>,
        day_assessment: Option<&PersonalDayAssessment>,
    ) -> Result<Vec<RankedHourV1>, String>;
}
```

`DaySnapshot` is required. `PersonalDayAssessment` is optional so snapshot-only callers remain possible, but callers that have a canonical day assessment should pass it so warning context is preserved.

## Shared mechanics, separate language

Hour ranking should reuse the policy mechanics from day assessment where practical:

- feature observations;
- availability handling;
- weighted aggregation;
- contributions;
- trace shape/source evidence.

It should not reuse day feature IDs or day vector axes. Day assessment and hour ranking answer different domain questions and must remain separate at the domain-language level.

## Verification gates

Required tests:

- all twelve traditional hour slots are returned;
- Hoàng Đạo quality is binary under v1;
- unavailable personal alignment reweights remaining axes and lowers confidence/context;
- unavailable intent timing fit reweights remaining axes;
- exact score ties break by traditional Chi order;
- rank scores are deterministic for identical inputs and policy version;
- compatibility wrapper preserves current broad behavior: Hoàng Đạo hours generally rank above Hắc Đạo hours;
- an `Avoid` day assessment does not suppress ranking, but adds warning context;
- no hour result contains a day-level verdict or changes the day assessment bucket.

## Non-goals

- making an hour verdict bucket;
- allowing a strong hour to override an `Avoid` day;
- introducing neural or embedding-based ranking;
- adding BaZi hour-pillar scoring in v1;
- adding practical user calendar preferences in v1;
- claiming numeric weights are universal traditional truth.
