# Advisory / reasoning audit

Scope: `crates/amlich-core/src/advisory.rs` and `crates/amlich-core/src/reasoning/{personal.rs,synthesis.rs,initiation_opening_evaluator.rs,types.rs,action_evaluator.rs,graph_projection.rs}` plus the reasoning graph tests. This is a read-only audit; no source changes were made.

## Resolution addendum (2026-08-09)

This audit is retained as historical evidence. The `amlich-mwbp` repair closed
its findings as follows:

| Finding | Resolution |
| --- | --- |
| A-R01–A-R03 | Canonical assessment and explicit birth capabilities replaced the independent advisory/midnight-fallback paths. |
| A-R04 | Initiation/opening evaluates an explicit concept allowlist. |
| A-R05–A-R06 | Verdicts and normalized axes come from typed canonical contributions and capability/evidence coverage; graph note counts no longer synthesize decisions. |
| A-R07 | `SemanticFact` replaces localized-summary and serialized-tag parsing for Trực, stars, Xung/Hợp, and directions. |
| A-R08 | The dead referenced-edge contract was removed; node/edge evidence remains on exported graph records. |
| A-R09 | Typed star polarity prevents negative 28-star facts from becoming support; duplicate graph facts cannot alter canonical scores. |
| A-R10 | Numeric severity overloads were removed; personal effects and direction scores are typed fields. |
| A-R11 | `PersonalAssessmentFacts` and request contexts reuse chart, matrix, snapshot, and assessment builds. |
| A-R12 | Policy/ruleset/source/contribution metadata is exposed through `PersonalDayAssessment` and its DTO projection. |

The localization, duplicate-evidence, action-isolation, provenance, severity,
standalone-profile, core/API parity, and serialization tests are the deletion
gates for these resolutions.

## Rubric used

- **Architecture/depth/seams:** one canonical domain pipeline, explicit boundaries, action-specific inputs, no duplicated decision logic.
- **Correctness:** typed facts instead of localized string parsing, missing-data semantics, deterministic and explainable rules, no false precision.
- **Verification:** tests for invariants, negative/unknown cases, parity between projections, and golden/source provenance.
- **Documentation:** public score/confidence semantics and rule/version provenance are documented.
- **UX/product risk:** avoid presenting a heuristic or incomplete profile as a personalised/high-confidence verdict; recommendations must be traceable to evidence.

Severity means Blocker (unsafe canonical result), Major (materially misleading or architectural drift), Minor (local quality/maintainability). “Deletion test” states what would fail or disappear if the code were removed, to distinguish essential behavior from accidental complexity.

## Findings

### A-R01 — Major: `score_day_selection` is a parallel, shallow decision engine

Evidence: `advisory.rs:276-371` starts at 50, applies fixed bucket/taboo/chi/Kua deltas, and emits its own verdict; the graph-backed evaluator lives separately in `reasoning/initiation_opening_evaluator.rs:591-689`. The advisory never consumes `PersonalReasoningInput`, Bazi matrices, axis scores, or semantic graph evidence.

Impact: the same date/profile can produce a numeric `weak_match` while the reasoning pipeline produces a favorable/cautious semantic result. The numeric result is likely to be treated as canonical by API/UI even though it only uses year chi and optional gender.

Deletion test: delete `score_day_selection`; `build_personalized_day_selection` and all numeric/ranked-date consumers lose their only score, while graph reasoning still works. This proves it is a separate product path, not a projection.

Remediation: make a single `PersonalDayAssessment` own facts, contributions, confidence and evidence. Project `AdvisoryScoring`, semantic graph and UI DTOs from it; retain the old function only as a compatibility adapter during migration. Add parity tests over representative profiles/intents.

Priority: P0 before exposing more personalised scoring. Confidence: high.

### A-R02 — Major: personalised score claims more data than it uses

Evidence: `advisory.rs:382-432` derives only lunar birth **year** chi, compares it to day chi, and optionally Kua; month/day/hour, Bazi, element resonance and intent-specific personal matrices do not affect the score. `score_day_selection` labels any supplied birth as `confidence = medium` (`advisory.rs:347-355`).

Impact: two people with the same birth year receive identical “personalized” scores, and a date-only birth gets the same confidence as a fully timed profile. This is false precision and can mislead high-stakes decisions (medical, burial, contracts).

Deletion test: replace two `BirthInput`s with same year but different month/day/hour; score remains identical unless gender changes Kua. Remove hour/minute entirely and confidence remains medium.

Remediation: introduce validated `BirthProfile`/data tier once in core; make every contribution declare required fields and return `unknown/not-computable` when absent. Separate generic day quality, intent fit, personal alignment and data completeness; derive confidence from evidence coverage, not `birth.is_some()`.

Priority: P0. Confidence: high.

### A-R03 — Blocker: missing birth hour is silently converted to midnight Bazi

