//! Hour ranking policy v1 contract tests.
//!
//! Source spec: `docs/architecture/personal-day-audit/HOUR-RANKING-POLICY-V1-SPEC.md`
//! Epic: `amlich-rv13`. Bead: `amlich-rv13.6`.
//!
//! These tests pin the v1 contract on the **public** API surface
//! (`HourRankingPolicy::baseline_v1`, `rank_hours_for_intent`, and the
//! canonical `RankedHourV1` / `RankedHourCandidate` output shapes). They
//! are intentionally black-box against the public surface: the goal is
//! to lock the contract that downstream consumers (CLI, API, TUI,
//! desktop) rely on, not to retest internal mechanics already covered
//! in `crate::assessment::hour_ranking` and `crate::advisory` unit
//! tests.
//!
//! Each section maps to a bullet from the spec's "Verification gates"
//! list. The tests are organized so that a future spec gate removal /
//! addition shows up as a clearly-named test diff.
//!
//! Cross-reference vs the spec's verification gates:
//!
//! 1. all twelve traditional hour slots are returned
//! 2. Hoàng Đạo quality is binary under v1
//! 3. unavailable personal alignment reweights remaining axes and lowers
//!    confidence/context
//! 4. unavailable intent timing fit reweights remaining axes
//! 5. exact score ties break by traditional Chi order
//! 6. rank scores are deterministic for identical inputs and policy
//!    version
//! 7. compatibility wrapper preserves current broad behavior: Hoàng Đạo
//!    hours generally rank above Hắc Đạo hours
//! 8. an `Avoid` day assessment does not suppress ranking, but adds
//!    warning context
//! 9. no hour result contains a day-level verdict or changes the day
//!    assessment bucket

use amlich_core::{
    advisory::{
        build_hour_selection_reasoning, rank_hours_for_intent, BirthInput, ConsultationIntent,
        RankedHourCandidate,
    },
    almanac::tu_menh::Gender,
    assessment::{PersonalDayAssessmentBuilder, PersonalDayDecision},
    reasoning::RecommendationBucket,
    BirthProfile, DaySnapshot, HourRankingAxis, HourRankingPolicy, HourRankingWarning,
    HOUR_RANKING_POLICY_V1_ID, HOUR_RANKING_POLICY_V1_VERSION, VIETNAM_TIMEZONE,
};

fn snapshot_2024_02_10() -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE)
}

fn birth_input_1990() -> BirthInput {
    BirthInput {
        day: 1,
        month: 1,
        year: 1990,
        hour: None,
        minute: None,
        timezone: VIETNAM_TIMEZONE,
        gender: None,
        location_name: None,
    }
}

fn birth_profile_1990_male() -> BirthProfile {
    BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: None,
        timezone: VIETNAM_TIMEZONE,
        longitude: None,
        use_solar_time: false,
        gender: Some(Gender::Male),
        location_name: None,
    }
}

fn forced_bucket_assessment(
    snapshot: &DaySnapshot,
    bucket: RecommendationBucket,
) -> amlich_core::assessment::PersonalDayAssessment {
    let mut assessment = PersonalDayAssessmentBuilder::new(
        snapshot.clone(),
        birth_profile_1990_male(),
        ConsultationIntent::Travel,
    )
    .build();
    assessment.decision = PersonalDayDecision {
        bucket,
        ..assessment.decision
    };
    assessment
}

// ---------------------------------------------------------------------
// Gate 1: all twelve traditional hour slots are returned.
//
// The spec's candidate set is exactly the twelve traditional Chi-aligned
// slots; the policy must score every one and never filter to a subset.
// ---------------------------------------------------------------------

#[test]
fn v1_returns_exactly_twelve_hour_slots() {
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Travel,
            None,
            None,
        )
        .expect("rank");
    assert_eq!(
        ranked.len(),
        12,
        "v1 must return all twelve traditional hour slots; got {}",
        ranked.len()
    );
}

#[test]
fn v1_covers_every_chi_index_exactly_once() {
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Travel,
            None,
            None,
        )
        .expect("rank");
    let mut indices: Vec<usize> = ranked.iter().map(|h| h.chi_index).collect();
    indices.sort_unstable();
    assert_eq!(
        indices,
        (0..12).collect::<Vec<_>>(),
        "v1 must cover each chi_index 0..12 exactly once; got {indices:?}"
    );
}

