/**
 * Compile-time type tests that lock the desktop TS DTO mirrors against the
 * Rust serialization contract for `PersonalDayAssessmentDto` (amlich-mg01).
 *
 * Source of truth: `PersonalDayAssessmentDto` and its sub-structs in
 * crates/amlich-api/src/dto.rs. Rust uses `#[serde(skip_serializing_if =
 * "Option::is_none")]` on every `Option<T>` field, so those keys are OMITTED
 * from the wire payload (never emitted as `null`). These tests assert both
 * mirrors expose exactly the serde field set, accept a complete payload, and
 * tolerate omission of every optional field.
 *
 * These are compile-time only — run via `pnpm check` (svelte-check). If a
 * field drifts in either direction, the relevant assertion becomes a hard
 * type error.
 */

import type {
    PersonalDayAdvisoryDto,
    PersonalDayAssessmentDto,
    PersonalDayAxesDto,
    PersonalDayAxisOutcomeDto,
    PersonalDayContributionDto,
    PersonalDayDecisionDto,
    PersonalDayEvidenceDto,
    PersonalDayMatrixReportDto,
    PersonalDayNormalizedBirthDto,
    HourSelectionAdvisoryDto,
    HourSelectionAnalysisDto,
    PersonalDayReportDto,
} from './types';

import type {
    PersonalDayAssessmentDto as InsightsPersonalDayAssessmentDto,
    PersonalDayAxesDto as InsightsPersonalDayAxesDto,
    PersonalDayAxisOutcomeDto as InsightsPersonalDayAxisOutcomeDto,
    PersonalDayContributionDto as InsightsPersonalDayContributionDto,
    PersonalDayDecisionDto as InsightsPersonalDayDecisionDto,
    PersonalDayEvidenceDto as InsightsPersonalDayEvidenceDto,
    PersonalDayNormalizedBirthDto as InsightsPersonalDayNormalizedBirthDto,
    PersonalDayReportDto as InsightsPersonalDayReportDto,
    PersonalDayMatrixReportDto as InsightsPersonalDayMatrixReportDto,
} from '$lib/insights/types';

// ---------------------------------------------------------------------------
// Type-level helpers
// ---------------------------------------------------------------------------

/** Compile error unless `T` is exactly `true`. */
type AssertTrue<T extends true> = T;

