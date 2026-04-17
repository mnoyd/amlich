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
    let analysis = get_hour_selection_analysis(&sample_query()).expect("analysis");
    assert_eq!(analysis.intent, "travel");
    assert!(!analysis.summary_vi.is_empty());
    assert!(!analysis.summary_en.is_empty());
    assert!(!analysis.good_hours.is_empty());
    assert!(!analysis.bad_hours.is_empty());
    assert!(analysis.top_recommendation.is_some());
}

#[test]
fn hour_selection_analysis_top_recommendation_matches_best_window() {
    let analysis = get_hour_selection_analysis(&sample_query()).expect("analysis");
    let advisory = get_hour_selection_advisory(&sample_query()).expect("advisory");

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
    let advisory = get_hour_selection_advisory(&sample_query()).expect("advisory");
    assert_eq!(advisory.intent, "travel");
    assert!(!advisory.summary_vi.is_empty());
    assert!(!advisory.summary_en.is_empty());
    assert!(!advisory.best_windows.is_empty());
    assert!(!advisory.caution_windows.is_empty());
}

#[test]
fn hour_selection_report_exposes_unified_surface() {
    let report = get_hour_selection_report(&sample_query()).expect("report");
    assert_eq!(
        report.chart.gio_hoang_dao.good_hour_count,
        report.computed_metrics.good_hour_count
    );
    assert_eq!(report.analysis.intent, report.advisory.intent);
    assert_eq!(report.analysis.summary_vi, report.advisory.summary_vi);
    assert_eq!(report.analysis.summary_en, report.advisory.summary_en);
    assert!(report.analysis.top_recommendation.is_some());
    assert!(!report.advisory.best_windows.is_empty());
}
