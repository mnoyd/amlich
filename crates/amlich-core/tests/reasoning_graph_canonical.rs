use amlich_core::{
    build_initiation_opening_reasoning_bundle, build_initiation_opening_reasoning,
    calculate_day_snapshot, calculate_day_snapshot_with_timezone,
    reasoning::{
        PersonalReasoningInput, RecommendationBucket,
        ReasoningNodeSeverity,
    },
    BirthInput, ConsultationIntent, Gender,
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

struct CanonicalCase {
    id: &'static str,
    day: i32,
    month: i32,
    year: i32,
    timezone: Option<f64>,
    personal: Option<PersonalReasoningInput>,
    expected_bucket: RecommendationBucket,
}

fn canonical_corpus() -> Vec<CanonicalCase> {
    vec![
        CanonicalCase {
            id: "baseline_favorable",
            day: 13,
            month: 5,
            year: 2024,
            timezone: None,
            personal: None,
            expected_bucket: RecommendationBucket::Favorable,
        },
        CanonicalCase {
            id: "baseline_avoid",
            day: 3,
            month: 1,
            year: 2024,
            timezone: None,
            personal: None,
            expected_bucket: RecommendationBucket::Avoid,
        },
        CanonicalCase {
            id: "baseline_cautious",
            day: 14,
            month: 2,
            year: 2024,
            timezone: None,
            personal: None,
            expected_bucket: RecommendationBucket::Cautious,
        },
        CanonicalCase {
            id: "personal_favorable_with_hour",
            day: 13,
            month: 5,
            year: 2024,
            timezone: None,
            personal: Some(profile_input(1, 1, 1990, 9, 0, 7.0, None)),
            expected_bucket: RecommendationBucket::Favorable,
        },
        CanonicalCase {
            id: "personal_with_kua_directions",
            day: 15,
            month: 6,
            year: 2024,
            timezone: None,
            personal: Some(profile_input(12, 8, 1992, 11, 30, 7.0, Some(Gender::Female))),
            expected_bucket: RecommendationBucket::Cautious,
        },
        CanonicalCase {
            id: "personal_avoid_with_profile",
            day: 1,
            month: 2,
            year: 2024,
            timezone: None,
            personal: Some(profile_input(30, 1, 1989, 23, 30, 7.0, Some(Gender::Male))),
            expected_bucket: RecommendationBucket::Avoid,
        },
        CanonicalCase {
            id: "boundary_vn_timezone",
            day: 14,
            month: 2,
            year: 2024,
            timezone: Some(7.0),
            personal: Some(profile_input(30, 1, 1989, 23, 30, 7.0, Some(Gender::Male))),
            expected_bucket: RecommendationBucket::Cautious,
        },
        CanonicalCase {
            id: "boundary_shifted_timezone",
            day: 14,
            month: 2,
            year: 2024,
            timezone: Some(8.0),
            personal: Some(profile_input(30, 1, 1989, 23, 30, 8.0, Some(Gender::Male))),
            expected_bucket: RecommendationBucket::Cautious,
        },
    ]
}

fn snapshot_for(case: &CanonicalCase) -> amlich_core::DaySnapshot {
    match case.timezone {
        Some(tz) => calculate_day_snapshot_with_timezone(case.day, case.month, case.year, tz),
        None => calculate_day_snapshot(case.day, case.month, case.year),
    }
}

fn bundle_value(case: &CanonicalCase) -> Value {
    let snapshot = snapshot_for(case);
    let bundle =
        build_initiation_opening_reasoning_bundle(&snapshot, case.personal.as_ref()).expect("bundle");
    serde_json::to_value(&bundle).expect("serialize")
}

fn require_object<'a>(value: &'a Value, path: &str) -> &'a serde_json::Map<String, Value> {
    value
        .pointer(path)
        .and_then(Value::as_object)
        .unwrap_or_else(|| panic!("expected object at {path}"))
}

fn require_array<'a>(value: &'a Value, path: &str) -> &'a Vec<Value> {
    value
        .pointer(path)
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("expected array at {path}"))
}

fn require_string<'a>(value: &'a Value, path: &str) -> &'a str {
    value
        .pointer(path)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("expected string at {path}"))
}

#[test]
fn canonical_decision_shape_locks_required_fields() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let decision = require_object(&value, "/decision");

    let required_keys = [
        "primary_conclusion",
        "recommendation_bucket",
        "strongest_supports",
        "strongest_resistances",
        "override_factors",
        "conflict_notes",
        "confidence",
        "context_is_clear",
        "suggested_hours",
        "suggested_directions",
    ];
    for key in &required_keys {
        assert!(decision.contains_key(*key), "decision missing key: {key}");
    }
}

