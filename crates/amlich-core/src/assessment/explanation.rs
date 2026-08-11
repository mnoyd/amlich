//! Consumer-facing explanation projection (`amlich-bz0f.6`).
//!
//! The personal-day, hour-ranking, and direction assessment pipelines emit
//! raw observation traces: feature lists, axis aggregations, vetoes, dedup'd
//! contributions, and a capability-derived decision confidence. The
//! [`AssessmentTrace`], [`crate::assessment::direction::DirectionAssessment`],
//! and [`crate::assessment::hour_ranking::RankedHourV1`] are the *substrate*
//! — but a user comparing recommendations wants to know, in one glance:
//!
//! - which favorable and adverse factors actually influenced the result;
//! - which facts were deduplicated so a single underlying signal could
//!   not double-count;
//! - which veto or guardrail won, when vetoes override weighted aggregation;
//! - what evidence was unavailable, so a "Medium" confidence is not
//!   silently misread as a strong signal;
//! - why the confidence level is what it is, broken down by missing or
//!   present evidence dimensions;
//! - and to see exactly the same answer whether they query the core
//!   library, the API, the terminal, or the desktop app.
//!
//! This module is the single source of truth for that projection. It is
//! deliberately a *projection* over the existing trace, not a new
//! assessment policy: it does not change scores, buckets, or vetoes —
//! it only re-shapes the substrate into a user-facing narrative whose
//! fields are stable, additive, and shared across the three surfaces.
//!
//! ## Stability contract
//!
//! The projection ships under [`EXPLANATION_PROJECTION_VERSION`]. Adding
//! a new optional field to any `*Explanation` struct is allowed; renaming
//! or repurposing one requires a version bump and a parity fixture.
//!
//! ## Precedence rule
//!
//! The projection always reports the same deterministic precedence
//! rule ([`PrecedenceRule::VetoOverridesAggregation`]) regardless of
//! policy version. The same rule applies to day, hour, and direction
//! assessments: a named veto wins over any weighted suitability signal
//! for its declared scope. Sensitivity tests
//! (`assessment_explanation_sensitivity`) pin this contract by
//! perturbing policy weights and asserting the reported veto set and
//! `precedence_rule` field are byte-equal.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    assessment::{
        direction::{
            DirectionAssessment, DirectionAssessmentAxis, DirectionAssessmentContribution,
        },
        hour_ranking::{
            HourRankingAxis, HourRankingAxisOutcome, RankedHourV1, HOUR_RANKING_POLICY_V1_ID,
            HOUR_RANKING_POLICY_V2_4_VERSION,
        },
        trace::VetoEvent,
        AssessmentAxis, DecisionContribution, EvidenceCoverage, FeatureObservation,
        PersonalDayAssessment, PersonalDayDecision, SourceEvidence, UnavailableSection,
    },
    birth::BirthCapability,
    reasoning::DecisionConfidence,
};

/// Stable version of the explanation projection. Bumping this constant
/// requires updating the explanation fixtures and the cross-surface
/// contract test (`assessment_explanation_cross_surface`).
pub const EXPLANATION_PROJECTION_VERSION: &str = "v1";

/// Stable identifier for the explanation projection family.
pub const EXPLANATION_PROJECTION_ID: &str = "explanation-projection";

/// Maximum number of favorable or adverse factors surfaced in the
/// per-assessment explanation. The substrate is preserved on the
/// assessment itself; the explanation is the *headline* view.
const MAX_FACTORS_PER_POLARITY: usize = 5;

/// Deterministic precedence rule a consumer is reading. The current
/// policy family always reports [`PrecedenceRule::VetoOverridesAggregation`].
/// A future policy family that needs a different precedence would
/// introduce a new variant and bump [`EXPLANATION_PROJECTION_VERSION`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrecedenceRule {
    /// Named vetoes force the decision to `Avoid` (or the veto's
    /// declared bucket) regardless of the weighted aggregation.
    /// Veto precedence is determined by the policy's `extract_vetoes`
    /// step; the explanation surfaces the winning vetoes in
    /// `vetoes_applied` and never collapses them into the weighted
    /// contribution list.
    VetoOverridesAggregation,
}

impl PrecedenceRule {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VetoOverridesAggregation => "veto_overrides_aggregation",
        }
    }
}

/// Family identifier for a deduplication rule. Stable across policy
/// versions so the explanation can describe "what rule applied" without
/// referring to a feature's internal identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeduplicationFamily {
    /// Bazi-to-day pillar relations (clash / lục hợp / tam hợp):
    /// each relation *kind* fires at most once per assessment so a
    /// target day that clashes with both year and month pillars emits
    /// a single Avoid contribution rather than two
    /// (`amlich-bz0f.2`).
    BaziTargetDayPillarRelation,
    /// Non-Bazi annual pressure observations: each declared system
    /// (Tam Tai / Kim Lâu / Hoàng Ốc / Thái Tuế / sao hạn) fires at
    /// most once per assessment (`amlich-bz0f.3`).
    NonBaziAnnualPressure,
    /// Direction assessment constraint facts: each (direction,
    /// fact-family) pair fires at most once per assessment
    /// (`amlich-bz0f.5`).
    DirectionConstraintFact,
    /// Hour ranking pillar relations: each (hour-slot, relation-kind)
    /// pair fires at most once per hour ranking report
    /// (`amlich-bz0f.4`).
    HourPillarRelation,
}

impl DeduplicationFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BaziTargetDayPillarRelation => "bazi_target_day_pillar_relation",
            Self::NonBaziAnnualPressure => "non_bazi_annual_pressure",
            Self::DirectionConstraintFact => "direction_constraint_fact",
            Self::HourPillarRelation => "hour_pillar_relation",
        }
    }

    pub fn rule_description(self) -> &'static str {
        match self {
            Self::BaziTargetDayPillarRelation => {
                "branch_relation_kind: each kind (clash, lục hợp, tam hợp) fires at most once per assessment"
            }
            Self::NonBaziAnnualPressure => {
                "annual_system: each declared system (Tam Tai, Kim Lâu, Hoàng Ốc, Thái Tuế, sao hạn) fires at most once per assessment"
            }
            Self::DirectionConstraintFact => {
                "direction_fact_family: each (direction, fact-family) pair fires at most once per assessment"
            }
            Self::HourPillarRelation => {
                "hour_relation_kind: each (hour-slot, relation-kind) pair fires at most once per hour ranking"
            }
        }
    }
}

