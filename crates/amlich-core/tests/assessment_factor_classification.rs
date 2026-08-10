use amlich_core::{
    assess_personal_day,
    assessment::{AssessmentFactorRole, AvailabilityState},
    birth::BirthProfile,
    calculate_day_snapshot, ConsultationIntent,
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

#[test]
fn canonical_assessment_classifies_facts_features_and_explanation_context() {
    let assessment = assess_personal_day(
        calculate_day_snapshot(10, 2, 2024),
        date_only_profile(),
        ConsultationIntent::Wedding,
    );

    assert!(assessment
        .factors
        .iter()
        .any(|factor| factor.role == AssessmentFactorRole::Fact));
    assert!(assessment
        .factors
        .iter()
        .any(|factor| factor.role == AssessmentFactorRole::ScoredFeature));
    assert!(assessment
        .factors
        .iter()
        .any(|factor| factor.role == AssessmentFactorRole::ExplanationOnly));
    assert!(assessment
        .factors
        .iter()
        .any(|factor| factor.factor_id == "fact.day.canchi"));
}

#[test]
fn unavailable_factor_is_not_projected_as_neutral() {
    let assessment = assess_personal_day(
        calculate_day_snapshot(10, 2, 2024),
        date_only_profile(),
        ConsultationIntent::Wedding,
    );

    let unavailable = assessment
        .factors
        .iter()
        .find(|factor| factor.factor_id == "unavailable.personal_alignment")
        .expect("missing personal alignment must be classified");
    assert_eq!(unavailable.role, AssessmentFactorRole::ScoredFeature);
    assert!(matches!(
        unavailable.availability,
        AvailabilityState::Unavailable { .. }
    ));
}

#[test]
fn classification_is_deterministic_and_does_not_change_the_verdict() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let first = assess_personal_day(
        snapshot.clone(),
        date_only_profile(),
        ConsultationIntent::Wedding,
    );
    let second = assess_personal_day(snapshot, date_only_profile(), ConsultationIntent::Wedding);

    assert_eq!(first.factors, second.factors);
    assert_eq!(first.axes, second.axes);
    assert_eq!(first.decision, second.decision);
    assert_eq!(first.contributions, second.contributions);
}