#[test]
fn canonical_decision_export_shape_locks_required_fields() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let export = require_object(&value, "/decision_export");

    let required_keys = [
        "primary_conclusion",
        "recommendation_bucket",
        "confidence",
        "context_is_clear",
        "semantic",
        "strongest_supports",
        "strongest_resistances",
        "override_factors",
        "conflict_notes",
        "suggested_hours",
        "suggested_directions",
        "axis_scores",
    ];
    for key in &required_keys {
        assert!(export.contains_key(*key), "decision_export missing key: {key}");
    }

    let axes = require_array(&value, "/decision_export/axis_scores");
    assert_eq!(axes.len(), 6, "axis_scores must have exactly 6 entries");

    for axis in axes {
        let axis_obj = axis.as_object().expect("axis object");
        assert!(axis_obj.contains_key("axis"), "axis entry missing 'axis'");
        assert!(axis_obj.contains_key("score"), "axis entry missing 'score'");
    }
}

#[test]
fn canonical_graph_shape_locks_required_fields() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let graph = require_object(&value, "/graph");

    assert!(graph.contains_key("action_id"));
    assert!(graph.contains_key("nodes"));
    assert!(graph.contains_key("edges"));

    let nodes = require_array(&value, "/graph/nodes");
    assert!(!nodes.is_empty());

    let first_node = nodes[0].as_object().expect("node object");
    let node_keys = ["id", "kind", "summary_vi", "evidence"];
    for key in &node_keys {
        assert!(first_node.contains_key(*key), "node missing key: {key}");
    }

    let edges = require_array(&value, "/graph/edges");
    assert!(!edges.is_empty());

    let first_edge = edges[0].as_object().expect("edge object");
    let edge_keys = [
        "from_node_id",
        "to_node_id",
        "effect",
        "weight",
        "justification",
    ];
    for key in &edge_keys {
        assert!(first_edge.contains_key(*key), "edge missing key: {key}");
    }
}

#[test]
fn canonical_node_export_envelopes_lock_source_family_values() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let nodes = require_array(&value, "/graph/nodes");

    let families: Vec<&str> = nodes
        .iter()
        .flat_map(|node| {
            node.get("evidence")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
        })
        .filter_map(|e| e.get("source_family").and_then(Value::as_str))
        .collect();

    let valid_families = [
        "snapshot",
        "interaction",
        "bazi",
        "axis",
        "almanac_rule",
        "insight",
        "derived",
    ];
    for family in &families {
        assert!(
            valid_families.contains(family),
            "unexpected source_family: {family}"
        );
    }
    assert!(!families.is_empty());
}

#[test]
fn canonical_bucket_values_span_valid_enum() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let bucket = require_string(&value, "/decision/recommendation_bucket");
    assert!(["avoid", "cautious", "mixed", "favorable"].contains(&bucket));
}

#[test]
fn canonical_semantic_values_span_valid_enum() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let semantic = require_string(&value, "/decision_export/semantic");
    let valid = [
        "override_avoid",
        "override_cautious",
        "conflicted_cautious",
        "resistance_led_cautious",
        "favorable_clear",
        "favorable_contextual",
    ];
    assert!(valid.contains(&semantic), "unexpected semantic: {semantic}");
}

#[test]
fn canonical_corpus_bucket_consistency() {
    for case in canonical_corpus() {
        let snapshot = snapshot_for(&case);
        let decision =
            build_initiation_opening_reasoning(&snapshot, case.personal.as_ref()).expect("decision");

        assert_eq!(
            decision.recommendation_bucket,
            case.expected_bucket,
            "{} bucket mismatch",
            case.id
        );
    }
}

#[test]
fn canonical_decision_and_export_stay_aligned_across_corpus() {
    for case in canonical_corpus() {
        let snapshot = snapshot_for(&case);
        let bundle =
            build_initiation_opening_reasoning_bundle(&snapshot, case.personal.as_ref())
                .expect("bundle");

        assert_eq!(
            bundle.decision.primary_conclusion,
            bundle.decision_export.primary_conclusion,
            "{} conclusion drift",
            case.id
        );
        assert_eq!(
            bundle.decision.recommendation_bucket,
            bundle.decision_export.recommendation_bucket,
            "{} bucket drift",
            case.id
        );
        assert_eq!(
            bundle.decision.confidence,
            bundle.decision_export.confidence,
            "{} confidence drift",
            case.id
        );
        assert_eq!(
            bundle.decision.context_is_clear,
            bundle.decision_export.context_is_clear,
            "{} context_is_clear drift",
            case.id
        );
        assert_eq!(
            bundle.decision.strongest_supports.len(),
            bundle.decision_export.strongest_supports.len(),
            "{} supports count drift",
            case.id
        );
        assert_eq!(
            bundle.decision.strongest_resistances.len(),
            bundle.decision_export.strongest_resistances.len(),
            "{} resistances count drift",
            case.id
        );
        assert_eq!(
            bundle.decision.override_factors.len(),
            bundle.decision_export.override_factors.len(),
            "{} overrides count drift",
            case.id
        );
        assert_eq!(
            bundle.decision.conflict_notes.len(),
            bundle.decision_export.conflict_notes.len(),
            "{} conflicts count drift",
            case.id
        );
        assert_eq!(
            bundle.decision.suggested_hours,
            bundle.decision_export.suggested_hours,
            "{} hours drift",
            case.id
        );
        assert_eq!(
            bundle.decision.suggested_directions,
            bundle.decision_export.suggested_directions,
            "{} directions drift",
            case.id
        );
    }
}

