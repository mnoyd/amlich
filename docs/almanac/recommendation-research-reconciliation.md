# Recommendation Research Reconciliation (v1)

## Purpose

Reconcile the independent recommendation-system research docs added on 2026-03-09 with the current `amlich` recommendation architecture, and decide what should become:

- default core behavior
- optional ruleset/pack behavior
- deferred work
- rejected implementation direction

This document is a policy bridge between research and code. It is not a source of new almanac rules by itself.

## Inputs Reviewed

- `docs/almanac/daily-guidance-research-and-rust-proposal.md`
- `docs/Almanac Recommendation Logic Framework.txt`
- `docs/deep-research-report (1).md`
- `docs/deep-research-report (2).md`

## Current Code Baseline

Current recommendation synthesis already exists and is deterministic:

- taxonomy: `crates/amlich-core/src/almanac/recommendation/activity.rs`
- contract: `crates/amlich-core/src/almanac/recommendation/types.rs`
- synthesis engine: `crates/amlich-core/src/almanac/recommendation/synthesize.rs`
- source actionability policy: `docs/almanac/recommendation-source-actionability.md`
- validation corpus: `docs/almanac/recommendation-corpus.md`

The main question is not whether to build a recommendation system. The question is how to evolve the existing one without mixing incompatible source traditions or introducing fake precision.

## High-Level Verdict

The research docs mostly support each other on structure and product safety:

- recommendations should be layered
- activity-specific outputs are preferable to generic day labels
- hard blockers must be separated from softer modifiers
- provenance must be preserved
- source variance must be handled explicitly

The main disagreements are:

- precedence-first engine vs score-driven engine
- whether `nhi_thap_bat_tu` belongs in default v1
- whether modern Vietnamese folk taboo sets should be part of default core logic or optional packs
- how much authority to give `hoang_dao/hac_dao`

## Decision Summary

### Accepted for default core

- Layered recommendation synthesis
- Activity-first output model
- Hard-block vs modifier separation
- Per-reason provenance metadata
- `truc` as a major activity-routing signal
- `hoang_dao/hac_dao` as a bounded modifier
- explicit source-family separation between core and optional packs

### Accepted with constraints

- limited use of scoring
  - allowed only as a bounded tie-break or summary aid
  - not allowed as the primary authority model
- modern Vietnamese practice sources
  - allowed for terminology, activity labels, expected feature scope, and optional pack design
  - not allowed to silently override court-standard or previously frozen baseline logic

### Deferred

- `nhi_thap_bat_tu` as a default veto engine
- personalized birth-year and marriage matching logic
- broad burial/funeral recommendation automation
- long-tail `than_sat` expansion without a curated source table

### Rejected

- a purely score-threshold engine such as `score > X => recommended`
- flattening Xieji-style rules, Yuxia-style compilations, and modern Vietnamese web-practice sources into one unversioned consensus model
- using `hoang_dao/hac_dao` as a sole or dominant day verdict

## Claim Matrix

| Claim | Support | Decision |
|---|---|---|
| Recommendations must be layered, not driven by one signal | All reviewed docs | Accept as core architecture |
| Activity-specific guidance is the right output model | All reviewed docs | Accept |
| Hard blockers must be separated from soft modifiers | All reviewed docs | Accept |
| Provenance/evidence should be attached to recommendations | All reviewed docs | Accept |
| `truc` is a major rule family for activity routing | All reviewed docs | Accept as default |
| `hoang_dao/hac_dao` should influence output | All reviewed docs | Accept as modifier only |
| `nhi_thap_bat_tu` is useful for recommendation logic | All reviewed docs | Accept only as optional or versioned pack for now |
| `tam_nuong`, `nguyet_ky`, and similar folk taboo sets matter in VN practice | Framework + deep reports + current project docs | Accept as optional VN-practice pack or explicitly frozen baseline family only |
| Modern VN user expectations should shape the product surface | All reviewed docs | Accept for UX and pack design, not as sole logic authority |
| A Xieji-like source hierarchy should anchor strong claims | Deep reports strongly; compatible with others | Accept as source policy baseline |
| Numeric scoring is the best main decision engine | Framework strongly; others only partially | Reject as primary model |
| `hoang_dao/hac_dao` can serve as a day-quality baseline | Daily guidance + framework | Accept at low authority |
| `hoang_dao/hac_dao` should not override stronger activity rules | Deep reports strongly; compatible with others | Accept |
| `nhi_thap_bat_tu` should be in default v1 | Daily guidance leans yes; deep reports caution | Defer from default |
| Burial/funeral logic should be auto-recommended like normal activities | No strong safe consensus | Reject for default v1 |
| Personalization by `xung tuoi` / birth data is valuable | Framework + deep reports | Defer to separate personalization layer |
| Core rules should be separated from optional packs or tradition modes | Deep reports strongly; compatible with others | Accept |

