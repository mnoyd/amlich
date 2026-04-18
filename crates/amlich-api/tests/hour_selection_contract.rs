use amlich_api::{
    get_hour_selection_advisory, get_hour_selection_analysis, get_hour_selection_chart,
    get_hour_selection_metrics, get_hour_selection_report, DateQuery,
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

#[test]
fn hour_selection_chart_exposes_contract_shape() {
    let chart = get_hour_selection_chart(&sample_query()).expect("chart");
    assert_eq!(chart.solar.day, 10);
    assert_eq!(chart.gio_hoang_dao.all_hours.len(), 12);
}

#[test]
fn hour_selection_analysis_exposes_good_and_bad_hours() {
    let analysis = get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
    assert_eq!(analysis.intent, "travel");
    assert!(!analysis.summary_vi.is_empty());
    assert!(!analysis.summary_en.is_empty());
    assert!(!analysis.good_hours.is_empty());
    assert!(!analysis.bad_hours.is_empty());
    assert!(analysis.top_recommendation.is_some());
}

#[test]
fn hour_selection_analysis_top_recommendation_matches_best_window() {
    let analysis = get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
    let advisory = get_hour_selection_advisory(&sample_query(), None, None, None, None).expect("advisory");

    let top = analysis.top_recommendation.expect("top recommendation");
    let expected = format!("{} {}", top.hour_chi, top.time_range);
    assert_eq!(advisory.best_windows.first().map(String::as_str), Some(expected.as_str()));
}

#[test]
fn hour_selection_metrics_expose_distribution() {
    let metrics = get_hour_selection_metrics(&sample_query()).expect("metrics");
    assert_eq!(metrics.good_hour_count + metrics.bad_hour_count, 12);
}

#[test]
fn hour_selection_advisory_exposes_windows() {
    let advisory = get_hour_selection_advisory(&sample_query(), None, None, None, None).expect("advisory");
    assert_eq!(advisory.intent, "travel");
    assert!(!advisory.summary_vi.is_empty());
    assert!(!advisory.summary_en.is_empty());
    assert!(!advisory.best_windows.is_empty());
    assert!(!advisory.caution_windows.is_empty());
}

#[test]
fn hour_selection_report_exposes_unified_surface() {
    let report = get_hour_selection_report(&sample_query(), None, None, None, None).expect("report");
    assert_eq!(
        report.chart.gio_hoang_dao.good_hour_count,
        report.computed_metrics.good_hour_count
    );
    assert_eq!(report.analysis.intent, report.advisory.intent);
    assert_eq!(report.analysis.summary_vi, report.advisory.summary_vi);
    assert_eq!(report.analysis.summary_en, report.advisory.summary_en);
    assert!(report.analysis.top_recommendation.is_some());
    assert!(!report.advisory.best_windows.is_empty());
    let top = report.analysis.top_recommendation.expect("top recommendation");
    let expected = format!("{} {}", top.hour_chi, top.time_range);
    assert_eq!(report.advisory.best_windows.first().map(String::as_str), Some(expected.as_str()));
}

#[test]
fn hour_selection_canonical_export_is_present_in_analysis_and_advisory() {
    let analysis = get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
    let advisory = get_hour_selection_advisory(&sample_query(), None, None, None, None).expect("advisory");

    let canonical = analysis.canonical.as_ref().expect("analysis canonical");
    assert_eq!(canonical.intent, "travel");
    assert_eq!(canonical.birth_data_tier, "anonymous");
    assert!(!canonical.summary_vi.is_empty());
    assert!(canonical.top_recommendation.is_some());
    assert!(!canonical.ranked_hours.is_empty());
    assert_eq!(canonical.total_hours, 12);
    assert!(canonical.auspicious_count > 0);
    assert!(!canonical.evidence.is_empty());

    let advisory_canonical = advisory.canonical.as_ref().expect("advisory canonical");
    assert_eq!(canonical.intent, advisory_canonical.intent);
    assert_eq!(canonical.summary_vi, advisory_canonical.summary_vi);
    assert_eq!(canonical.top_recommendation, advisory_canonical.top_recommendation);
    assert_eq!(canonical.ranked_hours, advisory_canonical.ranked_hours);
}

#[test]
fn hour_selection_canonical_export_reflects_birth_data_tier() {
    let with_birth = get_hour_selection_analysis(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some("male"),
    )
    .expect("with birth");
    let canonical = with_birth.canonical.as_ref().expect("canonical");
    assert_eq!(canonical.birth_data_tier, "date");
    assert!(canonical
        .evidence
        .iter()
        .any(|e| e.source_family == "birth_input"));
}

#[test]
fn hour_selection_canonical_export_serializes_cleanly() {
    let analysis = get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
    let canonical = analysis.canonical.as_ref().expect("canonical");
    let value = serde_json::to_value(canonical).expect("serialize");
    let obj = value.as_object().expect("object");

    let required = [
        "intent",
        "birth_data_tier",
        "summary_vi",
        "summary_en",
        "top_recommendation",
        "ranked_hours",
        "auspicious_count",
        "total_hours",
        "evidence",
    ];
    for key in &required {
        assert!(obj.contains_key(*key), "missing key: {key}");
    }
}

#[test]
fn hour_selection_report_analysis_and_advisory_keep_canonical_export_aligned() {
    let report = get_hour_selection_report(&sample_query(), None, None, None, None).expect("report");

    let analysis_canonical = report.analysis.canonical.as_ref().expect("analysis canonical");
    let advisory_canonical = report.advisory.canonical.as_ref().expect("advisory canonical");

    assert_eq!(analysis_canonical.intent, advisory_canonical.intent);
    assert_eq!(analysis_canonical.birth_data_tier, advisory_canonical.birth_data_tier);
    assert_eq!(analysis_canonical.summary_vi, advisory_canonical.summary_vi);
    assert_eq!(analysis_canonical.summary_en, advisory_canonical.summary_en);
    assert_eq!(analysis_canonical.top_recommendation, advisory_canonical.top_recommendation);
    assert_eq!(analysis_canonical.ranked_hours, advisory_canonical.ranked_hours);
    assert_eq!(analysis_canonical.auspicious_count, advisory_canonical.auspicious_count);
    assert_eq!(analysis_canonical.total_hours, advisory_canonical.total_hours);
}

#[test]
fn hour_selection_birth_tier_is_stable_across_analysis_advisory_and_report() {
    let report = get_hour_selection_report(&sample_query(), Some(1990), Some(1), Some(1), Some("male"))
        .expect("report");

    let analysis_canonical = report.analysis.canonical.as_ref().expect("analysis canonical");
    let advisory_canonical = report.advisory.canonical.as_ref().expect("advisory canonical");

    assert_eq!(analysis_canonical.birth_data_tier, "date");
    assert_eq!(analysis_canonical.birth_data_tier, advisory_canonical.birth_data_tier);
    assert!(analysis_canonical
        .evidence
        .iter()
        .any(|e| e.source_family == "birth_input"));
    assert!(advisory_canonical
        .evidence
        .iter()
        .any(|e| e.source_family == "birth_input"));
}