/// One deduplication rule that was applied to the assessment. The
/// `rule` string is the human-readable description; the
/// `observed_count` records how many distinct inputs were collapsed
/// into the contribution that ended up on the trace (0 when the
/// family was applicable but did not fire, ≥ 1 when it did).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeduplicatedFact {
    pub family: DeduplicationFamily,
    pub rule: String,
    pub observed_count: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// One contributing factor surfaced to the user-facing explanation.
/// Both favorable and adverse factors carry the same shape so the
/// consumer can render them through a single template; the `polarity`
/// is the discriminator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainedFactor {
    pub contribution_id: String,
    pub axis: AssessmentAxis,
    pub polarity: crate::assessment::ContributionPolarity,
    pub strength: f32,
    pub source_evidence: SourceEvidence,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// One veto that fired and won precedence over the weighted aggregation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainedVeto {
    pub veto_id: String,
    pub axis: AssessmentAxis,
    pub reason: String,
    pub source_evidence: SourceEvidence,
}

/// Pair of factors with opposing polarities on the same axis, surfaced
/// as a "conflicting evidence" entry. The rule is deterministic: a
/// `Favorable` feature and an `Avoid` feature on the same axis with
/// comparable strength is the canonical conflict pattern; the
/// explanation surfaces the pair so the consumer can describe it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainedConflict {
    pub axis: AssessmentAxis,
    pub favorable: ExplainedFactor,
    pub adverse: ExplainedFactor,
    pub rule: String,
}

/// Stable dimension identifier for a [`ConfidenceReason`]. The set is
/// closed and versioned by the projection; adding a new dimension
/// requires a version bump on [`EXPLANATION_PROJECTION_VERSION`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceDimension {
    Date,
    Time,
    Gender,
    Location,
    DirectionOverlay,
}

impl ConfidenceDimension {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Time => "time",
            Self::Gender => "gender",
            Self::Location => "location",
            Self::DirectionOverlay => "direction_overlay",
        }
    }
}

/// One reason a dimension of the input is present or missing. The
/// `dimension` is a stable, snake_case identifier; `present` records
/// the capability flag; `impact` is the human-readable consequence
/// on confidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfidenceReason {
    pub dimension: ConfidenceDimension,
    pub present: bool,
    pub impact: String,
}

/// The decision confidence level plus the per-dimension reasons it is
/// at that level. Consumers can render the full list, or summarise
/// (e.g., "3 of 4 evidence dimensions are present").
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExplainedConfidence {
    pub level: DecisionConfidence,
    pub reasons: Vec<ConfidenceReason>,
    /// `present_count` / `total_count` for the canonical capability
    /// dimensions. Convenience field so consumers do not have to
    /// recount the reasons list.
    pub present_count: usize,
    pub total_count: usize,
}

/// One unavailable evidence section, in the same shape the consumer
/// already knows from the legacy `unavailable_sections` field, but
/// namespaced under the explanation so the cross-surface contract
/// does not depend on the legacy field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnavailableEvidence {
    pub section: String,
    pub axis: Option<AssessmentAxis>,
    pub reason: String,
    pub required_fields: Vec<String>,
}

/// Day-assessment explanation projection. Built from a
/// [`PersonalDayAssessment`] by [`explain_day_assessment`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssessmentExplanation {
    pub projection_id: &'static str,
    pub projection_version: &'static str,
    pub policy_id: String,
    pub policy_version: String,
    pub intent_kind: String,
    pub precedence_rule: PrecedenceRule,
    pub favorable_factors: Vec<ExplainedFactor>,
    pub adverse_factors: Vec<ExplainedFactor>,
    pub vetoes_applied: Vec<ExplainedVeto>,
    pub deduplicated_facts: Vec<DeduplicatedFact>,
    pub conflicts: Vec<ExplainedConflict>,
    pub unavailable_evidence: Vec<UnavailableEvidence>,
    pub confidence: ExplainedConfidence,
    /// The decision this explanation projects. Mirrored from the
    /// assessment so consumers can correlate the explanation with the
    /// verdict without reaching back into the source assessment.
    pub decision: PersonalDayDecision,
    /// The evidence coverage flags the explanation was derived from.
    pub evidence_coverage: EvidenceCoverage,
}

/// Direction-assessment explanation projection. Mirrors the day
/// explanation's shape so the two consumer surfaces share a renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionExplanation {
    pub projection_id: &'static str,
    pub projection_version: &'static str,
    pub policy_id: String,
    pub policy_version: String,
    pub intent_kind: String,
    pub precedence_rule: PrecedenceRule,
    pub unavailable_evidence: Vec<UnavailableEvidence>,
    pub confidence: ExplainedConfidence,
    /// Deduplication rules that applied to the direction assessment.
    /// Always includes the canonical
    /// [`DeduplicationFamily::DirectionConstraintFact`] entry so
    /// consumers can show the rule that prevents per-direction fact
    /// double-counting.
    pub deduplicated_facts: Vec<DeduplicatedFact>,
    /// Per-direction summary: the dedup'd constraint facts that
    /// fired for that direction.
    pub constraint_facts: Vec<DirectionConstraintFactSummary>,
    /// Conflicts where the same direction received a favorable
    /// signal (Kua, travel deity) and an adverse constraint
    /// (Thái Tuế / Tam Sát / Sát Phương).
    pub conflicts: Vec<DirectionConflict>,
}

/// Per-direction summary of the deduplication rule applied to
/// constraint facts. One entry per direction the assessment evaluated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionConstraintFactSummary {
    pub direction: String,
    pub facts: Vec<ExplainedFactor>,
    pub rule: String,
}

/// One direction-level conflict: a favorable signal (Kua match,
/// travel deity, or favorable flying star) on the same direction as
/// a hard constraint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectionConflict {
    pub direction: String,
    pub favorable: ExplainedFactor,
    pub adverse: ExplainedFactor,
    pub rule: String,
}

