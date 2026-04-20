use crate::semantic_graph::build_reasoning_input_graph;
use crate::DaySnapshot;

use super::{
    build_fact_graph, derive_interpreted_signals, export_reasoning_graph, ActionEvaluator,
    ActionId, InitiationOpeningDecision, InitiationOpeningEvaluator,
    InitiationOpeningReasoningBundle, PersonalReasoningInput,
    project_initiation_opening_decision, project_initiation_opening_decision_export,
};

pub fn build_initiation_opening_decision(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<InitiationOpeningDecision, String> {
    Ok(build_initiation_opening_reasoning_bundle(snapshot, personal_input)?.decision)
}

pub fn build_initiation_opening_reasoning_bundle(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<InitiationOpeningReasoningBundle, String> {
    let bazi_input = personal_input.map(PersonalReasoningInput::to_bazi_input);
    let semantic_graph = build_reasoning_input_graph(snapshot, bazi_input.as_ref())?;
    let evaluator = InitiationOpeningEvaluator::new();
    let evaluation = evaluator.evaluate(&semantic_graph, snapshot, personal_input)?;

    Ok(InitiationOpeningReasoningBundle {
        decision: project_initiation_opening_decision(&evaluation),
        decision_export: project_initiation_opening_decision_export(&evaluation),
        graph: build_legacy_reasoning_graph_export(snapshot, personal_input)?,
    })
}

fn build_legacy_reasoning_graph_export(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<super::ReasoningGraphExport, String> {
    let graph = build_fact_graph(ActionId::InitiationOpening, snapshot, personal_input)?;
    let graph = derive_interpreted_signals(graph)?;
    Ok(export_reasoning_graph(&graph))
}
