mod builders;
mod edge;
mod graph;
mod ids;
mod node;
mod ontology;
mod provenance;
mod selectors;
mod views;

pub use selectors::{
    EvidenceSelectors, SelectHitDirection, SourceFamilyCounts,
};
pub use builders::{build_reasoning_input_graph, ReasoningInputGraph};
pub use edge::{SemanticEdge, SemanticEdgeId, SemanticEdgeLabel};
pub use graph::{GraphMergeError, GraphValidationError, SemanticGraph};
pub use ids::SemanticId;
pub use node::{NodeOrigin, SemanticNode, SemanticNodeId};
pub use ontology::{ConceptLabel, EdgeConcept, GraphOntology, NodeConcept};
pub use provenance::{ProvenanceEntry, ProvenanceSource, ProvenanceTracker};
pub use views::{
    ConvergenceView, LlmGraphSlice, LlmRecommendationSlice, LlmActivitySummary,
    RecommendationEvidenceGraphView, RecommendationEvidenceView, HitView, SourceBreakdown,
    SubgraphView, VisualizationEdge, VisualizationGraph, VisualizationNode,
};
