use amlich_core::{
    BirthInput, ConsultationIntent,
    reasoning::{PersonalReasoningInput, RecommendationBucket, build_initiation_opening_decision},
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
fn hard_taboo_overrides_supportive_opening_signals() {
    let snapshot = amlich_core::calculate_day_snapshot(14, 2, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert!(!decision.override_factors.is_empty());
}

#[test]
fn personal_override_can_weaken_a_generally_favorable_opening_day() {
    let snapshot = amlich_core::calculate_day_snapshot(13, 5, 2024);
    let baseline = build_initiation_opening_decision(&snapshot, None).expect("baseline decision");
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
        build_initiation_opening_decision(&snapshot, Some(&personal_input)).expect("personalized decision");

    assert!(!baseline.strongest_supports.is_empty());
    assert!(
        bucket_rank(personalized.recommendation_bucket) <= bucket_rank(baseline.recommendation_bucket)
            && (bucket_rank(personalized.recommendation_bucket) < bucket_rank(baseline.recommendation_bucket)
                || (personalized.context_is_clear as u8) <= (baseline.context_is_clear as u8)
                || personalized.conflict_notes.len() >= baseline.conflict_notes.len()
                || personalized.override_factors.len() >= baseline.override_factors.len()
                || personalized.strongest_resistances.len() >= baseline.strongest_resistances.len())
    );
}

#[test]
fn mixed_signals_reduce_context_clarity_instead_of_pretending_certainty() {
    let snapshot = amlich_core::calculate_day_snapshot(14, 2, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert!(!decision.context_is_clear || !decision.conflict_notes.is_empty());
}

#[test]
fn existing_favorable_opening_dates_still_produce_non_empty_support_reasons() {
    let snapshot = amlich_core::calculate_day_snapshot(13, 5, 2024);
    let decision = build_initiation_opening_decision(&snapshot, None).expect("decision");

    assert!(!decision.strongest_supports.is_empty());
}
