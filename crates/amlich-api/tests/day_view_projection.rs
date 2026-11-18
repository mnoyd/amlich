//! Day View Model projection contract (`amlich-b14l.7` S1, `amlich-b14l.8` S2).
//!
//! Golden fixtures pin the exact anonymous Day View projection over the
//! representative corpus (normal, leap-month, and boundary dates). S2 extends
//! the fixtures additively: v1 fields are unchanged; v1.1 adds the
//! selected-hour detail (with hour-context reasons) and pins the month-grid
//! navigation projection with leap-month labels. Regenerate deliberately with
//! `UPDATE_DAY_VIEW_GOLDENS=1 cargo test -p amlich-api --test day_view_projection`
//! when an additive field lands.

use amlich_api::{
    current_chi_index_from_hour, get_day_view_for_date, get_day_view_month,
    DayViewCoverageStateDto, DayViewPatternItemKindDto, DAY_VIEW_MONTH_SCHEMA_VERSION,
    DAY_VIEW_SCHEMA_VERSION,
};

const FIXTURE_DATES: [(&str, i32, i32, i32); 4] = [
    ("2024-06-15", 15, 6, 2024),
    ("2025-07-25", 25, 7, 2025),
    ("2024-02-10", 10, 2, 2024),
    ("2024-12-31", 31, 12, 2024),
];

fn fixture_path(name: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/day-view")
        .join(format!("{name}.json"))
}

fn serialize(value: &impl serde::Serialize) -> String {
    serde_json::to_string_pretty(value).expect("serialize day view")
}

#[test]
fn golden_day_view_projection() {
    let regenerate = std::env::var("UPDATE_DAY_VIEW_GOLDENS").is_ok();
    for (name, day, month, year) in FIXTURE_DATES {
        let view = get_day_view_for_date(day, month, year, None, None).expect("day view");
        let json = serialize(&view);
        let path = fixture_path(name);
        if regenerate {
            std::fs::create_dir_all(path.parent().unwrap()).expect("fixture dir");
            std::fs::write(&path, &json).expect("write fixture");
            continue;
        }
        let golden = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("missing golden fixture {}: {error}", path.display()));
        assert_eq!(
            golden.trim(),
            json.trim(),
            "day view projection drifted for {name}; regenerate deliberately if additive"
        );
    }
}

#[test]
fn golden_day_view_projection_with_selected_hour() {
    let regenerate = std::env::var("UPDATE_DAY_VIEW_GOLDENS").is_ok();
    // Leap-month fixture with a selected hour: pins the S2 hour detail.
    let view = get_day_view_for_date(25, 7, 2025, Some(5), Some(9)).expect("day view");
    let json = serialize(&view);
    let path = fixture_path("2025-07-25-hour-selected");
    if regenerate {
        std::fs::create_dir_all(path.parent().unwrap()).expect("fixture dir");
        std::fs::write(&path, &json).expect("write fixture");
        return;
    }
    let golden = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("missing golden fixture {}: {error}", path.display()));
    assert_eq!(
        golden.trim(),
        json.trim(),
        "selected-hour day view drifted; regenerate deliberately if additive"
    );
}

#[test]
fn golden_month_grid_projection() {
    let regenerate = std::env::var("UPDATE_DAY_VIEW_GOLDENS").is_ok();
    // July 2025 contains lunar leap month 6 — the grid must label it.
    let month =
        get_day_view_month(7, 2025, Some((15, 7, 2025)), Some((25, 7, 2025))).expect("month grid");
    let json = serialize(&month);
    let path = fixture_path("2025-07-month");
    if regenerate {
        std::fs::create_dir_all(path.parent().unwrap()).expect("fixture dir");
        std::fs::write(&path, &json).expect("write fixture");
        return;
    }
    let golden = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("missing golden fixture {}: {error}", path.display()));
    assert_eq!(
        golden.trim(),
        json.trim(),
        "month grid projection drifted; regenerate deliberately if additive"
    );
}

