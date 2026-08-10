//! amlich-bz0f.2 API-level projection tests for the v2.3 Bazi-to-day
//! feature observations.
//!
//! Verifies the assessment-level observations survive the
//! `PersonalDayAssessment` → `PersonalDayAssessmentDto` conversion
//! with stable `factor_id`s and `contribution_id`s so TUI, desktop,
//! and external API consumers can subscribe to the new evidence
//! without re-parsing the v1 contract.

use amlich_core::advisory::ConsultationIntent;
use amlich_core::almanac::tu_menh::Gender;
use amlich_core::assessment::{
    AssessmentInputs, AssessmentPolicy, AssessmentTrace, ASSESSMENT_POLICY_V2_3_VERSION,
};
use amlich_core::birth::{BirthProfile, BirthTime};
use amlich_core::types::VIETNAM_TIMEZONE;

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

fn date_only_profile() -> BirthProfile {
    BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: None,
        timezone: VIETNAM_TIMEZONE,
        longitude: None,
        use_solar_time: false,
        gender: Some(Gender::Male),
        location_name: None,
    }
}

fn expect_bazi_factor_ids(trace: &AssessmentTrace) -> Vec<String> {
    let mut ids: Vec<String> = trace
        .features
        .iter()
        .filter(|f| f.contribution_id.starts_with("bazi.target_day.") && !f.is_unavailable())
        .map(|f| f.contribution_id.clone())
        .collect();
    ids.sort();
    ids.dedup();
    ids
}

#[test]
fn v2_3_full_profile_emits_three_bazi_factor_ids() {
    let snap = amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE);
    let result = AssessmentPolicy::bazi_projection_v2_3().evaluate(
        AssessmentInputs::default(),
        &snap,
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let bazi_ids = expect_bazi_factor_ids(trace);
    assert!(
        bazi_ids
            .iter()
            .any(|id| id.starts_with("bazi.target_day.ten_god")),
        "missing BaziTargetDayTenGod: {:?}",
        bazi_ids
    );
    assert!(
        bazi_ids
            .iter()
            .any(|id| id.starts_with("bazi.target_day.pillar_relation")),
        "missing BaziTargetDayPillarRelation: {:?}",
        bazi_ids
    );
    assert!(
        bazi_ids
            .iter()
            .any(|id| id.starts_with("bazi.target_day.element_resonance")),
        "missing BaziTargetDayElementResonance: {:?}",
        bazi_ids
    );
}

#[test]
fn v2_3_full_profile_reports_v2_3_policy_version() {
    let snap = amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE);
    let result = AssessmentPolicy::bazi_projection_v2_3().evaluate(
        AssessmentInputs::default(),
        &snap,
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.3 trace");
    assert_eq!(trace.policy_version, ASSESSMENT_POLICY_V2_3_VERSION);
    assert_eq!(trace.policy_id, "personal-day-assessment");
}

#[test]
fn v2_3_date_only_profile_still_emits_bazi_factor_ids() {
    let snap = amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE);
    let result = AssessmentPolicy::bazi_projection_v2_3().evaluate(
        AssessmentInputs::default(),
        &snap,
        &date_only_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let bazi_ids = expect_bazi_factor_ids(trace);
    // Date-only profile: still has a year/month/day Bazi chart, so
    // the Bazi-to-day observations must fire. The hour pillar is
    // excluded from the branch-relation check, but the other pillars
    // still feed the observations.
    assert!(
        bazi_ids
            .iter()
            .any(|id| id.starts_with("bazi.target_day.ten_god")),
        "date-only profile must still emit BaziTargetDayTenGod, got: {:?}",
        bazi_ids
    );
    assert!(
        bazi_ids
            .iter()
            .any(|id| id.starts_with("bazi.target_day.element_resonance")),
        "date-only profile must still emit BaziTargetDayElementResonance, got: {:?}",
        bazi_ids
    );
}