/// Hour-ranking explanation projection. Built from a slice of
/// [`RankedHourV1`] by [`explain_hour_ranking`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourExplanation {
    pub projection_id: &'static str,
    pub projection_version: &'static str,
    pub policy_id: String,
    pub policy_version: String,
    pub precedence_rule: PrecedenceRule,
    pub hours: Vec<HourEntryExplanation>,
    /// The deduplication rules that were active for the hour
    /// ranking. Always includes the canonical
    /// [`DeduplicationFamily::HourPillarRelation`] rule when the
    /// full-profile v2.4 policy ran.
    pub deduplicated_facts: Vec<DeduplicatedFact>,
    pub confidence: ExplainedConfidence,
    /// Day-verdict warning context that propagated from the day
    /// assessment, if any. Carried over so a consumer can correlate
    /// the hour explanation with the day-level "Avoid" warning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_warning: Option<HourDayWarning>,
}

/// One hour-slot explanation: the per-axis outcomes and the
/// unavailable evidence for that hour. The `factors` are
/// `ExplainedFactor` records keyed off the canonical four
/// [`HourRankingAxis`] so a renderer can pivot on `axis` for the
/// per-hour summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourEntryExplanation {
    pub chi_index: usize,
    pub chi_name: String,
    pub time_range: String,
    pub is_auspicious: bool,
    pub rank_score: f32,
    pub factors: Vec<ExplainedFactor>,
    pub unavailable_evidence: Vec<UnavailableEvidence>,
    pub policy_version: String,
}

/// Day-verdict warning context carried into the hour explanation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HourDayWarning {
    pub day_bucket: String,
    pub message_vi: String,
}

// ---------------------------------------------------------------------
// Projection entry points
// ---------------------------------------------------------------------

/// Build a [`AssessmentExplanation`] from a [`PersonalDayAssessment`].
/// The projection is pure: it depends only on the assessment's
/// existing fields (contributions, factors, trace, evidence, decision,
/// capability) and produces a deterministic, byte-stable result.
pub fn explain_day_assessment(assessment: &PersonalDayAssessment) -> AssessmentExplanation {
    let contributions = &assessment.contributions;
    let trace = assessment.trace.as_ref();
    let capability = &assessment.capability;
    let evidence = &assessment.evidence;
    let decision = assessment.decision.clone();

    let (favorable, adverse) = split_factors(contributions, trace);

    let vetoes_applied: Vec<ExplainedVeto> = trace
        .map(|t| {
            t.vetoes
                .iter()
                .map(|v: &VetoEvent| ExplainedVeto {
                    veto_id: v.veto_id.clone(),
                    axis: v.axis,
                    reason: v.reason.clone(),
                    source_evidence: v.source_evidence.clone(),
                })
                .collect()
        })
        .unwrap_or_default();

    let deduplicated_facts = detect_day_deduplication(trace, contributions);
    let conflicts = detect_day_conflicts(&favorable, &adverse);
    let unavailable_evidence = unavailable_evidence_from_sections(&assessment.unavailable_sections);
    let confidence = explain_confidence(capability, decision.confidence);

    AssessmentExplanation {
        projection_id: EXPLANATION_PROJECTION_ID,
        projection_version: EXPLANATION_PROJECTION_VERSION,
        policy_id: assessment.policy_id.clone(),
        policy_version: assessment.policy_version.clone(),
        intent_kind: assessment.intent.event_kind().to_string(),
        precedence_rule: PrecedenceRule::VetoOverridesAggregation,
        favorable_factors: favorable,
        adverse_factors: adverse,
        vetoes_applied,
        deduplicated_facts,
        conflicts,
        unavailable_evidence,
        confidence,
        decision,
        evidence_coverage: evidence.clone(),
    }
}

/// Build a [`DirectionExplanation`] from a [`DirectionAssessment`].
pub fn explain_direction_assessment(assessment: &DirectionAssessment) -> DirectionExplanation {
    let confidence = explain_confidence_for_direction(assessment.confidence, assessment);
    let observed_count = count_unique_direction_facts(assessment);
    let deduplicated = vec![DeduplicatedFact {
        family: DeduplicationFamily::DirectionConstraintFact,
        rule: DeduplicationFamily::DirectionConstraintFact
            .rule_description()
            .to_string(),
        observed_count,
        note: Some(
            "Each (direction, fact-family) pair appears at most once per direction assessment"
                .to_string(),
        ),
    }];
    let _ = deduplicated;

    let unavailable_evidence: Vec<UnavailableEvidence> = assessment
        .unavailable_sections
        .iter()
        .map(|w| UnavailableEvidence {
            section: w.code.clone(),
            axis: None,
            reason: w.message_vi.clone(),
            required_fields: Vec::new(),
        })
        .collect();

    let constraint_facts = build_constraint_fact_summaries(assessment);
    let conflicts = detect_direction_conflicts(assessment);

    DirectionExplanation {
        projection_id: EXPLANATION_PROJECTION_ID,
        projection_version: EXPLANATION_PROJECTION_VERSION,
        policy_id: assessment.policy_id.clone(),
        policy_version: assessment.policy_version.clone(),
        intent_kind: assessment.intent.event_kind().to_string(),
        precedence_rule: PrecedenceRule::VetoOverridesAggregation,
        unavailable_evidence,
        confidence,
        deduplicated_facts: deduplicated,
        constraint_facts,
        conflicts,
    }
}

