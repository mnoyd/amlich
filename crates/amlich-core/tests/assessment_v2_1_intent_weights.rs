//! Personal-day assessment v2.1 — intent-aware axis weights
//! (`amlich-lxu3`).
//!
//! Locks the five acceptance criteria of `amlich-lxu3`:
//!
//! 1. **Weights are policy-versioned** — every v2.1 trace records the
//!    policy_version and the actual (renormalized) axis weights it
//!    applied.
//! 2. **All five axes remain visible** — the assessment envelope still
//!    surfaces the same five `AssessmentAxis` outcomes as `baseline_v2`;
//!    only the final decision aggregation changed.
//! 3. **Different intents can produce different final projections** —
//!    the same `(snapshot, profile)` pair under two different intents
//!    yields different `axis_weights` in the trace and (for at least one
//!    reviewed fixture) a different final `decision_score`.
//! 4. **API projections expose the resulting scores without contract
//!    drift** — the v2.1 policy still emits the same envelope shape
//!    (`PersonalDayAssessment`), the policy_version follows the
//!    conventional `v<digit>` format used by API parity gates, and the
//!    trace still round-trips through serde.
//! 5. **Reviewed fixtures document intentional divergences** — for the
//!    same `(snapshot, profile, intent)` triple, v2.1 vs `baseline_v2`
//!    axis subtotals match byte-for-byte; the only intentional
//!    divergence is the final decision aggregation, which is documented
//!    per fixture.
//!
//! ## Out of scope
//!
//! The stability gate (`amlich-31oa`) decides when v2.x is promoted to
//! the default. Until then, the production path
//! (`PersonalDayAssessment::assess`) stays pinned to v1 and these tests
//! cover only the opt-in v2.1 policy.

use amlich_core::almanac::tu_menh::Gender;
use amlich_core::{
    advisory::ConsultationIntent,
    assessment::{
        AssessmentAxis, AssessmentInputs, AssessmentPolicy, AssessmentTrace, IntentAxisWeightTable,
        IntentAxisWeights, PersonalDayAssessment, ASSESSMENT_POLICY_V2_1_VERSION,
        ASSESSMENT_POLICY_V2_ID, ASSESSMENT_POLICY_V2_VERSION, INTENT_AXIS_WEIGHTS_V2_1,
    },
    birth::{BirthProfile, BirthTime},
    reasoning::RecommendationBucket,
    types::VIETNAM_TIMEZONE,
    DaySnapshot,
};

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

fn snapshot_2024_02_10() -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE)
}

fn snapshot_2024_05_05() -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(5, 5, 2024, VIETNAM_TIMEZONE)
}

fn snapshot_2024_12_25() -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(25, 12, 2024, VIETNAM_TIMEZONE)
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

/// A profile that triggers `veto.annual.han_severe` on 2024-02-10
/// (birth year 1985 hits Hạn High severity). Used to verify the hard
/// veto precedence under v2.1.
fn han_severe_profile() -> BirthProfile {
    BirthProfile {
        year: 1985,
        gender: Some(Gender::Female),
        ..full_profile()
    }
}

/// Build a v2 (v1-parity) assessment.
fn v2(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) -> PersonalDayAssessment {
    AssessmentPolicy::baseline_v2().evaluate(AssessmentInputs::default(), snapshot, profile, intent)
}

/// Build a v2.1 (intent-aware) assessment.
fn v2_1(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) -> PersonalDayAssessment {
    AssessmentPolicy::intent_weighted_v2().evaluate(
        AssessmentInputs::default(),
        snapshot,
        profile,
        intent,
    )
}

