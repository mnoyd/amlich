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
    assert!(!analysis.good_hours.is_empty());
    assert!(!analysis.bad_hours.is_empty());
}

#[test]
fn hour_selection_metrics_expose_distribution() {
    let metrics = get_hour_selection_metrics(&sample_query()).expect("metrics");
    assert_eq!(metrics.good_hour_count + metrics.bad_hour_count, 12);
}

#[test]
fn hour_selection_advisory_exposes_windows() {
    let advisory = get_hour_selection_advisory(&sample_query()).expect("advisory");
    assert!(!advisory.best_windows.is_empty());
    assert!(!advisory.caution_windows.is_empty());
}

#[test]
fn hour_selection_report_exposes_unified_surface() {
    let report = get_hour_selection_report(&sample_query()).expect("report");
    assert_eq!(report.chart.gio_hoang_dao.good_hour_count, report.computed_metrics.good_hour_count);
    assert!(!report.advisory.best_windows.is_empty());
}
