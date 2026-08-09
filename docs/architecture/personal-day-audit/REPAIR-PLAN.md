# Personal-day core repair plan (implemented)

This plan synthesizes the advisory/reasoning, birth/API, and interaction/almanac audits. It is intentionally a migration plan, not a source patch.

## Implementation status

The `amlich-mwbp` migration is implemented. The final reasoning migration
(`amlich-mwbp.8`) made these architectural decisions concrete:

- `PersonalDayAssessment` owns the recommendation bucket, confidence,
  semantic classification, primary conclusion, and normalized axis scores.
  The initiation/opening graph is now an evidence/presentation projection,
  not a second verdict engine.
- `SemanticFact` carries typed Trực opening hits, star polarity, Xung/Hợp
  state, and direction scores. Vietnamese summaries may be translated or
  reworded without changing a decision.
- Hard, soft, and unqualified taboo facts produce ordered canonical
  contribution strengths. Missing rule evidence cannot cross the implicit
  hard-veto threshold.
- Standalone reasoning constructs the assessment from the actual birth
  profile, matching aggregate/API paths that supply a precomputed assessment.
- The legacy graph corpus contained intentional canonicalization diffs:
  several old `Favorable`/`Cautious` graph verdicts are `Mixed`/`Avoid` in the
  already-public canonical assessment. Golden fixtures now lock the canonical
  result rather than preserving cross-surface disagreement.

Deletion and parity coverage lives in
`crates/amlich-core/tests/reasoning_graph_metamorphic.rs`,
`reasoning_graph_parity.rs`, and the personal-day API contract suites.

## Non-negotiable acceptance rubric

1. **One canonical assessment:** advisory score, semantic reasoning, matrix report, and API aggregate are projections of one normalized `PersonalDayAssessment`; no endpoint computes an independent verdict.
2. **No false personalization:** `net_resonance` must either be removed from user-facing output or demonstrably vary with contrasting element distributions. Generic day quality, personal alignment, and data completeness remain separate axes.
3. **Explicit availability:** unknown birth time is not midnight; missing gender/Hạn is unavailable, never numeric zero. Every output states capability/data tier and why a section is unavailable.
4. **Typed semantics:** branch relations, star polarity, Trực hits, direction rows, and contribution strengths are typed facts; localized summaries are presentation only.
5. **Determinism and provenance:** every user-visible score has policy/ruleset version, contribution IDs, and source evidence. Duplicate facts must not inflate a score accidentally.
6. **Action isolation:** initiation/opening evaluates only its allowlisted subgraph; unrelated graph nodes cannot change its result.
7. **Boundary correctness:** personal-hour rows map slot, Can Chi, and Hoàng Đạo star by the same index; `day_canchi` serializes real Can Chi; XIANGXING distinguishes pair/group/self semantics.
8. **Parity:** standalone advisory, reasoning, matrix, and aggregate API responses agree on normalized profile, snapshot, score, confidence, and availability.
9. **Safety:** medical/burial/contract outputs must not gain high confidence from incomplete profile or caution-message counts alone.

## How to use this plan

This document records target architecture, migration order, and release gates. It
does not duplicate line-level evidence from the companion audits:

- `advisory-reasoning.md` owns advisory/reasoning findings and deletion tests.
- `birth-api.md` owns birth-input, capability, and transport findings.
- `interaction-almanac.md` owns interaction-matrix and almanac findings.

Implementation work is tracked under the `amlich-mwbp` beads epic. When an
implementation discovery changes a target decision, update this plan first and
then update the affected bead. New evidence that does not change the target
architecture belongs in the relevant audit and bead, not in this document.

## Target ownership of scores and matrices

The word `matrix` currently covers both internal policy tables and serialized
interaction results. The migration must separate these concepts before adding
more scoring behavior.

| Current surface | Target role | Public after migration | Canonical verdict source | Disposition |
| --- | --- | --- | --- | --- |
| `BaziScoringMatrixSet` and its element/season/visibility/interaction/Ten-God/domain tables | Versioned Bazi scoring policy/weight tables | No, unless an explicit expert configuration API is approved | No | Rename conceptually to policy/weight tables; keep behind the Bazi metrics builder |
| `BaziComputedMetrics` and domain scores | Measurements and long-horizon profile analysis | Yes, with confidence and provenance | No | Keep separate from target-day decisions; never present a domain score as day suitability |
| `DayPersonMatrix` | Typed raw day-to-pillar interaction signals | Yes as an explanation/detail projection | No | Preserve rows; replace broad membership booleans with approved typed relation facts |
| `ElementResonanceMatrix` | Typed element/day context signals | Conditional | No | Preserve entry-level facts; quarantine `net_resonance` until a personal, versioned aggregate is validated |
| `PersonalHourMatrix` | Twelve raw hour candidates plus an intent-policy ranking projection | Yes when birth time is known | No | Repair slot alignment; move composite score semantics into the canonical policy and expose contributions |
| `DirectionMergeMatrix` | Eight typed direction-signal rows plus an optional intent-policy ranking | Yes when Kua inputs are available | No | Preserve raw signals; treat `net_score` as a count projection, not a universal suitability score |
| `DomainDayBoostMatrix` | Unresolved experimental projection | No by default | No | Quarantine; either redesign with domain/intent-specific evidence or replace with clearly named generic day-quality context |
| Legacy `score_day_selection` / `AdvisoryScoring` | Compatibility projection | Temporarily | No | Project from the canonical assessment during migration, then deprecate the independent formula |
| Reasoning axis scores and semantic classifications | Explanation projections over selected evidence | Yes where useful | No | Consume canonical typed contributions; do not independently count notes or derive verdicts |
| `PersonalDayAssessment` | One normalized, intent-specific decision and evidence envelope | Yes | **Yes** | Build once in core; all numeric and semantic verdict surfaces project from it |

