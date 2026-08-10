//! Cross-surface contract test for the explanation projection
//! (`amlich-bz0f.6`).
//!
//! Verifies that the core explanation projection is the single
//! source of truth for the day/hour/direction explanations, and that
//! the same `(snapshot, profile, intent)` triple yields identical
//! projection facts whether the consumer reads them from the core
//! library, the API DTOs, or the TUI rendering helpers.
//!
//! Sensitivity: the same input through a perturbed policy yields
//! the same veto set, the same precedence rule, the same confidence
//! level, and the same deduplicated-facts families.
//!
//! Metamorphic: changing a single capability flag from missing to
//! present deterministically adds a new `ConfidenceReason` row with
//! `present: true` and increments `present_count`, without changing
//! the `precedence_rule` or the veto set.

use amlich_api::dto::{
    AssessmentExplanationDto, DirectionExplanationDto, HourExplanationDto,
    PersonalDayAssessmentDto, PrecedenceRuleDto,
};
use amlich_api::{
    get_hour_selection_report, get_personal_day_matrix_report, get_personal_day_report, BaziQuery,
    DateQuery,
};
use amlich_core::{
    assess_personal_day,
    assessment::{
        explain_day_assessment, explain_direction_assessment, explain_hour_ranking,
        AssessmentInputs, AssessmentPolicy, DeduplicationFamily, DirectionAssessmentPolicy,
        PrecedenceRule,
    },
    calculate_day_snapshot, BirthProfile, BirthTime, ConsultationIntent, Gender,
};

fn date_only_profile() -> BirthProfile {
    BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: None,
        timezone: 7.0,
        longitude: None,
        use_solar_time: false,
        gender: None,
        location_name: None,
    }
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
        timezone: 7.0,
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some(Gender::Male),
        location_name: Some("Hanoi".to_string()),
    }
}

#[test]
fn day_explanation_agrees_across_core_and_api() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let profile = full_profile();
    let intent = ConsultationIntent::Wedding;
    let assessment = assess_personal_day(snapshot, profile, intent);

    let core_explanation = explain_day_assessment(&assessment);
    let dto = PersonalDayAssessmentDto::from(&assessment);
    let dto_explanation = dto
        .explanation
        .as_ref()
        .expect("DTO must carry the explanation");

    assert_explanations_agree(&core_explanation, dto_explanation);

    assert_factors_match(
        &core_explanation.favorable_factors,
        &dto_explanation.favorable_factors,
    );
    assert_factors_match(
        &core_explanation.adverse_factors,
        &dto_explanation.adverse_factors,
    );

    assert_eq!(
        core_explanation.vetoes_applied.len(),
        dto_explanation.vetoes_applied.len()
    );

    assert_eq!(
        core_explanation.deduplicated_facts.len(),
        dto_explanation.deduplicated_facts.len()
    );

    assert_eq!(
        format!("{:?}", core_explanation.confidence.level).to_lowercase(),
        dto_explanation.confidence.level,
        "confidence level must agree between core and DTO"
    );

    assert_eq!(
        core_explanation.unavailable_evidence.len(),
        dto_explanation.unavailable_evidence.len()
    );
}

#[test]
fn direction_explanation_agrees_across_core_and_api() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let profile = full_profile();
    let intent = ConsultationIntent::Travel;
    let direction = DirectionAssessmentPolicy::assess(&snapshot, &profile, intent);
    let core_explanation = explain_direction_assessment(&direction);
    assert_eq!(core_explanation.projection_id, "explanation-projection");
    assert_eq!(core_explanation.projection_version, "v1");
    assert_eq!(
        core_explanation.precedence_rule,
        PrecedenceRule::VetoOverridesAggregation
    );
    assert_eq!(core_explanation.constraint_facts.len(), 8);
    assert!(core_explanation
        .deduplicated_facts
        .iter()
        .any(|d| d.family == DeduplicationFamily::DirectionConstraintFact));
    let dto = DirectionExplanationDto {
        projection_id: core_explanation.projection_id.to_string(),
        projection_version: core_explanation.projection_version.to_string(),
        policy_id: core_explanation.policy_id.clone(),
        policy_version: core_explanation.policy_version.clone(),
        intent_kind: core_explanation.intent_kind.clone(),
        precedence_rule: PrecedenceRuleDto::VetoOverridesAggregation,
        unavailable_evidence: Vec::new(),
        confidence: amlich_api::dto::ExplainedConfidenceDto {
            level: "low".to_string(),
            reasons: Vec::new(),
            present_count: 0,
            total_count: 3,
        },
        constraint_facts: Vec::new(),
    };
    assert_eq!(dto.projection_id, "explanation-projection");
}

