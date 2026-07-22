use crate::reasoning::action_evaluator::ActionEvaluation;
use crate::reasoning::types::{InitiationOpeningDecision, InitiationOpeningDecisionExport};

pub fn project_initiation_opening_decision(
    evaluation: &ActionEvaluation,
) -> InitiationOpeningDecision {
    InitiationOpeningDecision {
        primary_conclusion: evaluation.primary_conclusion.clone(),
        recommendation_bucket: evaluation.bucket,
        strongest_supports: evaluation
            .strongest_supports
            .iter()
            .map(|n| n.summary_vi.clone())
            .collect(),
        strongest_resistances: evaluation
            .strongest_resistances
            .iter()
            .map(|n| n.summary_vi.clone())
            .collect(),
        override_factors: evaluation
            .override_factors
            .iter()
            .map(|n| n.summary_vi.clone())
            .collect(),
        conflict_notes: evaluation
            .conflict_notes
            .iter()
            .map(|n| n.summary_vi.clone())
            .collect(),
        confidence: evaluation.confidence,
        context_is_clear: evaluation.context_is_clear,
        suggested_hours: evaluation.suggested_hours.clone(),
        suggested_directions: evaluation.suggested_directions.clone(),
    }
}

pub fn project_initiation_opening_decision_export(
    evaluation: &ActionEvaluation,
) -> InitiationOpeningDecisionExport {
    project_initiation_opening_decision_export_with_assessment(evaluation, None)
}

/// amlich-mwbp.6: when a canonical `PersonalDayAssessment` is available,
/// its axis scores MUST be the source of truth on the export. The
/// graph-derived `axis_scores` are kept as a fallback for legacy call
/// paths that do not have an assessment.
pub fn project_initiation_opening_decision_export_with_assessment(
    evaluation: &ActionEvaluation,
    assessment: Option<&crate::assessment::PersonalDayAssessment>,
) -> InitiationOpeningDecisionExport {
    let canonical_axis_scores = assessment.map(|a| a.axis_scores());
    let axis_scores = canonical_axis_scores.unwrap_or_else(|| evaluation.axis_scores.clone());
    InitiationOpeningDecisionExport {
        primary_conclusion: evaluation.primary_conclusion.clone(),
        recommendation_bucket: evaluation.bucket,
        confidence: evaluation.confidence,
        context_is_clear: evaluation.context_is_clear,
        semantic: evaluation.semantic,
        strongest_supports: evaluation.strongest_supports.clone(),
        strongest_resistances: evaluation.strongest_resistances.clone(),
        override_factors: evaluation.override_factors.clone(),
        conflict_notes: evaluation.conflict_notes.clone(),
        suggested_hours: evaluation.suggested_hours.clone(),
        suggested_directions: evaluation.suggested_directions.clone(),
        axis_scores,
    }
}