`Matrix` may remain in compatibility type names, but new domain names should
prefer `PolicyTable`, `InteractionSignals`, `Candidates`, or `Assessment` so a
data table is not mistaken for a calibrated decision engine.

## Score taxonomy and canonical contract

Every numeric value must be classified as exactly one of the following:

1. **Measurement** — observed or derived profile/day quantity, such as element
   distribution, interaction count, or evidence coverage. It describes input
   state and must not map directly to a recommendation verdict.
2. **Signal** — a typed favorable, unfavorable, neutral, unavailable, or
   disputed relation with strength and provenance. Signals remain independent
   of presentation language.
3. **Decision score** — an intent-specific policy result. Only this class may
   be thresholded into `recommended`, `consider`, `avoid`, or stronger safety
   verdicts.

Values from different classes, policies, intents, or versions are not
comparable even when they share a 0-100 range. Public field documentation and
DTOs must identify the class; ambiguous fields such as `score`, `net_score`,
and `boosted_score` remain compatibility-only until their semantics are made
explicit.

The canonical assessment must keep at least these axes separate before any
product-level summary is formed:

- generic day quality;
- activity/intent fit;
- personal alignment;
- annual/period pressure;
- evidence coverage and input capability.

Every decision contribution must contain stable, machine-readable metadata:

```text
contribution_id
axis
intent
polarity_or_effect
strength_or_weight
policy_id + policy_version
ruleset_id + ruleset_version
source evidence
availability/data-quality state
```

The assessment may expose a normalized decision score only after applying
hard-veto precedence, deduplication, intent relevance, and missing-data rules.
Confidence describes input/evidence coverage and policy support; it is not a
second name for score magnitude.

## Consumer migration map

```text
normalized day snapshot + normalized birth capabilities
                         |
                         v
        corrected typed interaction/Bazi facts
                         |
                         v
              PersonalDayAssessment
               /       |       |       \
              v        v       v        v
          advisory    API     TUI    reasoning/graph
```

| Consumer | Current behavior | Required migration | Removal/parity gate |
| --- | --- | --- | --- |
| Advisory and ranked dates | Computes an independent 0-100 score | Become a compatibility projection of the assessment and its intent contributions | Same normalized inputs, verdict, score, confidence, and contribution IDs across standalone and aggregate calls |
| Personal-day matrix API | Recomputes chart, snapshot, and five matrix outputs | Return raw-signal/detail projections from the already-built assessment; mark unavailable sections explicitly | No endpoint-local verdict or silent unknown-to-zero conversion; serialization compatibility fixtures pass |
| TUI personal summary | Picks maxima/counts from unrelated numeric fields | Render the canonical verdict first, then explain with hour/direction/raw-signal projections and score labels | TUI cannot imply that a count projection is the canonical score; incomplete-profile fixtures are clear |
| Flat reasoning pipeline | Rebuilds personal facts and interprets summaries/counts | Consume typed canonical facts and contributions for the allowlisted action | Localization, duplicate-fact, and unrelated-node metamorphic tests pass |
| Semantic graph/evaluator | Richer parallel path, primarily test-backed | Represent the same canonical facts/evidence; become an explanation/query substrate rather than another scoring engine | Semantic and flat projections agree before the duplicate production path is retired |

Compatibility fields may remain for one migration window, but they must be
tagged with their legacy policy and derived from, or explicitly compared
against, the canonical assessment. Safety fixes and explicit unavailable states
must not be hidden behind the rollback flag.

## Finding disposition workflow

Before implementation, every audit finding receives one of these dispositions
in its bead or source decision record:

- **Accepted** — behavior and remediation are sufficiently established to
  implement with a failing regression test first.
- **Source verification required** — the code defect or ambiguity is known,
  but the domain rule must be approved from a cited source before fixtures or
  coefficients are frozen. XIANGXING and disputed relation semantics are in
  this class.
