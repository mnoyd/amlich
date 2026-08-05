//! Calculation trace emitted by the v2 [`AssessmentPolicy`].
//!
//! The trace is a structured record of every feature observation, applied
//! weight, axis subtotal, and final aggregation step the policy performed
//! for one `(snapshot, profile, intent)` triple. It is the substrate that
//! the Evidence Graph projection (`amlich-8tdm`) consumes so that TUI, API,
//! and desktop explanations can describe the actual calculation rather than
//! a parallel recomputation.
//!
//! ## Stability contract
//!
//! Trace field names are part of the policy-versioned contract. Adding a
//! new optional field is allowed within a policy version; renaming or
//! repurposing one requires bumping [`crate::assessment::ASSESSMENT_POLICY_V2_VERSION`]
//! and adding a parity fixture.
//!
//! ## Placeholders for future issues
//!
//! `interactions` is intentionally empty under `baseline_v2`; declared
//! interaction features land in `amlich-47wn`. `vetoes` IS populated under
//! `baseline_v2` (`amlich-l0wu`): the legacy `strength >= 0.8` implicit
//! threshold was lifted into explicit, source-attributed [`VetoEvent`]s.

use serde::{Deserialize, Serialize};

use crate::assessment::feature::FeatureObservation;
use crate::assessment::{AssessmentAxis, SourceEvidence};
use crate::reasoning::RecommendationBucket;

/// One complete calculation trace for a single assessment. Built by
/// [`crate::assessment::AssessmentPolicy::evaluate`] and attached to the
/// resulting [`crate::assessment::PersonalDayAssessment`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssessmentTrace {
    pub policy_id: String,
    pub policy_version: String,
    /// Every feature observation the policy considered, including
    /// unavailable ones (so explanations can say what was missing).
    pub features: Vec<FeatureObservation>,
    /// Per-axis aggregation: which features contributed, with what raw
    /// value and applied weight, and the resulting subtotal. Unavailable
    /// axes carry an explicit `unavailable_reason`.
    pub axes: Vec<AxisAggregation>,
    /// Final decision aggregation: axis weights used, available/unavailable
    /// axis split, decision score, and bucket.
    pub decision: DecisionAggregation,
    /// Hard vetoes applied before weighted aggregation. Populated under
    /// `baseline_v2` by [`crate::assessment::extraction::extract_vetoes`]
    /// (`amlich-l0wu`): the legacy `strength >= 0.8` implicit threshold was
    /// lifted into named, source-attributed veto events that fire on the
    /// same source-data states for v1 decision parity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub vetoes: Vec<VetoEvent>,
    /// Declared interaction terms evaluated by the policy. Empty under
    /// `baseline_v2`; populated by `amlich-47wn`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interactions: Vec<InteractionTerm>,
}

/// Per-axis aggregation record inside an [`AssessmentTrace`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisAggregation {
    pub axis: AssessmentAxis,
    /// One entry per available feature that contributed to this axis.
    pub contributors: Vec<AxisContributor>,
    /// Aggregated axis score in `[0, 1]`, or `None` when the axis is
    /// unavailable for this profile/snapshot.
    pub subtotal: Option<f32>,
    /// Verdict label (`"favorable"`, `"mixed"`, `"cautious"`, `"avoid"`,
    /// `"unavailable"`) derived from the subtotal.
    pub verdict: String,
    /// Populated when the axis is unavailable. Explanations surface this
    /// reason instead of pretending the axis scored zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
}

/// One feature's contribution to one axis inside an [`AxisAggregation`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisContributor {
    pub feature_id: crate::assessment::feature::AssessmentFeatureId,
    pub contribution_id: String,
    /// Projected signed value of the feature observation (see
    /// [`FeatureObservation::signed_value`]).
    pub signed_value: f32,
    /// Weight the policy applied to this feature under this axis. Under
    /// `baseline_v2` the weight is the legacy `0.3` multiplier used by the
    /// v1 axis formula; intent-aware weights (`amlich-lxu3`) replace it
    /// with policy-table values.
    pub applied_weight: f32,
    /// `signed_value * applied_weight` — the actual delta this feature
    /// contributed to the axis numerator.
    pub contribution: f32,
}

/// Final decision aggregation inside an [`AssessmentTrace`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionAggregation {
    /// Axis weights used to combine axis subtotals into the final decision
    /// score. Under `baseline_v2` these are equal across the four scored
    /// axes (mirrors v1); intent-aware axis weights (`amlich-lxu3`) replace
    /// them with policy-table values per intent.
    pub axis_weights: Vec<AxisWeight>,
    /// Axes that contributed to the decision score (had an available
    /// subtotal).
    pub available_axes: Vec<AssessmentAxis>,
    /// Axes excluded from the decision score because they were unavailable.
    pub unavailable_axes: Vec<AssessmentAxis>,
    /// Final aggregated decision score in `[0, 1]`, or `None` when no axis
    /// was available.
    pub decision_score: Option<f32>,
    /// Bucket the policy classified the decision into.
    pub bucket: RecommendationBucket,
}

/// An axis-weight entry inside [`DecisionAggregation`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxisWeight {
    pub axis: AssessmentAxis,
    pub weight: f32,
}

/// A hard veto event applied by the policy. Each veto carries a stable
/// `veto_id`, the axis the constraint originates from, a human-readable
/// reason, and full source evidence. Populated under `baseline_v2` by
/// [`crate::assessment::extraction::extract_vetoes`] (`amlich-l0wu`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VetoEvent {
    /// Stable, policy-versioned veto identifier (e.g.
    /// `veto.personal.luc_xung`). Never reused; renaming or repurposing
    /// requires a policy version bump.
    pub veto_id: String,
    /// Axis the veto originates from, for explanation grouping.
    pub axis: AssessmentAxis,
    /// Human-readable reason the veto fired.
    pub reason: String,
    /// Source evidence attributing the veto to its domain provenance.
    pub source_evidence: SourceEvidence,
}

/// A declared interaction term evaluated by the policy. Empty under
/// `baseline_v2`; `amlich-47wn` introduces typed interaction features.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InteractionTerm {
    pub interaction_id: String,
    pub feature_ids: Vec<crate::assessment::feature::AssessmentFeatureId>,
    pub value: f32,
    pub weight: f32,
}