#[test]
fn v1_compatibility_wrapper_returns_all_twelve_slots() {
    let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
        .expect("ranked hours");
    assert_eq!(ranked.len(), 12);
}

// ---------------------------------------------------------------------
// Gate 2: Hoàng Đạo quality is binary under v1.
//
// The spec §"Hoàng Đạo quality" is explicit: 1.0 for Hoàng Đạo, 0.0
// for Hắc Đạo, no sub-grades. This test pins that public contract on
// the canonical `RankedHourV1.hoang_dao_quality` axis outcome.
// ---------------------------------------------------------------------

#[test]
fn v1_hoang_dao_quality_is_binary_no_subgrades() {
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Travel,
            None,
            None,
        )
        .expect("rank");
    for hour in &ranked {
        let outcome = &hour.axes.hoang_dao_quality;
        assert!(
            outcome.score.is_some(),
            "hoang_dao_quality must always be available in v1 (snapshot gio_hoang_dao covers all twelve slots)"
        );
        let score = outcome.score.unwrap();
        if hour.is_auspicious {
            assert!(
                (score - 1.0).abs() < 1e-6,
                "Hoàng Đạo hour must score exactly 1.0; got {score}"
            );
        } else {
            assert!(
                score.abs() < 1e-6,
                "Hắc Đạo hour must score exactly 0.0; got {score}"
            );
        }
        // No sub-grades: the score must not be any intermediate value.
        assert!(
            (score - 1.0).abs() < 1e-6 || score.abs() < 1e-6,
            "v1 hoang_dao_quality is binary; got {score}"
        );
    }
}

#[test]
fn v1_is_auspicious_flag_matches_snapshot_hoang_dao_table() {
    let policy = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let ranked = policy
        .rank(&snapshot, ConsultationIntent::Travel, None, None)
        .expect("rank");
    let snapshot_chi: Vec<(usize, &str, bool)> = snapshot
        .context
        .gio_hoang_dao
        .all_hours
        .iter()
        .map(|h| (h.hour_index, h.hour_chi.as_str(), h.is_good))
        .collect();
    for hour in &ranked {
        let expected = snapshot_chi
            .iter()
            .find(|(idx, _, _)| *idx == hour.chi_index)
            .expect("hour must come from snapshot");
        assert_eq!(
            hour.is_auspicious, expected.2,
            "is_auspicious for chi_index={} must match snapshot hoang_dao table",
            hour.chi_index
        );
    }
}

// ---------------------------------------------------------------------
// Gate 3: unavailable personal alignment reweights remaining axes and
// lowers confidence/context.
//
// The contract surfaces in `HourRankingAxisOutcome.unavailable_reason`
// (the "context" half) and in the contribution list being shorter than
// the available-axis denominator (the "confidence" half — the consumer
// can see which axes were dropped).
// ---------------------------------------------------------------------

#[test]
fn v1_unavailable_personal_alignment_carries_unavailable_reason() {
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Wedding,
            None,
            None,
        )
        .expect("rank");
    for hour in &ranked {
        let outcome = &hour.axes.personal_hour_alignment;
        assert!(
            outcome.score.is_none(),
            "without birth, personal_hour_alignment must be unavailable"
        );
        let reason = outcome
            .unavailable_reason
            .as_deref()
            .expect("unavailable outcome must carry a human-readable reason");
        assert!(
            !reason.trim().is_empty(),
            "unavailable_reason must be non-empty so confidence/context is auditable"
        );
    }
}

#[test]
fn v1_unavailable_personal_alignment_drops_axis_from_contributions() {
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Wedding,
            None,
            None,
        )
        .expect("rank");
    for hour in &ranked {
        let axes: Vec<HourRankingAxis> = hour.contributions.iter().map(|c| c.axis).collect();
        assert!(
            !axes.contains(&HourRankingAxis::PersonalHourAlignment),
            "personal_hour_alignment must be excluded from contributions when unavailable; got {axes:?}"
        );
    }
}

#[test]
fn v1_birth_profile_makes_personal_alignment_available() {
    let policy = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let birth = birth_input_1990();
    let ranked = policy
        .rank(&snapshot, ConsultationIntent::Wedding, Some(&birth), None)
        .expect("rank");
    let mut any_match = false;
    for hour in &ranked {
        let outcome = &hour.axes.personal_hour_alignment;
        assert!(
            outcome.score.is_some(),
            "with birth profile, personal_hour_alignment must be available"
        );
        if (outcome.score.unwrap() - 1.0).abs() < 1e-6 {
            any_match = true;
        }
    }
    assert!(
        any_match,
        "at least one hour must carry the birth-year match (1.0) score"
    );
}

