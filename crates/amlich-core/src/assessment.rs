//! Canonical PersonalDayAssessment — single normalized source of truth for
//! every personal-day projection (advisory, reasoning, matrix, API transport).
//!
//! Source plan: `docs/architecture/personal-day-audit/REPAIR-PLAN.md` P1.1.
//! Bead: `amlich-mwbp.6`.
//!
//! The assessment is built once per request from the normalized day snapshot,
//! the canonical [`crate::BirthProfile`] (with derived [`crate::BirthCapability`]),
//! the consultation intent, and any optional deeper inputs (Bazi chart,
//! analysis, year-hạn, Kua). All downstream projections — advisory scoring,
//! initiation/opening reasoning, matrix report, amlich-api DTOs — consume
//! projections of this object and must not recompute their own verdicts.
//!
//! ## Axes
//!
//! The assessment separates the work into five typed axes before any product
//! verdict is formed:
//!
//! 1. [`AssessmentAxis::GenericDayQuality`] — what the day itself supports,
//!    independent of any personal profile.
//! 2. [`AssessmentAxis::IntentFit`] — how the day's profile fits the
//!    consultation intent.
//! 3. [`AssessmentAxis::PersonalAlignment`] — personal interaction facts
//!    (xung/hợp/tam hợp/cung mệnh) gated on [`crate::BirthCapability`].
//! 4. [`AssessmentAxis::AnnualPressure`] — yearly Hạn / tam tai / kim lâu etc.
//! 5. [`AssessmentAxis::EvidenceCoverage`] — input coverage and ruleset/policy
//!    provenance.
//!
//! Each axis carries an optional 0..=1 normalized score, a verdict, and an
//! `unavailable_reason` when the evidence for that axis is not collected.
//!
//! ## Contribution contract
//!
//! Every [`DecisionContribution`] carries: stable `contribution_id`, axis,
//! polarity, strength, `policy_id` + `policy_version`, `ruleset_id` +
//! `ruleset_version`, source evidence, and an availability state. Stable IDs
//! make the assessment diffable between standalone and aggregate calls — a
//! contract prerequisite for amlich-mwbp.7 (API projection migration).

use serde::{Deserialize, Serialize};

use crate::{
    advisory::ConsultationIntent,
    almanac::{
        recommendation::{DailyRecommendations, RecommendationBucket},
        tu_menh::KuaResult,
        yearly_han::{HanSeverity, YearlyHanAssessment},
    },
    bazi::{analysis::BaziAnalysisReport, types::BaziChart},
    birth::{BirthCapability, BirthDataTier, BirthProfile, BirthTime},
    canchi::get_year_canchi,
    lunar::convert_solar_to_lunar,
    reasoning::{
        DecisionConfidence, InitiationOpeningDecision, ReasoningAxisScore, ReasoningNote,
        RecommendationBucket as ReasoningBucket,
    },
    sources::{SOURCE_KHCBPPT, SOURCE_VN_FOLK},
    DaySnapshot,
};

pub mod extraction;
pub mod feature;
pub mod hour_ranking;
pub mod interactions;
pub mod policy;
pub mod promotion;
pub mod stability;
pub mod trace;
pub mod weights;

pub use feature::{AssessmentFeatureId, FeatureObservation};
pub use interactions::{
    InteractionKind, InteractionWeight, InteractionWeightTable, INTERACTION_WEIGHTS_V2_2,
};
pub use policy::{
    AssessmentPolicy, ASSESSMENT_POLICY_V2_1_VERSION, ASSESSMENT_POLICY_V2_2_VERSION,
    ASSESSMENT_POLICY_V2_ID, ASSESSMENT_POLICY_V2_VERSION,
};
pub use promotion::{current_default_policy_version, PromotionStatus, PromotionStatusReport};
pub use stability::{GateDetail, GateResult, GateStatus, StabilityGate, StabilityReport};
pub use trace::{
    AssessmentTrace, AxisAggregation, AxisContributor, AxisWeight, DecisionAggregation,
    InteractionTerm, VetoEvent,
};
pub use weights::{IntentAxisWeightTable, IntentAxisWeights, INTENT_AXIS_WEIGHTS_V2_1};

/// Stable policy identifier for the personal-day assessment. Co-versioned
/// with [`ASSESSMENT_POLICY_VERSION`]: any change to score combination or
/// confidence derivation MUST bump the version.
pub const ASSESSMENT_POLICY_ID: &str = "personal-day-assessment";
pub const ASSESSMENT_POLICY_VERSION: &str = "v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentAxis {
    GenericDayQuality,
    IntentFit,
    PersonalAlignment,
    AnnualPressure,
    EvidenceCoverage,
}

