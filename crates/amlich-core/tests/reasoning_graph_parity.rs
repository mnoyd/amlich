use amlich_core::{
    build_initiation_opening_reasoning, calculate_day_snapshot, calculate_day_snapshot_with_timezone,
    reasoning::{PersonalReasoningInput, RecommendationBucket},
    BirthInput, ConsultationIntent, Gender,
};

#[derive(Clone, Copy)]
enum ParityKind {
    Baseline,
    Personal,
    Boundary,
}

#[derive(Clone)]
struct ParityCase {
    kind: ParityKind,
    id: &'static str,
    day: i32,
    month: i32,
    year: i32,
    snapshot_timezone: Option<f64>,
    personal_input: Option<PersonalReasoningInput>,
    expected_bucket: RecommendationBucket,
    expect_conflict_visibility: bool,
    expect_override_visibility: bool,
}

fn profile_input(
    day: i32,
    month: i32,
    year: i32,
    hour: u8,
    minute: u8,
    timezone: f64,
    gender: Option<Gender>,
) -> PersonalReasoningInput {
    PersonalReasoningInput::from_birth(
        BirthInput {
            day,
            month,
            year,
            hour: Some(hour),
            minute: Some(minute),
            timezone,
            gender,
            location_name: None,
        },
        ConsultationIntent::OpeningBusiness,
    )
}

fn decision_snapshot(case: &ParityCase) -> amlich_core::DaySnapshot {
    match case.snapshot_timezone {
        Some(timezone) => calculate_day_snapshot_with_timezone(case.day, case.month, case.year, timezone),
        None => calculate_day_snapshot(case.day, case.month, case.year),
    }
}

fn parity_case(id: &str) -> ParityCase {
    parity_corpus()
        .into_iter()
        .find(|case| case.id == id)
        .unwrap_or_else(|| panic!("missing parity case: {id}"))
}