// ---------------------------------------------------------------------------
// Acceptance criterion 1: Weights are policy-versioned.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_policy_reports_versioned_metadata() {
    let policy = AssessmentPolicy::intent_weighted_v2();
    assert_eq!(policy.policy_id(), ASSESSMENT_POLICY_V2_ID);
    assert_eq!(policy.policy_version(), ASSESSMENT_POLICY_V2_1_VERSION);
    assert_eq!(policy.policy_version(), "v2.1");

    // The policy family is unchanged; the version bump carries the
    // intent-aware divergence signal (same convention as v1 vs v2).
    assert_ne!(
        policy.policy_version(),
        ASSESSMENT_POLICY_V2_VERSION,
        "v2.1 must not collide with the v1-parity baseline v2 version"
    );
}

#[test]
fn v2_1_assessment_carries_v2_1_policy_version() {
    let assessment = v2_1(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    assert_eq!(assessment.policy_id, ASSESSMENT_POLICY_V2_ID);
    assert_eq!(assessment.policy_version, ASSESSMENT_POLICY_V2_1_VERSION);
    assert_eq!(assessment.policy_version, "v2.1");
    assert!(
        assessment.policy_version.starts_with('v'),
        "v2.1 policy_version must satisfy the API parity gate's 'v<digit>' format"
    );

    let trace = assessment.trace.as_ref().expect("v2.1 trace");
    assert_eq!(trace.policy_version, ASSESSMENT_POLICY_V2_1_VERSION);
}

#[test]
fn v2_1_trace_records_actual_applied_axis_weights() {
    // The trace must record the actual renormalized axis weights the
    // v2.1 policy applied, so explanations can show "Travel weighted
    // IntentFit at 0.40 of the final score".
    let assessment = v2_1(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Travel,
    );
    let trace = assessment.trace.as_ref().expect("v2.1 trace");

    let lookup = |axis: AssessmentAxis| -> Option<f32> {
        trace
            .decision
            .axis_weights
            .iter()
            .find(|w| w.axis == axis)
            .map(|w| w.weight)
    };

    // Every available scored axis carries a weight; the four scored
    // axes are all available under a full profile.
    for axis in [
        AssessmentAxis::GenericDayQuality,
        AssessmentAxis::IntentFit,
        AssessmentAxis::PersonalAlignment,
        AssessmentAxis::AnnualPressure,
    ] {
        assert!(
            lookup(axis).is_some(),
            "v2.1 trace must record a weight for {:?}",
            axis
        );
    }

    // Renormalized weights over the available axes must sum to ~1.0 so
    // the decision projection is not accidentally scaled.
    let total: f32 = trace.decision.axis_weights.iter().map(|w| w.weight).sum();
    assert!(
        (total - 1.0).abs() < 1e-5,
        "v2.1 renormalized axis weights must sum to 1.0; got {total}"
    );

    // EvidenceCoverage is excluded from decision aggregation.
    assert!(
        lookup(AssessmentAxis::EvidenceCoverage).is_none(),
        "EvidenceCoverage must NOT carry an intent-specific decision weight"
    );
}

// ---------------------------------------------------------------------------
// Acceptance criterion 2: All five axes remain visible.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_keeps_all_five_axes_visible_in_envelope() {
    let assessment = v2_1(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );

    // Every canonical axis must surface in the assessment envelope with
    // a populated score for a full profile.
    for axis in AssessmentAxis::ALL {
        let outcome = assessment.axes.iter().find(|o| o.axis == axis);
        let outcome = outcome.expect("v2.1 must surface every canonical axis");
        if axis == AssessmentAxis::EvidenceCoverage {
            // EvidenceCoverage is always available when there is *any*
            // capability signal.
            assert!(outcome.score.is_some(), "EvidenceCoverage must be scored");
        } else {
            assert!(
                outcome.score.is_some(),
                "{axis:?} must carry a score under v2.1 for a full profile"
            );
        }
        assert!(
            !outcome.verdict.is_empty(),
            "{axis:?} must carry a verdict under v2.1"
        );
    }
}

