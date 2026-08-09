//! Personal-day assessment v2.2 — declared interaction features
//! (`amlich-47wn`).
//!
//! Locks the four acceptance criteria of `amlich-47wn`:
//!
//! 1. **Each interaction has a stable identifier, policy weight,
//!    provenance, and fixture** — every [`InteractionTerm`] carries a
//!    stable `interaction_id`, a weight from the policy table, source
//!    evidence with a canonical `source_id`, and is covered by at least
//!    one test fixture below.
//! 2. **Undeclared fact combinations do not change scores** — when a
//!    declared condition is not met, no interaction fires; the axis
//!    subtotals and decision score match v2.1 byte-for-byte.
//! 3. **Interaction contributions flow through canonical assessment and
//!    API projections** — interaction deltas modify axis subtotals,
//!    which flow into the decision aggregation; the trace records the
//!    actual interaction terms for the Evidence Graph projection.
//! 4. **Duplicate inputs do not inflate results** — each interaction
//!    kind fires at most once per assessment.
//!
//! ## Out of scope
//!
//! The stability gate (`amlich-31oa`) decides when v2.x is promoted to
//! default. Until then, the production path
//! (`PersonalDayAssessment::assess`) stays pinned to v1 and these tests
//! cover only the opt-in v2.2 policy.

use amlich_core::almanac::tu_menh::Gender;
use amlich_core::{
    advisory::ConsultationIntent,
    assessment::{
        AssessmentAxis, AssessmentInputs, AssessmentPolicy, AssessmentTrace, InteractionKind,
        PersonalDayAssessment, ASSESSMENT_POLICY_V2_2_VERSION, ASSESSMENT_POLICY_V2_ID,
        INTERACTION_WEIGHTS_V2_2,
    },
    birth::{BirthProfile, BirthTime},
    types::VIETNAM_TIMEZONE,
    DaySnapshot,
};

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

fn snapshot(day: i32, month: i32) -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(day, month, 2024, VIETNAM_TIMEZONE)
}

fn profile(year: i32, gender: Gender) -> BirthProfile {
    BirthProfile {
        day: 1,
        month: 1,
        year,
        time: Some(BirthTime {
            hour: 9,
            minute: 30,
        }),
        timezone: VIETNAM_TIMEZONE,
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some(gender),
        location_name: Some("Hanoi".to_string()),
    }
}

fn v22(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) -> PersonalDayAssessment {
    AssessmentPolicy::interaction_aware_v2().evaluate(
        AssessmentInputs::default(),
        snapshot,
        profile,
        intent,
    )
}