fn parity_corpus() -> Vec<ParityCase> {
    vec![
        ParityCase {
            kind: ParityKind::Baseline,
            id: "strong_favorable",
            day: 13,
            month: 5,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Favorable,
            expect_conflict_visibility: false,
            expect_override_visibility: false,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "strong_avoid",
            day: 3,
            month: 1,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Avoid,
            expect_conflict_visibility: false,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "conflicting_layered",
            day: 14,
            month: 2,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "sparse_relative",
            day: 22,
            month: 5,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: true,
            expect_override_visibility: false,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "new_year_clear_window",
            day: 1,
            month: 1,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Favorable,
            expect_conflict_visibility: false,
            expect_override_visibility: false,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "mid_january_hard_taboo",
            day: 15,
            month: 1,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Avoid,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "late_january_layered_override",
            day: 29,
            month: 1,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "tet_month_hard_taboo",
            day: 1,
            month: 2,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Avoid,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "march_clear_window",
            day: 1,
            month: 3,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Favorable,
            expect_conflict_visibility: false,
            expect_override_visibility: false,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "april_transition_override",
            day: 1,
            month: 4,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Avoid,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "mid_june_clear_window",
            day: 15,
            month: 6,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Favorable,
            expect_conflict_visibility: false,
            expect_override_visibility: false,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "mid_august_layered_override",
            day: 15,
            month: 8,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Baseline,
            id: "late_september_clear_window",
            day: 30,
            month: 9,
            year: 2024,
            snapshot_timezone: None,
            personal_input: None,
            expected_bucket: RecommendationBucket::Favorable,
            expect_conflict_visibility: false,
            expect_override_visibility: false,
        },
        ParityCase {
            kind: ParityKind::Personal,
            id: "profile_hours_only_favorable_day",
            day: 13,
            month: 5,
            year: 2024,
            snapshot_timezone: None,
            personal_input: Some(profile_input(1, 1, 1990, 9, 0, 7.0, None)),
            expected_bucket: RecommendationBucket::Favorable,
            expect_conflict_visibility: false,
            expect_override_visibility: false,
        },
        ParityCase {
            kind: ParityKind::Personal,
            id: "profile_directions_with_gendered_kua",
            day: 15,
            month: 6,
            year: 2024,
            snapshot_timezone: None,
            personal_input: Some(profile_input(12, 8, 1992, 11, 30, 7.0, Some(Gender::Female))),
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: false,
            expect_override_visibility: false,
        },
        ParityCase {
            kind: ParityKind::Personal,
            id: "tet_adjacent_profile_pressure",
            day: 1,
            month: 2,
            year: 2024,
            snapshot_timezone: None,
            personal_input: Some(profile_input(30, 1, 1989, 23, 30, 7.0, Some(Gender::Male))),
            expected_bucket: RecommendationBucket::Avoid,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Boundary,
            id: "vn_midnight_conflict_window",
            day: 14,
            month: 2,
            year: 2024,
            snapshot_timezone: Some(7.0),
            personal_input: Some(profile_input(30, 1, 1989, 23, 30, 7.0, Some(Gender::Male))),
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Boundary,
            id: "shifted_timezone_same_birth_window",
            day: 14,
            month: 2,
            year: 2024,
            snapshot_timezone: Some(8.0),
            personal_input: Some(profile_input(30, 1, 1989, 23, 30, 8.0, Some(Gender::Male))),
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            kind: ParityKind::Boundary,
            id: "tet_adjacent_female_profile_boundary",
            day: 1,
            month: 2,
            year: 2024,
            snapshot_timezone: Some(7.0),
            personal_input: Some(profile_input(9, 2, 1993, 0, 15, 7.0, Some(Gender::Female))),
            expected_bucket: RecommendationBucket::Avoid,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
    ]
}

#[test]
fn reasoning_graph_parity_corpus_covers_baseline_personal_and_boundary_tracks() {
    let corpus = parity_corpus();

    assert!(corpus.iter().any(|case| matches!(case.kind, ParityKind::Baseline)));
    assert!(corpus.iter().any(|case| matches!(case.kind, ParityKind::Personal)));
    assert!(corpus.iter().any(|case| matches!(case.kind, ParityKind::Boundary)));
}

#[test]
fn baseline_parity_corpus_spans_multiple_bucket_patterns() {
    let baseline = parity_corpus()
        .into_iter()
        .filter(|case| matches!(case.kind, ParityKind::Baseline))
        .collect::<Vec<_>>();

    assert!(baseline.len() >= 10, "need a larger baseline corpus");
    assert!(baseline.iter().any(|case| case.expected_bucket == RecommendationBucket::Favorable));
    assert!(baseline.iter().any(|case| case.expected_bucket == RecommendationBucket::Avoid));
    assert!(baseline.iter().any(|case| case.expected_bucket == RecommendationBucket::Cautious));
}

#[test]
fn personal_parity_cases_keep_profile_effects_explicit() {
    for case in parity_corpus()
        .into_iter()
        .filter(|case| matches!(case.kind, ParityKind::Personal))
    {
        let snapshot = decision_snapshot(&case);
        let without_profile = build_initiation_opening_reasoning(&snapshot, None).expect("baseline");
        let with_profile = build_initiation_opening_reasoning(
            &snapshot,
            case.personal_input.as_ref(),
        )
        .expect("personalized");

        assert_ne!(
            with_profile.suggested_hours,
            without_profile.suggested_hours,
            "{} should expose personal hour refinement",
            case.id
        );
        assert!(
            !with_profile.suggested_directions.is_empty()
                || !with_profile.override_factors.is_empty()
                || !with_profile.conflict_notes.is_empty(),
            "{} should expose profile-dependent output",
            case.id
        );
    }
}

#[test]
fn boundary_parity_cases_cover_timezone_and_local_day_edges() {
    let boundary_cases = parity_corpus()
        .into_iter()
        .filter(|case| matches!(case.kind, ParityKind::Boundary))
        .collect::<Vec<_>>();

    assert!(!boundary_cases.is_empty());

    for case in boundary_cases {
        let snapshot = decision_snapshot(&case);
        let decision = build_initiation_opening_reasoning(&snapshot, case.personal_input.as_ref())
            .expect("decision");

        assert_eq!(
            decision.recommendation_bucket,
            case.expected_bucket,
            "{} bucket mismatch",
            case.id
        );
        assert!(
            case.expect_conflict_visibility == (!decision.context_is_clear || !decision.conflict_notes.is_empty())
                || case.expect_override_visibility == !decision.override_factors.is_empty(),
            "{} should preserve declared boundary visibility",
            case.id
        );
    }
}

#[test]
fn initiation_opening_reasoning_stays_stable_on_representative_dates() {
    for case in parity_corpus()
        .into_iter()
        .filter(|case| matches!(case.kind, ParityKind::Baseline))
    {
        let snapshot = calculate_day_snapshot(case.day, case.month, case.year);
        let decision = build_initiation_opening_reasoning(&snapshot, None).expect("decision");

        assert_eq!(
            decision.recommendation_bucket,
            case.expected_bucket,
            "{} bucket mismatch: expected {:?} vs reasoning {:?}",
            case.id,
            case.expected_bucket,
            decision.recommendation_bucket
        );
    }
}

#[test]
fn representative_explanation_ux_cases_stay_understandable_and_proportionate() {
    let representative_ids = [
        "strong_favorable",
        "conflicting_layered",
        "strong_avoid",
        "profile_directions_with_gendered_kua",
        "vn_midnight_conflict_window",
    ];

    for id in representative_ids {
        let case = parity_case(id);
        let snapshot = decision_snapshot(&case);
        let decision = build_initiation_opening_reasoning(&snapshot, case.personal_input.as_ref())
            .expect("decision");

        assert_eq!(
            decision.recommendation_bucket,
            case.expected_bucket,
            "{} headline bucket mismatch",
            case.id
        );
        assert!(
            !decision.primary_conclusion.is_empty(),
            "{} should keep a non-empty headline",
            case.id
        );
        assert!(
            !decision.strongest_supports.is_empty() || !decision.strongest_resistances.is_empty(),
            "{} should keep at least one rationale item",
            case.id
        );

        if case.expect_conflict_visibility {
            assert!(
                !decision.context_is_clear || !decision.conflict_notes.is_empty(),
                "{} should keep conflict visibility",
                case.id
            );
        }

        if case.expect_override_visibility {
            assert!(
                !decision.override_factors.is_empty(),
                "{} should keep override visibility",
                case.id
            );
        }

        if matches!(case.kind, ParityKind::Personal) {
            let baseline = build_initiation_opening_reasoning(&snapshot, None).expect("baseline");
            assert!(
                decision.suggested_hours != baseline.suggested_hours
                    || decision.suggested_directions != baseline.suggested_directions
                    || !decision.override_factors.is_empty()
                    || !decision.conflict_notes.is_empty(),
                "{} should preserve profile-dependent explanation output",
                case.id
            );
        }
    }
}

#[test]
fn initiation_opening_reasoning_keeps_reasons_and_conflict_signals_visible() {
    for case in parity_corpus()
        .into_iter()
        .filter(|case| matches!(case.kind, ParityKind::Baseline))
    {
        let snapshot = calculate_day_snapshot(case.day, case.month, case.year);
        let decision = build_initiation_opening_reasoning(&snapshot, None).expect("decision");

        assert!(
            !decision.strongest_supports.is_empty() || !decision.strongest_resistances.is_empty(),
            "{} should surface at least one explanation reason",
            case.id
        );

        if case.expect_conflict_visibility {
            assert!(
                !decision.context_is_clear || !decision.conflict_notes.is_empty(),
                "{} should keep conflict visibility",
                case.id
            );
        }

        if case.expect_override_visibility {
            assert!(
                !decision.override_factors.is_empty(),
                "{} should keep override visibility",
                case.id
            );
        }
    }
}
