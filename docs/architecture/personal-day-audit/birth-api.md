# Birth profile / Bazi / API transport audit

Scope: `crates/amlich-core/src/bazi`, `BirthInput` and advisory profile handling,
and `crates/amlich-api` Bazi/personal-day DTO and adapter code. This is a
read-only review; no source behavior was changed.

Rubric used: **Blocker** = incorrect or misleading user-facing result or data
loss; **Major** = contract/architecture defect likely to produce inconsistent
results or prevent an important feature; **Minor** = localized correctness,
maintainability, or documentation issue. Each item includes an evidence-backed
deletion test (what would disappear if the code were removed) and a proposed
remediation. Confidence is high for direct control-flow findings; rule semantics
were not independently validated against an external Bazi reference dataset.

## Findings

### Major — birth hour uses `00:00` as an undocumented “unknown” sentinel

Evidence: `BaziInput.hour` and `.minute` are required scalar values rather than
optional fields (`crates/amlich-core/src/bazi/types.rs:161-175`). The chart
builder suppresses the hour pillar whenever both equal zero
(`crates/amlich-core/src/bazi/chart.rs:21-28`). The API repeats the same test in
`query_has_birth_time` (`crates/amlich-api/src/lib.rs:36-38`) and labels this
input `Date` (`:40-45`). Thus a real birth at midnight cannot be represented,
and the transport silently changes a known time into an unknown-time profile.

Deletion test: remove the `hour == 0 && minute == 0` branch and replace the
fields with an explicit `Option<NaiveTime>`/presence flag; a midnight birth
should then produce an hour pillar while a missing hour remains unavailable.

Remediation: introduce a shared `BirthTime { hour: Option<u8>, minute:
Option<u8>, known: bool }` (or optional fields directly in `BaziInput`), validate
the pair atomically, and have one core `BirthDataTier` derive `Date` versus
`Datetime`. Preserve a compatibility deserializer for legacy zero values, but
do not emit the sentinel in new DTOs. Add golden tests for unknown, 00:00, and
00:01.

### Major — data-tier semantics are duplicated and disagree with availability