#[test]
fn canonical_personal_cases_surface_personal_nodes() {
    for case in canonical_corpus()
        .into_iter()
        .filter(|c| c.personal.is_some())
    {
        let value = bundle_value(&case);
        let nodes = require_array(&value, "/graph/nodes");
        let has_personal = nodes.iter().any(|node| {
            node.get("id")
                .and_then(Value::as_str)
                .is_some_and(|id| id.starts_with("fact.personal."))
        });
        assert!(has_personal, "{} should have personal nodes", case.id);
    }
}

#[test]
fn canonical_boundary_cases_preserve_graph_stability() {
    let case_a = canonical_corpus()
        .into_iter()
        .find(|c| c.id == "boundary_vn_timezone")
        .unwrap();
    let case_b = canonical_corpus()
        .into_iter()
        .find(|c| c.id == "boundary_shifted_timezone")
        .unwrap();

    let value_a = bundle_value(&case_a);
    let value_b = bundle_value(&case_b);

    let nodes_a = require_array(&value_a, "/graph/nodes");
    let nodes_b = require_array(&value_b, "/graph/nodes");

    assert_eq!(nodes_a.len(), nodes_b.len(), "boundary node count drift");
}

#[test]
fn canonical_serialization_roundtrip_is_lossless() {
    for case in canonical_corpus() {
        let snapshot = snapshot_for(&case);
        let bundle =
            build_initiation_opening_reasoning_bundle(&snapshot, case.personal.as_ref())
                .expect("bundle");

        let json = serde_json::to_value(&bundle).expect("to value");
        let re_json = serde_json::to_string(&json).expect("to string");
        let reparsed: Value = serde_json::from_str(&re_json).expect("from string");

        assert_eq!(json, reparsed, "{} roundtrip drift", case.id);
    }
}

#[test]
fn canonical_severity_exports_cover_known_node_ids() {
    let snapshot = calculate_day_snapshot(14, 2, 2024);
    let bundle = build_initiation_opening_reasoning_bundle(&snapshot, None).expect("bundle");

    let taboo_node = bundle
        .graph
        .nodes
        .iter()
        .find(|n| n.id == "fact.day.taboos")
        .expect("taboo node");
    assert!(taboo_node.severity.is_some());

    let deity_node = bundle
        .graph
        .nodes
        .iter()
        .find(|n| n.id == "fact.day.day_deity")
        .expect("deity node");
    assert!(deity_node.severity.is_some());

    let hours_node = bundle
        .graph
        .nodes
        .iter()
        .find(|n| n.id == "fact.day.hoang_dao_hours")
        .expect("hours node");
    assert!(hours_node.severity.is_some());
}

#[test]
fn canonical_node_kinds_are_valid_enum_variants() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let nodes = require_array(&value, "/graph/nodes");
    let valid_kinds = ["fact", "interpreted_signal", "decision_target"];

    for node in nodes {
        let kind = node
            .get("kind")
            .and_then(Value::as_str)
            .expect("node kind");
        assert!(
            valid_kinds.contains(&kind),
            "unexpected node kind: {kind}"
        );
    }
}

#[test]
fn canonical_edge_effects_are_valid_enum_variants() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let edges = require_array(&value, "/graph/edges");
    let valid_effects = [
        "supports",
        "weakens",
        "overrides",
        "conflicts_with",
        "conditions",
    ];

    for edge in edges {
        let effect = edge
            .get("effect")
            .and_then(Value::as_str)
            .expect("edge effect");
        assert!(
            valid_effects.contains(&effect),
            "unexpected edge effect: {effect}"
        );
    }
}

#[test]
fn canonical_edge_justifications_are_valid_enum_variants() {
    let value = bundle_value(canonical_corpus().first().unwrap());
    let edges = require_array(&value, "/graph/edges");
    let valid_justifications = [
        "favorable_day_signal",
        "truc_activity_support",
        "truc_activity_conflict",
        "day_deity_support",
        "star_support",
        "taboo_pressure",
        "taboo_stability_penalty",
        "taboo_context_penalty",
        "clash_pressure",
        "clash_stability_penalty",
        "hoang_dao_hour_support",
        "personal_day_alignment",
        "personal_hour_alignment",
        "mixed_signal_conflict",
        "available_context_support",
    ];

    for edge in edges {
        let justification = edge
            .get("justification")
            .and_then(Value::as_str)
            .expect("edge justification");
        assert!(
            valid_justifications.contains(&justification),
            "unexpected justification: {justification}"
        );
    }
}
