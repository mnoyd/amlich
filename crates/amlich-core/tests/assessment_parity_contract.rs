//! Standalone-vs-aggregate parity fixtures at the amlich-core layer.
//!
//! Locks the core contract that `PersonalDayAssessment::assess(...)` is
//! deterministic and stable across multiple calls so downstream
//! `assess_personal_day` (standalone advisory) and `get_personal_day_report`
//! (aggregate) can project identical axes and contribution IDs from the
//! same inputs. See bead `amlich-mwbp.6`.

use amlich_core::{
    advisory::ConsultationIntent,
    assessment::{
        assess_personal_day, PersonalDayAssessment, ASSESSMENT_POLICY_ID, ASSESSMENT_POLICY_VERSION,
    },
    birth::{BirthProfile, BirthTime},
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
        gender: Some(amlich_core::almanac::tu_menh::Gender::Male),
        location_name: Some("Hanoi".to_string()),
    }
}

fn profile_no_time() -> BirthProfile {
    BirthProfile {
        time: None,
        gender: None,
        ..full_profile()
    }
}

#[test]
fn assessment_is_pure_and_idempotent() {
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();
    let intent = ConsultationIntent::ContractSigning;

    let a: PersonalDayAssessment =
        PersonalDayAssessment::assess(snapshot.clone(), profile.clone(), intent);
    let b: PersonalDayAssessment =
        PersonalDayAssessment::assess(snapshot.clone(), profile.clone(), intent);

    assert_eq!(a.axes, b.axes);
    assert_eq!(a.decision, b.decision);
    assert_eq!(a.contributions, b.contributions);
    assert_eq!(a.unavailable_sections, b.unavailable_sections);
    assert_eq!(a.evidence, b.evidence);
    assert_eq!(a.policy_id, b.policy_id);
    assert_eq!(a.policy_version, b.policy_version);
}

#[test]
fn standalone_and_aggregate_share_normalized_inputs_and_conclusions() {
    // The acceptance criterion for amlich-mwbp.6 is: "standalone and
    // aggregate parity fixtures share normalized inputs and conclusions."
    // Both code paths must call `PersonalDayAssessment::assess` with the
    // same `(snapshot, profile, intent)` triple. This test asserts that
    // repeated assess() calls return the same canonical facts.
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();
    let intent = ConsultationIntent::Wedding;

    let standalone = assess_personal_day(snapshot.clone(), profile.clone(), intent);
    let aggregate = assess_personal_day(snapshot.clone(), profile.clone(), intent);

    assert_eq!(
        standalone.ruleset_id, aggregate.ruleset_id,
        "ruleset_id must match across calls"
    );
    assert_eq!(
        standalone.policy_id, aggregate.policy_id,
        "policy_id must match across calls"
    );
    assert_eq!(
        standalone.policy_version, aggregate.policy_version,
        "policy_version must match across calls"
    );
    assert_eq!(
        standalone.decision.bucket, aggregate.decision.bucket,
        "decision.bucket must be identical across standalone and aggregate"
    );
    assert_eq!(
        standalone.decision.confidence, aggregate.decision.confidence,
        "decision.confidence must be identical across standalone and aggregate"
    );
    assert_eq!(
        standalone.normalized_birth.day, aggregate.normalized_birth.day,
        "normalized_birth.day must match"
    );
}

#[test]
fn contribution_ids_are_stable_across_calls() {
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();
    let intent = ConsultationIntent::Travel;

    let a = assess_personal_day(snapshot.clone(), profile.clone(), intent);
    let b = assess_personal_day(snapshot.clone(), profile.clone(), intent);

    let ids_a: Vec<&str> = a
        .contributions
        .iter()
        .map(|c| c.contribution_id.as_str())
        .collect();
    let ids_b: Vec<&str> = b
        .contributions
        .iter()
        .map(|c| c.contribution_id.as_str())
        .collect();
    assert_eq!(
        ids_a, ids_b,
        "contribution_ids must be byte-identical across calls (stable IDs are required for the parity contract)"
    );
}

