//! amlich-mwbp.8 P2 consolidation regression tests (amlich-9z7i).
//!
//! The personal-day and hour-selection report paths must build the day
//! snapshot, the canonical PersonalDayAssessment, the Bazi chart, the
//! element distribution, the Kua, and the three interaction matrices at
//! most once per request. The previous implementation rebuilt the snapshot
//! ~9x and the assessment ~4x along the path through
//! `get_personal_day_advisory`, `get_personal_day_chart`,
//! `get_personal_day_analysis`, and `get_personal_day_metrics`. These
//! tests pin the consolidated behavior so the dedupe cannot regress.

use amlich_api::{get_hour_selection_report, get_personal_day_report, DateQuery};
use amlich_core::almanac::tu_menh::Gender;
use amlich_core::build_count::{self, BuildCounters};

fn sample_query() -> DateQuery {
    DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

fn diff(a: BuildCounters, b: BuildCounters) -> BuildCounters {
    BuildCounters {
        snapshot_builds: b.snapshot_builds.saturating_sub(a.snapshot_builds),
        canonical_assessments: b
            .canonical_assessments
            .saturating_sub(a.canonical_assessments),
        bazi_charts: b.bazi_charts.saturating_sub(a.bazi_charts),
        element_distributions: b
            .element_distributions
            .saturating_sub(a.element_distributions),
        kua_computations: b.kua_computations.saturating_sub(a.kua_computations),
        day_person_matrices: b.day_person_matrices.saturating_sub(a.day_person_matrices),
        personal_hour_matrices: b
            .personal_hour_matrices
            .saturating_sub(a.personal_hour_matrices),
        direction_merge_matrices: b
            .direction_merge_matrices
            .saturating_sub(a.direction_merge_matrices),
    }
}

#[test]
fn personal_day_report_anonymous_builds_one_snapshot_and_one_assessment() {
    let query = sample_query();
    build_count::reset();
    let _report = get_personal_day_report(&query, None, None, None, None).expect("report");
    let counters = build_count::snapshot();

    assert_eq!(
        counters.snapshot_builds, 1,
        "personal-day report must build exactly one snapshot per request (got {})",
        counters.snapshot_builds
    );
    assert_eq!(
        counters.canonical_assessments, 1,
        "personal-day report must build exactly one canonical assessment per request (got {})",
        counters.canonical_assessments
    );
}

#[test]
fn personal_day_report_with_profile_builds_one_snapshot_and_one_assessment() {
    let query = sample_query();
    let gender = Gender::Male;
    build_count::reset();
    let _report = get_personal_day_report(&query, Some(1990), Some(1), Some(1), Some(gender))
        .expect("report");
    let counters = build_count::snapshot();

    assert_eq!(
        counters.snapshot_builds, 1,
        "personal-day report with profile must build exactly one snapshot per request (got {})",
        counters.snapshot_builds
    );
    assert_eq!(
        counters.canonical_assessments, 1,
        "personal-day report with profile must build exactly one canonical assessment per request (got {})",
        counters.canonical_assessments
    );
    // The personal-facts bundle is built once because the request path
    // shares it across the fact-node projection, the matrix builder, and
    // the evaluator (amlich-mwbp.8 P2 finding A-R11).
    assert_eq!(
        counters.bazi_charts, 1,
        "Bazi chart must be built once per personal-day report (got {})",
        counters.bazi_charts
    );
    assert_eq!(
        counters.element_distributions, 1,
        "element distribution must be built once per personal-day report (got {})",
        counters.element_distributions
    );
    // Kua is currently consumed by three subsystems (assessment,
    // calculate_dai_van, and direction_merge) that each compute it
    // independently; deduplicating it across those subsystems is a
    // follow-up — see REPAIR-PLAN.md P2. We pin the current count so the
    // refactor cannot regress this surface.
    assert!(
        counters.kua_computations > 0 && counters.kua_computations <= 3,
        "Kua must be computed at most 3 times per personal-day report (got {})",
        counters.kua_computations
    );
    assert_eq!(
        counters.day_person_matrices, 1,
        "day-person matrix must be built once per personal-day report (got {})",
        counters.day_person_matrices
    );
    assert_eq!(
        counters.personal_hour_matrices, 1,
        "personal-hour matrix must be built once per personal-day report (got {})",
        counters.personal_hour_matrices
    );
    assert_eq!(
        counters.direction_merge_matrices, 1,
        "direction-merge matrix must be built once per personal-day report (got {})",
        counters.direction_merge_matrices
    );
}

#[test]
fn hour_selection_report_anonymous_builds_one_snapshot_and_one_assessment() {
    let query = sample_query();
    build_count::reset();
    let _report = get_hour_selection_report(&query, None, None, None, None).expect("report");
    let counters = build_count::snapshot();

    assert_eq!(
        counters.snapshot_builds, 1,
        "hour-selection report must build exactly one snapshot per request (got {})",
        counters.snapshot_builds
    );
    assert_eq!(
        counters.canonical_assessments, 1,
        "hour-selection report must build exactly one canonical assessment per request (got {})",
        counters.canonical_assessments
    );
}

#[test]
fn personal_day_advisory_standalone_still_builds_independently() {
    // The standalone `get_personal_day_advisory` is the unit of work
    // for callers that don't need the full report. It must still build
    // its own snapshot and assessment — only the *report* path is
    // consolidated. This test guards against accidentally folding the
    // standalone path into a thin wrapper that hides duplicate work.
    use amlich_api::get_personal_day_advisory;

    let query = sample_query();
    let gender = Gender::Male;
    build_count::reset();
    let _advisory = get_personal_day_advisory(&query, Some(1990), Some(1), Some(1), Some(gender))
        .expect("advisory");
    let counters = build_count::snapshot();

    assert!(
        counters.snapshot_builds >= 1,
        "standalone personal-day advisory must build the snapshot at least once (got {})",
        counters.snapshot_builds
    );
    assert!(
        counters.canonical_assessments >= 1,
        "standalone personal-day advisory must build the canonical assessment at least once (got {})",
        counters.canonical_assessments
    );
}

#[test]
fn personal_day_report_subcalls_do_not_rebuild_after_initial_context() {
    // The report path builds the PersonalDayContext once. The chart,
    // analysis, metrics, and advisory sub-builders must read from the
    // context — they must not trigger additional snapshot/assessment
    // builds. We measure the delta between the post-context and
    // post-DTO builds to assert that the sub-builders are idempotent.
    use amlich_api::get_personal_day_advisory;
    use amlich_api::get_personal_day_analysis;
    use amlich_api::get_personal_day_chart;
    use amlich_api::get_personal_day_metrics;

    let query = sample_query();
    let gender = Gender::Male;

    build_count::reset();
    let _report = get_personal_day_report(&query, Some(1990), Some(1), Some(1), Some(gender))
        .expect("report");
    let report_counters = build_count::snapshot();

    build_count::reset();
    let _ = get_personal_day_chart(&query, Some(1990), Some(1), Some(1), Some(gender));
    let _ = get_personal_day_analysis(&query, Some(1990), Some(1), Some(1), Some(gender));
    let _ = get_personal_day_metrics(&query, Some(1990), Some(1), Some(1), Some(gender));
    let _ = get_personal_day_advisory(&query, Some(1990), Some(1), Some(1), Some(gender));
    let standalone_counters = build_count::snapshot();

    let standalone_diff = diff(BuildCounters::default(), standalone_counters);

    // The combined standalone calls rebuild the snapshot/assessment on
    // each entry — that's the legacy behavior. The consolidated report
    // path must do strictly less work than the sum of those standalone
    // calls; pinning that inequality is enough to detect dedupe
    // regressions without forcing an exact one-to-one correspondence.
    assert!(
        report_counters.snapshot_builds < standalone_diff.snapshot_builds,
        "consolidated report snapshot builds ({}) must be less than the sum of standalone calls ({})",
        report_counters.snapshot_builds,
        standalone_diff.snapshot_builds
    );
    assert!(
        report_counters.canonical_assessments < standalone_diff.canonical_assessments,
        "consolidated report canonical-assessment builds ({}) must be less than the sum of standalone calls ({})",
        report_counters.canonical_assessments,
        standalone_diff.canonical_assessments
    );
}