// ---------------------------------------------------------------------
// Gate 4: unavailable intent timing fit reweights remaining axes.
//
// v1 emits no source-backed intent×hour rules, so the axis is
// structurally unavailable. The rank_score formula must drop its 0.25
// weight from the denominator — the spec explicitly forbids folding in
// a neutral 0.5 fallback.
// ---------------------------------------------------------------------

#[test]
fn v1_intent_timing_fit_is_uniformly_unavailable_across_intents() {
    let policy = HourRankingPolicy::baseline_v1();
    let intents = [
        ConsultationIntent::Travel,
        ConsultationIntent::Wedding,
        ConsultationIntent::ContractSigning,
        ConsultationIntent::OpeningBusiness,
        ConsultationIntent::Burial,
    ];
    for intent in intents {
        let ranked = policy
            .rank(&snapshot_2024_02_10(), intent, None, None)
            .expect("rank");
        for hour in &ranked {
            let outcome = &hour.axes.intent_timing_fit;
            assert!(
                outcome.score.is_none(),
                "intent_timing_fit must be unavailable for intent={}",
                intent.event_kind()
            );
            assert!(
                outcome.unavailable_reason.is_some(),
                "intent_timing_fit unavailable outcome must carry a reason"
            );
            assert!(
                !hour
                    .contributions
                    .iter()
                    .any(|c| c.axis == HourRankingAxis::IntentTimingFit),
                "intent_timing_fit must never appear in contributions when unavailable"
            );
        }
    }
}

#[test]
fn v1_intent_change_does_not_change_rank_without_intent_signal() {
    // intent_timing_fit is the only axis that could in principle react
    // to intent. Since it is uniformly unavailable, the rank order and
    // score must be invariant across intents. This is a behavioural
    // lock on the v1 reweighting formula.
    let policy = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let a = policy
        .rank(&snapshot, ConsultationIntent::Travel, None, None)
        .expect("rank travel");
    let b = policy
        .rank(&snapshot, ConsultationIntent::Wedding, None, None)
        .expect("rank wedding");
    let c = policy
        .rank(&snapshot, ConsultationIntent::ContractSigning, None, None)
        .expect("rank contract");
    let ids = |r: &[amlich_core::RankedHourV1]| -> Vec<(String, String, f32)> {
        r.iter()
            .map(|h| (h.chi_name.clone(), h.time_range.clone(), h.rank_score))
            .collect()
    };
    assert_eq!(ids(&a), ids(&b));
    assert_eq!(ids(&a), ids(&c));
}

// ---------------------------------------------------------------------
// Gate 5: exact score ties break by traditional Chi order.
//
// The spec §"Ranking order" mandates the secondary key
// `chi_index` ascending. The contract forbids alphabetical
// Vietnamese-name tie-break.
// ---------------------------------------------------------------------

#[test]
fn v1_ties_break_by_chi_index_ascending_never_alphabetical() {
    let policy = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let ranked = policy
        .rank(&snapshot, ConsultationIntent::Travel, None, None)
        .expect("rank");
    // Walk the ranked list and assert that consecutive entries with
    // identical rank_score are ordered by chi_index ascending.
    for window in ranked.windows(2) {
        let left = &window[0];
        let right = &window[1];
        if (left.rank_score - right.rank_score).abs() < 1e-6 {
            assert!(
                left.chi_index < right.chi_index,
                "tie at score {} must break by chi_index ascending; got {} then {}",
                left.rank_score,
                left.chi_index,
                right.chi_index
            );
        }
    }
}

#[test]
fn v1_tie_break_within_group_is_chi_index_ascending() {
    let policy = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let ranked = policy
        .rank(&snapshot, ConsultationIntent::Travel, None, None)
        .expect("rank");
    let mut i = 0;
    while i < ranked.len() {
        let mut j = i;
        while j < ranked.len() && (ranked[j].rank_score - ranked[i].rank_score).abs() < 1e-6 {
            j += 1;
        }
        let group = &ranked[i..j];
        let mut chi_indices: Vec<usize> = group.iter().map(|h| h.chi_index).collect();
        let original = chi_indices.clone();
        chi_indices.sort_unstable();
        assert_eq!(
            original, chi_indices,
            "tie at score {} must group by chi_index ascending; got {:?}",
            ranked[i].rank_score, original
        );
        i = j;
    }
}

