# Recommendation Safety Policy (v1)

## Purpose

Define product and source-policy guardrails for recommendation domains where overconfident or culturally tone-deaf output would damage user trust.

This policy applies to:

- recommendation synthesis in `crates/amlich-core/src/almanac/recommendation/*`
- API translation and DTO shaping
- TUI, desktop, and other presentation layers that render recommendation wording

This document is a policy contract. It does not itself add new almanac rules.

## Scope

This policy is required for:

- burial and funeral related activities
- medical or urgent-life-context activities
- any future personalized recommendation layer
- confidence or certainty presentation
- wording used for strong positive or strong negative claims

## Policy Baseline

### 1. Source hierarchy governs strong claims

Strong recommendation claims require:

1. a frozen project rule family or accepted decision
2. deterministic rule mapping and regression coverage
3. provenance that can point back to the relevant rule family or policy decision

Practice-facing or compilation-style sources may inform optional packs, terminology, or UX expectations, but they must not silently justify default-core strong claims.

### 2. Safety-sensitive domains default to conservative output

When a domain is culturally sensitive, profile-sensitive, or high-consequence, the default engine should prefer:

- `neutral`
- `caution`
- `consult_expert`

over aggressive positive endorsement.

Absence of a blocker is not sufficient reason to emit a strong positive recommendation.

### 3. Determinism beats fake precision

The engine may explain which rule families fired, but it must not present invented certainty.

Do not:

- invent percentages from sparse or mixed evidence
- imply consensus when sources are variant-sensitive
- collapse conflicting traditions into a single unexplained verdict

## Sensitive Domain Rules

### Burial / funeral

Default v1 posture:

- do not emit aggressive positive burial or funeral recommendations
- do not map generic auspicious-day language directly onto burial/funeral output
- do allow explicit cautionary or blocking signals when a deterministic taboo or activity-specific avoid rule exists
- do prefer expert-consult wording when product requires an output but evidence is mixed or incomplete

Rationale:

- this domain is culturally sensitive
- source material is variant-heavy
- false confidence here creates outsized trust risk

Allowed default wording patterns:

- `Not recommended`
- `Needs expert review`
- `Use caution`
- `No strong automated recommendation`

Disallowed default wording patterns:

- `Highly auspicious for burial`
- `Guaranteed safe`
- `Best possible day`

If burial/funeral automation is expanded later, it must ship behind explicit policy review, dedicated tests, and separate wording review.

### Medical / urgent contexts

Default v1 posture:

- recommendations may describe traditional support or caution signals
- recommendations must not imply medical efficacy, safety, or outcome guarantees
- urgency and real-world necessity outrank almanac logic

Required UX implication:

- render an explicit disclaimer in product surfaces if medical-context guidance is shown

### Personalized recommendation layers

Default v1 posture:

- keep profile-aware logic out of the date-only engine
- do not infer birth-year or compatibility advice without explicit user input
- personalized outputs must remain separately versioned and testable

## Confidence Semantics

### Default rule

Do not expose numeric confidence scores in default v1 recommendation APIs or UI.

Reason:

- the current engine is precedence-based, not probabilistic
- percentages would overstate certainty
- source convergence varies by rule family

### Allowed confidence expression

The system may express confidence only through explicit structural semantics, such as:

- provenance metadata
- whether the result is based on frozen baseline rules vs optional packs
- whether the output is blocked, cautionary, or advisory
- whether evidence is direct and activity-specific vs generic and weak

### Future numeric confidence requirement

Numeric confidence may be added only if it is derived from explicit semantics, not opaque scoring. At minimum it would need:

- rule-class definitions
- evidence-convergence criteria
- contract documentation
- API parity tests
- wording review in consumer surfaces

## Wording Limits

### Strong positive claims

Reserve strong positive wording for cases where:

- the recommendation is not in a safety-sensitive domain
- no stronger blocker exists
- the positive signal comes from a high-authority, deterministic rule family

Even then, avoid absolute language.

Allowed examples:

- `Suitable`
- `Generally favorable`
- `Supported by current baseline rules`

Disallowed examples:

- `Guaranteed`
- `Certain`
- `Perfect`
- `Risk-free`

### Strong negative claims

Negative wording must be tied to explicit reasons.

Prefer:

- `Not recommended because ...`
- `Avoid due to ...`
- `Blocked by ...`

Avoid unexplained fear language or generalized doom language.

### Generic auspiciousness labels

Do not use one generic label as if it were valid for every activity.

Examples:

- a favorable general day signal does not automatically mean favorable for burial
- a cautionary modifier does not automatically veto every activity

Presentation layers should favor activity-specific wording over global day adjectives.

## Implementation Gates

Any new sensitive recommendation domain must not ship until all of the following are true:

1. the source family is documented
2. the default behavior is classified as `default_core`, `optional_pack`, `personalization_layer`, `defer`, or `reject`
3. wording rules are written down
4. deterministic fixtures exist
5. API/UI contract impact is reviewed

## Related Documents

- `docs/almanac/recommendation-research-reconciliation.md`
- `docs/almanac/recommendation-conflict-triage.md`
- `docs/almanac/recommendation-rule-matrix.json`
- `docs/almanac/recommendation-promotion-order.json`
- `docs/almanac/research-sources.md`
- `docs/almanac/known-differences.md`
- `docs/almanac/decision-log.md`