#[test]
fn schema_version_is_pinned() {
    let view = get_day_view_for_date(15, 6, 2024, None, None).expect("day view");
    assert_eq!(view.schema_version, DAY_VIEW_SCHEMA_VERSION);
    assert_eq!(view.schema_version, "day-view-v1.2");
}

#[test]
fn current_chi_index_mapping_covers_the_twelve_windows() {
    assert_eq!(current_chi_index_from_hour(23), 0);
    assert_eq!(current_chi_index_from_hour(0), 0);
    assert_eq!(current_chi_index_from_hour(1), 1);
    assert_eq!(current_chi_index_from_hour(13), 7);
    assert_eq!(current_chi_index_from_hour(21), 11);
}

#[test]
fn current_hour_marks_exactly_one_window() {
    let view = get_day_view_for_date(15, 6, 2024, Some(5), None).expect("day view");
    let current: Vec<_> = view
        .hours
        .hours
        .iter()
        .filter(|hour| hour.is_current)
        .collect();
    assert_eq!(current.len(), 1);
    assert_eq!(current[0].hour_index, 5);
    assert_eq!(view.hours.current_hour_index, Some(5));
}

#[test]
fn hour_timeline_is_twelve_windows_with_notability() {
    let view = get_day_view_for_date(15, 6, 2024, None, None).expect("day view");
    assert_eq!(view.hours.hours.len(), 12);
    assert!(view.hours.notable_count > 0);
    assert!(view
        .hours
        .hours
        .iter()
        .all(|hour| hour.is_notable == hour.is_hoang_dao));
}

#[test]
fn anonymous_unknowns_stay_explicit() {
    let view = get_day_view_for_date(15, 6, 2024, None, None).expect("day view");
    assert_eq!(view.pattern.unknowns.len(), 3);
    assert!(view
        .pattern
        .unknowns
        .iter()
        .all(|item| item.kind == DayViewPatternItemKindDto::Unknown));
}

#[test]
fn coverage_counts_match_entries() {
    let view = get_day_view_for_date(25, 7, 2025, None, None).expect("day view");
    let entries = &view.coverage.entries;
    assert_eq!(
        view.coverage.present_count,
        entries
            .iter()
            .filter(|entry| entry.state == DayViewCoverageStateDto::Present)
            .count()
    );
    assert_eq!(
        view.coverage.unknown_count,
        entries
            .iter()
            .filter(|entry| entry.state == DayViewCoverageStateDto::Unknown)
            .count()
    );
    assert_eq!(view.coverage.pending_count, 0);
    assert_eq!(
        view.coverage.present_count + view.coverage.unknown_count,
        entries.len()
    );
}

#[test]
fn leap_month_fixture_carries_the_marker() {
    let view = get_day_view_for_date(25, 7, 2025, None, None).expect("day view");
    assert!(view.lunar.is_leap_month);
}

#[test]
fn every_pattern_item_carries_a_reason() {
    let view = get_day_view_for_date(10, 2, 2024, None, None).expect("day view");
    for item in view
        .pattern
        .supports
        .iter()
        .chain(view.pattern.constraints.iter())
        .chain(view.pattern.unknowns.iter())
    {
        assert!(!item.title.trim().is_empty(), "empty title: {item:?}");
        assert!(!item.reason.trim().is_empty(), "empty reason: {item:?}");
    }
}

// -----------------------------------------------------------------------
// S2 (`amlich-b14l.8`): Navigate & Hours — selected-hour and month-grid
// contracts.
// -----------------------------------------------------------------------

#[test]
fn selected_hour_detail_shows_classification_star_and_reasons() {
    let view = get_day_view_for_date(25, 7, 2025, Some(5), Some(9)).expect("day view");
    let detail = view.hours.detail.as_ref().expect("selected hour detail");
    assert_eq!(view.hours.selected_hour_index, Some(9));
    assert_eq!(detail.hour_index, 9);
    assert_eq!(detail.chi, "Dậu".to_string());
    assert_eq!(detail.time_range, "17:00-19:00");
    assert!(!detail.star.trim().is_empty(), "ruling star must render");
    assert!(
        detail.classification == "Hoàng Đạo" || detail.classification == "Hắc Đạo",
        "classification must be Hoàng Đạo or Hắc Đạo, got {}",
        detail.classification
    );
    assert_eq!(detail.is_hoang_dao, detail.classification == "Hoàng Đạo");
    assert!(!detail.reasons.iter().any(|reason| reason.trim().is_empty()));
}

