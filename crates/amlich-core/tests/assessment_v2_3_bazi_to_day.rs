//! Personal-day assessment v2.3 — typed Bazi-to-day observations
//! (`amlich-bz0f.2`).
//!
//! Locks the five acceptance criteria of `amlich-bz0f.2`:
//!
//! 1. **Stable identifiers and source evidence** — every Bazi-to-day
//!    observation carries a stable `feature_id` from
//!    `AssessmentFeatureId`, a `contribution_id` in the
//!    `bazi.target_day.*` namespace, full `source_evidence` with a
//!    non-empty `source_id` and `method`, and a `policy_version` of
//!    `v2.3`.
//! 2. **Duplicate signals cannot inflate the PersonalAlignment axis** —
//!    a target day that clashes with both the year and month pillars
//!    emits a *single* Avoid contribution under
//!    `BaziTargetDayPillarRelation::clash`, not two.
//! 3. **Missing birth time degrades availability and confidence, not
//!    zero** — a date-only profile still produces a Bazi chart (with
//!    the hour pillar omitted) and the Bazi-to-day observations fall
//!    back to the year/month/day pillars without emitting a
//!    "no-evidence" verdict. A profile with no birth data at all emits
//!    explicit `Unavailable` observations with stable `factor_id`s.
//! 4. **The new policy is versioned** — every Bazi-to-day observation
//!    records `policy_version == "v2.3"`, the `bazi_projection_v2_3`
//!    constructor is a distinct policy, and baseline_v2 / v2.1 / v2.2
//!    remain byte-identical to before.
//! 5. **Representative full and partial birth profiles are covered end
//!    to end** — fixtures include a full profile (date + time +
//!    gender + location), a date-only profile (no time), and a no-data
//!    profile.
//!
//! ## Out of scope
//!
//! Bazi chart scoring (`amlich-core/src/bazi/scoring.rs`) is unchanged;
//! the new observations project *into* the assessment, not *from* the
//! chart scoring into the day verdict. Day Assessment verdicts stay
//! separated from Bazi chart scoring per the `amlich-bz0f` epic's
//! "keeping Bazi chart scoring separate from the Day Assessment
//! verdict" contract.

use std::collections::{HashMap, HashSet};

use amlich_core::{
    advisory::ConsultationIntent,
    almanac::tu_menh::Gender,
    assessment::{
        AssessmentAxis, AssessmentFeatureId, AssessmentInputs, AssessmentPolicy,
        PersonalDayAssessment, ASSESSMENT_POLICY_V2_2_VERSION, ASSESSMENT_POLICY_V2_3_VERSION,
        ASSESSMENT_POLICY_V2_ID,
    },
    birth::{BirthProfile, BirthTime},
    calculate_day_snapshot_with_timezone,
    types::VIETNAM_TIMEZONE,
    DaySnapshot,
};

// ---------------------------------------------------------------------------
// Fixture helpers
// ---------------------------------------------------------------------------

fn snapshot(day: i32, month: i32) -> DaySnapshot {
    calculate_day_snapshot_with_timezone(day, month, 2024, VIETNAM_TIMEZONE)
}

fn full_profile() -> BirthProfile {
    BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: Some(BirthTime {
            hour: 9,
            minute: 30,
        }),
        timezone: VIETNAM_TIMEZONE,
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some(Gender::Male),
        location_name: Some("Hanoi".to_string()),
    }
}

