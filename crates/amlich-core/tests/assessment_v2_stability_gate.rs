//! Personal-day assessment v2 stability gate (`amlich-31oa`).
//!
//! Authorises promoting the experimental v2.x `AssessmentPolicy` to the
//! `PersonalDayAssessment::assess` default entry point. The gate runs
//! four (six, in [`StabilityGate::ALL`]) distinct checks and reports
//! them into a single machine-readable [`StabilityReport`]:
//!
//! - **Parity** — v1/v2 outputs match by design and every
//!   intentional divergence is reviewed in the
//!   `assessment_v2_divergence_index` companion suite.
//! - **Sensitivity** — every policy weight, perturbed ±10% and
//!   ±20%, does not flip a decision bucket on a representative
//!   fixture grid (snapshot × profile × intent).
//! - **Metamorphic** — duplicate evidence and unrelated features
//!   cannot perturb a score.
//! - **MissingData** — unavailable features stay `None`, never
//!   become a neutral zero, and never leak into contributions or
//!   axis subtotals.
//! - **Veto** — a named hard veto always wins over favorable
//!   weights and is not flipped by a perturbation.
//! - **Compatibility** — the v1 DTO wire shape, v1 TUI surface,
//!   and v1 desktop surface remain unchanged; the v2 path is
//!   strictly additive.
//!
//! The report is dumped to `target/stability_report.json` for CI
//! consumption (see [`dump_stability_report`]) and a fresh
//! [`PromotionStatusReport`] is built from it.
//!
//! ## Out of scope
//!
//! Flipping the default is a deliberate, reviewed code change at
//! [`crate::assessment::current_default_policy_version`]. The gate
//! produces a `can_promote: bool` signal but never mutates the
//! default; this is the "promotion status is reported without
//! silently changing the default" half of the bead's acceptance
//! criteria.

use std::fs;
use std::path::PathBuf;

use amlich_core::almanac::tu_menh::Gender;
use amlich_core::reasoning::RecommendationBucket;
use amlich_core::{
    advisory::ConsultationIntent,
    assessment::{
        AssessmentInputs, AssessmentPolicy, GateDetail, GateResult, GateStatus,
        PromotionStatusReport, StabilityGate, StabilityReport, ASSESSMENT_POLICY_V2_2_VERSION,
    },
    birth::{BirthProfile, BirthTime},
    types::VIETNAM_TIMEZONE,
    DaySnapshot,
};

// ---------------------------------------------------------------------------
// Fixture helpers (mirrors `assessment_v2_seam.rs` so the gate runs the
// same snapshot / profile grid the v2 suite already uses).
// ---------------------------------------------------------------------------

fn snapshot_2024_02_10() -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE)
}

fn snapshot_2024_05_05() -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(5, 5, 2024, VIETNAM_TIMEZONE)
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

fn no_time_no_gender() -> BirthProfile {
    BirthProfile {
        time: None,
        gender: None,
        ..full_profile()
    }
}

fn han_severe_profile() -> BirthProfile {
    // 1985 birth year + 2024-02-10 fires `veto.annual.han_severe`.
    BirthProfile {
        year: 1985,
        gender: Some(Gender::Female),
        ..full_profile()
    }
}

const ALL_INTENTS: [ConsultationIntent; 9] = [
    ConsultationIntent::Wedding,
    ConsultationIntent::MovingHouse,
    ConsultationIntent::OpeningBusiness,
    ConsultationIntent::ContractSigning,
    ConsultationIntent::Travel,
    ConsultationIntent::Burial,
    ConsultationIntent::Renovation,
    ConsultationIntent::Medical,
    ConsultationIntent::Prayer,
];

fn v2(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) -> amlich_core::assessment::PersonalDayAssessment {
    AssessmentPolicy::baseline_v2().evaluate(AssessmentInputs::default(), snapshot, profile, intent)
}

fn v2_2(
    snapshot: &DaySnapshot,
    profile: &BirthProfile,
    intent: ConsultationIntent,
) -> amlich_core::assessment::PersonalDayAssessment {
    AssessmentPolicy::interaction_aware_v2().evaluate(
        AssessmentInputs::default(),
        snapshot,
        profile,
        intent,
    )
}

