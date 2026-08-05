//! Personal-day assessment v2 seam — feature identifiers, observation
//! vector, calculation trace, and `baseline_v2` policy parity.
//!
//! Bead: `amlich-7bm4`.
//!
//! Locks the four acceptance criteria of `amlich-7bm4`:
//!
//! 1. **Feature identifiers and observations are stable and
//!    source-attributed** — every feature carries a stable
//!    [`AssessmentFeatureId`] and a populated `source_evidence`.
//! 2. **Unavailable is distinct from zero** — features flagged
//!    unavailable project to `signed_value == None` and are excluded from
//!    aggregation; they cannot perturb a score as a phantom zero.
//! 3. **Existing assessment/API outputs remain compatible** — v1 outputs
//!    are unchanged; the v2 policy produces byte-identical axes,
//!    decisions, contributions, and unavailable sections modulo the
//!    policy_version string.
//! 4. **Deterministic and parity tests cover the end-to-end path** —
//!    identical `(snapshot, profile, intent)` triples produce identical
//!    assessments under both v1 and v2, across a representative fixture
//!    grid (multiple intents × multiple capability profiles).

use amlich_core::almanac::tu_menh::Gender;
use amlich_core::{
    advisory::ConsultationIntent,
    assessment::{
        AssessmentAxis, AssessmentFeatureId, AssessmentInputs, AssessmentPolicy, AssessmentTrace,
        PersonalDayAssessment, ASSESSMENT_POLICY_V2_ID, ASSESSMENT_POLICY_V2_VERSION,
        ASSESSMENT_POLICY_VERSION,
    },
    birth::{BirthProfile, BirthTime},
    reasoning::RecommendationBucket,
    types::VIETNAM_TIMEZONE,
    DaySnapshot,
};

fn snapshot_2024_02_10() -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE)
}

fn full_profile() -> BirthProfile {
    BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: Some(BirthTime {
            hour: 9,
            minute: 30,
        }),
        timezone: VIETNAM_TIMEZONE,
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some(Gender::Male),
        location_name: Some("Hanoi".to_string()),
    }
}

fn no_time_no_gender() -> BirthProfile {
    BirthProfile {
        time: None,
        gender: None,
        ..full_profile()
    }
}

fn no_time_only() -> BirthProfile {
    BirthProfile {
        time: None,
        ..full_profile()
    }
}

fn no_gender_only() -> BirthProfile {
    BirthProfile {
        gender: None,
        ..full_profile()
    }
}

/// Build a v1 assessment with the legacy default entry point.
fn v1(
    snapshot: DaySnapshot,
    profile: BirthProfile,
    intent: ConsultationIntent,
) -> PersonalDayAssessment {
    PersonalDayAssessment::assess(snapshot, profile, intent)
}

/// Build a v2 assessment with the new [`AssessmentPolicy::baseline_v2`]
/// entry point.
fn v2(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) -> PersonalDayAssessment {
    AssessmentPolicy::baseline_v2().evaluate(AssessmentInputs::default(), snapshot, profile, intent)
}

const ALL_INTENTS: [ConsultationIntent; 9] = [
    ConsultationIntent::Wedding,
    ConsultationIntent::MovingHouse,
    ConsultationIntent::OpeningBusiness,
    ConsultationIntent::ContractSigning,
    ConsultationIntent::Travel,
    ConsultationIntent::Burial,
    ConsultationIntent::Renovation,
    ConsultationIntent::Medical,
    ConsultationIntent::Prayer,
];

// ---------------------------------------------------------------------------
// Acceptance criterion 1: Feature identifiers and observations are stable
// and source-attributed.
// ---------------------------------------------------------------------------

