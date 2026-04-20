use amlich_core::{
    build_initiation_opening_reasoning_bundle, calculate_day_snapshot,
    calculate_day_snapshot_with_timezone,
    reasoning::{
        PersonalReasoningInput,
        ReasoningEdgeJustification, ReasoningGraphExport, ReasoningNodeSeverity,
    },
    BirthInput, ConsultationIntent, Gender,
};
use std::collections::HashSet;

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

#[derive(Clone)]
struct ParityCase {
    id: &'static str,
    day: i32,
    month: i32,
    year: i32,
    timezone: Option<f64>,
    personal: Option<PersonalReasoningInput>,
}

fn parity_corpus() -> Vec<ParityCase> {
    vec![
        ParityCase {
            id: "favorable_clear",
            day: 13,
            month: 5,
            year: 2024,
            timezone: None,
            personal: None,
        },
        ParityCase {
            id: "hard_avoid",
            day: 3,
            month: 1,
            year: 2024,
            timezone: None,
            personal: None,
        },
        ParityCase {
            id: "layered_cautious",
            day: 14,
            month: 2,
            year: 2024,
            timezone: None,
            personal: None,
        },
        ParityCase {
            id: "new_year_clear",
            day: 1,
            month: 1,
            year: 2024,
            timezone: None,
            personal: None,
        },
        ParityCase {
            id: "mid_june_favorable",
            day: 15,
            month: 6,
            year: 2024,
            timezone: None,
            personal: None,
        },
        ParityCase {
            id: "with_personal",
            day: 13,
            month: 5,
            year: 2024,
            timezone: None,
            personal: Some(profile_input(1, 1, 1990, 9, 0, 7.0, None)),
        },
        ParityCase {
            id: "with_kua_directions",
            day: 15,
            month: 6,
            year: 2024,
            timezone: None,
            personal: Some(profile_input(12, 8, 1992, 11, 30, 7.0, Some(Gender::Female))),
        },
        ParityCase {
            id: "boundary_vn_tz",
            day: 14,
            month: 2,
            year: 2024,
            timezone: Some(7.0),
            personal: Some(profile_input(30, 1, 1989, 23, 30, 7.0, Some(Gender::Male))),
        },
    ]
}

fn snapshot_for(case: &ParityCase) -> amlich_core::DaySnapshot {
    match case.timezone {
        Some(tz) => calculate_day_snapshot_with_timezone(case.day, case.month, case.year, tz),
        None => calculate_day_snapshot(case.day, case.month, case.year),
    }
}

fn build_graph(case: &ParityCase) -> ReasoningGraphExport {
    let snapshot = snapshot_for(case);
    let bundle = build_initiation_opening_reasoning_bundle(&snapshot, case.personal.as_ref())
        .expect("bundle");
    bundle.graph
}

#[test]
fn semantic_projection_preserves_action_id() {
    for case in parity_corpus() {
        let graph = build_graph(&case);
        assert_eq!(
            format!("{:?}", graph.action_id),
            "InitiationOpening",
            "{} action_id mismatch",
            case.id
        );
    }
}

#[test]
fn semantic_projection_preserves_canonical_node_ids() {
    for case in parity_corpus() {
        let graph = build_graph(&case);
        let node_ids: HashSet<&str> = graph.nodes.iter().map(|n| n.id.as_str()).collect();

        assert!(node_ids.contains("fact.day.taboos"), "{} missing taboos", case.id);
        assert!(node_ids.contains("fact.day.day_deity"), "{} missing deity", case.id);
        assert!(node_ids.contains("fact.day.hoang_dao_hours"), "{} missing hours", case.id);
        assert!(node_ids.contains("fact.day.truc"), "{} missing truc", case.id);
        assert!(node_ids.contains("fact.day.xung_hop"), "{} missing xung_hop", case.id);
        assert!(node_ids.contains("fact.day.nhi_thap_bat_tu"), "{} missing star", case.id);
        assert!(node_ids.contains("fact.day.solar_term"), "{} missing solar_term", case.id);
        assert!(node_ids.contains("signal.support"), "{} missing signal.support", case.id);
        assert!(node_ids.contains("signal.resistance"), "{} missing signal.resistance", case.id);
        assert!(node_ids.contains("signal.stability"), "{} missing signal.stability", case.id);
        assert!(node_ids.contains("signal.timing_fit"), "{} missing signal.timing_fit", case.id);
        assert!(
            node_ids.contains("signal.context_clarity"),
            "{} missing signal.context_clarity",
            case.id
        );
        assert!(
            node_ids.contains("signal.personal_alignment"),
            "{} missing signal.personal_alignment",
            case.id
        );
    }
}

#[test]
fn semantic_projection_preserves_node_and_edge_non_emptiness() {
    for case in parity_corpus() {
        let graph = build_graph(&case);
        assert!(!graph.nodes.is_empty(), "{} should have nodes", case.id);
        assert!(!graph.edges.is_empty(), "{} should have edges", case.id);
    }
}

#[test]
fn semantic_projection_preserves_severity_annotations() {
    let graph = build_graph(&parity_corpus().into_iter().find(|c| c.id == "hard_avoid").unwrap());
    let taboo = graph.nodes.iter().find(|n| n.id == "fact.day.taboos").expect("taboo");
    assert!(taboo.severity.is_some());
    assert!(matches!(taboo.severity, Some(ReasoningNodeSeverity::HardTaboo)));
}