#[test]
fn v2_1_axis_subtotals_match_v2_byte_for_byte() {
    // Intent-awareness changes ONLY the final decision aggregation, not
    // the axis subtotals. For every fixture, v2.1 axis outcomes must
    // match baseline_v2 axis outcomes exactly.
    let snapshot = snapshot_2024_02_10();
    let profiles = [full_profile(), no_time_no_gender(), han_severe_profile()];
    for profile in &profiles {
        for intent in ALL_INTENTS {
            let v2 = v2(&snapshot, profile, intent);
            let v2_1 = v2_1(&snapshot, profile, intent);
            for axis in AssessmentAxis::ALL {
                let a = v2.axes.iter().find(|o| o.axis == axis).expect("v2 axis");
                let b = v2_1
                    .axes
                    .iter()
                    .find(|o| o.axis == axis)
                    .expect("v2.1 axis");
                assert_eq!(
                    a.score, b.score,
                    "{axis:?} subtotal diverged between v2 and v2.1 for {:?}",
                    intent
                );
                assert_eq!(
                    a.verdict, b.verdict,
                    "{axis:?} verdict diverged between v2 and v2.1 for {:?}",
                    intent
                );
                assert_eq!(
                    a.unavailable_reason, b.unavailable_reason,
                    "{axis:?} unavailable_reason diverged between v2 and v2.1 for {:?}",
                    intent
                );
            }
        }
    }
}

#[test]
fn v2_1_trace_covers_all_five_axes_in_order() {
    let assessment = v2_1(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = assessment.trace.as_ref().expect("v2.1 trace");
    assert_eq!(trace.axes.len(), AssessmentAxis::ALL.len());
    for (idx, axis) in AssessmentAxis::ALL.iter().enumerate() {
        assert_eq!(
            trace.axes[idx].axis, *axis,
            "v2.1 trace axis order must follow AssessmentAxis::ALL"
        );
    }
}

// ---------------------------------------------------------------------------
// Acceptance criterion 3: Different intents can produce different final
// projections.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_different_intents_produce_different_axis_weights() {
    // For the same (snapshot, profile), different intents MUST yield
    // different renormalized axis_weights in the trace. If they didn't,
    // the v2.1 table would be degenerate.
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();

    let wedding = v2_1(&snapshot, &profile, ConsultationIntent::Wedding);
    let travel = v2_1(&snapshot, &profile, ConsultationIntent::Travel);
    let burial = v2_1(&snapshot, &profile, ConsultationIntent::Burial);

    let weight_of = |a: &PersonalDayAssessment, axis: AssessmentAxis| -> Option<f32> {
        let t = a.trace.as_ref().expect("trace");
        t.decision
            .axis_weights
            .iter()
            .find(|w| w.axis == axis)
            .map(|w| w.weight)
    };

    // Travel prioritizes IntentFit (0.40) over PersonalAlignment (0.15);
    // Wedding does the opposite (0.25 vs 0.30). They must differ.
    assert_ne!(
        weight_of(&wedding, AssessmentAxis::IntentFit),
        weight_of(&travel, AssessmentAxis::IntentFit),
        "Wedding vs Travel IntentFit weights must differ"
    );
    assert_ne!(
        weight_of(&wedding, AssessmentAxis::PersonalAlignment),
        weight_of(&travel, AssessmentAxis::PersonalAlignment),
        "Wedding vs Travel PersonalAlignment weights must differ"
    );

    // Burial prioritizes AnnualPressure (0.35) over IntentFit (0.20);
    // Travel is the inverse. They must differ.
    assert_ne!(
        weight_of(&burial, AssessmentAxis::AnnualPressure),
        weight_of(&travel, AssessmentAxis::AnnualPressure),
        "Burial vs Travel AnnualPressure weights must differ"
    );
}

