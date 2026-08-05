//! Versioned assessment policy — the deep module that owns the v2
//! personal-day scoring pipeline.
//!
//! Source spec: `docs/architecture/personal-day-audit/SCORING-POLICY-V2-SPEC.md`
//! Bead: `amlich-7bm4`.
//!
//! The policy converts resolved inputs into stable feature observations,
//! aggregates them into the five canonical axes, synthesizes the final
//! decision, and emits a complete [`AssessmentTrace`] consumable by the
//! Evidence Graph projection (`amlich-8tdm`).
//!
//! `baseline_v2` is calibrated to reproduce the legacy v1 axis scores and
//! decision bucket on a representative fixture set so the v2 seam can land
//! without changing user-visible verdicts. The stability gate (`amlich-31oa`)
//! decides when v2 becomes the default; until then v1 stays the default
//! (`PersonalDayAssessment::assess`), and v2 is opt-in via
//! [`AssessmentPolicy::baseline_v2`].
//!
//! ## Policy variants
//!
//! Three opt-in constructors coexist under this module:
//!
//! - [`AssessmentPolicy::baseline_v2`] — the v1-parity baseline (`v2`).
//!   Uses equal weights across the four scored axes; locked by the
//!   `assessment_v2_seam::v1_v2_full_parity_*` suite.
//! - [`AssessmentPolicy::intent_weighted_v2`] — the v2.1 intent-aware
//!   variant (`amlich-lxu3`). Replaces the equal-weight aggregation with
//!   the sparse [`IntentAxisWeightTable`] so different intents can
//!   produce different final projections from the same axis scores.
//!   Divergences from v1/baseline_v2 are intentional and reviewed in
//!   `assessment_v2_1_intent_weights`.
//! - [`AssessmentPolicy::interaction_aware_v2`] — the v2.2
//!   interaction-aware variant (`amlich-47wn`). Layers declared
//!   interaction features on top of v2.1: axis subtotals still come from
//!   feature aggregation, then interaction deltas are applied as a
//!   post-processing step. Divergences from v2.1 are intentional and
//!   reviewed in `assessment_v2_2_interactions`.
//!
//! ## Out of scope here
//!
//! Named hard vetoes (`amlich-l0wu`) ARE populated under `baseline_v2`
//! and `intent_weighted_v2`. The legacy
//! `polarity == Avoid && strength >= 0.8` implicit threshold was lifted
//! into explicit, source-attributed [`VetoEvent`]s that fire on the same
//! source-data states for v1 decision parity.

use crate::{
    advisory::ConsultationIntent,
    almanac::recommendation::{DailyRecommendations, RecommendationBucket},
    assessment::{
        extraction::{
            extract_features, extract_vetoes, resolve_assessment_inputs, ResolvedAssessmentInputs,
        },
        feature::{polarity_sign, FeatureObservation},
        interactions::{
            apply_interaction_deltas, extract_interactions, InteractionWeightTable,
            INTERACTION_WEIGHTS_V2_2,
        },
        trace::{
            AssessmentTrace, AxisAggregation, AxisContributor, AxisWeight, DecisionAggregation,
            VetoEvent,
        },
        weights::{IntentAxisWeightTable, INTENT_AXIS_WEIGHTS_V2_1},
        AssessmentAxes, AssessmentAxis, AssessmentInputs, AxisOutcome, DecisionContribution,
        EvidenceCoverage, NormalizedBirth, PersonalDayAssessment, PersonalDayDecision,
        UnavailableSection,
    },
    birth::{BirthCapability, BirthProfile},
    reasoning::{DecisionConfidence, RecommendationBucket as ReasoningBucket},
    DaySnapshot,
};

/// Stable policy identifier for the v2 personal-day assessment policy.
/// Co-versioned with [`ASSESSMENT_POLICY_V2_VERSION`]: any change to
/// feature extraction, weight tables, axis aggregation, or decision
/// synthesis MUST bump the version.
pub const ASSESSMENT_POLICY_V2_ID: &str = "personal-day-assessment";

/// Current version of the v2 baseline policy. v2 introduces the
/// feature-vector model, source-attributed observations, and calculation
/// trace while preserving v1 axis scores and decision buckets under
/// `baseline_v2`.
pub const ASSESSMENT_POLICY_V2_VERSION: &str = "v2";