Evidence: `reasoning/personal.rs:161-172` maps `BirthInput.hour/minute` with `unwrap_or(0)` into required `BaziInput`. `build_fact_nodes` then computes Bazi and personal-hour matrices (`personal.rs:35-62`), even when the user never supplied a time.

Impact: unknown birth time becomes a real 00:00 chart, contaminating day-person, personal-hour and element conclusions. This is especially dangerous because the output has normal-looking summaries and evidence.

Deletion test: construct identical profiles with `hour=None` and `hour=Some(0)`; they collapse to the same chart and recommendations, proving unknown and midnight are not distinguishable.

Remediation: represent time as an explicit state (`Unknown`, `Known { hour, minute }`) or make Bazi input optional. Gate hour-pillar/personal-hour claims on complete time; expose a `data_tier` and warnings in reasoning output. Add unknown-vs-midnight regression tests.

Priority: P0. Confidence: high.

### A-R04 — Major: action evaluator does not select an action-specific subgraph

Evidence: `initiation_opening_evaluator.rs:582-589` returns `Ok(graph.clone())` from `select_subgraph`.

Impact: initiation/opening scoring can see unrelated facts (all taboos, directions, stars, interactions, and any future graph nodes). Adding a new graph node can silently change this action. The API advertises a seam that is not implemented.

Deletion test: remove `select_subgraph`; current behavior is unchanged because callers effectively evaluate the full graph anyway.

Remediation: define an allowlist/edge closure for `ActionId::InitiationOpening`, include only opening-relevant facts and personal inputs, and test that unrelated node insertion does not alter the decision.

Priority: P1. Confidence: high.

### A-R05 — Major: axis scores are raw counts with arbitrary caps, not calibrated signals

Evidence: `initiation_opening_evaluator.rs:301-369` counts support/resistance notes; stability is `3 - taboo_count`; personal alignment is `1.0` whenever input exists (`:339-346`); timing is capped at 3 hoàng-đạo hours (`:348-359`). These axes are not normalized, weighted, or tied to intent.

Impact: one weak star note equals one hard resistance note; adding duplicate evidence changes scores; merely supplying a profile creates “alignment.” Confidence and bucket are then derived from these counts (`:390-435`).

Deletion test: duplicate one graph fact and support/resistance score changes; swap a complete profile for a minimally valid profile and personal alignment remains 1.0.

Remediation: define typed contribution strengths/severity and per-axis policy (including hard override precedence), normalize scores, include source coverage, and keep raw evidence separate from decision weights. Add metamorphic tests for duplicate facts, hard-vs-soft taboo, and missing profile.

Priority: P1. Confidence: high.

### A-R06 — Major: hard taboo automatically yields high confidence without source/data qualification

Evidence: `synthesize_semantic` assigns `DecisionConfidence::High` to both `OverrideAvoid` and `OverrideCautious` (`initiation_opening_evaluator.rs:422-427`) solely from semantic category. Hard-taboo extraction only checks `severity == "hard"` (`:152-174`); no provenance quality or input completeness enters confidence.

Impact: a single graph severity string can produce a high-confidence “avoid” recommendation, including when the underlying source is heuristic, profile is incomplete, or action relevance is not established.

Deletion test: change only provenance/source family or remove personal data; confidence remains High as long as the hard string remains.

Remediation: model confidence as evidence quality × coverage × agreement; distinguish normative hard rules from advisory signals; require source/provenance and action relevance before high confidence. Add tests for hard taboo with missing/low-quality evidence.

Priority: P1. Confidence: high.

### A-R07 — Major: evaluator relies on brittle Vietnamese summary-string parsing

Evidence: `initiation_opening_evaluator.rs:102-105`, `:191-196`, `:252-268`, `:522-566` detect xung/hợp, direction rows, star quality, and Trực by `summary_vi.contains(...)`, prefixes, and serialized tag strings.

Impact: wording/localization changes, punctuation, or translation can change domain decisions; summary text becomes an undocumented wire protocol. It also makes semantic graph nodes hard to evolve safely.

Deletion test: alter “Xung...” capitalization or localize a summary; resistance/conflict evidence disappears while underlying fact is unchanged.

Remediation: expose typed fields/concepts (relation enum, star polarity, direction score, Trực activity hits) on `SemanticNode` or a typed fact view. Keep summaries presentation-only. Add localization-invariance tests.

Priority: P1. Confidence: high.

### A-R08 — Major: graph provenance is incomplete; referenced edges are always empty

Evidence: `ActionEvaluation` includes `referenced_edge_ids` (`action_evaluator.rs:22-27`), but evaluator returns `Vec::new()` (`initiation_opening_evaluator.rs:662-689`). `synthesis.rs:21-30` projects graph and decision independently.

Impact: consumers cannot trace a conclusion to the causal edges/weights that produced it; evidence lists facts but not the actual support/override path. This undermines explainability and parity debugging.

