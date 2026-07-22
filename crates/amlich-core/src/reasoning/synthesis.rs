use crate::advisory::ConsultationIntent;
use crate::birth::BirthProfile;
use crate::semantic_graph::build_reasoning_input_graph;
use crate::DaySnapshot;
use crate::{InitiationOpeningDecision, InitiationOpeningReasoningBundle};

use super::graph_projection::project_semantic_graph_export;
use super::initiation_opening_evaluator::InitiationOpeningEvaluator;
use super::personal::PersonalReasoningInput;
use super::projection::{
    project_initiation_opening_decision, project_initiation_opening_decision_export_with_assessment,
};
use super::ActionEvaluator;

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

    // amlich-mwbp.6: build the canonical assessment when a birth profile
    // reaches the reasoning pipeline. It becomes the axis-score source of
    // truth for the decision_export. The graph-based evaluation stays in
    // charge of typed notes and prose; only the numerical/typed axes move
    // to the canonical assessment.
    let assessment = personal_input.map(|p| {
        let profile = BirthProfile {
            day: snapshot.context.solar.day,
            month: snapshot.context.solar.month,
            year: snapshot.context.solar.year,
            time: None,
            timezone: p.birth.timezone,
            longitude: None,
            use_solar_time: false,
            gender: p.birth.gender,
            location_name: None,
        };
        crate::assessment::PersonalDayAssessment::assess(
            snapshot.clone(),
            profile,
            ConsultationIntent::OpeningBusiness,
        )
    });

    let graph =
        project_semantic_graph_export(&semantic_graph, &evaluation, snapshot, personal_input);

    Ok(InitiationOpeningReasoningBundle {
        decision: project_initiation_opening_decision(&evaluation),
        decision_export: project_initiation_opening_decision_export_with_assessment(
            &evaluation,
            assessment.as_ref(),
        ),
        graph,
    })
}