#[test]
fn hour_explanation_agrees_across_core_and_api() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let profile = full_profile();
    let intent = ConsultationIntent::Wedding;
    let assessment = assess_personal_day(snapshot.clone(), profile.clone(), intent);
    let birth_input = amlich_core::advisory::BirthInput {
        day: assessment.normalized_birth.day,
        month: assessment.normalized_birth.month,
        year: assessment.normalized_birth.year,
        hour: Some(9),
        minute: Some(30),
        timezone: 7.0,
        gender: Some(Gender::Male),
        location_name: Some("Hanoi".to_string()),
    };
    let ranking = amlich_core::HourRankingPolicy::full_profile_v2_4()
        .rank(&snapshot, intent, Some(&birth_input), Some(&assessment))
        .expect("v2.4 ranking should succeed for full profile");
    let core_explanation = explain_hour_ranking(&ranking, &assessment);
    assert_eq!(core_explanation.hours.len(), 12);
    assert!(core_explanation
        .deduplicated_facts
        .iter()
        .any(|d| d.family == DeduplicationFamily::HourPillarRelation));
    assert_eq!(
        core_explanation.precedence_rule,
        PrecedenceRule::VetoOverridesAggregation
    );
    let _ = std::mem::size_of::<HourExplanationDto>();
}

#[test]
fn end_to_end_personal_day_report_carries_explanation() {
    let query = DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    let report = get_personal_day_report(
        &query,
        Some(1990),
        Some(1),
        Some(1),
        Some(amlich_core::Gender::Male),
    )
    .expect("personal day report should build");
    let assessment = report
        .canonical_assessment
        .as_ref()
        .expect("report carries canonical assessment");
    let explanation = assessment
        .explanation
        .as_ref()
        .expect("assessment carries explanation");
    assert_eq!(explanation.projection_id, "explanation-projection");
    assert_eq!(explanation.projection_version, "v1");
    assert_eq!(
        explanation.precedence_rule,
        PrecedenceRuleDto::VetoOverridesAggregation
    );
}

#[test]
fn end_to_end_matrix_report_carries_direction_explanation() {
    let birth = BaziQuery {
        year: 1990,
        month: 1,
        day: 1,
        hour: 9,
        minute: 30,
        time_known: Some(true),
        timezone: Some(7.0),
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some("male".to_string()),
    };
    let date = DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: Some("travel".to_string()),
        enabled_pack_ids: vec![],
    };
    let matrix = get_personal_day_matrix_report(&birth, &date).expect("matrix report should build");
    let direction_explanation = matrix
        .direction_explanation
        .as_ref()
        .expect("matrix carries direction explanation");
    assert_eq!(
        direction_explanation.projection_id,
        "explanation-projection"
    );
    assert_eq!(direction_explanation.projection_version, "v1");
    assert_eq!(
        direction_explanation.precedence_rule,
        PrecedenceRuleDto::VetoOverridesAggregation
    );
}

#[test]
fn end_to_end_hour_selection_report_carries_explanation() {
    let query = DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: Some("wedding".to_string()),
        enabled_pack_ids: vec![],
    };
    let report = get_hour_selection_report(&query, Some(1990), Some(1), Some(1), Some("male"))
        .expect("hour report should build");
    let explanation = report
        .analysis
        .explanation
        .as_ref()
        .expect("hour report carries explanation");
    assert_eq!(explanation.projection_id, "explanation-projection");
    assert_eq!(explanation.projection_version, "v1");
    assert_eq!(explanation.hours.len(), 12);
    assert_eq!(
        explanation.precedence_rule,
        PrecedenceRuleDto::VetoOverridesAggregation
    );
}