Deletion test: remove `referenced_edge_ids` and no current test/output changes; the field is dead contract surface.

Remediation: have evaluation retain selected edge IDs while traversing the action subgraph, or remove the field until implemented. Add contract tests requiring every strongest note and override to resolve to graph provenance/edges.

Priority: P1. Confidence: high.

### A-R09 — Major: support/resistance extraction has false-positive and double-count paths

Evidence: `extract_support_evidence` treats any star whose summary contains “Nhị thập bát tú” or starts “Ngôi sao chính:” as supportive (`initiation_opening_evaluator.rs:66-79`, `:544-550`), regardless of polarity. `score_axis` calls extraction twice (`:303-313`, `:316-325`), and conflict extraction recomputes related scans (`:201-230`, `:361-368`).

Impact: an inauspicious 28-star can become support; repeated O(n) scans invite drift and make future side effects expensive. Counts may disagree if graph construction becomes non-pure.

Deletion test: feed a star summary containing “Nhị thập bát tú” with negative quality; support still increments. Instrument extraction and observe duplicate calls per evaluation.

Remediation: use typed star polarity/severity; compute an `EvidenceIndex` once per evaluation and pass slices to axis/conflict synthesis. Add positive/negative star fixtures and extraction-call/parity tests.

Priority: P1. Confidence: medium-high (depends on graph summary fixtures).

### A-R10 — Minor: personal facts carry numeric severity fields with unrelated meanings

Evidence: `personal.rs:51-58` stores number of hours as severity; `:73-80` stores Kua number as severity. `graph_projection` then maps these through generic `severity_for_node` into `ReasoningNodeSeverity`.

Impact: consumers may interpret “severity” as auspiciousness when it is a count or identifier; schema is semantically overloaded and difficult to validate.

Deletion test: changing Kua from 1 to 8 changes a field named severity without changing polarity; generic severity readers cannot distinguish it.

Remediation: use typed metadata (`hour_count`, `kua_number`) and reserve severity for polarity/strength. Update export schema with explicit optional fields.

Priority: P2. Confidence: high.

### A-R11 — Minor: personal helper recomputes expensive chart/matrices across projections

Evidence: `build_fact_nodes` builds chart and distributions (`personal.rs:35-37`); `suggested_hours` rebuilds chart/distribution (`:109-115`); `suggested_directions` recomputes Kua/merge (`:134-145`). `graph_projection` invokes `build_fact_nodes`, while evaluator separately asks suggested hours/directions.

Impact: duplicated computation can drift and increases latency; outputs can disagree if a rule changes between calls.

Deletion test: memoize a chart and output remains same; current code has no observable need for repeated calculation.

Remediation: construct a `PersonalAssessmentFacts` once per snapshot/profile and pass it to graph/evaluator/projections. Add parity test asserting suggestions equal the fact-node source.

Priority: P2. Confidence: high.

### A-R12 — Minor: public score/evidence semantics are under-documented and rule method is phase placeholder

Evidence: `advisory.rs:363-369` emits method `phase0_phase1_foundation`, while score fields expose no contribution breakdown or policy version. `AdvisoryScoring` and `ReasoningAxisScore` have no module-level contract docs.

Impact: downstream clients cannot explain score changes or know whether “confidence” is epistemic, data completeness, or rule certainty; UI may overstate the result.

Deletion test: remove the method string and no computation changes; it is metadata without a documented contract.

Remediation: publish a policy schema/version, contribution list, data-tier semantics, and safe-use disclaimer; include machine-readable reason IDs and per-contribution evidence.

Priority: P2. Confidence: high.

## Verification assessment

There is substantial graph parity/canonical coverage (`crates/amlich-core/tests/reasoning_graph_*.rs`), but current tests mostly lock output shape and parity between existing paths. They do not cover unknown-hour vs midnight, localized-summary invariance, action subgraph isolation, duplicate evidence monotonicity, typed star polarity, confidence under incomplete provenance, or advisory-vs-graph semantic parity. Add those as contract/metamorphic tests before refactoring.

## Ordered remediation plan

1. **P0 safety:** introduce explicit birth data tiers/unknown time; stop midnight fallback; make confidence reflect coverage; quarantine or relabel numeric advisory as non-canonical.
2. **P0 architecture:** implement one `PersonalDayAssessment` facts/contributions/evidence model and project both advisory and reasoning outputs from it.
3. **P1 correctness:** typed semantic facts (relations, star polarity, direction rows, Trực hits); action-specific subgraph selection; hard-taboo override policy with provenance-aware confidence.
4. **P1 explainability:** populate referenced edge IDs or remove the dead field; preserve contribution IDs and source versions end-to-end.
5. **P2 quality:** cache per-request personal facts; replace overloaded severities; document policy/score/confidence semantics and add the missing regression/metamorphic tests.
