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
        axis_scores: evaluation.axis_scores.clone(),
    }
}