/// Version of the v2.1 intent-aware policy (`amlich-lxu3`). Layers the
/// sparse, policy-versioned [`IntentAxisWeightTable`] on top of the v2
/// feature model so the final decision projection reflects what each
/// consultation intent emphasizes. Axis subtotals still match `v2` and
/// `v1` byte-for-byte; only the final decision aggregation changes.
pub const ASSESSMENT_POLICY_V2_1_VERSION: &str = "v2.1";

/// Version of the v2.2 interaction-aware policy (`amlich-47wn`). Layers
/// declared interaction features on top of the v2.1 intent-aware policy:
/// axis subtotals are computed from features as in v2/v2.1, then
/// interaction deltas (`weight × value`) are applied as a post-processing
/// step to the relevant axis subtotals. The interaction weights come from
/// [`INTERACTION_WEIGHTS_V2_2`].
pub const ASSESSMENT_POLICY_V2_2_VERSION: &str = "v2.2";

/// Legacy axis-aggregation multiplier carried over from v1 so baseline_v2
/// reproduces v1 axis scores exactly. The v2.1 intent-aware variant
/// (`amlich-lxu3`) keeps this multiplier for axis subtotals and only
/// changes the final decision aggregation via
/// [`IntentAxisWeightTable`].
const V1_AXIS_DELTA_MULTIPLIER: f32 = 0.3;

/// Versioned policy that owns the v2 personal-day scoring pipeline.
///
/// Construct via [`AssessmentPolicy::baseline_v2`] for the v1-parity
/// baseline, or via [`AssessmentPolicy::intent_weighted_v2`] for the
/// intent-aware v2.1 variant (`amlich-lxu3`). Callers feed the same
/// `(inputs, snapshot, profile, intent)` they would have fed the legacy
/// builder; the policy returns a fully built [`PersonalDayAssessment`]
/// with the calculation [`AssessmentTrace`] attached.
#[derive(Debug, Clone)]
pub struct AssessmentPolicy {
    policy_id: String,
    policy_version: String,
    axis_delta_multiplier: f32,
    /// Optional intent×axis weight table. When `None` (the v1-parity
    /// `baseline_v2` case) the decision aggregation uses equal weights
    /// across the available scored axes. When `Some` (the v2.1
    /// `intent_weighted_v2` case) the aggregation uses per-intent
    /// weights from the table, renormalized over the available axes.
    intent_axis_weights: Option<&'static IntentAxisWeightTable>,
    /// Optional interaction weight table. When `None` (v2 / v2.1) no
    /// interaction terms are evaluated. When `Some` (v2.2
    /// `interaction_aware_v2`) declared interactions are extracted and
    /// their deltas applied to axis subtotals after feature aggregation.
    interaction_weights: Option<&'static InteractionWeightTable>,
}

impl Default for AssessmentPolicy {
    fn default() -> Self {
        Self::baseline_v2()
    }
}

impl AssessmentPolicy {
    /// Baseline v2 policy: v1-compatible weights and aggregation formula
    /// plus the v2 feature-vector model and calculation trace. Use this to
    /// opt into the v2 seam without changing user-visible verdicts.
    pub fn baseline_v2() -> Self {
        Self {
            policy_id: ASSESSMENT_POLICY_V2_ID.to_string(),
            policy_version: ASSESSMENT_POLICY_V2_VERSION.to_string(),
            axis_delta_multiplier: V1_AXIS_DELTA_MULTIPLIER,
            intent_axis_weights: None,
            interaction_weights: None,
        }
    }

    /// Intent-aware v2.1 policy (`amlich-lxu3`). Same feature model and
    /// axis aggregation as [`baseline_v2`](Self::baseline_v2) — axis
    /// subtotals still match v1 byte-for-byte — but the final decision
    /// aggregation uses per-intent axis weights from
    /// [`INTENT_AXIS_WEIGHTS_V2_1`] instead of an equal-weight average,
    /// so different intents can produce different final projections.
    ///
    /// Weights of unavailable axes are excluded and the remaining
    /// weights renormalize to sum to 1.0, preserving the
    /// "unavailable is not zero" contract from `amlich-7bm4`.
    pub fn intent_weighted_v2() -> Self {
        Self {
            policy_id: ASSESSMENT_POLICY_V2_ID.to_string(),
            policy_version: ASSESSMENT_POLICY_V2_1_VERSION.to_string(),
            axis_delta_multiplier: V1_AXIS_DELTA_MULTIPLIER,
            intent_axis_weights: Some(&INTENT_AXIS_WEIGHTS_V2_1),
            interaction_weights: None,
        }
    }