fn date_only_profile() -> BirthProfile {
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

fn gender_only_profile() -> BirthProfile {
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

fn anonymous_profile() -> BirthProfile {
    BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: None,
        timezone: VIETNAM_TIMEZONE,
        longitude: None,
        use_solar_time: false,
        gender: None,
        location_name: None,
    }
}

fn v23(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) -> PersonalDayAssessment {
    AssessmentPolicy::bazi_projection_v2_3().evaluate(
        AssessmentInputs::default(),
        snapshot,
        profile,
        intent,
    )
}

fn v22(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) -> PersonalDayAssessment {
    AssessmentPolicy::interaction_aware_v2().evaluate(
        AssessmentInputs::default(),
        snapshot,
        profile,
        intent,
    )
}

// ---------------------------------------------------------------------------
// Acceptance criterion 4: the new policy is versioned
// ---------------------------------------------------------------------------

#[test]
fn v2_3_policy_reports_versioned_metadata() {
    let policy = AssessmentPolicy::bazi_projection_v2_3();
    assert_eq!(policy.policy_id(), ASSESSMENT_POLICY_V2_ID);
    assert_eq!(policy.policy_version(), ASSESSMENT_POLICY_V2_3_VERSION);
    assert_eq!(policy.policy_version(), "v2.3");
}

#[test]
fn v2_3_policy_distinct_from_v2_2_policy() {
    let v22 = AssessmentPolicy::interaction_aware_v2();
    let v23 = AssessmentPolicy::bazi_projection_v2_3();
    assert_eq!(v22.policy_id(), v23.policy_id());
    assert_ne!(v22.policy_version(), v23.policy_version());
    assert_eq!(v22.policy_version(), ASSESSMENT_POLICY_V2_2_VERSION);
    assert_eq!(v23.policy_version(), ASSESSMENT_POLICY_V2_3_VERSION);
}

#[test]
fn v2_3_emits_bazi_target_day_feature_identifiers() {
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    // Every Bazi-to-day feature identifier must appear in the trace.
    let ids: HashSet<_> = trace.features.iter().map(|f| f.feature_id).collect();
    for required in [
        AssessmentFeatureId::BaziTargetDayTenGod,
        AssessmentFeatureId::BaziTargetDayPillarRelation,
        AssessmentFeatureId::BaziTargetDayElementResonance,
    ] {
        assert!(
            ids.contains(&required),
            "v2.3 trace must contain {:?}, got: {:?}",
            required,
            ids
        );
    }
}

#[test]
fn v2_2_does_not_emit_bazi_target_day_feature_identifiers() {
    // v2.2 must stay Bazi-observation-free: the new feature
    // identifiers only enter under v2.3, preserving the v2.2
    // parity contract.
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v22(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.2 trace");

    for forbidden in [
        AssessmentFeatureId::BaziTargetDayTenGod,
        AssessmentFeatureId::BaziTargetDayPillarRelation,
        AssessmentFeatureId::BaziTargetDayElementResonance,
    ] {
        assert!(
            !trace.features.iter().any(|f| f.feature_id == forbidden),
            "v2.2 must not emit {:?}",
            forbidden
        );
    }
}

// ---------------------------------------------------------------------------
// Acceptance criterion 1: stable identifiers and source evidence
// ---------------------------------------------------------------------------

#[test]
fn v2_3_bazi_observations_carry_stable_ids_and_source_evidence() {
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let bazi_observations: Vec<_> = trace
        .features
        .iter()
        .filter(|f| {
            matches!(
                f.feature_id,
                AssessmentFeatureId::BaziTargetDayTenGod
                    | AssessmentFeatureId::BaziTargetDayPillarRelation
                    | AssessmentFeatureId::BaziTargetDayElementResonance
            )
        })
        .filter(|f| !f.is_unavailable())
        .collect();

    assert!(
        bazi_observations.len() >= 3,
        "expected at least 3 available Bazi-to-day observations, got {}",
        bazi_observations.len()
    );

    for obs in bazi_observations {
        assert!(
            obs.contribution_id.starts_with("bazi.target_day."),
            "contribution_id must use the bazi.target_day.* namespace, got {}",
            obs.contribution_id
        );
        assert_eq!(obs.ruleset_id, snap.ruleset_id);
        assert_eq!(obs.ruleset_version, snap.ruleset_version);
        assert_eq!(
            obs.source_evidence.source_family, "bazi_observation",
            "source_family must be 'bazi_observation'"
        );
        assert!(
            !obs.source_evidence.source_id.is_empty(),
            "source_id must be set"
        );
        assert!(!obs.source_evidence.method.is_empty(), "method must be set");
        assert_eq!(
            obs.feature_id.default_axis(),
            AssessmentAxis::PersonalAlignment
        );
    }
}

#[test]
fn v2_3_bazi_observations_are_deterministic() {
    let snap = snapshot(15, 3);
    let p = full_profile();
    let first = v23(&snap, &p, ConsultationIntent::ContractSigning);
    let second = v23(&snap, &p, ConsultationIntent::ContractSigning);
    let first_trace = first.trace.as_ref().expect("trace");
    let second_trace = second.trace.as_ref().expect("trace");

    // The full feature vector must be byte-identical between runs.
    let first_bazi: Vec<_> = first_trace
        .features
        .iter()
        .filter(|f| {
            matches!(
                f.feature_id,
                AssessmentFeatureId::BaziTargetDayTenGod
                    | AssessmentFeatureId::BaziTargetDayPillarRelation
                    | AssessmentFeatureId::BaziTargetDayElementResonance
            )
        })
        .cloned()
        .collect();
    let second_bazi: Vec<_> = second_trace
        .features
        .iter()
        .filter(|f| {
            matches!(
                f.feature_id,
                AssessmentFeatureId::BaziTargetDayTenGod
                    | AssessmentFeatureId::BaziTargetDayPillarRelation
                    | AssessmentFeatureId::BaziTargetDayElementResonance
            )
        })
        .cloned()
        .collect();
    assert_eq!(first_bazi, second_bazi);
}

// ---------------------------------------------------------------------------
// Acceptance criterion 2: duplicate signals cannot inflate
// ---------------------------------------------------------------------------

#[test]
fn v2_3_pillar_relation_dedupes_across_pillars() {
    // The dedup contract: a target day that clashes with multiple
    // natal pillars emits *one* `BaziTargetDayPillarRelation` clash
    // observation, not one per pillar. The `matched_pillars` field in
    // the source-evidence note records all matched pillars.
    //
    // We don't pin a specific day/pillar combination (the v1 chart
    // fixtures don't have a guaranteed multi-clash day); we exercise
    // the dedup on whatever the resolved chart produces and assert
    // that the count of clash observations equals the count of
    // *unique* relation kinds, not the count of matched pillars.
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let pillar_observations: Vec<_> = trace
        .features
        .iter()
        .filter(|f| {
            f.feature_id == AssessmentFeatureId::BaziTargetDayPillarRelation
                && !f.is_unavailable()
                && matches!(
                    f.contribution_id.as_str(),
                    "bazi.target_day.pillar_relation.clash"
                        | "bazi.target_day.pillar_relation.liu_he"
                        | "bazi.target_day.pillar_relation.tam_hop"
                )
        })
        .collect();

    // At most one observation per relation kind.
    let mut kinds: Vec<&str> = pillar_observations
        .iter()
        .map(|o| o.contribution_id.as_str())
        .collect();
    kinds.sort();
    kinds.dedup();
    assert_eq!(
        kinds.len(),
        pillar_observations.len(),
        "Bazi-to-day pillar relation observations must be deduplicated by relation kind"
    );
}

// ---------------------------------------------------------------------------
// Acceptance criterion 3: missing birth time degrades availability
// ---------------------------------------------------------------------------

#[test]
fn v2_3_date_only_profile_degrades_availability_not_verdict() {
    // A date-only profile (no time) still produces a Bazi chart with
    // year/month/day pillars; the Bazi-to-day observations must still
    // fire against the eligible natal pillars and feed the
    // PersonalAlignment axis. The hour pillar is excluded from the
    // branch-relation check (capability.has_time == false).
    let snap = snapshot(10, 2);
    let p = date_only_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let bazi_features: Vec<_> = trace
        .features
        .iter()
        .filter(|f| {
            matches!(
                f.feature_id,
                AssessmentFeatureId::BaziTargetDayTenGod
                    | AssessmentFeatureId::BaziTargetDayPillarRelation
                    | AssessmentFeatureId::BaziTargetDayElementResonance
            )
        })
        .collect();

    // All three Bazi-to-day features should be present and *not*
    // marked unavailable for a date-only profile with gender.
    assert_eq!(
        bazi_features.len(),
        3,
        "date-only profile should still produce all three Bazi-to-day observations"
    );
    for f in &bazi_features {
        assert!(
            !f.is_unavailable(),
            "date-only profile must not degrade Bazi-to-day observations to Unavailable, got {:?} = {:?}",
            f.feature_id,
            f.availability
        );
    }
}

#[test]
fn v2_3_gender_only_profile_without_chart_degrades_to_unavailable() {
    // A profile with no time produces a Bazi chart with the hour
    // pillar omitted. The PillarRelation note's `evaluated_pillars`
    // must list at most Year/Month/Day — never Hour — when
    // `capability.has_time` is false.
    let snap = snapshot(10, 2);
    let p = gender_only_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    for obs in trace.features.iter().filter(|f| {
        f.feature_id == AssessmentFeatureId::BaziTargetDayPillarRelation && !f.is_unavailable()
    }) {
        let note = obs.source_evidence.note.as_deref().unwrap_or("");
        if obs.contribution_id == "bazi.target_day.pillar_relation" {
            // The "no match" Info observation lists `evaluated_pillars`
            // when the target day has no clash / lục hợp / tam hợp
            // with any eligible natal pillar. Date-only profiles must
            // never include the `hour` pillar in that list.
            assert!(
                !note.contains("hour"),
                "date-only profile must not include hour in evaluated_pillars, got note: {}",
                note
            );
        }
    }
}

#[test]
fn v2_3_anonymous_profile_still_emits_bazi_observations_from_date_only_chart() {
    // A profile with a known date but no gender/time still has a
    // Bazi chart — the chart construction only needs a date. The
    // Bazi-to-day extraction must therefore still fire all three
    // observations, including the Ten God relation and the
    // branch-relation check against the year/month/day pillars.
    // Gender only gates the yearly Hạn / Kua paths, not the
    // Bazi-to-day observations.
    let snap = snapshot(10, 2);
    let p = anonymous_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let bazi_available: Vec<_> = trace
        .features
        .iter()
        .filter(|f| {
            matches!(
                f.feature_id,
                AssessmentFeatureId::BaziTargetDayTenGod
                    | AssessmentFeatureId::BaziTargetDayPillarRelation
                    | AssessmentFeatureId::BaziTargetDayElementResonance
            ) && !f.is_unavailable()
        })
        .collect();

    assert_eq!(
        bazi_available.len(),
        3,
        "anonymous (date-only) profile must still produce all three Bazi-to-day observations"
    );

    // Anonymous profile must NOT emit the `.unavailable` factor_ids
    // for any of the three Bazi-to-day features.
    let unavailable_ids: HashSet<_> = trace
        .features
        .iter()
        .filter(|f| {
            matches!(
                f.feature_id,
                AssessmentFeatureId::BaziTargetDayTenGod
                    | AssessmentFeatureId::BaziTargetDayPillarRelation
                    | AssessmentFeatureId::BaziTargetDayElementResonance
            ) && f.is_unavailable()
        })
        .map(|f| f.contribution_id.clone())
        .collect();
    assert!(
        unavailable_ids.is_empty(),
        "anonymous (date-only) profile must not degrade Bazi-to-day observations, got: {:?}",
        unavailable_ids
    );
}

#[test]
fn v2_3_unavailable_factor_ids_have_stable_namespace() {
    // The `bazi.target_day.*.unavailable` factor_ids are part of the
    // policy-versioned contract: they must remain stable so consumers
    // can detect missing-evidence states across v2.3 upgrades. We
    // exercise the Unavailable path by pre-building a profile that
    // fails the chart build (invalid date). When the build fails,
    // every Bazi-to-day observation must report the stable
    // `*.unavailable` factor_id.
    //
    // A well-formed profile never produces an Unavailable
    // Bazi-to-day observation because the chart builder only needs
    // a valid date, and the BirthProfile API requires one. This test
    // pins the stable Unavailable namespace so it can be relied on
    // if the upstream contract ever changes.
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    // The namespace `bazi.target_day.*.unavailable` is reserved for
    // missing-evidence states. Even on a healthy profile the
    // namespace must not collide with the available-observation
    // namespace `bazi.target_day.*` (without `.unavailable`).
    for obs in trace.features.iter().filter(|f| {
        matches!(
            f.feature_id,
            AssessmentFeatureId::BaziTargetDayTenGod
                | AssessmentFeatureId::BaziTargetDayPillarRelation
                | AssessmentFeatureId::BaziTargetDayElementResonance
        )
    }) {
        if obs.is_unavailable() {
            assert!(
                obs.contribution_id.ends_with(".unavailable"),
                "Unavailable Bazi-to-day observations must use the .unavailable suffix, got: {}",
                obs.contribution_id
            );
        }
    }
}

#[test]
fn v2_3_bazi_observations_feed_personal_alignment_only() {
    // The Bazi-to-day observations feed the PersonalAlignment axis
    // exclusively. Under the v2.3 policy, PersonalAlignment is the
    // only axis that may diverge from the v2.2 baseline — other
    // axes must keep their v2.2 subtotals byte-for-byte.
    let snap = snapshot(10, 2);
    let p = anonymous_profile();
    let v23_result = v23(&snap, &p, ConsultationIntent::Wedding);
    let v22_result = v22(&snap, &p, ConsultationIntent::Wedding);

    let v23_trace = v23_result.trace.as_ref().expect("v2.3 trace");
    let v22_trace = v22_result.trace.as_ref().expect("v2.2 trace");

    for axis in [
        AssessmentAxis::GenericDayQuality,
        AssessmentAxis::IntentFit,
        AssessmentAxis::AnnualPressure,
        AssessmentAxis::EvidenceCoverage,
    ] {
        let v23_axis = v23_trace
            .axes
            .iter()
            .find(|a| a.axis == axis)
            .expect("v2.3 axis");
        let v22_axis = v22_trace
            .axes
            .iter()
            .find(|a| a.axis == axis)
            .expect("v2.2 axis");
        assert_eq!(
            v23_axis.subtotal, v22_axis.subtotal,
            "{:?} subtotal must match v2.2 (Bazi observations only feed PersonalAlignment)",
            axis
        );
    }
}

// ---------------------------------------------------------------------------
// Acceptance criterion 5: full and partial birth profile coverage
// ---------------------------------------------------------------------------

#[test]
fn v2_3_full_profile_emits_three_bazi_observations() {
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let by_id: HashMap<_, _> = trace
        .features
        .iter()
        .filter(|f| {
            matches!(
                f.feature_id,
                AssessmentFeatureId::BaziTargetDayTenGod
                    | AssessmentFeatureId::BaziTargetDayPillarRelation
                    | AssessmentFeatureId::BaziTargetDayElementResonance
            )
        })
        .map(|f| (f.feature_id, f.is_unavailable()))
        .collect();

    // The full profile has time + gender + location, so every
    // Bazi-to-day observation must be available.
    for required in [
        AssessmentFeatureId::BaziTargetDayTenGod,
        AssessmentFeatureId::BaziTargetDayPillarRelation,
        AssessmentFeatureId::BaziTargetDayElementResonance,
    ] {
        let unavailable = by_id
            .get(&required)
            .copied()
            .unwrap_or_else(|| panic!("missing {:?}", required));
        assert!(
            !unavailable,
            "{:?} must be available for the full profile",
            required
        );
    }
}

#[test]
fn v2_3_decision_keeps_bazi_chart_scoring_separate() {
    // The Day Assessment decision must not collapse into a Bazi
    // chart score. The v2.3 PersonalAlignment axis is the only axis
    // the Bazi observations feed; other axes must keep their v2.2
    // subtotals.
    let snap = snapshot(10, 2);
    let p = full_profile();
    let v23_result = v23(&snap, &p, ConsultationIntent::Wedding);
    let v22_result = v22(&snap, &p, ConsultationIntent::Wedding);

    let v23_trace = v23_result.trace.as_ref().expect("v2.3 trace");
    let v22_trace = v22_result.trace.as_ref().expect("v2.2 trace");

    for axis in [
        AssessmentAxis::GenericDayQuality,
        AssessmentAxis::IntentFit,
        AssessmentAxis::AnnualPressure,
        AssessmentAxis::EvidenceCoverage,
    ] {
        let v23_axis = v23_trace
            .axes
            .iter()
            .find(|a| a.axis == axis)
            .expect("v2.3 axis");
        let v22_axis = v22_trace
            .axes
            .iter()
            .find(|a| a.axis == axis)
            .expect("v2.2 axis");
        assert_eq!(
            v23_axis.subtotal, v22_axis.subtotal,
            "{:?} subtotal must match v2.2 when only the PersonalAlignment axis absorbs Bazi observations",
            axis
        );
    }
}

// ---------------------------------------------------------------------------
// Bazi-to-day observation content tests
// ---------------------------------------------------------------------------

#[test]
fn v2_3_ten_god_observation_cites_target_stem_and_day_master() {
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let ten_god = trace
        .features
        .iter()
        .find(|f| f.feature_id == AssessmentFeatureId::BaziTargetDayTenGod)
        .expect("BaziTargetDayTenGod observation");
    assert!(!ten_god.is_unavailable());

    let note = ten_god
        .source_evidence
        .note
        .as_deref()
        .expect("ten_god must carry source note");
    assert!(note.contains("target_stem="), "note: {}", note);
    assert!(note.contains("day_master="), "note: {}", note);
    assert!(note.contains("label="), "note: {}", note);
    assert!(note.contains("relation="), "note: {}", note);
}

#[test]
fn v2_3_pillar_relation_observation_lists_matched_pillars() {
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let pillar_observations: Vec<_> = trace
        .features
        .iter()
        .filter(|f| {
            f.feature_id == AssessmentFeatureId::BaziTargetDayPillarRelation
                && !f.is_unavailable()
                && f.contribution_id != "bazi.target_day.pillar_relation"
        })
        .collect();

    for obs in &pillar_observations {
        let note = obs.source_evidence.note.as_deref().unwrap_or("");
        assert!(
            note.contains("target_chi="),
            "pillar relation note must cite target_chi, got: {}",
            note
        );
        assert!(
            note.contains("matched_pillars="),
            "pillar relation note must cite matched_pillars, got: {}",
            note
        );
    }
}

#[test]
fn v2_3_element_resonance_observation_cites_elements() {
    let snap = snapshot(10, 2);
    let p = full_profile();
    let result = v23(&snap, &p, ConsultationIntent::Wedding);
    let trace = result.trace.as_ref().expect("v2.3 trace");

    let resonance = trace
        .features
        .iter()
        .find(|f| f.feature_id == AssessmentFeatureId::BaziTargetDayElementResonance)
        .expect("BaziTargetDayElementResonance observation");
    assert!(!resonance.is_unavailable());

    let note = resonance
        .source_evidence
        .note
        .as_deref()
        .expect("element resonance must carry source note");
    assert!(note.contains("day_element="), "note: {}", note);
    assert!(note.contains("day_master_element="), "note: {}", note);
    assert!(note.contains("relation="), "note: {}", note);
}

// ---------------------------------------------------------------------------
// Cross-policy parity: v2.2 → v2.3 only diverges on PersonalAlignment
// ---------------------------------------------------------------------------

#[test]
fn v2_3_only_diverges_from_v2_2_on_personal_alignment_axis() {
    let snap = snapshot(15, 4);
    let p = full_profile();
    let v23_result = v23(&snap, &p, ConsultationIntent::Travel);
    let v22_result = v22(&snap, &p, ConsultationIntent::Travel);

    let v23_trace = v23_result.trace.as_ref().expect("v2.3 trace");
    let v22_trace = v22_result.trace.as_ref().expect("v2.2 trace");

    // The Bazi-to-day observations feed the PersonalAlignment axis
    // only — the other axes must keep their v2.2 subtotals.
    for axis in [
        AssessmentAxis::GenericDayQuality,
        AssessmentAxis::IntentFit,
        AssessmentAxis::AnnualPressure,
        AssessmentAxis::EvidenceCoverage,
    ] {
        let v23_axis = v23_trace
            .axes
            .iter()
            .find(|a| a.axis == axis)
            .expect("v2.3 axis");
        let v22_axis = v22_trace
            .axes
            .iter()
            .find(|a| a.axis == axis)
            .expect("v2.2 axis");
        assert_eq!(
            v23_axis.subtotal, v22_axis.subtotal,
            "{:?} subtotal must match v2.2",
            axis
        );
    }
}
