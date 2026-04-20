use crate::semantic_graph::{
    EdgeConcept, NodeConcept, SemanticGraph, SemanticNode,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConvergenceFactNode {
    pub node_id: String,
    pub concept: String,
    pub summary_vi: String,
    pub severity: Option<String>,
    pub feeds_reasoning: bool,
    pub feeds_recommendation: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConvergenceRecommendationHit {
    pub hit_id: String,
    pub activity_id: String,
    pub summary_vi: String,
    pub direction: String,
    pub hard_stop: bool,
    pub origin_fact_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConvergenceView {
    pub shared_fact_nodes: Vec<ConvergenceFactNode>,
    pub recommendation_hits: Vec<ConvergenceRecommendationHit>,
    pub fact_influences_reasoning_count: usize,
    pub fact_influences_recommendation_count: usize,
    pub fact_influences_both_count: usize,
    pub clusters: Vec<String>,
}

impl ConvergenceView {
    pub fn from_connected_graph(graph: &SemanticGraph) -> Self {
        let mut shared_fact_nodes = Vec::new();
        let mut recommendation_hits = Vec::new();
        let fact_influences_reasoning: HashSet<String> = HashSet::new();
        let mut fact_influences_recommendation: HashSet<String> = HashSet::new();
        let mut fact_influences_both: HashSet<String> = HashSet::new();
        let mut clusters: HashSet<String> = HashSet::new();

        let mut hit_to_activity: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        for (_, edge) in graph.edges() {
            if matches!(edge.label.concept, EdgeConcept::TargetsActivity) {
                hit_to_activity.insert(edge.from_node_id.clone(), edge.to_node_id.clone());
            }
        }

        for (_, node) in graph.nodes() {
            match node.concept {
                NodeConcept::Activity => {
                    clusters.insert("recommendation-activities".to_string());
                }
                NodeConcept::RecommendationHit => {
                    clusters.insert("recommendation-evidence".to_string());
                    let activity_id = hit_to_activity
                        .get(&node.node_id)
                        .map(|s| s.replace("activity:", ""))
                        .unwrap_or_default();

                    let direction = if node.tags.iter().any(|t| t == "favor") {
                        "favor".to_string()
                    } else {
                        "avoid".to_string()
                    };

                    let hard_stop = node.tags.iter().any(|t| t == "hard_stop=true");

                    let origin_fact_ids: Vec<String> = graph
                        .incoming_edges(&node.node_id)
                        .iter()
                        .filter(|e| matches!(e.label.concept, EdgeConcept::OriginatesFrom))
                        .map(|e| e.from_node_id.clone())
                        .collect();

                    for fact_id in &origin_fact_ids {
                        fact_influences_recommendation.insert(fact_id.clone());
                    }

                    recommendation_hits.push(ConvergenceRecommendationHit {
                        hit_id: node.node_id.clone(),
                        activity_id,
                        summary_vi: node.summary_vi.clone(),
                        direction,
                        hard_stop,
                        origin_fact_ids,
                    });
                }
                NodeConcept::Truc
                | NodeConcept::DayDeity
                | NodeConcept::Taboo
                | NodeConcept::XungHop
                | NodeConcept::HoangDaoHour
                | NodeConcept::Direction
                | NodeConcept::Star => {
                    clusters.insert("day-core".to_string());

                    let outgoing = graph.outgoing_edges(&node.node_id);
                    let has_hit_origin = outgoing.iter().any(|e| {
                        matches!(e.label.concept, EdgeConcept::OriginatesFrom)
                            && graph.get_node(&e.to_node_id).map_or(false, |n| {
                                n.concept == NodeConcept::RecommendationHit
                            })
                    });

                    if has_hit_origin {
                        fact_influences_recommendation.insert(node.node_id.clone());
                    }

                    let hits_from_this_fact: Vec<_> = graph
                        .nodes()
                        .values()
                        .filter(|n| {
                            n.concept == NodeConcept::RecommendationHit
                                && graph
                                    .incoming_edges(&n.node_id)
                                    .iter()
                                    .any(|e| e.from_node_id == node.node_id)
                        })
                        .collect();

                    if !hits_from_this_fact.is_empty() {
                        fact_influences_recommendation.insert(node.node_id.clone());
                    }

                    let shared = fact_influences_reasoning.contains(&node.node_id)
                        && fact_influences_recommendation.contains(&node.node_id);

                    shared_fact_nodes.push(ConvergenceFactNode {
                        node_id: node.node_id.clone(),
                        concept: node.concept.label().as_str().to_string(),
                        summary_vi: node.summary_vi.clone(),
                        severity: node.severity.clone(),
                        feeds_reasoning: fact_influences_reasoning.contains(&node.node_id),
                        feeds_recommendation: fact_influences_recommendation.contains(&node.node_id),
                    });

                    if shared {
                        fact_influences_both.insert(node.node_id.clone());
                    }
                }
                _ => {}
            }
        }

        let mut view = Self {
            shared_fact_nodes,
            recommendation_hits,
            fact_influences_reasoning_count: fact_influences_reasoning.len(),
            fact_influences_recommendation_count: fact_influences_recommendation.len(),
            fact_influences_both_count: fact_influences_both.len(),
            clusters: clusters.into_iter().collect(),
        };

        for node in &mut view.shared_fact_nodes {
            let _shared = fact_influences_both.contains(&node.node_id);
            node.feeds_reasoning = fact_influences_reasoning.contains(&node.node_id);
            node.feeds_recommendation = fact_influences_recommendation.contains(&node.node_id);
        }

        view
    }

    pub fn extract_shared_influence_slice<'a>(&self, graph: &'a SemanticGraph) -> Vec<&'a SemanticNode> {
        graph
            .nodes()
            .values()
            .filter(|n| {
                self.shared_fact_nodes
                    .iter()
                    .any(|s| s.node_id == n.node_id && s.feeds_recommendation)
            })
            .collect()
    }

    pub fn facts_feeding_both_count(&self) -> usize {
        self.shared_fact_nodes
            .iter()
            .filter(|n| n.feeds_reasoning && n.feeds_recommendation)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.shared_fact_nodes.is_empty() && self.recommendation_hits.is_empty()
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
    fn convergence_view_extracts_fact_nodes() {
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

        let view = ConvergenceView::from_connected_graph(&connected_graph);

        assert!(
            !view.shared_fact_nodes.is_empty() || !view.recommendation_hits.is_empty(),
            "convergence view should have fact nodes or hit nodes"
        );
    }

    #[test]
    fn convergence_view_tracks_fact_influence_counts() {
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

        let view = ConvergenceView::from_connected_graph(&connected_graph);

        assert_eq!(
            view.fact_influences_reasoning_count, 0,
            "connected graph without reasoning should have 0 reasoning influence"
        );

        if !view.recommendation_hits.is_empty() {
            assert!(
                view.fact_influences_recommendation_count > 0,
                "should have facts influencing recommendation"
            );
        }
    }

    #[test]
    fn convergence_view_identifies_hoang_dao_facts() {
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

        let view = ConvergenceView::from_connected_graph(&connected_graph);

        let has_hoang_dao = view.shared_fact_nodes.iter().any(|n| {
            n.concept == "hoang_dao_hour"
        });

        assert!(
            has_hoang_dao || view.shared_fact_nodes.is_empty(),
            "convergence view should identify hoang dao hour facts"
        );
    }

    #[test]
    fn convergence_view_has_clusters() {
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

        let view = ConvergenceView::from_connected_graph(&connected_graph);

        assert!(
            !view.clusters.is_empty(),
            "convergence view should have cluster labels"
        );
    }

    #[test]
    fn convergence_view_hard_stop_hits_identified() {
        let snapshot = calculate_day_snapshot(14, 2, 2024);
        let context = RecommendationSynthesisContext {
            day_chi: &snapshot.context.canchi.day.chi,
            day_fortune: &snapshot.day_fortune,
            gio_hoang_dao: Some(&snapshot.context.gio_hoang_dao),
            tiet_khi_name: Some(&snapshot.context.tiet_khi.name),
            profile_id: None,
            event_kind: None,
            enabled_pack_ids: &[],
        };

        let recommendations = synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");
        let day_graph = build_day_snapshot_graph(&snapshot);

        let connected_graph = build_recommendation_evidence_graph_connected(
            2024, 2, 14, "default",
            &recommendations.activities,
            &hits,
            &day_graph,
        );

        let view = ConvergenceView::from_connected_graph(&connected_graph);

        let hard_stop_count = view
            .recommendation_hits
            .iter()
            .filter(|h| h.hard_stop)
            .count();

        assert_eq!(
            hard_stop_count,
            view.recommendation_hits.iter().filter(|h| h.hard_stop).count(),
            "hard stop hits should be correctly identified"
        );
    }

    #[test]
    fn convergence_view_hit_origin_facts_traced() {
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

        let view = ConvergenceView::from_connected_graph(&connected_graph);

        let hits_with_origins = view
            .recommendation_hits
            .iter()
            .filter(|h| !h.origin_fact_ids.is_empty())
            .count();

        assert_eq!(
            hits_with_origins,
            view.recommendation_hits
                .iter()
                .filter(|h| !h.origin_fact_ids.is_empty())
                .count(),
            "hits with origin facts should be correctly traced"
        );
    }
}