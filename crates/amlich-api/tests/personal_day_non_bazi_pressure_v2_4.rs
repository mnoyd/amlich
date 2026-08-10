//! amlich-bz0f.3 API-level projection tests for the v2.4 non-Bazi
//! annual pressure observations.
//!
//! Verifies the per-system observations survive the
//! `PersonalDayAssessment` → `PersonalDayAssessmentDto` conversion
//! with stable `contribution_id`s and source attribution so TUI,
//! desktop, and external API consumers can subscribe to the new
//! evidence without re-parsing the v1 / v2.3 catch-all contract.

use amlich_api::{get_personal_day_report, DateQuery, PersonalDayAssessmentDto};
use amlich_core::advisory::ConsultationIntent;
use amlich_core::almanac::tu_menh::Gender;
use amlich_core::assessment::{AssessmentInputs, AssessmentPolicy, ASSESSMENT_POLICY_V2_4_VERSION};
use amlich_core::birth::{BirthProfile, BirthTime};
use amlich_core::types::VIETNAM_TIMEZONE;

fn ng0_full_profile() -> BirthProfile {
    BirthProfile {
        day: 15,
        month: 6,
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

#[test]
fn v2_4_per_system_observations_round_trip_through_dto() {
    // 2026 (Ngọ chi) for a 1990 Ngọ birth: Kim Lâu, Thái Tuế, sao hạn.
    let snap = amlich_core::calculate_day_snapshot_with_timezone(15, 6, 2026, VIETNAM_TIMEZONE);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_full_profile(),
        ConsultationIntent::Wedding,
    );
    let dto = PersonalDayAssessmentDto::from(&result);

    // Source-family classification is preserved through the DTO.
    let kim_lau = dto
        .contributions
        .iter()
        .find(|c| c.contribution_id == "annual.kim_lau")
        .expect("Kim Lâu contribution survives DTO conversion");
    assert_eq!(kim_lau.source_family, "almanac_rule");
    assert_eq!(kim_lau.source_id, "ngoc-hap-ky");
    assert_eq!(kim_lau.policy_version, ASSESSMENT_POLICY_V2_4_VERSION);

    let sao_han = dto
        .contributions
        .iter()
        .find(|c| c.contribution_id == "annual.sao_han")
        .expect("sao hạn contribution survives DTO conversion");
    assert_eq!(sao_han.source_id, "cuu-dieu");
}

#[test]
fn v2_4_factors_round_trip_through_json() {
    let snap = amlich_core::calculate_day_snapshot_with_timezone(15, 6, 2026, VIETNAM_TIMEZONE);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_full_profile(),
        ConsultationIntent::Wedding,
    );
    let dto = PersonalDayAssessmentDto::from(&result);
    let json = serde_json::to_value(&dto).expect("serialize dto");

    let factors = json["factors"].as_array().expect("factor array");
    assert!(
        factors.iter().any(|f| f["factor_id"] == "annual.kim_lau"),
        "Kim Lâu factor missing from JSON: {:?}",
        factors
    );
    assert!(
        factors.iter().any(|f| f["factor_id"] == "annual.sao_han"),
        "sao hạn factor missing from JSON"
    );
}

#[test]
fn anonymous_report_still_uses_legacy_v1_default_path() {
    // The default anonymous path (no policy opt-in) must remain on
    // the v1 builder, so the catch-all aggregation observation is
    // still emitted. v2.4 is opt-in only.
    let query = DateQuery {
        day: 15,
        month: 6,
        year: 2026,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    let report =
        get_personal_day_report(&query, None, None, None, None).expect("anonymous report builds");
    let assessment = report
        .canonical_assessment
        .as_ref()
        .expect("canonical assessment present");
    // Legacy v1 path still uses the single catch-all id under v2.3
    // and earlier (no `annual.kim_lau` / `annual.sao_han` ids).
    let factor_ids: Vec<&str> = assessment
        .factors
        .iter()
        .map(|f| f.factor_id.as_str())
        .collect();
    assert!(
        !factor_ids
            .iter()
            .any(|id| id.starts_with("annual.kim_lau") || id.starts_with("annual.sao_han")),
        "v1 default path must not leak v2.4 per-system ids: {:?}",
        factor_ids
    );
}