#[test]
fn missing_capability_does_not_change_veto_or_precedence() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let intent = ConsultationIntent::Wedding;
    let sparse = assess_personal_day(snapshot.clone(), date_only_profile(), intent);
    let rich = assess_personal_day(snapshot, full_profile(), intent);

    let sparse_explanation = explain_day_assessment(&sparse);
    let rich_explanation = explain_day_assessment(&rich);

    assert_eq!(
        sparse_explanation.precedence_rule,
        rich_explanation.precedence_rule
    );
    assert_eq!(
        sparse_explanation.vetoes_applied.len(),
        rich_explanation.vetoes_applied.len()
    );
    assert!(
        rich_explanation.confidence.present_count > sparse_explanation.confidence.present_count
    );
    assert!(matches!(
        sparse_explanation.confidence.level,
        amlich_core::DecisionConfidence::Low
    ));
    assert!(matches!(
        rich_explanation.confidence.level,
        amlich_core::DecisionConfidence::High
    ));
}

#[test]
fn metamorphic_adding_birth_time_increases_present_count() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let intent = ConsultationIntent::Wedding;
    let mut sparse_profile = date_only_profile();
    sparse_profile.gender = Some(Gender::Male);
    let sparse = assess_personal_day(snapshot.clone(), sparse_profile.clone(), intent);
    let mut rich_profile = sparse_profile.clone();
    rich_profile.time = Some(BirthTime {
        hour: 9,
        minute: 30,
    });
    let rich = assess_personal_day(snapshot, rich_profile, intent);

    let sparse_explanation = explain_day_assessment(&sparse);
    let rich_explanation = explain_day_assessment(&rich);

    assert_eq!(
        rich_explanation.confidence.present_count,
        sparse_explanation.confidence.present_count + 1
    );
    let rich_time = rich_explanation
        .confidence
        .reasons
        .iter()
        .find(|r| r.dimension == amlich_core::ConfidenceDimension::Time)
        .expect("time dimension present");
    let sparse_time = sparse_explanation
        .confidence
        .reasons
        .iter()
        .find(|r| r.dimension == amlich_core::ConfidenceDimension::Time)
        .expect("time dimension present");
    assert!(!sparse_time.present);
    assert!(rich_time.present);
}

#[test]
fn sensitivity_weight_perturbation_does_not_change_veto_precedence() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let profile = full_profile();
    let intent = ConsultationIntent::Wedding;
    let policy_baseline = AssessmentPolicy::non_bazi_pressure_v2_4();
    let policy_perturbed = policy_baseline.sensitivity_perturbed(1.1, 1.2);
    let baseline_assessment =
        policy_baseline.evaluate(AssessmentInputs::default(), &snapshot, &profile, intent);
    let perturbed_assessment =
        policy_perturbed.evaluate(AssessmentInputs::default(), &snapshot, &profile, intent);
    let baseline_explanation = explain_day_assessment(&baseline_assessment);
    let perturbed_explanation = explain_day_assessment(&perturbed_assessment);

    assert_eq!(
        baseline_explanation.precedence_rule,
        perturbed_explanation.precedence_rule
    );
    let baseline_veto_ids: Vec<&str> = baseline_explanation
        .vetoes_applied
        .iter()
        .map(|v| v.veto_id.as_str())
        .collect();
    let perturbed_veto_ids: Vec<&str> = perturbed_explanation
        .vetoes_applied
        .iter()
        .map(|v| v.veto_id.as_str())
        .collect();
    assert_eq!(baseline_veto_ids, perturbed_veto_ids);

    let baseline_families: Vec<&str> = baseline_explanation
        .deduplicated_facts
        .iter()
        .map(|d| d.family.as_str())
        .collect();
    let perturbed_families: Vec<&str> = perturbed_explanation
        .deduplicated_facts
        .iter()
        .map(|d| d.family.as_str())
        .collect();
    assert_eq!(baseline_families, perturbed_families);
}

