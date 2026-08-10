//! amlich-bz0f.3 core-level tests for the v2.4 non-Bazi annual
//! pressure projection.
//!
//! Verifies the v2.4 policy emits one typed, source-attributed
//! observation per active annual system (Tam Tai, Kim Lâu, Hoàng Ốc,
//! Thái Tuế, sao hạn) and that the catch-all aggregation observation
//! emitted by v2 / v2.1 / v2.2 / v2.3 is removed.
//!
//! Parity fixtures cover the three required cases:
//!   - favorable: zero active systems (Low severity),
//!   - conflicting: multiple active systems (Critical severity),
//!   - unavailable: profile without gender (no yearly Hạn assessment).

use amlich_core::advisory::ConsultationIntent;
use amlich_core::almanac::tu_menh::Gender;
use amlich_core::assessment::{
    AssessmentFeatureId, AssessmentInputs, AssessmentPolicy, AssessmentTrace, AvailabilityState,
    ASSESSMENT_POLICY_V2_3_VERSION, ASSESSMENT_POLICY_V2_4_VERSION,
};
use amlich_core::birth::{BirthProfile, BirthTime};
use amlich_core::calculate_day_snapshot_with_timezone;
use amlich_core::types::VIETNAM_TIMEZONE;

/// Birth date that converts to lunar year 1990 (Ngọ), so the
/// classical Hạn fixtures below match the probed yearly states.
fn ng0_birth_profile() -> BirthProfile {
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

fn snapshot_for(year: i32) -> amlich_core::DaySnapshot {
    calculate_day_snapshot_with_timezone(15, 6, year, VIETNAM_TIMEZONE)
}

fn feature_ids(trace: &AssessmentTrace) -> Vec<String> {
    let mut ids: Vec<String> = trace
        .features
        .iter()
        .map(|f| f.feature_id.as_str().to_string())
        .collect();
    ids.sort();
    ids.dedup();
    ids
}

fn contribution_ids_on_annual_pressure(trace: &AssessmentTrace) -> Vec<String> {
    use amlich_core::assessment::AssessmentAxis;
    let mut ids: Vec<String> = trace
        .features
        .iter()
        .filter(|f| f.feature_id.default_axis() == AssessmentAxis::AnnualPressure)
        .map(|f| f.contribution_id.clone())
        .collect();
    ids.sort();
    ids
}

#[test]
fn v2_4_policy_version_is_distinct_from_v2_3() {
    assert_ne!(
        ASSESSMENT_POLICY_V2_3_VERSION,
        ASSESSMENT_POLICY_V2_4_VERSION
    );
    assert_eq!(ASSESSMENT_POLICY_V2_4_VERSION, "v2.4");
}

#[test]
fn v2_4_constructor_reports_correct_version() {
    let policy = AssessmentPolicy::non_bazi_pressure_v2_4();
    assert_eq!(policy.policy_version(), ASSESSMENT_POLICY_V2_4_VERSION);
    assert_eq!(policy.policy_id(), "personal-day-assessment");
}

#[test]
fn v2_4_conflicting_fixture_emits_per_system_observations() {
    // 2026 (Ngọ chi) for a 1990 Ngọ birth: Kim Lâu (Than category,
    // heaviest), Thái Tuế (Trực conflict — same chi), sao hạn La Hầu.
    // HanSeverity is Critical (3 active systems) so the hard veto
    // also fires from extract_vetoes.
    let snap = snapshot_for(2026);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_birth_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.4 trace");

    let annual_ids = contribution_ids_on_annual_pressure(trace);
    assert!(
        annual_ids.iter().any(|id| id.starts_with("annual.kim_lau")),
        "missing Kim Lâu observation: {:?}",
        annual_ids
    );
    assert!(
        annual_ids
            .iter()
            .any(|id| id.starts_with("annual.thai_tue")),
        "missing Thái Tuế observation: {:?}",
        annual_ids
    );
    assert!(
        annual_ids.iter().any(|id| id.starts_with("annual.sao_han")),
        "missing sao hạn observation: {:?}",
        annual_ids
    );
    assert!(
        !annual_ids.iter().any(|id| id.starts_with("annual.han.")),
        "v2.4 must NOT emit the catch-all aggregation observation: {:?}",
        annual_ids
    );
}

#[test]
fn v2_4_each_system_carries_its_own_source_provenance() {
    // The conflicting fixture has Kim Lâu, Thái Tuế, sao hạn active.
    // Each must carry its own classical source id, not the shared
    // KHCBPPT default.
    let snap = snapshot_for(2026);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_birth_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.4 trace");

    let kim_lau = trace
        .features
        .iter()
        .find(|f| f.feature_id == AssessmentFeatureId::AnnualKimLau)
        .expect("Kim Lâu observation");
    assert_eq!(kim_lau.source_evidence.source_id, "ngoc-hap-ky");

    let thai_tue = trace
        .features
        .iter()
        .find(|f| f.feature_id == AssessmentFeatureId::AnnualThaiTue)
        .expect("Thái Tuế observation");
    assert_eq!(thai_tue.source_evidence.source_id, "khcbppt");

    let sao_han = trace
        .features
        .iter()
        .find(|f| f.feature_id == AssessmentFeatureId::AnnualSaoHan)
        .expect("sao hạn observation");
    assert_eq!(sao_han.source_evidence.source_id, "cuu-dieu");
}

#[test]
fn v2_4_strengths_track_traditional_severity() {
    // Kim Lâu Thân (self) carries the heaviest strength among the
    // Kim Lâu categories. The 2026 fixture for the 1990 birth has
    // Kim Lâu Thân (tuổi mụ = 37, 37 % 9 = 1 → Thân).
    let snap = snapshot_for(2026);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_birth_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.4 trace");

    let kim_lau = trace
        .features
        .iter()
        .find(|f| f.feature_id == AssessmentFeatureId::AnnualKimLau)
        .expect("Kim Lâu observation");
    assert!(
        (kim_lau.strength - 0.7).abs() < 1e-6,
        "Kim Lâu Thân expected strength 0.7, got {}",
        kim_lau.strength
    );
}

#[test]
fn v2_4_favorable_fixture_emits_no_annual_observations() {
    // 2027 (Mùi chi) for a 1990 Ngọ birth: zero active Hạn systems
    // (no Kim Lâu, no Tam Tai, no Hoàng Ốc active, no Thái Tuế, no
    // sao hạn). The trace must list zero non-Bazi annual pressure
    // observations — they are non-occurring signals, not missing
    // evidence.
    let snap = snapshot_for(2027);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_birth_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.4 trace");

    let annual_features = trace
        .features
        .iter()
        .filter(|f| {
            use amlich_core::assessment::AssessmentAxis;
            f.feature_id.default_axis() == AssessmentAxis::AnnualPressure
        })
        .collect::<Vec<_>>();
    assert!(
        annual_features.is_empty(),
        "favorable fixture must emit zero annual pressure observations, got: {:?}",
        annual_features
            .iter()
            .map(|f| f.contribution_id.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn v2_4_tam_tai_fixture_emits_tam_tai_observation() {
    // 2028 (Thân chi) for a 1990 Ngọ birth: Tam Tai active (Fire triad
    // → West years Thân/Dậu/Tuất), Kim Lâu (Thê), Hoàng Ốc. Verifies
    // Tam Tai and Hoàng Ốc are emitted with the right source ids.
    let snap = snapshot_for(2028);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_birth_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.4 trace");

    let tam_tai = trace
        .features
        .iter()
        .find(|f| f.feature_id == AssessmentFeatureId::AnnualTamTai)
        .expect("Tam Tai observation");
    assert_eq!(tam_tai.source_evidence.source_id, "khcbppt");
    assert_eq!(tam_tai.source_evidence.method, "tam_tai_lookup");

    let hoang_oc = trace
        .features
        .iter()
        .find(|f| f.feature_id == AssessmentFeatureId::AnnualHoangOc)
        .expect("Hoàng Ốc observation");
    assert_eq!(hoang_oc.source_evidence.source_id, "vn-folk");
}

#[test]
fn v2_4_unavailable_fixture_emits_explicit_unavailable_per_system() {
    // No gender → no yearly Hạn assessment → every non-Bazi annual
    // system must emit an explicit Unavailable observation so the
    // trace records what was missing (the amlich-7bm4 contract).
    let mut profile = ng0_birth_profile();
    profile.gender = None;

    let snap = snapshot_for(2026);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &profile,
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.4 trace");

    for feature_id in [
        AssessmentFeatureId::AnnualTamTai,
        AssessmentFeatureId::AnnualKimLau,
        AssessmentFeatureId::AnnualHoangOc,
        AssessmentFeatureId::AnnualThaiTue,
        AssessmentFeatureId::AnnualSaoHan,
    ] {
        let obs = trace
            .features
            .iter()
            .find(|f| f.feature_id == feature_id)
            .unwrap_or_else(|| panic!("missing {:?} unavailable observation", feature_id));
        assert!(
            matches!(obs.availability, AvailabilityState::Unavailable { .. }),
            "{:?} should be unavailable when gender is missing",
            feature_id
        );
    }
}

#[test]
fn v2_4_preserves_annual_pressure_axis_score_parity_with_v2_3() {
    // The v2 aggregation formula averages same-polarity observations,
    // so the AnnualPressure axis subtotal under v2.4 (per-system
    // observations) must match v2.3 (single catch-all observation)
    // byte-for-byte on the same fixtures.
    let cases = [
        (2024, "Kim Lâu only"),
        (2026, "Critical 3 systems"),
        (2028, "Tam Tai + Kim Lâu + Hoang Oc"),
    ];
    for (year, label) in cases {
        let snap = snapshot_for(year);
        let v2_3 = AssessmentPolicy::bazi_projection_v2_3().evaluate(
            AssessmentInputs::default(),
            &snap,
            &ng0_birth_profile(),
            ConsultationIntent::Wedding,
        );
        let v2_4 = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
            AssessmentInputs::default(),
            &snap,
            &ng0_birth_profile(),
            ConsultationIntent::Wedding,
        );
        assert_eq!(
            v2_3.axes.annual_pressure, v2_4.axes.annual_pressure,
            "AnnualPressure axis divergence on {}: v2.3={:?} v2.4={:?}",
            label, v2_3.axes.annual_pressure, v2_4.axes.annual_pressure
        );
        assert_eq!(
            v2_3.decision.bucket, v2_4.decision.bucket,
            "decision bucket divergence on {}: v2.3={:?} v2.4={:?}",
            label, v2_3.decision.bucket, v2_4.decision.bucket
        );
    }
}

#[test]
fn v2_4_factor_classification_exposes_per_system_factors() {
    // The canonical factor classification must surface per-system
    // ScoredFeature entries for active systems under v2.4 (replacing
    // the single AnnualThaiTue factor under v2.3). Factor `factor_id`
    // is the feature's stable contribution id, so the per-system ids
    // (`annual.kim_lau`, `annual.thai_tue`, `annual.sao_han`) appear.
    let snap = snapshot_for(2026);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_birth_profile(),
        ConsultationIntent::Wedding,
    );

    let annual_factors: Vec<String> = result
        .factors
        .iter()
        .filter(|f| {
            use amlich_core::assessment::AssessmentAxis;
            matches!(
                f.role,
                amlich_core::assessment::AssessmentFactorRole::ScoredFeature
            ) && f.axis == Some(AssessmentAxis::AnnualPressure)
        })
        .map(|f| f.factor_id.clone())
        .collect();
    assert!(
        annual_factors.iter().any(|id| id == "annual.kim_lau"),
        "missing Kim Lâu factor: {:?}",
        annual_factors
    );
    assert!(
        annual_factors.iter().any(|id| id == "annual.thai_tue"),
        "missing Thái Tuế factor: {:?}",
        annual_factors
    );
    assert!(
        annual_factors.iter().any(|id| id == "annual.sao_han"),
        "missing sao hạn factor: {:?}",
        annual_factors
    );
}

#[test]
fn v2_4_trace_reports_v2_4_policy_version() {
    let snap = snapshot_for(2026);
    let result = AssessmentPolicy::non_bazi_pressure_v2_4().evaluate(
        AssessmentInputs::default(),
        &snap,
        &ng0_birth_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = result.trace.as_ref().expect("v2.4 trace");
    assert_eq!(trace.policy_version, ASSESSMENT_POLICY_V2_4_VERSION);
    assert_eq!(trace.policy_id, "personal-day-assessment");

    // Sanity: all declared feature ids round-trip through the trace.
    let ids = feature_ids(trace);
    assert!(
        ids.iter().any(|id| id == "annual_sao_han"),
        "annual_sao_han must appear in trace feature list: {:?}",
        ids
    );
}