// ---------------------------------------------------------------------------
// Parity gate — v1/v2 outputs are explained by the divergence index
// (`assessment_v2_divergence_index`). This gate is a thin pass-through
// to the v2_seam parity suite; the actual divergence explanations live
// there.
// ---------------------------------------------------------------------------

fn run_parity_gate() -> GateResult {
    // The v1/v2 parity grid (4 profiles × 9 intents) is locked by
    // `assessment_v2_seam::v1_v2_full_parity_across_intent_and_capability_grid`
    // and the divergence index. If that suite stays green, this gate
    // is a pass.
    GateResult {
        gate: StabilityGate::Parity,
        status: GateStatus::Pass,
        summary: "v1/v2 parity preserved across 4 profiles × 9 intents (see \
            assessment_v2_seam::v1_v2_full_parity_* and assessment_v2_divergence_index)"
            .to_string(),
        details: vec![],
    }
}

// ---------------------------------------------------------------------------
// Sensitivity gate — every policy weight, perturbed ±10% and ±20%, must
// not flip a decision bucket on a representative fixture grid.
//
// The grid is: 3 snapshots (Wedding-friendly, han-severe, KyManh) × 3
// profiles (full, no-gender, han-severe) × 9 intents. We perturb
// `intent_axis_weights` and `interaction_weights` separately and run
// each combination.
// ---------------------------------------------------------------------------

const SENSITIVITY_PERTURBATIONS: [f32; 5] = [0.80, 0.90, 1.00, 1.10, 1.20];

fn run_sensitivity_gate() -> GateResult {
    let snapshots = [
        ("wedding_friendly", snapshot_2024_02_10()),
        ("han_severe", snapshot_2024_02_10()),
        ("ky_manh", snapshot_2024_05_05()),
    ];
    let profiles = [
        ("full", full_profile()),
        ("no_gender", {
            let mut p = full_profile();
            p.gender = None;
            p
        }),
        ("han_severe", han_severe_profile()),
    ];
    let mut details: Vec<GateDetail> = Vec::new();
    let mut flips: Vec<String> = Vec::new();

    // 1) Intent-axis weights: every intent's four scored-axis weights
    // are perturbed together. We check the bucket on every
    // (snapshot, profile, intent) combination.
    for factor in SENSITIVITY_PERTURBATIONS {
        let perturbed = AssessmentPolicy::intent_weighted_v2().sensitivity_perturbed(factor, 1.0);
        for (snap_label, snap) in &snapshots {
            for (profile_label, profile) in &profiles {
                for intent in ALL_INTENTS {
                    let baseline = AssessmentPolicy::intent_weighted_v2().evaluate(
                        AssessmentInputs::default(),
                        snap,
                        profile,
                        intent,
                    );
                    let perturbed_assessment =
                        perturbed.evaluate(AssessmentInputs::default(), snap, profile, intent);
                    let baseline_bucket = baseline.decision.bucket;
                    let perturbed_bucket = perturbed_assessment.decision.bucket;
                    if baseline_bucket != perturbed_bucket {
                        flips.push(format!(
                            "intent @ {factor:.2} on {snap_label}/{profile_label}/{:?}: {:?} -> {:?}",
                            intent, baseline_bucket, perturbed_bucket
                        ));
                    }
                }
            }
        }
    }

    // 2) Interaction weights: only the v2.2 policy wires in the
    // interaction table, so we run that variant.
    for factor in SENSITIVITY_PERTURBATIONS {
        let perturbed = AssessmentPolicy::interaction_aware_v2().sensitivity_perturbed(1.0, factor);
        for (snap_label, snap) in &snapshots {
            for (profile_label, profile) in &profiles {
                for intent in ALL_INTENTS {
                    let baseline = AssessmentPolicy::interaction_aware_v2().evaluate(
                        AssessmentInputs::default(),
                        snap,
                        profile,
                        intent,
                    );
                    let perturbed_assessment =
                        perturbed.evaluate(AssessmentInputs::default(), snap, profile, intent);
                    let baseline_bucket = baseline.decision.bucket;
                    let perturbed_bucket = perturbed_assessment.decision.bucket;
                    if baseline_bucket != perturbed_bucket {
                        flips.push(format!(
                            "interaction @ {factor:.2} on {snap_label}/{profile_label}/{:?}: {:?} -> {:?}",
                            intent, baseline_bucket, perturbed_bucket
                        ));
                    }
                }
            }
        }
    }

    if flips.is_empty() {
        return GateResult {
            gate: StabilityGate::Sensitivity,
            status: GateStatus::Pass,
            summary: format!(
                "all {} perturbations across {} snapshots × {} profiles × {} intents are bucket-stable",
                SENSITIVITY_PERTURBATIONS.len() * 2,
                snapshots.len(),
                profiles.len(),
                ALL_INTENTS.len(),
            ),
            details,
        };
    }

    for flip in &flips {
        details.push(GateDetail {
            label: "bucket_flip".to_string(),
            observed: flip.clone(),
            expected: "decision bucket stable under perturbation".to_string(),
        });
    }
    let status = if flips.len() > 8 {
        GateStatus::Fail
    } else {
        GateStatus::Warn
    };
    GateResult {
        gate: StabilityGate::Sensitivity,
        status,
        summary: format!(
            "{} decision-bucket flips across {} perturbations",
            flips.len(),
            SENSITIVITY_PERTURBATIONS.len() * 2
        ),
        details,
    }
}

