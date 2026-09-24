//! Day View Model v1 projection contract (`amlich-b14l.7`, S1).
//!
//! Golden fixtures pin the exact anonymous Day View projection over the
//! representative corpus (normal, leap-month, and boundary dates). Regenerate
//! deliberately with `UPDATE_DAY_VIEW_GOLDENS=1 cargo test -p amlich-api --test day_view_projection`
//! when an additive field lands.

use amlich_api::{
    current_chi_index_from_hour, get_day_view_for_date, DayViewCoverageStateDto,
    DayViewPatternItemKindDto, DAY_VIEW_SCHEMA_VERSION,
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

#[test]
fn golden_day_view_projection() {
    let regenerate = std::env::var("UPDATE_DAY_VIEW_GOLDENS").is_ok();
    for (name, day, month, year) in FIXTURE_DATES {
        let view = get_day_view_for_date(day, month, year, None).expect("day view");
        let json = serde_json::to_string_pretty(&view).expect("serialize day view");
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
fn schema_version_is_pinned() {
    let view = get_day_view_for_date(15, 6, 2024, None).expect("day view");
    assert_eq!(view.schema_version, DAY_VIEW_SCHEMA_VERSION);
    assert_eq!(view.schema_version, "day-view-v1");
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
    let view = get_day_view_for_date(15, 6, 2024, Some(5)).expect("day view");
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
    let view = get_day_view_for_date(15, 6, 2024, None).expect("day view");
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
    let view = get_day_view_for_date(15, 6, 2024, None).expect("day view");
    assert_eq!(view.pattern.unknowns.len(), 3);
    assert!(view
        .pattern
        .unknowns
        .iter()
        .all(|item| item.kind == DayViewPatternItemKindDto::Unknown));
}

#[test]
fn coverage_counts_match_entries() {
    let view = get_day_view_for_date(25, 7, 2025, None).expect("day view");
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
    let view = get_day_view_for_date(25, 7, 2025, None).expect("day view");
    assert!(view.lunar.is_leap_month);
}

#[test]
fn every_pattern_item_carries_a_reason() {
    let view = get_day_view_for_date(10, 2, 2024, None).expect("day view");
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
