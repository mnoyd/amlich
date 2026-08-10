//! Stable feature identifiers and normalized observations for the v2
//! personal-day assessment policy.
//!
//! Source spec: `docs/architecture/personal-day-audit/SCORING-POLICY-V2-SPEC.md`
//!
//! A [`FeatureObservation`] is the canonical, source-attributed input to the
//! versioned [`crate::assessment::AssessmentPolicy`]. Every observation
//! carries:
//!
//! - a stable [`AssessmentFeatureId`] (never reused, never renamed without a
//!   policy version bump);
//! - an explicit [`AvailabilityState`] — *unavailable* is distinct from a
//!   literal zero observation, so a missing input cannot be silently treated
//!   as a neutral signal;
//! - a signed normalized value in `[-1, 1]` (favorable → positive, avoid →
//!   negative) projected from the typed `polarity` + `strength` pair;
//! - full source evidence and ruleset/policy provenance.
//!
//! The signed value is the projection the policy aggregates over; the
//! original `polarity` and `strength` are preserved so the baseline-v2
//! aggregation can reproduce the legacy v1 axis formula exactly while future
//! policy versions (intent-aware weights in `amlich-lxu3`, typed
//! interactions in `amlich-47wn`) can layer new projections on top without
//! rewriting extraction.

use serde::{Deserialize, Serialize};

use crate::assessment::{AssessmentAxis, AvailabilityState, ContributionPolarity, SourceEvidence};

/// Stable, source-attributed identifier for a normalized personal-day
/// feature observation. Identifiers are versioned by the policy that consumes
/// them and never reused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentFeatureId {
    GenericDayQuality,
    IntentFit,
    PersonalSameChi,
    PersonalLucXung,
    PersonalTamHop,
    PersonalLiuHe,
    KuaDirectionMatch,
    TimingHoangDaoRatio,
    AnnualTamTai,
    AnnualKimLau,
    AnnualHoangOc,
    AnnualThaiTue,
    /// Cửu Diệu (Nine Star) personal fortune star flagged as a sao hạn
    /// (`amlich-bz0f.3`). One of the three Hung-quality stars (La Hầu,
    /// Kế Đô, Thái Bạch) maps to an `Avoid` scored feature on the
    /// `AnnualPressure` axis. Trung / Cát stars stay omitted (not
    /// missing evidence, just non-occurring affliction).
    AnnualSaoHan,
    BaziElementResonance,
    /// Target-day stem's Thập Thần relation to the birth day master
    /// (`amlich-bz0f.2`). Fires on every assessment that has a Bazi chart
    /// available; degrades to `Unavailable` when the birth time is unknown.
    BaziTargetDayTenGod,
    /// Target-day branch's xung/hợp/lục-hợp relation to one or more
    /// natal pillars (`amlich-bz0f.2`). Dedupes across pillars: each
    /// relation type fires at most once.
    BaziTargetDayPillarRelation,
    /// Target-day element's resonance (sinh/khắc) with the natal day
    /// master's element (`amlich-bz0f.2`). Distinct from the v2.2
    /// interaction-only [`Self::BaziElementResonance`] in that it
    /// contributes a typed feature observation rather than only feeding
    /// the weak-element interaction.
    BaziTargetDayElementResonance,
    EvidenceCoverage,
}

