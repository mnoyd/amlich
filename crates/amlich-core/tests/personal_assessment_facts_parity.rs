//! A-R11 deletion test (amlich-mwbp.8 P2 / amlich-9z7i).
//!
//! `PersonalReasoningInput::build_fact_nodes` and
//! `PersonalReasoningInput::suggested_hours` / `suggested_directions` must
//! share the same underlying [`PersonalAssessmentFacts`] so they cannot
//! drift. Previously each helper rebuilt the chart and matrices
//! independently — a rule change between calls could make the
//! suggestion list disagree with the fact-node summary. After the
//! `amlich-9z7i` consolidation, every call site reads from one cached
//! facts bundle per snapshot/profile pair.

use amlich_core::{
    calculate_day_snapshot,
    reasoning::{PersonalAssessmentFacts, PersonalReasoningInput},
    BirthInput, ConsultationIntent, Gender,
};

fn sample_personal() -> PersonalReasoningInput {
    PersonalReasoningInput::from_birth(
        BirthInput {
            day: 15,
            month: 8,
            year: 1990,
            hour: Some(9),
            minute: Some(30),
            timezone: 7.0,
            gender: Some(Gender::Male),
            location_name: None,
        },
        ConsultationIntent::OpeningBusiness,
    )
}

#[test]
fn build_fact_nodes_from_facts_matches_snapshot_based_variant() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let personal = sample_personal();

    let facts = PersonalAssessmentFacts::build(&personal, &snapshot).expect("facts");
    let from_facts = personal.build_fact_nodes_from_facts(&facts);
    let from_snapshot = personal.build_fact_nodes(&snapshot).expect("from snapshot");

    assert_eq!(
        from_facts.len(),
        from_snapshot.len(),
        "fact-node count must match between the cached and snapshot-based variants"
    );
    for (cached, rebuilt) in from_facts.iter().zip(from_snapshot.iter()) {
        assert_eq!(
            cached.id, rebuilt.id,
            "fact-node id must match (cached vs snapshot-based)"
        );
        assert_eq!(
            cached.summary_vi, rebuilt.summary_vi,
            "fact-node summary must match for id={} (cached vs snapshot-based)",
            cached.id
        );
        assert_eq!(
            cached.evidence, rebuilt.evidence,
            "fact-node evidence must match for id={} (cached vs snapshot-based)",
            cached.id
        );
    }
}

#[test]
fn suggested_hours_from_facts_matches_snapshot_based_variant() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let personal = sample_personal();
    let facts = PersonalAssessmentFacts::build(&personal, &snapshot).expect("facts");

    let from_facts = personal.suggested_hours_from_facts(&facts);
    let from_snapshot = personal.suggested_hours(&snapshot);

    assert_eq!(
        from_facts, from_snapshot,
        "suggested_hours must be byte-identical between the cached and snapshot-based variants"
    );
}

#[test]
fn suggested_directions_from_facts_matches_snapshot_based_variant() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let personal = sample_personal();
    let facts = PersonalAssessmentFacts::build(&personal, &snapshot).expect("facts");

    let from_facts = personal.suggested_directions_from_facts(&facts);
    let from_snapshot = personal.suggested_directions(&snapshot);

    assert_eq!(
        from_facts, from_snapshot,
        "suggested_directions must be byte-identical between the cached and snapshot-based variants"
    );
}

#[test]
fn facts_bundle_is_deterministic_across_rebuilds() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let personal = sample_personal();

    let facts1 = PersonalAssessmentFacts::build(&personal, &snapshot).expect("facts1");
    let facts2 = PersonalAssessmentFacts::build(&personal, &snapshot).expect("facts2");

    assert_eq!(
        facts1.day_person_matrix, facts2.day_person_matrix,
        "day-person matrix must be deterministic across rebuilds"
    );
    assert_eq!(
        facts1.personal_hour_matrix, facts2.personal_hour_matrix,
        "personal-hour matrix must be deterministic across rebuilds"
    );
    assert_eq!(
        facts1.direction_merge_matrix, facts2.direction_merge_matrix,
        "direction-merge matrix must be deterministic across rebuilds"
    );
}
