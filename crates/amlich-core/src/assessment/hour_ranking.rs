//! First-class hour-ranking policy.
//!
//! Source spec: `docs/architecture/personal-day-audit/HOUR-RANKING-POLICY-V1-SPEC.md`
//! Bead: `amlich-rv13.1` (parent epic: `amlich-rv13`).
//! ADR: `docs/adr/0001-separate-day-and-hour-scoring-axes.md`.
//!
//! Hour ranking orders the twelve traditional hour slots within an
//! already-assessed day. It does **not** decide day suitability and does
//! **not** introduce an hour-level verdict bucket — the day assessment
//! verdict is authoritative on whether the day itself is usable.
//!
//! ## Why a separate policy
//!
//! Per ADR-0001, day assessment and hour ranking answer different domain
//! questions: day suitability vs hour ordering within a usable day. They
//! share the *mechanics* (feature observations, availability handling,
//! weighted aggregation, contribution/evidence trace) but use separate
//! feature IDs and vector axes. A high-ranked hour on an `Avoid` day must
//! not flip the day verdict, and the ranking must surface a warning
//! context so consumers don't present the hours as a day-override.
//!
//! ## Vector and weights (v1)
//!
//! ```text
//! hoang_dao_quality        0.45
//! intent_timing_fit        0.25
//! personal_hour_alignment  0.20
//! day_hour_harmony         0.10
//! ```
//!
//! When axes are unavailable, the denominator is the sum of available
//! weights only:
//!
//! ```text
//! rank_score = Σ(axis_score × axis_weight) / Σ(available_axis_weights)
//! ```
//!
//! Scores are clamped to `0.0..=1.0`. Exact ties break by traditional Chi
//! order (`chi_index` ascending) so two hours with identical scores are
//! always ordered the same way for a given day.
//!
//! ## Output contract
//!
//! Every ranked hour carries:
//!
//! - normalized `rank_score` in `0.0..=1.0`
//! - `is_auspicious` (Hoàng Đạo membership)
//! - per-axis outcomes with explicit `unavailable` state
//! - one contribution per available axis with full source evidence
//! - `chi_index`, `chi_name`, `time_range`
//! - `warning_context` when the surrounding day assessment is `Avoid`
//!
//! There is no hour-level suitability bucket. Consumers must read the
//! canonical day verdict off `PersonalDayAssessment` and use this ranking
//! only to pick among hours the day verdict permits.

use serde::{Deserialize, Serialize};

use crate::{
    advisory::{BirthInput, ConsultationIntent},
    almanac::xung_hop,
    assessment::{AvailabilityState, PersonalDayAssessment, SourceEvidence},
    reasoning::RecommendationBucket,
    sources::SOURCE_KHCBPPT,
    types::CHI,
    DaySnapshot,
};

/// Stable policy identifier for the v1 hour-ranking policy. Co-versioned
/// with [`HOUR_RANKING_POLICY_V1_VERSION`]: any change to axes, weights,
/// or aggregation MUST bump the version.
pub const HOUR_RANKING_POLICY_V1_ID: &str = "hour-ranking";

/// Current version of the hour-ranking baseline policy. v1 introduces the
/// four-axis vector (`hoang_dao_quality`, `intent_timing_fit`,
/// `personal_hour_alignment`, `day_hour_harmony`) with separate hour
/// feature IDs from `PersonalDayAssessment`.
pub const HOUR_RANKING_POLICY_V1_VERSION: &str = "v1";

/// Four semantic axes that order the twelve traditional hour slots in v1.
/// Axes are intentionally separate from `AssessmentAxis` so day and hour
/// keep distinct domain vocabulary (ADR-0001).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HourRankingAxis {
    HoangDaoQuality,
    IntentTimingFit,
    PersonalHourAlignment,
    DayHourHarmony,
}

impl HourRankingAxis {
    /// All axes in canonical declaration order. Stable across policy
    /// versions; serialized traces and parity fixtures rely on this order.
    pub const ALL: [Self; 4] = [
        Self::HoangDaoQuality,
        Self::IntentTimingFit,
        Self::PersonalHourAlignment,
        Self::DayHourHarmony,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::HoangDaoQuality => "hoang_dao_quality",
            Self::IntentTimingFit => "intent_timing_fit",
            Self::PersonalHourAlignment => "personal_hour_alignment",
            Self::DayHourHarmony => "day_hour_harmony",
        }
    }
}

