use amlich_api::{
    get_personal_day_advisory, get_personal_day_analysis, get_personal_day_chart,
    get_personal_day_metrics, get_personal_day_report, DateQuery,
};

fn sample_query() -> DateQuery {
    DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

fn sample_gender() -> amlich_core::almanac::tu_menh::Gender {
    amlich_core::almanac::tu_menh::Gender::Male
}

#[test]
fn personal_day_chart_exposes_contract_shape() {
    let chart = get_personal_day_chart(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("chart");
    assert_eq!(chart.input.birth_year, Some(1990));
    assert_eq!(chart.tier, amlich_api::BirthDataTierDto::Date);
    assert_eq!(chart.solar.day, 10);
    assert!(chart.canchi.is_some());
}

#[test]
fn personal_day_analysis_exposes_profile_sections() {
    let analysis = get_personal_day_analysis(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("analysis");
    assert_eq!(analysis.tier, amlich_api::BirthDataTierDto::Date);
    assert!(analysis.decision.is_some());
    assert!(analysis.decision_export.is_some());
    assert!(analysis.graph.is_some());
    assert!(analysis.tu_menh.is_some());
    assert!(analysis.yearly_han.is_some());
    assert!(analysis.unavailable_sections.is_empty());
}

#[test]
fn personal_day_analysis_yearly_han_has_all_components() {
    let analysis = get_personal_day_analysis(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("analysis");
    let han = analysis.yearly_han.expect("yearly_han should be present");
    assert!(!han.sao_han.star_name.is_empty());
    assert!(!han.tam_tai.tam_hop_group.is_empty());
    assert!(han.kim_lau.tuoi_mu > 0);
    assert!(han.hoang_oc.tuoi_mu > 0);
    assert!(!han.severity.is_empty());
}

#[test]
fn personal_day_analysis_yearly_han_absent_without_birth_year() {
    let analysis =
        get_personal_day_analysis(&sample_query(), None, None, None, None).expect("analysis");
    assert!(analysis.yearly_han.is_none());
    assert_eq!(analysis.tier, amlich_api::BirthDataTierDto::Anonymous);
    assert!(analysis.decision.is_none());
    assert!(analysis.decision_export.is_none());
    assert!(analysis.graph.is_none());
    assert!(!analysis.unavailable_sections.is_empty());
    assert!(analysis
        .unavailable_sections
        .iter()
        .any(|section| section.section == "yearly_han"));
}

#[test]
fn personal_day_metrics_expose_available_sections() {
    let metrics = get_personal_day_metrics(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("metrics");
    assert!(metrics.profile_completeness >= 2);
    assert_eq!(metrics.tier, amlich_api::BirthDataTierDto::Date);
    assert!(!metrics.available_sections.is_empty());
    assert!(metrics
        .available_sections
        .contains(&"yearly_han".to_string()));
    assert!(metrics.unavailable_sections.is_empty());
}

#[test]
fn personal_day_advisory_exposes_highlights_or_cautions() {
    let advisory = get_personal_day_advisory(
        &sample_query(),
        Some(1990),
        None,
        None,
        Some(sample_gender()),
    )
    .expect("advisory");
    assert!(!advisory.summary.is_empty());
    assert!(matches!(
        advisory.severity.as_str(),
        "low" | "medium" | "high"
    ));
    assert!(!advisory.top_signals.is_empty());
    assert!(!advisory.why_this_matters.is_empty());
    assert!(!advisory.recommended_actions.is_empty());
    assert!(!advisory.priority_order.is_empty());
    assert!(!advisory.highlights.is_empty() || !advisory.cautions.is_empty());
}

/// Regression for amlich-mwbp.5: a profile with NO birth date (fully
/// anonymous) must accumulate missing-context messages in
/// `unavailable_context`, but those messages must NOT push `severity`
/// above "low". Previously the missing-context strings were pushed into
/// `cautions` and could lift severity to "medium" or "high" purely from
/// absence-of-data.
#[test]
fn personal_day_advisory_missing_context_does_not_inflate_severity() {
    let advisory =
        get_personal_day_advisory(&sample_query(), None, None, None, None).expect("advisory");

    // Sanity: missing-profile messages are present somewhere.
    assert!(
        !advisory.unavailable_context.is_empty(),
        "expected missing-context messages in unavailable_context; got {:?}",
        advisory.unavailable_context
    );

    // Severity must NOT have been lifted by missing-context strings. With
    // no genuine adverse day signals and no reasoning resistances, the
    // result must be "low".
    assert_eq!(
        advisory.severity, "low",
        "missing-context must not inflate severity; cautions={:?}",
        advisory.cautions
    );

    // `cautions` must NOT contain the historical missing-context strings.
    for forbidden in [
        "missing kua profile context",
        "missing dai van timing context",
        "ten gods analysis unavailable",
    ] {
        assert!(
            !advisory.cautions.iter().any(|c| c.contains(forbidden)),
            "cautions must not contain missing-context message {:?}; got {:?}",
            forbidden,
            advisory.cautions
        );
    }
}

#[test]
fn personal_day_report_exposes_unified_surface() {
    let report = get_personal_day_report(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("report");
    assert!(!report.summary.is_empty());
    assert!(matches!(
        report.severity.as_str(),
        "low" | "medium" | "high"
    ));
    assert!(!report.top_signals.is_empty());
    assert!(report.decision.is_some());
    assert!(report.decision_export.is_some());
    assert!(report.graph.is_some());
    assert_eq!(report.summary, report.advisory.summary);
    assert_eq!(report.severity, report.advisory.severity);
    assert_eq!(report.top_signals, report.advisory.top_signals);
    assert_eq!(report.chart.tier, amlich_api::BirthDataTierDto::Date);
    assert!(report.chart.canchi.is_some());
    assert!(!report.computed_metrics.available_sections.is_empty());
}

#[test]
fn personal_day_report_omits_reasoning_bundle_without_full_birth_data() {
    let report = get_personal_day_report(
        &sample_query(),
        Some(1990),
        None,
        None,
        Some(sample_gender()),
    )
    .expect("report");

    assert!(report.decision.is_none());
    assert!(report.decision_export.is_none());
    assert!(report.graph.is_none());
}

#[test]
fn personal_day_advisory_aligns_with_reasoning_bundle() {
    let report = get_personal_day_report(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("report");

    let decision = report.decision.as_ref().expect("decision");
    let advisory = &report.advisory;

    assert!(advisory.reasoning_bucket.is_some());
    assert!(advisory.reasoning_confidence.is_some());
    assert_eq!(
        advisory.reasoning_bucket.as_deref(),
        Some(decision.recommendation_bucket.as_str())
    );

    let analysis = &report.analysis;
    let analysis_decision = analysis.decision.as_ref().expect("analysis decision");
    assert_eq!(
        analysis_decision.primary_conclusion,
        decision.primary_conclusion
    );
    assert_eq!(
        analysis_decision.recommendation_bucket,
        decision.recommendation_bucket
    );
    assert_eq!(analysis_decision.confidence, decision.confidence);
}

#[test]
fn personal_day_advisory_incorporates_reasoning_signals() {
    let advisory = get_personal_day_advisory(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("advisory");

    assert!(advisory.reasoning_bucket.is_some());
    assert!(advisory
        .top_signals
        .iter()
        .any(|signal| signal.contains("khởi sự")
            || signal.contains("thận trọng")
            || signal.contains("Không nên")));
}

#[test]
fn personal_day_report_analysis_and_advisory_keep_reasoning_fields_aligned() {
    let report = get_personal_day_report(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("report");

    let report_decision = report.decision.as_ref().expect("report decision");
    let report_export = report
        .decision_export
        .as_ref()
        .expect("report decision export");
    let analysis_decision = report
        .analysis
        .decision
        .as_ref()
        .expect("analysis decision");
    let analysis_export = report
        .analysis
        .decision_export
        .as_ref()
        .expect("analysis decision export");
    let advisory = &report.advisory;

    assert_eq!(
        report_decision.primary_conclusion,
        analysis_decision.primary_conclusion
    );
    assert_eq!(
        report_decision.recommendation_bucket,
        analysis_decision.recommendation_bucket
    );
    assert_eq!(report_decision.confidence, analysis_decision.confidence);
    assert_eq!(report_export.semantic, analysis_export.semantic);
    assert_eq!(report_export.axis_scores, analysis_export.axis_scores);
    assert_eq!(report.graph, report.analysis.graph);
    assert_eq!(
        advisory.reasoning_bucket.as_deref(),
        Some(report_decision.recommendation_bucket.as_str())
    );
}

#[test]
fn personal_day_reasoning_bundle_stays_absent_consistently_when_profile_is_incomplete() {
    let report = get_personal_day_report(
        &sample_query(),
        Some(1990),
        None,
        None,
        Some(sample_gender()),
    )
    .expect("report");

    assert!(report.decision.is_none());
    assert!(report.decision_export.is_none());
    assert!(report.graph.is_none());
    assert!(report.analysis.decision.is_none());
    assert!(report.analysis.decision_export.is_none());
    assert!(report.analysis.graph.is_none());
    assert!(report.advisory.reasoning_bucket.is_none());
    assert!(report.advisory.reasoning_confidence.is_none());
}