    /// Interaction-aware v2.2 policy (`amlich-47wn`). Layers declared
    /// interaction features on top of the v2.1 intent-aware policy: same
    /// feature model, same intent-aware axis weights for the decision
    /// aggregation, plus typed interaction terms that apply synergistic
    /// deltas to axis subtotals after feature aggregation.
    ///
    /// Interactions fire only on explicitly declared conditions (spec: "No
    /// interaction is inferred merely because two source facts coexist").
    /// Each interaction can fire at most once per assessment, so duplicate
    /// inputs cannot inflate results.
    pub fn interaction_aware_v2() -> Self {
        Self {
            policy_id: ASSESSMENT_POLICY_V2_ID.to_string(),
            policy_version: ASSESSMENT_POLICY_V2_2_VERSION.to_string(),
            axis_delta_multiplier: V1_AXIS_DELTA_MULTIPLIER,
            intent_axis_weights: Some(&INTENT_AXIS_WEIGHTS_V2_1),
            interaction_weights: Some(&INTERACTION_WEIGHTS_V2_2),
        }
    }

    pub fn policy_id(&self) -> &str {
        &self.policy_id
    }

    pub fn policy_version(&self) -> &str {
        &self.policy_version
    }

    /// Run the v2 scoring pipeline against the supplied inputs and return
    /// a fully built [`PersonalDayAssessment`] with the calculation
    /// [`AssessmentTrace`] populated.
    ///
    /// Pure and deterministic: identical `(policy, inputs, snapshot,
    /// profile, intent)` quintuples produce identical assessments.
    pub fn evaluate(
        &self,
        inputs: AssessmentInputs,
        snapshot: &DaySnapshot,
        profile: &BirthProfile,
        intent: ConsultationIntent,
    ) -> PersonalDayAssessment {
        crate::build_count::canonical_assessment_built();

        let capability = profile.capability();
        let capability_tier = capability.default_tier();
        let normalized_birth = NormalizedBirth::from(profile);
        let ruleset_id = snapshot.ruleset_id.clone();
        let ruleset_version = snapshot.ruleset_version.clone();
        let profile_id = snapshot.profile.clone();

        let resolved = resolve_assessment_inputs(snapshot, profile, capability, inputs);
        let features = extract_features(snapshot, profile, intent, capability, &resolved);
        let vetoes = extract_vetoes(snapshot, profile, intent, capability, &resolved);

        // Declared interaction terms (amlich-47wn). Only the v2.2 policy
        // wires in an interaction weight table; v2 / v2.1 produce no
        // interactions, preserving their parity contracts.
        let interactions = match self.interaction_weights {
            Some(table) => extract_interactions(
                &features,
                snapshot,
                profile,
                intent,
                &capability,
                &resolved,
                table,
            ),
            None => Vec::new(),
        };

        let evidence = build_evidence_coverage(&resolved);

        let contributions =
            project_features_to_contributions(&features, &self.policy_id, &self.policy_version);

        let (mut axes, mut axis_aggregations) = self.aggregate_axes(&features, &capability);

        // Apply interaction deltas to axis subtotals after feature
        // aggregation (amlich-47wn). Each interaction contributes
        // `weight × value` to its target axis; the subtotal is clamped to
        // [0, 1] and the verdict refreshed. Under v2 / v2.1 this is a
        // no-op (interactions is empty).
        if !interactions.is_empty() {
            apply_interaction_deltas(&mut axes, &mut axis_aggregations, &interactions);
        }

        let (decision, decision_aggregation) = self.synthesize_decision(
            &axes,
            &contributions,
            &vetoes,
            &capability,
            resolved.recommendations.as_ref(),
            intent,
        );

        let primary_conclusion = synthesize_primary_conclusion(decision.bucket, &axes, intent);
        let decision = PersonalDayDecision {
            bucket: decision.bucket,
            confidence: decision.confidence,
            semantic: decision.semantic,
            primary_conclusion,
            decision_score: decision.decision_score,
            context_is_clear: evidence.has_chart || evidence.has_yearly_han,
        };

        let unavailable_sections = build_unavailable_sections(&capability, &resolved);

        let trace = AssessmentTrace {
            policy_id: self.policy_id.clone(),
            policy_version: self.policy_version.clone(),
            features,
            axes: axis_aggregations,
            decision: decision_aggregation,
            vetoes,
            interactions,
        };

        PersonalDayAssessment {
            ruleset_id,
            ruleset_version,
            profile: profile_id,
            policy_id: self.policy_id.clone(),
            policy_version: self.policy_version.clone(),
            intent,
            capability,
            capability_tier,
            normalized_birth,
            axes,
            contributions,
            decision,
            unavailable_sections,
            evidence,
            trace: Some(trace),
        }
    }

