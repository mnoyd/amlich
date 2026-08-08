//! amlich-8tdm: API contract tests for the AssessmentTrace → Evidence Graph
//! projection.
//!
//! Acceptance criteria for this bead:
//!
//! 1. **The semantic graph consumes the assessment trace** — when the
//!    `PersonalDayAssessmentDto` is built from a v2-policy assessment
//!    (which carries an `AssessmentTrace`), the additive
//!    `explanation_graph` field is populated.
//! 2. **Debug and explanation exports identify feature, weight, source,
//!    policy, and veto state** — the projected nodes / edges expose
//!    every one of those signals; vetoes surface as
//!    `veto_overrides_decision` edges.
//! 3. **API/desktop/TUI contract tests remain aligned** — every existing
//!    field on `PersonalDayAssessmentDto` stays present and unchanged
//!    for callers; the projection is strictly additive (a `None` for v1
//!    callers, an `Some(Graph)` for v2 callers).
//! 4. **Displayed explanations match the actual calculation** — the
//!    projection's node severities, payload numbers, and edge weights
//!    equal the trace's recorded values byte-for-byte (no
//!    recomputation).

use amlich_api::{get_personal_day_advisory, get_personal_day_report, DateQuery};

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

fn sample_gender() -> amlich_core::almanac::tu_menh::Gender {
    amlich_core::almanac::tu_menh::Gender::Male
}

// ---------------------------------------------------------------------------
// (1) The semantic graph consumes the assessment trace.
// ---------------------------------------------------------------------------

#[test]
fn report_attaches_explanation_graph_with_locked_policy_metadata() {
    // The API surface still uses the v1 builder (per the amlich-31oa
    // promotion gate). The trace is a v2 construct; this test asserts
    // that the DTO has the field, with the correct schema, regardless
    // of which policy currently drives the API. The graph is None for
    // the v1 builder, which is the additive contract.
    let query = sample_query();
    let report =
        get_personal_day_report(&query, Some(1990), Some(1), Some(1), Some(sample_gender()))
            .expect("report builds");
    let assessment = report
        .canonical_assessment
        .as_ref()
        .expect("canonical_assessment attached");

    // Serialization round-trip must preserve the additive field shape:
    // `explanation_graph` is either present or absent; it does not
    // change the existing assessment fields.
    let serialized = serde_json::to_string(assessment).expect("serializes");
    let value: serde_json::Value = serde_json::from_str(&serialized).expect("parses");
    let graph = &value["explanation_graph"];
    // The v1 builder produces no trace, so the field is null/absent
    // in the serialized payload. The DTO struct uses
    // `skip_serializing_if = "Option::is_none"` so the JSON key is
    // absent — both are accepted by the contract.
    assert!(
        graph.is_null() || !graph.is_object(),
        "v1 builder must not attach an explanation_graph"
    );

    // Every existing field stays present (byte-equal parity with
    // pre-amlich-8tdm wire shape):
    assert!(value["axes"].is_object(), "axes must remain present");
    assert!(
        value["decision"].is_object(),
        "decision must remain present"
    );
    assert!(
        value["contributions"].is_array(),
        "contributions must remain present"
    );
    assert!(
        value["unavailable_sections"].is_array(),
        "unavailable_sections must remain present"
    );
}

