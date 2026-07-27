# Personal-Day Scoring Policy v2

Status: draft implementation spec

## Decision summary

`amlich-core` will use a constrained, explainable multi-criteria model for
`PersonalDayAssessment`:

```text
typed facts → normalized feature vector → sparse policy weights
           → explicit interaction terms → five assessment axes
           → veto/guardrail policy → decision bucket
```

The semantic graph remains the explanation and provenance projection. It is
not the numerical scoring engine. A neural model is out of scope.

## Scope

This spec changes the canonical personal-day decision surface only. It does
not redesign:

- Bazi chart construction or Bazi domain scoring;
- baseline almanac rule derivations;
- recommendation-pack semantics;
- semantic-graph identity, merge, or visualization contracts.

Bazi, interaction matrices, yearly Hạn, Kua, and recommendations remain
upstream signal providers. Their outputs become typed assessment features.

## Model

### Feature vector

Introduce stable feature identifiers and normalized values in `[-1, 1]`:

- `generic_day_quality`
- `intent_fit`
- `personal_same_chi`
- `personal_luc_xung`
- `personal_tam_hop`
- `personal_liu_he`
- `kua_direction_match`
- `timing_hoang_dao_ratio`
- `annual_tam_tai`
- `annual_kim_lau`
- `annual_hoang_oc`
- `annual_thai_tue`
- `bazi_element_resonance`
- `evidence_coverage`

Every feature carries availability, source evidence, and a stable
`contribution_id`. Unavailable is distinct from zero.

### Sparse policy matrix

The policy maps features to the five existing axes:

```text
axis_score[a] = baseline[a]
              + Σ(weight[policy, intent, a, f] × feature[f])
              + Σ(interaction_weight[i] × interaction[i])
```

Only declared feature/axis pairs and declared interactions are evaluated.
Weights and formulas are versioned by `policy_id` and `policy_version`.

Axis scores are clamped to `[0, 1]` after aggregation. The denominator must
reflect the absolute weights of available features, so a missing feature does
not silently behave as a neutral signal or inflate confidence.

### Interactions

Interactions are explicit, typed features rather than a general tensor. Initial
supported terms are:

- `hard_taboo × requested_activity`;
- `personal_relation × important_birth_pillar`;
- `weak_element × day_generates_element`;
- `kua_direction × travel_intent`;
- `annual_pressure × requested_activity`.

No interaction is inferred merely because two source facts coexist. Each term
needs a policy entry, source evidence, and a test fixture.

### Vetoes and guardrails

Hard vetoes are separate from weighted contributions. A veto has:

- `veto_id`;
- applicable activity/intent scope;
- reason and source evidence;
- policy version;
- deterministic precedence.

Veto precedence is evaluated before weighted suitability. A strong negative
signal must not be represented only as a large weight or as
`strength >= 0.8`.

### Intent projection

The five axis scores remain visible in the result. The final decision uses an
intent-specific axis-weight vector rather than an equal average:

```text
decision_score = Σ(intent_axis_weight[intent, axis] × axis_score[axis])
```

The final bucket thresholds and semantic labels remain policy-versioned and
are applied after vetoes and availability checks.

### Confidence

Confidence is derived from evidence coverage and source quality, not just the
number of birth fields supplied. It must report both:

- what was available;
- what was missing and therefore excluded.

## Proposed interface

Keep callers on the existing assessment seam and hide policy mechanics behind
one deep module:

```rust
pub struct AssessmentPolicy {
    pub policy_id: String,
    pub policy_version: String,
    // private feature weights, interaction rules, vetoes, thresholds
}

impl AssessmentPolicy {
    pub fn baseline_v2() -> Self;

    pub fn evaluate(
        &self,
        inputs: AssessmentInputs,
        snapshot: &DaySnapshot,
        profile: &BirthProfile,
        intent: ConsultationIntent,
    ) -> PersonalDayAssessment;
}
```

The public result remains `PersonalDayAssessment`; existing API, TUI, and
desktop callers continue consuming axes, contributions, decision, and
evidence. The policy implementation owns extraction, normalization, weighting,
interaction evaluation, veto precedence, and trace construction.

## Migration from current implementation

The current implementation already provides the target result shape:

- five axes;
- stable contributions;
- source and ruleset provenance;
- unavailable sections;
- policy identifiers;
- decision bucket and confidence.

Replace incrementally:

1. Extract current contribution creation into a feature extraction module.
2. Add `AssessmentFeatureId`, availability masks, and normalized values.
3. Introduce `AssessmentPolicy::baseline_v2()` with weights equivalent to the
   current behavior where practical.
4. Separate vetoes from ordinary contributions.
5. Add intent-specific axis weights.
6. Add explicit interaction features.
7. Emit a calculation trace consumable by the semantic graph.
8. Run v1 and v2 in parity tests; review every decision divergence.
9. Promote v2 only after sensitivity and golden-data gates pass.

## Verification gates

Required tests:

- deterministic output for identical inputs and policy version;
- feature-level contribution IDs remain stable;
- missing input is unavailable, not an implicit zero;
- hard veto always wins over favorable weighted signals;
- changing an unrelated feature does not change an axis;
- duplicate evidence does not inflate a score;
- policy weights are sensitivity-tested at ±10% and ±20%;
- v1/v2 parity fixtures explain intentional divergences;
- semantic-graph nodes and edges preserve feature, weight, and source trace;
- API/TUI/desktop projections remain byte- and key-compatible where fields
  are unchanged.

## Non-goals

- machine-learned or neural scoring;
- automatically discovering new interactions;
- claiming that numeric weights are universal traditional truth;
- replacing source-cited domain rules with web-sourced calculator values;
- collapsing the five axes into an opaque scalar in the core result.

## Research boundary

External research is required to validate disputed rule semantics and source
authority. Numeric weights are a versioned product policy and must be reviewed
with sensitivity analysis and project fixtures; no online source is assumed to
provide a canonical numeric matrix.

Research findings supporting this boundary and the MCDA/veto/provenance choices
are recorded in [`docs/almanac/research-scoring-policy.md`](../../almanac/research-scoring-policy.md).
