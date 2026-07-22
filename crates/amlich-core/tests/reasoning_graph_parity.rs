use amlich_core::{
    build_initiation_opening_reasoning, calculate_day_snapshot,
    calculate_day_snapshot_with_timezone,
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
        Some(timezone) => {
            calculate_day_snapshot_with_timezone(case.day, case.month, case.year, timezone)
        }
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
            personal_input: Some(profile_input(
                12,
                8,
                1992,
                11,
                30,
                7.0,
                Some(Gender::Female),
            )),
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

    assert!(corpus
        .iter()
        .any(|case| matches!(case.kind, ParityKind::Baseline)));
    assert!(corpus
        .iter()
        .any(|case| matches!(case.kind, ParityKind::Personal)));
    assert!(corpus
        .iter()
        .any(|case| matches!(case.kind, ParityKind::Boundary)));
}

#[test]
fn baseline_parity_corpus_spans_multiple_bucket_patterns() {
    let baseline = parity_corpus()
        .into_iter()
        .filter(|case| matches!(case.kind, ParityKind::Baseline))
        .collect::<Vec<_>>();

    assert!(baseline.len() >= 10, "need a larger baseline corpus");
    assert!(baseline
        .iter()
        .any(|case| case.expected_bucket == RecommendationBucket::Favorable));
    assert!(baseline
        .iter()
        .any(|case| case.expected_bucket == RecommendationBucket::Avoid));
    assert!(baseline
        .iter()
        .any(|case| case.expected_bucket == RecommendationBucket::Cautious));
}