// ---------------------------------------------------------------------
// Gate 6: rank scores are deterministic for identical inputs and
// policy version.
//
// The policy version is part of the determinism contract: a future
// version bump that changes axes / weights must NOT satisfy this
// test (any version-bound code change must show up as a test diff).
// ---------------------------------------------------------------------

#[test]
fn v1_policy_id_and_version_are_locked() {
    let policy = HourRankingPolicy::baseline_v1();
    assert_eq!(
        policy.policy_id(),
        HOUR_RANKING_POLICY_V1_ID,
        "policy_id must be the v1 id constant"
    );
    assert_eq!(
        policy.policy_version(),
        HOUR_RANKING_POLICY_V1_VERSION,
        "policy_version must be the v1 version constant"
    );
    assert_eq!(
        policy.policy_version(),
        "v1",
        "v1 baseline version must be exactly 'v1'"
    );
}

#[test]
fn v1_axis_weights_match_spec_initial_weight_profile() {
    let policy = HourRankingPolicy::baseline_v1();
    let weights = policy.axis_weights();
    let profile: Vec<(HourRankingAxis, f32)> = weights.iter().map(|w| (w.axis, w.weight)).collect();
    assert_eq!(
        profile,
        vec![
            (HourRankingAxis::HoangDaoQuality, 0.45),
            (HourRankingAxis::IntentTimingFit, 0.25),
            (HourRankingAxis::PersonalHourAlignment, 0.20),
            (HourRankingAxis::DayHourHarmony, 0.10),
        ],
        "v1 axis weights must match the spec's 'Initial weight profile' verbatim"
    );
    let total: f32 = weights.iter().map(|w| w.weight).sum();
    assert!(
        (total - 1.0).abs() < 1e-6,
        "v1 axis weights must sum to 1.0; got {total}"
    );
}

#[test]
fn v1_is_deterministic_for_identical_inputs() {
    let policy = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let a = policy
        .rank(&snapshot, ConsultationIntent::Travel, None, None)
        .expect("rank a");
    let b = policy
        .rank(&snapshot, ConsultationIntent::Travel, None, None)
        .expect("rank b");
    assert_eq!(
        a, b,
        "v1 rank must be deterministic for identical (snapshot, intent, birth, assessment) inputs"
    );
}

#[test]
fn v1_score_is_in_unit_interval() {
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Travel,
            None,
            None,
        )
        .expect("rank");
    for hour in &ranked {
        assert!(
            (0.0..=1.0).contains(&hour.rank_score),
            "rank_score must lie in [0.0, 1.0]; got {}",
            hour.rank_score
        );
    }
}

#[test]
fn v1_score_is_deterministic_across_policy_instances() {
    // Two independent baseline_v1 instances must produce byte-equal
    // results — proves the policy has no hidden state mutated across
    // rank calls.
    let a = HourRankingPolicy::baseline_v1();
    let b = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let ra = a
        .rank(&snapshot, ConsultationIntent::Travel, None, None)
        .expect("rank a");
    let rb = b
        .rank(&snapshot, ConsultationIntent::Travel, None, None)
        .expect("rank b");
    assert_eq!(ra, rb);
}

// ---------------------------------------------------------------------
// Gate 7: compatibility wrapper preserves current broad behavior.
//
// The legacy `rank_hours_for_intent` shape returns integer
// `0..=100` scores on twelve slots. v1 must keep that surface intact:
// twelve slots, integer score in `[0, 100]`, Hoàng Đạo hours strictly
// outranking Hắc Đạo hours (no birth, intent_timing unavailable).
// ---------------------------------------------------------------------

#[test]
fn v1_compat_wrapper_returns_twelve_slots_with_int_scores_in_range() {
    let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
        .expect("ranked hours");
    assert_eq!(ranked.len(), 12);
    for hour in &ranked {
        assert!(
            (0..=100).contains(&hour.score),
            "compat score must be in 0..=100; got {}",
            hour.score
        );
    }
}