/// Per-axis weight entry used by the v1 aggregation. Versioned by the
/// owning policy; future variants may swap the table without changing
/// call sites.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct HourRankingAxisWeightEntry {
    pub axis: HourRankingAxis,
    pub weight: f32,
}

/// Outcome for one axis on one hour slot. `score == None` means the axis
/// was unavailable (the spec contract: unavailable ≠ zero).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourRankingAxisOutcome {
    pub axis: HourRankingAxis,
    pub score: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
}

impl HourRankingAxisOutcome {
    /// Build an unavailable outcome with a human-readable reason.
    pub fn unavailable(axis: HourRankingAxis, reason: &str) -> Self {
        Self {
            axis,
            score: None,
            unavailable_reason: Some(reason.to_string()),
        }
    }

    /// Build a scored outcome; the score is clamped to `0.0..=1.0`.
    pub fn from_score(axis: HourRankingAxis, score: f32) -> Self {
        Self {
            axis,
            score: Some(score.clamp(0.0, 1.0)),
            unavailable_reason: None,
        }
    }

    pub fn is_available(&self) -> bool {
        self.score.is_some()
    }
}

/// One axis's contribution to a hour slot's rank score, with full source
/// evidence. Mirrors the contribution shape used by the personal-day
/// policy (`assessment::trace::AxisContributor`) but is hour-specific and
/// does not share day-level feature IDs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourRankingContribution {
    pub axis: HourRankingAxis,
    pub score: f32,
    pub weight: f32,
    /// `score * weight` — the actual delta this axis contributed to the
    /// weighted numerator for this hour.
    pub contribution: f32,
    pub source_evidence: SourceEvidence,
    pub availability: AvailabilityState,
}

/// Warning context attached when the surrounding day assessment is
/// `Avoid`. Consumers must surface this rather than treating the
/// top-ranked hour as a recommendation that overrides the day verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HourRankingWarning {
    pub day_bucket: RecommendationBucket,
    pub message_vi: String,
}

/// All four axis outcomes for one hour slot, in canonical axis order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourRankingAxes {
    pub hoang_dao_quality: HourRankingAxisOutcome,
    pub intent_timing_fit: HourRankingAxisOutcome,
    pub personal_hour_alignment: HourRankingAxisOutcome,
    pub day_hour_harmony: HourRankingAxisOutcome,
}

impl HourRankingAxes {
    pub fn iter(&self) -> impl Iterator<Item = &HourRankingAxisOutcome> {
        [
            &self.hoang_dao_quality,
            &self.intent_timing_fit,
            &self.personal_hour_alignment,
            &self.day_hour_harmony,
        ]
        .into_iter()
    }
}

/// One ranked hour slot. The final output of the policy. There is no
/// hour-level suitability bucket — consumers read the day verdict off
/// [`PersonalDayAssessment::decision`] and use this struct only to order
/// hour slots within that verdict.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankedHourV1 {
    pub chi_index: usize,
    pub chi_name: String,
    pub time_range: String,
    pub is_auspicious: bool,
    pub rank_score: f32,
    pub axes: HourRankingAxes,
    pub contributions: Vec<HourRankingContribution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_context: Option<HourRankingWarning>,
}

/// Versioned policy that owns the v1 hour ranking pipeline.
///
/// Construct via [`HourRankingPolicy::baseline_v1`] for the initial
/// weighted-average policy. The policy is deterministic: identical
/// `(policy, snapshot, intent, birth, day_assessment)` quintuples produce
/// identical `Vec<RankedHourV1>`.
#[derive(Debug, Clone)]
pub struct HourRankingPolicy {
    policy_id: String,
    policy_version: String,
    axis_weights: [HourRankingAxisWeightEntry; 4],
}

impl Default for HourRankingPolicy {
    fn default() -> Self {
        Self::baseline_v1()
    }
}

