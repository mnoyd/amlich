use amlich_core::{
    build_initiation_opening_reasoning_bundle, calculate_day_snapshot,
    calculate_day_snapshot_with_timezone, reasoning::PersonalReasoningInput, BirthInput,
    ConsultationIntent, Gender, ReasoningConclusionSemantic, ReasoningEdgeJustification,
    ReasoningEvidenceSourceFamily, ReasoningNodeSeverity, RecommendationBucket,
};
use serde_json::Value;

fn profile_input(
    day: i32,
    month: i32,
    year: i32,
    hour: u8,
    minute: u8,
    timezone: f64,
    gender: Option<Gender>,
) -> PersonalReasoningInput {
    PersonalReasoningInput::from_birth(
        BirthInput {
            day,
            month,
            year,
            hour: Some(hour),
            minute: Some(minute),
            timezone,
            gender,
            location_name: None,
        },
        ConsultationIntent::OpeningBusiness,
    )
}

#[test]
fn reasoning_bundle_exposes_decision_and_exported_graph() {
    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let bundle = build_initiation_opening_reasoning_bundle(&snapshot, None).expect("bundle");

    assert!(!bundle.decision.primary_conclusion.is_empty());
    assert!(!bundle.graph.nodes.is_empty());
    assert!(!bundle.graph.edges.is_empty());
}

#[test]
fn exported_graph_marks_override_and_taboo_metadata() {
    let snapshot = calculate_day_snapshot(3, 1, 2024);
    let bundle = build_initiation_opening_reasoning_bundle(&snapshot, None).expect("bundle");

    assert_eq!(
        bundle.decision.recommendation_bucket,
        RecommendationBucket::Avoid
    );
    assert_eq!(
        bundle.decision_export.semantic,
        ReasoningConclusionSemantic::OverrideAvoid
    );

    let taboo_node = bundle
        .graph
        .nodes
        .iter()
        .find(|node| node.id == "fact.day.taboos")
        .expect("taboo node");
    assert_eq!(taboo_node.severity, Some(ReasoningNodeSeverity::HardTaboo));
    assert!(taboo_node.tags.iter().any(|tag| tag == "resistance"));
    assert!(taboo_node
        .evidence
        .iter()
        .any(|e| e.source_family == ReasoningEvidenceSourceFamily::AlmanacRule));

    assert!(bundle
        .graph
        .edges
        .iter()
        .any(|edge| edge.tags.iter().any(|tag| tag == "override")));
    assert!(bundle
        .graph
        .edges
        .iter()
        .any(|edge| edge.justification == ReasoningEdgeJustification::TabooPressure));
    assert!(bundle
        .decision_export
        .override_factors
        .iter()
        .any(|note| note.tags.iter().any(|tag| tag == "override")));
}

#[test]
fn exported_graph_keeps_personal_nodes_and_signal_axes_visible() {
    let snapshot = calculate_day_snapshot(15, 6, 2024);
    let input = profile_input(12, 8, 1992, 11, 30, 7.0, Some(Gender::Female));
    let bundle =
        build_initiation_opening_reasoning_bundle(&snapshot, Some(&input)).expect("bundle");

    assert!(bundle
        .graph
        .nodes
        .iter()
        .any(|node| node.tags.iter().any(|tag| tag == "personal")));
    assert!(bundle.graph.nodes.iter().any(|node| node.axis.is_some()));
    assert!(bundle
        .graph
        .nodes
        .iter()
        .flat_map(|node| node.evidence.iter())
        .any(|e| e.source_family == ReasoningEvidenceSourceFamily::Interaction));
    assert!(bundle.graph.edges.iter().any(|edge| edge.justification
        == ReasoningEdgeJustification::PersonalDayAlignment
        || edge.justification == ReasoningEdgeJustification::PersonalHourAlignment));
    assert!(bundle
        .decision_export
        .axis_scores
        .iter()
        .any(|axis| axis.strongest_node_id.is_some()));
}

#[test]
fn reasoning_bundle_serializes_to_stable_machine_readable_shape() {
    let snapshot = calculate_day_snapshot_with_timezone(14, 2, 2024, 7.0);
    let input = profile_input(30, 1, 1989, 23, 30, 7.0, Some(Gender::Male));
    let bundle =
        build_initiation_opening_reasoning_bundle(&snapshot, Some(&input)).expect("bundle");

    let value = serde_json::to_value(&bundle).expect("serialize bundle");
    let object = value.as_object().expect("bundle object");

    assert!(object.contains_key("decision"));
    assert!(object.contains_key("decision_export"));
    assert!(object.contains_key("graph"));
    assert_eq!(
        value.pointer("/decision/recommendation_bucket"),
        Some(&Value::String("cautious".to_string()))
    );
    assert_eq!(
        value.pointer("/graph/action_id"),
        Some(&Value::String("initiation_opening".to_string()))
    );
    assert_eq!(
        value.pointer("/decision_export/semantic"),
        Some(&Value::String("override_cautious".to_string()))
    );
    assert!(value
        .pointer("/graph/nodes")
        .and_then(Value::as_array)
        .is_some_and(|nodes| !nodes.is_empty()));
    assert_eq!(
        value.pointer("/graph/nodes/0/evidence/0/source_family"),
        Some(&Value::String("snapshot".to_string()))
    );
    assert!(value
        .pointer("/graph/edges")
        .and_then(Value::as_array)
        .is_some_and(|edges| !edges.is_empty()));
    assert!(value.pointer("/graph/edges/0/justification").is_some());
    assert!(value
        .pointer("/graph/edges/0/evidence/0/source_family")
        .is_some());
    assert!(value
        .pointer("/decision_export/axis_scores")
        .and_then(Value::as_array)
        .is_some_and(|axes| axes.len() == 6));
}