    /// Aggregate features into the five canonical axes using the baseline-v2
    /// formula (v1 parity). Returns both the typed [`AssessmentAxes`] (for
    /// the assessment envelope) and the trace's [`AxisAggregation`] list
    /// (for the Evidence Graph projection).
    fn aggregate_axes(
        &self,
        features: &[FeatureObservation],
        capability: &BirthCapability,
    ) -> (AssessmentAxes, Vec<AxisAggregation>) {
        let mut axis_outcomes: Vec<AxisOutcome> = Vec::with_capacity(AssessmentAxis::ALL.len());
        let mut axis_traces: Vec<AxisAggregation> = Vec::with_capacity(AssessmentAxis::ALL.len());

        for axis in AssessmentAxis::ALL {
            let axis_features = features
                .iter()
                .filter(|f| f.feature_id.default_axis() == axis);

            let (outcome, trace) = match axis {
                AssessmentAxis::EvidenceCoverage => {
                    // v1 parity: evidence-coverage score is the capability
                    // coverage ratio, not a feature aggregation.
                    let score = evidence_coverage_score(capability);
                    let outcome = AxisOutcome::from_score(axis, score);
                    (
                        outcome.clone(),
                        AxisAggregation {
                            axis,
                            contributors: Vec::new(),
                            subtotal: outcome.score,
                            verdict: outcome.verdict.clone(),
                            unavailable_reason: None,
                        },
                    )
                }
                AssessmentAxis::PersonalAlignment if !capability.has_gender => {
                    let reason = "requires gender for personal interaction facts";
                    (
                        AxisOutcome::unavailable(axis, reason),
                        AxisAggregation {
                            axis,
                            contributors: Vec::new(),
                            subtotal: None,
                            verdict: "unavailable".to_string(),
                            unavailable_reason: Some(reason.to_string()),
                        },
                    )
                }
                AssessmentAxis::AnnualPressure
                    if !features
                        .iter()
                        .any(|f| f.feature_id.default_axis() == axis && !f.is_unavailable()) =>
                {
                    // v1 parity: AnnualPressure is unavailable when no
                    // yearly Hạn assessment could be produced (no gender).
                    // The check counts only *available* features because
                    // amlich-l0wu now emits explicit unavailable
                    // observations for capability gaps — those should not
                    // flip the axis to "available with neutral score".
                    let reason = "requires gender for yearly Hạn assessment";
                    (
                        AxisOutcome::unavailable(axis, reason),
                        AxisAggregation {
                            axis,
                            contributors: Vec::new(),
                            subtotal: None,
                            verdict: "unavailable".to_string(),
                            unavailable_reason: Some(reason.to_string()),
                        },
                    )
                }
                _ => self.aggregate_one_axis(axis, axis_features),
            };

            axis_outcomes.push(outcome);
            axis_traces.push(trace);
        }

        let axes = AssessmentAxes {
            generic_day_quality: axis_outcomes
                .iter()
                .find(|o| o.axis == AssessmentAxis::GenericDayQuality)
                .cloned()
                .expect("generic_day_quality axis"),
            intent_fit: axis_outcomes
                .iter()
                .find(|o| o.axis == AssessmentAxis::IntentFit)
                .cloned()
                .expect("intent_fit axis"),
            personal_alignment: axis_outcomes
                .iter()
                .find(|o| o.axis == AssessmentAxis::PersonalAlignment)
                .cloned()
                .expect("personal_alignment axis"),
            annual_pressure: axis_outcomes
                .iter()
                .find(|o| o.axis == AssessmentAxis::AnnualPressure)
                .cloned()
                .expect("annual_pressure axis"),
            evidence_coverage: axis_outcomes
                .iter()
                .find(|o| o.axis == AssessmentAxis::EvidenceCoverage)
                .cloned()
                .expect("evidence_coverage axis"),
        };

        (axes, axis_traces)
    }