#[test]
fn v2_1_intent_weights_project_the_published_table_values() {
    // For a full profile (all four scored axes available), the
    // renormalized axis weights must equal the raw v2.1 table values
    // exactly (because the raw weights already sum to 1.0).
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();

    for intent in ALL_INTENTS {
        let assessment = v2_1(&snapshot, &profile, intent);
        let trace = assessment.trace.as_ref().expect("trace");
        let expected = INTENT_AXIS_WEIGHTS_V2_1.weights_for(intent);

        let lookup = |axis: AssessmentAxis| -> f32 {
            trace
                .decision
                .axis_weights
                .iter()
                .find(|w| w.axis == axis)
                .map(|w| w.weight)
                .unwrap_or(0.0)
        };

        assert!(
            (lookup(AssessmentAxis::GenericDayQuality) - expected.generic_day_quality).abs() < 1e-5,
            "{intent:?}: GenericDayQuality weight diverged from v2.1 table"
        );
        assert!(
            (lookup(AssessmentAxis::IntentFit) - expected.intent_fit).abs() < 1e-5,
            "{intent:?}: IntentFit weight diverged from v2.1 table"
        );
        assert!(
            (lookup(AssessmentAxis::PersonalAlignment) - expected.personal_alignment).abs() < 1e-5,
            "{intent:?}: PersonalAlignment weight diverged from v2.1 table"
        );
        assert!(
            (lookup(AssessmentAxis::AnnualPressure) - expected.annual_pressure).abs() < 1e-5,
            "{intent:?}: AnnualPressure weight diverged from v2.1 table"
        );
    }
}

#[test]
fn v2_1_intent_weights_change_decision_score_for_at_least_one_fixture() {
    // The whole point of amlich-lxu3: across a representative grid of
    // fixtures, there must be at least one (snapshot, profile, intent)
    // triple where v2.1 produces a different decision_score than v2.
    // Otherwise the weights have no effect and the bead is vacuous.
    //
    // The grid below intentionally mixes several snapshots (to vary
    // axis subtotals) so that an axis-score-sensitive weighting change
    // has a chance to actually move the projection.
    let snapshots = [
        snapshot_2024_02_10(),
        snapshot_2024_05_05(),
        snapshot_2024_12_25(),
    ];
    let profiles = [full_profile(), no_time_no_gender(), han_severe_profile()];

    let mut divergent = 0;
    let mut total = 0;
    for snapshot in &snapshots {
        for profile in &profiles {
            for intent in ALL_INTENTS {
                total += 1;
                let v2 = v2(snapshot, profile, intent);
                let v2_1 = v2_1(snapshot, profile, intent);

                // Skip veto-firing fixtures: the veto forces the same
                // decision_score (Some(0.15)) under both policies, by
                // design. Weights are irrelevant there. That is the
                // correct, intentional behavior — it just isn't a
                // weighted-divergence data point.
                let v2_vetoed = v2
                    .trace
                    .as_ref()
                    .map(|t| !t.vetoes.is_empty())
                    .unwrap_or(false);
                if v2_vetoed {
                    continue;
                }

                // Skip the KyManh / Tranh override path: those also
                // force a fixed score (0.2 / weighted average), so
                // any weighted-score divergence there would be
                // coincidental rather than the intentional v2.1
                // behavior under test.
                let v2_override = matches!(v2.decision.semantic.as_str(), "override_avoid");

                if !v2_override && v2.decision.decision_score != v2_1.decision.decision_score {
                    divergent += 1;
                }
            }
        }
    }
    assert!(
        divergent > 0,
        "expected at least one weighted decision_score divergence between v2 and v2.1 across {} candidates (found {divergent})",
        total
    );
}