#[test]
fn v1_compat_wrapper_hoang_dao_strictly_outranks_hac_dao() {
    let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
        .expect("ranked hours");
    let min_hoang = ranked
        .iter()
        .filter(|h| h.is_auspicious)
        .map(|h| h.score)
        .min()
        .expect("at least one Hoàng Đạo hour");
    let max_hac = ranked
        .iter()
        .filter(|h| !h.is_auspicious)
        .map(|h| h.score)
        .max()
        .expect("at least one Hắc Đạo hour");
    assert!(
        min_hoang > max_hac,
        "Hoàng Đạo hours must strictly outrank Hắc Đạo hours; min_hoang={min_hoang}, max_hac={max_hac}"
    );
}

#[test]
fn v1_compat_wrapper_score_is_monotonic_with_v1_rank_score() {
    // The compat score is `round(rank_score * 100).clamp(0, 100)`, so
    // ordering is preserved. This pins the projection formula at the
    // public contract boundary.
    let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
        .expect("ranked hours");
    let policy = HourRankingPolicy::baseline_v1();
    let v1 = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Travel,
            None,
            None,
        )
        .expect("v1 rank");
    for hour in &ranked {
        let v1_hour = v1
            .iter()
            .find(|h| h.chi_name == hour.chi_name && h.time_range == hour.time_range)
            .expect("every wrapper hour must map to a v1 hour");
        let expected = (v1_hour.rank_score * 100.0).round().clamp(0.0, 100.0) as i32;
        assert_eq!(
            hour.score, expected,
            "compat score for {} must equal round(v1_rank_score * 100) = {expected}, got {}",
            v1_hour.chi_name, hour.score
        );
    }
}

#[test]
fn v1_compat_wrapper_is_deterministic() {
    let a =
        rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None).expect("rank a");
    let b =
        rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None).expect("rank b");
    assert_eq!(a, b);
}

// ---------------------------------------------------------------------
// Gate 8: an `Avoid` day assessment does not suppress ranking, but adds
// warning context.
//
// Day assessment stays the canonical day-suitability source. Hour
// ranking must keep returning twelve slots with the same scores, then
// attach a structured `HourRankingWarning` to every ranked hour.
// ---------------------------------------------------------------------

#[test]
fn v1_avoid_day_assessment_keeps_twelve_slots_with_same_scores() {
    let snapshot = snapshot_2024_02_10();
    let assessment = forced_bucket_assessment(&snapshot, RecommendationBucket::Avoid);
    let no_assessment = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
        .expect("no assessment");
    let with_avoid = rank_hours_for_intent(
        10,
        2,
        2024,
        ConsultationIntent::Travel,
        None,
        Some(&assessment),
    )
    .expect("with avoid assessment");
    assert_eq!(no_assessment.len(), with_avoid.len());
    assert_eq!(no_assessment.len(), 12);
    for (lhs, rhs) in no_assessment.iter().zip(with_avoid.iter()) {
        assert_eq!(lhs.chi_name, rhs.chi_name);
        assert_eq!(lhs.time_range, rhs.time_range);
        assert_eq!(lhs.is_auspicious, rhs.is_auspicious);
        assert_eq!(
            lhs.score, rhs.score,
            "Avoid day must not change compat rank score for {}",
            lhs.chi_name
        );
    }
}

#[test]
fn v1_avoid_day_assessment_attaches_structured_warning_to_canonical_output() {
    let snapshot = snapshot_2024_02_10();
    let assessment = forced_bucket_assessment(&snapshot, RecommendationBucket::Avoid);
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("rank");
    assert_eq!(ranked.len(), 12);
    for hour in &ranked {
        let warning = hour
            .warning_context
            .as_ref()
            .expect("Avoid day must attach warning_context to every hour");
        assert_eq!(warning.day_bucket, RecommendationBucket::Avoid);
        assert!(
            !warning.message_vi.trim().is_empty(),
            "warning context must carry a non-empty Vietnamese message"
        );
    }
}

#[test]
fn v1_non_avoid_day_assessment_omits_warning_context() {
    let snapshot = snapshot_2024_02_10();
    let assessment = forced_bucket_assessment(&snapshot, RecommendationBucket::Favorable);
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("rank");
    for hour in &ranked {
        assert!(
            hour.warning_context.is_none(),
            "Favorable day must NOT attach warning_context; got {:?}",
            hour.warning_context
        );
    }
}