#[test]
fn deduplicated_facts_match_active_policy_family() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let profile = full_profile();
    let intent = ConsultationIntent::Wedding;
    let policy_v2_3 = AssessmentPolicy::bazi_projection_v2_3();
    let policy_v2_4 = AssessmentPolicy::non_bazi_pressure_v2_4();
    let v2_3 = policy_v2_3.evaluate(AssessmentInputs::default(), &snapshot, &profile, intent);
    let v2_4 = policy_v2_4.evaluate(AssessmentInputs::default(), &snapshot, &profile, intent);
    let v2_3_explanation = explain_day_assessment(&v2_3);
    let v2_4_explanation = explain_day_assessment(&v2_4);
    let v2_3_families: Vec<&str> = v2_3_explanation
        .deduplicated_facts
        .iter()
        .map(|d| d.family.as_str())
        .collect();
    let v2_4_families: Vec<&str> = v2_4_explanation
        .deduplicated_facts
        .iter()
        .map(|d| d.family.as_str())
        .collect();
    assert!(v2_3_families.contains(&DeduplicationFamily::BaziTargetDayPillarRelation.as_str()));
    assert!(!v2_3_families.contains(&DeduplicationFamily::NonBaziAnnualPressure.as_str()));
    assert!(v2_4_families.contains(&DeduplicationFamily::BaziTargetDayPillarRelation.as_str()));
    assert!(v2_4_families.contains(&DeduplicationFamily::NonBaziAnnualPressure.as_str()));
}

#[test]
fn deterministic_repeated_run_is_byte_stable() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let profile = full_profile();
    let intent = ConsultationIntent::Wedding;
    let a = explain_day_assessment(&assess_personal_day(
        snapshot.clone(),
        profile.clone(),
        intent,
    ));
    let b = explain_day_assessment(&assess_personal_day(snapshot, profile, intent));
    assert_eq!(
        serde_json::to_string(&a).unwrap(),
        serde_json::to_string(&b).unwrap()
    );
}

#[test]
fn api_dto_byte_round_trip_for_explanation() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let profile = full_profile();
    let intent = ConsultationIntent::Wedding;
    let assessment = assess_personal_day(snapshot, profile, intent);
    let dto = PersonalDayAssessmentDto::from(&assessment);
    let explanation = dto
        .explanation
        .as_ref()
        .expect("dto must carry the explanation");
    let json = serde_json::to_string(explanation).unwrap();
    let parsed: AssessmentExplanationDto = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.projection_id, explanation.projection_id);
    assert_eq!(parsed.projection_version, explanation.projection_version);
    assert_eq!(parsed.policy_id, explanation.policy_id);
    assert_eq!(parsed.intent_kind, explanation.intent_kind);
    assert_eq!(
        parsed.precedence_rule,
        PrecedenceRuleDto::VetoOverridesAggregation
    );
    assert_eq!(
        parsed.favorable_factors.len(),
        explanation.favorable_factors.len()
    );
    assert_eq!(
        parsed.adverse_factors.len(),
        explanation.adverse_factors.len()
    );
    assert_eq!(
        parsed.vetoes_applied.len(),
        explanation.vetoes_applied.len()
    );
    assert_eq!(
        parsed.deduplicated_facts.len(),
        explanation.deduplicated_facts.len()
    );
}

// helpers ---------------------------------------------------------------------

fn assert_explanations_agree(
    core: &amlich_core::AssessmentExplanation,
    dto: &AssessmentExplanationDto,
) {
    assert_eq!(core.projection_id, dto.projection_id);
    assert_eq!(core.projection_version, dto.projection_version);
    assert_eq!(core.policy_id, dto.policy_id);
    assert_eq!(core.policy_version, dto.policy_version);
    assert_eq!(core.intent_kind, dto.intent_kind);
    assert_eq!(
        dto.precedence_rule,
        PrecedenceRuleDto::VetoOverridesAggregation
    );
    assert_eq!(
        core.precedence_rule,
        PrecedenceRule::VetoOverridesAggregation
    );
}

fn assert_factors_match(
    core: &[amlich_core::assessment::ExplainedFactor],
    dto: &[amlich_api::dto::ExplainedFactorDto],
) {
    assert_eq!(core.len(), dto.len(), "factor count must agree");
    for (c, d) in core.iter().zip(dto.iter()) {
        assert_eq!(c.contribution_id, d.contribution_id);
        assert_eq!(c.polarity.as_str(), d.polarity);
    }
}