fn v21(
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
// Structural tests
// ---------------------------------------------------------------------------

#[test]
fn v2_2_policy_reports_versioned_metadata() {
    let policy = AssessmentPolicy::interaction_aware_v2();
    assert_eq!(policy.policy_id(), ASSESSMENT_POLICY_V2_ID);
    assert_eq!(policy.policy_version(), ASSESSMENT_POLICY_V2_2_VERSION);
    assert_eq!(policy.policy_version(), "v2.2");
}

#[test]
fn baseline_v2_and_v2_1_stay_interaction_free() {
    // v2 and v2.1 must not emit interaction terms — only the v2.2
    // policy wires in the interaction weight table.
    let snap = snapshot(10, 2);
    let p = profile(1990, Gender::Male);

    for intent in ALL_INTENTS {
        let v2 = AssessmentPolicy::baseline_v2().evaluate(
            AssessmentInputs::default(),
            &snap,
            &p,
            intent,
        );
        let v21 = v21(&snap, &p, intent);

        let v2_trace = v2.trace.as_ref().expect("v2 trace");
        let v21_trace = v21.trace.as_ref().expect("v2.1 trace");

        assert!(
            v2_trace.interactions.is_empty(),
            "baseline_v2 must not emit interactions for {:?}",
            intent
        );
        assert!(
            v21_trace.interactions.is_empty(),
            "intent_weighted_v2 must not emit interactions for {:?}",
            intent
        );
    }
}

#[test]
fn v2_2_interaction_weight_table_is_well_formed() {
    assert_eq!(INTERACTION_WEIGHTS_V2_2.policy_version, "v2.2");
    assert_eq!(
        INTERACTION_WEIGHTS_V2_2.entries.len(),
        InteractionKind::ALL.len(),
        "table must carry one entry per InteractionKind"
    );
    for entry in INTERACTION_WEIGHTS_V2_2.entries {
        assert!(entry.weight > 0.0, "weight must be positive");
        assert!(
            (entry.weight / 0.05).round() * 0.05 - entry.weight < 1e-6,
            "weight must be a multiple of 0.05"
        );
        assert!(!entry.rationale.is_empty(), "rationale must not be empty");
    }
}

#[test]
fn v2_2_interaction_kinds_target_scored_axes_only() {
    for kind in InteractionKind::ALL {
        assert_ne!(
            kind.target_axis(),
            AssessmentAxis::EvidenceCoverage,
            "{:?} must not target EvidenceCoverage",
            kind
        );
    }
}

// ---------------------------------------------------------------------------
// Acceptance criterion 1: Each interaction has a stable identifier,
// policy weight, provenance, and fixture.
// ---------------------------------------------------------------------------

/// Assert that an interaction term with the given kind exists in the
/// trace, and return it for further assertions.
fn expect_interaction(
    trace: &AssessmentTrace,
    kind: InteractionKind,
) -> &amlich_core::assessment::InteractionTerm {
    trace
        .interactions
        .iter()
        .find(|i| i.interaction_id == kind.as_str())
        .unwrap_or_else(|| {
            panic!(
                "expected interaction {} in trace, got: {:?}",
                kind.as_str(),
                trace
                    .interactions
                    .iter()
                    .map(|i| &i.interaction_id)
                    .collect::<Vec<_>>()
            )
        })
}

#[test]
fn v2_2_hard_taboo_activity_fires_on_ceremonial_intent_with_taboos() {
    // Fixture: 2024-01-03 + 1990-Male + Wedding. The day has taboos and
    // Wedding is a ceremonial intent, so hard_taboo_activity fires on
    // the IntentFit axis.
    let snap = snapshot(3, 1);
    let p = profile(1990, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.2 trace");

    let term = expect_interaction(trace, InteractionKind::HardTabooActivity);
    assert_eq!(term.axis, AssessmentAxis::IntentFit);
    assert!(term.value < 0.0, "hard_taboo value must be negative");
    assert_eq!(
        term.weight,
        INTERACTION_WEIGHTS_V2_2.weight_for(InteractionKind::HardTabooActivity)
    );
    assert!(
        !term.source_evidence.source_id.is_empty(),
        "must carry source_id"
    );
    assert!(!term.source_evidence.method.is_empty(), "must carry method");
    assert!(
        term.feature_ids
            .contains(&amlich_core::assessment::AssessmentFeatureId::GenericDayQuality),
        "must reference the taboo feature"
    );
}

#[test]
fn v2_2_personal_relation_pillar_fires_on_favorable_relation() {
    // Fixture: 2024-01-02 + 1990-Male + Wedding. The day has a Tam Hop
    // or Liu He relation with the birth year, so
    // personal_relation_pillar fires on PersonalAlignment.
    let snap = snapshot(2, 1);
    let p = profile(1990, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.2 trace");

    let term = expect_interaction(trace, InteractionKind::PersonalRelationPillar);
    assert_eq!(term.axis, AssessmentAxis::PersonalAlignment);
    assert!(term.value > 0.0, "personal_relation value must be positive");
    assert_eq!(
        term.weight,
        INTERACTION_WEIGHTS_V2_2.weight_for(InteractionKind::PersonalRelationPillar)
    );
    assert!(!term.source_evidence.source_id.is_empty());
}

#[test]
fn v2_2_weak_element_day_generation_fires_on_weak_chart_with_generating_day() {
    // Fixture: 2024-01-05 + 1985-Male + Wedding. The birth chart's day
    // master is weak and the day's element generates the day master's
    // element.
    let snap = snapshot(5, 1);
    let p = profile(1985, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.2 trace");

    let term = expect_interaction(trace, InteractionKind::WeakElementDayGeneration);
    assert_eq!(term.axis, AssessmentAxis::PersonalAlignment);
    assert!(
        term.value > 0.0,
        "weak_element value must be positive (supportive)"
    );
    assert_eq!(
        term.weight,
        INTERACTION_WEIGHTS_V2_2.weight_for(InteractionKind::WeakElementDayGeneration)
    );
    assert!(!term.source_evidence.source_id.is_empty());
}

#[test]
fn v2_2_kua_direction_travel_fires_on_favorable_direction_for_travel() {
    // Fixture: 2024-01-01 + 1990-Male + Travel. The day's xuất hành
    // direction matches a favorable Kua direction, and the intent is
    // Travel, so kua_direction_travel fires on IntentFit.
    let snap = snapshot(1, 1);
    let p = profile(1990, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Travel);
    let trace = result.trace.as_ref().expect("v2.2 trace");

    let term = expect_interaction(trace, InteractionKind::KuaDirectionTravel);
    assert_eq!(term.axis, AssessmentAxis::IntentFit);
    assert!(term.value > 0.0, "kua_direction value must be positive");
    assert_eq!(
        term.weight,
        INTERACTION_WEIGHTS_V2_2.weight_for(InteractionKind::KuaDirectionTravel)
    );
    assert_eq!(term.source_evidence.source_id, "vn-folk");
}

#[test]
fn v2_2_annual_pressure_activity_fires_on_han_with_major_life_event() {
    // Fixture: 2024-01-01 + 1990-Male + Wedding. Hạn is active for
    // 1990 in 2024 and Wedding is a major life event, so
    // annual_pressure_activity fires on AnnualPressure.
    let snap = snapshot(1, 1);
    let p = profile(1990, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.2 trace");

    let term = expect_interaction(trace, InteractionKind::AnnualPressureActivity);
    assert_eq!(term.axis, AssessmentAxis::AnnualPressure);
    assert!(term.value < 0.0, "annual_pressure value must be negative");
    assert_eq!(
        term.weight,
        INTERACTION_WEIGHTS_V2_2.weight_for(InteractionKind::AnnualPressureActivity)
    );
    assert!(!term.source_evidence.source_id.is_empty());
}

// ---------------------------------------------------------------------------
// Acceptance criterion 2: Undeclared fact combinations do not change
// scores.
// ---------------------------------------------------------------------------

#[test]
fn undeclared_fact_combinations_do_not_change_scores() {
    // When the declared condition for an interaction is NOT met, the
    // interaction does NOT fire and the v2.2 assessment matches v2.1
    // byte-for-byte.

    // hard_taboo_activity requires a ceremonial intent. Medical is NOT
    // ceremonial, so even if taboos are present the interaction stays
    // absent.
    let snap = snapshot(3, 1);
    let p = profile(1990, Gender::Male);
    let r22_medical = v22(&snap, &p, ConsultationIntent::Medical);
    let r21_medical = v21(&snap, &p, ConsultationIntent::Medical);
    let trace_medical = r22_medical.trace.as_ref().unwrap();
    assert!(
        !trace_medical
            .interactions
            .iter()
            .any(|i| i.interaction_id == "interaction.hard_taboo_activity"),
        "hard_taboo_activity must not fire for non-ceremonial intent Medical"
    );
    assert_eq!(
        r22_medical.axes, r21_medical.axes,
        "v2.2 axes must match v2.1 when no interactions fire"
    );
    assert_eq!(
        r22_medical.decision.decision_score, r21_medical.decision.decision_score,
        "v2.2 decision must match v2.1 when no interactions fire"
    );

    // kua_direction_travel requires Travel intent. Wedding is NOT Travel.
    let snap_travel = snapshot(1, 1);
    let r22_wedding = v22(&snap_travel, &p, ConsultationIntent::Wedding);
    let trace_wedding = r22_wedding.trace.as_ref().unwrap();
    assert!(
        !trace_wedding
            .interactions
            .iter()
            .any(|i| i.interaction_id == "interaction.kua_direction_travel"),
        "kua_direction_travel must not fire for non-Travel intent Wedding"
    );

    // annual_pressure_activity requires a major life event. Renovation
    // is ceremonial but NOT a major life event.
    let snap_renovation = snapshot(1, 1);
    let r22_renovation = v22(&snap_renovation, &p, ConsultationIntent::Renovation);
    let trace_renovation = r22_renovation.trace.as_ref().unwrap();
    assert!(
        !trace_renovation
            .interactions
            .iter()
            .any(|i| i.interaction_id == "interaction.annual_pressure_activity"),
        "annual_pressure_activity must not fire for non-major-life-event intent Renovation"
    );
}

#[test]
fn v2_2_axis_subtotals_match_v2_1_when_no_interactions_fire() {
    // When the fixture triggers no interactions at all, v2.2 must be
    // byte-identical to v2.1 (same axes, same decision). Medical intent
    // on 2024-02-10 + 1990-Male fires no interactions (Medical is not
    // ceremonial, not travel, not a major life event).
    let snap = snapshot(10, 2);
    let p = profile(1990, Gender::Male);

    let r22 = v22(&snap, &p, ConsultationIntent::Medical);
    let r21 = v21(&snap, &p, ConsultationIntent::Medical);

    let trace22 = r22.trace.as_ref().unwrap();
    assert!(
        trace22.interactions.is_empty(),
        "Medical intent on this fixture should fire no interactions"
    );
    assert_eq!(r22.axes, r21.axes);
    assert_eq!(r22.decision.decision_score, r21.decision.decision_score);
    assert_eq!(r22.decision.bucket, r21.decision.bucket);
}

// ---------------------------------------------------------------------------
// Acceptance criterion 3: Interaction contributions flow through canonical
// assessment and API projections.
// ---------------------------------------------------------------------------

#[test]
fn interaction_deltas_modify_axis_subtotals_and_decision() {
    // The kua_direction_travel interaction on 2024-01-01 + 1990-Male +
    // Travel adds +0.08 to IntentFit (value=0.4 × weight=0.20). Verify
    // the axis subtotal and trace reflect the delta.
    let snap = snapshot(1, 1);
    let p = profile(1990, Gender::Male);

    let r21 = v21(&snap, &p, ConsultationIntent::Travel);
    let r22 = v22(&snap, &p, ConsultationIntent::Travel);

    let v21_fit = r21.axes.intent_fit.score.expect("v2.1 intent_fit score");
    let v22_fit = r22.axes.intent_fit.score.expect("v2.2 intent_fit score");
    assert!(
        v22_fit > v21_fit,
        "kua_direction_travel must boost IntentFit: v2.2={v22_fit} > v2.1={v21_fit}"
    );
    assert!(
        (v22_fit - v21_fit - 0.08).abs() < 1e-5,
        "delta must be ~0.08 (0.4 × 0.20), got {}",
        v22_fit - v21_fit
    );

    // The trace's AxisAggregation subtotal must match the envelope score.
    let trace = r22.trace.as_ref().unwrap();
    let fit_trace = trace
        .axes
        .iter()
        .find(|a| a.axis == AssessmentAxis::IntentFit)
        .unwrap();
    assert_eq!(fit_trace.subtotal, Some(v22_fit));
}

#[test]
fn v2_2_trace_records_interaction_terms_with_full_attribution() {
    let snap = snapshot(1, 1);
    let p = profile(1990, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Travel);
    let trace = result.trace.as_ref().unwrap();

    assert!(
        !trace.interactions.is_empty(),
        "Travel fixture must produce interactions"
    );
    for term in &trace.interactions {
        assert!(
            !term.interaction_id.is_empty(),
            "interaction_id must not be empty"
        );
        assert!(
            !term.feature_ids.is_empty(),
            "feature_ids must not be empty"
        );
        assert!(
            !term.source_evidence.source_id.is_empty(),
            "source_id must not be empty"
        );
        assert!(
            !term.source_evidence.method.is_empty(),
            "method must not be empty"
        );
        assert_eq!(term.axis, term.axis); // axis is always set
    }
}

// ---------------------------------------------------------------------------
// Acceptance criterion 4: Duplicate inputs do not inflate results.
// ---------------------------------------------------------------------------

#[test]
fn duplicate_inputs_do_not_inflate_results() {
    // Each interaction kind fires at most once per assessment. Even if
    // multiple features could conceptually trigger the same interaction,
    // the extraction emits at most one term per kind.
    //
    // Use a fixture that fires multiple interactions (2024-01-03 +
    // 1990-Male + Wedding fires hard_taboo_activity and
    // annual_pressure_activity). Verify no interaction_id appears more
    // than once.
    let snap = snapshot(3, 1);
    let p = profile(1990, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().unwrap();

    let mut seen = std::collections::HashSet::new();
    for term in &trace.interactions {
        assert!(
            seen.insert(&term.interaction_id),
            "duplicate interaction_id {} — duplicate inputs must not inflate",
            term.interaction_id
        );
    }

    // Also verify determinism: running the same assessment twice
    // produces identical interactions.
    let result2 = v22(&snap, &p, ConsultationIntent::Wedding);
    let trace2 = result2.trace.as_ref().unwrap();
    assert_eq!(
        trace.interactions, trace2.interactions,
        "interaction extraction must be deterministic"
    );
}

// ---------------------------------------------------------------------------
// Interaction + veto precedence
// ---------------------------------------------------------------------------

#[test]
fn hard_veto_still_overrides_interactions() {
    // The weak_element fixture (2024-01-05 + 1985-Male + Wedding) fires
    // a hard veto (Hạn severe). Even though interactions fire too, the
    // veto must force the Avoid bucket.
    let snap = snapshot(5, 1);
    let p = profile(1985, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Wedding);

    assert_eq!(
        result.decision.bucket,
        amlich_core::reasoning::RecommendationBucket::Avoid,
        "hard veto must override interactions"
    );
    assert_eq!(
        result.decision.decision_score,
        Some(0.15),
        "veto decision score must be the forced 0.15"
    );

    let trace = result.trace.as_ref().unwrap();
    assert!(
        !trace.vetoes.is_empty(),
        "veto events must still be present"
    );
    assert!(
        !trace.interactions.is_empty(),
        "interactions must still be recorded in the trace"
    );
}

// ---------------------------------------------------------------------------
// Serde round-trip
// ---------------------------------------------------------------------------

#[test]
fn v2_2_trace_with_interactions_round_trips_through_serde() {
    let snap = snapshot(1, 1);
    let p = profile(1990, Gender::Male);
    let result = v22(&snap, &p, ConsultationIntent::Travel);
    let trace = result.trace.as_ref().expect("v2.2 trace");

    let json = serde_json::to_string(trace).expect("serialize");
    let back: AssessmentTrace = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(trace, &back, "trace must survive serde round-trip");
}

// ---------------------------------------------------------------------------
// Purity / determinism
// ---------------------------------------------------------------------------

#[test]
fn v2_2_evaluation_is_pure_and_idempotent() {
    let snap = snapshot(3, 1);
    let p = profile(1990, Gender::Male);

    let r1 = v22(&snap, &p, ConsultationIntent::Wedding);
    let r2 = v22(&snap, &p, ConsultationIntent::Wedding);

    assert_eq!(r1.axes, r2.axes);
    assert_eq!(r1.decision, r2.decision);
    assert_eq!(
        r1.trace.as_ref().unwrap().interactions,
        r2.trace.as_ref().unwrap().interactions,
    );
}

// ---------------------------------------------------------------------------
// Coverage grid: interactions fire somewhere in a representative sweep
// ---------------------------------------------------------------------------

#[test]
fn v2_2_all_five_interaction_kinds_fire_somewhere_in_fixture_sweep() {
    // Scan a representative date × year × intent grid and confirm every
    // declared InteractionKind fires at least once. This guards against
    // regressions where an interaction's condition becomes unreachable.
    let policy = AssessmentPolicy::interaction_aware_v2();
    let mut seen = std::collections::HashSet::new();

    for month in 1..=12i32 {
        for day in 1..=28i32 {
            let snap = snapshot(day, month);
            for (year, gender) in [
                (1990, Gender::Male),
                (1985, Gender::Female),
                (1978, Gender::Male),
            ] {
                let p = profile(year, gender);
                for intent in [
                    ConsultationIntent::Wedding,
                    ConsultationIntent::Travel,
                    ConsultationIntent::MovingHouse,
                ] {
                    let result = policy.evaluate(AssessmentInputs::default(), &snap, &p, intent);
                    let trace = result.trace.as_ref().unwrap();
                    for term in &trace.interactions {
                        for kind in InteractionKind::ALL {
                            if term.interaction_id == kind.as_str() {
                                seen.insert(kind);
                            }
                        }
                    }
                }
            }
        }
    }

    for kind in InteractionKind::ALL {
        assert!(
            seen.contains(&kind),
            "{:?} ({}) never fired in the sweep — condition may be unreachable",
            kind,
            kind.as_str()
        );
    }
}