    /// Aggregate one axis's available features into a score using the
    /// baseline-v2 (v1-parity) formula. Returns both the typed
    /// [`AxisOutcome`] and the trace record.
    fn aggregate_one_axis<'a, I>(
        &self,
        axis: AssessmentAxis,
        features: I,
    ) -> (AxisOutcome, AxisAggregation)
    where
        I: IntoIterator<Item = &'a FeatureObservation>,
    {
        let features: Vec<&FeatureObservation> = features.into_iter().collect();

        if features.is_empty() {
            // v1 returns a neutral 0.5 outcome when no contributions landed
            // on the axis (i.e., it was available but nothing matched).
            let neutral = AxisOutcome::from_score(axis, 0.5);
            let trace = AxisAggregation {
                axis,
                contributors: Vec::new(),
                subtotal: Some(0.5),
                verdict: neutral.verdict.clone(),
                unavailable_reason: None,
            };
            return (neutral, trace);
        }

        let mut contributors: Vec<AxisContributor> = Vec::with_capacity(features.len());
        let mut delta = 0.0_f32;
        let mut total_weight = 0.0_f32;

        for feature in &features {
            // Unavailable features are excluded from the aggregation and
            // reported only in the trace (amlich-7bm4 contract:
            // unavailable != zero). baseline_v2's extract_features only
            // emits available observations, but the guard keeps the
            // aggregation robust for future policy versions that emit
            // explicit unavailable observations alongside available ones.
            if feature.is_unavailable() {
                continue;
            }
            let signed_value = feature.signed_value().unwrap_or(0.0);
            // v1 parity: weight each contribution by its raw strength in
            // the denominator, while the numerator uses
            // sign(polarity) * strength * 0.3.
            let contribution = polarity_sign(feature.polarity) * feature.strength;
            delta += contribution * self.axis_delta_multiplier;
            total_weight += feature.strength;

            contributors.push(AxisContributor {
                feature_id: feature.feature_id,
                contribution_id: feature.contribution_id.clone(),
                signed_value,
                applied_weight: self.axis_delta_multiplier,
                contribution: contribution * self.axis_delta_multiplier,
            });
        }

        let balance = if total_weight > 0.0 {
            (delta / total_weight).clamp(-0.5, 0.5)
        } else {
            0.0
        };
        let score = (0.5 + balance).clamp(0.0, 1.0);
        let outcome = AxisOutcome::from_score(axis, score);
        let trace = AxisAggregation {
            axis,
            contributors,
            subtotal: outcome.score,
            verdict: outcome.verdict.clone(),
            unavailable_reason: None,
        };
        (outcome, trace)
    }

    /// Synthesize the final decision. Under baseline_v2, named hard
    /// vetoes (`amlich-l0wu`) force the `Avoid` bucket with deterministic
    /// precedence before any weighted suitability aggregation. The
    /// remaining paths (recommendation-bucket overrides, axis averaging)
    /// reproduce the v1 decision formula.
    ///
    /// Under v2.1 (`amlich-lxu3`), the axis averaging is replaced by an
    /// intent-specific weight vector from
    /// [`INTENT_AXIS_WEIGHTS_V2_1`], renormalized over the available
    /// axes so a capability gap cannot inflate the score.
    fn synthesize_decision(
        &self,
        axes: &AssessmentAxes,
        contributions: &[DecisionContribution],
        vetoes: &[VetoEvent],
        capability: &BirthCapability,
        recommendations: Option<&DailyRecommendations>,
        intent: ConsultationIntent,
    ) -> (PersonalDayDecision, DecisionAggregation) {
        // Named hard vetoes win over any weighted signal (amlich-l0wu).
        // The veto list is produced by [`extract_vetoes`] from explicit
        // domain declarations — an ordinary negative contribution can no
        // longer flip the decision to `Avoid` merely by crossing a
        // strength threshold.
        let hard_veto = !vetoes.is_empty();
        let _ = contributions; // contributions no longer drive the veto; kept for future weighting

        let axis_scores: [(AssessmentAxis, Option<f32>); 4] = [
            (
                AssessmentAxis::GenericDayQuality,
                axes.generic_day_quality.score,
            ),
            (AssessmentAxis::IntentFit, axes.intent_fit.score),
            (
                AssessmentAxis::PersonalAlignment,
                axes.personal_alignment.score,
            ),
            (AssessmentAxis::AnnualPressure, axes.annual_pressure.score),
        ];

        let available: Vec<(AssessmentAxis, f32)> = axis_scores
            .iter()
            .filter_map(|(axis, score)| score.map(|s| (*axis, s)))
            .collect();
        let unavailable_axes: Vec<AssessmentAxis> = axis_scores
            .iter()
            .filter_map(|(axis, score)| if score.is_none() { Some(*axis) } else { None })
            .collect();
        let available_axes: Vec<AssessmentAxis> = available.iter().map(|(a, _)| *a).collect();

        // Pick the axis weights for the available axes. baseline_v2
        // (intent_axis_weights == None) keeps the equal-weight aggregate
        // for v1 parity; v2.1 (Some table) uses the per-intent weights,
        // renormalized over the available axes so a missing axis cannot
        // inflate the score (amlich-lxu3 contract).
        let (axis_weights, average_score) =
            self.aggregate_decision_score(&available, intent, &available_axes);

        let intent_primary_bucket = recommendations.and_then(|rec| {
            rec.activities
                .iter()
                .find(|a| a.activity_id == intent.primary_activity())
                .map(|a| a.bucket)
        });

        let (bucket, semantic, decision_score) = if hard_veto {
            (
                ReasoningBucket::Avoid,
                "override_avoid".to_string(),
                Some(0.15),
            )
        } else if let Some(primary_bucket) = intent_primary_bucket {
            match primary_bucket {
                RecommendationBucket::KyManh => (
                    ReasoningBucket::Avoid,
                    "override_avoid".to_string(),
                    Some(0.2),
                ),
                RecommendationBucket::Tranh => (
                    ReasoningBucket::Cautious,
                    "resistance_led_cautious".to_string(),
                    Some(average_score.unwrap_or(0.4)),
                ),
                _ => {
                    let score = average_score.unwrap_or(0.5);
                    let bucket = classify_score_into_bucket(score);
                    (
                        bucket,
                        semantic_for_bucket(bucket),
                        Some(score.clamp(0.0, 1.0)),
                    )
                }
            }
        } else {
            let score = average_score.unwrap_or(0.5);
            let bucket = classify_score_into_bucket(score);
            (
                bucket,
                semantic_for_bucket(bucket),
                Some(score.clamp(0.0, 1.0)),
            )
        };

        let decision = PersonalDayDecision {
            bucket,
            confidence: confidence_from_capability(capability),
            semantic,
            primary_conclusion: String::new(),
            decision_score: decision_score.map(|s| s.clamp(0.0, 1.0)),
            context_is_clear: false,
        };

        let aggregation = DecisionAggregation {
            axis_weights,
            available_axes,
            unavailable_axes,
            decision_score: decision_score.map(|s| s.clamp(0.0, 1.0)),
            bucket,
        };

        (decision, aggregation)
    }

    /// Combine the available axis scores into a single decision projection.
    /// Returns the per-axis [`AxisWeight`] entries (after renormalization,
    /// so they sum to 1.0 over the available axes) and the resulting
    /// weighted-average score.
    ///
    /// - `baseline_v2` (no intent table): equal weights `1 / N_available`,
    ///   matching v1 byte-for-byte.
    /// - `intent_weighted_v2` (v2.1): per-intent weights from the table,
    ///   renormalized over the available axes. If the table somehow
    ///   yields a zero total for the available axes (defensive: every
    ///   real intent×axis entry is positive), falls back to equal weight
    ///   rather than dividing by zero.
    fn aggregate_decision_score(
        &self,
        available: &[(AssessmentAxis, f32)],
        intent: ConsultationIntent,
        available_axes: &[AssessmentAxis],
    ) -> (Vec<AxisWeight>, Option<f32>) {
        if available.is_empty() {
            return (Vec::new(), None);
        }

        let raw_weights: Vec<(AssessmentAxis, f32)> = match self.intent_axis_weights {
            None => available.iter().map(|(axis, _)| (*axis, 1.0_f32)).collect(),
            Some(table) => {
                let entry = table.weights_for(intent);
                available
                    .iter()
                    .map(|(axis, _)| (*axis, entry.weight_for(*axis).unwrap_or(0.0)))
                    .collect()
            }
        };

        let total_raw: f32 = raw_weights.iter().map(|(_, w)| *w).sum();
        let n = available.len() as f32;

        // Defensive fallback: every v2.1 table entry is positive, so this
        // branch only triggers under a malformed future table. Keep the
        // decision finite and report equal weights so the trace stays
        // honest about what was actually applied.
        let (normalized, score) = if total_raw > 0.0 {
            let normalized: Vec<AxisWeight> = raw_weights
                .iter()
                .map(|(axis, w)| AxisWeight {
                    axis: *axis,
                    weight: *w / total_raw,
                })
                .collect();
            let weighted: f32 = available
                .iter()
                .zip(normalized.iter())
                .map(|((_, s), w)| *s * w.weight)
                .sum();
            (normalized, weighted)
        } else {
            let eq = 1.0 / n;
            let normalized: Vec<AxisWeight> = available_axes
                .iter()
                .map(|axis| AxisWeight {
                    axis: *axis,
                    weight: eq,
                })
                .collect();
            let sum: f32 = available.iter().map(|(_, s)| *s).sum();
            (normalized, sum * eq)
        };

        (normalized, Some(score.clamp(0.0, 1.0)))
    }
}

