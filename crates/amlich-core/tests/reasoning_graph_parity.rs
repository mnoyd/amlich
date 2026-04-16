use amlich_core::{build_initiation_opening_reasoning, calculate_day_snapshot, reasoning::RecommendationBucket};

struct ParityCase {
    id: &'static str,
    day: i32,
    month: i32,
    year: i32,
    expected_bucket: RecommendationBucket,
    expect_conflict_visibility: bool,
    expect_override_visibility: bool,
}

fn representative_cases() -> [ParityCase; 4] {
    [
        ParityCase {
            id: "strong_favorable",
            day: 13,
            month: 5,
            year: 2024,
            expected_bucket: RecommendationBucket::Favorable,
            expect_conflict_visibility: false,
            expect_override_visibility: false,
        },
        ParityCase {
            id: "strong_avoid",
            day: 3,
            month: 1,
            year: 2024,
            expected_bucket: RecommendationBucket::Avoid,
            expect_conflict_visibility: false,
            expect_override_visibility: true,
        },
        ParityCase {
            id: "conflicting_layered",
            day: 14,
            month: 2,
            year: 2024,
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: true,
            expect_override_visibility: true,
        },
        ParityCase {
            id: "sparse_relative",
            day: 22,
            month: 5,
            year: 2024,
            expected_bucket: RecommendationBucket::Cautious,
            expect_conflict_visibility: true,
            expect_override_visibility: false,
        },
    ]
}

#[test]
fn initiation_opening_reasoning_stays_stable_on_representative_dates() {
    for case in representative_cases() {
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
fn initiation_opening_reasoning_keeps_reasons_and_conflict_signals_visible() {
    for case in representative_cases() {
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