impl AssessmentFeatureId {
    /// All declared feature identifiers, in canonical order. The order is
    /// stable across policy versions and is used by trace serialization and
    /// parity fixtures.
    pub const ALL: [Self; 18] = [
        Self::GenericDayQuality,
        Self::IntentFit,
        Self::PersonalSameChi,
        Self::PersonalLucXung,
        Self::PersonalTamHop,
        Self::PersonalLiuHe,
        Self::KuaDirectionMatch,
        Self::TimingHoangDaoRatio,
        Self::AnnualTamTai,
        Self::AnnualKimLau,
        Self::AnnualHoangOc,
        Self::AnnualThaiTue,
        Self::AnnualSaoHan,
        Self::BaziElementResonance,
        Self::BaziTargetDayTenGod,
        Self::BaziTargetDayPillarRelation,
        Self::BaziTargetDayElementResonance,
        Self::EvidenceCoverage,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::GenericDayQuality => "generic_day_quality",
            Self::IntentFit => "intent_fit",
            Self::PersonalSameChi => "personal_same_chi",
            Self::PersonalLucXung => "personal_luc_xung",
            Self::PersonalTamHop => "personal_tam_hop",
            Self::PersonalLiuHe => "personal_liu_he",
            Self::KuaDirectionMatch => "kua_direction_match",
            Self::TimingHoangDaoRatio => "timing_hoang_dao_ratio",
            Self::AnnualTamTai => "annual_tam_tai",
            Self::AnnualKimLau => "annual_kim_lau",
            Self::AnnualHoangOc => "annual_hoang_oc",
            Self::AnnualThaiTue => "annual_thai_tue",
            Self::AnnualSaoHan => "annual_sao_han",
            Self::BaziElementResonance => "bazi_element_resonance",
            Self::BaziTargetDayTenGod => "bazi_target_day_ten_god",
            Self::BaziTargetDayPillarRelation => "bazi_target_day_pillar_relation",
            Self::BaziTargetDayElementResonance => "bazi_target_day_element_resonance",
            Self::EvidenceCoverage => "evidence_coverage",
        }
    }

    /// Axis the feature primarily contributes to under `baseline_v2`.
    /// Intent-aware axis weighting (`amlich-lxu3`) and typed interactions
    /// (`amlich-47wn`) layer additional projections on top of this baseline
    /// mapping; the baseline mapping itself is policy-versioned.
    pub fn default_axis(self) -> AssessmentAxis {
        match self {
            Self::GenericDayQuality => AssessmentAxis::GenericDayQuality,
            Self::IntentFit | Self::TimingHoangDaoRatio => AssessmentAxis::IntentFit,
            Self::PersonalSameChi
            | Self::PersonalLucXung
            | Self::PersonalTamHop
            | Self::PersonalLiuHe
            | Self::KuaDirectionMatch
            | Self::BaziElementResonance
            | Self::BaziTargetDayTenGod
            | Self::BaziTargetDayPillarRelation
            | Self::BaziTargetDayElementResonance => AssessmentAxis::PersonalAlignment,
            Self::AnnualTamTai
            | Self::AnnualKimLau
            | Self::AnnualHoangOc
            | Self::AnnualThaiTue
            | Self::AnnualSaoHan => AssessmentAxis::AnnualPressure,
            Self::EvidenceCoverage => AssessmentAxis::EvidenceCoverage,
        }
    }
}