/// Project feature observations into the legacy v1-compatible
/// [`DecisionContribution`] shape so consumer contracts (advisory,
/// reasoning, API DTO) remain unchanged. The projection is lossless:
/// contribution_id, axis, polarity, strength, source evidence, and note
/// all map 1:1.
fn project_features_to_contributions(
    features: &[FeatureObservation],
    policy_id: &str,
    policy_version: &str,
) -> Vec<DecisionContribution> {
    features
        .iter()
        .filter(|f| !f.is_unavailable())
        .map(|f| DecisionContribution {
            contribution_id: f.contribution_id.clone(),
            axis: f.feature_id.default_axis(),
            polarity: f.polarity,
            strength: f.strength,
            policy_id: policy_id.to_string(),
            policy_version: policy_version.to_string(),
            ruleset_id: f.ruleset_id.clone(),
            ruleset_version: f.ruleset_version.clone(),
            source_evidence: f.source_evidence.clone(),
            availability: f.availability.clone(),
            note: f.note.clone(),
        })
        .collect()
}

fn build_evidence_coverage(resolved: &ResolvedAssessmentInputs) -> EvidenceCoverage {
    EvidenceCoverage {
        has_chart: resolved.chart.is_some(),
        has_analysis: resolved.analysis.is_some(),
        has_yearly_han: resolved.yearly_han.is_some(),
        has_kua: resolved.kua.is_some(),
        has_kim_lau: resolved
            .yearly_han
            .as_ref()
            .map(|h| h.kim_lau.in_kim_lau)
            .unwrap_or(false),
        has_tam_tai: resolved
            .yearly_han
            .as_ref()
            .map(|h| h.tam_tai.in_tam_tai)
            .unwrap_or(false),
        has_hoang_oc: resolved.yearly_han.is_some(),
        has_thai_tue: resolved
            .yearly_han
            .as_ref()
            .map(|h| h.thai_tue.has_conflict)
            .unwrap_or(false),
        has_sao_han: resolved
            .yearly_han
            .as_ref()
            .map(|h| h.sao_han.is_han)
            .unwrap_or(false),
        recommendation_count: resolved
            .recommendations
            .as_ref()
            .map(|r| r.activities.len())
            .unwrap_or(0),
    }
}