// ---------------------------------------------------------------------------
// Acceptance criterion 3b: Renormalization under capability gaps.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_renormalizes_axis_weights_when_some_axes_are_unavailable() {
    // The no-time-no-gender profile leaves PersonalAlignment and
    // AnnualPressure unavailable. v2.1 must:
    //   (a) drop those axes' raw weights from the denominator;
    //   (b) renormalize the remaining axes' weights to sum to 1.0;
    //   (c) not report a weight for an unavailable axis.
    let assessment = v2_1(
        &snapshot_2024_02_10(),
        &no_time_no_gender(),
        ConsultationIntent::Wedding,
    );
    let trace = assessment.trace.as_ref().expect("v2.1 trace");

    let expected = INTENT_AXIS_WEIGHTS_V2_1.weights_for(ConsultationIntent::Wedding);
    let raw_total_available = expected.generic_day_quality + expected.intent_fit; // only these two are available

    let lookup = |axis: AssessmentAxis| -> Option<f32> {
        trace
            .decision
            .axis_weights
            .iter()
            .find(|w| w.axis == axis)
            .map(|w| w.weight)
    };

    let g = lookup(AssessmentAxis::GenericDayQuality).expect("GDQ weight");
    let i = lookup(AssessmentAxis::IntentFit).expect("IntentFit weight");

    assert!(
        (g - expected.generic_day_quality / raw_total_available).abs() < 1e-5,
        "GenericDayQuality not renormalized: got {g}, expected {}",
        expected.generic_day_quality / raw_total_available
    );
    assert!(
        (i - expected.intent_fit / raw_total_available).abs() < 1e-5,
        "IntentFit not renormalized: got {i}, expected {}",
        expected.intent_fit / raw_total_available
    );

    // PersonalAlignment and AnnualPressure were unavailable, so they
    // must NOT appear in the decision weights list.
    assert!(
        lookup(AssessmentAxis::PersonalAlignment).is_none(),
        "unavailable PersonalAlignment must not carry a decision weight"
    );
    assert!(
        lookup(AssessmentAxis::AnnualPressure).is_none(),
        "unavailable AnnualPressure must not carry a decision weight"
    );

    // The available_axes / unavailable_axes split must reflect the
    // capability gap exactly.
    assert_eq!(
        trace.decision.available_axes.len(),
        2,
        "no-time-no-gender profile exposes exactly two available scored axes"
    );
    assert_eq!(
        trace.decision.unavailable_axes.len(),
        2,
        "no-time-no-gender profile leaves exactly two scored axes unavailable"
    );
}

#[test]
fn v2_1_capability_gap_does_not_inflate_score() {
    // The "unavailable is not zero" contract from amlich-7bm4 carries
    // over to v2.1: a capability gap (unavailable axes) must not push
    // the decision_score above what those same axes would have
    // contributed as neutral 0.5 outcomes. We can't easily prove
    // "lower than neutral" without the neutral reference, but we CAN
    // assert the structural guarantee: the score is a weighted average
    // of available axis subtotals only, and renormalized weights sum
    // to 1.0 (locked by the previous test). Here we additionally check
    // that the score is in [0, 1] and that the renormalized weights
    // strictly increased compared to the all-available case for
    // Wedding (because the denominator shrank).
    let full = v2_1(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let gapped = v2_1(
        &snapshot_2024_02_10(),
        &no_time_no_gender(),
        ConsultationIntent::Wedding,
    );

    let full_score = full.decision.decision_score.expect("full score");
    let gapped_score = gapped.decision.decision_score.expect("gapped score");
    assert!(
        (0.0..=1.0).contains(&full_score) && (0.0..=1.0).contains(&gapped_score),
        "decision_score must remain in [0, 1]"
    );

    let expected = INTENT_AXIS_WEIGHTS_V2_1.weights_for(ConsultationIntent::Wedding);
    let raw_total_available = expected.generic_day_quality + expected.intent_fit;
    let g_gapped = gapped
        .trace
        .as_ref()
        .expect("trace")
        .decision
        .axis_weights
        .iter()
        .find(|w| w.axis == AssessmentAxis::GenericDayQuality)
        .map(|w| w.weight)
        .expect("GDQ weight");
    // Renormalization strictly increased the per-axis weight because
    // the denominator shrank (the available-axis weight total is now
    // < 1.0 in raw terms).
    assert!(
        g_gapped > expected.generic_day_quality,
        "renormalized weight {g_gapped} must exceed the raw table weight {} \
         when some axes are unavailable",
        expected.generic_day_quality
    );
    assert!((g_gapped - expected.generic_day_quality / raw_total_available).abs() < 1e-5);
}

// ---------------------------------------------------------------------------
// Acceptance criterion 4: API projections expose the resulting scores
// without contract drift.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_decision_score_matches_trace_decision_score() {
    // The trace's decision_score must equal the assessment envelope's
    // decision_score exactly — the API DTO consumes the envelope, the
    // Evidence Graph consumes the trace, and they must not disagree.
    let assessment = v2_1(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Travel,
    );
    let trace = assessment.trace.as_ref().expect("trace");
    assert_eq!(
        assessment.decision.decision_score, trace.decision.decision_score,
        "envelope and trace decision_score must agree for the API/EG contract"
    );
    assert_eq!(
        assessment.decision.bucket, trace.decision.bucket,
        "envelope and trace bucket must agree for the API/EG contract"
    );
}

