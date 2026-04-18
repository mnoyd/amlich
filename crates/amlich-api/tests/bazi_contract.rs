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

    assert!(!advisory.summary.is_empty());
    assert!(matches!(
        advisory.severity.as_str(),
        "low" | "medium" | "high"
    ));
    assert!(!advisory.top_signals.is_empty());
    assert!(!advisory.why_this_matters.is_empty());
    assert!(!advisory.recommended_actions.is_empty());
    assert!(!advisory.priority_order.is_empty());
    assert!(!advisory.summary_vi.is_empty());
    assert!(!advisory.domains.timing.is_empty());
    assert!(advisory
        .top_signals
        .iter()
        .any(|signal| signal.contains("yong_shen") || signal.contains("xi_shen")));
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

    assert!(!report.summary.is_empty());
    assert!(matches!(
        report.severity.as_str(),
        "low" | "medium" | "high"
    ));
    assert!(!report.top_signals.is_empty());
    assert_eq!(report.chart.pillars.len(), 4);
    assert!(!report.analysis.day_master_strength.label.is_empty());
    assert!(report.timing.is_some());
    assert_eq!(report.summary, report.advisory.summary);
    assert_eq!(report.severity, report.advisory.severity);
    assert_eq!(report.top_signals, report.advisory.top_signals);
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

#[test]
fn bazi_advisory_canonical_export_locks_required_fields() {
    let advisory = get_bazi_advisory(
        &sample_query(),
        Some(&BaziTimingQuery {
            current_age: 15.0,
            target_year: 2027,
            months: vec![1, 2],
        }),
    )
    .expect("advisory");

    let value = serde_json::to_value(&advisory).expect("serialize");
    let obj = value.as_object().expect("object");

    let required = [
        "summary",
        "severity",
        "top_signals",
        "why_this_matters",
        "recommended_actions",
        "priority_order",
        "useful_god_analysis",
        "summary_vi",
        "warnings",
        "domains",
    ];
    for key in &required {
        assert!(obj.contains_key(*key), "bazi advisory missing key: {key}");
    }

    let useful = obj["useful_god_analysis"].as_object().expect("useful god object");
    assert!(useful.contains_key("favorable_elements"));
    assert!(useful.contains_key("unfavorable_elements"));
    assert!(useful.contains_key("confidence"));
    assert!(useful.contains_key("reasons"));

    let domains = obj["domains"].as_object().expect("domains object");
    assert!(domains.contains_key("career"));
    assert!(domains.contains_key("wealth"));
    assert!(domains.contains_key("relationship"));
    assert!(domains.contains_key("health"));
    assert!(domains.contains_key("timing"));
}

#[test]
fn bazi_report_advisory_matches_standalone_advisory() {
    let report = get_bazi_report(
        &sample_query(),
        Some(&BaziTimingQuery {
            current_age: 15.0,
            target_year: 2027,
            months: vec![1, 2],
        }),
    )
    .expect("report");
    let advisory = get_bazi_advisory(
        &sample_query(),
        Some(&BaziTimingQuery {
            current_age: 15.0,
            target_year: 2027,
            months: vec![1, 2],
        }),
    )
    .expect("advisory");

    assert_eq!(report.advisory.summary, advisory.summary);
    assert_eq!(report.advisory.severity, advisory.severity);
    assert_eq!(report.advisory.top_signals, advisory.top_signals);
    assert_eq!(report.advisory.why_this_matters, advisory.why_this_matters);
    assert_eq!(report.advisory.recommended_actions, advisory.recommended_actions);
    assert_eq!(report.advisory.priority_order, advisory.priority_order);
    assert_eq!(report.advisory.summary_vi, advisory.summary_vi);
    assert_eq!(report.advisory.warnings, advisory.warnings);
}
