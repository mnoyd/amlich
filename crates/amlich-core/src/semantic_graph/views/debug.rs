use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::helpers::NodeViewAccumulator;
use super::visualization::VisualizationGraph;
use crate::almanac::recommendation::{
    collect_recommendation_hits, synthesize_daily_recommendations, RecommendationSynthesisContext,
};
use crate::calculate_day_snapshot;
use crate::semantic_graph::builders::{
    build_day_snapshot_graph, build_recommendation_evidence_graph_connected,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugSemanticGraphInspection {
    pub surface: String,
    pub date: DebugInspectionDate,
    pub visualization: VisualizationGraph,
    pub summary: DebugInspectionSummary,
    pub cluster_counts: HashMap<String, usize>,
    pub semantic_kind_counts: HashMap<String, usize>,
    pub severity_counts: HashMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInspectionDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugInspectionSummary {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub clusters: Vec<String>,
    pub semantic_kinds: Vec<String>,
    pub has_recommendation_evidence: bool,
}

pub fn debug_inspect_semantic_graph(
    day: i32,
    month: i32,
    year: i32,
    include_recommendations: bool,
) -> DebugSemanticGraphInspection {
    let snapshot = calculate_day_snapshot(day, month, year);
    let day_graph = build_day_snapshot_graph(&snapshot);

    let graph = if include_recommendations {
        let context = RecommendationSynthesisContext {
            day_chi: &snapshot.context.canchi.day.chi,
            day_fortune: &snapshot.day_fortune,
            gio_hoang_dao: Some(&snapshot.context.gio_hoang_dao),
            tiet_khi_name: Some(&snapshot.context.tiet_khi.name),
            profile_id: Some("default"),
            event_kind: None,
            enabled_pack_ids: &[],
        };

        let recommendations = synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[])
            .expect("recommendation hits should collect for debug surface");

        build_recommendation_evidence_graph_connected(
            year,
            month,
            day,
            "default",
            &recommendations.activities,
            &hits,
            &day_graph,
        )
    } else {
        day_graph
    };

    let visualization = VisualizationGraph::from_semantic_graph(&graph);

    let mut acc = NodeViewAccumulator::new();
    for (_, node) in graph.nodes() {
        acc.accumulate(node);
    }

    let cluster_counts = acc.cluster_counts;
    let semantic_kind_counts = acc.semantic_kind_counts;
    let severity_counts = acc.severity_counts;

    let clusters: Vec<String> = cluster_counts.keys().cloned().collect();
    let semantic_kinds: Vec<String> = semantic_kind_counts.keys().cloned().collect();
    let has_recommendation_evidence = clusters
        .iter()
        .any(|c| c == "recommendation-evidence" || c == "recommendation-summary");

    let summary = DebugInspectionSummary {
        total_nodes: visualization.nodes.len(),
        total_edges: visualization.edges.len(),
        clusters,
        semantic_kinds,
        has_recommendation_evidence,
    };

    DebugSemanticGraphInspection {
        surface: "debug_semantic_graph_inspector".to_string(),
        date: DebugInspectionDate { year, month, day },
        visualization,
        summary,
        cluster_counts,
        semantic_kind_counts,
        severity_counts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_inspection_day_only_returns_graph() {
        let inspection = debug_inspect_semantic_graph(10, 2, 2024, false);

        assert_eq!(inspection.surface, "debug_semantic_graph_inspector");
        assert_eq!(inspection.date.year, 2024);
        assert_eq!(inspection.date.month, 2);
        assert_eq!(inspection.date.day, 10);
        assert!(
            inspection.summary.total_nodes > 0,
            "should have nodes in day snapshot graph"
        );
        assert!(
            !inspection.summary.clusters.is_empty(),
            "should have clusters"
        );
        assert!(
            !inspection.cluster_counts.is_empty(),
            "cluster_counts should be populated"
        );
        assert!(
            !inspection.semantic_kind_counts.is_empty(),
            "semantic_kind_counts should be populated"
        );
    }

    #[test]
    fn debug_inspection_with_recommendations_adds_evidence() {
        let without = debug_inspect_semantic_graph(10, 2, 2024, false);
        let with = debug_inspect_semantic_graph(10, 2, 2024, true);

        assert!(
            with.summary.total_nodes >= without.summary.total_nodes,
            "connected graph should have at least as many nodes"
        );
        assert!(
            with.summary.has_recommendation_evidence,
            "should have recommendation evidence when requested"
        );
    }

    #[test]
    fn debug_inspection_visualization_nodes_have_required_fields() {
        let inspection = debug_inspect_semantic_graph(10, 2, 2024, true);

        for node in &inspection.visualization.nodes {
            assert!(!node.node_id.is_empty(), "node should have node_id");
            assert!(
                !node.cluster.is_empty(),
                "node {} should have cluster",
                node.node_id
            );
            assert!(
                !node.semantic_kind.is_empty(),
                "node {} should have semantic_kind",
                node.node_id
            );
        }
    }

    #[test]
    fn debug_inspection_visualization_edges_have_required_fields() {
        let inspection = debug_inspect_semantic_graph(10, 2, 2024, true);

        for edge in &inspection.visualization.edges {
            assert!(!edge.edge_id.is_empty(), "edge should have edge_id");
            assert!(
                !edge.from_id.is_empty(),
                "edge {} should have from_id",
                edge.edge_id
            );
            assert!(
                !edge.to_id.is_empty(),
                "edge {} should have to_id",
                edge.edge_id
            );
            assert!(
                !edge.semantic_kind.is_empty(),
                "edge {} should have semantic_kind",
                edge.edge_id
            );
        }
    }

    #[test]
    fn debug_inspection_serializes_to_json() {
        let inspection = debug_inspect_semantic_graph(10, 2, 2024, true);
        let json = serde_json::to_string(&inspection).expect("should serialize to JSON");
        assert!(json.contains("\"surface\""));
        assert!(json.contains("\"visualization\""));
        assert!(json.contains("\"summary\""));
        assert!(json.contains("\"cluster_counts\""));
    }
}