- **Quarantined** — a public aggregate or claim is disabled/marked unavailable
  until a valid policy exists. Current `net_resonance` and domain-day boost are
  the initial candidates.
- **Deferred** — safe to leave in place for the current migration, with a
  bounded impact and explicit dependency.
- **Rejected** — no change, with recorded evidence explaining why the finding
  does not apply.

No AFK implementation task may silently choose a tradition, invent scoring
coefficients, or promote a disputed finding from source-verification-required
to accepted. Post-implementation audit findings follow the same workflow.

## User-journey acceptance gates

The migration is not complete merely because matrix shapes and unit tests pass.
The following end-to-end questions must have one consistent, traceable answer:

1. **“Is this date suitable for signing a contract?”** The answer uses the
   contract intent policy, identifies any veto, distinguishes generic day
   quality from personal alignment, and exposes confidence/evidence.
2. **“If I still proceed today, which time and direction are preferable?”**
   Hour and direction refinements use the same assessment, known capabilities,
   and intent; they cannot reverse a hard avoid verdict without explaining the
   distinction.
3. **“Why did the answer change after I supplied my birth time?”** The response
   identifies newly available facts/contributions and changes confidence only
   for the capabilities and evidence actually added.
4. **“What can be said with birth date but no time or gender?”** Available
   generic/profile signals remain useful, while personal-hour, Kua, and annual
   Hạn-dependent claims are explicitly unavailable rather than defaulted.

Each journey requires golden API fixtures plus parity coverage for the
standalone advisory, aggregate report, and whichever TUI/reasoning projection
exposes it.

## Implementation and review cadence

1. **Before implementation:** assign dispositions, resolve source-sensitive
   decisions, freeze the failing regression/golden cases, and confirm the
   affected score or matrix role in the ownership table above.
2. **During implementation:** land the smallest corrected vertical slice,
   preserve raw evidence, and compare legacy and canonical projections in
   shadow/parity tests. A local fix must not introduce a new endpoint-owned
   verdict.
3. **Before switching consumers:** pass capability, serialization, user-journey,
   and cross-surface parity gates. Record intentional legacy/canonical diffs and
   their policy versions.
4. **After implementation:** repeat the focused audit against actual data flow,
   classify any new finding through the same disposition workflow, and remove
   compatibility paths only when their deletion tests and rollback conditions
   are satisfied.

The post-implementation audit verifies the plan; it is not the point at which
ownership, score semantics, or source-sensitive rules are first decided.

## Ordered work packages

### P0 — quarantine unsafe semantics and establish canonical inputs

**P0.1 Birth profile/time model**

- Areas: `crates/amlich-core/src/advisory.rs` (`BirthInput`), `crates/amlich-core/src/bazi/types.rs`, `bazi/chart.rs`, `crates/amlich-api/src/lib.rs` tier helpers and query adapters.
- Introduce one core birth profile/capability model carrying date, explicit time-known state (including real 00:00), timezone, longitude/solar-time policy, gender, and location metadata.
- Remove `unwrap_or(0)` conversion in `reasoning/personal.rs:161-172`; gate hour pillar/personal-hour facts on known time.
- Replace duplicated `bazi_birth_data_tier`, `personal_birth_data_tier`, and `matrix_birth_data_tier` with projections from the core capability resolver.
- Acceptance: unknown, 00:00, 00:01, date+gender, and full solar-time profiles have distinct golden outputs; missing capabilities are explicit.

**P0.2 Quarantine false aggregates**

- Area: `crates/amlich-core/src/interaction/element_resonance.rs` and callers.
- Remove/quarantine `net_resonance` until it is weighted by normalized personal element distribution, or version/document a valid aggregate. Preserve entry-level signals.
- Fix `compute_han_count`/domain boost path so missing gender/Hạn returns unavailable, not `0`.
- Acceptance: contrasting distributions produce different aggregate values after policy implementation; date-only/no-gender output omits or marks unavailable annual-Hạn boost.

**P0.3 Make advisory non-canonical**

- Area: `crates/amlich-core/src/advisory.rs:276-371` and `amlich-api` personal-day report.
- Mark legacy `score_day_selection` as compatibility projection/quarantine its user-facing confidence. Do not claim medium confidence merely because `BirthInput` exists.
- Acceptance: API does not expose conflicting canonical numeric/semantic verdicts; confidence derives from capability/evidence coverage.

### P1 — build the shared assessment and repair domain signals

**P1.1 `PersonalDayAssessment` seam**