impl HourRankingPolicy {
    /// Baseline v1 policy: the four-axis weighted aggregation from the
    /// spec's "Initial weight profile" section. Future variants (e.g.
    /// intent-specific weight tables) will land as additional
    /// constructors on this struct under new policy versions.
    pub fn baseline_v1() -> Self {
        Self {
            policy_id: HOUR_RANKING_POLICY_V1_ID.to_string(),
            policy_version: HOUR_RANKING_POLICY_V1_VERSION.to_string(),
            axis_weights: [
                HourRankingAxisWeightEntry {
                    axis: HourRankingAxis::HoangDaoQuality,
                    weight: 0.45,
                },
                HourRankingAxisWeightEntry {
                    axis: HourRankingAxis::IntentTimingFit,
                    weight: 0.25,
                },
                HourRankingAxisWeightEntry {
                    axis: HourRankingAxis::PersonalHourAlignment,
                    weight: 0.20,
                },
                HourRankingAxisWeightEntry {
                    axis: HourRankingAxis::DayHourHarmony,
                    weight: 0.10,
                },
            ],
        }
    }

    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn policy_version(&self) -> &str {
        &self.policy_version
    }

    /// Axis weights declared by this policy. Stable order matches
    /// [`HourRankingAxis::ALL`].
    pub fn axis_weights(&self) -> &[HourRankingAxisWeightEntry; 4] {
        &self.axis_weights
    }