impl AssessmentAxis {
    pub const ALL: [Self; 5] = [
        Self::GenericDayQuality,
        Self::IntentFit,
        Self::PersonalAlignment,
        Self::AnnualPressure,
        Self::EvidenceCoverage,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::GenericDayQuality => "generic_day_quality",
            Self::IntentFit => "intent_fit",
            Self::PersonalAlignment => "personal_alignment",
            Self::AnnualPressure => "annual_pressure",
            Self::EvidenceCoverage => "evidence_coverage",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributionPolarity {
    Favorable,
    Avoid,
    Neutral,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "state")]
pub enum AvailabilityState {
    Complete,
    Unavailable { reason: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisOutcome {
    pub axis: AssessmentAxis,
    pub score: Option<f32>,
    pub verdict: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
}

impl AxisOutcome {
    fn unavailable(axis: AssessmentAxis, reason: &str) -> Self {
        Self {
            axis,
            score: None,
            verdict: "unavailable".to_string(),
            unavailable_reason: Some(reason.to_string()),
        }
    }

    fn from_score(axis: AssessmentAxis, score: f32) -> Self {
        let score = score.clamp(0.0, 1.0);
        Self {
            axis,
            score: Some(score),
            verdict: classify_score(score),
            unavailable_reason: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NormalizedBirth {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub timezone: f64,
    pub has_time: bool,
    pub has_gender: bool,
    pub has_location: bool,
    pub has_solar_time_policy: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<BirthTime>,
}

impl From<&BirthProfile> for NormalizedBirth {
    fn from(value: &BirthProfile) -> Self {
        Self {
            day: value.day,
            month: value.month,
            year: value.year,
            timezone: value.timezone,
            has_time: value.time.is_some(),
            has_gender: value.gender.is_some(),
            has_location: value.location_name.is_some(),
            has_solar_time_policy: value.longitude.is_some() || value.use_solar_time,
            time: value.time,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssessmentAxes {
    pub generic_day_quality: AxisOutcome,
    pub intent_fit: AxisOutcome,
    pub personal_alignment: AxisOutcome,
    pub annual_pressure: AxisOutcome,
    pub evidence_coverage: AxisOutcome,
}

impl AssessmentAxes {
    pub fn iter(&self) -> impl Iterator<Item = &AxisOutcome> {
        [
            &self.generic_day_quality,
            &self.intent_fit,
            &self.personal_alignment,
            &self.annual_pressure,
            &self.evidence_coverage,
        ]
        .into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceEvidence {
    pub source_family: String,
    pub source_id: String,
    pub method: String,
    pub profile: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionContribution {
    pub contribution_id: String,
    pub axis: AssessmentAxis,
    pub polarity: ContributionPolarity,
    pub strength: f32,
    pub policy_id: String,
    pub policy_version: String,
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub source_evidence: SourceEvidence,
    pub availability: AvailabilityState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalDayDecision {
    pub bucket: ReasoningBucket,
    pub confidence: DecisionConfidence,
    pub semantic: String,
    pub primary_conclusion: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decision_score: Option<f32>,
    pub context_is_clear: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnavailableSection {
    pub section: String,
    pub reason: String,
    pub required_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceCoverage {
    pub has_chart: bool,
    pub has_analysis: bool,
    pub has_yearly_han: bool,
    pub has_kua: bool,
    pub has_kim_lau: bool,
    pub has_tam_tai: bool,
    pub has_hoang_oc: bool,
    pub has_thai_tue: bool,
    pub has_sao_han: bool,
    pub recommendation_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PersonalDayAssessment {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub policy_id: String,
    pub policy_version: String,
    pub intent: ConsultationIntent,
    pub capability: BirthCapability,
    pub capability_tier: BirthDataTier,
    pub normalized_birth: NormalizedBirth,
    pub axes: AssessmentAxes,
    pub contributions: Vec<DecisionContribution>,
    pub decision: PersonalDayDecision,
    pub unavailable_sections: Vec<UnavailableSection>,
    pub evidence: EvidenceCoverage,
    /// Calculation trace emitted by the v2 [`AssessmentPolicy`]. Populated
    /// when the assessment was built via `AssessmentPolicy::evaluate`;
    /// `None` for the legacy v1 builder. The trace is the substrate for
    /// the Evidence Graph projection (`amlich-8tdm`) and is omitted from
    /// serialized output when absent so the v1 wire contract is unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<AssessmentTrace>,
}

#[derive(Debug, Clone, Default)]
pub struct AssessmentInputs {
    pub chart: Option<Result<BaziChart, String>>,
    pub analysis: Option<Result<BaziAnalysisReport, String>>,
    pub yearly_han: Option<Result<YearlyHanAssessment, HanSeverity>>,
    pub kua: Option<Result<KuaResult, String>>,
    pub recommendations: Option<Result<DailyRecommendations, String>>,
}

pub struct PersonalDayAssessmentBuilder {
    snapshot: DaySnapshot,
    profile: BirthProfile,
    intent: ConsultationIntent,
    inputs: AssessmentInputs,
}

impl PersonalDayAssessmentBuilder {
    pub fn new(snapshot: DaySnapshot, profile: BirthProfile, intent: ConsultationIntent) -> Self {
        Self {
            snapshot,
            profile,
            intent,
            inputs: AssessmentInputs::default(),
        }
    }

    pub fn with_chart(mut self, chart: Result<BaziChart, String>) -> Self {
        self.inputs.chart = Some(chart);
        self
    }

    pub fn with_analysis(mut self, analysis: Result<BaziAnalysisReport, String>) -> Self {
        self.inputs.analysis = Some(analysis);
        self
    }

    pub fn with_yearly_han(mut self, yearly_han: Result<YearlyHanAssessment, HanSeverity>) -> Self {
        self.inputs.yearly_han = Some(yearly_han);
        self
    }

    pub fn with_kua(mut self, kua: Result<KuaResult, String>) -> Self {
        self.inputs.kua = Some(kua);
        self
    }

    pub fn with_recommendations(
        mut self,
        recommendations: Result<DailyRecommendations, String>,
    ) -> Self {
        self.inputs.recommendations = Some(recommendations);
        self
    }

    pub fn build(self) -> PersonalDayAssessment {
        let snapshot = self.snapshot;
        let profile = self.profile;
        let intent = self.intent;
        let inputs = self.inputs;
        let capability = profile.capability();
        let capability_tier = capability.default_tier();
        let normalized_birth = NormalizedBirth::from(&profile);

        let ruleset_id = snapshot.ruleset_id.clone();
        let ruleset_version = snapshot.ruleset_version.clone();
        let profile_id = snapshot.profile.clone();

        // Resolve upstream signals through the shared seam so the legacy
        // v1 builder and the v2 [`AssessmentPolicy`] feed byte-identical
        // inputs into the assessment pipeline (amlich-mwbp.6 parity).
        let resolved =
            extraction::resolve_assessment_inputs(&snapshot, &profile, capability, inputs);
        let chart = resolved.chart;
        let analysis = resolved.analysis;
        let yearly_han = resolved.yearly_han;
        let kua = resolved.kua;
        let recommendations = resolved.recommendations;

        // --- Coverage flags ---
        let evidence = EvidenceCoverage {
            has_chart: chart.is_some(),
            has_analysis: analysis.is_some(),
            has_yearly_han: yearly_han.is_some(),
            has_kua: kua.is_some(),
            has_kim_lau: yearly_han
                .as_ref()
                .map(|h| h.kim_lau.in_kim_lau)
                .unwrap_or(false),
            has_tam_tai: yearly_han
                .as_ref()
                .map(|h| h.tam_tai.in_tam_tai)
                .unwrap_or(false),
            has_hoang_oc: yearly_han.is_some(),
            has_thai_tue: yearly_han
                .as_ref()
                .map(|h| h.thai_tue.has_conflict)
                .unwrap_or(false),
            has_sao_han: yearly_han
                .as_ref()
                .map(|h| h.sao_han.is_han)
                .unwrap_or(false),
            recommendation_count: recommendations
                .as_ref()
                .map(|r| r.activities.len())
                .unwrap_or(0),
        };

        // --- Contributions ---
        let mut contributions: Vec<DecisionContribution> = Vec::new();
        let mut unavailable_sections: Vec<UnavailableSection> = Vec::new();

        if let Some(rec) = recommendations.as_ref() {
            for activity in &rec.activities {
                let polarity = match activity.bucket {
                    RecommendationBucket::Nen => ContributionPolarity::Favorable,
                    RecommendationBucket::CoThe => ContributionPolarity::Neutral,
                    RecommendationBucket::Tranh => ContributionPolarity::Avoid,
                    RecommendationBucket::KyManh => ContributionPolarity::Avoid,
                };
                let strength = match activity.bucket {
                    RecommendationBucket::Nen => 0.7,
                    RecommendationBucket::CoThe => 0.4,
                    RecommendationBucket::Tranh => 0.6,
                    RecommendationBucket::KyManh => 0.9,
                };
                contributions.push(DecisionContribution {
                    contribution_id: format!(
                        "rec.{}.{}",
                        activity.activity_id.as_str(),
                        snapshot.context.solar.day
                    ),
                    axis: AssessmentAxis::GenericDayQuality,
                    polarity,
                    strength,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: rec.ruleset_id.clone(),
                    ruleset_version: rec.ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "almanac_rule".to_string(),
                        source_id: SOURCE_KHCBPPT.to_string(),
                        method: "recommendation_synthesis".to_string(),
                        profile: rec.profile.clone(),
                        note: None,
                    },
                    availability: AvailabilityState::Complete,
                    note: Some(activity.label.vi.clone()),
                });
            }

            if let Some(primary) = rec
                .activities
                .iter()
                .find(|a| a.activity_id == intent.primary_activity())
            {
                let polarity = match primary.bucket {
                    RecommendationBucket::Nen => ContributionPolarity::Favorable,
                    RecommendationBucket::CoThe => ContributionPolarity::Neutral,
                    RecommendationBucket::Tranh => ContributionPolarity::Avoid,
                    RecommendationBucket::KyManh => ContributionPolarity::Avoid,
                };
                let strength = match primary.bucket {
                    RecommendationBucket::Nen => 0.8,
                    RecommendationBucket::CoThe => 0.5,
                    RecommendationBucket::Tranh => 0.7,
                    RecommendationBucket::KyManh => 1.0,
                };
                contributions.push(DecisionContribution {
                    contribution_id: format!("intent.{}", intent.event_kind()),
                    axis: AssessmentAxis::IntentFit,
                    polarity,
                    strength,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: rec.ruleset_id.clone(),
                    ruleset_version: rec.ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "almanac_rule".to_string(),
                        source_id: SOURCE_KHCBPPT.to_string(),
                        method: "intent_fit_lookup".to_string(),
                        profile: rec.profile.clone(),
                        note: Some(format!("intent={}", intent.event_kind())),
                    },
                    availability: AvailabilityState::Complete,
                    note: Some(primary.label.vi.clone()),
                });
            }
        }

        if !snapshot.day_fortune.taboos.is_empty() {
            let taboo_count = snapshot.day_fortune.taboos.len();
            let strength = (taboo_count.min(3) as f32) / 3.0 * 0.6 + 0.2;
            contributions.push(DecisionContribution {
                contribution_id: format!("day_fortune.taboo.{}", snapshot.context.solar.day),
                axis: AssessmentAxis::GenericDayQuality,
                polarity: ContributionPolarity::Avoid,
                strength,
                policy_id: ASSESSMENT_POLICY_ID.to_string(),
                policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                ruleset_id: ruleset_id.clone(),
                ruleset_version: ruleset_version.clone(),
                source_evidence: SourceEvidence {
                    source_family: "almanac_rule".to_string(),
                    source_id: SOURCE_KHCBPPT.to_string(),
                    method: "day_fortune.taboos".to_string(),
                    profile: profile_id.clone(),
                    note: Some(format!("count={taboo_count}")),
                },
                availability: AvailabilityState::Complete,
                note: None,
            });
        }

        if capability.has_gender {
            let birth_year = get_year_canchi(
                convert_solar_to_lunar(profile.day, profile.month, profile.year, profile.timezone)
                    .year,
            );
            let day_chi = snapshot.context.canchi.day.chi.as_str();
            let xung_hop = &snapshot.day_fortune.xung_hop;
            if birth_year.chi == day_chi {
                contributions.push(DecisionContribution {
                    contribution_id: "personal.same_chi".to_string(),
                    axis: AssessmentAxis::PersonalAlignment,
                    polarity: ContributionPolarity::Neutral,
                    strength: 0.3,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: ruleset_id.clone(),
                    ruleset_version: ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "interaction".to_string(),
                        source_id: SOURCE_KHCBPPT.to_string(),
                        method: "day_chi_eq_year_chi".to_string(),
                        profile: profile_id.clone(),
                        note: None,
                    },
                    availability: AvailabilityState::Complete,
                    note: None,
                });
            } else if xung_hop.luc_xung == birth_year.chi {
                contributions.push(DecisionContribution {
                    contribution_id: "personal.luc_xung".to_string(),
                    axis: AssessmentAxis::PersonalAlignment,
                    polarity: ContributionPolarity::Avoid,
                    strength: 0.8,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: ruleset_id.clone(),
                    ruleset_version: ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "interaction".to_string(),
                        source_id: SOURCE_KHCBPPT.to_string(),
                        method: "luc_xung_lookup".to_string(),
                        profile: profile_id.clone(),
                        note: None,
                    },
                    availability: AvailabilityState::Complete,
                    note: None,
                });
            } else if xung_hop.tam_hop.iter().any(|c| c == &birth_year.chi) {
                contributions.push(DecisionContribution {
                    contribution_id: "personal.tam_hop".to_string(),
                    axis: AssessmentAxis::PersonalAlignment,
                    polarity: ContributionPolarity::Favorable,
                    strength: 0.4,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: ruleset_id.clone(),
                    ruleset_version: ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "interaction".to_string(),
                        source_id: SOURCE_KHCBPPT.to_string(),
                        method: "tam_hop_lookup".to_string(),
                        profile: profile_id.clone(),
                        note: None,
                    },
                    availability: AvailabilityState::Complete,
                    note: None,
                });
            } else if xung_hop.liu_he.as_deref() == Some(birth_year.chi.as_str()) {
                contributions.push(DecisionContribution {
                    contribution_id: "personal.liu_he".to_string(),
                    axis: AssessmentAxis::PersonalAlignment,
                    polarity: ContributionPolarity::Favorable,
                    strength: 0.3,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: ruleset_id.clone(),
                    ruleset_version: ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "interaction".to_string(),
                        source_id: SOURCE_KHCBPPT.to_string(),
                        method: "liu_he_lookup".to_string(),
                        profile: profile_id.clone(),
                        note: None,
                    },
                    availability: AvailabilityState::Complete,
                    note: None,
                });
            }
        } else {
            unavailable_sections.push(unavailable_section(
                "personal_alignment",
                "requires gender for personal interaction facts",
                &["gender"],
            ));
        }

        if let Some(kua_result) = kua.as_ref() {
            let xuat_hanh = &snapshot.day_fortune.travel.xuat_hanh_huong;
            if kua_result
                .favorable_directions
                .iter()
                .any(|d| d.as_vn_str() == xuat_hanh.as_str())
            {
                contributions.push(DecisionContribution {
                    contribution_id: "personal.kua_favorable".to_string(),
                    axis: AssessmentAxis::PersonalAlignment,
                    polarity: ContributionPolarity::Favorable,
                    strength: 0.4,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: ruleset_id.clone(),
                    ruleset_version: ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "interaction".to_string(),
                        source_id: SOURCE_VN_FOLK.to_string(),
                        method: "kua_favorable_match".to_string(),
                        profile: profile_id.clone(),
                        note: Some(format!("kua={} direction={}", kua_result.kua, xuat_hanh)),
                    },
                    availability: AvailabilityState::Complete,
                    note: None,
                });
            } else if kua_result
                .unfavorable_directions
                .iter()
                .any(|d| d.as_vn_str() == xuat_hanh.as_str())
            {
                contributions.push(DecisionContribution {
                    contribution_id: "personal.kua_unfavorable".to_string(),
                    axis: AssessmentAxis::PersonalAlignment,
                    polarity: ContributionPolarity::Avoid,
                    strength: 0.4,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: ruleset_id.clone(),
                    ruleset_version: ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "interaction".to_string(),
                        source_id: SOURCE_VN_FOLK.to_string(),
                        method: "kua_unfavorable_match".to_string(),
                        profile: profile_id.clone(),
                        note: Some(format!("kua={} direction={}", kua_result.kua, xuat_hanh)),
                    },
                    availability: AvailabilityState::Complete,
                    note: None,
                });
            }
        }

        if capability.has_time {
            let good = snapshot.context.gio_hoang_dao.good_hour_count as f32;
            let total = snapshot.context.gio_hoang_dao.all_hours.len() as f32;
            let ratio = if total > 0.0 { good / total } else { 0.0 };
            contributions.push(DecisionContribution {
                contribution_id: "timing.hoang_dao_ratio".to_string(),
                axis: AssessmentAxis::IntentFit,
                polarity: if ratio >= 0.4 {
                    ContributionPolarity::Favorable
                } else {
                    ContributionPolarity::Neutral
                },
                strength: ratio,
                policy_id: ASSESSMENT_POLICY_ID.to_string(),
                policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                ruleset_id: ruleset_id.clone(),
                ruleset_version: ruleset_version.clone(),
                source_evidence: SourceEvidence {
                    source_family: "almanac_rule".to_string(),
                    source_id: SOURCE_KHCBPPT.to_string(),
                    method: "gio_hoang_dao_ratio".to_string(),
                    profile: profile_id.clone(),
                    note: Some(format!("good={} total={}", good as u32, total as u32)),
                },
                availability: AvailabilityState::Complete,
                note: None,
            });
        } else {
            unavailable_sections.push(unavailable_section(
                "personal_hours",
                "requires explicit birth time for personal-hour context",
                &["hour", "minute"],
            ));
        }

        if let Some(han) = yearly_han.as_ref() {
            if han.han_count > 0 {
                let severity_strength = match han.severity {
                    HanSeverity::Low => 0.3,
                    HanSeverity::Medium => 0.55,
                    HanSeverity::High => 0.85,
                    HanSeverity::Critical => 1.0,
                };
                contributions.push(DecisionContribution {
                    contribution_id: format!("annual.han.{}", snapshot.context.solar.day),
                    axis: AssessmentAxis::AnnualPressure,
                    polarity: ContributionPolarity::Avoid,
                    strength: severity_strength,
                    policy_id: ASSESSMENT_POLICY_ID.to_string(),
                    policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
                    ruleset_id: ruleset_id.clone(),
                    ruleset_version: ruleset_version.clone(),
                    source_evidence: SourceEvidence {
                        source_family: "almanac_rule".to_string(),
                        source_id: SOURCE_KHCBPPT.to_string(),
                        method: "yearly_han".to_string(),
                        profile: profile_id.clone(),
                        note: Some(format!(
                            "count={} severity={:?}",
                            han.han_count, han.severity
                        )),
                    },
                    availability: AvailabilityState::Complete,
                    note: None,
                });
            }
        } else {
            unavailable_sections.push(unavailable_section(
                "annual_han",
                "requires gender for yearly Hạn assessment",
                &["gender"],
            ));
        }

        // --- Axes ---
        let generic_score = aggregate_axis(
            AssessmentAxis::GenericDayQuality,
            contributions
                .iter()
                .filter(|c| c.axis == AssessmentAxis::GenericDayQuality),
        );
        let intent_score = aggregate_axis(
            AssessmentAxis::IntentFit,
            contributions
                .iter()
                .filter(|c| c.axis == AssessmentAxis::IntentFit),
        );
        let personal_score = if capability.has_gender {
            aggregate_axis(
                AssessmentAxis::PersonalAlignment,
                contributions
                    .iter()
                    .filter(|c| c.axis == AssessmentAxis::PersonalAlignment),
            )
        } else {
            AxisOutcome::unavailable(
                AssessmentAxis::PersonalAlignment,
                "requires gender for personal interaction facts",
            )
        };
        let annual_score = if yearly_han.is_some() {
            aggregate_axis(
                AssessmentAxis::AnnualPressure,
                contributions
                    .iter()
                    .filter(|c| c.axis == AssessmentAxis::AnnualPressure),
            )
        } else {
            AxisOutcome::unavailable(
                AssessmentAxis::AnnualPressure,
                "requires gender for yearly Hạn assessment",
            )
        };

        let coverage_fields = [
            capability.has_date,
            capability.has_time,
            capability.has_gender,
            capability.has_location,
        ];
        let coverage_count = coverage_fields.iter().filter(|v| **v).count() as f32;
        let coverage_score = coverage_count / 4.0;
        let evidence_axis =
            AxisOutcome::from_score(AssessmentAxis::EvidenceCoverage, coverage_score);

        let axes = AssessmentAxes {
            generic_day_quality: generic_score,
            intent_fit: intent_score,
            personal_alignment: personal_score,
            annual_pressure: annual_score,
            evidence_coverage: evidence_axis,
        };

        let (bucket, confidence, semantic, score) = synthesize_decision(
            &axes,
            &contributions,
            &capability,
            recommendations.as_ref(),
            intent,
        );

        let decision = PersonalDayDecision {
            bucket,
            confidence,
            semantic,
            primary_conclusion: synthesize_primary_conclusion(bucket, &axes, intent),
            decision_score: score,
            context_is_clear: evidence.has_chart || evidence.has_yearly_han,
        };

        PersonalDayAssessment {
            ruleset_id,
            ruleset_version,
            profile: profile_id,
            policy_id: ASSESSMENT_POLICY_ID.to_string(),
            policy_version: ASSESSMENT_POLICY_VERSION.to_string(),
            intent,
            capability,
            capability_tier,
            normalized_birth,
            axes,
            contributions,
            decision,
            unavailable_sections,
            evidence,
            trace: None,
        }
    }
}

fn synthesize_decision(
    axes: &AssessmentAxes,
    contributions: &[DecisionContribution],
    capability: &BirthCapability,
    recommendations: Option<&DailyRecommendations>,
    intent: ConsultationIntent,
) -> (ReasoningBucket, DecisionConfidence, String, Option<f32>) {
    let hard_veto = contributions
        .iter()
        .any(|c| matches!(c.polarity, ContributionPolarity::Avoid) && c.strength >= 0.8);
    if hard_veto {
        return (
            ReasoningBucket::Avoid,
            confidence_from_capability(capability),
            "override_avoid".to_string(),
            Some(0.15),
        );
    }

    let available: Vec<f32> = [
        axes.generic_day_quality.score,
        axes.intent_fit.score,
        axes.personal_alignment.score,
        axes.annual_pressure.score,
    ]
    .into_iter()
    .flatten()
    .collect();

    let score = if available.is_empty() {
        None
    } else {
        let sum: f32 = available.iter().sum();
        Some(sum / available.len() as f32)
    };

    if let Some(rec) = recommendations {
        if let Some(primary) = rec
            .activities
            .iter()
            .find(|a| a.activity_id == intent.primary_activity())
        {
            if matches!(primary.bucket, RecommendationBucket::KyManh) {
                return (
                    ReasoningBucket::Avoid,
                    confidence_from_capability(capability),
                    "override_avoid".to_string(),
                    Some(0.2),
                );
            }
            if matches!(primary.bucket, RecommendationBucket::Tranh) {
                return (
                    ReasoningBucket::Cautious,
                    confidence_from_capability(capability),
                    "resistance_led_cautious".to_string(),
                    Some(score.unwrap_or(0.4)),
                );
            }
        }
    }

    let bucket = match score.unwrap_or(0.5) {
        s if s >= 0.7 => ReasoningBucket::Favorable,
        s if s >= 0.45 => ReasoningBucket::Mixed,
        s if s >= 0.3 => ReasoningBucket::Cautious,
        _ => ReasoningBucket::Avoid,
    };
    let semantic = match bucket {
        ReasoningBucket::Favorable => "favorable_clear".to_string(),
        ReasoningBucket::Mixed => "favorable_contextual".to_string(),
        ReasoningBucket::Cautious => "resistance_led_cautious".to_string(),
        ReasoningBucket::Avoid => "conflicted_cautious".to_string(),
    };
    (
        bucket,
        confidence_from_capability(capability),
        semantic,
        score.map(|s| s.clamp(0.0, 1.0)),
    )
}

fn confidence_from_capability(cap: &BirthCapability) -> DecisionConfidence {
    let score = (cap.has_date as u8 as i32)
        + (cap.has_time as u8 as i32)
        + (cap.has_gender as u8 as i32)
        + (cap.has_location as u8 as i32);
    match score {
        4..=i32::MAX => DecisionConfidence::High,
        3 => DecisionConfidence::Medium,
        _ => DecisionConfidence::Low,
    }
}

fn synthesize_primary_conclusion(
    bucket: ReasoningBucket,
    axes: &AssessmentAxes,
    intent: ConsultationIntent,
) -> String {
    let tail = axes
        .personal_alignment
        .score
        .map(|_| "đã đối chiếu theo tuổi và hướng cung mệnh.")
        .unwrap_or("chưa đối chiếu cá nhân hóa — bổ sung ngày sinh và giới tính để cá nhân hóa.");
    match bucket {
        ReasoningBucket::Favorable => {
            format!("Ngày phù hợp cho {}; {}", intent.event_kind(), tail)
        }
        ReasoningBucket::Mixed => format!(
            "Ngày có thể phù hợp cho {} ở mức trung bình; {}",
            intent.event_kind(),
            tail
        ),
        ReasoningBucket::Cautious => {
            format!("Cần thận trọng cho {}; {}", intent.event_kind(), tail)
        }
        ReasoningBucket::Avoid => format!("Không nên {} hôm nay; {}", intent.event_kind(), tail),
    }
}

fn aggregate_axis<'a, I>(axis: AssessmentAxis, items: I) -> AxisOutcome
where
    I: IntoIterator<Item = &'a DecisionContribution>,
{
    let items: Vec<&'a DecisionContribution> = items.into_iter().collect();
    if items.is_empty() {
        return AxisOutcome {
            axis,
            score: Some(0.5),
            verdict: classify_score(0.5),
            unavailable_reason: None,
        };
    }
    let mut total_weight = 0.0_f32;
    let mut delta = 0.0_f32;
    for c in &items {
        let sign = match c.polarity {
            ContributionPolarity::Favorable => 1.0,
            ContributionPolarity::Avoid => -1.0,
            ContributionPolarity::Neutral => 0.05,
            ContributionPolarity::Info => 0.0,
        };
        delta += sign * c.strength * 0.3;
        total_weight += c.strength;
    }
    let balance = if total_weight > 0.0 {
        (delta / total_weight).clamp(-0.5, 0.5)
    } else {
        0.0
    };
    let score = (0.5 + balance).clamp(0.0, 1.0);
    AxisOutcome::from_score(axis, score)
}

fn classify_score(score: f32) -> String {
    let s = score.clamp(0.0, 1.0);
    if s >= 0.7 {
        "favorable".to_string()
    } else if s >= 0.45 {
        "mixed".to_string()
    } else if s >= 0.3 {
        "cautious".to_string()
    } else {
        "avoid".to_string()
    }
}

fn unavailable_section(
    section: &str,
    reason: &str,
    required_fields: &[&str],
) -> UnavailableSection {
    UnavailableSection {
        section: section.to_string(),
        reason: reason.to_string(),
        required_fields: required_fields.iter().map(|f| f.to_string()).collect(),
    }
}

impl PersonalDayAssessment {
    pub fn assess(
        snapshot: DaySnapshot,
        profile: BirthProfile,
        intent: ConsultationIntent,
    ) -> Self {
        crate::build_count::canonical_assessment_built();
        PersonalDayAssessmentBuilder::new(snapshot, profile, intent).build()
    }

    /// Like [`assess`](Self::assess) but accepts a precomputed Kua so the
    /// assessment path does not independently recompute it. The personal-day
    /// report path threads a single Kua through the assessment, the facts
    /// bundle, and the Tu Menh insight so `build_count::kua_computations`
    /// stays at one per request — see `amlich-efkp`. `KuaResult` is small and
    /// [`Clone`], so the seam takes it by reference.
    pub fn assess_with_kua(
        snapshot: DaySnapshot,
        profile: BirthProfile,
        intent: ConsultationIntent,
        kua: Option<&KuaResult>,
    ) -> Self {
        crate::build_count::canonical_assessment_built();
        let builder = PersonalDayAssessmentBuilder::new(snapshot, profile, intent);
        let builder = match kua {
            Some(k) => builder.with_kua(Ok(k.clone())),
            None => builder,
        };
        builder.build()
    }

    pub fn project_to_initiation_opening_decision(&self) -> InitiationOpeningDecision {
        let primary = self.decision.primary_conclusion.clone();
        let bucket = self.decision.bucket;
        let supports: Vec<String> = self
            .contributions
            .iter()
            .filter(|c| matches!(c.polarity, ContributionPolarity::Favorable))
            .filter_map(|c| c.note.clone())
            .take(3)
            .collect();
        let resistances: Vec<String> = self
            .contributions
            .iter()
            .filter(|c| matches!(c.polarity, ContributionPolarity::Avoid))
            .filter_map(|c| c.note.clone())
            .take(3)
            .collect();
        let conflicts: Vec<String> = self
            .contributions
            .iter()
            .filter(|c| matches!(c.polarity, ContributionPolarity::Avoid) && c.strength >= 0.6)
            .map(|c| format!("{} ({:.2})", c.contribution_id, c.strength))
            .collect();
        InitiationOpeningDecision {
            primary_conclusion: primary,
            recommendation_bucket: bucket,
            strongest_supports: supports,
            strongest_resistances: resistances,
            override_factors: self
                .contributions
                .iter()
                .filter(|c| matches!(c.polarity, ContributionPolarity::Avoid) && c.strength >= 0.8)
                .map(|c| format!("{} ({:.2})", c.contribution_id, c.strength))
                .collect(),
            conflict_notes: conflicts,
            confidence: self.decision.confidence,
            context_is_clear: self.decision.context_is_clear,
            suggested_hours: Vec::new(),
            suggested_directions: Vec::new(),
        }
    }

    pub fn axis_scores(&self) -> Vec<ReasoningAxisScore> {
        use crate::reasoning::InterpretedAxis;
        let axes = &self.axes;

        let pick_strongest = |outcome: &AxisOutcome| -> (Option<String>, Option<String>) {
            // Find the highest-strength contribution for the matched
            // assessment axis. This keeps graph provenance on the export
            // while honouring the canonical assessment contract.
            let target_axis = outcome.axis;
            let best = self
                .contributions
                .iter()
                .filter(|c| c.axis == target_axis)
                .max_by(|left, right| {
                    left.strength
                        .partial_cmp(&right.strength)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            match best {
                Some(contrib) => (
                    Some(contrib.contribution_id.clone()),
                    Some(
                        contrib
                            .note
                            .clone()
                            .unwrap_or_else(|| contrib.contribution_id.clone()),
                    ),
                ),
                None => (None, None),
            }
        };

        let pairs: [(InterpretedAxis, &AxisOutcome); 6] = [
            (InterpretedAxis::Support, &axes.generic_day_quality),
            (InterpretedAxis::Resistance, &axes.annual_pressure),
            (InterpretedAxis::Stability, &axes.intent_fit),
            (InterpretedAxis::PersonalAlignment, &axes.personal_alignment),
            (InterpretedAxis::TimingFit, &axes.intent_fit),
            (InterpretedAxis::ContextClarity, &axes.evidence_coverage),
        ];
        pairs
            .into_iter()
            .map(|(axis, outcome)| {
                let (strongest_node_id, strongest_summary_vi) = pick_strongest(outcome);
                ReasoningAxisScore {
                    axis,
                    score: outcome.score.unwrap_or(0.5),
                    strongest_node_id,
                    strongest_summary_vi,
                }
            })
            .collect()
    }

    pub fn strongest_reasoning_notes(&self, polarity: ContributionPolarity) -> Vec<ReasoningNote> {
        self.contributions
            .iter()
            .filter(|c| c.polarity == polarity)
            .take(5)
            .map(|c| ReasoningNote {
                node_id: Some(c.contribution_id.clone()),
                summary_vi: c.note.clone().unwrap_or_else(|| c.contribution_id.clone()),
                tags: vec![c.axis.as_str().to_string()],
                provenance: vec![crate::reasoning::ReasoningEvidenceEnvelope {
                    source_family: crate::reasoning::ReasoningEvidenceSourceFamily::AlmanacRule,
                    source_id: c.source_evidence.source_id.clone(),
                    method: c.source_evidence.method.clone(),
                    note: c.source_evidence.note.clone(),
                }],
            })
            .collect()
    }
}

pub fn assess_personal_day(
    snapshot: DaySnapshot,
    profile: BirthProfile,
    intent: ConsultationIntent,
) -> PersonalDayAssessment {
    PersonalDayAssessment::assess(snapshot, profile, intent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VIETNAM_TIMEZONE;

    fn base_snapshot() -> DaySnapshot {
        crate::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE)
    }

    fn base_profile() -> BirthProfile {
        BirthProfile {
            day: 1,
            month: 1,
            year: 1990,
            time: None,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(crate::almanac::tu_menh::Gender::Male),
            location_name: None,
        }
    }

    #[test]
    fn assess_builds_five_axes_in_stable_order() {
        let assessment = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::ContractSigning,
        );

        let names: Vec<&str> = assessment.axes.iter().map(|a| a.axis.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "generic_day_quality",
                "intent_fit",
                "personal_alignment",
                "annual_pressure",
                "evidence_coverage",
            ]
        );
    }

    #[test]
    fn assess_carries_policy_and_ruleset_metadata() {
        let assessment = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::Wedding,
        );

        assert_eq!(assessment.policy_id, ASSESSMENT_POLICY_ID);
        assert_eq!(assessment.policy_version, ASSESSMENT_POLICY_VERSION);
        assert_eq!(assessment.ruleset_id, base_snapshot().ruleset_id);
        assert_eq!(assessment.ruleset_version, base_snapshot().ruleset_version);
    }

    #[test]
    fn assess_confidence_derives_from_capability_not_birth_presence() {
        let rich = {
            let mut p = base_profile();
            p.time = Some(BirthTime {
                hour: 9,
                minute: 30,
            });
            p.longitude = Some(105.85);
            p.use_solar_time = true;
            p.location_name = Some("Hanoi".to_string());
            p
        };
        let rich_assessment =
            PersonalDayAssessment::assess(base_snapshot(), rich, ConsultationIntent::Wedding);
        assert!(matches!(
            rich_assessment.decision.confidence,
            DecisionConfidence::High | DecisionConfidence::Medium
        ));

        let sparse = BirthProfile {
            gender: None,
            ..base_profile()
        };
        let sparse_assessment =
            PersonalDayAssessment::assess(base_snapshot(), sparse, ConsultationIntent::Wedding);
        assert_eq!(
            sparse_assessment.decision.confidence,
            DecisionConfidence::Low
        );
    }

    #[test]
    fn unknown_time_profile_omits_chart_and_personal_hour_axis() {
        let assessment = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::Wedding,
        );
        assert!(!assessment.evidence.has_chart);
        assert!(assessment
            .unavailable_sections
            .iter()
            .any(|u| u.section == "personal_hours"));
    }

    #[test]
    fn identical_inputs_produce_identical_assessments() {
        let a = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::Travel,
        );
        let b = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::Travel,
        );
        assert_eq!(a.axes, b.axes);
        assert_eq!(a.decision, b.decision);
        assert_eq!(a.evidence, b.evidence);
    }

    #[test]
    fn full_solar_profile_distinct_from_unknown_time() {
        let unknown = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::Travel,
        );
        let known = {
            let mut p = base_profile();
            p.time = Some(BirthTime {
                hour: 23,
                minute: 0,
            });
            p
        };
        let known_assessment =
            PersonalDayAssessment::assess(base_snapshot(), known, ConsultationIntent::Travel);

        assert_ne!(
            unknown.evidence.has_chart,
            known_assessment.evidence.has_chart
        );
        assert!(!unknown.evidence.has_chart);
        assert!(known_assessment.evidence.has_chart);
    }

    #[test]
    fn intent_fit_axis_reacts_to_intent_change() {
        let a = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::Wedding,
        );
        let b = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::Burial,
        );
        // Intent-fit contribution is named after the intent itself, so the
        // contribution set MUST change even when both intents land in the
        // same high-level bucket. The axis score is allowed to stay equal
        // only when both intents resolve to the same primary bucket; we
        // only assert the contribution-id split.
        let a_intent_contrib = a
            .contributions
            .iter()
            .find(|c| c.axis == AssessmentAxis::IntentFit)
            .map(|c| c.contribution_id.clone())
            .expect("intent_fit contribution");
        let b_intent_contrib = b
            .contributions
            .iter()
            .find(|c| c.axis == AssessmentAxis::IntentFit)
            .map(|c| c.contribution_id.clone())
            .expect("intent_fit contribution");
        assert_ne!(a_intent_contrib, b_intent_contrib);
    }

    #[test]
    fn classification_in_all_four_buckets() {
        let cases: [(f32, &str); 4] = [
            (0.95, "favorable"),
            (0.55, "mixed"),
            (0.35, "cautious"),
            (0.05, "avoid"),
        ];
        for (score, expected) in cases {
            assert_eq!(classify_score(score), expected);
        }
    }

    #[test]
    fn assessments_share_inputs_across_calls_parity_smoke() {
        let a = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::ContractSigning,
        );
        let b = PersonalDayAssessment::assess(
            base_snapshot(),
            base_profile(),
            ConsultationIntent::ContractSigning,
        );
        assert_eq!(a.ruleset_id, b.ruleset_id);
        assert_eq!(a.policy_id, b.policy_id);
        assert_eq!(a.contributions, b.contributions);
        assert_eq!(a.axes, b.axes);
        assert_eq!(a.decision, b.decision);
    }
}