/// Build the legacy v1-compatible unavailable-section list. Each entry
/// surfaces a section the policy could not score, the human-readable
/// reason, and the required fields a caller would need to supply. Emission
/// order matches v1 so the serialized envelope is byte-identical apart
/// from policy metadata.
fn build_unavailable_sections(
    capability: &BirthCapability,
    resolved: &ResolvedAssessmentInputs,
) -> Vec<UnavailableSection> {
    let mut sections = Vec::new();

    if !capability.has_gender {
        sections.push(UnavailableSection {
            section: "personal_alignment".to_string(),
            reason: "requires gender for personal interaction facts".to_string(),
            required_fields: vec!["gender".to_string()],
        });
    }

    if !capability.has_time {
        sections.push(UnavailableSection {
            section: "personal_hours".to_string(),
            reason: "requires explicit birth time for personal-hour context".to_string(),
            required_fields: vec!["hour".to_string(), "minute".to_string()],
        });
    }

    if !capability.has_gender {
        sections.push(UnavailableSection {
            section: "annual_han".to_string(),
            reason: "requires gender for yearly Hạn assessment".to_string(),
            required_fields: vec!["gender".to_string()],
        });
    }

    // Defensive: future policy variants may produce a Hạn without gender.
    // baseline_v2 always gates this on gender, so the resolved.yearly_han
    // check would be redundant here.
    let _ = resolved;

    sections
}

