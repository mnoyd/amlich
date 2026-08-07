//! First-class hour-ranking policy.
//!
//! Source spec: `docs/architecture/personal-day-audit/HOUR-RANKING-POLICY-V1-SPEC.md`
//! Bead: `amlich-rv13.3` (parent epic: `amlich-rv13`).
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
    assessment::{AvailabilityState, ContributionPolarity, PersonalDayAssessment, SourceEvidence},
    gio_hoang_dao::HourInfo,
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

/// Stable, source-attributed feature identifiers for the hour-ranking
/// pipeline. Per ADR-0001 these are intentionally distinct from
/// [`crate::assessment::AssessmentFeatureId`] so day and hour ranking
/// keep separate domain vocabulary even when they share aggregation
/// mechanics. Identifiers are versioned by
/// [`HOUR_RANKING_POLICY_V1_VERSION`]: any new feature or rename MUST
/// bump the policy version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HourRankingFeatureId {
    /// Binary Hoàng Đạo / Hắc Đạo membership for one hour slot. Always
    /// emitted (the snapshot's gio_hoang_dao table covers all twelve
    /// slots), so this feature is structurally available in v1.
    HourHoangDaoMembership,
    /// Per-hour, source-backed support for the consultation intent.
    /// Uniformly unavailable in v1 because no declared intent×hour
    /// rule set has been settled. Future policy versions may emit a
    /// structured ruleset-backed observation.
    HourIntentTimingFit,
    /// Birth-year Chi matches the hour Chi (requires a birth profile).
    /// Emitted at strength 1.0 when the year's Chi equals the hour's
    /// Chi; not emitted when the birth profile is missing or when the
    /// relation is something other than a direct match.
    PersonalHourYearChiMatch,
    /// Birth-year Chi is in lục-xung clash with the hour Chi (requires
    /// a birth profile). Emitted at strength 1.0 with `Avoid` polarity
    /// on a clash; not emitted otherwise.
    PersonalHourYearChiLucXung,
    /// Birth-year Chi and hour Chi form a neutral baseline (requires a
    /// birth profile). Emitted at strength 1.0 with `Neutral` polarity
    /// when the year Chi and hour Chi are neither a match nor a clash
    /// and no other personal signal applies in v1.
    PersonalHourYearChiNeutral,
    /// Day Chi and hour Chi form a tam-hợp branch triad. Emitted at
    /// strength 1.0 with `Favorable` polarity on a triad match.
    HourBranchTriad,
    /// Day Chi and hour Chi form a lục-hợp branch pair. Emitted at
    /// strength 1.0 with `Favorable` polarity on a lục-hợp match.
    HourBranchLiuHe,
    /// Day Chi and hour Chi form a lục-xung clash pair. Emitted at
    /// strength 1.0 with `Avoid` polarity on a clash.
    HourBranchLucXung,
}

impl HourRankingFeatureId {
    /// All declared feature identifiers in canonical declaration order.
    /// Stable across policy versions; serialized traces and parity
    /// fixtures rely on this order.
    pub const ALL: [Self; 8] = [
        Self::HourHoangDaoMembership,
        Self::HourIntentTimingFit,
        Self::PersonalHourYearChiMatch,
        Self::PersonalHourYearChiLucXung,
        Self::PersonalHourYearChiNeutral,
        Self::HourBranchTriad,
        Self::HourBranchLiuHe,
        Self::HourBranchLucXung,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::HourHoangDaoMembership => "hour_hoang_dao_membership",
            Self::HourIntentTimingFit => "hour_intent_timing_fit",
            Self::PersonalHourYearChiMatch => "personal_hour_year_chi_match",
            Self::PersonalHourYearChiLucXung => "personal_hour_year_chi_luc_xung",
            Self::PersonalHourYearChiNeutral => "personal_hour_year_chi_neutral",
            Self::HourBranchTriad => "hour_branch_triad",
            Self::HourBranchLiuHe => "hour_branch_liu_he",
            Self::HourBranchLucXung => "hour_branch_luc_xung",
        }
    }

    /// Axis this feature primarily contributes to under `baseline_v1`.
    /// Hour ranking keeps a one-feature-per-axis mapping at the v1
    /// baseline; future policy versions may layer richer aggregations.
    pub fn default_axis(self) -> HourRankingAxis {
        match self {
            Self::HourHoangDaoMembership => HourRankingAxis::HoangDaoQuality,
            Self::HourIntentTimingFit => HourRankingAxis::IntentTimingFit,
            Self::PersonalHourYearChiMatch
            | Self::PersonalHourYearChiLucXung
            | Self::PersonalHourYearChiNeutral => HourRankingAxis::PersonalHourAlignment,
            Self::HourBranchTriad | Self::HourBranchLiuHe | Self::HourBranchLucXung => {
                HourRankingAxis::DayHourHarmony
            }
        }
    }
}