#[test]
fn v1_avoid_day_warning_serializes_in_json() {
    let snapshot = snapshot_2024_02_10();
    let assessment = forced_bucket_assessment(&snapshot, RecommendationBucket::Avoid);
    let reasoning = build_hour_selection_reasoning(
        10,
        2,
        2024,
        ConsultationIntent::Travel,
        None,
        Some(&assessment),
    )
    .expect("reasoning");
    let warning = reasoning
        .warning_context
        .as_ref()
        .expect("Avoid day reasoning must carry warning_context");
    assert_eq!(warning.day_bucket, RecommendationBucket::Avoid);

    let export = reasoning.export(None);
    let export_warning: &HourRankingWarning = export
        .warning_context
        .as_ref()
        .expect("export must surface warning_context for Avoid days");
    assert_eq!(export_warning.day_bucket, RecommendationBucket::Avoid);

    let json = serde_json::to_string(&export).expect("serialize");
    assert!(
        json.contains("\"warning_context\""),
        "Avoid export JSON must include warning_context; got {json}"
    );
}

#[test]
fn v1_warning_context_is_omitted_from_json_when_absent() {
    let reasoning =
        build_hour_selection_reasoning(10, 2, 2024, ConsultationIntent::Travel, None, None)
            .expect("reasoning");
    assert!(reasoning.warning_context.is_none());
    let export = reasoning.export(None);
    let json = serde_json::to_string(&export).expect("serialize");
    assert!(
        !json.contains("\"warning_context\""),
        "absent warning_context must NOT appear in JSON; got {json}"
    );
}

// ---------------------------------------------------------------------
// Gate 9: no hour result contains a day-level verdict or changes the
// day assessment bucket.
//
// The day policy is the canonical day-suitability authority. Hour
// ranking must never:
//   * emit a "bucket" / "verdict" / "decision" field on a ranked hour,
//   * flip the day assessment's bucket when the same inputs are re-used.
// ---------------------------------------------------------------------

#[test]
fn v1_ranked_hour_does_not_carry_a_day_verdict() {
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Travel,
            None,
            None,
        )
        .expect("rank");
    for hour in &ranked {
        let debug = format!("{hour:?}");
        assert!(
            !debug.contains("bucket"),
            "RankedHourV1 must not carry a day-level bucket; got {debug}"
        );
        assert!(
            !debug.contains("verdict"),
            "RankedHourV1 must not carry a day-level verdict; got {debug}"
        );
        assert!(
            !debug.contains("decision"),
            "RankedHourV1 must not carry a day-level decision; got {debug}"
        );
    }
}

#[test]
fn v1_ranked_hour_does_not_carry_a_day_field_in_json() {
    let policy = HourRankingPolicy::baseline_v1();
    let ranked = policy
        .rank(
            &snapshot_2024_02_10(),
            ConsultationIntent::Travel,
            None,
            None,
        )
        .expect("rank");
    for hour in &ranked {
        let json = serde_json::to_string(hour).expect("serialize");
        for forbidden in ["bucket", "verdict", "decision", "decision_score"] {
            assert!(
                !json.contains(forbidden),
                "RankedHourV1 JSON must not contain '{forbidden}'; got {json}"
            );
        }
    }
}

