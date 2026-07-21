# Interaction / Almanac audit

Scope: `crates/amlich-core/src/interaction/*`, `almanac/xung_hop.rs`, and the
associated unit tests. This is a read-only architecture/correctness review.

## Rubric used

- **Blocker**: produces materially false advice or makes a canonical result
  unusable; must fix or quarantine before exposing as a personal-day score.
- **Major**: deterministic domain/cross-module defect, missing contract, or
  untestable policy that can change user recommendations.
- **Minor**: naming, provenance, robustness, or test-gap issue with bounded
  impact.

Each finding includes evidence, a deletion test (what can be removed or
disabled to prove the dependency is not silently required), remediation,
priority, and confidence/limits.

## Findings

### Blocker — `net_resonance` is not personal

Evidence: `interaction/element_resonance.rs:19-41` computes
`effective_resonance = relation * season_factor` for every element, then sums
the five coefficients. `personal_score` is only used for the deficit boolean
at lines 24, 27, and never weights the aggregate. For any two people with the
same day stem and month branch, `net_resonance` is identical even when their
element distributions are opposite. The existing test at lines 214-220 only
asserts that the field equals its own entry sum, so it codifies this behavior.

Impact: a consumer can display a positive/negative “personal” resonance that
contains no personal signal; this is a false confidence risk in a day score.

Remediation: either remove/quarantine `net_resonance` from user-facing
contracts, or define and version an aggregate that weights each relation by a
normalized chart distribution (and documents deficit/surplus treatment). Add a
golden test with two contrasting `ElementDistribution`s and assert different
aggregates; retain an entry-level test independent of the aggregate.

Deletion test: delete the `personal_score` field from `ElementResonanceEntry`
and all existing aggregate tests should still pass, demonstrating that the
current aggregate does not depend on it. This test must fail after the fix.

Priority: P0. Confidence: high (direct data-flow inspection; no external
ruleset source was required).

### Major — season factor is applied to the wrong quantity

Evidence: `interaction/element_resonance.rs:19-34` computes one seasonal
strength for the **day element**, then applies it to every target element.
The comment at lines 84-85 describes strength “of `element` during the month”,
but target rows do not receive their own seasonal strength. This makes a day
element's seasonal strength scale both supportive and controlling relations,
regardless of the target's seasonal state.

Remediation: make the policy explicit: compute `season_strength(target,
month)` per row when evaluating target support, or rename the field to
`day_element_season_factor` and keep it only as day-energy context. Add a
five-element golden matrix for at least spring/autumn and verify each row's
season provenance.

Deletion test: replace `season_factor` with a constant in a temporary test
fixture; current tests still pass except the two day-element season tests,
showing no row-level season contract exists.

Priority: P1. Confidence: medium (depends on intended scoring tradition, but
the implementation/comment mismatch is certain).

### Major — branch-punishment lookup encodes incorrect classical groups

Evidence: `almanac/xung_hop.rs:131-138` defines `XIANGXING` as
`[寅,卯,巳]`, `[子,辰,丑]`, `[申,酉,亥]`, and `[午,午,午]`. Standard
three-punishment groups are conventionally 寅巳申 and 丑未戌, with 子卯 as a
separate two-branch punishment and self-punishment for 辰/午/酉/亥. The current
table therefore marks unrelated pairs (for example 寅–卯) as punishment and
misses canonical pairs (for example 寅–申 and 丑–未).

Impact: `compute_branch_relation` consumes this table and feeds both day/person
matrices and personal-hour scores, so wrong punishment flags can lower or
raise recommendations.

Remediation: represent punishment as typed pair/group rules rather than a
fixed `[[usize;3];4]`; distinguish pair relation, completed triad, and
self-punishment. Add a source/ruleset identifier and golden fixtures for every
canonical pair/group. Do not infer a pair solely from “membership in a group”
when completion requires three occurrences.

