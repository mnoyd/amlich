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
    let chart =
        get_personal_day_chart(&sample_query(), Some(1990), Some(1), Some(1), Some(sample_gender()))
            .expect("chart");
    assert_eq!(chart.input.birth_year, Some(1990));
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
    assert!(analysis.tu_menh.is_some());
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
    assert!(!metrics.available_sections.is_empty());
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
    assert!(!advisory.highlights.is_empty() || !advisory.cautions.is_empty());
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
    assert!(report.chart.canchi.is_some());
    assert!(!report.computed_metrics.available_sections.is_empty());
}
