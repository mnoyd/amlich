mod action;
mod export;
mod facts;
mod personal;
mod signals;
mod synthesis;
mod types;
mod vector;

pub use action::InitiationOpeningVector;
pub use export::export_reasoning_graph;
pub use facts::build_fact_graph;
pub use personal::PersonalReasoningInput;
pub use signals::derive_interpreted_signals;
pub use synthesis::{build_initiation_opening_decision, build_initiation_opening_reasoning_bundle};
pub use types::{
    ActionId, DecisionConfidence, EdgeEffect, InitiationOpeningDecision, InterpretedAxis, NodeKind,
    InitiationOpeningDecisionExport, InitiationOpeningReasoningBundle, ReasoningAxisScore,
    ReasoningConclusionSemantic, ReasoningEdge, ReasoningEdgeExport, ReasoningEdgeJustification,
    ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily, ReasoningGraph,
    ReasoningGraphExport, ReasoningNode, ReasoningNodeExport, ReasoningNodeSeverity,
    ReasoningNote, RecommendationBucket,
};
pub use vector::assemble_action_vector;