    /// Order all twelve hour slots for the supplied snapshot. Pure and
    /// deterministic. `day_assessment` is optional — snapshot-only
    /// callers may pass `None`, but callers that already built a
    /// canonical day assessment should pass it so the `Avoid`-day
    /// warning context is attached.
    ///
    /// Returns `Err` if the snapshot carries fewer than twelve hour
    /// slots (the policy's mandatory candidate set per spec §"Candidate
    /// set"). Under normal snapshot construction this branch is
    /// unreachable; the explicit guard keeps the policy honest against
    /// malformed inputs.
    pub fn rank(
        &self,
        snapshot: &DaySnapshot,
        intent: ConsultationIntent,
        birth: Option<&BirthInput>,
        day_assessment: Option<&PersonalDayAssessment>,
    ) -> Result<Vec<RankedHourV1>, String> {
        let hoang_dao = &snapshot.context.gio_hoang_dao;
        if hoang_dao.all_hours.len() != 12 {
            return Err(format!(
                "hour-ranking requires twelve candidate slots; snapshot provides {}",
                hoang_dao.all_hours.len()
            ));
        }

        let day_chi_index = hoang_dao.day_chi_index;
        let profile_id = snapshot.profile.clone();
        let is_avoid_day = matches!(
            day_assessment.map(|a| a.decision.bucket),
            Some(RecommendationBucket::Avoid)
        );

        let mut ranked: Vec<RankedHourV1> = Vec::with_capacity(12);
        for hour in &hoang_dao.all_hours {
            let hour_chi_index = hour.hour_index;

            // Axis 1 — Hoàng Đạo quality (binary in v1 per spec).
            let hoang_dao_outcome = HourRankingAxisOutcome::from_score(
                HourRankingAxis::HoangDaoQuality,
                if hour.is_good { 1.0 } else { 0.0 },
            );
            let hoang_dao_evidence = SourceEvidence {
                source_family: "almanac_rule".to_string(),
                source_id: SOURCE_KHCBPPT.to_string(),
                method: "gio_hoang_dao_lookup".to_string(),
                profile: profile_id.clone(),
                note: Some(format!(
                    "star={} is_good={}",
                    hour.star, hour.is_good
                )),
            };

            // Axis 2 — Intent timing fit. v1 declares no source-backed
            // intent×hour rules, so this axis is uniformly unavailable.
            // The unavailability reason is surfaced verbatim in the
            // trace so explanations can say why the rank didn't fold
            // intent in. Future policy versions may populate this from
            // a ruleset-backed table.
            let intent_outcome = HourRankingAxisOutcome::unavailable(
                HourRankingAxis::IntentTimingFit,
                "no source-backed hour-specific intent rules declared in v1",
            );

            // Axis 3 — Personal hour alignment (v1 uses birth year chi
            // only, per spec §"Personal hour alignment"). Missing
            // personal birth facts make this axis unavailable — never
            // a negative signal.
            let (personal_outcome, personal_evidence, personal_availability) =
                compute_personal_hour_alignment(birth, hour_chi_index, &profile_id);

            // Axis 4 — Day-hour harmony (v1 uses only the branch
            // relation between day Chi and hour Chi, per spec
            // §"Day-hour harmony"). Stars, deities, and other overlays
            // are intentionally deferred.
            let (harmony_outcome, harmony_evidence) =
                compute_day_hour_harmony(day_chi_index, hour_chi_index, &profile_id);

            let axes = HourRankingAxes {
                hoang_dao_quality: hoang_dao_outcome.clone(),
                intent_timing_fit: intent_outcome.clone(),
                personal_hour_alignment: personal_outcome.clone(),
                day_hour_harmony: harmony_outcome.clone(),
            };

            let mut contributions: Vec<HourRankingContribution> = Vec::new();
            let mut weighted_sum = 0.0_f32;
            let mut available_weight = 0.0_f32;
            for entry in &self.axis_weights {
                let outcome = match entry.axis {
                    HourRankingAxis::HoangDaoQuality => &axes.hoang_dao_quality,
                    HourRankingAxis::IntentTimingFit => &axes.intent_timing_fit,
                    HourRankingAxis::PersonalHourAlignment => &axes.personal_hour_alignment,
                    HourRankingAxis::DayHourHarmony => &axes.day_hour_harmony,
                };
                let Some(score) = outcome.score else {
                    continue;
                };
                let contribution = score * entry.weight;
                weighted_sum += contribution;
                available_weight += entry.weight;
                let (source_evidence, availability) = match entry.axis {
                    HourRankingAxis::HoangDaoQuality => {
                        (hoang_dao_evidence.clone(), AvailabilityState::Complete)
                    }
                    HourRankingAxis::PersonalHourAlignment => {
                        (personal_evidence.clone(), personal_availability.clone())
                    }
                    HourRankingAxis::DayHourHarmony => {
                        (harmony_evidence.clone(), AvailabilityState::Complete)
                    }
                    HourRankingAxis::IntentTimingFit => {
                        // Defensive: the loop's `continue` above prevents
                        // this branch. Reaching it would be a bug, not a
                        // domain signal.
                        unreachable!("intent_timing_fit is unavailable in v1")
                    }
                };
                contributions.push(HourRankingContribution {
                    axis: entry.axis,
                    score,
                    weight: entry.weight,
                    contribution,
                    source_evidence,
                    availability,
                });
            }

            let rank_score = if available_weight > 0.0 {
                (weighted_sum / available_weight).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let warning_context = if is_avoid_day {
                Some(HourRankingWarning {
                    day_bucket: RecommendationBucket::Avoid,
                    message_vi:
                        "Giờ xếp hạng cao nhất trong ngày bị Tránh — đây là giờ tốt nhất \
                         trong các giờ khả dụng, không thay đổi đánh giá ngày."
                            .to_string(),
                })
            } else {
                None
            };

            ranked.push(RankedHourV1 {
                chi_index: hour_chi_index,
                chi_name: hour.hour_chi.clone(),
                time_range: hour.time_range.clone(),
                is_auspicious: hour.is_good,
                rank_score,
                axes,
                contributions,
                warning_context,
            });
        }

        // Rank order: rank_score descending, then chi_index ascending
        // (traditional Chi order, never alphabetical Vietnamese name).
        ranked.sort_by(|a, b| {
            b.rank_score
                .partial_cmp(&a.rank_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.chi_index.cmp(&b.chi_index))
        });

        // `intent` is intentionally unused in v1 because
        // intent_timing_fit is uniformly unavailable. The argument
        // stays in the signature so future policy versions that DO
        // fold intent in don't have to break callers.
        let _ = intent;

        Ok(ranked)
    }
}

/// Compute the v1 personal-hour-alignment axis. v1 uses birth year chi
/// only. Missing birth data → unavailable (never zero). Missing data is
/// never a negative signal.
fn compute_personal_hour_alignment(
    birth: Option<&BirthInput>,
    hour_chi_index: usize,
    profile_id: &str,
) -> (HourRankingAxisOutcome, SourceEvidence, AvailabilityState) {
    let Some(birth) = birth else {
        let reason = "missing birth profile — not a negative signal".to_string();
        return (
            HourRankingAxisOutcome::unavailable(
                HourRankingAxis::PersonalHourAlignment,
                &reason,
            ),
            SourceEvidence {
                source_family: "interaction".to_string(),
                source_id: SOURCE_KHCBPPT.to_string(),
                method: "personal_hour_alignment_unavailable".to_string(),
                profile: profile_id.to_string(),
                note: None,
            },
            AvailabilityState::Unavailable { reason },
        );
    };

    let birth_year = birth.birth_year_canchi();
    let birth_chi_index = birth_year.chi_index;
    let hour_chi_name = CHI[hour_chi_index % 12];

    let score = if birth_chi_index == hour_chi_index {
        1.0
    } else if xung_hop::luc_xung(birth_chi_index) == hour_chi_name {
        0.0
    } else {
        // Birth-year × hour-chi without a same-triad/lục-hợp signal is a
        // neutral baseline in v1; future versions may fold tam_hop /
        // liu_he features for a richer score.
        0.5
    };

    let evidence = SourceEvidence {
        source_family: "interaction".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: "birth_year_chi_hour_chi_lookup".to_string(),
        profile: profile_id.to_string(),
        note: Some(format!(
            "birth_year_chi={} ({}) hour_chi={}",
            birth_year.chi, birth_chi_index, hour_chi_name
        )),
    };

    (
        HourRankingAxisOutcome::from_score(
            HourRankingAxis::PersonalHourAlignment,
            score,
        ),
        evidence,
        AvailabilityState::Complete,
    )
}

/// Compute the v1 day-hour-harmony axis. Uses only the branch relation
/// between day Chi and hour Chi (per spec §"Day-hour harmony"). Stars,
/// deities, and other overlays are deferred.
fn compute_day_hour_harmony(
    day_chi_index: usize,
    hour_chi_index: usize,
    profile_id: &str,
) -> (HourRankingAxisOutcome, SourceEvidence) {
    let hour_chi_name = CHI[hour_chi_index % 12];

    let same_triad = xung_hop::tam_hop(day_chi_index).contains(&hour_chi_name);
    let is_liu_he = xung_hop::get_liu_he(day_chi_index) == hour_chi_name;
    let is_luc_xung = xung_hop::luc_xung(day_chi_index) == hour_chi_name;

    // Scores are ordinal heuristics calibrated so the spec verification
    // gate "Hoàng Đạo hours generally rank above Hắc Đạo hours" holds:
    // the dominant 0.45 weight on Hoàng Đạo quality plus the 0.10
    // harmony weight produces a strict ordering across the twelve slots.
    let score = if same_triad {
        0.8
    } else if is_liu_he {
        0.7
    } else if is_luc_xung {
        0.1
    } else {
        0.5
    };

    let evidence = SourceEvidence {
        source_family: "interaction".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: "day_chi_hour_chi_branch_relation".to_string(),
        profile: profile_id.to_string(),
        note: Some(format!(
            "day_chi={} hour_chi={} triad={} liu_he={} luc_xung={}",
            CHI[day_chi_index % 12],
            hour_chi_name,
            same_triad,
            is_liu_he,
            is_luc_xung
        )),
    };

    (
        HourRankingAxisOutcome::from_score(HourRankingAxis::DayHourHarmony, score),
        evidence,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VIETNAM_TIMEZONE;

    fn base_snapshot() -> DaySnapshot {
        crate::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE)
    }

    #[test]
    fn baseline_v1_policy_metadata_is_versioned() {
        let policy = HourRankingPolicy::baseline_v1();
        assert_eq!(policy.policy_id(), HOUR_RANKING_POLICY_V1_ID);
        assert_eq!(policy.policy_version(), HOUR_RANKING_POLICY_V1_VERSION);
        assert_eq!(policy.policy_version(), "v1");
    }

    #[test]
    fn baseline_v1_axis_weights_match_spec() {
        let policy = HourRankingPolicy::baseline_v1();
        let weights = policy.axis_weights();
        assert_eq!(weights.len(), 4);
        assert_eq!(weights[0].axis, HourRankingAxis::HoangDaoQuality);
        assert!((weights[0].weight - 0.45).abs() < 1e-6);
        assert_eq!(weights[1].axis, HourRankingAxis::IntentTimingFit);
        assert!((weights[1].weight - 0.25).abs() < 1e-6);
        assert_eq!(weights[2].axis, HourRankingAxis::PersonalHourAlignment);
        assert!((weights[2].weight - 0.20).abs() < 1e-6);
        assert_eq!(weights[3].axis, HourRankingAxis::DayHourHarmony);
        assert!((weights[3].weight - 0.10).abs() < 1e-6);

        let total: f32 = weights.iter().map(|w| w.weight).sum();
        assert!(
            (total - 1.0).abs() < 1e-6,
            "axis weights must sum to 1.0; got {total}"
        );
    }

    #[test]
    fn axis_enum_lists_all_four_axes_in_stable_order() {
        let names: Vec<&str> = HourRankingAxis::ALL.iter().map(|a| a.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "hoang_dao_quality",
                "intent_timing_fit",
                "personal_hour_alignment",
                "day_hour_harmony",
            ]
        );
    }

