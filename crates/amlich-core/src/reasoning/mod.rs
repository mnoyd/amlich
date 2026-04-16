mod action;
mod facts;
mod personal;
mod signals;
mod synthesis;
mod types;
mod vector;

pub use action::InitiationOpeningVector;
pub use facts::build_fact_graph;
pub use personal::PersonalReasoningInput;
pub use signals::derive_interpreted_signals;
pub use synthesis::build_initiation_opening_decision;
pub use types::{
    ActionId, DecisionConfidence, EdgeEffect, InitiationOpeningDecision, InterpretedAxis,
    NodeKind, ReasoningEdge, ReasoningGraph, ReasoningNode, RecommendationBucket,
};
pub use vector::assemble_action_vector;