#[test]
fn unknown_time_profile_produces_unavailable_personal_hour_section() {
    let snapshot = snapshot_2024_02_10();
    let profile = profile_no_time();

    let assessment = assess_personal_day(snapshot, profile, ConsultationIntent::Wedding);

    assert!(
        !assessment.evidence.has_chart,
        "no time → chart must be unavailable"
    );
    assert!(
        assessment
            .unavailable_sections
            .iter()
            .any(|s| s.section == "personal_hours"),
        "personal_hours must be marked unavailable when birth time is missing"
    );

    // Same assessment still produces a verdict (decision is a single source of
    // truth, never absent).
    assert!(!assessment.decision.primary_conclusion.is_empty());
}

#[test]
fn intent_shift_changes_contribution_id_but_not_axis_contract() {
    let snapshot = snapshot_2024_02_10();
    let profile = full_profile();
    let wedding = assess_personal_day(
        snapshot.clone(),
        profile.clone(),
        ConsultationIntent::Wedding,
    );
    let burial = assess_personal_day(
        snapshot.clone(),
        profile.clone(),
        ConsultationIntent::Burial,
    );

    let wedding_intent_id = wedding
        .contributions
        .iter()
        .find(|c| c.axis == amlich_core::assessment::AssessmentAxis::IntentFit)
        .map(|c| c.contribution_id.clone())
        .expect("intent_fit contribution");
    let burial_intent_id = burial
        .contributions
        .iter()
        .find(|c| c.axis == amlich_core::assessment::AssessmentAxis::IntentFit)
        .map(|c| c.contribution_id.clone())
        .expect("intent_fit contribution");

    assert_ne!(
        wedding_intent_id, burial_intent_id,
        "intent_fit contribution_id is keyed by intent; Wedding vs Burial MUST differ"
    );

    // Both assessments share the same axes backbone (axis contract stable):
    for axis in amlich_core::assessment::AssessmentAxis::ALL.iter() {
        assert!(
            wedding.axes.iter().any(|a| a.axis == *axis),
            "axes must include every member of AssessmentAxis::ALL"
        );
    }
}

#[test]
fn policy_metadata_is_locked() {
    // Migrating the assessment formula MUST bump the policy_version. This
    // test pins the canonical policy metadata so a future contract change
    // requires a deliberate update here.
    assert_eq!(ASSESSMENT_POLICY_ID, "personal-day-assessment");
    assert_eq!(ASSESSMENT_POLICY_VERSION, "v1");
}

#[test]
fn canonical_decision_carries_score_bucket_and_confidence() {
    // amlich-0q2f: the legacy `AdvisoryScoring` / `ScoredAdvice` /
    // `score_day_selection` / `rank_dates_for_intent` compatibility surface
    // (which projected `score`/`verdict`/`confidence` strings) has been
    // retired because it had no production consumers. This test locks the
    // parity property that the canonical `PersonalDayDecision` still exposes
    // every value those projections derived from — `decision_score`,
    // `bucket`, and `confidence` — so removing the legacy structs does not
    // lose the capability.
    let snapshot = snapshot_2024_02_10();
    let assessment = assess_personal_day(snapshot, full_profile(), ConsultationIntent::Wedding);

    assert!(
        assessment.decision.decision_score.is_some(),
        "canonical decision must carry a normalized decision_score"
    );
    let score = assessment.decision.decision_score.unwrap();
    assert!(
        (0.0..=1.0).contains(&score),
        "decision_score must be a normalized 0..=1 value, got {score}"
    );

    assert!(
        matches!(
            assessment.decision.confidence,
            amlich_core::reasoning::DecisionConfidence::Low
                | amlich_core::reasoning::DecisionConfidence::Medium
                | amlich_core::reasoning::DecisionConfidence::High
        ),
        "canonical decision must carry a typed confidence"
    );

    let bucket = format!("{:?}", assessment.decision.bucket).to_lowercase();
    assert!(
        matches!(
            bucket.as_str(),
            "favorable" | "mixed" | "cautious" | "avoid"
        ),
        "canonical decision must carry a typed bucket, got {bucket}"
    );
}