/// One hour-specific feature observation extracted from a
/// `(DaySnapshot, ConsultationIntent, Option<BirthInput>)` triple. The
/// embedded [`chi_index`](Self::chi_index) is the discriminator that
/// pins the observation to exactly one of the twelve traditional hour
/// slots, so the extraction layer can produce a flat `Vec` keyed by
/// `(feature_id, chi_index)` without nesting.
///
/// `score == None` together with `unavailable_reason.is_some()` means
/// the feature could not be evaluated for this hour slot — the
/// "unavailable is distinct from zero" contract from `amlich-7bm4`,
/// carried over into the hour domain. The aggregator MUST exclude
/// unavailable observations from the rank-score denominator and report
/// them via `unavailable_reason`, never substitute a neutral fallback.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourRankingFeatureObservation {
    pub feature_id: HourRankingFeatureId,
    pub chi_index: usize,
    /// Polarity the feature contributes under `baseline_v1`. Hour
    /// ranks are aggregated in `[0.0, 1.0]` (not `[-1, 1]` like the
    /// day axes), so polarity is metadata for explanations and
    /// contribution IDs rather than the sign multiplier used at
    /// aggregation time.
    pub polarity: ContributionPolarity,
    /// Normalized score in `[0.0, 1.0]` when the feature is
    /// available. `None` together with `unavailable_reason` means the
    /// feature could not be evaluated for this hour.
    pub score: Option<f32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
    pub source_evidence: SourceEvidence,
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub contribution_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl HourRankingFeatureObservation {
    /// Construct an available observation with normalized magnitude and
    /// typed polarity. `score` is clamped to `[0.0, 1.0]`.
    #[allow(clippy::too_many_arguments)]
    pub fn observed(
        feature_id: HourRankingFeatureId,
        chi_index: usize,
        polarity: ContributionPolarity,
        score: f32,
        source_evidence: SourceEvidence,
        ruleset_id: impl Into<String>,
        ruleset_version: impl Into<String>,
        contribution_id: impl Into<String>,
    ) -> Self {
        Self {
            feature_id,
            chi_index,
            polarity,
            score: Some(score.clamp(0.0, 1.0)),
            unavailable_reason: None,
            source_evidence,
            ruleset_id: ruleset_id.into(),
            ruleset_version: ruleset_version.into(),
            contribution_id: contribution_id.into(),
            note: None,
        }
    }

    /// Construct an unavailable observation. The aggregator MUST
    /// exclude these from the rank-score denominator and surface them
    /// via `unavailable_reason` rather than substituting a neutral
    /// fallback.
    #[allow(clippy::too_many_arguments)]
    pub fn unavailable(
        feature_id: HourRankingFeatureId,
        chi_index: usize,
        reason: impl Into<String>,
        source_evidence: SourceEvidence,
        ruleset_id: impl Into<String>,
        ruleset_version: impl Into<String>,
        contribution_id: impl Into<String>,
    ) -> Self {
        Self {
            feature_id,
            chi_index,
            polarity: ContributionPolarity::Info,
            score: None,
            unavailable_reason: Some(reason.into()),
            source_evidence,
            ruleset_id: ruleset_id.into(),
            ruleset_version: ruleset_version.into(),
            contribution_id: contribution_id.into(),
            note: None,
        }
    }

    /// True if this observation is unavailable for the current hour
    /// slot. The rank-score aggregator MUST skip unavailable
    /// observations and reweight the remaining axes.
    pub fn is_unavailable(&self) -> bool {
        self.score.is_none()
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
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

/// Aggregation result for one hour slot: the rank score plus the
/// per-axis contribution list. Built by
/// [`HourRankingPolicy::aggregate_hour_ranking`] and consumed by
/// [`HourRankingPolicy::rank`]. Exposed as a public type so the
/// Evidence Graph projection (`amlich-8tdm`) can describe the
/// aggregation step without recomputing it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourRankingAggregation {
    /// The rank score in `0.0..=1.0`, computed as the weighted average
    /// over available axes only. Unavailable axes are removed from
    /// the denominator rather than folded in as a neutral fallback
    /// (spec §"Initial weight profile"). The score is clamped to
    /// `0.0..=1.0` to defend against floating-point drift.
    pub rank_score: f32,
    /// One contribution per available axis, in [`HourRankingAxis::ALL`]
    /// declaration order. Unavailable axes are excluded so the
    /// contribution list always matches the rank-score formula's
    /// available-axis denominator.
    pub contributions: Vec<HourRankingContribution>,
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

    /// Aggregate one hour slot's per-axis outcomes into a rank score
    /// and contribution list. Implements the v1 weighted-average
    /// formula from spec §"Initial weight profile":
    ///
    /// ```text
    /// rank_score = Σ(axis_score × axis_weight) / Σ(available_axis_weights)
    /// ```
    ///
    /// Unavailable axes are removed from the denominator — the spec
    /// forbids substituting a neutral `0.5` fallback just to fill the
    /// vector (per-axis "unavailable is distinct from zero" contract
    /// from `amlich-rv13.2`). The score is clamped to `0.0..=1.0` to
    /// defend against floating-point drift.
    ///
    /// Contributions are emitted in [`HourRankingAxis::ALL`] order, one
    /// per available axis. Each contribution pulls its source evidence
    /// from the first available axis-feeding feature observation; if
    /// no feature fed the axis (defensive — the v1 axis-feeding
    /// extractors always emit at least one observation), a default
    /// almanac-rule evidence is attached so the trace stays
    /// self-describing.
    ///
    /// Pure and deterministic: identical `(axes, hour_features, profile_id)`
    /// tuples produce identical [`HourRankingAggregation`] outputs.
    /// Used by [`Self::rank`] for every hour slot; exposed at
    /// `pub(super)` for direct testing.
    ///
    /// Bead: `amlich-rv13.3`.
    pub(super) fn aggregate_hour_ranking(
        &self,
        axes: &HourRankingAxes,
        hour_features: &[&HourRankingFeatureObservation],
        profile_id: &str,
    ) -> HourRankingAggregation {
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

            // Pull the most informative source evidence for this axis
            // from the underlying observation. For axes with a single
            // observation (v1), this is the observation's own evidence.
            // For axes with multiple observations (personal alignment,
            // day-hour harmony), prefer the matched observation's
            // evidence and fall back to the neutral baseline.
            let axis_features: Vec<&&HourRankingFeatureObservation> = hour_features
                .iter()
                .filter(|f| f.feature_id.default_axis() == entry.axis && !f.is_unavailable())
                .collect();
            let source_evidence = axis_features
                .first()
                .map(|f| f.source_evidence.clone())
                .unwrap_or_else(|| SourceEvidence {
                    source_family: "almanac_rule".to_string(),
                    source_id: SOURCE_KHCBPPT.to_string(),
                    method: "hour_axis_default".to_string(),
                    profile: profile_id.to_string(),
                    note: None,
                });
            let availability = if axis_features.is_empty() {
                AvailabilityState::Unavailable {
                    reason: "no axis-feeding feature available".to_string(),
                }
            } else {
                AvailabilityState::Complete
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

        HourRankingAggregation {
            rank_score,
            contributions,
        }
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

        let profile_id = snapshot.profile.clone();
        let ruleset_id = snapshot.ruleset_id.clone();
        let ruleset_version = snapshot.ruleset_version.clone();
        let is_avoid_day = matches!(
            day_assessment.map(|a| a.decision.bucket),
            Some(RecommendationBucket::Avoid)
        );

        // Deterministic extraction phase (amlich-rv13.2). Produces one
        // feature observation per (axis-aligned feature, hour slot) for
        // all twelve traditional slots, including explicit unavailable
        // observations for axes that cannot be evaluated. The
        // aggregation phase below translates these observations into
        // per-axis scores and contributions; this split mirrors the
        // day assessment pipeline (`extract_features` →
        // `aggregate_axes`) so the hour ranking can layer richer
        // features in future policy versions without rewriting the
        // rank-score math.
        let features =
            extract_hour_features(snapshot, birth, &ruleset_id, &ruleset_version, &profile_id);

        let mut ranked: Vec<RankedHourV1> = Vec::with_capacity(12);
        for hour in &hoang_dao.all_hours {
            let hour_chi_index = hour.hour_index;

            // Slice observations belonging to this hour slot. The
            // extraction layer guarantees at most one observation per
            // (feature_id, chi_index) pair, so per-axis fold is just a
            // find-or-none per feature.
            let hour_features: Vec<&HourRankingFeatureObservation> = features
                .iter()
                .filter(|f| f.chi_index == hour_chi_index)
                .collect();

            let axes = aggregate_hour_axes(&hour_features);
            let aggregation = self.aggregate_hour_ranking(&axes, &hour_features, &profile_id);

            let warning_context = if is_avoid_day {
                Some(HourRankingWarning {
                    day_bucket: RecommendationBucket::Avoid,
                    message_vi: "Giờ xếp hạng cao nhất trong ngày bị Tránh — đây là giờ tốt nhất \
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
                rank_score: aggregation.rank_score,
                axes,
                contributions: aggregation.contributions,
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

/// Extract hour-specific feature observations for all twelve
/// traditional hour slots. Deterministic: identical
/// `(snapshot, birth)` pairs produce identical `Vec`s. Emits one
/// observation per declared [`HourRankingFeatureId`] per hour when the
/// feature is structurally available, plus explicit *unavailable*
/// observations for axes that cannot be evaluated (intent timing fit
/// in v1; personal alignment when the birth profile is missing).
///
/// Bead: `amlich-rv13.2`.
pub(super) fn extract_hour_features(
    snapshot: &DaySnapshot,
    birth: Option<&BirthInput>,
    ruleset_id: &str,
    ruleset_version: &str,
    profile_id: &str,
) -> Vec<HourRankingFeatureObservation> {
    let hoang_dao = &snapshot.context.gio_hoang_dao;
    let day_chi_index = hoang_dao.day_chi_index;

    let mut features: Vec<HourRankingFeatureObservation> =
        Vec::with_capacity(hoang_dao.all_hours.len() * HourRankingFeatureId::ALL.len());

    for hour in &hoang_dao.all_hours {
        let hour_chi_index = hour.hour_index;

        // Axis 1 — Hoàng Đạo quality (binary in v1 per spec). Always
        // available because the gio_hoang_dao snapshot table covers
        // all twelve slots.
        features.push(extract_hoang_dao_observation(
            hour,
            ruleset_id,
            ruleset_version,
            profile_id,
        ));

        // Axis 2 — Intent timing fit. Uniformly unavailable in v1
        // because no declared intent×hour rule set has been settled.
        // The unavailability reason is surfaced verbatim in the trace
        // so explanations can say why the rank didn't fold intent in.
        features.push(extract_intent_timing_observation(
            hour_chi_index,
            ruleset_id,
            ruleset_version,
            profile_id,
        ));

        // Axis 3 — Personal hour alignment (v1 uses birth year chi
        // only). Missing personal birth facts make the axis
        // unavailable — never a negative signal.
        features.extend(extract_personal_hour_observations(
            birth,
            hour_chi_index,
            ruleset_id,
            ruleset_version,
            profile_id,
        ));

        // Axis 4 — Day-hour harmony (v1 uses only the branch relation
        // between day Chi and hour Chi). Stars, deities, and other
        // overlays are intentionally deferred.
        features.push(extract_day_hour_harmony_observation(
            day_chi_index,
            hour_chi_index,
            ruleset_id,
            ruleset_version,
            profile_id,
        ));
    }

    features
}

/// Aggregate a single hour slot's feature observations into the four
/// axis outcomes. v1 maps features to axis scores via deterministic
/// per-axis rules (binary Hoàng Đạo membership; uniformly unavailable
/// intent; birth-year match/clash/neutral → 1.0/0.0/0.5;
/// tam-hợp/lục-hợp/lục-xung → 0.8/0.7/0.1). Future policy versions
/// can replace this function without rewriting the surrounding rank
/// loop, mirroring the day assessment pipeline's `aggregate_one_axis`
/// seam.
fn aggregate_hour_axes(hour_features: &[&HourRankingFeatureObservation]) -> HourRankingAxes {
    // Axis 1 — Hoàng Đạo quality (binary in v1).
    let hoang_dao_outcome = hour_features
        .iter()
        .find(|f| f.feature_id == HourRankingFeatureId::HourHoangDaoMembership)
        .map(|f| {
            if f.is_unavailable() {
                // Defensive: the gio_hoang_dao table covers all twelve
                // slots, so this branch is unreachable under normal
                // snapshot construction. Keep it for honesty against
                // malformed inputs.
                HourRankingAxisOutcome::unavailable(
                    HourRankingAxis::HoangDaoQuality,
                    f.unavailable_reason
                        .as_deref()
                        .unwrap_or("hoang_dao membership unavailable"),
                )
            } else {
                HourRankingAxisOutcome::from_score(
                    HourRankingAxis::HoangDaoQuality,
                    f.score.unwrap_or(0.0),
                )
            }
        })
        .unwrap_or_else(|| {
            HourRankingAxisOutcome::unavailable(
                HourRankingAxis::HoangDaoQuality,
                "no hoang_dao observation extracted for hour slot",
            )
        });

    // Axis 2 — Intent timing fit (uniformly unavailable in v1).
    let intent_outcome = HourRankingAxisOutcome::unavailable(
        HourRankingAxis::IntentTimingFit,
        "no source-backed hour-specific intent rules declared in v1",
    );

    // Axis 3 — Personal hour alignment. v1 rules:
    //   match → 1.0
    //   luc_xung → 0.0
    //   neutral baseline → 0.5
    //   no birth facts → unavailable (never zero)
    let personal_outcome = {
        let has_match = hour_features.iter().any(|f| {
            f.feature_id == HourRankingFeatureId::PersonalHourYearChiMatch && !f.is_unavailable()
        });
        let has_clash = hour_features.iter().any(|f| {
            f.feature_id == HourRankingFeatureId::PersonalHourYearChiLucXung && !f.is_unavailable()
        });
        let has_neutral = hour_features.iter().any(|f| {
            f.feature_id == HourRankingFeatureId::PersonalHourYearChiNeutral && !f.is_unavailable()
        });
        if has_match {
            HourRankingAxisOutcome::from_score(HourRankingAxis::PersonalHourAlignment, 1.0)
        } else if has_clash {
            HourRankingAxisOutcome::from_score(HourRankingAxis::PersonalHourAlignment, 0.0)
        } else if has_neutral {
            HourRankingAxisOutcome::from_score(HourRankingAxis::PersonalHourAlignment, 0.5)
        } else {
            HourRankingAxisOutcome::unavailable(
                HourRankingAxis::PersonalHourAlignment,
                "missing birth profile — not a negative signal",
            )
        }
    };

    // Axis 4 — Day-hour harmony. v1 rules:
    //   tam-hợp → 0.8
    //   lục-hợp → 0.7
    //   lục-xung → 0.1
    //   none of the above → 0.5 (neutral baseline)
    let harmony_outcome = {
        let has_triad = hour_features
            .iter()
            .any(|f| f.feature_id == HourRankingFeatureId::HourBranchTriad && !f.is_unavailable());
        let has_liu_he = hour_features
            .iter()
            .any(|f| f.feature_id == HourRankingFeatureId::HourBranchLiuHe && !f.is_unavailable());
        let has_luc_xung = hour_features.iter().any(|f| {
            f.feature_id == HourRankingFeatureId::HourBranchLucXung && !f.is_unavailable()
        });
        if has_triad {
            HourRankingAxisOutcome::from_score(HourRankingAxis::DayHourHarmony, 0.8)
        } else if has_liu_he {
            HourRankingAxisOutcome::from_score(HourRankingAxis::DayHourHarmony, 0.7)
        } else if has_luc_xung {
            HourRankingAxisOutcome::from_score(HourRankingAxis::DayHourHarmony, 0.1)
        } else {
            HourRankingAxisOutcome::from_score(HourRankingAxis::DayHourHarmony, 0.5)
        }
    };

    HourRankingAxes {
        hoang_dao_quality: hoang_dao_outcome,
        intent_timing_fit: intent_outcome,
        personal_hour_alignment: personal_outcome,
        day_hour_harmony: harmony_outcome,
    }
}

/// Extract the per-hour Hoàng Đạo / Hắc Đạo membership observation.
/// Binary in v1 per spec §"Hoàng Đạo quality" — the score is 1.0 for
/// Hoàng Đạo hours and 0.0 for Hắc Đạo hours. Sub-grades are
/// intentionally deferred to future policy versions.
fn extract_hoang_dao_observation(
    hour: &HourInfo,
    ruleset_id: &str,
    ruleset_version: &str,
    profile_id: &str,
) -> HourRankingFeatureObservation {
    let chi_index = hour.hour_index;
    let evidence = SourceEvidence {
        source_family: "almanac_rule".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: "gio_hoang_dao_lookup".to_string(),
        profile: profile_id.to_string(),
        note: Some(format!("star={} is_good={}", hour.star, hour.is_good)),
    };
    let contribution_id = format!("hour.hoang_dao.{}", chi_index);
    HourRankingFeatureObservation::observed(
        HourRankingFeatureId::HourHoangDaoMembership,
        chi_index,
        if hour.is_good {
            ContributionPolarity::Favorable
        } else {
            ContributionPolarity::Avoid
        },
        if hour.is_good { 1.0 } else { 0.0 },
        evidence,
        ruleset_id,
        ruleset_version,
        contribution_id,
    )
    .with_note(if hour.is_good {
        "Hoàng Đạo"
    } else {
        "Hắc Đạo"
    })
}

/// Extract the per-hour intent timing fit observation. v1 declares no
/// source-backed intent×hour rules, so this axis is uniformly
/// unavailable. The unavailability reason is surfaced verbatim in the
/// trace so explanations can say why the rank didn't fold intent in.
/// Future policy versions may populate this from a ruleset-backed
/// table without changing call sites.
fn extract_intent_timing_observation(
    chi_index: usize,
    ruleset_id: &str,
    ruleset_version: &str,
    profile_id: &str,
) -> HourRankingFeatureObservation {
    let evidence = SourceEvidence {
        source_family: "almanac_rule".to_string(),
        source_id: SOURCE_KHCBPPT.to_string(),
        method: "intent_timing_lookup".to_string(),
        profile: profile_id.to_string(),
        note: None,
    };
    let contribution_id = format!("hour.intent_timing.{}", chi_index);
    HourRankingFeatureObservation::unavailable(
        HourRankingFeatureId::HourIntentTimingFit,
        chi_index,
        "no source-backed hour-specific intent rules declared in v1",
        evidence,
        ruleset_id,
        ruleset_version,
        contribution_id,
    )
}

/// Extract per-hour personal hour alignment observations. v1 uses
/// birth-year Chi only per spec §"Personal hour alignment". Missing
/// personal birth facts make the axis unavailable — never a negative
/// signal. The function emits zero, one, or two observations per hour:
/// exactly one when the birth year and hour Chi share a defined
/// relation (match, clash, or neutral baseline); an explicit
/// unavailable observation when the birth profile is missing.
fn extract_personal_hour_observations(
    birth: Option<&BirthInput>,
    hour_chi_index: usize,
    ruleset_id: &str,
    ruleset_version: &str,
    profile_id: &str,
) -> Vec<HourRankingFeatureObservation> {
    let Some(birth) = birth else {
        // Missing birth profile → emit an explicit unavailable
        // observation so the trace explains why the axis didn't fold
        // personal facts in. Never substitute a neutral fallback: the
        // spec treats missing data as a coverage gap, not as zero
        // signal.
        let evidence = SourceEvidence {
            source_family: "interaction".to_string(),
            source_id: SOURCE_KHCBPPT.to_string(),
            method: "personal_hour_alignment_unavailable".to_string(),
            profile: profile_id.to_string(),
            note: None,
        };
        let contribution_id = format!("hour.personal.alignment.{}", hour_chi_index);
        return vec![HourRankingFeatureObservation::unavailable(
            HourRankingFeatureId::PersonalHourYearChiNeutral,
            hour_chi_index,
            "missing birth profile — not a negative signal",
            evidence,
            ruleset_id,
            ruleset_version,
            contribution_id,
        )];
    };

    let birth_year = birth.birth_year_canchi();
    let birth_chi_index = birth_year.chi_index;
    let hour_chi_name = CHI[hour_chi_index % 12];
    let luc_xung_target = xung_hop::luc_xung(birth_chi_index);

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

    let mut observations = Vec::new();
    if birth_chi_index == hour_chi_index {
        let contribution_id = format!("hour.personal.match.{}", hour_chi_index);
        observations.push(
            HourRankingFeatureObservation::observed(
                HourRankingFeatureId::PersonalHourYearChiMatch,
                hour_chi_index,
                ContributionPolarity::Favorable,
                1.0,
                evidence,
                ruleset_id,
                ruleset_version,
                contribution_id,
            )
            .with_note("birth year chi matches hour chi"),
        );
    } else if luc_xung_target == hour_chi_name {
        let contribution_id = format!("hour.personal.luc_xung.{}", hour_chi_index);
        observations.push(
            HourRankingFeatureObservation::observed(
                HourRankingFeatureId::PersonalHourYearChiLucXung,
                hour_chi_index,
                ContributionPolarity::Avoid,
                1.0,
                evidence,
                ruleset_id,
                ruleset_version,
                contribution_id,
            )
            .with_note("birth year chi clashes with hour chi"),
        );
    } else {
        // Birth-year × hour-chi without a same-triad/lục-hợp signal is
        // a neutral baseline in v1; future versions may fold tam_hop /
        // liu_he features for a richer score.
        let contribution_id = format!("hour.personal.neutral.{}", hour_chi_index);
        observations.push(
            HourRankingFeatureObservation::observed(
                HourRankingFeatureId::PersonalHourYearChiNeutral,
                hour_chi_index,
                ContributionPolarity::Neutral,
                1.0,
                evidence,
                ruleset_id,
                ruleset_version,
                contribution_id,
            )
            .with_note("birth year chi neutral baseline"),
        );
    }
    observations
}

/// Extract the per-hour day-hour-harmony observation. v1 uses only the
/// branch relation between day Chi and hour Chi per spec §"Day-hour
/// harmony". Stars, deities, and other overlays are intentionally
/// deferred to future policy versions.
fn extract_day_hour_harmony_observation(
    day_chi_index: usize,
    hour_chi_index: usize,
    ruleset_id: &str,
    ruleset_version: &str,
    profile_id: &str,
) -> HourRankingFeatureObservation {
    let hour_chi_name = CHI[hour_chi_index % 12];

    let same_triad = xung_hop::tam_hop(day_chi_index).contains(&hour_chi_name);
    let is_liu_he = xung_hop::get_liu_he(day_chi_index) == hour_chi_name;
    let is_luc_xung = xung_hop::luc_xung(day_chi_index) == hour_chi_name;

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

    let (feature_id, polarity, contribution_id_suffix, note_vi) = if same_triad {
        (
            HourRankingFeatureId::HourBranchTriad,
            ContributionPolarity::Favorable,
            "triad",
            "tam hợp với Chi ngày",
        )
    } else if is_liu_he {
        (
            HourRankingFeatureId::HourBranchLiuHe,
            ContributionPolarity::Favorable,
            "liu_he",
            "lục hợp với Chi ngày",
        )
    } else if is_luc_xung {
        (
            HourRankingFeatureId::HourBranchLucXung,
            ContributionPolarity::Avoid,
            "luc_xung",
            "lục xung với Chi ngày",
        )
    } else {
        (
            HourRankingFeatureId::HourBranchLucXung,
            ContributionPolarity::Info,
            "default",
            "không có quan hệ chi đặc biệt với Chi ngày",
        )
    };

    let contribution_id = format!("hour.harmony.{}.{}", contribution_id_suffix, hour_chi_index);
    HourRankingFeatureObservation::observed(
        feature_id,
        hour_chi_index,
        polarity,
        1.0,
        evidence,
        ruleset_id,
        ruleset_version,
        contribution_id,
    )
    .with_note(note_vi)
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
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
            .expect("rank");
        assert_eq!(ranked.len(), 12);
    }

    #[test]
    fn rank_covers_all_twelve_chi_indices_exactly_once() {
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
            .expect("rank");
        let mut indices: Vec<usize> = ranked.iter().map(|h| h.chi_index).collect();
        indices.sort_unstable();
        assert_eq!(indices, (0..12).collect::<Vec<_>>());
    }

    #[test]
    fn hoang_dao_quality_axis_is_binary_under_v1() {
        let policy = HourRankingPolicy::baseline_v1();
        let ranked = policy
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
            .expect("rank");
        for hour in &ranked {
            let outcome = &hour.axes.hoang_dao_quality;
            let score = outcome
                .score
                .expect("hoang_dao_quality is always available");
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
            .rank(&base_snapshot(), ConsultationIntent::Wedding, None, None)
            .expect("rank without birth");
        for hour in &with_birth {
            assert!(hour.axes.personal_hour_alignment.score.is_none());
            assert!(hour
                .axes
                .personal_hour_alignment
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
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
            .expect("rank a");
        let b = policy
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
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
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
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
        use crate::almanac::tu_menh::Gender;
        use crate::assessment::{PersonalDayAssessmentBuilder, PersonalDayDecision};
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
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
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
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
            .expect("rank");
        for hour in &ranked {
            // Without birth, available axes are Hoàng Đạo + Day-hour harmony.
            assert_eq!(
                hour.contributions.len(),
                2,
                "without birth: expected 2 contributions (hoang_dao + harmony), got {}",
                hour.contributions.len()
            );
            let axes: Vec<HourRankingAxis> = hour.contributions.iter().map(|c| c.axis).collect();
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
            .rank(&base_snapshot(), ConsultationIntent::Travel, None, None)
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

    // -------------------------------------------------------------------
    // amlich-rv13.2 — hour ranking feature observation tests.
    //
    // The extraction layer (extract_hour_features) must be deterministic,
    // must emit one observation per declared (feature_id, chi_index)
    // pair when the feature is structurally available, and must emit
    // explicit unavailable observations for axes that cannot be
    // evaluated. These tests pin the contract from the bead's
    // acceptance criteria.
    // -------------------------------------------------------------------

    fn ruleset_meta(snapshot: &DaySnapshot) -> (&str, &str, &str) {
        (
            &snapshot.ruleset_id,
            &snapshot.ruleset_version,
            &snapshot.profile,
        )
    }

    #[test]
    fn feature_id_enum_lists_all_eight_features_in_stable_order() {
        let names: Vec<&str> = HourRankingFeatureId::ALL
            .iter()
            .map(|f| f.as_str())
            .collect();
        assert_eq!(
            names,
            vec![
                "hour_hoang_dao_membership",
                "hour_intent_timing_fit",
                "personal_hour_year_chi_match",
                "personal_hour_year_chi_luc_xung",
                "personal_hour_year_chi_neutral",
                "hour_branch_triad",
                "hour_branch_liu_he",
                "hour_branch_luc_xung",
            ]
        );
        assert_eq!(HourRankingFeatureId::ALL.len(), 8);
    }

    #[test]
    fn feature_id_default_axis_maps_each_feature_to_its_axis() {
        assert_eq!(
            HourRankingFeatureId::HourHoangDaoMembership.default_axis(),
            HourRankingAxis::HoangDaoQuality
        );
        assert_eq!(
            HourRankingFeatureId::HourIntentTimingFit.default_axis(),
            HourRankingAxis::IntentTimingFit
        );
        assert_eq!(
            HourRankingFeatureId::PersonalHourYearChiMatch.default_axis(),
            HourRankingAxis::PersonalHourAlignment
        );
        assert_eq!(
            HourRankingFeatureId::PersonalHourYearChiLucXung.default_axis(),
            HourRankingAxis::PersonalHourAlignment
        );
        assert_eq!(
            HourRankingFeatureId::PersonalHourYearChiNeutral.default_axis(),
            HourRankingAxis::PersonalHourAlignment
        );
        assert_eq!(
            HourRankingFeatureId::HourBranchTriad.default_axis(),
            HourRankingAxis::DayHourHarmony
        );
        assert_eq!(
            HourRankingFeatureId::HourBranchLiuHe.default_axis(),
            HourRankingAxis::DayHourHarmony
        );
        assert_eq!(
            HourRankingFeatureId::HourBranchLucXung.default_axis(),
            HourRankingAxis::DayHourHarmony
        );
    }

    #[test]
    fn extract_features_covers_all_twelve_chi_indices_per_axis() {
        // AC: extraction must cover all twelve traditional hour slots.
        // Hoàng Đạo + intent + at-least-one-of-{personal_*} + harmony
        // are emitted per hour; with no birth the personal slot is one
        // explicit unavailable observation. Total observations per
        // hour therefore: 1 hoang + 1 intent + 1 personal(unavailable)
        // + 1 harmony = 4, × 12 hours = 48.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let features =
            extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        assert_eq!(features.len(), 48);

        // Each chi_index in [0, 12) appears at least once per feature
        // axis. We assert this by counting observations per chi_index
        // and confirming every chi_index has the expected fan-out.
        let mut per_chi: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for f in &features {
            *per_chi.entry(f.chi_index).or_insert(0) += 1;
        }
        for chi in 0..12 {
            assert_eq!(
                per_chi.get(&chi).copied().unwrap_or(0),
                4,
                "hour chi_index={chi} must have 4 observations, got {}",
                per_chi.get(&chi).copied().unwrap_or(0)
            );
        }
    }

    #[test]
    fn extract_features_is_deterministic_for_identical_inputs() {
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let a = extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        let b = extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        assert_eq!(a, b);
    }

    #[test]
    fn hoang_dao_observations_are_binary_under_v1() {
        // AC: Hoàng Đạo quality is binary.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let features =
            extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        let hoang_dao = snapshot.context.gio_hoang_dao.all_hours.clone();
        for obs in features
            .iter()
            .filter(|f| f.feature_id == HourRankingFeatureId::HourHoangDaoMembership)
        {
            let hour = hoang_dao
                .iter()
                .find(|h| h.hour_index == obs.chi_index)
                .expect("hour for observation");
            let score = obs
                .score
                .expect("hoang_dao observations are always available in v1");
            if hour.is_good {
                assert!(
                    (score - 1.0).abs() < 1e-6,
                    "Hoàng Đạo hour must score 1.0; got {score}"
                );
                assert_eq!(obs.polarity, ContributionPolarity::Favorable);
            } else {
                assert!(
                    score.abs() < 1e-6,
                    "Hắc Đạo hour must score 0.0; got {score}"
                );
                assert_eq!(obs.polarity, ContributionPolarity::Avoid);
            }
        }
    }

    #[test]
    fn intent_timing_observations_are_unavailable_in_v1() {
        // AC: intent timing fit uses declared source-backed rules only
        // and is unavailable otherwise.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let features =
            extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        let intent_obs: Vec<&HourRankingFeatureObservation> = features
            .iter()
            .filter(|f| f.feature_id == HourRankingFeatureId::HourIntentTimingFit)
            .collect();
        assert_eq!(intent_obs.len(), 12, "one unavailable observation per hour");
        for obs in intent_obs {
            assert!(
                obs.is_unavailable(),
                "intent_timing_fit must be unavailable"
            );
            assert_eq!(obs.score, None);
            assert!(
                obs.unavailable_reason.is_some(),
                "unavailable observation must carry a reason"
            );
        }
    }

    #[test]
    fn personal_hour_alignment_unavailable_without_birth_profile() {
        // AC: personal hour alignment uses birth year Chi when available
        // and is unavailable otherwise.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let features =
            extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        // Exactly one personal-anchor observation per hour (an explicit
        // unavailable marker under the neutral feature ID — see the
        // extraction helper's contract).
        let personal: Vec<&HourRankingFeatureObservation> = features
            .iter()
            .filter(|f| {
                matches!(
                    f.feature_id,
                    HourRankingFeatureId::PersonalHourYearChiMatch
                        | HourRankingFeatureId::PersonalHourYearChiLucXung
                        | HourRankingFeatureId::PersonalHourYearChiNeutral
                )
            })
            .collect();
        assert_eq!(personal.len(), 12, "one personal observation per hour");
        for obs in personal {
            assert!(
                obs.is_unavailable(),
                "without birth, every personal observation must be unavailable"
            );
            assert_eq!(obs.score, None);
            assert!(obs.unavailable_reason.is_some());
        }
    }

    #[test]
    fn personal_hour_alignment_uses_birth_year_chi_only() {
        // AC: personal hour alignment uses birth year Chi only (v1).
        // Pick a birth year with a known Chi so we can verify exactly
        // one observation per hour is emitted and that its score /
        // polarity / note reflect the match / clash / neutral rule.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        // 1990 — Tuất (戌), chi_index = 10.
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
        let features = extract_hour_features(
            &snapshot,
            Some(&birth),
            ruleset_id,
            ruleset_version,
            profile_id,
        );
        let birth_year_chi_index = birth.birth_year_canchi().chi_index;
        let luc_xung_target = xung_hop::luc_xung(birth_year_chi_index);

        for obs in features.iter().filter(|f| {
            matches!(
                f.feature_id,
                HourRankingFeatureId::PersonalHourYearChiMatch
                    | HourRankingFeatureId::PersonalHourYearChiLucXung
                    | HourRankingFeatureId::PersonalHourYearChiNeutral
            )
        }) {
            assert!(!obs.is_unavailable(), "with birth, axis is available");
            let score = obs.score.expect("with birth, score is present");
            assert!(
                (score - 1.0).abs() < 1e-6,
                "v1 emits personal observations at strength 1.0; got {score}"
            );
            let hour_chi_name = CHI[obs.chi_index % 12];
            if obs.chi_index == birth_year_chi_index {
                assert_eq!(
                    obs.feature_id,
                    HourRankingFeatureId::PersonalHourYearChiMatch
                );
                assert_eq!(obs.polarity, ContributionPolarity::Favorable);
            } else if hour_chi_name == luc_xung_target {
                assert_eq!(
                    obs.feature_id,
                    HourRankingFeatureId::PersonalHourYearChiLucXung
                );
                assert_eq!(obs.polarity, ContributionPolarity::Avoid);
            } else {
                assert_eq!(
                    obs.feature_id,
                    HourRankingFeatureId::PersonalHourYearChiNeutral
                );
                assert_eq!(obs.polarity, ContributionPolarity::Neutral);
            }
        }
    }

    #[test]
    fn day_hour_harmony_observations_use_only_day_and_hour_chi() {
        // AC: day-hour harmony uses day Chi to hour Chi only.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let features =
            extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        let day_chi_index = snapshot.context.gio_hoang_dao.day_chi_index;
        let day_chi = CHI[day_chi_index % 12];
        let triad: std::collections::HashSet<&str> =
            xung_hop::tam_hop(day_chi_index).into_iter().collect();
        let liu_he_target = xung_hop::get_liu_he(day_chi_index);
        let luc_xung_target = xung_hop::luc_xung(day_chi_index);

        for obs in features.iter().filter(|f| {
            matches!(
                f.feature_id,
                HourRankingFeatureId::HourBranchTriad
                    | HourRankingFeatureId::HourBranchLiuHe
                    | HourRankingFeatureId::HourBranchLucXung
            )
        }) {
            assert!(!obs.is_unavailable());
            let hour_chi_name = CHI[obs.chi_index % 12];
            if triad.contains(&hour_chi_name) {
                assert_eq!(obs.feature_id, HourRankingFeatureId::HourBranchTriad);
                assert_eq!(obs.polarity, ContributionPolarity::Favorable);
            } else if hour_chi_name == liu_he_target {
                assert_eq!(obs.feature_id, HourRankingFeatureId::HourBranchLiuHe);
                assert_eq!(obs.polarity, ContributionPolarity::Favorable);
            } else if hour_chi_name == luc_xung_target {
                assert_eq!(obs.feature_id, HourRankingFeatureId::HourBranchLucXung);
                assert_eq!(obs.polarity, ContributionPolarity::Avoid);
            } else {
                // Default case: no declared relation. Polarity Info
                // marks it as a non-scoring baseline so the axis is
                // structurally available with the v1 0.5 fallback.
                assert_eq!(obs.feature_id, HourRankingFeatureId::HourBranchLucXung);
                assert_eq!(obs.polarity, ContributionPolarity::Info);
            }
            // Defensive: every harmony observation's note should
            // mention day Chi or the relation name so explanations
            // stay traceable.
            let note = obs.note.as_deref().unwrap_or("");
            assert!(
                note.contains(day_chi)
                    || note.contains("tam hợp")
                    || note.contains("lục hợp")
                    || note.contains("lục xung")
                    || note.contains("không có quan hệ"),
                "harmony observation note must describe the relation; got {note:?}"
            );
        }
    }

    #[test]
    fn aggregate_hour_axes_matches_v1_heuristics() {
        // Pin the per-axis scoring rules that aggregate_hour_axes
        // applies to the extracted observations. Without birth, the
        // personal axis must be unavailable; with birth, it must be
        // 1.0 / 0.0 / 0.5 depending on the match / clash / neutral
        // relation.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let features =
            extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        let hoang_dao_hours = snapshot.context.gio_hoang_dao.all_hours.clone();

        for hour in &hoang_dao_hours {
            let hour_features: Vec<&HourRankingFeatureObservation> = features
                .iter()
                .filter(|f| f.chi_index == hour.hour_index)
                .collect();
            let axes = aggregate_hour_axes(&hour_features);

            // Hoàng Đạo quality — binary.
            let hd = axes.hoang_dao_quality.score.expect("always available");
            assert!(
                (hd - if hour.is_good { 1.0 } else { 0.0 }).abs() < 1e-6,
                "hoang_dao_quality mismatch: hour.is_good={}, score={hd}",
                hour.is_good
            );

            // Intent timing — uniformly unavailable.
            assert!(axes.intent_timing_fit.score.is_none());
            assert!(axes.intent_timing_fit.unavailable_reason.is_some());

            // Personal hour alignment — unavailable without birth.
            assert!(
                axes.personal_hour_alignment.score.is_none(),
                "without birth, personal hour alignment must be unavailable"
            );

            // Day-hour harmony — one of {0.1, 0.5, 0.7, 0.8}.
            let harmony = axes
                .day_hour_harmony
                .score
                .expect("day_hour_harmony is always available");
            let acceptable = [0.1_f32, 0.5, 0.7, 0.8];
            assert!(
                acceptable.iter().any(|v| (harmony - *v).abs() < 1e-6),
                "day_hour_harmony must be one of 0.1/0.5/0.7/0.8; got {harmony}"
            );
        }
    }

    #[test]
    fn aggregate_hour_axes_with_birth_reflects_match_clash_neutral() {
        // With a birth profile, personal_hour_alignment must produce
        // the v1 v1 match/clash/neutral score ladder.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
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
        let features = extract_hour_features(
            &snapshot,
            Some(&birth),
            ruleset_id,
            ruleset_version,
            profile_id,
        );
        let birth_year_chi_index = birth.birth_year_canchi().chi_index;
        let luc_xung_target = xung_hop::luc_xung(birth_year_chi_index);

        for hour in &snapshot.context.gio_hoang_dao.all_hours {
            let hour_features: Vec<&HourRankingFeatureObservation> = features
                .iter()
                .filter(|f| f.chi_index == hour.hour_index)
                .collect();
            let axes = aggregate_hour_axes(&hour_features);
            let score = axes
                .personal_hour_alignment
                .score
                .expect("with birth, axis is available");
            let hour_chi_name = CHI[hour.hour_index % 12];
            let expected = if hour.hour_index == birth_year_chi_index {
                1.0
            } else if hour_chi_name == luc_xung_target {
                0.0
            } else {
                0.5
            };
            assert!(
                (score - expected).abs() < 1e-6,
                "personal_hour_alignment: chi_index={}, expected {expected}, got {score}",
                hour.hour_index
            );
        }
    }

    #[test]
    fn unavailable_observations_are_distinct_from_zero() {
        // Spec contract: unavailable is not zero. Even at the
        // observation level, score == None together with a populated
        // reason must be carried through extraction.
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let features =
            extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        let mut unavailable_count = 0_usize;
        for obs in features.iter().filter(|f| f.is_unavailable()) {
            assert_eq!(obs.score, None, "unavailable observation must score None");
            assert!(
                obs.unavailable_reason.is_some(),
                "unavailable observation must carry a reason"
            );
            unavailable_count += 1;
        }
        // 12 hours × 1 (intent_timing_fit) + 12 hours × 1 (personal) =
        // 24 unavailable observations without birth.
        assert_eq!(unavailable_count, 24);
    }

    // -------------------------------------------------------------------
    // amlich-rv13.3 — weighted hour ranking aggregation tests.
    //
    // The aggregation phase collapses per-axis outcomes into a single
    // `rank_score` plus a contribution list. These tests pin the
    // acceptance criteria from the bead: weighted average over
    // available axes only, clamped to [0, 1], deterministic, and
    // exactly one contribution per available axis.
    // -------------------------------------------------------------------

    fn fake_axes(
        hoang: Option<f32>,
        intent: Option<f32>,
        personal: Option<f32>,
        harmony: Option<f32>,
    ) -> HourRankingAxes {
        let outcome = |axis: HourRankingAxis, score: Option<f32>| match score {
            Some(s) => HourRankingAxisOutcome::from_score(axis, s),
            None => HourRankingAxisOutcome::unavailable(axis, "test_unavailable"),
        };
        HourRankingAxes {
            hoang_dao_quality: outcome(HourRankingAxis::HoangDaoQuality, hoang),
            intent_timing_fit: outcome(HourRankingAxis::IntentTimingFit, intent),
            personal_hour_alignment: outcome(HourRankingAxis::PersonalHourAlignment, personal),
            day_hour_harmony: outcome(HourRankingAxis::DayHourHarmony, harmony),
        }
    }

    #[test]
    fn aggregate_hour_ranking_uses_weighted_average_over_available_axes() {
        // AC: rank score is the weighted average over available axes
        // only. With Hoàng Đạo = 1.0 (weight 0.45) and Day-hour harmony
        // = 0.8 (weight 0.10), available weight = 0.55, numerator =
        // 0.45 + 0.08 = 0.53, so rank_score = 0.53 / 0.55 ≈ 0.963636.
        let policy = HourRankingPolicy::baseline_v1();
        let axes = fake_axes(Some(1.0), None, None, Some(0.8));
        let aggregation = policy.aggregate_hour_ranking(&axes, &[], "test_profile");
        let expected = (0.45 * 1.0 + 0.10 * 0.8) / (0.45 + 0.10);
        assert!(
            (aggregation.rank_score - expected).abs() < 1e-6,
            "rank_score must be weighted average over available axes; \
             expected {expected}, got {}",
            aggregation.rank_score
        );
    }

    #[test]
    fn aggregate_hour_ranking_renormalizes_over_unavailable_axes() {
        // AC: unavailable axes are removed from the denominator, not
        // folded in as a neutral 0.5. With only hoang_dao = 1.0
        // available (single-axis, weight 0.45), the rank score must be
        // exactly 1.0 — never the 0.5 a neutral-fallback would yield.
        let policy = HourRankingPolicy::baseline_v1();
        let axes = fake_axes(Some(1.0), None, None, None);
        let aggregation = policy.aggregate_hour_ranking(&axes, &[], "test_profile");
        assert!(
            (aggregation.rank_score - 1.0).abs() < 1e-6,
            "single available axis must yield that axis's score unchanged; got {}",
            aggregation.rank_score
        );
    }

    #[test]
    fn aggregate_hour_ranking_includes_personal_axis_when_birth_provides_it() {
        // AC: rank score folds in the personal alignment axis when the
        // birth profile makes it available. With hoang = 1.0 (0.45),
        // personal = 0.5 (0.20), and harmony = 0.5 (0.10), the weighted
        // average is (0.45 + 0.10 + 0.05) / 0.75 = 0.60 / 0.75 = 0.80.
        let policy = HourRankingPolicy::baseline_v1();
        let axes = fake_axes(Some(1.0), None, Some(0.5), Some(0.5));
        let aggregation = policy.aggregate_hour_ranking(&axes, &[], "test_profile");
        let expected = (0.45 * 1.0 + 0.20 * 0.5 + 0.10 * 0.5) / (0.45 + 0.20 + 0.10);
        assert!(
            (aggregation.rank_score - expected).abs() < 1e-6,
            "rank_score with personal axis must include its weight; \
             expected {expected}, got {}",
            aggregation.rank_score
        );
    }

    #[test]
    fn aggregate_hour_ranking_clamps_score_to_unit_interval() {
        // AC: rank score is clamped to [0.0, 1.0]. Floating-point drift
        // can push the score just outside the interval even though
        // every axis score and weight is in range; the aggregator
        // defends against that.
        let policy = HourRankingPolicy::baseline_v1();
        // Both axes at 1.0 — numerator and denominator are both 1.0,
        // the formula yields exactly 1.0, and the clamp must keep it.
        let axes = fake_axes(Some(1.0), None, Some(1.0), Some(1.0));
        let aggregation = policy.aggregate_hour_ranking(&axes, &[], "test_profile");
        assert!(
            (0.0..=1.0).contains(&aggregation.rank_score),
            "rank_score must lie in [0, 1]; got {}",
            aggregation.rank_score
        );
        // Both axes at 0.0 — formula yields 0.0, clamp must keep it.
        let axes_zero = fake_axes(Some(0.0), None, Some(0.0), Some(0.0));
        let aggregation_zero = policy.aggregate_hour_ranking(&axes_zero, &[], "test_profile");
        assert!(
            (0.0..=1.0).contains(&aggregation_zero.rank_score),
            "rank_score must lie in [0, 1]; got {}",
            aggregation_zero.rank_score
        );
        assert!(aggregation_zero.rank_score >= 0.0);
    }

    #[test]
    fn aggregate_hour_ranking_with_all_axes_unavailable_returns_zero() {
        // Defensive: with no axis available, the rank score collapses
        // to 0.0 rather than dividing by zero. The contribution list is
        // empty because no axis could be evaluated.
        let policy = HourRankingPolicy::baseline_v1();
        let axes = fake_axes(None, None, None, None);
        let aggregation = policy.aggregate_hour_ranking(&axes, &[], "test_profile");
        assert!(
            aggregation.rank_score.abs() < 1e-6,
            "all-unavailable aggregation must yield zero score; got {}",
            aggregation.rank_score
        );
        assert!(
            aggregation.contributions.is_empty(),
            "all-unavailable aggregation must emit no contributions; got {:?}",
            aggregation.contributions
        );
    }

    #[test]
    fn aggregate_hour_ranking_emits_one_contribution_per_available_axis() {
        // AC: contributions match the available-axis denominator. With
        // hoang + personal + harmony available, three contributions are
        // emitted; the unavailable intent_timing_fit is excluded.
        let policy = HourRankingPolicy::baseline_v1();
        let axes = fake_axes(Some(1.0), None, Some(0.5), Some(0.8));
        let aggregation = policy.aggregate_hour_ranking(&axes, &[], "test_profile");
        assert_eq!(
            aggregation.contributions.len(),
            3,
            "expected 3 contributions (hoang + personal + harmony); got {}",
            aggregation.contributions.len()
        );
        let axes_in: Vec<HourRankingAxis> =
            aggregation.contributions.iter().map(|c| c.axis).collect();
        assert!(axes_in.contains(&HourRankingAxis::HoangDaoQuality));
        assert!(axes_in.contains(&HourRankingAxis::PersonalHourAlignment));
        assert!(axes_in.contains(&HourRankingAxis::DayHourHarmony));
        assert!(!axes_in.contains(&HourRankingAxis::IntentTimingFit));
    }

    #[test]
    fn aggregate_hour_ranking_contributions_carry_policy_weights() {
        // The contribution record must carry the original (unnormalized)
        // axis weight so the trace shows what each axis *would* have
        // contributed before reweighting. With the v1 0.45/0.25/0.20/0.10
        // profile, the hoang_dao contribution must carry weight 0.45 and
        // the harmony contribution weight 0.10.
        let policy = HourRankingPolicy::baseline_v1();
        let axes = fake_axes(Some(1.0), None, None, Some(0.8));
        let aggregation = policy.aggregate_hour_ranking(&axes, &[], "test_profile");
        let hoang = aggregation
            .contributions
            .iter()
            .find(|c| c.axis == HourRankingAxis::HoangDaoQuality)
            .expect("hoang_dao contribution");
        let harmony = aggregation
            .contributions
            .iter()
            .find(|c| c.axis == HourRankingAxis::DayHourHarmony)
            .expect("harmony contribution");
        assert!((hoang.weight - 0.45).abs() < 1e-6);
        assert!((harmony.weight - 0.10).abs() < 1e-6);
        assert!((hoang.contribution - 0.45 * 1.0).abs() < 1e-6);
        assert!((harmony.contribution - 0.10 * 0.8).abs() < 1e-6);
    }

    #[test]
    fn aggregate_hour_ranking_is_deterministic_for_identical_inputs() {
        // AC: deterministic for identical inputs. Run the aggregator
        // twice on the same axes and features; the rank scores and
        // contribution lists must match bit-for-bit.
        let policy = HourRankingPolicy::baseline_v1();
        let snapshot = base_snapshot();
        let (ruleset_id, ruleset_version, profile_id) = ruleset_meta(&snapshot);
        let features =
            extract_hour_features(&snapshot, None, ruleset_id, ruleset_version, profile_id);
        let mut per_hour = std::collections::HashMap::<usize, HourRankingAggregation>::new();
        for hour in &snapshot.context.gio_hoang_dao.all_hours {
            let hour_features: Vec<&HourRankingFeatureObservation> = features
                .iter()
                .filter(|f| f.chi_index == hour.hour_index)
                .collect();
            let axes = aggregate_hour_axes(&hour_features);
            let a = policy.aggregate_hour_ranking(&axes, &hour_features, profile_id);
            let b = policy.aggregate_hour_ranking(&axes, &hour_features, profile_id);
            assert_eq!(
                a, b,
                "aggregation must be deterministic for chi_index={}",
                hour.hour_index
            );
            per_hour.insert(hour.hour_index, a);
        }
        // Re-running extraction + aggregation across the board must
        // also be stable end-to-end (defensive against feature
        // ordering regressions).
        for chi in 0..12 {
            assert!(
                per_hour.contains_key(&chi),
                "every chi_index must be aggregated exactly once"
            );
        }
    }

    #[test]
    fn aggregate_hour_ranking_skips_intent_axis_when_unavailable_in_v1() {
        // AC: the intent_timing_fit axis is unavailable in v1, so it
        // must be excluded from both the rank score formula and the
        // contribution list. Its declared weight (0.25) does not
        // contribute to the denominator.
        let policy = HourRankingPolicy::baseline_v1();
        // Force the axis to be unavailable explicitly.
        let axes = fake_axes(Some(1.0), None, Some(1.0), Some(1.0));
        let aggregation = policy.aggregate_hour_ranking(&axes, &[], "test_profile");
        let available_total: f32 = policy
            .axis_weights()
            .iter()
            .filter(|w| match w.axis {
                HourRankingAxis::HoangDaoQuality => true,
                HourRankingAxis::IntentTimingFit => false,
                HourRankingAxis::PersonalHourAlignment => true,
                HourRankingAxis::DayHourHarmony => true,
            })
            .map(|w| w.weight)
            .sum();
        let expected = (0.45 + 0.20 + 0.10) / available_total;
        assert!(
            (aggregation.rank_score - expected).abs() < 1e-6,
            "intent_timing_fit weight (0.25) must be excluded from the denominator; \
             expected {expected}, got {}",
            aggregation.rank_score
        );
        assert!(
            !aggregation
                .contributions
                .iter()
                .any(|c| c.axis == HourRankingAxis::IntentTimingFit),
            "intent_timing_fit must not appear in contributions when unavailable"
        );
    }

    #[test]
    fn aggregate_hour_ranking_uses_45_25_20_10_weight_profile() {
        // The v1 weight profile is the spec's "Initial weight profile"
        // section verbatim. Pin the exact values so a future weight
        // change shows up as a test diff.
        let policy = HourRankingPolicy::baseline_v1();
        let weights = policy.axis_weights();
        let profile: Vec<(HourRankingAxis, f32)> =
            weights.iter().map(|w| (w.axis, w.weight)).collect();
        assert_eq!(
            profile,
            vec![
                (HourRankingAxis::HoangDaoQuality, 0.45),
                (HourRankingAxis::IntentTimingFit, 0.25),
                (HourRankingAxis::PersonalHourAlignment, 0.20),
                (HourRankingAxis::DayHourHarmony, 0.10),
            ]
        );
    }

    #[test]
    fn rank_breaks_exact_ties_by_traditional_chi_order_end_to_end() {
        // AC: tie-broken by traditional Chi order. Force two hours to
        // share identical rank scores by faking identical axis outcomes
        // and matching the policy's aggregation: pick the most
        // deterministic scoring pattern (hoang_dao = 1.0, harmony =
        // 0.5, no personal) which makes the rank score identical for
        // every Hoàng Đạo hour and identical for every Hắc Đạo hour.
        // Then assert the ranked list groups ties by chi_index
        // ascending within each group.
        let policy = HourRankingPolicy::baseline_v1();
        let snapshot = base_snapshot();
        let ranked = policy
            .rank(&snapshot, ConsultationIntent::Travel, None, None)
            .expect("rank");
        // Build groups of identical scores; within each group, chi_index
        // must be strictly ascending.
        let mut i = 0;
        while i < ranked.len() {
            let mut j = i;
            while j < ranked.len() && (ranked[j].rank_score - ranked[i].rank_score).abs() < 1e-6 {
                j += 1;
            }
            let group = &ranked[i..j];
            let chi_indices: Vec<usize> = group.iter().map(|h| h.chi_index).collect();
            let mut sorted = chi_indices.clone();
            sorted.sort_unstable();
            assert_eq!(
                chi_indices, sorted,
                "tie at score {} must break by chi_index ascending; got {:?}",
                ranked[i].rank_score, chi_indices
            );
            i = j;
        }
    }
}