/// v1-parity evidence-coverage score: simple capability coverage across
/// the four birth fields. baseline_v2 reproduces this so the
/// `EvidenceCoverage` axis score matches v1 exactly.
fn evidence_coverage_score(capability: &BirthCapability) -> f32 {
    let coverage_fields = [
        capability.has_date,
        capability.has_time,
        capability.has_gender,
        capability.has_location,
    ];
    let coverage_count = coverage_fields.iter().filter(|v| **v).count() as f32;
    coverage_count / 4.0
}

fn classify_score_into_bucket(score: f32) -> ReasoningBucket {
    let s = score.clamp(0.0, 1.0);
    match s {
        s if s >= 0.7 => ReasoningBucket::Favorable,
        s if s >= 0.45 => ReasoningBucket::Mixed,
        s if s >= 0.3 => ReasoningBucket::Cautious,
        _ => ReasoningBucket::Avoid,
    }
}

fn semantic_for_bucket(bucket: ReasoningBucket) -> String {
    match bucket {
        ReasoningBucket::Favorable => "favorable_clear",
        ReasoningBucket::Mixed => "favorable_contextual",
        ReasoningBucket::Cautious => "resistance_led_cautious",
        ReasoningBucket::Avoid => "conflicted_cautious",
    }
    .to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_v2_policy_metadata_is_versioned() {
        let policy = AssessmentPolicy::baseline_v2();
        assert_eq!(policy.policy_id(), ASSESSMENT_POLICY_V2_ID);
        assert_eq!(policy.policy_version(), ASSESSMENT_POLICY_V2_VERSION);
        assert_eq!(policy.policy_version(), "v2");
        // baseline_v2 is the v1-parity baseline: no intent-aware
        // weight table. The decision aggregation falls back to equal
        // weights across the available scored axes.
        assert!(
            policy.intent_axis_weights.is_none(),
            "baseline_v2 must not wire in an intent-aware weight table"
        );
    }

    #[test]
    fn intent_weighted_v2_policy_metadata_is_versioned() {
        // amlich-lxu3: the v2.1 variant carries a distinct policy
        // version and an intent-aware weight table.
        let policy = AssessmentPolicy::intent_weighted_v2();
        assert_eq!(policy.policy_id(), ASSESSMENT_POLICY_V2_ID);
        assert_eq!(policy.policy_version(), ASSESSMENT_POLICY_V2_1_VERSION);
        assert_eq!(policy.policy_version(), "v2.1");
        let table = policy
            .intent_axis_weights
            .expect("v2.1 must wire in the intent-aware weight table");
        assert_eq!(table.policy_version, ASSESSMENT_POLICY_V2_1_VERSION);
        assert!(
            !table.entries.is_empty(),
            "v2.1 weight table must declare at least one intent entry"
        );
    }

    #[test]
    fn v2_and_v2_1_share_policy_family_but_diverge_on_version() {
        // The policy_id family stays stable across v2 / v2.1 (callers
        // can tell they're looking at the same assessment seam). The
        // policy_version carries the divergence signal.
        let v2 = AssessmentPolicy::baseline_v2();
        let v2_1 = AssessmentPolicy::intent_weighted_v2();
        assert_eq!(v2.policy_id(), v2_1.policy_id());
        assert_ne!(v2.policy_version(), v2_1.policy_version());
    }

    #[test]
    fn baseline_v2_constants_do_not_collide_with_v1() {
        // v1 constants stay pinned at "v1" until the stability gate
        // (amlich-31oa) retires them; v2 introduces separate constants.
        assert_eq!(
            crate::assessment::ASSESSMENT_POLICY_VERSION,
            "v1",
            "v1 metadata must stay pinned (locked by assessment_parity_contract)"
        );
    }

    #[test]
    fn evidence_coverage_score_matches_v1_capability_ratio() {
        let mut none = BirthCapability::default();
        none.has_date = false;
        none.has_time = false;
        none.has_gender = false;
        none.has_location = false;
        assert!((evidence_coverage_score(&none) - 0.0).abs() < 1e-6);

        let mut all = BirthCapability::default();
        all.has_date = true;
        all.has_time = true;
        all.has_gender = true;
        all.has_location = true;
        assert!((evidence_coverage_score(&all) - 1.0).abs() < 1e-6);

        let mut half = BirthCapability::default();
        half.has_date = true;
        half.has_time = false;
        half.has_gender = true;
        half.has_location = false;
        assert!((evidence_coverage_score(&half) - 0.5).abs() < 1e-6);
    }
}
