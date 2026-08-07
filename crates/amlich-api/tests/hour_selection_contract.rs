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
    let analysis =
        get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
    assert_eq!(analysis.intent, "travel");
    assert!(!analysis.summary_vi.is_empty());
    assert!(!analysis.summary_en.is_empty());
    assert!(!analysis.good_hours.is_empty());
    assert!(!analysis.bad_hours.is_empty());
    assert!(analysis.top_recommendation.is_some());
}

#[test]
fn hour_selection_analysis_top_recommendation_matches_best_window() {
    let analysis =
        get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
    let advisory =
        get_hour_selection_advisory(&sample_query(), None, None, None, None).expect("advisory");

    let top = analysis.top_recommendation.expect("top recommendation");
    let expected = format!("{} {}", top.hour_chi, top.time_range);
    assert_eq!(
        advisory.best_windows.first().map(String::as_str),
        Some(expected.as_str())
    );
}

#[test]
fn hour_selection_metrics_expose_distribution() {
    let metrics = get_hour_selection_metrics(&sample_query()).expect("metrics");
    assert_eq!(metrics.good_hour_count + metrics.bad_hour_count, 12);
}

#[test]
fn hour_selection_advisory_exposes_windows() {
    let advisory =
        get_hour_selection_advisory(&sample_query(), None, None, None, None).expect("advisory");
    assert_eq!(advisory.intent, "travel");
    assert!(!advisory.summary_vi.is_empty());
    assert!(!advisory.summary_en.is_empty());
    assert!(!advisory.best_windows.is_empty());
    assert!(!advisory.caution_windows.is_empty());
}

#[test]
fn hour_selection_report_exposes_unified_surface() {
    let report =
        get_hour_selection_report(&sample_query(), None, None, None, None).expect("report");
    assert_eq!(
        report.chart.gio_hoang_dao.good_hour_count,
        report.computed_metrics.good_hour_count
    );
    assert_eq!(report.analysis.intent, report.advisory.intent);
    assert_eq!(report.analysis.summary_vi, report.advisory.summary_vi);
    assert_eq!(report.analysis.summary_en, report.advisory.summary_en);
    assert!(report.analysis.top_recommendation.is_some());
    assert!(!report.advisory.best_windows.is_empty());
    let top = report
        .analysis
        .top_recommendation
        .expect("top recommendation");
    let expected = format!("{} {}", top.hour_chi, top.time_range);
    assert_eq!(
        report.advisory.best_windows.first().map(String::as_str),
        Some(expected.as_str())
    );
}

#[test]
fn hour_selection_canonical_export_is_present_in_analysis_and_advisory() {
    let analysis =
        get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
    let advisory =
        get_hour_selection_advisory(&sample_query(), None, None, None, None).expect("advisory");

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
    assert_eq!(
        canonical.top_recommendation,
        advisory_canonical.top_recommendation
    );
    assert_eq!(canonical.ranked_hours, advisory_canonical.ranked_hours);
}

#[test]
fn hour_selection_canonical_export_reflects_birth_data_tier() {
    let with_birth =
        get_hour_selection_analysis(&sample_query(), Some(1990), Some(1), Some(1), Some("male"))
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
    let analysis =
        get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
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
    let report =
        get_hour_selection_report(&sample_query(), None, None, None, None).expect("report");

    let analysis_canonical = report
        .analysis
        .canonical
        .as_ref()
        .expect("analysis canonical");
    let advisory_canonical = report
        .advisory
        .canonical
        .as_ref()
        .expect("advisory canonical");

    assert_eq!(analysis_canonical.intent, advisory_canonical.intent);
    assert_eq!(
        analysis_canonical.birth_data_tier,
        advisory_canonical.birth_data_tier
    );
    assert_eq!(analysis_canonical.summary_vi, advisory_canonical.summary_vi);
    assert_eq!(analysis_canonical.summary_en, advisory_canonical.summary_en);
    assert_eq!(
        analysis_canonical.top_recommendation,
        advisory_canonical.top_recommendation
    );
    assert_eq!(
        analysis_canonical.ranked_hours,
        advisory_canonical.ranked_hours
    );
    assert_eq!(
        analysis_canonical.auspicious_count,
        advisory_canonical.auspicious_count
    );
    assert_eq!(
        analysis_canonical.total_hours,
        advisory_canonical.total_hours
    );
}

