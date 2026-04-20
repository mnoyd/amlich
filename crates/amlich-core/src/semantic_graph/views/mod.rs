mod convergence;
mod llm;
mod recommendation;
mod subgraph;
mod visualization;

pub use convergence::ConvergenceView;
pub use llm::{
    ClusterSummary, ConvergenceFactRef, ConvergenceHitRef, LlmConvergenceSlice, LlmGraphSlice,
};
pub use recommendation::{
    LlmRecommendationSlice, LlmActivitySummary, RecommendationEvidenceGraphView,
    RecommendationEvidenceView, HitView, SourceBreakdown,
};
pub use subgraph::SubgraphView;
pub use visualization::{VisualizationEdge, VisualizationGraph, VisualizationNode};
