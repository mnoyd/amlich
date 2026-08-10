use amlich_api::{get_personal_day_report, DateQuery};

fn sample_query() -> DateQuery {
    DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

#[test]
fn anonymous_report_exposes_canonical_factor_roles() {
    let report = get_personal_day_report(&sample_query(), None, None, None, None)
        .expect("anonymous report builds");
    let assessment = report
        .canonical_assessment
        .expect("canonical assessment is present");

    for role in ["fact", "scored_feature", "explanation_only"] {
        assert!(
            assessment.factors.iter().any(|factor| factor.role == role),
            "missing role {role}"
        );
    }

    let unavailable = assessment
        .factors
        .iter()
        .find(|factor| factor.availability == "unavailable")
        .expect("anonymous report must expose unavailable factors");
    assert!(unavailable.unavailable_reason.is_some());
}

#[test]
fn factor_contract_round_trips_through_json() {
    let report = get_personal_day_report(&sample_query(), None, None, None, None)
        .expect("anonymous report builds");
    let json = serde_json::to_value(&report).expect("serialize report");
    let factors = json["canonical_assessment"]["factors"]
        .as_array()
        .expect("factor array");

    assert!(!factors.is_empty());
    assert!(factors.iter().all(|factor| factor["factor_id"].is_string()));
    assert!(factors.iter().all(|factor| factor["role"].is_string()));
    assert!(factors
        .iter()
        .all(|factor| factor["availability"].is_string()));
}
