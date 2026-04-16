use amlich_core::{
    calculate_day_snapshot,
    reasoning::{build_fact_graph, ActionId, PersonalReasoningInput},
    BirthInput, ConsultationIntent,
};

#[test]
fn fact_graph_omits_personal_nodes_when_birth_input_is_missing() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let graph = build_fact_graph(ActionId::InitiationOpening, &snapshot, None).expect("graph");

    assert!(!graph
        .nodes
        .iter()
        .any(|n| n.id.starts_with("fact.personal.")));
}

#[test]
fn fact_graph_adds_personal_nodes_when_birth_input_is_present() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let input = PersonalReasoningInput::from_birth(
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

    let graph =
        build_fact_graph(ActionId::InitiationOpening, &snapshot, Some(&input)).expect("graph");

    assert!(graph
        .nodes
        .iter()
        .any(|n| n.id == "fact.personal.day_person_matrix"));
}