#[test]
fn hour_detail_carries_no_ranking_language() {
    // Notable Hours are distinguished from any ranking: the detail
    // projection must stay free of score/rank/verdict vocabulary.
    let view = get_day_view_for_date(25, 7, 2025, None, Some(9)).expect("day view");
    let detail = view.hours.detail.as_ref().expect("selected hour detail");
    let value = serde_json::to_value(detail).expect("serialize detail");
    let mut keys = Vec::new();
    fn collect_keys(value: &serde_json::Value, out: &mut Vec<String>) {
        match value {
            serde_json::Value::Object(map) => {
                for (key, child) in map {
                    out.push(key.clone());
                    collect_keys(child, out);
                }
            }
            serde_json::Value::Array(items) => {
                for item in items {
                    collect_keys(item, out);
                }
            }
            _ => {}
        }
    }
    collect_keys(&value, &mut keys);
    for key in &keys {
        assert!(
            !key.contains("score") && !key.contains("rank") && !key.contains("verdict"),
            "hour detail must not carry ranking key `{key}`"
        );
    }
    let text = value.to_string().to_lowercase();
    for phrase in ["best hour", "lucky hour", "phù hợp nhất", "giờ tốt nhất"] {
        assert!(
            !text.contains(phrase),
            "hour detail must not carry ranking phrasing `{phrase}`"
        );
    }
}

#[test]
fn selecting_an_hour_never_moves_the_date_anchor() {
    let plain = get_day_view_for_date(25, 7, 2025, Some(5), None).expect("day view");
    let selected = get_day_view_for_date(25, 7, 2025, Some(5), Some(0)).expect("day view");
    let plain_json = serde_json::to_value(&plain).unwrap();
    let selected_json = serde_json::to_value(&selected).unwrap();
    for field in [
        "solar", "lunar", "canchi", "tiet_khi", "pattern", "coverage",
    ] {
        assert_eq!(
            plain_json[field].to_string(),
            selected_json[field].to_string(),
            "date-anchored section `{field}` must not move on hour selection"
        );
    }
    // The twelve windows themselves are identical; only selection state differs.
    assert_eq!(
        plain_json["hours"]["hours"].to_string(),
        selected_json["hours"]["hours"].to_string()
    );
    // The unselected timeline stays detail-free and unranked.
    assert!(plain.hours.detail.is_none());
    assert!(plain.hours.selected_hour_index.is_none());
    assert!(selected.hours.detail.is_some());
    assert_eq!(selected.hours.selected_hour_index, Some(0));
}

#[test]
fn current_and_selected_hours_are_distinct_marks() {
    let view = get_day_view_for_date(25, 7, 2025, Some(5), Some(9)).expect("day view");
    let current = view
        .hours
        .hours
        .iter()
        .find(|hour| hour.is_current)
        .expect("one current hour");
    assert_eq!(current.hour_index, 5);
    assert_ne!(
        view.hours.current_hour_index,
        view.hours.selected_hour_index
    );
    let detail = view.hours.detail.as_ref().expect("detail");
    assert!(!detail.is_current);
}

#[test]
fn month_grid_labels_leap_months() {
    // 2025-07-25 sits in lunar leap month 6 — the grid must label it.
    let month = get_day_view_month(7, 2025, None, None).expect("month grid");
    assert_eq!(month.schema_version, DAY_VIEW_MONTH_SCHEMA_VERSION);
    let leap_cells: Vec<_> = month
        .cells
        .iter()
        .filter(|cell| cell.is_leap_lunar_month)
        .collect();
    assert!(!leap_cells.is_empty(), "leap cells must be present");
    assert!(leap_cells
        .iter()
        .all(|cell| cell.lunar_label.contains("(nhuận)")));
    let leap = month
        .cells
        .iter()
        .find(|cell| cell.day == 25)
        .expect("day 25 cell");
    assert!(leap.is_leap_lunar_month);
    assert_eq!(leap.lunar_month, 6);
    assert!(leap.lunar_label.contains("(nhuận)"));
}