#[test]
fn v1_rank_does_not_modify_day_assessment_bucket() {
    // Same snapshot + same assessment, with vs. without calling v1.rank.
    // The assessment's decision.bucket must be unchanged — hour ranking
    // is a rank-only consumer of the day assessment.
    let snapshot = snapshot_2024_02_10();
    let assessment = forced_bucket_assessment(&snapshot, RecommendationBucket::Avoid);
    let bucket_before = assessment.decision.bucket;

    let policy = HourRankingPolicy::baseline_v1();
    let _ = policy
        .rank(
            &snapshot,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("rank");

    let bucket_after = assessment.decision.bucket;
    assert_eq!(
        bucket_before, bucket_after,
        "v1.rank must not change the threaded day assessment bucket"
    );
}

// ---------------------------------------------------------------------
// Cross-cutting contract guards.
//
// These sit outside the spec's verification gate list but are part of
// the public contract: the contribution list must be in canonical axis
// order, the wrapped `RankedHourCandidate` must still be returned
// sorted by `rank_score` descending, and the canonical warnings must
// surface identically across the wrapper and the canonical reasoning
// surfaces.
// ---------------------------------------------------------------------

#[test]
fn v1_contributions_are_in_canonical_axis_order() {
    let policy = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let birth = birth_input_1990();
    let ranked = policy
        .rank(&snapshot, ConsultationIntent::Wedding, Some(&birth), None)
        .expect("rank");
    // Contributions are emitted in axis_weights declaration order
    // (HoangDaoQuality, IntentTimingFit, PersonalHourAlignment,
    // DayHourHarmony) with the intent axis skipped because it is
    // unavailable in v1. The remaining three therefore follow the
    // canonical order.
    let canonical_with_birth = [
        HourRankingAxis::HoangDaoQuality,
        HourRankingAxis::PersonalHourAlignment,
        HourRankingAxis::DayHourHarmony,
    ];
    for hour in &ranked {
        let axes: Vec<HourRankingAxis> = hour.contributions.iter().map(|c| c.axis).collect();
        assert_eq!(
            axes, canonical_with_birth,
            "contributions must follow canonical axis order for chi_index={}",
            hour.chi_index
        );
    }
}

#[test]
fn v1_contributions_without_birth_omit_personal_axis() {
    let policy = HourRankingPolicy::baseline_v1();
    let snapshot = snapshot_2024_02_10();
    let ranked = policy
        .rank(&snapshot, ConsultationIntent::Wedding, None, None)
        .expect("rank");
    let canonical_without_birth = [
        HourRankingAxis::HoangDaoQuality,
        HourRankingAxis::DayHourHarmony,
    ];
    for hour in &ranked {
        let axes: Vec<HourRankingAxis> = hour.contributions.iter().map(|c| c.axis).collect();
        assert_eq!(
            axes, canonical_without_birth,
            "without birth, contributions must be HoangDaoQuality + DayHourHarmony for chi_index={}",
            hour.chi_index
        );
    }
}

#[test]
fn v1_compat_wrapper_returns_slots_sorted_by_score_desc() {
    let ranked: Vec<RankedHourCandidate> =
        rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
            .expect("ranked hours");
    for window in ranked.windows(2) {
        let left = &window[0];
        let right = &window[1];
        assert!(
            left.score >= right.score,
            "compat wrapper must return slots sorted by score descending; got {} then {}",
            left.score,
            right.score
        );
    }
}

#[test]
fn v1_compat_wrapper_note_vi_carries_hoang_dao_clause() {
    // The compat `note_vi` is a derived Vietnamese description; its
    // first clause must always describe the Hoàng Đạo / Hắc Đạo
    // membership so existing readers of the string keep working.
    let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
        .expect("ranked hours");
    for hour in &ranked {
        if hour.is_auspicious {
            assert!(
                hour.note_vi.contains("là giờ hoàng đạo"),
                "Hoàng Đạo note_vi must lead with the 'là giờ hoàng đạo' clause; got {:?}",
                hour.note_vi
            );
        } else {
            assert!(
                hour.note_vi.contains("không thuộc giờ hoàng đạo"),
                "Hắc Đạo note_vi must lead with the 'không thuộc giờ hoàng đạo' clause; got {:?}",
                hour.note_vi
            );
        }
    }
}

#[test]
fn v1_compat_wrapper_warning_matches_canonical_warning_for_avoid_day() {
    // The compat `[Cảnh báo]` clause and the canonical
    // `warning_context` must carry the same underlying message for an
    // Avoid day. This locks the legacy string-projection against the
    // structured warning.
    let snapshot = snapshot_2024_02_10();
    let assessment = forced_bucket_assessment(&snapshot, RecommendationBucket::Avoid);
    let ranked = rank_hours_for_intent(
        10,
        2,
        2024,
        ConsultationIntent::Travel,
        None,
        Some(&assessment),
    )
    .expect("ranked hours");
    let policy = HourRankingPolicy::baseline_v1();
    let v1 = policy
        .rank(
            &snapshot,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("v1 rank");
    for hour in &ranked {
        let v1_hour = v1
            .iter()
            .find(|h| h.chi_name == hour.chi_name && h.time_range == hour.time_range)
            .expect("v1 hour");
        let warning = v1_hour
            .warning_context
            .as_ref()
            .expect("v1 hour must carry warning_context for Avoid day");
        assert!(
            hour.note_vi.contains("[Cảnh báo]"),
            "compat note_vi must carry the [Cảnh báo] prefix for Avoid day; got {:?}",
            hour.note_vi
        );
        assert!(
            hour.note_vi.contains(&warning.message_vi),
            "compat note_vi must embed the canonical warning message; got {:?}",
            hour.note_vi
        );
    }
}