#[test]
fn hour_selection_birth_tier_is_stable_across_analysis_advisory_and_report() {
    let report =
        get_hour_selection_report(&sample_query(), Some(1990), Some(1), Some(1), Some("male"))
            .expect("report");

    let analysis_canonical = report
        .analysis
        .canonical
        .as_ref()
        .expect("analysis canonical");
    let advisory_canonical = report
        .advisory
        .canonical
        .as_ref()
        .expect("advisory canonical");

    assert_eq!(analysis_canonical.birth_data_tier, "date");
    assert_eq!(
        analysis_canonical.birth_data_tier,
        advisory_canonical.birth_data_tier
    );
    assert!(analysis_canonical
        .evidence
        .iter()
        .any(|e| e.source_family == "birth_input"));
    assert!(advisory_canonical
        .evidence
        .iter()
        .any(|e| e.source_family == "birth_input"));
}

// amlich-rv13.5 — day-verdict warning context threading.
//
// The hour-selection surfaces must thread the canonical
// `PersonalDayAssessment` through the hour-ranking wrapper so an `Avoid`
// day carries the structured `warning_context` (Vietnamese clarification
// that the ranking is "best available within an avoided day" and does
// not change the day verdict). The warning must surface identically on
// the analysis, advisory, and report DTOs; absent for non-Avoid days; and
// omitted from JSON when absent.

#[test]
fn hour_selection_analysis_warning_context_is_none_for_default_query() {
    // Without birth, the canonical assessment is still threaded, but for
    // 10/2/2024 the bucket is not Avoid so warning_context stays None.
    let analysis =
        get_hour_selection_analysis(&sample_query(), None, None, None, None).expect("analysis");
    assert!(
        analysis.warning_context.is_none(),
        "default query must NOT carry warning_context; got {:?}",
        analysis.warning_context
    );
    let json = serde_json::to_string(&analysis).expect("serialize");
    assert!(
        !json.contains("\"warning_context\""),
        "absent warning_context must NOT appear in JSON; got {json}"
    );
}

#[test]
fn hour_selection_advisory_warning_context_is_none_for_default_query() {
    let advisory =
        get_hour_selection_advisory(&sample_query(), None, None, None, None).expect("advisory");
    assert!(
        advisory.warning_context.is_none(),
        "default query must NOT carry warning_context; got {:?}",
        advisory.warning_context
    );
    let json = serde_json::to_string(&advisory).expect("serialize");
    assert!(
        !json.contains("\"warning_context\""),
        "absent warning_context must NOT appear in JSON; got {json}"
    );
}

#[test]
fn hour_selection_analysis_warning_context_attaches_for_avoid_day() {
    use amlich_core::almanac::tu_menh::Gender;
    use amlich_core::assessment::PersonalDayAssessment;
    use amlich_core::assessment::{PersonalDayAssessmentBuilder, PersonalDayDecision};
    use amlich_core::reasoning::RecommendationBucket;

    // Build a snapshot, force the day verdict to Avoid, then call the
    // reasoning pipeline with that assessment threaded through. The
    // resulting analysis DTO must carry the structured warning.
    let snapshot = amlich_core::calculate_day_snapshot(10, 2, 2024);
    let profile = amlich_core::BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: None,
        timezone: amlich_core::VIETNAM_TIMEZONE,
        longitude: None,
        use_solar_time: false,
        gender: Some(Gender::Male),
        location_name: None,
    };
    let mut assessment: PersonalDayAssessment = PersonalDayAssessmentBuilder::new(
        snapshot,
        profile,
        amlich_core::ConsultationIntent::Travel,
    )
    .build();
    assessment.decision = PersonalDayDecision {
        bucket: RecommendationBucket::Avoid,
        ..assessment.decision
    };

    let reasoning = amlich_core::build_hour_selection_reasoning(
        10,
        2,
        2024,
        amlich_core::ConsultationIntent::Travel,
        None,
        Some(&assessment),
    )
    .expect("reasoning");

    let warning = reasoning
        .warning_context
        .as_ref()
        .expect("Avoid day must attach warning_context");
    assert_eq!(warning.day_bucket, RecommendationBucket::Avoid);
    assert!(!warning.message_vi.is_empty());

    // The structured warning must flow through every ranked hour's
    // note_vi as the legacy `[Cảnh báo]` prefix so existing consumers
    // reading the string still see the clarification.
    for hour in &reasoning.ranked_hours {
        assert!(
            hour.note_vi.contains("[Cảnh báo]"),
            "Avoid day must surface [Cảnh báo] in note_vi for hour {}; got {:?}",
            hour.chi_name,
            hour.note_vi
        );
    }
}