#[test]
fn v2_1_trace_round_trips_through_serde() {
    let assessment = v2_1(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = assessment.trace.as_ref().expect("trace");

    let json = serde_json::to_string(trace).expect("serialize");
    let back: AssessmentTrace = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(trace, &back);
}

#[test]
fn v2_1_assessment_round_trips_through_serde() {
    // The full envelope — including the v2.1 trace — must survive a
    // serde round-trip so amlich-api DTOs and fixtures stay
    // byte-compatible with the v2.x wire shape.
    let assessment = v2_1(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let json = serde_json::to_string(&assessment).expect("serialize envelope");
    let back: PersonalDayAssessment = serde_json::from_str(&json).expect("deserialize envelope");
    assert_eq!(assessment, back);
}

// ---------------------------------------------------------------------------
// Acceptance criterion 4b: Hard veto precedence still wins under v2.1.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_hard_veto_still_overrides_intent_aware_weights() {
    // The 1985 birth year hits Hạn High on 2024-02-10 and fires
    // `veto.annual.han_severe`. v2.1's intent-aware weighting must
    // still bow to the hard veto: every intent must produce an Avoid
    // bucket with the override score, exactly like baseline_v2 / v1.
    let snapshot = snapshot_2024_02_10();
    let profile = han_severe_profile();

    for intent in ALL_INTENTS {
        let assessment = v2_1(&snapshot, &profile, intent);
        let trace = assessment.trace.as_ref().expect("trace");
        assert!(
            !trace.vetoes.is_empty(),
            "veto must still fire under v2.1 for {:?}",
            intent
        );
        assert_eq!(
            assessment.decision.bucket,
            RecommendationBucket::Avoid,
            "hard veto must force Avoid for {:?} regardless of intent weights",
            intent
        );
        assert_eq!(
            assessment.decision.semantic, "override_avoid",
            "veto override semantic must be preserved for {:?}",
            intent
        );
        // Override score is the fixed veto score; the weighted
        // aggregation does NOT perturb it.
        assert_eq!(
            assessment.decision.decision_score,
            Some(0.15),
            "veto override decision_score must be locked for {:?}",
            intent
        );
    }
}

#[test]
fn v2_1_recommendation_ky_manh_override_still_wins_over_weights() {
    // The KyManh recommendation veto (a hard veto that fires when the
    // Wedding activity lands in the KyManh bucket) must still win over
    // the v2.1 weighted aggregation. 2024-05-05 + Wedding fires
    // `veto.recommendation.ky_manh` (per
    // `v2_surfaces_named_veto_events_when_conditions_fire`), so the
    // veto override path is taken rather than the
    // non-veto KyManh override — the score is the veto override 0.15,
    // not the recommendation-override 0.2.
    let snapshot = snapshot_2024_05_05();
    let assessment = v2_1(&snapshot, &full_profile(), ConsultationIntent::Wedding);
    let trace = assessment.trace.as_ref().expect("trace");

    assert!(
        trace
            .vetoes
            .iter()
            .any(|v| v.veto_id == "veto.recommendation.ky_manh"),
        "KyManh veto must fire under v2.1 for Wedding on 2024-05-05"
    );
    assert_eq!(assessment.decision.bucket, RecommendationBucket::Avoid);
    assert_eq!(assessment.decision.semantic, "override_avoid");
    // The hard veto override path takes precedence over the
    // recommendation-override path; its score is the locked veto
    // override score 0.15, NOT 0.2. This is the v1/v2/v2.1 byte-parity
    // behavior for the veto path (locked by
    // `assessment_v2_seam::v1_v2_full_parity_*`).
    assert_eq!(assessment.decision.decision_score, Some(0.15));
}

// ---------------------------------------------------------------------------
// Acceptance criterion 5: Reviewed fixtures document intentional
// divergences.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_divergence_from_v2_is_only_in_decision_aggregation() {
    // The ONLY intentional v2.1 divergence from v2 is in the decision
    // aggregation: the axis_weights in the trace, the resulting
    // decision_score, and (occasionally) the bucket/semantic when the
    // weighted score crosses a classification threshold.
    //
    // For every non-veto fixture, this test pins exactly which fields
    // are allowed to differ and which must stay byte-identical.
    let snapshots = [
        snapshot_2024_02_10(),
        snapshot_2024_05_05(),
        snapshot_2024_12_25(),
    ];
    let profiles = [full_profile(), no_time_no_gender(), han_severe_profile()];

    for snapshot in &snapshots {
        for profile in &profiles {
            for intent in ALL_INTENTS {
                let v2 = v2(snapshot, profile, intent);
                let v2_1 = v2_1(snapshot, profile, intent);

                // MUST stay identical (intent-aware weights don't touch these):
                assert_eq!(v2.policy_id, v2_1.policy_id);
                assert_eq!(v2.ruleset_id, v2_1.ruleset_id);
                assert_eq!(v2.ruleset_version, v2_1.ruleset_version);
                assert_eq!(v2.profile, v2_1.profile);
                assert_eq!(v2.capability, v2_1.capability);
                assert_eq!(v2.capability_tier, v2_1.capability_tier);
                assert_eq!(v2.normalized_birth, v2_1.normalized_birth);
                assert_eq!(v2.intent, v2_1.intent);
                assert_eq!(v2.evidence, v2_1.evidence);
                assert_eq!(v2.unavailable_sections, v2_1.unavailable_sections);
                // Contributions are produced by feature extraction,
                // which is identical across v2 and v2.1 — every field
                // matches byte-for-byte except `policy_version`, which
                // carries the v2 vs v2.1 signal (same convention the
                // v1/v2 parity suite uses).
                assert_eq!(
                    v2.contributions.len(),
                    v2_1.contributions.len(),
                    "{intent:?}: contribution count diverged"
                );
                for (c2, c2_1) in v2.contributions.iter().zip(v2_1.contributions.iter()) {
                    assert_eq!(c2.contribution_id, c2_1.contribution_id);
                    assert_eq!(c2.axis, c2_1.axis);
                    assert_eq!(c2.polarity, c2_1.polarity);
                    assert_eq!(c2.strength, c2_1.strength);
                    assert_eq!(c2.policy_id, c2_1.policy_id);
                    assert_eq!(c2.ruleset_id, c2_1.ruleset_id);
                    assert_eq!(c2.ruleset_version, c2_1.ruleset_version);
                    assert_eq!(c2.source_evidence, c2_1.source_evidence);
                    assert_eq!(c2.availability, c2_1.availability);
                    assert_eq!(c2.note, c2_1.note);
                    assert_eq!(c2.policy_version, "v2");
                    assert_eq!(c2_1.policy_version, "v2.1");
                }
                // Axis subtotals are produced by the v1-parity axis
                // aggregation, identical across v2 and v2.1.
                for axis in AssessmentAxis::ALL {
                    let a = v2.axes.iter().find(|o| o.axis == axis).expect("v2 axis");
                    let b = v2_1
                        .axes
                        .iter()
                        .find(|o| o.axis == axis)
                        .expect("v2.1 axis");
                    assert_eq!(a.score, b.score);
                    assert_eq!(a.verdict, b.verdict);
                    assert_eq!(a.unavailable_reason, b.unavailable_reason);
                }

                // The policy_version intentionally diverges (v2 vs v2.1).
                assert_eq!(v2.policy_version, "v2");
                assert_eq!(v2_1.policy_version, "v2.1");

                // The decision axis_weights are allowed to differ
                // (that's the whole point of v2.1), and the
                // decision_score / bucket / semantic MAY differ when
                // the weighted score crosses a threshold. No
                // invariant assertion here — the divergence tests
                // above already lock the intent-awareness behavior.
            }
        }
    }
}