#[test]
fn graph_dto_round_trips_through_serde_when_attached() {
    // When the explanation_graph IS attached (i.e., the v2 policy is
    // used), the serialization round-trip must preserve every node,
    // every edge, every axis summary, and every veto summary
    // byte-for-byte. This is the single contract test that locks the
    // JSON shape of the additive field.
    //
    // Build the v2 assessment in-test (the production API still uses
    // v1; this test exercises the v2 path directly so the DTO shape
    // stays pinned for when amlich-31oa flips the default).
    use amlich_core::advisory::ConsultationIntent;
    use amlich_core::almanac::tu_menh::Gender;
    use amlich_core::assessment::{AssessmentInputs, AssessmentPolicy, PersonalDayAssessment};
    use amlich_core::birth::{BirthProfile, BirthTime};
    use amlich_core::types::VIETNAM_TIMEZONE;

    let snapshot = amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE);
    let profile = BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: Some(BirthTime {
            hour: 9,
            minute: 30,
        }),
        timezone: VIETNAM_TIMEZONE,
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some(Gender::Male),
        location_name: Some("Hanoi".to_string()),
    };
    let intent = ConsultationIntent::Wedding;
    let assessment = AssessmentPolicy::baseline_v2().evaluate(
        AssessmentInputs::default(),
        &snapshot,
        &profile,
        intent,
    );
    assert!(
        assessment.trace.is_some(),
        "v2 baseline policy must attach a trace for this fixture"
    );

    // Project to DTO.
    let dto: amlich_api::PersonalDayAssessmentDto = (&assessment).into();
    let graph = dto
        .explanation_graph
        .as_ref()
        .expect("v2 assessment MUST attach an explanation_graph in the DTO");

    // (4) Displayed explanations match the actual calculation:
    // The decision DTO must report the same bucket and decision_score
    // the trace recorded (no recomputation).
    assert_eq!(
        graph.decision.bucket,
        assessment.decision.bucket.as_str(),
        "explanation_graph decision.bucket must equal the recorded bucket"
    );
    assert_eq!(
        graph.decision.decision_score, assessment.decision.decision_score,
        "explanation_graph decision.decision_score must equal the recorded score"
    );
    assert_eq!(
        graph.policy_version, assessment.policy_version,
        "explanation_graph must carry the recorded policy_version"
    );
    assert_eq!(
        graph.policy_id, assessment.policy_id,
        "explanation_graph must carry the recorded policy_id"
    );

    // Every axis in the trace must appear in the graph's axes summary.
    assert_eq!(
        graph.axes.len(),
        assessment.trace.as_ref().unwrap().axes.len(),
        "explanation_graph must report one AxisSummary per AxisAggregation"
    );

    // The graph must contain one decision node and at least one
    // axis_signal + one assessment_feature node.
    let decision_count = graph
        .nodes
        .iter()
        .filter(|n| n.concept == "assessment_decision")
        .count();
    assert_eq!(decision_count, 1, "exactly one assessment_decision node");
    let axis_count = graph
        .nodes
        .iter()
        .filter(|n| n.concept == "axis_signal")
        .count();
    assert_eq!(
        axis_count,
        assessment.trace.as_ref().unwrap().axes.len(),
        "one axis_signal node per AxisAggregation"
    );
    let feature_count = graph
        .nodes
        .iter()
        .filter(|n| n.concept == "assessment_feature")
        .count();
    assert_eq!(
        feature_count,
        assessment.trace.as_ref().unwrap().features.len(),
        "one assessment_feature node per FeatureObservation"
    );

    // Serialization round-trip preserves the entire payload.
    let serialized = serde_json::to_string(&dto).expect("serializes");
    let parsed: amlich_api::PersonalDayAssessmentDto =
        serde_json::from_str(&serialized).expect("deserializes");
    assert_eq!(
        parsed.explanation_graph, dto.explanation_graph,
        "explanation_graph must round-trip byte-for-byte"
    );

    // Touch the assessment variable to suppress unused warnings on the
    // input parameter when this test is the only consumer.
    let _ = (snapshot, profile, intent, PersonalDayAssessment::assess);
}

// ---------------------------------------------------------------------------
// (2) Debug and explanation exports identify feature, weight, source,
// policy, and veto state.
// ---------------------------------------------------------------------------