#[test]
fn hour_selection_report_warning_context_attaches_on_analysis_and_advisory() {
    // The consolidated report path builds the canonical assessment once
    // and threads it into both the analysis and advisory DTOs. We assert
    // that for the default query (non-Avoid day) both DTOs stay None —
    // this pins the threading plumbing even when no warning fires, and
    // is paired with the unit tests in advisory.rs that prove the Avoid
    // path attaches the structured warning.
    let report =
        get_hour_selection_report(&sample_query(), Some(1990), Some(1), Some(1), Some("male"))
            .expect("report");
    assert!(
        report.analysis.warning_context.is_none(),
        "default report's analysis must NOT carry warning_context; got {:?}",
        report.analysis.warning_context
    );
    assert!(
        report.advisory.warning_context.is_none(),
        "default report's advisory must NOT carry warning_context; got {:?}",
        report.advisory.warning_context
    );
    // And the canonical export must not include the warning either.
    let analysis_export = report.analysis.canonical.as_ref().expect("canonical");
    let advisory_export = report.advisory.canonical.as_ref().expect("canonical");
    assert!(analysis_export.warning_context.is_none());
    assert!(advisory_export.warning_context.is_none());
}

#[test]
fn hour_selection_report_warning_context_json_keys_appear_when_avoid_day() {
    // End-to-end: force an Avoid day through the public reasoning
    // pipeline and assert the JSON surfaces the new
    // `warning_context` key on every nested surface.
    use amlich_core::almanac::tu_menh::Gender;
    use amlich_core::assessment::PersonalDayAssessment;
    use amlich_core::assessment::{PersonalDayAssessmentBuilder, PersonalDayDecision};
    use amlich_core::reasoning::RecommendationBucket;

    let snapshot = amlich_core::calculate_day_snapshot(10, 2, 2024);
    let profile = amlich_core::BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: None,
        timezone: amlich_core::VIETNAM_TIMEZONE,
        longitude: None,
        use_solar_time: false,
        gender: Some(Gender::Male),
        location_name: None,
    };
    let mut assessment: PersonalDayAssessment = PersonalDayAssessmentBuilder::new(
        snapshot,
        profile,
        amlich_core::ConsultationIntent::Travel,
    )
    .build();
    assessment.decision = PersonalDayDecision {
        bucket: RecommendationBucket::Avoid,
        ..assessment.decision
    };

    let reasoning = amlich_core::build_hour_selection_reasoning(
        10,
        2,
        2024,
        amlich_core::ConsultationIntent::Travel,
        None,
        Some(&assessment),
    )
    .expect("reasoning");

    let export = reasoning.export(None);
    assert!(export.warning_context.is_some());
    let json = serde_json::to_string(&export).expect("serialize");
    assert!(
        json.contains("\"warning_context\""),
        "Avoid export JSON must contain warning_context; got {json}"
    );
    // The warning text must also appear in the legacy note_vi of every
    // ranked hour (via the embedded [Cảnh báo] prefix).
    assert!(
        export
            .ranked_hours
            .iter()
            .all(|h| h.note_vi.contains("[Cảnh báo]")),
        "every ranked hour in the Avoid export must carry [Cảnh báo]; got {:?}",
        export.ranked_hours
    );
}