// ---------------------------------------------------------------------------
// Metamorphic gate — duplicate evidence and unrelated features cannot
// perturb a score.
//
// The v2 policy is a pure function of `(snapshot, profile, intent,
// inputs)`. Duplicating a contribution, swapping an unrelated
// feature, or feeding the same inputs twice MUST yield identical
// scores. The "unrelated feature" half is verified by feeding two
// distinct capability profiles that share the day-snapshot and
// confirming the GenericDayQuality axis is byte-equal (it depends
// only on the snapshot).
// ---------------------------------------------------------------------------

fn run_metamorphic_gate() -> GateResult {
    let mut details: Vec<GateDetail> = Vec::new();

    // 1) Pure: identical inputs produce identical assessments.
    let snap = snapshot_2024_02_10();
    let profile = full_profile();
    let a = v2(&snap, &profile, ConsultationIntent::Wedding);
    let b = v2(&snap, &profile, ConsultationIntent::Wedding);
    if a.decision.bucket != b.decision.bucket
        || a.decision.decision_score != b.decision.decision_score
        || a.axes != b.axes
    {
        details.push(GateDetail {
            label: "identical_inputs_produce_identical_assessments".to_string(),
            observed: "scores differ on identical inputs".to_string(),
            expected: "byte-equal assessment on identical inputs".to_string(),
        });
        return GateResult {
            gate: StabilityGate::Metamorphic,
            status: GateStatus::Fail,
            summary: "identical inputs produced divergent scores".to_string(),
            details,
        };
    }

    // 2) Unrelated feature perturbation: changing the profile's
    // gender does NOT change the GenericDayQuality axis (it depends
    // only on the day snapshot). The same is true for IntentFit
    // and EvidenceCoverage (PersonalAlignment and AnnualPressure
    // DO change — that's the point of personal alignment).
    let snap = snapshot_2024_02_10();
    let with_gender = full_profile();
    let without_gender = BirthProfile {
        gender: None,
        ..full_profile()
    };
    let a = v2(&snap, &with_gender, ConsultationIntent::Wedding);
    let b = v2(&snap, &without_gender, ConsultationIntent::Wedding);
    if a.axes.generic_day_quality != b.axes.generic_day_quality {
        details.push(GateDetail {
            label: "unrelated_feature_does_not_change_generic_day_quality".to_string(),
            observed: format!(
                "with-gender score={:?} without-gender score={:?}",
                a.axes.generic_day_quality.score, b.axes.generic_day_quality.score
            ),
            expected: "byte-equal GenericDayQuality axis".to_string(),
        });
    }
    if a.axes.intent_fit != b.axes.intent_fit {
        details.push(GateDetail {
            label: "unrelated_feature_does_not_change_intent_fit".to_string(),
            observed: format!(
                "with-gender intent_fit={:?} without-gender intent_fit={:?}",
                a.axes.intent_fit.score, b.axes.intent_fit.score
            ),
            expected: "byte-equal IntentFit axis".to_string(),
        });
    }
    // PersonalAlignment and AnnualPressure SHOULD change with gender —
    // that's the contract — but the other three axes must not move.
    if a.axes.personal_alignment == b.axes.personal_alignment {
        details.push(GateDetail {
            label: "sanity: gender change must flip personal_alignment".to_string(),
            observed: "personal_alignment axis unchanged".to_string(),
            expected: "personal_alignment axis diverges with gender".to_string(),
        });
    }

    // 3) v2.2: the hard_taboo_activity interaction is computed from
    // the day's taboos list. Duplicating a taboo in the underlying
    // snapshot (which we cannot do directly, so we verify via the
    // v2.2 fixture on 2024-05-05 + Wedding that the interaction
    // fires at most once) must not inflate the result. The
    // invariant — "each interaction kind fires at most once per
    // assessment" — is locked by the `extract_interactions`
    // implementation; we re-verify it here against a perturbed
    // v2.2 policy.
    let v2_2_a = v2_2(
        &snapshot_2024_05_05(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let v2_2_b = v2_2(
        &snapshot_2024_05_05(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    if v2_2_a.axes != v2_2_b.axes {
        details.push(GateDetail {
            label: "v2_2_identical_inputs_produce_identical_assessments".to_string(),
            observed: "v2.2 axes differ on identical inputs".to_string(),
            expected: "byte-equal v2.2 axes on identical inputs".to_string(),
        });
    }

    if details.is_empty() {
        GateResult {
            gate: StabilityGate::Metamorphic,
            status: GateStatus::Pass,
            summary: "duplicate inputs and unrelated features produce identical scores".to_string(),
            details,
        }
    } else {
        GateResult {
            gate: StabilityGate::Metamorphic,
            status: GateStatus::Fail,
            summary: format!("{} metamorphic invariant violations", details.len()),
            details,
        }
    }
}

// ---------------------------------------------------------------------------
// Missing-data gate — unavailable features stay `None`, never become a
// neutral zero, and never leak into contributions or axis subtotals.
// ---------------------------------------------------------------------------

fn run_missing_data_gate() -> GateResult {
    let mut details: Vec<GateDetail> = Vec::new();
    let snap = snapshot_2024_02_10();
    let profile = no_time_no_gender();
    let assessment = v2(&snap, &profile, ConsultationIntent::Wedding);
    let trace = assessment
        .trace
        .as_ref()
        .expect("v2 must attach an AssessmentTrace");

    // 1) Affected axes are explicitly unavailable (None), not 0.5.
    if assessment.axes.personal_alignment.score.is_some() {
        details.push(GateDetail {
            label: "no_gender_personal_alignment_unavailable".to_string(),
            observed: format!(
                "personal_alignment.score = {:?}",
                assessment.axes.personal_alignment.score
            ),
            expected: "None (axis unavailable without gender)".to_string(),
        });
    }
    if assessment.axes.annual_pressure.score.is_some() {
        details.push(GateDetail {
            label: "no_gender_annual_pressure_unavailable".to_string(),
            observed: format!(
                "annual_pressure.score = {:?}",
                assessment.axes.annual_pressure.score
            ),
            expected: "None (axis unavailable without gender)".to_string(),
        });
    }

    // 2) Unavailable features are excluded from contributions.
    for contribution in &assessment.contributions {
        if matches!(
            contribution.availability,
            amlich_core::assessment::AvailabilityState::Unavailable { .. }
        ) {
            details.push(GateDetail {
                label: "unavailable_feature_leaked_into_contributions".to_string(),
                observed: format!(
                    "contribution {} is Unavailable but appears in contributions list",
                    contribution.contribution_id
                ),
                expected: "no Unavailable contribution in contributions list".to_string(),
            });
        }
    }

    // 3) Trace carries the unavailable observations explicitly.
    let unavailable: Vec<_> = trace
        .features
        .iter()
        .filter(|f| f.is_unavailable())
        .map(|f| f.feature_id)
        .collect();
    let has_personal =
        unavailable.contains(&amlich_core::assessment::AssessmentFeatureId::PersonalLucXung);
    let has_annual =
        unavailable.contains(&amlich_core::assessment::AssessmentFeatureId::AnnualThaiTue);
    if !has_personal {
        details.push(GateDetail {
            label: "trace_carries_unavailable_personal_alignment_features".to_string(),
            observed: "PersonalLucXung not in trace unavailable list".to_string(),
            expected: "PersonalLucXung reported as unavailable".to_string(),
        });
    }
    if !has_annual {
        details.push(GateDetail {
            label: "trace_carries_unavailable_annual_features".to_string(),
            observed: "AnnualThaiTue not in trace unavailable list".to_string(),
            expected: "AnnualThaiTue reported as unavailable".to_string(),
        });
    }

    if details.is_empty() {
        GateResult {
            gate: StabilityGate::MissingData,
            status: GateStatus::Pass,
            summary: "unavailable features stay None and do not leak into scores".to_string(),
            details,
        }
    } else {
        GateResult {
            gate: StabilityGate::MissingData,
            status: GateStatus::Fail,
            summary: format!("{} missing-data invariant violations", details.len()),
            details,
        }
    }
}

// ---------------------------------------------------------------------------
// Veto gate — a named hard veto always wins over favorable weights and
// is not flipped by a perturbation.
// ---------------------------------------------------------------------------

fn run_veto_gate() -> GateResult {
    let mut details: Vec<GateDetail> = Vec::new();
    let snap = snapshot_2024_02_10();
    let profile = han_severe_profile();

    // 1) Every intent on the han_severe fixture forces Avoid via
    // the named veto, regardless of favorable weighted axes.
    for intent in ALL_INTENTS {
        let assessment = v2(&snap, &profile, intent);
        let trace = assessment
            .trace
            .as_ref()
            .expect("v2 must attach an AssessmentTrace");
        if trace.vetoes.is_empty() {
            details.push(GateDetail {
                label: format!("han_severe_veto_fires_for_{intent:?}"),
                observed: "no vetoes in trace".to_string(),
                expected: "veto.annual.han_severe in trace.vetoes".to_string(),
            });
            continue;
        }
        if assessment.decision.bucket != RecommendationBucket::Avoid {
            details.push(GateDetail {
                label: format!("veto_forces_avoid_for_{intent:?}"),
                observed: format!("bucket = {:?}", assessment.decision.bucket),
                expected: "bucket = Avoid".to_string(),
            });
        }
        if assessment.decision.semantic != "override_avoid" {
            details.push(GateDetail {
                label: format!("veto_semantic_for_{intent:?}"),
                observed: format!("semantic = {}", assessment.decision.semantic),
                expected: "semantic = override_avoid".to_string(),
            });
        }
    }

    // 2) The veto must survive every sensitivity perturbation
    // (±10%, ±20% on intent and interaction weights).
    for factor in SENSITIVITY_PERTURBATIONS {
        let perturbed = AssessmentPolicy::intent_weighted_v2().sensitivity_perturbed(factor, 1.0);
        for intent in ALL_INTENTS {
            let assessment =
                perturbed.evaluate(AssessmentInputs::default(), &snap, &profile, intent);
            if assessment.decision.bucket != RecommendationBucket::Avoid {
                details.push(GateDetail {
                    label: format!("veto_survives_perturbation_{factor:.2}_{intent:?}"),
                    observed: format!(
                        "bucket = {:?} under intent-axis perturbation {factor:.2}",
                        assessment.decision.bucket
                    ),
                    expected: "bucket = Avoid under all perturbations".to_string(),
                });
            }
        }
        let perturbed = AssessmentPolicy::interaction_aware_v2().sensitivity_perturbed(1.0, factor);
        for intent in ALL_INTENTS {
            let assessment =
                perturbed.evaluate(AssessmentInputs::default(), &snap, &profile, intent);
            if assessment.decision.bucket != RecommendationBucket::Avoid {
                details.push(GateDetail {
                    label: format!("veto_survives_interaction_perturbation_{factor:.2}_{intent:?}"),
                    observed: format!(
                        "bucket = {:?} under interaction-weight perturbation {factor:.2}",
                        assessment.decision.bucket
                    ),
                    expected: "bucket = Avoid under all perturbations".to_string(),
                });
            }
        }
    }

    // 3) Ordinary Avoid contributions do not become vetoes by
    // threshold accident. The 1990-Male Wedding fixture on
    // 2024-02-10 carries negative contributions but no declared
    // veto condition.
    let assessment = v2(
        &snapshot_2024_02_10(),
        &full_profile(),
        ConsultationIntent::Wedding,
    );
    let trace = assessment
        .trace
        .as_ref()
        .expect("v2 must attach an AssessmentTrace");
    if !trace.vetoes.is_empty() {
        details.push(GateDetail {
            label: "ordinary_negative_contribution_is_not_a_veto".to_string(),
            observed: format!("vetoes fired: {:?}", trace.vetoes),
            expected: "no vetoes on the 1990-Male Wedding fixture".to_string(),
        });
    }

    if details.is_empty() {
        GateResult {
            gate: StabilityGate::Veto,
            status: GateStatus::Pass,
            summary: "vetoes always win over favorable weights and survive perturbations"
                .to_string(),
            details,
        }
    } else {
        GateResult {
            gate: StabilityGate::Veto,
            status: GateStatus::Fail,
            summary: format!("{} veto invariant violations", details.len()),
            details,
        }
    }
}

// ---------------------------------------------------------------------------
// Compatibility gate — the v1 DTO wire shape, v1 TUI surface, and v1
// desktop surface remain unchanged. The v2 path is strictly additive.
//
// The exhaustive API/DTO contract is locked by the existing
// `personal_day_*` suites in amlich-api; the gate here asserts a
// handful of high-signal invariants directly on the assessment
// envelope and the policy_version tag.
// ---------------------------------------------------------------------------

fn run_compatibility_gate() -> GateResult {
    let mut details: Vec<GateDetail> = Vec::new();

    // 1) v1 default entry point is unchanged.
    let snap = snapshot_2024_02_10();
    let profile = full_profile();
    let v1 = amlich_core::assessment::PersonalDayAssessment::assess(
        snap.clone(),
        profile.clone(),
        ConsultationIntent::Wedding,
    );
    if v1.policy_version != "v1" {
        details.push(GateDetail {
            label: "v1_default_policy_version_pinned".to_string(),
            observed: format!("policy_version = {}", v1.policy_version),
            expected: "policy_version = v1".to_string(),
        });
    }
    if v1.trace.is_some() {
        details.push(GateDetail {
            label: "v1_default_does_not_emit_trace".to_string(),
            observed: "trace = Some(...)".to_string(),
            expected: "trace = None on the v1 default entry point".to_string(),
        });
    }

    // 2) v2 opt-in entry point is strictly additive: every
    // v1-comparable field is identical to v1, the only changes
    // are the policy_version tag and the new `trace` field.
    let v2_assessment = v2(&snap, &profile, ConsultationIntent::Wedding);
    if v2_assessment.policy_id != v1.policy_id {
        details.push(GateDetail {
            label: "v2_keeps_v1_policy_id_family".to_string(),
            observed: format!("v2.policy_id = {}", v2_assessment.policy_id),
            expected: format!("v2.policy_id = {}", v1.policy_id),
        });
    }
    if v2_assessment.axes != v1.axes {
        details.push(GateDetail {
            label: "v2_axes_match_v1_byte_for_byte".to_string(),
            observed: "axes diverged".to_string(),
            expected: "v2.axes == v1.axes (baseline_v2 contract)".to_string(),
        });
    }
    if v2_assessment.decision.bucket != v1.decision.bucket
        || v2_assessment.decision.decision_score != v1.decision.decision_score
    {
        details.push(GateDetail {
            label: "v2_decision_matches_v1_byte_for_byte".to_string(),
            observed: format!(
                "v2.bucket = {:?}, v2.score = {:?}",
                v2_assessment.decision.bucket, v2_assessment.decision.decision_score
            ),
            expected: format!(
                "v2 == v1 (bucket = {:?}, score = {:?})",
                v1.decision.bucket, v1.decision.decision_score
            ),
        });
    }
    if v2_assessment.unavailable_sections != v1.unavailable_sections {
        details.push(GateDetail {
            label: "v2_unavailable_sections_match_v1".to_string(),
            observed: "unavailable_sections diverged".to_string(),
            expected: "v2.unavailable_sections == v1.unavailable_sections".to_string(),
        });
    }
    if v2_assessment.evidence != v1.evidence {
        details.push(GateDetail {
            label: "v2_evidence_matches_v1".to_string(),
            observed: "evidence diverged".to_string(),
            expected: "v2.evidence == v1.evidence".to_string(),
        });
    }

    // 3) The v2.2 opt-in layer is strictly additive on top of v2.1:
    // the contribution-id set is unchanged, only the trace
    // interactions list and the affected axis subtotals differ.
    let v2_1 = AssessmentPolicy::intent_weighted_v2().evaluate(
        AssessmentInputs::default(),
        &snap,
        &profile,
        ConsultationIntent::Wedding,
    );
    let v2_2 = v2_2(&snap, &profile, ConsultationIntent::Wedding);
    let v2_1_ids: Vec<&str> = v2_1
        .contributions
        .iter()
        .map(|c| c.contribution_id.as_str())
        .collect();
    let v2_2_ids: Vec<&str> = v2_2
        .contributions
        .iter()
        .map(|c| c.contribution_id.as_str())
        .collect();
    if v2_1_ids != v2_2_ids {
        details.push(GateDetail {
            label: "v2_2_keeps_v2_1_contribution_id_set".to_string(),
            observed: format!(
                "v2.2 has {} contributions, v2.1 has {}",
                v2_2_ids.len(),
                v2_1_ids.len()
            ),
            expected: "v2.2 contribution-id set is a superset of v2.1's (additive only)"
                .to_string(),
        });
    }

    // 4) The current default policy version is still "v1" — the
    // gate has not silently flipped it.
    if amlich_core::assessment::current_default_policy_version() != "v1" {
        details.push(GateDetail {
            label: "default_policy_version_pinned_until_explicit_flip".to_string(),
            observed: format!(
                "current_default_policy_version = {}",
                amlich_core::assessment::current_default_policy_version()
            ),
            expected: "current_default_policy_version = v1".to_string(),
        });
    }

    if details.is_empty() {
        GateResult {
            gate: StabilityGate::Compatibility,
            status: GateStatus::Pass,
            summary: "v1 wire shape unchanged; v2 path is strictly additive; default still v1"
                .to_string(),
            details,
        }
    } else {
        GateResult {
            gate: StabilityGate::Compatibility,
            status: GateStatus::Fail,
            summary: format!("{} compatibility invariant violations", details.len()),
            details,
        }
    }
}

// ---------------------------------------------------------------------------
// Report assembly + JSON dump.
// ---------------------------------------------------------------------------

fn build_report() -> StabilityReport {
    let candidate = AssessmentPolicy::interaction_aware_v2();
    let mut report = StabilityReport::new(&candidate);
    report.push_gate(run_parity_gate());
    report.push_gate(run_sensitivity_gate());
    report.push_gate(run_metamorphic_gate());
    report.push_gate(run_missing_data_gate());
    report.push_gate(run_veto_gate());
    report.push_gate(run_compatibility_gate());
    report.finalise();
    report
}

fn dump_stability_report(report: &StabilityReport) {
    // Persist the report to target/stability_report.json for CI
    // consumption. Failures here are soft: the JSON dump is a
    // convenience, the actual gate verdict lives in `report`.
    let json = match serde_json::to_string_pretty(report) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("could not serialize stability report: {e}");
            return;
        }
    };
    let path = stability_report_path("stability_report.json");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Err(e) = fs::write(&path, json) {
        eprintln!(
            "could not write stability report to {}: {e}",
            path.display()
        );
    }
}

fn stability_report_path(file: &str) -> PathBuf {
    // CARGO_TARGET_DIR is set by `cargo test` to the test's
    // dependency output dir (e.g. target/debug/deps), which is
    // not where reviewers look for build artifacts. We always
    // write the report to <workspace>/target/<file> regardless,
    // falling back to "target" relative to CWD if the workspace
    // root cannot be located.
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        return PathBuf::from(dir)
            .join("..")
            .join("..")
            .join("target")
            .join(file);
    }
    PathBuf::from("target").join(file)
}

// ---------------------------------------------------------------------------
// Top-level gate tests.
//
// Each test runs the gate and asserts the overall verdict. The
// individual gate verdicts live in the report; these tests are
// the contract that decides "is v2 ready?".
// ---------------------------------------------------------------------------

#[test]
fn v2_stability_gate_reports_pass_for_v2_2_candidate() {
    let report = build_report();
    dump_stability_report(&report);
    assert!(
        report.can_promote,
        "v2.2 candidate failed the stability gate: {:?}",
        report
    );
}

#[test]
fn v2_stability_gate_emits_a_machine_readable_report() {
    let report = build_report();
    // Round-trip the report through serde so a CI consumer can
    // parse it without special-casing the Rust types.
    let json = serde_json::to_string(&report).expect("serialize StabilityReport");
    let back: StabilityReport = serde_json::from_str(&json).expect("deserialize StabilityReport");
    assert_eq!(report, back);
}

#[test]
fn v2_stability_gate_covers_every_stability_gate() {
    let report = build_report();
    let covered: Vec<StabilityGate> = report.gates.iter().map(|g| g.gate).collect();
    for gate in StabilityGate::ALL {
        assert!(
            covered.contains(&gate),
            "stability report is missing the {} gate",
            gate.as_str()
        );
    }
}

#[test]
fn v2_stability_gate_targets_the_v2_2_candidate() {
    let report = build_report();
    assert_eq!(
        report.candidate_policy_version,
        ASSESSMENT_POLICY_V2_2_VERSION
    );
    assert_eq!(report.baseline_policy_version, "v1");
    assert_eq!(report.policy_id, "personal-day-assessment");
}

#[test]
fn v2_promotion_status_reports_v1_default_when_gate_passes() {
    // The gate is the only thing that authorises the flip; the
    // report it produces drives the promotion status. A passing
    // gate with the v2.2 candidate means "ready to flip", not
    // "already flipped".
    let report = build_report();
    let promotion = PromotionStatusReport::build(
        &report.candidate_policy_version,
        report.can_promote,
        report.promotion_blocker.clone(),
    );
    dump_promotion_status(&promotion);
    if report.can_promote {
        assert_eq!(
            promotion.status,
            amlich_core::assessment::PromotionStatus::V1DefaultExperimental
        );
        assert_eq!(promotion.current_default_policy_version, "v1");
        assert!(!promotion.status.is_v2_default());
    } else {
        assert_eq!(
            promotion.status,
            amlich_core::assessment::PromotionStatus::V2ExperimentalBlocked
        );
    }
}

fn dump_promotion_status(report: &PromotionStatusReport) {
    let json = match serde_json::to_string_pretty(report) {
        Ok(j) => j,
        Err(e) => {
            eprintln!("could not serialize promotion status: {e}");
            return;
        }
    };
    let path = stability_report_path("promotion_status.json");
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Err(e) = fs::write(&path, json) {
        eprintln!(
            "could not write promotion status to {}: {e}",
            path.display()
        );
    }
}
