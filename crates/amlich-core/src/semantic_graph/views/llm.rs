use std::collections::HashMap;

use crate::semantic_graph::{NodeConcept, SemanticGraph, SemanticNode};
use serde::{Deserialize, Serialize};

use super::subgraph::SubgraphView;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LlmGraphSlice {
    pub root_ids: Vec<String>,
    pub node_refs: Vec<String>,
    pub edge_refs: Vec<String>,
    pub summary_points: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_summary: Option<ClusterSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_kind_counts: Option<HashMap<String, usize>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity_counts: Option<HashMap<String, usize>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClusterSummary {
    pub clusters: HashMap<String, usize>,
    pub total_nodes: usize,
    pub total_edges: usize,
}

impl LlmGraphSlice {
    pub fn from_subgraph(graph: &SemanticGraph, view: &SubgraphView) -> Self {
        let node_refs = view.node_ids.clone();
        let edge_refs = view.edge_ids.clone();

        let mut summary_points = Vec::new();
        let mut cluster_counts: HashMap<String, usize> = HashMap::new();
        let mut semantic_kind_counts: HashMap<String, usize> = HashMap::new();
        let mut severity_counts: HashMap<String, usize> = HashMap::new();

        for node_id in &view.node_ids {
            if let Some(node) = graph.get_node(node_id) {
                let cluster = cluster_label_for_node(node);
                *cluster_counts.entry(cluster).or_insert(0) += 1;

                let kind_label = node.concept.label().as_str().to_string();
                *semantic_kind_counts.entry(kind_label).or_insert(0) += 1;

                if let Some(ref sev) = node.severity {
                    *severity_counts.entry(sev.clone()).or_insert(0) += 1;
                }

                let mut parts = vec![format!(
                    "[{}] {}",
                    node.concept.label().as_str(),
                    node.summary_vi
                )];
                if let Some(sev) = &node.severity {
                    parts.push(format!("severity={}", sev));
                }
                if !node.provenance.is_empty() {
                    let sources: Vec<_> = node
                        .provenance
                        .iter()
                        .map(|p| format!("{:?}", p.source))
                        .collect();
                    parts.push(format!("sources={}", sources.join(",")));
                }
                summary_points.push(parts.join(" | "));
            }
        }

        let cluster_summary = if cluster_counts.is_empty() {
            None
        } else {
            Some(ClusterSummary {
                total_nodes: node_refs.len(),
                total_edges: edge_refs.len(),
                clusters: cluster_counts,
            })
        };

        let semantic_kind_counts = if semantic_kind_counts.is_empty() {
            None
        } else {
            Some(semantic_kind_counts)
        };

        let severity_counts = if severity_counts.is_empty() {
            None
        } else {
            Some(severity_counts)
        };

        Self {
            root_ids: view.root_ids.clone(),
            node_refs,
            edge_refs,
            summary_points,
            cluster_summary,
            semantic_kind_counts,
            severity_counts,
        }
    }

    pub fn from_graph(graph: &SemanticGraph) -> Self {
        let all_node_ids: Vec<String> = graph.nodes().keys().cloned().collect();
        let all_edge_ids: Vec<String> = graph.edges().keys().cloned().collect();

        let mut summary_points = Vec::new();
        let mut cluster_counts: HashMap<String, usize> = HashMap::new();
        let mut semantic_kind_counts: HashMap<String, usize> = HashMap::new();
        let mut severity_counts: HashMap<String, usize> = HashMap::new();

        for (_node_id, node) in graph.nodes() {
            let cluster = cluster_label_for_node(node);
            *cluster_counts.entry(cluster).or_insert(0) += 1;

            let kind_label = node.concept.label().as_str().to_string();
            *semantic_kind_counts.entry(kind_label).or_insert(0) += 1;

            if let Some(ref sev) = node.severity {
                *severity_counts.entry(sev.clone()).or_insert(0) += 1;
            }

            let mut parts = vec![format!(
                "[{}] {}",
                node.concept.label().as_str(),
                node.summary_vi
            )];
            if let Some(sev) = &node.severity {
                parts.push(format!("severity={}", sev));
            }
            if !node.provenance.is_empty() {
                let sources: Vec<_> = node
                    .provenance
                    .iter()
                    .map(|p| format!("{:?}", p.source))
                    .collect();
                parts.push(format!("sources={}", sources.join(",")));
            }
            summary_points.push(parts.join(" | "));
        }

        let cluster_summary = if cluster_counts.is_empty() {
            None
        } else {
            Some(ClusterSummary {
                total_nodes: all_node_ids.len(),
                total_edges: all_edge_ids.len(),
                clusters: cluster_counts,
            })
        };

        let semantic_kind_counts = if semantic_kind_counts.is_empty() {
            None
        } else {
            Some(semantic_kind_counts)
        };

        let severity_counts = if severity_counts.is_empty() {
            None
        } else {
            Some(severity_counts)
        };

        Self {
            root_ids: vec![],
            node_refs: all_node_ids,
            edge_refs: all_edge_ids,
            summary_points,
            cluster_summary,
            semantic_kind_counts,
            severity_counts,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.node_refs.is_empty()
    }

    pub fn node_count(&self) -> usize {
        self.node_refs.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_refs.len()
    }

    pub fn top_severity_nodes(&self, graph: &SemanticGraph, limit: usize) -> Vec<String> {
        let mut nodes_with_severity: Vec<(String, &str)> = self
            .node_refs
            .iter()
            .filter_map(|id| {
                graph.get_node(id).and_then(|n| {
                    n.severity
                        .as_ref()
                        .map(|sev| (id.clone(), sev.as_str()))
                })
            })
            .collect();

        nodes_with_severity.sort_by(|a, b| b.1.cmp(a.1));
        nodes_with_severity
            .into_iter()
            .take(limit)
            .map(|(id, _)| id)
            .collect()
    }
}

fn cluster_label_for_node(node: &SemanticNode) -> String {
    match node.concept {
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
            if node.node_id.starts_with("bazi_profile:")
                || node.node_id.starts_with("pillar:")
                || node.node_id.starts_with("element_distribution:")
            {
                "bazi-core".to_string()
            } else if node.node_id.starts_with("day:")
                || node.node_id.starts_with("solar_term:")
                || node.node_id.starts_with("truc:")
                || node.node_id.contains(":day:")
            {
                "day-core".to_string()
            } else {
                "misc".to_string()
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LlmConvergenceSlice {
    pub root_ids: Vec<String>,
    pub shared_fact_refs: Vec<ConvergenceFactRef>,
    pub recommendation_hit_refs: Vec<ConvergenceHitRef>,
    pub fact_influences_reasoning_count: usize,
    pub fact_influences_recommendation_count: usize,
    pub fact_influences_both_count: usize,
    pub cluster_counts: HashMap<String, usize>,
    pub top_shared_facts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceFactRef {
    pub node_id: String,
    pub concept: String,
    pub summary_vi: String,
    pub severity: Option<String>,
    pub feeds_reasoning: bool,
    pub feeds_recommendation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceHitRef {
    pub hit_id: String,
    pub activity_id: String,
    pub summary_vi: String,
    pub direction: String,
    pub hard_stop: bool,
    pub origin_fact_ids: Vec<String>,
}

impl LlmConvergenceSlice {
    pub fn from_graph(graph: &SemanticGraph, root_ids: &[&str]) -> Self {
        use crate::semantic_graph::views::ConvergenceView;

        let view = ConvergenceView::from_connected_graph(graph);

        let mut cluster_counts: HashMap<String, usize> = HashMap::new();
        for cluster in &view.clusters {
            *cluster_counts.entry(cluster.clone()).or_insert(0) += 1;
        }

        let shared_fact_refs: Vec<ConvergenceFactRef> = view
            .shared_fact_nodes
            .iter()
            .map(|n| ConvergenceFactRef {
                node_id: n.node_id.clone(),
                concept: n.concept.clone(),
                summary_vi: n.summary_vi.clone(),
                severity: n.severity.clone(),
                feeds_reasoning: n.feeds_reasoning,
                feeds_recommendation: n.feeds_recommendation,
            })
            .collect();

        let recommendation_hit_refs: Vec<ConvergenceHitRef> = view
            .recommendation_hits
            .iter()
            .map(|h| ConvergenceHitRef {
                hit_id: h.hit_id.clone(),
                activity_id: h.activity_id.clone(),
                summary_vi: h.summary_vi.clone(),
                direction: h.direction.clone(),
                hard_stop: h.hard_stop,
                origin_fact_ids: h.origin_fact_ids.clone(),
            })
            .collect();

        let top_shared_facts: Vec<String> = view
            .shared_fact_nodes
            .iter()
            .filter(|n| n.feeds_reasoning && n.feeds_recommendation)
            .take(10)
            .map(|n| n.node_id.clone())
            .collect();

        let root_ids_str: Vec<String> = root_ids.iter().map(|s| s.to_string()).collect();

        Self {
            root_ids: root_ids_str,
            shared_fact_refs,
            recommendation_hit_refs,
            fact_influences_reasoning_count: view.fact_influences_reasoning_count,
            fact_influences_recommendation_count: view.fact_influences_recommendation_count,
            fact_influences_both_count: view.fact_influences_both_count,
            cluster_counts,
            top_shared_facts,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.shared_fact_refs.is_empty() && self.recommendation_hit_refs.is_empty()
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
    fn llm_graph_slice_has_semantic_kind_counts() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let slice = LlmGraphSlice::from_graph(&graph);

        assert!(
            slice.semantic_kind_counts.is_some(),
            "slice should have semantic_kind_counts"
        );
        assert!(
            !slice.semantic_kind_counts.as_ref().unwrap().is_empty(),
            "semantic_kind_counts should not be empty"
        );
    }

    #[test]
    fn llm_graph_slice_has_cluster_summary() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let slice = LlmGraphSlice::from_graph(&graph);

        assert!(
            slice.cluster_summary.is_some(),
            "slice should have cluster_summary"
        );
        assert!(
            !slice.cluster_summary.as_ref().unwrap().clusters.is_empty(),
            "cluster_summary should not be empty"
        );
    }

    #[test]
    fn llm_graph_slice_has_summary_points() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let slice = LlmGraphSlice::from_graph(&graph);

        assert!(
            !slice.summary_points.is_empty(),
            "slice should have summary_points"
        );
    }

    #[test]
    fn llm_graph_slice_traces_to_canonical_ids() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let slice = LlmGraphSlice::from_graph(&graph);

        for node_ref in &slice.node_refs {
            assert!(
                graph.get_node(node_ref).is_some(),
                "node_ref {} should exist in graph",
                node_ref
            );
        }
    }

    #[test]
    fn llm_graph_slice_top_severity_nodes() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);
        let slice = LlmGraphSlice::from_graph(&graph);

        let top = slice.top_severity_nodes(&graph, 5);
        assert!(
            top.len() <= 5,
            "top severity nodes should be limited to 5"
        );
    }

    #[test]
    fn llm_convergence_slice_has_fact_refs() {
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

        let slice = LlmConvergenceSlice::from_graph(&connected_graph, &["day:2024-02-10:tz+7"]);

        if !slice.is_empty() {
            assert!(
                !slice.cluster_counts.is_empty(),
                "convergence slice should have cluster counts"
            );
        }
    }

    #[test]
    fn llm_convergence_slice_traces_hit_origins() {
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

        let slice = LlmConvergenceSlice::from_graph(&connected_graph, &["day:2024-02-10:tz+7"]);

        for hit_ref in &slice.recommendation_hit_refs {
            assert!(
                !hit_ref.origin_fact_ids.is_empty() || !slice.shared_fact_refs.is_empty(),
                "hits should have origin facts or shared facts should exist"
            );
        }
    }

    #[test]
    fn connected_llm_slice_covers_multiple_concerns() {
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

        let slice = LlmGraphSlice::from_graph(&connected_graph);

        assert!(
            slice.node_count() > 0,
            "connected graph slice should have nodes"
        );

        if let Some(ref cluster_summary) = slice.cluster_summary {
            assert!(
                cluster_summary.clusters.contains_key("day-core"),
                "should have day-core cluster"
            );
        }
    }
}