Deletion test: remove `XIANGXING` and run relation tests; currently no test
asserts canonical 寅–申/丑–未 or rejects 寅–卯, so the suite remains green. Add
those tests before changing callers.

Priority: P0. Confidence: high for code behavior; exact tradition should be
confirmed against the project's cited source before freezing fixtures.

### Major — `tam_hop` and `tuong_hinh` are membership flags, not pair semantics

Evidence: `interaction/day_person.rs:75-85` sets `tam_hop` and `tuong_hinh`
when the pillar branch appears in the day branch's returned group. Since
`tam_hop()` always includes its input (`xung_hop.rs:31-38`), a branch compared
with itself is flagged as tam-hop. `get_xiang_xing()` returns a whole group and
the caller marks any member as punishment, including the self branch and
partial groups.

Impact: same-branch day/pillar can simultaneously look harmonious and (for
bad table groups) punitive; score code then adds/subtracts multiple unrelated
weights.

Remediation: expose explicit relation enums (`DirectPair`, `TriadMember`,
`CompletedGroup`, `SelfPunishment`) or return a relation set with cardinality
and evidence. Define whether a two-person comparison can claim a triad.
Update `BranchRelation::is_neutral/has_conflict` to consume typed relations.

Deletion test: assert a same-branch comparison has no `tam_hop`; assert a
two-member input cannot claim a completed three-branch punishment. These
tests currently do not exist.

Priority: P1. Confidence: high (direct membership logic).

### Major — personal-hour matrix shifts every hour relative to the star table

Evidence: `interaction/personal_hour.rs:47-55` iterates `slot = 0..11`, but
calls `compute_hour_pillar(..., (slot * 2 + 1), ...)`. `resolve_hour_branch_slot`
maps 01:00 to slot 1 (Sửu), while slot 0 should be Tý (23:00–01:00). The
generated rows are therefore rotated (Sửu..Hợi,Tý), while
`hoang_dao.all_hours[slot]` is indexed using the unshifted slot, pairing each
star with a different Can Chi.
The coverage test only checks set membership and therefore misses ordering and
star/Can-Chi alignment.

Remediation: generate each branch by slot directly (a dedicated
`compute_hour_pillar_for_slot(day_stem, slot)`), or call 23:00 for slot 0 and
the correct representative minute for slots 1..11. Assert `hours[i].chi_index
== i` and that `star_name/is_hoang_dao` came from the same index. Add boundary
goldens at Tý and Hợi.

Deletion test: remove the `hoang_dao.all_hours[slot]` indexing and current tests
still pass, proving no test verifies star/row correspondence. Add a parity test
that should fail under the current implementation.

Priority: P0. Confidence: high (deterministic mapping; confirmed by
`almanac/hour_pillar.rs:30-42`).

### Major — personal-hour score policy is hard-coded and double-counts relation

Evidence: `interaction/personal_hour.rs:95-151` documents weights totaling
30/30/25/15, but implementation starts at 50 then adds +20/-10, up to +15
Thập Thần, five independent branch adjustments, and +15 weak-element support.
The actual range and relative weights do not match the documented 30/30/25/15
model; harmony and conflict booleans can also co-exist and both affect score.

Remediation: define a versioned scoring policy with named contributions,
mutual exclusivity/precedence, and evidence per contribution. Keep raw signals
separate from the ranking projection; test exact expected scores for each
relation and boundary.

Deletion test: delete any one documented weight line and verify a policy test
fails. Existing tests only check ordering/range, so they would not detect drift.

Priority: P1. Confidence: high for implementation/documentation mismatch.

### Major — domain-day boost applies one global multiplier to all domains

Evidence: `interaction/domain_day_boost.rs:15-46` computes one
`day_modifier` from star/trực/thần and applies it identically to career,
wealth, relationship, health, and timing. No domain mapping or activity intent
is consulted. `han_active_count` is a bare count, losing which domain/year
rule caused the penalty. A positive star count can inflate health as much as
career even when the star has no stated domain relevance.