/** Exact type equality (well-known `(<T>() => ...)` trick). */
type Equals<X, Y> =
    (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;

/** True when `T` declares an optional `canonical_assessment` of the contract type. */
type HasCanonicalAssessment<T> =
    T extends { canonical_assessment?: PersonalDayAssessmentDto | null } ? true : false;

// ---------------------------------------------------------------------------
// Key-set contracts (locked against crates/amlich-api/src/dto.rs)
//
// Each `AssertTrue<Equals<keyof X, ExpectedKeys>>` fails to compile if the
// mirror gains or loses a field relative to the Rust serde field set.
// ---------------------------------------------------------------------------

type _NormalizedBirthKeys = AssertTrue<
    Equals<
        keyof PersonalDayNormalizedBirthDto,
        'day' | 'month' | 'year' | 'has_time' | 'has_gender' | 'has_location' | 'has_solar_time_policy'
    >
>;

type _AxisOutcomeKeys = AssertTrue<
    Equals<keyof PersonalDayAxisOutcomeDto, 'axis' | 'score' | 'verdict' | 'unavailable_reason'>
>;

type _AxesKeys = AssertTrue<
    Equals<
        keyof PersonalDayAxesDto,
        | 'generic_day_quality'
        | 'intent_fit'
        | 'personal_alignment'
        | 'annual_pressure'
        | 'evidence_coverage'
    >
>;

type _DecisionKeys = AssertTrue<
    Equals<
        keyof PersonalDayDecisionDto,
        'bucket' | 'confidence' | 'semantic' | 'primary_conclusion' | 'decision_score' | 'context_is_clear'
    >
>;

type _ContributionKeys = AssertTrue<
    Equals<
        keyof PersonalDayContributionDto,
        | 'contribution_id'
        | 'axis'
        | 'polarity'
        | 'strength'
        | 'policy_id'
        | 'policy_version'
        | 'ruleset_id'
        | 'ruleset_version'
        | 'source_family'
        | 'source_id'
        | 'method'
        | 'note'
    >
>;

type _EvidenceKeys = AssertTrue<
    Equals<
        keyof PersonalDayEvidenceDto,
        'has_chart' | 'has_analysis' | 'has_yearly_han' | 'has_kua' | 'recommendation_count'
    >
>;

type _AssessmentKeys = AssertTrue<
    Equals<
        keyof PersonalDayAssessmentDto,
        | 'ruleset_id'
        | 'ruleset_version'
        | 'policy_id'
        | 'policy_version'
        | 'profile'
        | 'intent'
        | 'capability_tier'
        | 'normalized_birth'
        | 'axes'
        | 'decision'
        | 'contributions'
        | 'unavailable_sections'
        | 'evidence'
    >
>;

// ---------------------------------------------------------------------------
// Parent DTOs MUST surface the canonical verdict (amlich-mwbp.6/7).
// ---------------------------------------------------------------------------

type _AdvisoryHasIt = AssertTrue<HasCanonicalAssessment<PersonalDayAdvisoryDto>>;
type _ReportHasIt = AssertTrue<HasCanonicalAssessment<PersonalDayReportDto>>;
type _MatrixHasIt = AssertTrue<HasCanonicalAssessment<PersonalDayMatrixReportDto>>;
type _HourAnalysisHasIt = AssertTrue<HasCanonicalAssessment<HourSelectionAnalysisDto>>;
type _HourAdvisoryHasIt = AssertTrue<HasCanonicalAssessment<HourSelectionAdvisoryDto>>;

// ---------------------------------------------------------------------------
// The two mirrors (api/types.ts and insights/types/personal-day-dto.ts) MUST
// agree on the exact same field set for every canonical sub-DTO.
// ---------------------------------------------------------------------------

type _MirrorsAgreeAssessment = AssertTrue<
    Equals<keyof PersonalDayAssessmentDto, keyof InsightsPersonalDayAssessmentDto>
>;
type _MirrorsAgreeNormalizedBirth = AssertTrue<
    Equals<keyof PersonalDayNormalizedBirthDto, keyof InsightsPersonalDayNormalizedBirthDto>
>;
type _MirrorsAgreeAxes = AssertTrue<
    Equals<keyof PersonalDayAxesDto, keyof InsightsPersonalDayAxesDto>
>;
type _MirrorsAgreeAxisOutcome = AssertTrue<
    Equals<keyof PersonalDayAxisOutcomeDto, keyof InsightsPersonalDayAxisOutcomeDto>
>;
type _MirrorsAgreeDecision = AssertTrue<
    Equals<keyof PersonalDayDecisionDto, keyof InsightsPersonalDayDecisionDto>
>;
type _MirrorsAgreeContribution = AssertTrue<
    Equals<keyof PersonalDayContributionDto, keyof InsightsPersonalDayContributionDto>
>;
type _MirrorsAgreeEvidence = AssertTrue<
    Equals<keyof PersonalDayEvidenceDto, keyof InsightsPersonalDayEvidenceDto>
>;
type _InsightsReportHasIt = AssertTrue<HasCanonicalAssessment<InsightsPersonalDayReportDto>>;
type _InsightsMatrixHasIt = AssertTrue<HasCanonicalAssessment<InsightsPersonalDayMatrixReportDto>>;

// ---------------------------------------------------------------------------
// Value-level fixtures: `satisfies` enforces assignability AND rejects excess
// properties on the object literal, locking the deep shape.
// ---------------------------------------------------------------------------

// A complete payload that includes every optional field populated (the Rust
// serializer emits these when `Some`).
const COMPLETE_ASSESSMENT = {
    ruleset_id: 'vietnam-traditional',
    ruleset_version: '2024.1',
    policy_id: 'personal-day-v2',
    policy_version: '1.0.0',
    profile: 'personal-day',
    intent: 'initiation_opening',
    capability_tier: 'datetime' as const,
    normalized_birth: {
        day: 15,
        month: 3,
        year: 1990,
        has_time: true,
        has_gender: true,
        has_location: false,
        has_solar_time_policy: false,
    },
    axes: {
        generic_day_quality: { axis: 'generic_day_quality', score: 0.72, verdict: 'mixed' },
        intent_fit: { axis: 'intent_fit', score: 0.6, verdict: 'favorable' },
        personal_alignment: {
            axis: 'personal_alignment',
            score: 0.55,
            verdict: 'mixed',
            unavailable_reason: null,
        },
        annual_pressure: { axis: 'annual_pressure', verdict: 'clear' },
        evidence_coverage: { axis: 'evidence_coverage', score: 0.9, verdict: 'high' },
    },
    decision: {
        bucket: 'favorable',
        confidence: 'high',
        semantic: 'favorable_clear',
        primary_conclusion: 'Ngày thuận lợi cho khai trương.',
        decision_score: 0.71,
        context_is_clear: true,
    },
    contributions: [
        {
            contribution_id: 'truc-build',
            axis: 'intent_fit',
            polarity: 'support',
            strength: 0.4,
            policy_id: 'personal-day-v2',
            policy_version: '1.0.0',
            ruleset_id: 'vietnam-traditional',
            ruleset_version: '2024.1',
            source_family: 'almanac_rule',
            source_id: 'truc.khai.tru',
            method: 'truc_mapping',
            note: 'Trực phù hợp với ý định khai trương.',
        },
        {
            contribution_id: 'taboo-soft',
            axis: 'generic_day_quality',
            polarity: 'resistance',
            strength: 0.2,
            policy_id: 'personal-day-v2',
            policy_version: '1.0.0',
            ruleset_id: 'vietnam-traditional',
            ruleset_version: '2024.1',
            source_family: 'almanac_rule',
            source_id: 'sat.tinh.kim.2',
            method: 'sat_tinh_lookup',
        },
    ],
    unavailable_sections: [],
    evidence: {
        has_chart: true,
        has_analysis: true,
        has_yearly_han: false,
        has_kua: true,
        recommendation_count: 6,
    },
} satisfies PersonalDayAssessmentDto;

// A minimal payload that omits every `skip_serializing_if = Option::is_none`
// field — exactly what the Rust serializer emits when all Options are None.
// This proves each optional field is genuinely optional on the TS side.
const MINIMAL_ASSESSMENT = {
    ruleset_id: 'vietnam-traditional',
    ruleset_version: '2024.1',
    policy_id: 'personal-day-v2',
    policy_version: '1.0.0',
    profile: 'personal-day',
    intent: 'initiation_opening',
    capability_tier: 'anonymous' as const,
    normalized_birth: {
        day: 1,
        month: 1,
        year: 2024,
        has_time: false,
        has_gender: false,
        has_location: false,
        has_solar_time_policy: false,
    },
    axes: {
        generic_day_quality: { axis: 'generic_day_quality', verdict: 'mixed' },
        intent_fit: { axis: 'intent_fit', verdict: 'mixed' },
        personal_alignment: {
            axis: 'personal_alignment',
            verdict: 'unavailable',
            unavailable_reason: 'anonymous_tier',
        },
        annual_pressure: { axis: 'annual_pressure', verdict: 'clear' },
        evidence_coverage: { axis: 'evidence_coverage', verdict: 'low' },
    },
    decision: {
        bucket: 'cautious',
        confidence: 'low',
        semantic: 'conflicted_cautious',
        primary_conclusion: 'Thiếu dữ liệu sinh tháng ngày giờ.',
        context_is_clear: false,
    },
    contributions: [],
    unavailable_sections: [
        { section: 'personal_alignment', reason: 'anonymous_tier', required_fields: ['birth_time'] },
    ],
    evidence: {
        has_chart: false,
        has_analysis: false,
        has_yearly_han: false,
        has_kua: false,
        recommendation_count: 0,
    },
} satisfies PersonalDayAssessmentDto;

// The minimal payload must also satisfy the insights mirror.
const _minimalAlsoInsights: InsightsPersonalDayAssessmentDto = MINIMAL_ASSESSMENT;

// The complete payload's capability_tier is the typed literal, locking the
// enum contract ('anonymous' | 'date' | 'datetime').
const _capabilityTierLiteral: 'datetime' = COMPLETE_ASSESSMENT.capability_tier;

// Silence "declared but never read" for the value-level fixtures.
export const _fixtures = {
    complete: COMPLETE_ASSESSMENT,
    minimal: MINIMAL_ASSESSMENT,
    minimalAlsoInsights: _minimalAlsoInsights,
    capabilityTierLiteral: _capabilityTierLiteral,
};
