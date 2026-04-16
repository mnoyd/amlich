use amlich_core::{
    build_initiation_opening_reasoning, calculate_day_snapshot,
    reasoning::{PersonalReasoningInput, RecommendationBucket},
    BirthInput, ConsultationIntent,
};

fn bucket_rank(bucket: RecommendationBucket) -> u8 {
    match bucket {
        RecommendationBucket::Avoid => 0,
        RecommendationBucket::Cautious => 1,
        RecommendationBucket::Mixed => 2,
        RecommendationBucket::Favorable => 3,
    }
}

#[test]
fn public_api_favorable_case_returns_supports_and_refinements() {
    let snapshot = calculate_day_snapshot(13, 5, 2024);
    let decision = build_initiation_opening_reasoning(&snapshot, None).expect("decision");

    assert!(!decision.primary_conclusion.is_empty());
    assert!(!decision.strongest_supports.is_empty());
    assert!(!decision.suggested_hours.is_empty());
}

#[test]
fn public_api_hard_taboo_case_surfaces_override_pressure() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let decision = build_initiation_opening_reasoning(&snapshot, None).expect("decision");

    assert!(!decision.override_factors.is_empty());
    assert!(decision.recommendation_bucket != RecommendationBucket::Favorable);
}

#[test]
fn public_api_mixed_case_keeps_conflict_semantics_visible() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let decision = build_initiation_opening_reasoning(&snapshot, None).expect("decision");

    assert!(
        !decision.context_is_clear
            || !decision.conflict_notes.is_empty()
            || matches!(
                decision.recommendation_bucket,
                RecommendationBucket::Mixed | RecommendationBucket::Cautious
            )
    );
}

#[test]
fn public_api_personal_override_does_not_become_strictly_more_favorable() {
    let snapshot = calculate_day_snapshot(13, 5, 2024);
    let baseline = build_initiation_opening_reasoning(&snapshot, None).expect("baseline");
    let personal_input = PersonalReasoningInput::from_birth(
        BirthInput {
            day: 1,
            month: 1,
            year: 1990,
            hour: Some(9),
            minute: Some(0),
            timezone: 7.0,
            gender: None,
            location_name: None,
        },
        ConsultationIntent::OpeningBusiness,
    );
    let personalized =
        build_initiation_opening_reasoning(&snapshot, Some(&personal_input)).expect("personalized");

    assert!(
        bucket_rank(personalized.recommendation_bucket)
            <= bucket_rank(baseline.recommendation_bucket)
            && (bucket_rank(personalized.recommendation_bucket)
                < bucket_rank(baseline.recommendation_bucket)
                || (personalized.context_is_clear as u8) <= (baseline.context_is_clear as u8)
                || personalized.conflict_notes.len() >= baseline.conflict_notes.len()
                || personalized.override_factors.len() >= baseline.override_factors.len()
                || personalized.strongest_resistances.len()
                    >= baseline.strongest_resistances.len())
    );
}