/// One normalized observation of a single feature for a given
/// `(snapshot, profile, intent)` triple.
///
/// `value` (projected via [`Self::signed_value`]) is the canonical aggregated
/// signal: positive for favorable, negative for avoid, in `[-1, 1]`.
/// Unavailable observations carry [`AvailabilityState::Unavailable`] and
/// project `None` — the policy MUST exclude them from the aggregation
/// denominator so a missing input is never confused with a neutral signal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeatureObservation {
    pub feature_id: AssessmentFeatureId,
    pub polarity: ContributionPolarity,
    /// Raw magnitude in `[0.0, 1.0]`. Preserved alongside `polarity` so the
    /// baseline-v2 aggregation can reproduce the legacy v1 axis formula
    /// exactly; future policy versions may re-derive weights from policy
    /// tables and the projected signed value.
    pub strength: f32,
    pub availability: AvailabilityState,
    pub source_evidence: SourceEvidence,
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub contribution_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl FeatureObservation {
    /// Construct an available observation with normalized magnitude and
    /// typed polarity. `strength` is clamped to `[0, 1]`.
    pub fn observed(
        feature_id: AssessmentFeatureId,
        polarity: ContributionPolarity,
        strength: f32,
        contribution_id: impl Into<String>,
        source_evidence: SourceEvidence,
        ruleset_id: impl Into<String>,
        ruleset_version: impl Into<String>,
    ) -> Self {
        Self {
            feature_id,
            polarity,
            strength: strength.clamp(0.0, 1.0),
            availability: AvailabilityState::Complete,
            source_evidence,
            ruleset_id: ruleset_id.into(),
            ruleset_version: ruleset_version.into(),
            contribution_id: contribution_id.into(),
            note: None,
        }
    }

    /// Construct an unavailable observation. Unavailable features are
    /// excluded from aggregation and reported separately; their projected
    /// signed value is `None`.
    pub fn unavailable(
        feature_id: AssessmentFeatureId,
        contribution_id: impl Into<String>,
        reason: impl Into<String>,
        source_evidence: SourceEvidence,
        ruleset_id: impl Into<String>,
        ruleset_version: impl Into<String>,
    ) -> Self {
        Self {
            feature_id,
            polarity: ContributionPolarity::Info,
            strength: 0.0,
            availability: AvailabilityState::Unavailable {
                reason: reason.into(),
            },
            source_evidence,
            ruleset_id: ruleset_id.into(),
            ruleset_version: ruleset_version.into(),
            contribution_id: contribution_id.into(),
            note: None,
        }
    }

    /// True if this observation is unavailable for the current profile /
    /// snapshot. The aggregation pipeline MUST skip unavailable observations
    /// and record them in the trace's unavailable list, never as zero.
    pub fn is_unavailable(&self) -> bool {
        matches!(self.availability, AvailabilityState::Unavailable { .. })
    }

    /// Signed normalized value in `[-1, 1]` projected from `(polarity,
    /// strength)`. Returns `None` when the observation is unavailable.
    ///
    /// The neutral polarity projects a small positive value to mirror the
    /// legacy v1 axis formula under baseline-v2 parity; this is policy-
    /// versioned behavior, not a domain claim about neutrality.
    pub fn signed_value(&self) -> Option<f32> {
        if self.is_unavailable() {
            return None;
        }
        let sign = polarity_sign(self.polarity);
        Some(sign * self.strength)
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Polarity → signed multiplier used to project `(polarity, strength)` into
/// the normalized `[-1, 1]` value space. Kept symmetric with the legacy v1
/// axis aggregation (`amlich-core::assessment::aggregate_axis`) so the v2
/// baseline produces numerical parity; changing this multiplier is a policy-
/// versioned change.
pub(crate) fn polarity_sign(polarity: ContributionPolarity) -> f32 {
    match polarity {
        ContributionPolarity::Favorable => 1.0,
        ContributionPolarity::Avoid => -1.0,
        ContributionPolarity::Neutral => 0.05,
        ContributionPolarity::Info => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn src() -> SourceEvidence {
        SourceEvidence {
            source_family: "test".to_string(),
            source_id: "test".to_string(),
            method: "test".to_string(),
            profile: "test".to_string(),
            note: None,
        }
    }

    #[test]
    fn unavailable_is_distinct_from_zero() {
        let obs = FeatureObservation::unavailable(
            AssessmentFeatureId::BaziElementResonance,
            "bazi.unavailable",
            "no chart",
            src(),
            "ruleset",
            "1.0",
        );
        assert!(obs.is_unavailable());
        assert_eq!(obs.signed_value(), None);
    }

    #[test]
    fn observed_signed_value_respects_polarity_and_strength() {
        let fav = FeatureObservation::observed(
            AssessmentFeatureId::PersonalTamHop,
            ContributionPolarity::Favorable,
            0.4,
            "personal.tam_hop",
            src(),
            "ruleset",
            "1.0",
        );
        assert!(!fav.is_unavailable());
        assert_eq!(fav.signed_value(), Some(0.4));

        let avoid = FeatureObservation::observed(
            AssessmentFeatureId::PersonalLucXung,
            ContributionPolarity::Avoid,
            0.8,
            "personal.luc_xung",
            src(),
            "ruleset",
            "1.0",
        );
        assert_eq!(avoid.signed_value(), Some(-0.8));

        let neutral = FeatureObservation::observed(
            AssessmentFeatureId::PersonalSameChi,
            ContributionPolarity::Neutral,
            1.0,
            "personal.same_chi",
            src(),
            "ruleset",
            "1.0",
        );
        assert_eq!(neutral.signed_value(), Some(0.05));
    }

    #[test]
    fn strength_is_clamped_to_unit_interval() {
        let obs = FeatureObservation::observed(
            AssessmentFeatureId::IntentFit,
            ContributionPolarity::Favorable,
            1.5,
            "intent.test",
            src(),
            "ruleset",
            "1.0",
        );
        assert!((obs.strength - 1.0).abs() < 1e-6);
    }

    #[test]
    fn all_feature_ids_have_stable_string_and_default_axis() {
        for feature in AssessmentFeatureId::ALL {
            assert!(!feature.as_str().is_empty());
            // Every declared feature must map to a real axis under baseline.
            let _ = feature.default_axis();
        }
        assert_eq!(AssessmentFeatureId::ALL.len(), 18);
    }
}