#[test]
fn personal_parity_cases_keep_profile_effects_explicit() {
    for case in parity_corpus()
        .into_iter()
        .filter(|case| matches!(case.kind, ParityKind::Personal))
    {
        let snapshot = decision_snapshot(&case);
        let without_profile =
            build_initiation_opening_reasoning(&snapshot, None).expect("baseline");
        let with_profile =
            build_initiation_opening_reasoning(&snapshot, case.personal_input.as_ref())
                .expect("personalized");

        assert_ne!(
            with_profile.suggested_hours, without_profile.suggested_hours,
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
            decision.recommendation_bucket, case.expected_bucket,
            "{} bucket mismatch",
            case.id
        );
        assert!(
            case.expect_conflict_visibility
                == (!decision.context_is_clear || !decision.conflict_notes.is_empty())
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
            decision.recommendation_bucket, case.expected_bucket,
            "{} bucket mismatch: expected {:?} vs reasoning {:?}",
            case.id, case.expected_bucket, decision.recommendation_bucket
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
            decision.recommendation_bucket, case.expected_bucket,
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

#[test]
fn graph_backed_evaluator_bucket_parity_with_current_pipeline() {
    use amlich_core::reasoning::project_initiation_opening_decision;
    use amlich_core::reasoning::{ActionEvaluator, InitiationOpeningEvaluator};

    for case in parity_corpus()
        .into_iter()
        .filter(|case| matches!(case.kind, ParityKind::Baseline))
    {
        let snapshot = decision_snapshot(&case);
        let graph = amlich_core::build_reasoning_input_graph(&snapshot, None).expect("valid graph");
        let evaluator = InitiationOpeningEvaluator::new();

        let evaluation = evaluator
            .evaluate(&graph, &snapshot, None)
            .expect("valid evaluation");
        let graph_decision = project_initiation_opening_decision(&evaluation);
        let current_decision =
            build_initiation_opening_reasoning(&snapshot, None).expect("current decision");

        assert_eq!(
            graph_decision.recommendation_bucket, case.expected_bucket,
            "{} graph bucket mismatch: expected {:?} vs got {:?}",
            case.id, case.expected_bucket, graph_decision.recommendation_bucket
        );

        assert_eq!(
            graph_decision.recommendation_bucket, current_decision.recommendation_bucket,
            "{} bucket should match current pipeline",
            case.id
        );
    }
}

#[test]
fn production_reasoning_entrypoint_matches_graph_evaluator_projection() {
    use amlich_core::reasoning::{
        project_initiation_opening_decision, ActionEvaluator, InitiationOpeningEvaluator,
    };

    let case = parity_case("profile_directions_with_gendered_kua");
    let snapshot = decision_snapshot(&case);
    let personal_input = case.personal_input.as_ref().expect("personal input");
    let graph = amlich_core::build_reasoning_input_graph(
        &snapshot,
        Some(&amlich_core::bazi::BaziInput {
            day: personal_input.birth.day,
            month: personal_input.birth.month,
            year: personal_input.birth.year,
            hour: personal_input.birth.hour.unwrap_or(0),
            minute: personal_input.birth.minute.unwrap_or(0),
            time_known: personal_input.birth.hour.is_some()
                && personal_input.birth.minute.is_some(),
            timezone: personal_input.birth.timezone,
            longitude: None,
            use_solar_time: false,
            gender: personal_input.birth.gender,
        }),
    )
    .expect("valid graph");
    let evaluator = InitiationOpeningEvaluator::new();

    let evaluation = evaluator
        .evaluate(&graph, &snapshot, Some(personal_input))
        .expect("valid evaluation");
    let projected = project_initiation_opening_decision(&evaluation);
    let production =
        build_initiation_opening_reasoning(&snapshot, Some(personal_input)).expect("decision");

    assert_eq!(production, projected);
}

#[test]
fn production_reasoning_bundle_export_overlays_canonical_assessment_axes() {
    use amlich_core::reasoning::{
        project_initiation_opening_decision_export, ActionEvaluator, InitiationOpeningEvaluator,
    };

    let case = parity_case("vn_midnight_conflict_window");
    let snapshot = decision_snapshot(&case);
    let personal_input = case.personal_input.as_ref().expect("personal input");
    let graph = amlich_core::build_reasoning_input_graph(
        &snapshot,
        Some(&amlich_core::bazi::BaziInput {
            day: personal_input.birth.day,
            month: personal_input.birth.month,
            year: personal_input.birth.year,
            hour: personal_input.birth.hour.unwrap_or(0),
            minute: personal_input.birth.minute.unwrap_or(0),
            time_known: personal_input.birth.hour.is_some()
                && personal_input.birth.minute.is_some(),
            timezone: personal_input.birth.timezone,
            longitude: None,
            use_solar_time: false,
            gender: personal_input.birth.gender,
        }),
    )
    .expect("valid graph");
    let evaluator = InitiationOpeningEvaluator::new();

    let evaluation = evaluator
        .evaluate(&graph, &snapshot, Some(personal_input))
        .expect("valid evaluation");
    let legacy_projection = project_initiation_opening_decision_export(&evaluation);
    let production =
        amlich_core::build_initiation_opening_reasoning_bundle(&snapshot, Some(personal_input))
            .expect("bundle");

    // amlich-mwbp.6: the production path overlays the canonical
    // PersonalDayAssessment's axis scores, so its decision_export is NOT
    // expected to be byte-identical to the graph-only projection. The
    // contract is now: typed prose/shapes match the graph, while axis
    // scores are sourced from the assessment. We verify the overlay by
    // checking that the production axis scores come from the assessment's
    // normalized 0..=1 range rather than raw note counts.
    assert_eq!(
        production.decision_export.primary_conclusion, legacy_projection.primary_conclusion,
        "primary_conclusion must come from the graph prose, not the assessment"
    );
    assert_eq!(
        production.decision_export.recommendation_bucket, legacy_projection.recommendation_bucket,
        "recommendation_bucket must come from the graph evaluator"
    );
    assert_eq!(
        production.decision_export.semantic, legacy_projection.semantic,
        "semantic category must come from the graph evaluator"
    );

    for production_axis in &production.decision_export.axis_scores {
        // Canonical assessment axis scores live in 0..=1 (a normalized
        // 0..=100 score with a uniform scale). Legacy graph projections
        // emitted raw note counts (e.g. support=1.0 because a single star
        // note matched). After .6 the production axes must come from the
        // assessment, so the normalized scores never exceed 1.0.
        assert!(
            production_axis.score >= 0.0 && production_axis.score <= 1.0,
            "production axis score must come from the canonical assessment (0..=1); got {:?}",
            production_axis.score
        );
    }
}

#[test]
fn graph_backed_evaluator_confidence_parity() {
    use amlich_core::reasoning::project_initiation_opening_decision;
    use amlich_core::reasoning::{ActionEvaluator, InitiationOpeningEvaluator};

    let snapshot = calculate_day_snapshot(3, 1, 2024);
    let graph = amlich_core::build_reasoning_input_graph(&snapshot, None).expect("valid graph");
    let evaluator = InitiationOpeningEvaluator::new();

    let evaluation = evaluator
        .evaluate(&graph, &snapshot, None)
        .expect("valid evaluation");
    let graph_decision = project_initiation_opening_decision(&evaluation);
    let current_decision =
        build_initiation_opening_reasoning(&snapshot, None).expect("current decision");

    assert_eq!(
        graph_decision.confidence, current_decision.confidence,
        "3/1/2024 confidence should match current pipeline"
    );
}

#[test]
fn graph_backed_evaluator_produces_valid_action_evaluation() {
    use amlich_core::reasoning::project_initiation_opening_decision;
    use amlich_core::reasoning::{ActionEvaluator, InitiationOpeningEvaluator};

    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let graph = amlich_core::build_reasoning_input_graph(&snapshot, None).expect("valid graph");
    let evaluator = InitiationOpeningEvaluator::new();

    let evaluation = evaluator
        .evaluate(&graph, &snapshot, None)
        .expect("valid evaluation");

    assert!(
        !evaluation.primary_conclusion.is_empty(),
        "should have conclusion"
    );
    assert!(
        !evaluation.strongest_supports.is_empty() || !evaluation.strongest_resistances.is_empty(),
        "should have at least one support or resistance note"
    );
    assert!(
        matches!(
            evaluation.semantic,
            amlich_core::reasoning::ReasoningConclusionSemantic::FavorableClear
                | amlich_core::reasoning::ReasoningConclusionSemantic::FavorableContextual
                | amlich_core::reasoning::ReasoningConclusionSemantic::ConflictedCautious
                | amlich_core::reasoning::ReasoningConclusionSemantic::ResistanceLedCautious
                | amlich_core::reasoning::ReasoningConclusionSemantic::OverrideCautious
                | amlich_core::reasoning::ReasoningConclusionSemantic::OverrideAvoid
        ),
        "should have valid semantic"
    );

    let decision = project_initiation_opening_decision(&evaluation);
    assert!(
        matches!(
            decision.recommendation_bucket,
            amlich_core::reasoning::RecommendationBucket::Favorable
                | amlich_core::reasoning::RecommendationBucket::Cautious
                | amlich_core::reasoning::RecommendationBucket::Avoid
        ),
        "should have valid bucket"
    );
}

#[test]
fn graph_backed_evaluator_handles_personal_input() {
    use amlich_core::reasoning::{ActionEvaluator, InitiationOpeningEvaluator};

    let snapshot = calculate_day_snapshot(13, 5, 2024);
    let personal_input = profile_input(1, 1, 1990, 9, 0, 7.0, Some(Gender::Male));

    let bazi_input = amlich_core::bazi::BaziInput {
        day: personal_input.birth.day,
        month: personal_input.birth.month,
        year: personal_input.birth.year,
        hour: personal_input.birth.hour.unwrap_or(0),
        minute: personal_input.birth.minute.unwrap_or(0),
        time_known: personal_input.birth.hour.is_some() && personal_input.birth.minute.is_some(),
        timezone: personal_input.birth.timezone,
        longitude: None,
        use_solar_time: false,
        gender: personal_input.birth.gender,
    };

    let graph = amlich_core::build_reasoning_input_graph(&snapshot, Some(&bazi_input))
        .expect("valid graph");
    let evaluator = InitiationOpeningEvaluator::new();

    let evaluation = evaluator
        .evaluate(&graph, &snapshot, Some(&personal_input))
        .expect("valid evaluation");

    assert!(
        !evaluation.primary_conclusion.is_empty(),
        "should have conclusion with personal input"
    );
    // amlich-zakn: axis scores now come from the canonical assessment's typed
    // contributions (snapshot-derived, deduplicated), so each is in the
    // normalized 0..=1 range rather than the legacy raw note counts. The
    // PersonalAlignment axis is only > 0 when a typed personal interaction
    // fact actually fires (it needs gender + a matching same_chi / luc_xung /
    // tam_hop / liu_he / kua fact), replacing the legacy "1.0 whenever any
    // personal input exists" boolean proxy.
    assert_eq!(
        evaluation.axis_scores.len(),
        6,
        "evaluation must expose all six reasoning axes"
    );
    for axis in &evaluation.axis_scores {
        assert!(
            axis.score >= 0.0 && axis.score <= 1.0,
            "axis {:?} score must come from the canonical assessment (0..=1); got {}",
            axis.axis,
            axis.score
        );
    }
}

#[test]
fn graph_backed_evaluator_suggested_hours_from_day() {
    use amlich_core::reasoning::project_initiation_opening_decision;
    use amlich_core::reasoning::{ActionEvaluator, InitiationOpeningEvaluator};

    let snapshot = calculate_day_snapshot(10, 2, 2024);
    let graph = amlich_core::build_reasoning_input_graph(&snapshot, None).expect("valid graph");
    let evaluator = InitiationOpeningEvaluator::new();

    let evaluation = evaluator
        .evaluate(&graph, &snapshot, None)
        .expect("valid evaluation");
    let decision = project_initiation_opening_decision(&evaluation);

    assert!(
        !decision.suggested_hours.is_empty(),
        "should have suggested hours from day"
    );
}