#[test]
fn v2_1_decision_score_is_a_renormalization_of_v2_subtotals() {
    // For a fixed (snapshot, profile, intent), the v2.1 decision_score
    // must equal Σ(intent_weight[axis] × v2_axis_score[axis]) over the
    // available axes, with weights renormalized. This locks the
    // numerical contract: any future refactor of the aggregation must
    // preserve this formula.
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();

    for intent in ALL_INTENTS {
        let v2 = v2(&snapshot, &profile, intent);
        let v2_1 = v2_1(&snapshot, &profile, intent);
        let expected_weights = INTENT_AXIS_WEIGHTS_V2_1.weights_for(intent);

        let mut total_weight = 0.0_f32;
        let mut weighted_sum = 0.0_f32;
        for (axis, weight) in [
            (
                AssessmentAxis::GenericDayQuality,
                expected_weights.generic_day_quality,
            ),
            (AssessmentAxis::IntentFit, expected_weights.intent_fit),
            (
                AssessmentAxis::PersonalAlignment,
                expected_weights.personal_alignment,
            ),
            (
                AssessmentAxis::AnnualPressure,
                expected_weights.annual_pressure,
            ),
        ] {
            if let Some(score) = v2
                .axes
                .iter()
                .find(|o| o.axis == axis)
                .and_then(|o| o.score)
            {
                total_weight += weight;
                weighted_sum += weight * score;
            }
        }

        let expected_score = (weighted_sum / total_weight).clamp(0.0, 1.0);
        let actual_score = v2_1.decision.decision_score.expect("v2.1 score");
        assert!(
            (expected_score - actual_score).abs() < 1e-5,
            "{intent:?}: expected decision_score {expected_score} (renormalized weighted sum of v2 subtotals), got {actual_score}"
        );
    }
}