Evidence: API has separate `bazi_birth_data_tier` (`lib.rs:40-45`),
`personal_birth_data_tier` (`:795-803`), and `matrix_birth_data_tier
`(:805-810`). Personal-day tier requires date **and gender**; Bazi/matrix tier
only examines whether a non-zero time was sent. `personal_day_unavailable_sections`
marks every analysis section unavailable for `Anonymous`, but the reasoning
input can still be built whenever the three date fields exist, regardless of
gender (`lib.rs:866-891`). This gives callers a tier saying “anonymous” while
returning a non-empty reasoning decision and can make identical birth dates
change availability merely by adding gender.

Deletion test: delete the local tier helpers and route all endpoints through a
single core profile resolver; tests should assert the same tier and required
fields for Bazi, personal-day, and matrix responses.

Remediation: make `BirthProfile`/`BirthDataTier` a core value with independent
capabilities (date, four pillars, hour pillar, Kua/gender, annual-han). DTOs
should project capabilities rather than infer them. Distinguish “date known,
gender missing” from anonymous input and list only the actually missing
capability in `unavailable_sections`.

### Major — missing gender is treated as a numeric zero rather than unavailable

Evidence: matrix direction merge is correctly optional on `chart.input.gender`
(`crates/amlich-api/src/lib.rs:1539-1548`), but domain-day boost is always
returned (`:1550-1557`). Its `compute_han_count` helper returns `0` when gender
is absent (`:1575` onward), so the boost receives a plausible-looking zero
instead of an unavailable value. The resulting DTO advertises `domain_day_boost`
even though annual-han input was not computed.

Deletion test: pass a date-only Bazi query with no gender; the result should
either omit the boost or include an explicit `Unavailable` state, never a
normal matrix whose han contribution is zero.

Remediation: change `compute_han_count` to `Result<Option<u8>, Availability>`
and make `domain_day_boost` optional (or carry an availability/reason field).
Add a contract test asserting no zero-value substitution for missing gender.

### Major — endpoint report recomputes the same profile and day repeatedly

Evidence: `get_personal_day_report` calls advisory, reasoning, chart, analysis,
and metrics separately (`crates/amlich-api/src/lib.rs:1242-1268`), while each
path calls `get_day_insight_with_profile` and/or rebuilds a reasoning bundle.
The matrix endpoint independently builds a Bazi report, analyzes the chart, and
recomputes day context (`:1503-1512`). This creates multiple seams where tier,
timezone, ruleset, and profile normalization can drift and needlessly repeats
expensive lunar/Bazi work.

Deletion test: build one `PersonalDayAssessment` containing normalized profile,
snapshot, chart, analysis, capabilities, and evidence; all endpoint fields
should be projections from that object and produce identical serialized
sub-results.

Remediation: add a core assessment builder with explicit inputs (date,
timezone/ruleset, birth profile, intent) and memoize chart/snapshot within the
request. Keep API functions as thin DTO adapters. Add parity tests comparing
fields returned by standalone endpoints versus the aggregate report.

### Major — `BirthInput` and `BaziInput` are parallel, lossy contracts

Evidence: advisory `BirthInput` has optional hour/minute and `location_name`
(`crates/amlich-core/src/advisory.rs:62-76`), while Bazi `BaziInput` requires
scalar hour/minute and carries longitude/solar-time (`bazi/types.rs:161-175`).
`personal_reasoning_input` constructs the advisory type with `hour: None` and
does not pass longitude or solar-time (`amlich-api/src/lib.rs:866-891`). A user
can therefore submit a solar-time-aware Bazi query but receive a personal-day
reasoning path calculated with default Vietnam timezone and no solar-time
metadata.

Deletion test: create a profile with non-default timezone, longitude, and
solar-time enabled; assert every aggregate section exposes the same normalized
metadata. Removing the parallel conversion should make this test pass.

Remediation: define one canonical `BirthProfile`/`BirthInstant` in core and
convert legacy advisory/Bazi structs at the boundary. Include timezone,
longitude, solar-time policy, gender, and time-known state in evidence and DTO
input echoes.

### Major — advisory confidence is based only on presence, not completeness

Evidence: `score_day_selection` sets confidence to `medium` whenever any
`BirthInput` exists and `low` otherwise (`crates/amlich-core/src/advisory.rs:347-355`).
The input may omit hour, minute, gender, and location (`:62-76`), yet receives
the same confidence as a complete profile. API `profile_completeness` counts
four booleans (`amlich-api/src/lib.rs:1044-1052`) but this value does not feed
the scoring confidence or availability decisions.

Deletion test: compare date-only, date+gender, and full datetime profiles;
confidence and available evidence should differ. If all remain `medium`, the
presence-only policy is still active.

Remediation: derive confidence from capability coverage and rule evidence,
returning structured reasons (e.g. `date_only`, `hour_unknown`,
`gender_missing`) rather than a free-form string.

### Minor — API documentation claims full datetime while accepting date-only

Evidence: `PersonalDayMatrixQueryDto` is documented as “full birth datetime
required” (`crates/amlich-api/src/dto.rs:1159-1168`), but `get_personal_day_matrix_report`
accepts the same query with `00:00`, labels it `Date`, and still emits day-person
and element matrices (`lib.rs:1503-1536`). Only personal hours are listed as
unavailable (`matrix_unavailable_sections`, `:941-949`).

Deletion test: either reject date-only input at the API boundary or update the
contract/docs and add explicit unavailable entries for hour-dependent sections.

Remediation: make request validation and DTO documentation agree; represent
capability-level availability rather than an all-or-nothing tier.

### Minor — aggregate advisory severity is a count of strings

Evidence: `get_personal_day_advisory` maps severity solely from `cautions.len()`
(`crates/amlich-api/src/lib.rs:1185-1192`). Missing Kua, missing Đại Vận, and
missing Ten Gods each append caution strings, so incomplete input can raise
severity even when no adverse day signal exists.

Deletion test: run the same date with no birth profile and with a complete
profile; severity should not become “high” merely because unavailable-context
messages were appended.

Remediation: separate `warnings`/`unavailable` from adverse signals and derive
severity from typed, weighted evidence with provenance.

## Recommended implementation order

1. Add canonical birth profile/time-known model and tests for midnight and
   partial fields (Blocker/Major correctness foundation).
2. Replace all API tier helpers and zero fallbacks with capability-based
   availability; make matrix DTO states explicit.
3. Build one core personal-day assessment and project all API endpoints from it;
   preserve legacy DTOs as adapters during migration.
4. Rework confidence/severity to consume typed evidence and profile coverage.
5. Add parity, serialization, and golden tests for date-only, midnight,
   date+gender, and full datetime/solar-time profiles.

## Limits

This audit did not validate the astronomical/Bazi formulas themselves against an
external golden dataset. It focuses on profile semantics, control flow, and
transport contracts visible in the referenced source.