/// Build a [`HourExplanation`] from a slice of [`RankedHourV1`] plus
/// the surrounding day assessment that drove the ranking.
pub fn explain_hour_ranking(
    ranking: &[RankedHourV1],
    day_assessment: &PersonalDayAssessment,
) -> HourExplanation {
    let policy_id = HOUR_RANKING_POLICY_V1_ID.to_string();
    let policy_version = ranking
        .first()
        .and_then(|h| h.policy_version.clone())
        .unwrap_or_default();

    let deduplicated: Vec<DeduplicatedFact> = if policy_version == HOUR_RANKING_POLICY_V2_4_VERSION
    {
        vec![DeduplicatedFact {
            family: DeduplicationFamily::HourPillarRelation,
            rule: DeduplicationFamily::HourPillarRelation
                .rule_description()
                .to_string(),
            observed_count: ranking
                .iter()
                .flat_map(|h| h.contributions.iter())
                .filter(|c| {
                    c.source_evidence.method.contains("hour_pillar_relation")
                        || c.source_evidence.method.contains("hour_branch_relation")
                })
                .count(),
            note: Some(
                "Each (hour-slot, relation-kind) pair appears at most once in the ranking"
                    .to_string(),
            ),
        }]
    } else {
        Vec::new()
    };

    let hours: Vec<HourEntryExplanation> = ranking
        .iter()
        .map(|ranked| {
            let mut factors: Vec<ExplainedFactor> = ranked
                .axes
                .iter()
                .filter_map(|axis| explained_factor_from_axis(axis, &policy_version))
                .collect();
            factors.sort_by(|a, b| {
                b.strength
                    .partial_cmp(&a.strength)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            let unavailable_evidence: Vec<UnavailableEvidence> = ranked
                .axes
                .iter()
                .filter_map(unavailable_from_axis)
                .collect();
            HourEntryExplanation {
                chi_index: ranked.chi_index,
                chi_name: ranked.chi_name.clone(),
                time_range: ranked.time_range.clone(),
                is_auspicious: ranked.is_auspicious,
                rank_score: ranked.rank_score,
                factors,
                unavailable_evidence,
                policy_version: policy_version.clone(),
            }
        })
        .collect();

    let confidence = explain_confidence(
        &day_assessment.capability,
        day_assessment.decision.confidence,
    );

    let day_warning = ranking
        .first()
        .and_then(|h| h.warning_context.as_ref())
        .map(|w| HourDayWarning {
            day_bucket: format!("{:?}", w.day_bucket).to_lowercase(),
            message_vi: w.message_vi.clone(),
        });

    HourExplanation {
        projection_id: EXPLANATION_PROJECTION_ID,
        projection_version: EXPLANATION_PROJECTION_VERSION,
        policy_id,
        policy_version,
        precedence_rule: PrecedenceRule::VetoOverridesAggregation,
        hours,
        deduplicated_facts: deduplicated,
        confidence,
        day_warning,
    }
}

// ---------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------

fn split_factors(
    contributions: &[DecisionContribution],
    trace: Option<&crate::assessment::trace::AssessmentTrace>,
) -> (Vec<ExplainedFactor>, Vec<ExplainedFactor>) {
    // The trace is the source of truth for "what contributed to the
    // axis aggregation" under the v2 policy. When the trace is absent
    // (legacy v1 builder), we fall back to the contributions list so
    // the explanation still works for the v1 path.
    let source_features: Vec<&FeatureObservation> = trace
        .map(|t| t.features.iter().filter(|f| !f.is_unavailable()).collect())
        .unwrap_or_default();

    if !source_features.is_empty() {
        let mut favorable: Vec<ExplainedFactor> = source_features
            .iter()
            .filter(|f| {
                matches!(
                    f.polarity,
                    crate::assessment::ContributionPolarity::Favorable
                )
            })
            .map(|f| ExplainedFactor {
                contribution_id: f.contribution_id.clone(),
                axis: f.feature_id.default_axis(),
                polarity: f.polarity,
                strength: f.strength,
                source_evidence: f.source_evidence.clone(),
                note: f.note.clone(),
            })
            .collect();
        let mut adverse: Vec<ExplainedFactor> = source_features
            .iter()
            .filter(|f| matches!(f.polarity, crate::assessment::ContributionPolarity::Avoid))
            .map(|f| ExplainedFactor {
                contribution_id: f.contribution_id.clone(),
                axis: f.feature_id.default_axis(),
                polarity: f.polarity,
                strength: f.strength,
                source_evidence: f.source_evidence.clone(),
                note: f.note.clone(),
            })
            .collect();
        favorable.sort_by(|a, b| {
            b.strength
                .partial_cmp(&a.strength)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        adverse.sort_by(|a, b| {
            b.strength
                .partial_cmp(&a.strength)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        favorable.truncate(MAX_FACTORS_PER_POLARITY);
        adverse.truncate(MAX_FACTORS_PER_POLARITY);
        return (favorable, adverse);
    }

    let mut favorable: Vec<ExplainedFactor> = contributions
        .iter()
        .filter(|c| {
            matches!(
                c.polarity,
                crate::assessment::ContributionPolarity::Favorable
            )
        })
        .map(explained_from_contribution)
        .collect();
    let mut adverse: Vec<ExplainedFactor> = contributions
        .iter()
        .filter(|c| matches!(c.polarity, crate::assessment::ContributionPolarity::Avoid))
        .map(explained_from_contribution)
        .collect();
    favorable.sort_by(|a, b| {
        b.strength
            .partial_cmp(&a.strength)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    adverse.sort_by(|a, b| {
        b.strength
            .partial_cmp(&a.strength)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    favorable.truncate(MAX_FACTORS_PER_POLARITY);
    adverse.truncate(MAX_FACTORS_PER_POLARITY);
    (favorable, adverse)
}

fn explained_from_contribution(c: &DecisionContribution) -> ExplainedFactor {
    ExplainedFactor {
        contribution_id: c.contribution_id.clone(),
        axis: c.axis,
        polarity: c.polarity,
        strength: c.strength,
        source_evidence: c.source_evidence.clone(),
        note: c.note.clone(),
    }
}

fn detect_day_deduplication(
    trace: Option<&crate::assessment::trace::AssessmentTrace>,
    contributions: &[DecisionContribution],
) -> Vec<DeduplicatedFact> {
    let mut facts = Vec::new();

    let is_v2_3_or_later = trace.is_some_and(|t| {
        matches!(
            t.policy_version.as_str(),
            crate::assessment::ASSESSMENT_POLICY_V2_3_VERSION
                | crate::assessment::ASSESSMENT_POLICY_V2_4_VERSION
        )
    });
    if is_v2_3_or_later {
        let observed = contributions
            .iter()
            .filter(|c| {
                c.contribution_id
                    .starts_with("bazi.target_day.pillar_relation")
            })
            .count();
        facts.push(DeduplicatedFact {
            family: DeduplicationFamily::BaziTargetDayPillarRelation,
            rule: DeduplicationFamily::BaziTargetDayPillarRelation
                .rule_description()
                .to_string(),
            observed_count: observed,
            note: Some(
                "Target-day pillar relations collapse to one observation per kind (clash / lục hợp / tam hợp)"
                    .to_string(),
            ),
        });
    }

    let is_v2_4 = trace
        .is_some_and(|t| t.policy_version == crate::assessment::ASSESSMENT_POLICY_V2_4_VERSION);
    if is_v2_4 {
        let observed = contributions
            .iter()
            .filter(|c| {
                c.contribution_id.starts_with("annual.tam_tai")
                    || c.contribution_id.starts_with("annual.kim_lau")
                    || c.contribution_id.starts_with("annual.hoang_oc")
                    || c.contribution_id.starts_with("annual.thai_tue")
                    || c.contribution_id.starts_with("annual.sao_han")
            })
            .count();
        facts.push(DeduplicatedFact {
            family: DeduplicationFamily::NonBaziAnnualPressure,
            rule: DeduplicationFamily::NonBaziAnnualPressure
                .rule_description()
                .to_string(),
            observed_count: observed,
            note: Some(
                "Annual pressure observations collapse to one per declared system".to_string(),
            ),
        });
    }

    facts
}

fn detect_day_conflicts(
    favorable: &[ExplainedFactor],
    adverse: &[ExplainedFactor],
) -> Vec<ExplainedConflict> {
    let mut conflicts = Vec::new();
    for fav in favorable {
        if let Some(adv) = adverse
            .iter()
            .find(|a| a.axis == fav.axis && (fav.strength - a.strength).abs() < 0.5)
        {
            conflicts.push(ExplainedConflict {
                axis: fav.axis,
                favorable: fav.clone(),
                adverse: adv.clone(),
                rule: format!(
                    "favorable and adverse observations on axis `{}` with comparable strength",
                    fav.axis.as_str()
                ),
            });
        }
    }
    conflicts
}

fn unavailable_evidence_from_sections(sections: &[UnavailableSection]) -> Vec<UnavailableEvidence> {
    sections
        .iter()
        .map(|s| UnavailableEvidence {
            section: s.section.clone(),
            axis: axis_for_unavailable_section(&s.section),
            reason: s.reason.clone(),
            required_fields: s.required_fields.clone(),
        })
        .collect()
}

fn axis_for_unavailable_section(section: &str) -> Option<AssessmentAxis> {
    match section {
        "personal_alignment" | "personal_kua" => Some(AssessmentAxis::PersonalAlignment),
        "annual_pressure" | "annual_han" => Some(AssessmentAxis::AnnualPressure),
        "personal_hours" => Some(AssessmentAxis::IntentFit),
        _ => None,
    }
}

fn explain_confidence(
    capability: &BirthCapability,
    level: DecisionConfidence,
) -> ExplainedConfidence {
    let reasons = vec![
        ConfidenceReason {
            dimension: ConfidenceDimension::Date,
            present: capability.has_date,
            impact: if capability.has_date {
                "birth date is present"
            } else {
                "missing birth date: personal-day evaluation cannot anchor to a birth year"
            }
            .to_string(),
        },
        ConfidenceReason {
            dimension: ConfidenceDimension::Time,
            present: capability.has_time,
            impact: if capability.has_time {
                "birth time is present: personal-hour and chart pillar derivations available"
            } else {
                "missing birth time: personal-hour and chart hour pillar unavailable"
            }
            .to_string(),
        },
        ConfidenceReason {
            dimension: ConfidenceDimension::Gender,
            present: capability.has_gender,
            impact: if capability.has_gender {
                "gender is present: personal interaction facts and yearly Hạn available"
            } else {
                "missing gender: personal interaction facts and yearly Hạn unavailable"
            }
            .to_string(),
        },
        ConfidenceReason {
            dimension: ConfidenceDimension::Location,
            present: capability.has_location,
            impact: if capability.has_location {
                "location is present: location-sensitive overlays (Phi Tinh) can be applied"
            } else {
                "missing location: location-sensitive overlays (Phi Tinh) unavailable"
            }
            .to_string(),
        },
    ];
    let present_count = reasons.iter().filter(|r| r.present).count();
    ExplainedConfidence {
        level,
        reasons,
        present_count,
        total_count: 4,
    }
}

fn explain_confidence_for_direction(
    level: DecisionConfidence,
    assessment: &DirectionAssessment,
) -> ExplainedConfidence {
    let has_gender = !assessment
        .unavailable_sections
        .iter()
        .any(|w| w.code == "kua_unavailable");
    let has_location = !assessment
        .unavailable_sections
        .iter()
        .any(|w| w.code == "location_unavailable");
    let has_intent_overlay = !assessment
        .unavailable_sections
        .iter()
        .any(|w| w.code == "directional_overlay_unavailable");

    let reasons = vec![
        ConfidenceReason {
            dimension: ConfidenceDimension::Gender,
            present: has_gender,
            impact: if has_gender {
                "gender is present: Kua compatibility can be evaluated"
            } else {
                "missing gender: Kua compatibility unavailable across all directions"
            }
            .to_string(),
        },
        ConfidenceReason {
            dimension: ConfidenceDimension::Location,
            present: has_location,
            impact: if has_location {
                "location is present: location-sensitive flying-star overlay available"
            } else {
                "missing location: flying-star overlay unavailable across all directions"
            }
            .to_string(),
        },
        ConfidenceReason {
            dimension: ConfidenceDimension::DirectionOverlay,
            present: has_intent_overlay,
            impact: if has_intent_overlay {
                "directional constraint overlay (Thái Tuế / Tam Sát / Sát Phương) is available"
            } else {
                "directional constraint overlay is unavailable for this snapshot"
            }
            .to_string(),
        },
    ];
    let present_count = reasons.iter().filter(|r| r.present).count();
    ExplainedConfidence {
        level,
        reasons,
        present_count,
        total_count: 3,
    }
}

fn build_constraint_fact_summaries(
    assessment: &DirectionAssessment,
) -> Vec<DirectionConstraintFactSummary> {
    assessment
        .entries
        .iter()
        .map(|entry| {
            let facts: Vec<ExplainedFactor> = entry
                .contributions
                .iter()
                .filter(|c| matches!(c.axis, DirectionAssessmentAxis::DirectionalConstraints))
                .map(explained_from_direction_contribution)
                .collect();
            DirectionConstraintFactSummary {
                direction: entry.direction.as_vn_str().to_string(),
                facts,
                rule: DeduplicationFamily::DirectionConstraintFact
                    .rule_description()
                    .to_string(),
            }
        })
        .collect()
}

fn explained_from_direction_contribution(c: &DirectionAssessmentContribution) -> ExplainedFactor {
    ExplainedFactor {
        contribution_id: c.contribution_id.clone(),
        axis: AssessmentAxis::IntentFit,
        polarity: c.polarity,
        strength: c.strength,
        source_evidence: c.source_evidence.clone(),
        note: c.note.clone(),
    }
}

fn count_unique_direction_facts(assessment: &DirectionAssessment) -> usize {
    let mut count = 0;
    for entry in &assessment.entries {
        let unique: BTreeSet<&str> = entry
            .contributions
            .iter()
            .map(|c| c.contribution_id.as_str())
            .collect();
        count += unique.len();
    }
    count
}

fn detect_direction_conflicts(assessment: &DirectionAssessment) -> Vec<DirectionConflict> {
    let mut conflicts = Vec::new();
    for entry in &assessment.entries {
        let favorables: Vec<&DirectionAssessmentContribution> = entry
            .contributions
            .iter()
            .filter(|c| {
                matches!(
                    c.polarity,
                    crate::assessment::ContributionPolarity::Favorable
                )
            })
            .collect();
        let adverses: Vec<&DirectionAssessmentContribution> = entry
            .contributions
            .iter()
            .filter(|c| matches!(c.polarity, crate::assessment::ContributionPolarity::Avoid))
            .collect();
        for fav in &favorables {
            for adv in &adverses {
                conflicts.push(DirectionConflict {
                    direction: entry.direction.as_vn_str().to_string(),
                    favorable: explained_from_direction_contribution(fav),
                    adverse: explained_from_direction_contribution(adv),
                    rule: format!(
                        "favorable signal `{}` and adverse constraint `{}` on direction `{}`",
                        fav.contribution_id,
                        adv.contribution_id,
                        entry.direction.as_vn_str()
                    ),
                });
            }
        }
    }
    conflicts
}

fn explained_factor_from_axis(
    axis: &HourRankingAxisOutcome,
    policy_version: &str,
) -> Option<ExplainedFactor> {
    let score = axis.score?;
    let polarity = if score >= 0.7 {
        crate::assessment::ContributionPolarity::Favorable
    } else if score < 0.3 {
        crate::assessment::ContributionPolarity::Avoid
    } else {
        crate::assessment::ContributionPolarity::Neutral
    };
    let contribution_id = format!("hour_axis.{}", axis.axis.as_str());
    let mapped_axis = map_hour_axis_to_day_axis(axis.axis);
    Some(ExplainedFactor {
        contribution_id,
        axis: mapped_axis,
        polarity,
        strength: score,
        source_evidence: SourceEvidence {
            source_family: "hour_ranking".to_string(),
            source_id: "hour-ranking".to_string(),
            method: "aggregate_hour_axes".to_string(),
            profile: "default".to_string(),
            note: Some(format!("policy_version={}", policy_version)),
        },
        note: Some(axis.axis.as_str().to_string()),
    })
}

fn map_hour_axis_to_day_axis(axis: HourRankingAxis) -> AssessmentAxis {
    match axis {
        HourRankingAxis::HoangDaoQuality | HourRankingAxis::IntentTimingFit => {
            AssessmentAxis::IntentFit
        }
        HourRankingAxis::PersonalHourAlignment => AssessmentAxis::PersonalAlignment,
        HourRankingAxis::DayHourHarmony => AssessmentAxis::GenericDayQuality,
    }
}

fn unavailable_from_axis(axis: &HourRankingAxisOutcome) -> Option<UnavailableEvidence> {
    let reason = axis.unavailable_reason.as_deref()?;
    Some(UnavailableEvidence {
        section: format!("hour_axis.{}", axis.axis.as_str()),
        axis: Some(map_hour_axis_to_day_axis(axis.axis)),
        reason: reason.to_string(),
        required_fields: Vec::new(),
    })
}

// ---------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advisory::ConsultationIntent;
    use crate::assessment::{
        trace::{AxisAggregation, AxisWeight, DecisionAggregation, InteractionTerm},
        AssessmentAxes, AssessmentFactor, AssessmentFactorRole, AvailabilityState, AxisOutcome,
        DecisionContribution, EvidenceCoverage, NormalizedBirth, PersonalDayAssessment,
        PersonalDayDecision, UnavailableSection,
    };
    use crate::birth::{BirthCapability, BirthDataTier, BirthTime};
    use crate::reasoning::RecommendationBucket as ReasoningBucket;

    fn sample_evidence() -> EvidenceCoverage {
        EvidenceCoverage {
            has_chart: true,
            has_analysis: true,
            has_yearly_han: false,
            has_kua: true,
            has_kim_lau: false,
            has_tam_tai: false,
            has_hoang_oc: false,
            has_thai_tue: false,
            has_sao_han: false,
            recommendation_count: 3,
        }
    }

    fn sample_decision(confidence: DecisionConfidence) -> PersonalDayDecision {
        PersonalDayDecision {
            bucket: ReasoningBucket::Mixed,
            confidence,
            semantic: "favorable_contextual".to_string(),
            primary_conclusion: String::new(),
            decision_score: Some(0.55),
            context_is_clear: true,
        }
    }

    fn sample_assessment_v2_4() -> PersonalDayAssessment {
        let capability = BirthCapability {
            has_date: true,
            has_time: true,
            has_gender: true,
            has_location: true,
            ..BirthCapability::default()
        };
        let features = vec![
            FeatureObservation::observed(
                crate::assessment::feature::AssessmentFeatureId::BaziTargetDayPillarRelation,
                crate::assessment::ContributionPolarity::Avoid,
                0.6,
                "bazi.target_day.pillar_relation.clash",
                SourceEvidence {
                    source_family: "bazi_observation".to_string(),
                    source_id: "KHCBPPT".to_string(),
                    method: "bazi_target_day_pillar_relation".to_string(),
                    profile: "default".to_string(),
                    note: None,
                },
                "ruleset",
                "1.0",
            ),
            FeatureObservation::observed(
                crate::assessment::feature::AssessmentFeatureId::AnnualTamTai,
                crate::assessment::ContributionPolarity::Avoid,
                0.5,
                "annual.tam_tai.day10",
                SourceEvidence {
                    source_family: "almanac_rule".to_string(),
                    source_id: "KHCBPPT".to_string(),
                    method: "tam_tai_lookup".to_string(),
                    profile: "default".to_string(),
                    note: None,
                },
                "ruleset",
                "1.0",
            ),
        ];

        PersonalDayAssessment {
            ruleset_id: "ruleset".to_string(),
            ruleset_version: "1.0".to_string(),
            profile: "default".to_string(),
            policy_id: crate::assessment::ASSESSMENT_POLICY_V2_ID.to_string(),
            policy_version: crate::assessment::ASSESSMENT_POLICY_V2_4_VERSION.to_string(),
            intent: ConsultationIntent::Wedding,
            capability,
            capability_tier: BirthDataTier::Datetime,
            normalized_birth: NormalizedBirth {
                day: 1,
                month: 1,
                year: 1990,
                timezone: 7.0,
                has_time: true,
                has_gender: true,
                has_location: true,
                has_solar_time_policy: false,
                time: Some(BirthTime {
                    hour: 9,
                    minute: 30,
                }),
            },
            axes: AssessmentAxes {
                generic_day_quality: AxisOutcome {
                    axis: AssessmentAxis::GenericDayQuality,
                    score: Some(0.55),
                    verdict: "mixed".to_string(),
                    unavailable_reason: None,
                },
                intent_fit: AxisOutcome {
                    axis: AssessmentAxis::IntentFit,
                    score: Some(0.5),
                    verdict: "mixed".to_string(),
                    unavailable_reason: None,
                },
                personal_alignment: AxisOutcome {
                    axis: AssessmentAxis::PersonalAlignment,
                    score: Some(0.4),
                    verdict: "cautious".to_string(),
                    unavailable_reason: None,
                },
                annual_pressure: AxisOutcome {
                    axis: AssessmentAxis::AnnualPressure,
                    score: Some(0.3),
                    verdict: "cautious".to_string(),
                    unavailable_reason: None,
                },
                evidence_coverage: AxisOutcome {
                    axis: AssessmentAxis::EvidenceCoverage,
                    score: Some(1.0),
                    verdict: "favorable".to_string(),
                    unavailable_reason: None,
                },
            },
            contributions: features
                .iter()
                .map(|f| DecisionContribution {
                    contribution_id: f.contribution_id.clone(),
                    axis: f.feature_id.default_axis(),
                    polarity: f.polarity,
                    strength: f.strength,
                    policy_id: "test-policy".to_string(),
                    policy_version: crate::assessment::ASSESSMENT_POLICY_V2_4_VERSION.to_string(),
                    ruleset_id: "ruleset".to_string(),
                    ruleset_version: "1.0".to_string(),
                    source_evidence: f.source_evidence.clone(),
                    availability: f.availability.clone(),
                    note: f.note.clone(),
                })
                .collect(),
            decision: sample_decision(DecisionConfidence::High),
            unavailable_sections: vec![UnavailableSection {
                section: "personal_hours".to_string(),
                reason: "requires explicit birth time for personal-hour context".to_string(),
                required_fields: vec!["hour".to_string(), "minute".to_string()],
            }],
            evidence: sample_evidence(),
            factors: Vec::new(),
            trace: Some(crate::assessment::trace::AssessmentTrace {
                policy_id: crate::assessment::ASSESSMENT_POLICY_V2_ID.to_string(),
                policy_version: crate::assessment::ASSESSMENT_POLICY_V2_4_VERSION.to_string(),
                features,
                axes: vec![AxisAggregation {
                    axis: AssessmentAxis::PersonalAlignment,
                    contributors: Vec::new(),
                    subtotal: Some(0.4),
                    verdict: "cautious".to_string(),
                    unavailable_reason: None,
                }],
                decision: DecisionAggregation {
                    axis_weights: vec![AxisWeight {
                        axis: AssessmentAxis::GenericDayQuality,
                        weight: 0.25,
                    }],
                    available_axes: vec![AssessmentAxis::GenericDayQuality],
                    unavailable_axes: Vec::new(),
                    decision_score: Some(0.55),
                    bucket: ReasoningBucket::Mixed,
                },
                vetoes: vec![],
                interactions: vec![InteractionTerm {
                    interaction_id: "test".to_string(),
                    feature_ids: vec![],
                    axis: AssessmentAxis::PersonalAlignment,
                    value: 0.1,
                    weight: 0.0,
                    source_evidence: SourceEvidence {
                        source_family: "test".to_string(),
                        source_id: "test".to_string(),
                        method: "test".to_string(),
                        profile: "test".to_string(),
                        note: None,
                    },
                    note: None,
                }],
            }),
        }
    }

    fn sample_contribution(
        contribution_id: &str,
        axis: AssessmentAxis,
        polarity: crate::assessment::ContributionPolarity,
        strength: f32,
    ) -> DecisionContribution {
        DecisionContribution {
            contribution_id: contribution_id.to_string(),
            axis,
            polarity,
            strength,
            policy_id: "test-policy".to_string(),
            policy_version: "v2.4".to_string(),
            ruleset_id: "ruleset".to_string(),
            ruleset_version: "1.0".to_string(),
            source_evidence: SourceEvidence {
                source_family: "test".to_string(),
                source_id: "test".to_string(),
                method: "test".to_string(),
                profile: "test".to_string(),
                note: None,
            },
            availability: AvailabilityState::Complete,
            note: Some(format!("note for {contribution_id}")),
        }
    }

    #[test]
    fn precedence_rule_is_stable_veto_overrides_aggregation() {
        let assessment = sample_assessment_v2_4();
        let explanation = explain_day_assessment(&assessment);
        assert_eq!(
            explanation.precedence_rule,
            PrecedenceRule::VetoOverridesAggregation
        );
        assert_eq!(
            explanation.precedence_rule.as_str(),
            "veto_overrides_aggregation"
        );
    }

    #[test]
    fn stable_projection_version_is_v1() {
        assert_eq!(EXPLANATION_PROJECTION_VERSION, "v1");
        assert_eq!(EXPLANATION_PROJECTION_ID, "explanation-projection");
    }

    #[test]
    fn explanation_includes_favorable_and_adverse_factors() {
        let assessment = sample_assessment_v2_4();
        let explanation = explain_day_assessment(&assessment);
        let total = explanation.favorable_factors.len() + explanation.adverse_factors.len();
        assert!(total > 0);
    }

    #[test]
    fn deduplicated_facts_records_branch_relation_and_annual_pressure() {
        let assessment = sample_assessment_v2_4();
        let explanation = explain_day_assessment(&assessment);
        let families: Vec<DeduplicationFamily> = explanation
            .deduplicated_facts
            .iter()
            .map(|f| f.family)
            .collect();
        assert!(families.contains(&DeduplicationFamily::BaziTargetDayPillarRelation));
        assert!(families.contains(&DeduplicationFamily::NonBaziAnnualPressure));
    }

    #[test]
    fn v1_path_still_produces_explanation_without_trace() {
        let mut assessment = sample_assessment_v2_4();
        assessment.trace = None;
        assessment.policy_version = crate::assessment::ASSESSMENT_POLICY_VERSION.to_string();
        let explanation = explain_day_assessment(&assessment);
        assert_eq!(
            explanation.precedence_rule,
            PrecedenceRule::VetoOverridesAggregation
        );
        assert!(explanation.deduplicated_facts.is_empty());
    }

    #[test]
    fn vetoes_are_separate_from_weighted_factors() {
        let mut assessment = sample_assessment_v2_4();
        if let Some(trace) = assessment.trace.as_mut() {
            trace.vetoes = vec![crate::assessment::trace::VetoEvent {
                veto_id: "veto.annual.han_critical".to_string(),
                axis: AssessmentAxis::AnnualPressure,
                reason: "Han severity critical".to_string(),
                source_evidence: SourceEvidence {
                    source_family: "almanac_rule".to_string(),
                    source_id: "KHCBPPT".to_string(),
                    method: "yearly_han".to_string(),
                    profile: "default".to_string(),
                    note: None,
                },
            }];
        }
        let explanation = explain_day_assessment(&assessment);
        assert_eq!(explanation.vetoes_applied.len(), 1);
        assert_eq!(
            explanation.vetoes_applied[0].veto_id,
            "veto.annual.han_critical"
        );
        assert!(!explanation
            .adverse_factors
            .iter()
            .any(|f| f.contribution_id == "veto.annual.han_critical"));
    }

    #[test]
    fn conflicts_surface_favorable_vs_adverse_on_same_axis() {
        // No trace → falls back to the contributions list, which the
        // test fully controls.
        let mut assessment = sample_assessment_v2_4();
        assessment.trace = None;
        assessment.policy_version = crate::assessment::ASSESSMENT_POLICY_VERSION.to_string();
        assessment.contributions = vec![
            sample_contribution(
                "personal.luc_xung",
                AssessmentAxis::PersonalAlignment,
                crate::assessment::ContributionPolarity::Avoid,
                0.8,
            ),
            sample_contribution(
                "personal.tam_hop",
                AssessmentAxis::PersonalAlignment,
                crate::assessment::ContributionPolarity::Favorable,
                0.7,
            ),
        ];
        let explanation = explain_day_assessment(&assessment);
        assert!(!explanation.conflicts.is_empty());
        assert_eq!(
            explanation.conflicts[0].axis,
            AssessmentAxis::PersonalAlignment
        );
    }

    #[test]
    fn confidence_reports_present_and_missing_evidence() {
        let assessment = sample_assessment_v2_4();
        let explanation = explain_day_assessment(&assessment);
        assert_eq!(explanation.confidence.present_count, 4);
        assert_eq!(explanation.confidence.total_count, 4);
        assert!(explanation
            .confidence
            .reasons
            .iter()
            .any(|r| r.dimension == ConfidenceDimension::Time && r.present));
    }

    #[test]
    fn unavailable_evidence_replicates_sections_with_axis() {
        let assessment = sample_assessment_v2_4();
        let explanation = explain_day_assessment(&assessment);
        let personal_hours = explanation
            .unavailable_evidence
            .iter()
            .find(|u| u.section == "personal_hours")
            .expect("personal_hours section must be present");
        assert_eq!(personal_hours.axis, Some(AssessmentAxis::IntentFit));
        assert_eq!(
            personal_hours.required_fields,
            vec!["hour".to_string(), "minute".to_string()]
        );
    }

    #[test]
    fn confidence_reflects_missing_capability() {
        let mut assessment = sample_assessment_v2_4();
        let capability = BirthCapability {
            has_date: true,
            has_time: false,
            has_gender: false,
            has_location: false,
            ..BirthCapability::default()
        };
        assessment.capability = capability;
        assessment.decision.confidence = DecisionConfidence::Low;
        let explanation = explain_day_assessment(&assessment);
        assert_eq!(explanation.confidence.present_count, 1);
        assert_eq!(explanation.confidence.total_count, 4);
        assert_eq!(explanation.confidence.level, DecisionConfidence::Low);
    }

    #[test]
    fn explanation_is_deterministic_byte_stable() {
        let assessment = sample_assessment_v2_4();
        let a = explain_day_assessment(&assessment);
        let b = explain_day_assessment(&assessment);
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
    }

    #[test]
    fn role_veto_factor_does_not_inflate_adverse_factors() {
        let mut assessment = sample_assessment_v2_4();
        let veto_factor = AssessmentFactor {
            factor_id: "veto.legacy.test".to_string(),
            role: AssessmentFactorRole::Veto,
            axis: Some(AssessmentAxis::AnnualPressure),
            availability: AvailabilityState::Complete,
            source_evidence: SourceEvidence {
                source_family: "test".to_string(),
                source_id: "test".to_string(),
                method: "test".to_string(),
                profile: "test".to_string(),
                note: None,
            },
            note: None,
        };
        assessment.factors.push(veto_factor);
        let explanation = explain_day_assessment(&assessment);
        assert!(!explanation
            .adverse_factors
            .iter()
            .any(|f| f.contribution_id == "veto.legacy.test"));
    }
}