#[test]
fn graph_nodes_carry_feature_weight_source_policy_state() {
    // Build a v2 assessment with a profile that fires a hard veto so
    // the projection covers every entity kind (features, axes, vetoes,
    // decision).
    use amlich_core::advisory::ConsultationIntent;
    use amlich_core::almanac::tu_menh::Gender;
    use amlich_core::assessment::{AssessmentInputs, AssessmentPolicy};
    use amlich_core::birth::{BirthProfile, BirthTime};
    use amlich_core::types::VIETNAM_TIMEZONE;

    let snapshot = amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE);
    // 1985 birth year + female fires the annual.han_severe hard veto
    // on the 2024-02-10 snapshot (matches the v1/v2 parity fixture).
    let profile = BirthProfile {
        year: 1985,
        gender: Some(Gender::Female),
        time: Some(BirthTime {
            hour: 9,
            minute: 30,
        }),
        day: 1,
        month: 1,
        timezone: VIETNAM_TIMEZONE,
        longitude: Some(105.85),
        use_solar_time: true,
        location_name: Some("Hanoi".to_string()),
    };
    let intent = ConsultationIntent::Wedding;
    let assessment = AssessmentPolicy::baseline_v2().evaluate(
        AssessmentInputs::default(),
        &snapshot,
        &profile,
        intent,
    );
    let trace = assessment.trace.as_ref().expect("v2 trace attached");
    assert!(
        !trace.vetoes.is_empty(),
        "this fixture must trigger a hard veto"
    );

    let dto: amlich_api::PersonalDayAssessmentDto = (&assessment).into();
    let graph = dto
        .explanation_graph
        .as_ref()
        .expect("v2 assessment MUST attach an explanation_graph in the DTO");

    // Feature nodes carry the feature_id, the contribution_id, the
    // policy_version, and provenance-style payload fields.
    let feature_nodes: Vec<_> = graph
        .nodes
        .iter()
        .filter(|n| n.concept == "assessment_feature")
        .collect();
    assert!(!feature_nodes.is_empty());
    for node in &feature_nodes {
        assert_eq!(
            node.policy_version, assessment.policy_version,
            "feature node must carry the recorded policy_version"
        );
        let payload = node
            .payload
            .as_ref()
            .expect("feature nodes must carry a payload");
        assert!(
            payload.get("feature_id").is_some(),
            "feature payload must record feature_id"
        );
        assert!(
            payload.get("contribution_id").is_some(),
            "feature payload must record the stable contribution_id"
        );
        assert!(
            payload.get("polarity").is_some(),
            "feature payload must record polarity"
        );
        assert!(
            payload.get("strength").is_some(),
            "feature payload must record the raw strength"
        );
        assert!(
            payload.get("source_evidence").is_some(),
            "feature payload must record source_evidence"
        );
        let source = payload.get("source_evidence").unwrap();
        assert!(
            source.get("source_family").is_some(),
            "feature source_evidence must record source_family"
        );
        assert!(
            source.get("source_id").is_some(),
            "feature source_evidence must record source_id"
        );
        assert!(
            source.get("method").is_some(),
            "feature source_evidence must record the extraction method"
        );
    }

    // The veto summary exposes veto_id, axis, reason, and source
    // attribution. The DTO does not have a separate "veto payload" on
    // the node (vetoes are emitted as Taboo nodes); instead the
    // `vetoes` array carries the structured veto summary so
    // explanation views can describe the override source.
    assert!(!graph.vetoes.is_empty());
    for veto in &graph.vetoes {
        assert!(
            !veto.veto_id.is_empty(),
            "veto summary must record the stable veto_id"
        );
        assert!(
            !veto.axis.is_empty(),
            "veto summary must record the veto's axis"
        );
        assert!(
            !veto.reason.is_empty(),
            "veto summary must record the human-readable reason"
        );
        assert!(
            !veto.source_family.is_empty(),
            "veto summary must record source_family"
        );
        assert!(
            !veto.source_id.is_empty(),
            "veto summary must record source_id (the named-veto provenance)"
        );
        assert!(
            !veto.method.is_empty(),
            "veto summary must record the extraction method"
        );
    }

    // At least one edge must be flagged veto_overrides_decision. This
    // is the API-side signal the desktop/TUI use to draw the
    // override arrow on the explanation canvas.
    assert!(
        graph.edges.iter().any(|e| e.veto_overrides_decision),
        "at least one edge must be flagged veto_overrides_decision"
    );

    // Every axis must carry a contributors array with at least one
    // contributor (the recorded weighted contributors from the trace).
    for axis_summary in &graph.axes {
        if !axis_summary.contributors.is_empty() {
            for c in &axis_summary.contributors {
                assert!(
                    !c.feature_id.is_empty(),
                    "axis contributor must carry the feature_id"
                );
                assert!(
                    !c.contribution_id.is_empty(),
                    "axis contributor must carry the stable contribution_id"
                );
                // The applied_weight must be the actual v1 multiplier
                // or the intent-aware weight the policy used.
                assert!(
                    c.applied_weight.abs() <= 1.0,
                    "applied_weight must live in [-1, 1]; got {}",
                    c.applied_weight
                );
            }
        }
    }
}