#[test]
fn month_grid_marks_today_and_selected_without_reordering() {
    let month =
        get_day_view_month(7, 2025, Some((15, 7, 2025)), Some((25, 7, 2025))).expect("month grid");
    assert_eq!(month.cells.len(), 31);
    assert_eq!(month.year, 2025);
    assert_eq!(month.month, 7);
    // 2025-07-01 is a Tuesday (Sunday = 0).
    assert_eq!(month.first_weekday, 2);
    for (index, cell) in month.cells.iter().enumerate() {
        assert_eq!(cell.day, index as i32 + 1, "cells stay in day order");
    }
    assert!(month
        .cells
        .iter()
        .any(|cell| cell.is_today && cell.day == 15));
    assert!(month
        .cells
        .iter()
        .any(|cell| cell.is_selected && cell.day == 25));
    assert!(month.cells.iter().all(|cell| !cell.can_chi_day.is_empty()));
    assert!(month
        .cells
        .iter()
        .all(|cell| cell.lunar_day >= 1 && cell.lunar_day <= 30));
}

#[test]
fn month_grid_rejects_invalid_months() {
    assert!(get_day_view_month(0, 2025, None, None).is_err());
    assert!(get_day_view_month(13, 2025, None, None).is_err());
}

// -----------------------------------------------------------------------
// S3 (`amlich-b14l.9`): Assessment on demand — Personal Context,
// enrichment layers, Unknown rendering, non-verdict baseline.
// -----------------------------------------------------------------------

use amlich_api::{get_day_view_enriched_for_date, DayViewPersonalContextInput, DAY_VIEW_INTENTS};

/// The sample profile pinned by the slice spec: `1990-01-01 09:30 Nam`.
fn sample_profile_context(intent: &str) -> DayViewPersonalContextInput {
    DayViewPersonalContextInput {
        intent: Some(intent.to_string()),
        birth_year: Some(1990),
        birth_month: Some(1),
        birth_day: Some(1),
        birth_hour: Some(9),
        birth_minute: Some(30),
        gender: Some("nam".to_string()),
        location_name: None,
    }
}

#[test]
fn golden_day_view_projection_intent_only() {
    let regenerate = std::env::var("UPDATE_DAY_VIEW_GOLDENS").is_ok();
    let context = DayViewPersonalContextInput {
        intent: Some("travel".to_string()),
        ..Default::default()
    };
    let view = get_day_view_enriched_for_date(25, 7, 2025, Some(5), None, &context)
        .expect("enriched day view");
    let json = serialize(&view);
    let path = fixture_path("2025-07-25-intent-travel");
    if regenerate {
        std::fs::create_dir_all(path.parent().unwrap()).expect("fixture dir");
        std::fs::write(&path, &json).expect("write fixture");
        return;
    }
    let golden = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("missing golden fixture {}: {error}", path.display()));
    assert_eq!(
        golden.trim(),
        json.trim(),
        "intent-only day view drifted; regenerate deliberately if additive"
    );
}

#[test]
fn golden_day_view_projection_sample_profile() {
    let regenerate = std::env::var("UPDATE_DAY_VIEW_GOLDENS").is_ok();
    let context = sample_profile_context("opening_business");
    let view = get_day_view_enriched_for_date(25, 7, 2025, Some(5), None, &context)
        .expect("enriched day view");
    let json = serialize(&view);
    let path = fixture_path("2025-07-25-profile-1990");
    if regenerate {
        std::fs::create_dir_all(path.parent().unwrap()).expect("fixture dir");
        std::fs::write(&path, &json).expect("write fixture");
        return;
    }
    let golden = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("missing golden fixture {}: {error}", path.display()));
    assert_eq!(
        golden.trim(),
        json.trim(),
        "sample-profile day view drifted; regenerate deliberately if additive"
    );
}