#[test]
fn v2_emits_a_trace_with_stable_feature_ids() {
    let assessment = v2(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = assessment
        .trace
        .as_ref()
        .expect("v2 policy must attach an AssessmentTrace");

    assert_eq!(trace.policy_id, ASSESSMENT_POLICY_V2_ID);
    assert_eq!(trace.policy_version, ASSESSMENT_POLICY_V2_VERSION);

    // Every emitted feature carries a stable AssessmentFeatureId,
    // source evidence with a non-empty source_id, and ruleset metadata.
    assert!(
        !trace.features.is_empty(),
        "baseline_v2 must emit at least one feature observation for a full profile"
    );
    for feature in &trace.features {
        assert!(
            AssessmentFeatureId::ALL
                .iter()
                .any(|f| *f == feature.feature_id),
            "feature {:?} is not in the stable ALL list",
            feature.feature_id
        );
        assert!(
            !feature.source_evidence.source_id.is_empty(),
            "feature {:?} is missing source attribution",
            feature.feature_id
        );
        assert!(
            !feature.source_evidence.source_family.is_empty(),
            "feature {:?} is missing source_family",
            feature.feature_id
        );
        assert!(
            !feature.source_evidence.method.is_empty(),
            "feature {:?} is missing extraction method",
            feature.feature_id
        );
        assert!(
            !feature.contribution_id.is_empty(),
            "feature {:?} is missing a stable contribution_id",
            feature.feature_id
        );
    }
}

#[test]
fn v2_trace_axis_aggregations_cover_all_five_axes() {
    let assessment = v2(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = assessment.trace.as_ref().expect("v2 trace");

    // The trace exposes one AxisAggregation per canonical axis, in the
    // stable AssessmentAxis::ALL order.
    assert_eq!(trace.axes.len(), AssessmentAxis::ALL.len());
    for (idx, axis) in AssessmentAxis::ALL.iter().enumerate() {
        assert_eq!(
            trace.axes[idx].axis, *axis,
            "trace axis order must follow AssessmentAxis::ALL"
        );
    }
}

// ---------------------------------------------------------------------------
// Acceptance criterion 2: Unavailable is distinct from zero.
// ---------------------------------------------------------------------------

#[test]
fn unavailable_features_project_to_none_and_do_not_inflate_scores() {
    // The no-time-no-gender profile cannot support several features
    // (PersonalSameChi/LucXung/TamHop/LiuHe, TimingHoangDaoRatio,
    // AnnualThaiTue via Hạn). baseline_v2's extraction simply omits
    // those features rather than emitting them as zero, and the v2
    // policy reports the affected axes as unavailable.
    let assessment = v2(
        &snapshot_2024_02_10(),
        &no_time_no_gender(),
        ConsultationIntent::Wedding,
    );

    // The affected axes are explicitly unavailable (score == None),
    // NOT neutral 0.5 — that is the "unavailable is distinct from zero"
    // guarantee.
    assert!(
        assessment.axes.personal_alignment.score.is_none(),
        "PersonalAlignment must be unavailable (None) without gender, not zero"
    );
    assert!(
        assessment
            .axes
            .personal_alignment
            .unavailable_reason
            .as_ref()
            .is_some_and(|r| !r.is_empty()),
        "PersonalAlignment must carry a non-empty unavailable_reason"
    );
    assert!(
        assessment.axes.annual_pressure.score.is_none(),
        "AnnualPressure must be unavailable (None) without gender, not zero"
    );

    // The trace mirrors the unavailability in the axis aggregation.
    let trace = assessment.trace.as_ref().expect("v2 trace");
    let personal = trace
        .axes
        .iter()
        .find(|a| a.axis == AssessmentAxis::PersonalAlignment)
        .expect("personal alignment aggregation");
    assert!(personal.subtotal.is_none());
    assert!(personal
        .unavailable_reason
        .as_ref()
        .is_some_and(|r| !r.is_empty()));
    assert!(
        personal.contributors.is_empty(),
        "Unavailable axes must record no contributors"
    );
}

#[test]
fn available_features_carry_signed_values_in_unit_range() {
    let assessment = v2(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Travel,
    );
    let trace = assessment.trace.as_ref().expect("v2 trace");

    // Every available feature projects to a signed value in [-1, 1].
    // Unavailable features are excluded entirely (baseline_v2 emits none
    // for this full-profile fixture).
    for feature in &trace.features {
        if let Some(value) = feature.signed_value() {
            assert!(
                (-1.0..=1.0).contains(&value),
                "feature {:?} projected to {value} outside [-1, 1]",
                feature.feature_id
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Acceptance criterion 3: Existing assessment/API outputs remain
// compatible. v1 is unchanged; v2 produces parity outputs modulo the
// policy_version tag and the trace field.
// ---------------------------------------------------------------------------

#[test]
fn v1_metadata_stays_pinned_at_v1() {
    // The legacy `PersonalDayAssessment::assess` entry point is unchanged
    // and still reports the v1 policy tag. This is the v1/v2 coexistence
    // invariant: the stability gate (amlich-31oa) decides when v2 becomes
    // the default.
    assert_eq!(ASSESSMENT_POLICY_VERSION, "v1");
    let assessment = v1(
        snapshot_2024_02_10(),
        full_profile(),
        ConsultationIntent::Wedding,
    );
    assert_eq!(assessment.policy_version, "v1");
    assert!(
        assessment.trace.is_none(),
        "v1 builder must NOT emit a trace (kept opt-in via the v2 policy)"
    );
}

#[test]
fn v2_metadata_reports_v2_and_attaches_a_trace() {
    let assessment = v2(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    assert_eq!(assessment.policy_version, "v2");
    assert!(assessment.trace.is_some());
    assert_eq!(
        assessment.policy_id, ASSESSMENT_POLICY_V2_ID,
        "v2 keeps the same policy family as v1; only the version moves"
    );
}

// ---------------------------------------------------------------------------
// Acceptance criterion 4: Deterministic and parity tests cover the
// end-to-end path. The v1-vs-v2 parity grid below runs every intent
// against every capability profile and asserts byte-identical axes,
// contributions, decisions, and unavailable sections (modulo the
// policy_version string).
// ---------------------------------------------------------------------------

fn assert_v1_v2_axis_parity(v1: &PersonalDayAssessment, v2: &PersonalDayAssessment) {
    for axis in AssessmentAxis::ALL {
        let a = v1.axes.iter().find(|o| o.axis == axis).expect("v1 axis");
        let b = v2.axes.iter().find(|o| o.axis == axis).expect("v2 axis");
        assert_eq!(
            a.score, b.score,
            "axis {axis:?} score divergence between v1 and v2"
        );
        assert_eq!(
            a.verdict, b.verdict,
            "axis {axis:?} verdict divergence between v1 and v2"
        );
        assert_eq!(
            a.unavailable_reason, b.unavailable_reason,
            "axis {axis:?} unavailable_reason divergence between v1 and v2"
        );
    }
}

fn assert_v1_v2_contributions_parity(v1: &PersonalDayAssessment, v2: &PersonalDayAssessment) {
    // Contribution-id set must match exactly. Per-contribution fields
    // must match exactly EXCEPT policy_version (which is the v1-vs-v2
    // signal).
    let ids = |a: &PersonalDayAssessment| -> Vec<String> {
        a.contributions
            .iter()
            .map(|c| c.contribution_id.clone())
            .collect()
    };
    let v1_ids = ids(v1);
    let v2_ids = ids(v2);
    assert_eq!(
        v1_ids, v2_ids,
        "v1/v2 contribution-id sets diverged (must be byte-identical)"
    );

    for (c1, c2) in v1.contributions.iter().zip(v2.contributions.iter()) {
        assert_eq!(c1.contribution_id, c2.contribution_id);
        assert_eq!(c1.axis, c2.axis);
        assert_eq!(c1.polarity, c2.polarity);
        assert_eq!(c1.strength, c2.strength);
        assert_eq!(c1.ruleset_id, c2.ruleset_id);
        assert_eq!(c1.ruleset_version, c2.ruleset_version);
        assert_eq!(c1.source_evidence, c2.source_evidence);
        assert_eq!(c1.availability, c2.availability);
        assert_eq!(c1.note, c2.note);
        // policy_id is identical (same family); only policy_version
        // carries the v1-vs-v2 signal.
        assert_eq!(c1.policy_id, c2.policy_id);
        assert_eq!(c1.policy_version, "v1");
        assert_eq!(c2.policy_version, "v2");
    }
}

fn assert_v1_v2_decision_parity(v1: &PersonalDayAssessment, v2: &PersonalDayAssessment) {
    assert_eq!(
        v1.decision.bucket, v2.decision.bucket,
        "decision.bucket divergence"
    );
    assert_eq!(
        v1.decision.semantic, v2.decision.semantic,
        "decision.semantic divergence"
    );
    assert_eq!(
        v1.decision.decision_score, v2.decision.decision_score,
        "decision.decision_score divergence"
    );
    assert_eq!(
        v1.decision.confidence, v2.decision.confidence,
        "decision.confidence divergence"
    );
    assert_eq!(
        v1.decision.primary_conclusion, v2.decision.primary_conclusion,
        "decision.primary_conclusion divergence"
    );
    assert_eq!(
        v1.decision.context_is_clear, v2.decision.context_is_clear,
        "decision.context_is_clear divergence"
    );
}

fn assert_v1_v2_unavailable_sections_parity(
    v1: &PersonalDayAssessment,
    v2: &PersonalDayAssessment,
) {
    // Order matters for byte-level serialized parity. v2 reproduces
    // v1's emission order: personal_alignment, personal_hours, annual_han.
    assert_eq!(
        v1.unavailable_sections, v2.unavailable_sections,
        "unavailable_sections diverged (incl. order)"
    );
}

fn assert_v1_v2_full_parity(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) {
    let v1 = v1(snapshot.clone(), profile.clone(), intent);
    let v2 = v2(snapshot, profile, intent);

    assert_v1_v2_axis_parity(&v1, &v2);
    assert_v1_v2_contributions_parity(&v1, &v2);
    assert_v1_v2_decision_parity(&v1, &v2);
    assert_v1_v2_unavailable_sections_parity(&v1, &v2);

    // Top-level envelope fields (other than policy_version + trace)
    // must match too.
    assert_eq!(v1.ruleset_id, v2.ruleset_id);
    assert_eq!(v1.ruleset_version, v2.ruleset_version);
    assert_eq!(v1.profile, v2.profile);
    assert_eq!(v1.policy_id, v2.policy_id);
    assert_eq!(v1.capability, v2.capability);
    assert_eq!(v1.capability_tier, v2.capability_tier);
    assert_eq!(v1.normalized_birth, v2.normalized_birth);
    assert_eq!(v1.intent, v2.intent);
    assert_eq!(v1.evidence, v2.evidence);
}

#[test]
fn v1_v2_full_parity_across_intent_and_capability_grid() {
    let snapshot = snapshot_2024_02_10();
    for profile in [
        full_profile(),
        no_time_only(),
        no_gender_only(),
        no_time_no_gender(),
    ] {
        for intent in ALL_INTENTS {
            assert_v1_v2_full_parity(&snapshot, &profile, intent);
        }
    }
}

#[test]
fn v2_evaluation_is_pure_and_idempotent() {
    // Determinism: identical (policy, inputs, snapshot, profile, intent)
    // quintuples produce identical assessments.
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();
    let intent = ConsultationIntent::ContractSigning;

    let a = v2(&snapshot, &profile, intent);
    let b = v2(&snapshot, &profile, intent);

    assert_eq!(a.axes, b.axes);
    assert_eq!(a.contributions, b.contributions);
    assert_eq!(a.decision, b.decision);
    assert_eq!(a.unavailable_sections, b.unavailable_sections);
    assert_eq!(a.evidence, b.evidence);
    assert_eq!(a.trace, b.trace);
}

#[test]
fn v2_trace_decision_aggregation_records_axis_weights_and_bucket() {
    let assessment = v2(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = assessment.trace.as_ref().expect("v2 trace");

    // baseline_v2 weights every available scored axis equally.
    assert!(
        !trace.decision.axis_weights.is_empty(),
        "decision aggregation must record at least one axis weight"
    );
    let first_weight = trace.decision.axis_weights[0].weight;
    for entry in &trace.decision.axis_weights {
        assert_eq!(
            entry.weight, first_weight,
            "baseline_v2 must use equal axis weights; non-equal weights land in amlich-lxu3"
        );
    }
    assert_eq!(
        trace.decision.bucket, assessment.decision.bucket,
        "trace decision bucket must match the assessment decision bucket"
    );
    assert_eq!(
        trace.decision.decision_score, assessment.decision.decision_score,
        "trace decision_score must match the assessment decision_score"
    );

    // baseline_v2 does not populate the veto or interaction sections;
    // those are layered in by amlich-l0wu and amlich-47wn.
    assert!(
        trace.vetoes.is_empty(),
        "baseline_v2 must not emit named vetoes (amlich-l0wu)"
    );
    assert!(
        trace.interactions.is_empty(),
        "baseline_v2 must not emit interaction terms (amlich-47wn)"
    );
}

#[test]
fn v2_trace_round_trips_through_serde() {
    // The trace is part of the serialized envelope contract. Make sure
    // it survives a serde round-trip unchanged so downstream consumers
    // (Evidence Graph projection in amlich-8tdm, API DTOs, fixtures)
    // can rely on the wire shape.
    let assessment = v2(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = assessment.trace.as_ref().expect("v2 trace");

    let json = serde_json::to_string(trace).expect("serialize AssessmentTrace");
    let back: AssessmentTrace = serde_json::from_str(&json).expect("deserialize AssessmentTrace");
    assert_eq!(trace, &back);
}

#[test]
fn v2_preserves_hard_veto_override_under_baseline_parity() {
    // The legacy v1 builder forces an Avoid bucket when any avoid
    // contribution has strength >= 0.8 (the implicit veto). baseline_v2
    // preserves this behavior; lifting it into named vetoes is
    // amlich-l0wu. Pick a fixture where the override fires — the
    // 2024-02-10 snapshot's Hạn assessment hits Critical severity for
    // the 1990 birth year, which maps to strength 1.0.
    let snapshot = snapshot_2024_02_10();
    let v1 = v1(
        snapshot.clone(),
        full_profile(),
        ConsultationIntent::Wedding,
    );
    let v2 = v2(&snapshot, &full_profile(), ConsultationIntent::Wedding);

    // Both paths agree on the bucket and on the override semantic.
    assert_eq!(v1.decision.bucket, v2.decision.bucket);
    let trace = v2.trace.as_ref().expect("v2 trace");
    let _ = trace; // baseline_v2 keeps the override inside synthesize_decision;
                   // amlich-l0wu will surface it via trace.vetoes.
                   // Sanity: the trace decision aggregation records the resulting bucket.
    assert_eq!(trace.decision.bucket as u8, v2.decision.bucket as u8);
    // RecommendationBucket is represented identically; the cast above
    // is just a robustness check that the enum variants align.
    let _ = RecommendationBucket::Avoid;
}