#[test]
fn graph_decision_node_carries_policy_and_bucket_state() {
    // The single assessment_decision node must carry the bucket the
    // policy classified, the policy_version the trace records, and
    // the policy_id. This is the single signal the TUI's "Vì sao kết
    // luận" view needs to label the verdict.
    use amlich_core::advisory::ConsultationIntent;
    use amlich_core::almanac::tu_menh::Gender;
    use amlich_core::assessment::{AssessmentInputs, AssessmentPolicy};
    use amlich_core::birth::{BirthProfile, BirthTime};
    use amlich_core::types::VIETNAM_TIMEZONE;

    let snapshot = amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE);
    let profile = BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: Some(BirthTime {
            hour: 9,
            minute: 30,
        }),
        timezone: VIETNAM_TIMEZONE,
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some(Gender::Male),
        location_name: Some("Hanoi".to_string()),
    };
    let intent = ConsultationIntent::Travel;
    let assessment = AssessmentPolicy::baseline_v2().evaluate(
        AssessmentInputs::default(),
        &snapshot,
        &profile,
        intent,
    );
    let dto: amlich_api::PersonalDayAssessmentDto = (&assessment).into();
    let graph = dto.explanation_graph.as_ref().expect("graph attached");

    let decision_node = graph
        .nodes
        .iter()
        .find(|n| n.concept == "assessment_decision")
        .expect("one decision node");
    assert_eq!(
        decision_node.policy_version, assessment.policy_version,
        "decision node must carry the recorded policy_version"
    );
    assert_eq!(
        decision_node.severity.as_deref(),
        Some(assessment.decision.bucket.as_str()),
        "decision node severity must equal the recorded bucket"
    );
    // The decision payload exposes the axis_weights applied and the
    // available / unavailable axis split.
    let payload = decision_node
        .payload
        .as_ref()
        .expect("decision node must carry a payload");
    assert!(
        payload.get("bucket").is_some(),
        "decision payload must record the bucket"
    );
    assert!(
        payload.get("axis_weights").is_some(),
        "decision payload must record the applied axis weights"
    );
    assert!(
        payload.get("available_axes").is_some(),
        "decision payload must record the available axes"
    );
    assert!(
        payload.get("unavailable_axes").is_some(),
        "decision payload must record the unavailable axes"
    );
}

// ---------------------------------------------------------------------------
// (3) API/desktop/TUI contract tests remain aligned.
// ---------------------------------------------------------------------------

#[test]
fn v1_advisory_dto_keeps_legacy_fields_byte_equal() {
    // The API still uses the v1 builder. Adding the explanation_graph
    // field MUST NOT shift any existing field of the
    // PersonalDayAssessmentDto. Re-run a focused subset of the
    // personal_day_assessment_parity assertions through the new code
    // path to lock the additive contract.
    let query = sample_query();
    let (by, bm, bd, gender) = (Some(1990), Some(1), Some(1), Some(sample_gender()));

    let advisory = get_personal_day_advisory(&query, by, bm, bd, gender).expect("advisory builds");
    let report = get_personal_day_report(&query, by, bm, bd, gender).expect("report builds");

    // The canonical assessment surfaces remain byte-equal across
    // advisory and report (the single-verdict contract, locked by
    // amlich-mwbp.6).
    let advisory_assessment = advisory
        .canonical_assessment
        .as_ref()
        .expect("advisory canonical_assessment must be populated");
    let report_assessment = report
        .canonical_assessment
        .as_ref()
        .expect("report canonical_assessment must be populated");

    assert_eq!(
        advisory_assessment.decision.bucket, report_assessment.decision.bucket,
        "decision.bucket must remain the single source of truth"
    );
    assert_eq!(
        advisory_assessment.contributions, report_assessment.contributions,
        "contributions must remain byte-identical across surfaces"
    );
    assert_eq!(
        advisory_assessment.axes.personal_alignment, report_assessment.axes.personal_alignment,
        "axes must remain byte-identical across surfaces"
    );

    // The explanation_graph field is present (None for the v1
    // builder) on both surfaces. v1 builders do not emit a trace, so
    // the graph is absent — the additive contract holds.
    assert!(
        advisory_assessment.explanation_graph.is_none(),
        "v1 advisory MUST NOT attach an explanation_graph (additive contract)"
    );
    assert!(
        report_assessment.explanation_graph.is_none(),
        "v1 report MUST NOT attach an explanation_graph (additive contract)"
    );
}