#[test]
fn intent_is_required_before_an_assessment_exists() {
    // No Intent → the view is the anonymous projection, assessment absent,
    // whatever the other layers carry (Progressive Enrichment order).
    let anonymous = get_day_view_for_date(25, 7, 2025, None, None).expect("day view");
    let no_intent = DayViewPersonalContextInput {
        birth_year: Some(1990),
        birth_month: Some(1),
        birth_day: Some(1),
        gender: Some("nam".to_string()),
        ..Default::default()
    };
    let view = get_day_view_enriched_for_date(25, 7, 2025, None, None, &no_intent)
        .expect("enriched day view");
    assert!(view.assessment.is_none());
    // The missing Intent stays the first unknown even when the birth
    // profile itself was supplied (Progressive Enrichment order).
    assert_eq!(view.pattern.unknowns.len(), 2);
    assert_eq!(view.pattern.unknowns[0].title, "Chưa có mục đích");
    assert_eq!(view.pattern.unknowns[1].title, "Chưa có vị trí");
    // Byte-parity with the anonymous path except the coverage chips that
    // report the supplied (but not yet activating) layers.
    let anonymous_json = serde_json::to_value(&anonymous).unwrap();
    let view_json = serde_json::to_value(&view).unwrap();
    for field in ["solar", "lunar", "canchi", "tiet_khi", "hours"] {
        assert_eq!(
            anonymous_json[field].to_string(),
            view_json[field].to_string(),
            "field `{field}` must not move without an Intent"
        );
    }
    // Calendar facts inside the pattern do not move; only the unknowns
    // reflect the supplied-but-inactive layers.
    assert_eq!(
        anonymous_json["pattern"]["supports"].to_string(),
        view_json["pattern"]["supports"].to_string()
    );
    assert_eq!(
        anonymous_json["pattern"]["constraints"].to_string(),
        view_json["pattern"]["constraints"].to_string()
    );
}

#[test]
fn enrichment_layers_are_explicit_and_ordered() {
    let context = sample_profile_context("travel");
    let view = get_day_view_enriched_for_date(25, 7, 2025, None, None, &context)
        .expect("enriched day view");
    let assessment = view.assessment.as_ref().expect("assessment");
    let layers: Vec<&str> = assessment.layers.iter().map(|l| l.layer.as_str()).collect();
    assert_eq!(layers, ["intent", "birth_profile", "location"]);
    let applied: Vec<bool> = assessment.layers.iter().map(|l| l.applied).collect();
    assert_eq!(applied, [true, true, false]);
    for layer in &assessment.layers {
        assert!(!layer.summary.trim().is_empty(), "empty summary: {layer:?}");
        assert!(
            !layer.changes.is_empty(),
            "layer {} must make its changes explicit",
            layer.layer
        );
        assert!(
            layer.changes.iter().all(|c| !c.trim().is_empty()),
            "empty change on {}",
            layer.layer
        );
    }
    // The intent layer states that the anonymous overview is untouched.
    let intent_layer = &assessment.layers[0];
    assert!(intent_layer
        .changes
        .iter()
        .any(|c| c.contains("tổng quan ẩn danh")));
    // The birth layer names the sample profile with time and gender.
    let birth_layer = &assessment.layers[1];
    assert!(birth_layer.summary.contains("01/01/1990"));
    assert!(birth_layer.summary.contains("09:30"));
    assert!(birth_layer.summary.contains("Nam"));
}