// ---------------------------------------------------------------------------
// Determinism / idempotency: v2.1 is pure, like v2.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_evaluation_is_pure_and_idempotent() {
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();
    let intent = ConsultationIntent::ContractSigning;

    let a = v2_1(&snapshot, &profile, intent);
    let b = v2_1(&snapshot, &profile, intent);

    assert_eq!(a.axes, b.axes);
    assert_eq!(a.contributions, b.contributions);
    assert_eq!(a.decision, b.decision);
    assert_eq!(a.unavailable_sections, b.unavailable_sections);
    assert_eq!(a.evidence, b.evidence);
    assert_eq!(a.trace, b.trace);
}

// ---------------------------------------------------------------------------
// Sanity: the public re-exports used by downstream consumers (api,
// reasoning-graph, fixtures) are reachable and the table is exposed.
// ---------------------------------------------------------------------------

#[test]
fn v2_1_table_and_types_are_publicly_reachable() {
    // amlich-31oa (the stability gate) and amlich-8tdm (Evidence Graph
    // projection) will consume the v2.1 table directly to enumerate
    // divergences and to render weight-attributed explanations. Lock
    // the public surface here.
    let table: &IntentAxisWeightTable = &INTENT_AXIS_WEIGHTS_V2_1;
    let weights: IntentAxisWeights = table.weights_for(ConsultationIntent::Wedding);
    assert_eq!(weights.intent, ConsultationIntent::Wedding);
    assert!((weights.total() - 1.0).abs() < 1e-6);
}