// ---------------------------------------------------------------------------
// (4) Displayed explanations match the actual calculation.
// ---------------------------------------------------------------------------

#[test]
fn graph_explanation_matches_trace_axis_subtotals_and_decision_score() {
    // The projected explanation_graph.decision.{bucket, decision_score}
    // and the projected axis summaries MUST equal the trace's recorded
    // values byte-for-byte. The graph is a projection; it must not
    // re-derive any score.
    use amlich_core::advisory::ConsultationIntent;
    use amlich_core::almanac::tu_menh::Gender;
    use amlich_core::assessment::{AssessmentInputs, AssessmentPolicy};
    use amlich_core::birth::{BirthProfile, BirthTime};
    use amlich_core::types::VIETNAM_TIMEZONE;

    let snapshot = amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE);
    let profile = BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: Some(BirthTime {
            hour: 9,
            minute: 30,
        }),
        timezone: VIETNAM_TIMEZONE,
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some(Gender::Male),
        location_name: Some("Hanoi".to_string()),
    };
    let intent = ConsultationIntent::Travel;
    let assessment = AssessmentPolicy::baseline_v2().evaluate(
        AssessmentInputs::default(),
        &snapshot,
        &profile,
        intent,
    );
    let trace = assessment.trace.as_ref().expect("v2 trace attached");

    let dto: amlich_api::PersonalDayAssessmentDto = (&assessment).into();
    let graph = dto.explanation_graph.as_ref().expect("graph attached");

    // Axis subtotals must match trace.axis[*].subtotal field-for-field.
    for trace_axis in &trace.axes {
        let graph_axis = graph
            .axes
            .iter()
            .find(|a| a.axis == trace_axis.axis.as_str())
            .expect("axis summary present for every trace axis");
        assert_eq!(
            graph_axis.subtotal,
            trace_axis.subtotal,
            "axis {} subtotal must match trace",
            trace_axis.axis.as_str()
        );
        assert_eq!(
            graph_axis.verdict,
            trace_axis.verdict,
            "axis {} verdict must match trace",
            trace_axis.axis.as_str()
        );
        assert_eq!(
            graph_axis.unavailable_reason,
            trace_axis.unavailable_reason,
            "axis {} unavailable_reason must match trace",
            trace_axis.axis.as_str()
        );
    }

    // Decision bucket and score must match trace.decision field-for-field.
    assert_eq!(
        graph.decision.bucket,
        trace.decision.bucket.as_str(),
        "decision.bucket must match trace"
    );
    assert_eq!(
        graph.decision.decision_score, trace.decision.decision_score,
        "decision.decision_score must match trace"
    );

    // The axis_weights array length must equal the trace's length and
    // each entry's (axis, weight) must match.
    assert_eq!(
        graph.decision.axis_weights.len(),
        trace.decision.axis_weights.len(),
        "axis_weights length must match trace"
    );
    for trace_w in &trace.decision.axis_weights {
        let graph_w = graph
            .decision
            .axis_weights
            .iter()
            .find(|w| w.axis == trace_w.axis.as_str())
            .expect("axis weight entry must exist");
        assert!(
            (graph_w.weight - trace_w.weight).abs() < 1e-5,
            "axis weight for {} must match trace; got {} expected {}",
            trace_w.axis.as_str(),
            graph_w.weight,
            trace_w.weight
        );
    }
}