- Areas: new core assessment module; `reasoning/synthesis.rs`, `reasoning/personal.rs`, `reasoning/initiation_opening_evaluator.rs`, `reasoning/graph_projection.rs`, `advisory.rs`, `amlich-api/src/lib.rs` aggregate report.
- Build snapshot, normalized profile, chart/analysis (once), raw interaction facts, intent policy contributions, availability, evidence, and confidence in one object. Make existing DTOs/advisory/reasoning graph thin projections.
- Acceptance: standalone and aggregate endpoint parity tests serialize identical normalized inputs, ruleset, score contributions, confidence, and unavailable sections.

**P1.2 Typed relation and action policy**

- Areas: `almanac/xung_hop.rs`, `interaction/day_person.rs`, `reasoning/initiation_opening_evaluator.rs`, semantic graph node types.
- Replace membership booleans with explicit direct pair, triad membership, completed group, and self-punishment relations. Correct/audit XIANGXING with cited ruleset before freezing fixtures.
- Implement action-specific `select_subgraph` (currently clones full graph). Replace summary-string parsing for Xung/Hợp, stars, directions, and Trực with typed fields.
- Acceptance: same branch is not automatically Tam Hợp; incomplete triads are not completed punishments; unrelated graph node insertion leaves opening decision unchanged; localization does not alter outcomes.

**P1.3 Personal-hour and domain correctness**

- Areas: `interaction/personal_hour.rs`, `interaction/domain_day_boost.rs`, related types/tests.
- Generate slot 0 as Tý (23:00–01:00) and align each row's Can Chi/star index; define a versioned score policy matching documented weights and precedence.
- Pass canonical Can Chi into domain boost (`day_canchi` must not be element names). Replace one global domain multiplier with explicit domain/intent policy or rename it as global day quality.
- Acceptance: all 12 slot parity fixtures, 23:00/00:00/01:00 boundaries, exact score contribution tests, real “Bính Thân”-style serialization, and domain-isolation fixtures pass.

**P1.4 Reasoning evidence/confidence**

- Areas: `reasoning/types.rs`, `action_evaluator.rs`, `initiation_opening_evaluator.rs`, `graph_projection.rs`.
- Replace raw note counts/arbitrary caps with typed contribution strengths and policy weights; distinguish hard/soft taboo and source quality. Populate referenced edge IDs or remove the unused contract. Avoid star false positives and duplicate extraction scans.
- Acceptance: high confidence requires qualified evidence/coverage; duplicate facts do not change result; hard-vs-soft and missing-profile metamorphic tests pass; every strongest note resolves to provenance/edge evidence.

### P2 — consolidation, documentation, and cleanup

- Cache/reuse normalized chart, matrices, and assessment across graph/advisory/API projections; eliminate repeated endpoint recomputation (`amlich-api/src/lib.rs:1242-1268`, matrix report path).
- Remove overloaded numeric severities (hour count/Kua number) and use typed metadata fields.
- Add policy/ruleset/source provenance to matrix and score contributions; document confidence, availability, limitations, and safe-use disclaimers in DTO/core docs.
- Add invalid branch-index tests and make lookup failures explicit instead of self-fallback.
- Acceptance: benchmark shows no duplicate chart/snapshot builds per request; schema/docs describe all availability states and policy versions.

## Dependencies and migration seams

1. Birth profile/time capability must land before assessment confidence or personal-hour gating.
2. Typed branch/star/direction facts must land before action evaluator and graph projections stop parsing summaries.
3. Correct personal-hour/domain signals and `net_resonance` policy must land before they are admitted as assessment contributions.
4. Assessment builder then becomes the seam for advisory, reasoning, and API adapters; legacy structs remain boundary-compatible until parity gates pass.
5. Only after parity and golden fixtures pass should old tier helpers, duplicate scoring, and dead edge contracts be removed.

## Compatibility and rollback

- Keep legacy `BirthInput`, `BaziInput`, DTO field names, and advisory function as adapters with an explicit compatibility/policy version.
- Accept legacy `00:00` payloads under a deserialization compatibility mode, but emit new explicit time-known fields; never silently reinterpret a new real-midnight value.
- Feature-flag the new assessment projection and retain old serialized fields during one migration window. Compare old/new outputs in shadow mode and log contribution diffs without changing user output.
- Roll back by switching the projection flag to legacy adapters; do not roll back typed validation or unknown-data safety fixes.

## Final quality gates

- `cargo fmt --check`, `cargo test -p amlich-core`, `cargo test -p amlich-api`, and workspace build/lint gates.
- Golden fixtures for branch relations, personal-hour slot/star alignment, real Can Chi serialization, midnight/unknown time, missing gender/Hạn, and contrasting element distributions.
- Metamorphic tests: duplicate evidence, localization changes, unrelated graph node insertion, date-only vs full profile, and standalone-vs-aggregate parity.
- Serialization/API contract tests verify explicit unavailable states and stable policy/ruleset/evidence IDs.
- Review source citations/ruleset versions for XIANGXING and scoring coefficients before enabling user-facing high-confidence conclusions.
