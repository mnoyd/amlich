use crate::semantic_graph::{EdgeConcept, NodeConcept, SemanticGraph};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationNode {
    pub node_id: String,
    pub label: String,
    pub cluster: String,
    pub semantic_kind: String,
    pub severity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shape_hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationEdge {
    pub edge_id: String,
    pub from_id: String,
    pub to_id: String,
    pub label: String,
    pub semantic_kind: String,
    pub weight: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VisualizationGraph {
    pub nodes: Vec<VisualizationNode>,
    pub edges: Vec<VisualizationEdge>,
}

impl VisualizationGraph {
    pub fn from_semantic_graph(graph: &SemanticGraph) -> Self {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        for (_, node) in graph.nodes() {
            let cluster = cluster_for_node(&node.node_id, node.concept);
            let semantic_kind = semantic_kind_for_node(node.concept);
            let shape_hint = shape_hint_for_node(node.concept);
            nodes.push(VisualizationNode {
                node_id: node.node_id.clone(),
                label: node.summary_vi.clone(),
                cluster,
                semantic_kind,
                severity: node.severity.clone(),
                shape_hint,
            });
        }

        for (_, edge) in graph.edges() {
            let semantic_kind = semantic_kind_for_edge(edge.label.concept);
            edges.push(VisualizationEdge {
                edge_id: edge.edge_id.clone(),
                from_id: edge.from_node_id.clone(),
                to_id: edge.to_node_id.clone(),
                label: edge.label.concept.label().as_str().to_string(),
                semantic_kind,
                weight: edge.label.weight,
            });
        }

        Self { nodes, edges }
    }
}

fn cluster_for_node(node_id: &str, concept: NodeConcept) -> String {
    match concept {
        NodeConcept::DayCanchi
        | NodeConcept::MonthCanchi
        | NodeConcept::YearCanchi
        | NodeConcept::SolarTerm
        | NodeConcept::HourCanchi
        | NodeConcept::Truc
        | NodeConcept::DayDeity
        | NodeConcept::NaAm
        | NodeConcept::Star
        | NodeConcept::Taboo
        | NodeConcept::XungHop
        | NodeConcept::HoangDaoHour
        | NodeConcept::Direction
        | NodeConcept::Element
        | NodeConcept::PersonalAlignment => "day-core".to_string(),

        NodeConcept::ChartPillar
        | NodeConcept::AxisSignal
        | NodeConcept::DayPersonMatrix
        | NodeConcept::PersonalHourMatrix
        | NodeConcept::ElementResonanceMatrix
        | NodeConcept::DirectionMergeMatrix
        | NodeConcept::DomainDayBoostMatrix
        | NodeConcept::InteractionRow
        | NodeConcept::TenGodRelation
        | NodeConcept::BranchRelationNode
        | NodeConcept::ElementRelationNode
        | NodeConcept::DirectionSignalNode
        | NodeConcept::HourSlot
        | NodeConcept::InteractionSignal => "interaction-core".to_string(),

        NodeConcept::Activity
        | NodeConcept::RecommendationHit
        | NodeConcept::RecommendationLayer
        | NodeConcept::RecommendationSummary => "recommendation-evidence".to_string(),

        NodeConcept::Recommendation => "recommendation-summary".to_string(),

        _ => {
            if node_id.starts_with("bazi_profile:")
                || node_id.starts_with("pillar:")
                || node_id.starts_with("element_distribution:")
            {
                "bazi-core".to_string()
            } else if node_id.starts_with("day:")
                || node_id.starts_with("solar_term:")
                || node_id.starts_with("truc:")
                || node_id.contains(":day:")
            {
                "day-core".to_string()
            } else {
                "misc".to_string()
            }
        }
    }
}

fn semantic_kind_for_node(concept: NodeConcept) -> String {
    concept.label().as_str().to_string()
}

fn semantic_kind_for_edge(concept: EdgeConcept) -> String {
    concept.label().as_str().to_string()
}

fn shape_hint_for_node(concept: NodeConcept) -> Option<String> {
    match concept {
        NodeConcept::DayCanchi
        | NodeConcept::MonthCanchi
        | NodeConcept::YearCanchi
        | NodeConcept::HourCanchi
        | NodeConcept::SolarTerm => Some("diamond".to_string()),

        NodeConcept::Truc
        | NodeConcept::DayDeity
        | NodeConcept::Taboo
        | NodeConcept::XungHop
        | NodeConcept::HoangDaoHour
        | NodeConcept::Star
        | NodeConcept::Direction
        | NodeConcept::RecommendationLayer => Some("hexagon".to_string()),

        NodeConcept::NaAm
        | NodeConcept::Element
        | NodeConcept::PersonalAlignment
        | NodeConcept::ChartPillar
        | NodeConcept::AxisSignal
        | NodeConcept::Activity
        | NodeConcept::RecommendationSummary
        | NodeConcept::Recommendation => Some("box".to_string()),

        NodeConcept::DayPersonMatrix
        | NodeConcept::PersonalHourMatrix
        | NodeConcept::ElementResonanceMatrix
        | NodeConcept::DirectionMergeMatrix
        | NodeConcept::DomainDayBoostMatrix
        | NodeConcept::InteractionRow
        | NodeConcept::TenGodRelation
        | NodeConcept::BranchRelationNode
        | NodeConcept::ElementRelationNode
        | NodeConcept::DirectionSignalNode
        | NodeConcept::HourSlot
        | NodeConcept::InteractionSignal => Some("ellipse".to_string()),

        NodeConcept::RecommendationHit => Some("diamond".to_string()),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_graph::builders::{
        build_day_snapshot_graph, build_recommendation_evidence_graph_connected,
    };
    use crate::almanac::recommendation::{
        collect_recommendation_hits, synthesize_daily_recommendations, RecommendationSynthesisContext,
    };
    use crate::calculate_day_snapshot;

    #[test]
    fn visualization_node_has_semantic_kind() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let viz = VisualizationGraph::from_semantic_graph(&graph);

        for node in &viz.nodes {
            assert!(
                !node.semantic_kind.is_empty(),
                "node {} should have semantic_kind",
                node.node_id
            );
        }
    }

    #[test]
    fn visualization_node_has_cluster() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let viz = VisualizationGraph::from_semantic_graph(&graph);

        for node in &viz.nodes {
            assert!(
                !node.cluster.is_empty(),
                "node {} should have cluster",
                node.node_id
            );
        }
    }

    #[test]
    fn day_fact_nodes_have_day_core_cluster() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let viz = VisualizationGraph::from_semantic_graph(&graph);

        let day_nodes: Vec<_> = viz
            .nodes
            .iter()
            .filter(|n| n.node_id.contains("truc") || n.node_id.contains("day_deity"))
            .collect();

        for node in day_nodes {
            assert_eq!(
                node.cluster, "day-core",
                "day fact nodes should have day-core cluster"
            );
        }
    }

    #[test]
    fn recommendation_nodes_have_correct_cluster() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
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
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");
        let day_graph = build_day_snapshot_graph(&snapshot);

        let connected_graph = build_recommendation_evidence_graph_connected(
            2024, 2, 10, "default",
            &recommendations.activities,
            &hits,
            &day_graph,
        );

        let viz = VisualizationGraph::from_semantic_graph(&connected_graph);

        let activity_nodes: Vec<_> = viz
            .nodes
            .iter()
            .filter(|n| n.node_id.starts_with("activity:"))
            .collect();

        for node in activity_nodes {
            assert_eq!(
                node.cluster, "recommendation-evidence",
                "activity nodes should have recommendation-evidence cluster"
            );
        }
    }

    #[test]
    fn visualization_edge_has_semantic_kind() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let viz = VisualizationGraph::from_semantic_graph(&graph);

        for edge in &viz.edges {
            assert!(
                !edge.semantic_kind.is_empty(),
                "edge {} should have semantic_kind",
                edge.edge_id
            );
        }
    }

    #[test]
    fn shape_hints_are_set_for_known_concepts() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let viz = VisualizationGraph::from_semantic_graph(&graph);

        let has_shape_hint = viz.nodes.iter().any(|n| n.shape_hint.is_some());
        assert!(
            has_shape_hint,
            "some nodes should have shape hints"
        );
    }

    #[test]
    fn connected_graph_covers_all_clusters() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
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
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");
        let day_graph = build_day_snapshot_graph(&snapshot);

        let connected_graph = build_recommendation_evidence_graph_connected(
            2024, 2, 10, "default",
            &recommendations.activities,
            &hits,
            &day_graph,
        );

        let viz = VisualizationGraph::from_semantic_graph(&connected_graph);

        let clusters: std::collections::HashSet<_> = viz.nodes.iter().map(|n| n.cluster.clone()).collect();

        assert!(
            clusters.contains(&"day-core".to_string()),
            "connected graph should have day-core cluster"
        );
        assert!(
            clusters.contains(&"recommendation-evidence".to_string()),
            "connected graph should have recommendation-evidence cluster"
        );
    }
}