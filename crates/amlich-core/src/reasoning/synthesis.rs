use crate::advisory::ConsultationIntent;
use crate::assessment::PersonalDayAssessment;
use crate::birth::BirthProfile;
use crate::semantic_graph::build_reasoning_input_graph_with_facts;
use crate::DaySnapshot;
use crate::{InitiationOpeningDecision, InitiationOpeningReasoningBundle};

use super::graph_projection::project_semantic_graph_export_with_facts;
use super::initiation_opening_evaluator::InitiationOpeningEvaluator;
use super::personal::{PersonalAssessmentFacts, PersonalReasoningInput};
use super::projection::{
    project_initiation_opening_decision, project_initiation_opening_decision_export_with_assessment,
};

pub fn build_initiation_opening_decision(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<InitiationOpeningDecision, String> {
    Ok(build_initiation_opening_reasoning_bundle(snapshot, personal_input)?.decision)
}

/// Build the reasoning bundle reusing a precomputed
/// [`PersonalAssessmentFacts`] and canonical [`PersonalDayAssessment`] so
/// the chart, the matrices, and the assessment are not rebuilt alongside
/// the graph build. Per-request request paths must use this entry point —
/// see REPAIR-PLAN.md P2 (`amlich-mwbp.8` finding A-R11).
pub fn build_initiation_opening_reasoning_bundle_with_facts(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
    facts: Option<&PersonalAssessmentFacts>,
    canonical_assessment: Option<&PersonalDayAssessment>,
) -> Result<InitiationOpeningReasoningBundle, String> {
    let bazi_input = personal_input.map(PersonalReasoningInput::to_bazi_input);
    let semantic_graph =
        build_reasoning_input_graph_with_facts(snapshot, bazi_input.as_ref(), facts)?;

    // amlich-mwbp.6: build the canonical assessment when a birth profile
    // reaches the reasoning pipeline. It becomes the axis-score source of
    // truth for the decision_export. The graph-based evaluation stays in
    // charge of typed notes and prose; only the numerical/typed axes move
    // to the canonical assessment. amlich-mwbp.8 P2: reuse the cached
    // assessment when the caller provides one so the per-request request
    // path can dedupe endpoint-local rebuilds.
    let anonymous_owned: Option<PersonalDayAssessment>;
    let assessment: Option<&PersonalDayAssessment> = match canonical_assessment {
        Some(a) => Some(a),
        None => match personal_input {
            Some(p) => {
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
                anonymous_owned = Some(PersonalDayAssessment::assess(
                    snapshot.clone(),
                    profile,
                    ConsultationIntent::OpeningBusiness,
                ));
                anonymous_owned.as_ref()
            }
            None => {
                // Anonymous callers still need an axis-score backbone so
                // the decision_export has six typed axes (the evaluator
                // requires a non-empty `canonical_assessment`). The
                // capability tier for an anonymous profile produces an
                // empty assessment with the six core axes, no chart, and
                // no yearly_han — see assessment::PersonalDayAssessmentBuilder.
                anonymous_owned = Some(PersonalDayAssessment::assess(
                    snapshot.clone(),
                    BirthProfile {
                        day: snapshot.context.solar.day,
                        month: snapshot.context.solar.month,
                        year: snapshot.context.solar.year,
                        time: None,
                        timezone: crate::types::VIETNAM_TIMEZONE,
                        longitude: None,
                        use_solar_time: false,
                        gender: None,
                        location_name: None,
                    },
                    ConsultationIntent::OpeningBusiness,
                ));
                anonymous_owned.as_ref()
            }
        },
    };

    let evaluator = InitiationOpeningEvaluator::new();
    let evaluation = evaluator.evaluate_with_facts(
        &semantic_graph,
        snapshot,
        personal_input,
        facts,
        assessment,
    )?;

    let graph = project_semantic_graph_export_with_facts(
        &semantic_graph,
        &evaluation,
        snapshot,
        personal_input,
        facts,
    );

    Ok(InitiationOpeningReasoningBundle {
        decision: project_initiation_opening_decision(&evaluation),
        decision_export: project_initiation_opening_decision_export_with_assessment(
            &evaluation,
            assessment,
        ),
        graph,
    })
}

pub fn build_initiation_opening_reasoning_bundle(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<InitiationOpeningReasoningBundle, String> {
    let facts = personal_input
        .map(|p| PersonalAssessmentFacts::build(p, snapshot))
        .transpose()?;
    build_initiation_opening_reasoning_bundle_with_facts(
        snapshot,
        personal_input,
        facts.as_ref(),
        None,
    )
}
