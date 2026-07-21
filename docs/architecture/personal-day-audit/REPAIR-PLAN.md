# Personal-day core repair plan (draft)

This plan synthesizes the advisory/reasoning, birth/API, and interaction/almanac audits. It is intentionally a migration plan, not a source patch.

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