#[test]
fn missing_inputs_surface_as_unknowns_layer_by_layer() {
    // Anonymous: three unknowns (intent, birth profile, location).
    let anonymous = get_day_view_for_date(15, 6, 2024, None, None).expect("day view");
    assert_eq!(anonymous.pattern.unknowns.len(), 3);
    // Intent only: birth profile and location remain unknown.
    let intent_only = DayViewPersonalContextInput {
        intent: Some("prayer".to_string()),
        ..Default::default()
    };
    let view = get_day_view_enriched_for_date(15, 6, 2024, None, None, &intent_only)
        .expect("enriched day view");
    assert_eq!(view.pattern.unknowns.len(), 2);
    assert!(view
        .pattern
        .unknowns
        .iter()
        .all(|item| item.kind == DayViewPatternItemKindDto::Unknown));
    let assessment = view.assessment.as_ref().expect("assessment");
    assert_eq!(assessment.unknowns.len(), 2);
    // Full sample: only location remains unknown.
    let view =
        get_day_view_enriched_for_date(15, 6, 2024, None, None, &sample_profile_context("prayer"))
            .expect("enriched day view");
    assert_eq!(view.pattern.unknowns.len(), 1);
    assert_eq!(view.pattern.unknowns[0].title, "Chưa có vị trí");
    // Adding the location layer resolves the last unknown.
    let mut full = sample_profile_context("prayer");
    full.location_name = Some("Hà Nội".to_string());
    let view =
        get_day_view_enriched_for_date(15, 6, 2024, None, None, &full).expect("enriched day view");
    assert!(view.pattern.unknowns.is_empty());
    assert!(view
        .assessment
        .as_ref()
        .expect("assessment")
        .layers
        .iter()
        .all(|layer| layer.applied));
}

#[test]
fn assessment_renders_result_reasons_and_confidence() {
    let context = sample_profile_context("wedding");
    let view = get_day_view_enriched_for_date(10, 2, 2024, None, None, &context)
        .expect("enriched day view");
    let assessment = view.assessment.as_ref().expect("assessment");
    // Result.
    assert!(
        ["favorable", "mixed", "cautious", "avoid"].contains(&assessment.result.bucket.as_str())
    );
    assert!(!assessment.result.bucket_label.trim().is_empty());
    assert!(!assessment.result.primary_conclusion.trim().is_empty());
    // Reasons: every canonical contribution is projected with provenance.
    assert!(!assessment.reasons.is_empty());
    for reason in &assessment.reasons {
        assert!(!reason.text.trim().is_empty());
        assert!(!reason.axis.trim().is_empty());
        assert!(!reason.axis_label.trim().is_empty());
        assert!(["favorable", "avoid", "neutral", "info"].contains(&reason.polarity.as_str()));
        assert!(!reason.source_family.trim().is_empty());
        assert!(!reason.source_id.trim().is_empty());
        assert!(!reason.method.trim().is_empty());
        assert!(
            reason.text.contains("nguồn"),
            "reason must cite its source: {}",
            reason.text
        );
    }
    // Confidence: level + completeness reasons, never predictive language.
    assert!(["low", "medium", "high"].contains(&assessment.confidence.level.as_str()));
    assert!(!assessment.confidence.reasons.is_empty());
    let confidence_text = serde_json::to_string(&assessment.confidence)
        .expect("serialize confidence")
        .to_lowercase();
    for phrase in ["xác suất", "probability", "chính xác ", "accuracy"] {
        assert!(
            !confidence_text.contains(phrase),
            "confidence must not carry predictive phrasing `{phrase}`"
        );
    }
    // Provenance rides along for S4 to follow.
    assert!(!assessment.policy_id.trim().is_empty());
    assert!(!assessment.ruleset_id.trim().is_empty());
}