#[test]
fn semantic_projection_preserves_edge_justifications() {
    for case in parity_corpus() {
        let graph = build_graph(&case);
        let justifications: HashSet<String> = graph
            .edges
            .iter()
            .map(|e| format!("{:?}", e.justification))
            .collect();

        assert!(!justifications.is_empty(), "{} should have edges", case.id);
    }

    let hard_avoid = build_graph(&parity_corpus().into_iter().find(|c| c.id == "hard_avoid").unwrap());
    let has_taboo_pressure = hard_avoid
        .edges
        .iter()
        .any(|e| e.justification == ReasoningEdgeJustification::TabooPressure);
    assert!(has_taboo_pressure, "hard_avoid should have TabooPressure edge");
}

#[test]
fn semantic_projection_preserves_node_tags() {
    let graph = build_graph(&parity_corpus().into_iter().find(|c| c.id == "hard_avoid").unwrap());
    let taboo = graph.nodes.iter().find(|n| n.id == "fact.day.taboos").expect("taboo");
    assert!(taboo.tags.iter().any(|t| t == "resistance"), "taboo should have resistance tag");
    assert!(taboo.tags.iter().any(|t| t == "day"), "taboo should have day tag");
}

#[test]
fn semantic_projection_preserves_edge_tags() {
    let graph = build_graph(&parity_corpus().into_iter().find(|c| c.id == "hard_avoid").unwrap());
    let has_override_edge = graph.edges.iter().any(|e| e.tags.iter().any(|t| t == "override"));
    assert!(has_override_edge, "hard_avoid should have override-tagged edge");
}

#[test]
fn semantic_projection_preserves_evidence_provenance() {
    for case in parity_corpus() {
        let graph = build_graph(&case);
        let all_evidence: Vec<_> = graph.nodes.iter().flat_map(|n| n.evidence.iter()).collect();
        assert!(!all_evidence.is_empty(), "{} should have evidence", case.id);

        for ev in &all_evidence {
            assert!(!ev.source_id.is_empty(), "{} evidence should have source_id", case.id);
            assert!(!ev.method.is_empty(), "{} evidence should have method", case.id);
        }
    }
}

#[test]
fn semantic_projection_preserves_personal_nodes() {
    let graph = build_graph(
        &parity_corpus()
            .into_iter()
            .find(|c| c.id == "with_kua_directions")
            .unwrap(),
    );
    let personal_nodes: Vec<_> = graph
        .nodes
        .iter()
        .filter(|n| n.id.starts_with("fact.personal."))
        .collect();
    assert!(!personal_nodes.is_empty(), "should have personal nodes");

    let has_personal_tag = graph
        .nodes
        .iter()
        .any(|n| n.tags.iter().any(|t| t == "personal"));
    assert!(has_personal_tag, "should have personal-tagged nodes");
}

#[test]
fn semantic_projection_preserves_personal_edge_justifications() {
    let graph = build_graph(
        &parity_corpus()
            .into_iter()
            .find(|c| c.id == "with_kua_directions")
            .unwrap(),
    );
    let has_personal_alignment = graph.edges.iter().any(|e| {
        e.justification == ReasoningEdgeJustification::PersonalDayAlignment
            || e.justification == ReasoningEdgeJustification::PersonalHourAlignment
    });
    assert!(
        has_personal_alignment,
        "personal case should have PersonalDayAlignment or PersonalHourAlignment edge"
    );
}

#[test]
fn semantic_projection_preserves_interaction_evidence() {
    let graph = build_graph(
        &parity_corpus()
            .into_iter()
            .find(|c| c.id == "with_kua_directions")
            .unwrap(),
    );
    let has_interaction = graph
        .nodes
        .iter()
        .flat_map(|n| n.evidence.iter())
        .any(|e| e.source_family == amlich_core::ReasoningEvidenceSourceFamily::Interaction);
    assert!(has_interaction, "personal case should have interaction evidence");
}

#[test]
fn semantic_projection_preserves_signal_axis_annotations() {
    for case in parity_corpus() {
        let graph = build_graph(&case);
        let signal_nodes: Vec<_> = graph
            .nodes
            .iter()
            .filter(|n| n.id.starts_with("signal."))
            .collect();
        assert_eq!(signal_nodes.len(), 6, "{} should have 6 signal nodes", case.id);

        for node in &signal_nodes {
            assert!(node.axis.is_some(), "signal node {} should have axis", node.id);
        }
    }
}

#[test]
fn semantic_projection_preserves_node_kinds() {
    for case in parity_corpus() {
        let graph = build_graph(&case);
        let fact_count = graph.nodes.iter().filter(|n| matches!(n.kind, amlich_core::NodeKind::Fact)).count();
        let signal_count = graph
            .nodes
            .iter()
            .filter(|n| matches!(n.kind, amlich_core::NodeKind::InterpretedSignal))
            .count();
        assert!(fact_count >= 8, "{} should have at least 8 fact nodes", case.id);
        assert_eq!(signal_count, 6, "{} should have exactly 6 signal nodes", case.id);
    }
}

#[test]
fn semantic_projection_preserves_serialization_shape() {
    let graph = build_graph(&parity_corpus().first().unwrap());
    let value = serde_json::to_value(&graph).expect("serialize");
    let obj = value.as_object().expect("graph object");

    assert!(obj.contains_key("action_id"));
    assert!(obj.contains_key("nodes"));
    assert!(obj.contains_key("edges"));

    let nodes = value.pointer("/nodes").and_then(serde_json::Value::as_array).expect("nodes");
    assert!(!nodes.is_empty());

    let first = nodes[0].as_object().expect("node");
    for key in &["id", "kind", "summary_vi", "evidence"] {
        assert!(first.contains_key(*key), "node should have key: {key}");
    }
}