    #[test]
    fn rank_returns_twelve_hour_slots() {
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
            .expect("rank");
        assert_eq!(ranked.len(), 12);
    }

    #[test]
    fn rank_covers_all_twelve_chi_indices_exactly_once() {
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
            .expect("rank");
        let mut indices: Vec<usize> = ranked.iter().map(|h| h.chi_index).collect();
        indices.sort_unstable();
        assert_eq!(indices, (0..12).collect::<Vec<_>>());
    }

    #[test]
    fn hoang_dao_quality_axis_is_binary_under_v1() {
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
            .expect("rank");
        for hour in &ranked {
            let outcome = &hour.axes.hoang_dao_quality;
            let score = outcome.score.expect("hoang_dao_quality is always available");
            if hour.is_auspicious {
                assert!(
                    (score - 1.0).abs() < 1e-6,
                    "Hoàng Đạo hour must score 1.0; got {score}"
                );
            } else {
                assert!(
                    score.abs() < 1e-6,
                    "Hắc Đạo hour must score 0.0; got {score}"
                );
            }
        }
    }

    #[test]
    fn intent_timing_fit_axis_is_unavailable_in_v1() {
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::ContractSigning,
                None,
                None,
            )
            .expect("rank");
        for hour in &ranked {
            assert!(
                hour.axes.intent_timing_fit.score.is_none(),
                "intent_timing_fit must be unavailable in v1"
            );
            assert!(hour.axes.intent_timing_fit.unavailable_reason.is_some());
        }
    }

    #[test]
    fn missing_birth_profile_makes_personal_axis_unavailable_not_zero() {
        let policy = HourRankingPolicy::baseline_v1();
        let with_birth = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Wedding,
                None,
                None,
            )
            .expect("rank without birth");
        for hour in &with_birth {
            assert!(hour.axes.personal_hour_alignment.score.is_none());
            assert!(hour.axes.personal_hour_alignment
                .unavailable_reason
                .is_some());
        }
    }

    #[test]
    fn personal_axis_uses_birth_year_chi_only_in_v1() {
        let policy = HourRankingPolicy::baseline_v1();
        let birth = BirthInput {
            day: 15,
            month: 6,
            year: 1990,
            hour: None,
            minute: None,
            timezone: VIETNAM_TIMEZONE,
            gender: None,
            location_name: None,
        };
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Wedding,
                Some(&birth),
                None,
            )
            .expect("rank");
        let birth_year = birth.birth_year_canchi();
        let birth_chi_index = birth_year.chi_index;
        for hour in &ranked {
            let outcome = &hour.axes.personal_hour_alignment;
            assert!(outcome.score.is_some(), "with birth, axis is available");
            let score = outcome.score.unwrap();
            if hour.chi_index == birth_chi_index {
                assert!(
                    (score - 1.0).abs() < 1e-6,
                    "same chi as birth year must score 1.0; got {score}"
                );
            } else {
                // Either lục-xung (0.0) or neutral (0.5).
                let luc_xung = xung_hop::luc_xung(birth_chi_index);
                let hour_chi = CHI[hour.chi_index % 12];
                if luc_xung == hour_chi {
                    assert!(score.abs() < 1e-6);
                } else {
                    assert!((score - 0.5).abs() < 1e-6);
                }
            }
        }
    }

    #[test]
    fn rank_score_is_clamped_and_deterministic() {
        let policy = HourRankingPolicy::baseline_v1();
        let a = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
            .expect("rank a");
        let b = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
            .expect("rank b");
        assert_eq!(a, b);
        for hour in &a {
            assert!(
                (0.0..=1.0).contains(&hour.rank_score),
                "rank_score must be in [0, 1]; got {}",
                hour.rank_score
            );
        }
    }

    #[test]
    fn exact_ties_break_by_traditional_chi_order() {
        // Build a snapshot where at least two hours share the same
        // effective rank score (Hoàng Đạo quality dominant). The Hoàng
        // Đạo hours all score 1.0 on that axis, so we focus on the
        // Hoàng Đạo subset — without birth/personal alignment, all
        // Hoàng Đạo hours share the same contribution from
        // hoang_dao_quality (0.45 × 1.0) and from day_hour_harmony
        // (0.10 × score_h). The tie-break must therefore be chi_index
        // ascending (Tý < Sửu < …).
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
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
    fn avoid_day_attaches_warning_context_to_every_hour() {
        use crate::assessment::{
            PersonalDayAssessmentBuilder, PersonalDayDecision,
        };
        use crate::almanac::tu_menh::Gender;
        use crate::birth::BirthProfile;

        let snapshot = base_snapshot();
        let profile = BirthProfile {
            day: 1,
            month: 1,
            year: 1990,
            time: None,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(Gender::Male),
            location_name: None,
        };
        let mut assessment = PersonalDayAssessmentBuilder::new(
            snapshot.clone(),
            profile,
            ConsultationIntent::Wedding,
        )
        .build();
        // Force the day verdict to Avoid so warning_context must fire.
        assessment.decision = PersonalDayDecision {
            bucket: RecommendationBucket::Avoid,
            ..assessment.decision
        };

        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &snapshot,
                ConsultationIntent::Wedding,
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
            assert!(!warning.message_vi.is_empty());
        }
    }

    #[test]
    fn non_avoid_day_omits_warning_context() {
        use crate::birth::BirthProfile;
        use crate::almanac::tu_menh::Gender;
        let snapshot = base_snapshot();
        let profile = BirthProfile {
            day: 1,
            month: 1,
            year: 1990,
            time: None,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(Gender::Male),
            location_name: None,
        };
        let assessment = crate::assessment::PersonalDayAssessment::assess(
            snapshot.clone(),
            profile,
            ConsultationIntent::Wedding,
        );

        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &snapshot,
                ConsultationIntent::Wedding,
                None,
                Some(&assessment),
            )
            .expect("rank");
        for hour in &ranked {
            if hour.warning_context.is_some() {
                // Allow Mixed / Cautious / Favorable; only Avoid should attach.
                assert_ne!(
                    assessment.decision.bucket,
                    RecommendationBucket::Avoid,
                    "warning_context must only fire on Avoid days"
                );
            }
        }
    }

    #[test]
    fn no_hour_emits_a_verdict_bucket() {
        // Spec: no hour-level verdict bucket. RankedHourV1 does not have
        // a `bucket` / `verdict` field by construction; this test
        // guards the structural contract by checking the debug output
        // does not contain any verdict field name.
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
            .expect("rank");
        for hour in &ranked {
            let debug = format!("{hour:?}");
            assert!(
                !debug.contains("bucket"),
                "RankedHourV1 must not carry a verdict bucket; got {debug}"
            );
            assert!(
                !debug.contains("verdict"),
                "RankedHourV1 must not carry a verdict verdict; got {debug}"
            );
        }
    }

    #[test]
    fn contributions_match_available_axes() {
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
            .expect("rank");
        for hour in &ranked {
            // Without birth, available axes are Hoàng Đạo + Day-hour harmony.
            assert_eq!(
                hour.contributions.len(),
                2,
                "without birth: expected 2 contributions (hoang_dao + harmony), got {}",
                hour.contributions.len()
            );
            let axes: Vec<HourRankingAxis> =
                hour.contributions.iter().map(|c| c.axis).collect();
            assert!(axes.contains(&HourRankingAxis::HoangDaoQuality));
            assert!(axes.contains(&HourRankingAxis::DayHourHarmony));
            assert!(!axes.contains(&HourRankingAxis::PersonalHourAlignment));
            assert!(!axes.contains(&HourRankingAxis::IntentTimingFit));
        }
    }

    #[test]
    fn hoang_dao_hours_outrank_hac_dao_hours_without_birth() {
        // Spec verification gate: Hoàng Đạo hours generally rank above
        // Hắc Đạo hours. With intent_timing_fit unavailable and no
        // personal alignment, the only axis that differentiates is
        // hoang_dao_quality, so every Hoàng Đạo hour must strictly
        // outrank every Hắc Đạo hour.
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(
                &base_snapshot(),
                ConsultationIntent::Travel,
                None,
                None,
            )
            .expect("rank");
        let min_hoang_dao = ranked
            .iter()
            .filter(|h| h.is_auspicious)
            .map(|h| h.rank_score)
            .fold(f32::INFINITY, f32::min);
        let max_hac_dao = ranked
            .iter()
            .filter(|h| !h.is_auspicious)
            .map(|h| h.rank_score)
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(
            min_hoang_dao > max_hac_dao,
            "Hoàng Đạo hours must strictly outrank Hắc Đạo hours; \
             min_hoang_dao={min_hoang_dao}, max_hac_dao={max_hac_dao}"
        );
    }
}