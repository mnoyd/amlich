use crate::reasoning::PersonalReasoningInput;
use crate::semantic_graph::SemanticGraph;
use crate::DaySnapshot;

use super::types::{
    ActionId, DecisionConfidence, ReasoningAxisScore, ReasoningConclusionSemantic, ReasoningNote,
    RecommendationBucket,
};

#[derive(Debug, Clone)]
pub struct ActionEvaluation {
    pub action_id: ActionId,
    pub bucket: RecommendationBucket,
    pub confidence: DecisionConfidence,
    pub semantic: ReasoningConclusionSemantic,
    pub context_is_clear: bool,
    pub primary_conclusion: String,
    pub strongest_supports: Vec<ReasoningNote>,
    pub strongest_resistances: Vec<ReasoningNote>,
    pub override_factors: Vec<ReasoningNote>,
    pub conflict_notes: Vec<ReasoningNote>,
    pub suggested_hours: Vec<String>,
    pub suggested_directions: Vec<String>,
    pub axis_scores: Vec<ReasoningAxisScore>,
    pub referenced_node_ids: Vec<String>,
}

impl ActionEvaluation {
    pub fn empty(action_id: ActionId) -> Self {
        Self {
            action_id,
            bucket: RecommendationBucket::Mixed,
            confidence: DecisionConfidence::Low,
            semantic: ReasoningConclusionSemantic::FavorableContextual,
            context_is_clear: false,
            primary_conclusion: String::new(),
            strongest_supports: Vec::new(),
            strongest_resistances: Vec::new(),
            override_factors: Vec::new(),
            conflict_notes: Vec::new(),
            suggested_hours: Vec::new(),
            suggested_directions: Vec::new(),
            axis_scores: Vec::new(),
            referenced_node_ids: Vec::new(),
        }
    }
}

pub trait ActionEvaluator: Send + Sync {
    fn action_id(&self) -> ActionId;

    fn select_subgraph(
        &self,
        graph: &SemanticGraph,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
    ) -> Result<SemanticGraph, String>;

    fn evaluate(
        &self,
        graph: &SemanticGraph,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
    ) -> Result<ActionEvaluation, String>;
}