#[test]
fn anonymous_baseline_stays_non_verdict() {
    let anonymous = get_day_view_for_date(25, 7, 2025, None, None).expect("day view");
    let enriched =
        get_day_view_enriched_for_date(25, 7, 2025, None, None, &sample_profile_context("travel"))
            .expect("enriched day view");
    // Anonymous: no assessment section at all.
    assert!(anonymous.assessment.is_none());
    // Calendar facts do not move when layers are added.
    let anonymous_json = serde_json::to_value(&anonymous).unwrap();
    let enriched_json = serde_json::to_value(&enriched).unwrap();
    for field in ["solar", "lunar", "canchi", "tiet_khi"] {
        assert_eq!(
            anonymous_json[field].to_string(),
            enriched_json[field].to_string(),
            "calendar identity `{field}` must not move on enrichment"
        );
    }
    assert_eq!(
        anonymous_json["pattern"]["supports"].to_string(),
        enriched_json["pattern"]["supports"].to_string(),
        "supports must not move on enrichment"
    );
    assert_eq!(
        anonymous_json["pattern"]["constraints"].to_string(),
        enriched_json["pattern"]["constraints"].to_string(),
        "constraints must not move on enrichment"
    );
    assert_eq!(
        anonymous_json["hours"]["hours"].to_string(),
        enriched_json["hours"]["hours"].to_string(),
        "hour windows must not move on enrichment"
    );
}

#[test]
fn coverage_flips_with_the_applied_layers() {
    let intent_only = DayViewPersonalContextInput {
        intent: Some("travel".to_string()),
        ..Default::default()
    };
    let view = get_day_view_enriched_for_date(25, 7, 2025, None, None, &intent_only)
        .expect("enriched day view");
    let entry = |family: &str| {
        view.coverage
            .entries
            .iter()
            .find(|entry| serde_json::to_value(entry.family).unwrap().as_str() == Some(family))
            .expect(family)
    };
    assert_eq!(entry("intent").state, DayViewCoverageStateDto::Present);
    assert_eq!(
        entry("birth_profile").state,
        DayViewCoverageStateDto::Unknown
    );
    assert_eq!(entry("location").state, DayViewCoverageStateDto::Unknown);
    let mut full = sample_profile_context("travel");
    full.location_name = Some("Hà Nội".to_string());
    let view =
        get_day_view_enriched_for_date(25, 7, 2025, None, None, &full).expect("enriched day view");
    assert!(view.coverage.entries.iter().all(|entry| {
        !matches!(
            (entry.family, entry.state),
            (
                amlich_api::DayViewCoverageFamilyDto::Intent
                    | amlich_api::DayViewCoverageFamilyDto::BirthProfile
                    | amlich_api::DayViewCoverageFamilyDto::Location,
                DayViewCoverageStateDto::Unknown
            )
        )
    }));
}

#[test]
fn unknown_context_inputs_are_rejected_loudly() {
    let mut context = DayViewPersonalContextInput {
        intent: Some("skydiving".to_string()),
        ..Default::default()
    };
    let error = get_day_view_enriched_for_date(25, 7, 2025, None, None, &context)
        .expect_err("unknown intent must fail");
    assert!(error.contains("unknown intent"));

    context.intent = Some("travel".to_string());
    context.gender = Some("robot".to_string());
    let error = get_day_view_enriched_for_date(25, 7, 2025, None, None, &context)
        .expect_err("unknown gender must fail");
    assert!(error.contains("unknown gender"));

    context.gender = None;
    context.birth_year = Some(1990);
    let error = get_day_view_enriched_for_date(25, 7, 2025, None, None, &context)
        .expect_err("partial birth profile must fail");
    assert!(error.contains("together"));

    context.birth_month = Some(1);
    context.birth_day = Some(1);
    context.birth_hour = Some(9);
    let error = get_day_view_enriched_for_date(25, 7, 2025, None, None, &context)
        .expect_err("birth hour without minute must fail");
    assert!(error.contains("together"));
}

#[test]
fn intent_keys_cover_the_nine_consultation_intents() {
    // The shared intent list is the cross-surface picker contract.
    let keys: Vec<&str> = DAY_VIEW_INTENTS.iter().map(|(key, _)| *key).collect();
    for key in [
        "wedding",
        "moving_house",
        "opening_business",
        "contract_signing",
        "travel",
        "burial",
        "renovation",
        "medical_checkup",
        "prayer",
    ] {
        assert!(
            keys.contains(&key),
            "intent `{key}` missing from DAY_VIEW_INTENTS"
        );
    }
    for (_, label) in DAY_VIEW_INTENTS {
        assert!(!label.trim().is_empty());
    }
}