Remediation: define per-domain contribution tables (or explicitly call this a
global day-quality projection), pass structured Hạn evidence, and return the
contribution list plus confidence. Add fixtures where a career-specific signal
changes career only and where missing Hạn data is distinguishable from zero.

Deletion test: supply identical `DayFortune` values and vary only the domain
base scores; current tests prove all outputs simply scale, not that domains
respond to domain-specific evidence. A domain-isolation test should fail until
policy is explicit.

Priority: P1. Confidence: high.

### Major — `day_canchi` in domain boost is not Can Chi

Evidence: `interaction/domain_day_boost.rs:51-54` formats
`day_fortune.day_element.can_element` + `chi_element` (element names such as
"Mộc Thổ"), while `DomainDayBoostMatrix.day_canchi` is documented as the day's
Can Chi (`interaction/types.rs:268-273`). `DayFortune` does not carry the full
Can Chi in this function, so the field is semantically corrupted.

Remediation: pass the canonical `CanChi` (or full day label) into the function,
or rename the field to `day_element_context`. Add serialization golden test
expecting a real label such as “Bính Thân”.

Deletion test: remove `day_canchi` from the JSON assertion; current tests only
check that the key exists, not its value/type semantics.

Priority: P1. Confidence: high.

### Minor — branch APIs accept unchecked indices and silently self-fallback

Evidence: `xung_hop.rs:20-22,117-126` indexes `CHI` directly and returns the
input branch itself as a “no partner” fallback. `compute_branch_relation`
similarly indexes `CHI[pillar_chi]` without validation. Invalid external input
can panic; a missing rule becomes a false self relation.

Remediation: use a `Branch` enum/newtype or return `Result`; make missing
partners explicit (`Option`). Add invalid-index tests and ensure adapters map
errors to a data-quality state, not a neutral/harmony signal.

Deletion test: pass index 12 in a boundary test; current code panics. Pass a
partner lookup for an intentionally incomplete table and assert it does not
return the same branch.

Priority: P2. Confidence: high.

### Minor — provenance is too coarse for rule matrices

Evidence: all reviewed matrices use `SOURCE_KHCBPPT` with `profile: "baseline"`
and method names only (`day_person.rs:22-29`, `element_resonance.rs:50-54`,
`personal_hour.rs:87-91`, `domain_day_boost.rs:56-60`). The hard-coded scoring
coefficients and punishment tables have no rule version, citation, or per-row
evidence.

Remediation: include ruleset/version and contribution provenance in matrix
outputs; preserve source IDs through adapters. Add a contract test that every
user-visible score has at least one evidence record.

Deletion test: remove `evidence` from a matrix and current unit tests still
mostly pass (only method equality checks exist), showing provenance is not a
behavioral contract.

Priority: P2. Confidence: high.

## Architectural seams / consolidation

The interaction modules expose raw matrices while `advisory.rs` independently
implements day and hour scores. There is no shared policy object, so branch
relations and quality weights can drift between `compute_personal_hour_matrix`,
`score_day_selection`, and transport/semantic-graph projections. Introduce a
versioned `PersonalDayAssessment` policy layer that consumes these raw signals,
keeps missing-data states distinct from neutral values, and emits score
contributions plus evidence. Keep these matrix functions as deterministic
signal builders; they should not each invent user-facing semantics.

## Test plan for the parent implementation plan

1. Add branch golden fixtures for all xung/hop/hai/xing pairs, including same
   branch and incomplete triads.
2. Add personal-hour index/star/Can-Chi parity fixtures for all 12 slots and
   23:00/00:00/01:00 boundaries.
3. Add contrasting element distributions and a policy-level aggregate test.
4. Add domain-isolation and real-Can-Chi serialization fixtures.
5. Add invalid-index and missing-Hạn-data tests.
6. Add cross-surface parity tests ensuring advisory, matrix, and semantic graph
   consume one scoring policy/version.