## Architectural Reconciliation

### 1. Engine style

Recommended engine style:

- precedence-first
- deterministic
- evidence-preserving
- activity-centric

Practical interpretation:

- hard taboo or non-overridable blocker wins
- activity-specific avoid beats generic positive day labels
- multiple positive signals can upgrade a recommendation only when no stronger blocker is present
- scoring, if used, should only summarize already-bounded evidence rather than replace precedence rules

This fits the existing `merge_hits(...)` approach in `synthesize.rs` much better than a large score-only BRMS design.

### 2. Source hierarchy

Recommended hierarchy for strong claims:

1. project-frozen baseline docs and accepted decisions
2. court-standard / highly structured source families reflected in the deep research
3. compilation and practice-facing source families as optional packs or advisory layers
4. modern consumer-web summaries only as expectation checks, not default authority

This means the codebase should continue to avoid silent source blending.

### 3. Ruleset policy

The research implies at least three conceptual layers:

- `core`
  - deterministic baseline rules with stable provenance
- `optional pack`
  - tradition-specific or source-variant families such as mansion packs or folk taboo packs
- `personalization`
  - birth-year, event-kind, and profile-sensitive logic

The current code already has a useful hook for this direction through layered synthesis.

## Guidance by Rule Family

### `truc`

Status:

- default core

Why:

- all research views `truc` as high-signal and highly actionable
- it maps cleanly to current activity taxonomy
- it is explainable in UI

Implementation note:

- keep `truc` as one of the main routing sources for `opening_start`, `contract_agreement`, `construction_groundbreaking`, `medical_treatment`, `travel`, and similar categories

### `hoang_dao/hac_dao`

Status:

- default core modifier

Why:

- highly recognizable to users
- useful as a bounded support/caution signal
- not strong enough to replace activity rules

Implementation note:

- preserve current modifier treatment
- do not let it override stronger taboos or activity-specific avoid signals

### `nhi_thap_bat_tu`

Status:

- defer from default core
- prepare as versioned optional pack

Why:

- research agrees it is important
- research also indicates high variant risk and high consequence when mapped incorrectly

Implementation note:

- require explicit computation/source validation before promotion into default recommendation synthesis

### Folk taboo families

Examples:

- `tam_nuong`
- `nguyet_ky`
- Yang Gong style taboo sets

Status:

- keep explicit and versioned

Why:

- they matter in Vietnamese practice
- they should not be disguised as universal court-standard truth

Implementation note:

- current project already froze some taboo families for `vn_baseline_v1`
- continue to treat them as explicit rule families with provenance, not invisible consensus defaults

### Burial / funeral logic

Status:

- conservative handling only

Why:

- the research consistently implies this area is culturally sensitive and structurally distinct
- false confidence here is a product risk

Implementation note:

- do not emit aggressive positive burial recommendations in default v1
- prefer `consult expert` style handling or narrowly scoped cautionary output if product requires it

### Personalization

Status:

- defer to separate layer

Why:

- useful and expected
- requires profile input and different safety/UX handling

Implementation note:

- model as a `RecommendationLayer` or profile-aware ruleset extension, not as silent date-wide logic

## Coding Guidance

When converting future research into code, classify each candidate rule as exactly one of:

- `default_core`
- `optional_pack`
- `personalization_layer`
- `defer`
- `reject`

Each accepted rule should record:

- source family
- activity target
- polarity
- strength
- overridable vs non-overridable status
- evidence code

Rules should be added only when they can fit the current stable output contract in:

- `DailyRecommendations`
- `RecommendationReason`
- `RecommendationEvidence`

## Test Guidance

Any rule family promoted from research to code should add or update:

- core synthesis tests
- API parity tests
- corpus fixtures for bucket profile and required activities

Research alone is not sufficient for default behavior. Every promoted claim needs deterministic regression coverage.

## Recommended Next Steps

1. Build a machine-readable research-to-rules matrix from the reviewed docs.
2. Tag each candidate rule with one of the five classifications above.
3. Promote only `default_core` candidates that align with current source policy.
4. Open a separate design doc for optional `nhi_thap_bat_tu` and folk-taboo packs.
5. Keep personalization out of the default date-only engine until profile inputs are formalized.
