use amlich_api::{
    get_bazi_advisory, get_bazi_analysis, get_bazi_chart, get_bazi_metrics, get_bazi_report,
    get_bazi_timing, BaziQuery, BaziTimingQuery,
};

fn sample_query() -> BaziQuery {
    BaziQuery {
        day: 10,
        month: 2,
        year: 2024,
        hour: 9,
        minute: 30,
        timezone: Some(7.0),
        longitude: None,
        use_solar_time: false,
        gender: Some("male".to_string()),
    }
}

#[test]
fn bazi_chart_exposes_contract_shape() {
    let chart = get_bazi_chart(&sample_query()).expect("chart");

    assert_eq!(chart.input.day, 10);
    assert_eq!(chart.day_master.full, "Giáp Thìn");
    assert_eq!(chart.pillars.len(), 4);
    assert_eq!(chart.pillars[0].kind, "year");
}

#[test]
fn bazi_analysis_exposes_strength_and_distribution() {
    let analysis = get_bazi_analysis(&sample_query()).expect("analysis");

    assert!(!analysis.day_master_strength.label.is_empty());
    assert!(
        analysis.element_distribution.moc
            + analysis.element_distribution.hoa
            + analysis.element_distribution.tho
            + analysis.element_distribution.kim
            + analysis.element_distribution.thuy
            > 0
    );
}

#[test]
fn bazi_timing_requires_gender_and_returns_months() {
    let timing = get_bazi_timing(
        &sample_query(),
        &BaziTimingQuery {
            current_age: 15.0,
            target_year: 2027,
            months: vec![1, 2],
        },
    )
    .expect("timing");

    assert_eq!(timing.annual.year, 2027);
    assert_eq!(timing.monthly.len(), 2);
}

#[test]
fn bazi_advisory_can_include_timing_context() {
    let advisory = get_bazi_advisory(
        &sample_query(),
        Some(&BaziTimingQuery {
            current_age: 15.0,
            target_year: 2027,
            months: vec![1, 2],
        }),
    )
    .expect("advisory");

    assert!(!advisory.summary_vi.is_empty());
    assert!(!advisory.domains.timing.is_empty());
}

#[test]
fn bazi_metrics_expose_domain_scores_and_timing_windows() {
    let metrics = get_bazi_metrics(
        &sample_query(),
        Some(&BaziTimingQuery {
            current_age: 15.0,
            target_year: 2027,
            months: vec![1, 2],
        }),
    )
    .expect("metrics");

    assert!(metrics.core_metrics.day_master_strength_score > 0);
    assert!(metrics.domain_scores.career.score <= 100);
    assert_eq!(metrics.timing_metrics.monthly_windows.len(), 2);
}

#[test]
fn bazi_report_exposes_unified_surface() {
    let report = get_bazi_report(
        &sample_query(),
        Some(&BaziTimingQuery {
            current_age: 15.0,
            target_year: 2027,
            months: vec![1, 2],
        }),
    )
    .expect("report");

    assert_eq!(report.chart.pillars.len(), 4);
    assert!(!report.analysis.day_master_strength.label.is_empty());
    assert!(report.timing.is_some());
    assert!(!report.advisory.summary_vi.is_empty());
}

#[test]
fn bazi_timing_rejects_missing_gender() {
    let mut query = sample_query();
    query.gender = None;

    let err = get_bazi_timing(
        &query,
        &BaziTimingQuery {
            current_age: 15.0,
            target_year: 2027,
            months: vec![1],
        },
    )
    .expect_err("gender required");

    assert_eq!(
        err,
        "gender is required for bazi timing/advisory. supported values: male, female"
    );
}
