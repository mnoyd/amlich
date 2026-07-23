use crate::reasoning::ReasoningEvidenceEnvelope;
use crate::semantic_graph::selectors::SourceFamilyCounts;
use crate::semantic_graph::{EdgeConcept, NodeConcept, SemanticGraph};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendationEvidenceView {
    pub activity_id: String,
    pub label_vi: String,
    pub label_en: String,
    pub favor_hits: Vec<HitView>,
    pub avoid_hits: Vec<HitView>,
    pub has_hard_stop: bool,
    pub source_breakdown: SourceBreakdown,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HitView {
    pub hit_id: String,
    pub summary_vi: String,
    pub summary_en: String,
    pub source: String,
    pub severity: String,
    pub hard_stop: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance: Vec<ReasoningEvidenceEnvelope>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceBreakdown {
    pub truc_count: usize,
    pub taboo_count: usize,
    pub day_deity_count: usize,
    pub xung_hop_count: usize,
    pub stars_count: usize,
    pub gio_hoang_dao_count: usize,
    pub travel_count: usize,
    pub tiet_khi_count: usize,
    pub other_count: usize,
}

impl SourceBreakdown {
    fn from_counts(counts: &SourceFamilyCounts) -> Self {
        Self {
            truc_count: counts.truc,
            taboo_count: counts.taboo,
            day_deity_count: counts.day_deity,
            xung_hop_count: counts.xung_hop,
            stars_count: counts.stars,
            gio_hoang_dao_count: counts.gio_hoang_dao,
            travel_count: counts.travel,
            tiet_khi_count: counts.tiet_khi,
            other_count: counts.other,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendationEvidenceGraphView {
    pub activities: Vec<RecommendationEvidenceView>,
    pub total_hits: usize,
    pub total_favor_hits: usize,
    pub total_avoid_hits: usize,
    pub clusters: Vec<String>,
}

impl RecommendationEvidenceGraphView {
    pub fn from_graph(graph: &SemanticGraph) -> Self {
        let mut activity_views: Vec<RecommendationEvidenceView> = Vec::new();
        let mut total_hits = 0;
        let mut total_favor_hits = 0;
        let mut total_avoid_hits = 0;
        let mut clusters = std::collections::HashSet::new();

        for node in graph.nodes().values() {
            match node.concept {
                NodeConcept::Activity => {
                    let label_vi = node
                        .summary_vi
                        .split(" / ")
                        .next()
                        .unwrap_or("")
                        .to_string();
                    let label_en = node
                        .summary_vi
                        .split(" / ")
                        .nth(1)
                        .unwrap_or("")
                        .to_string();

                    let activity_id = node
                        .node_id
                        .strip_prefix("activity:")
                        .unwrap_or(&node.node_id)
                        .to_string();

                    let (favor_hits, avoid_hits) =
                        Self::collect_hits_for_activity(graph, &node.node_id);
                    let has_hard_stop = favor_hits
                        .iter()
                        .chain(avoid_hits.iter())
                        .any(|h| h.hard_stop);

                    let source_breakdown = Self::compute_source_breakdown(&favor_hits, &avoid_hits);

                    total_hits += favor_hits.len() + avoid_hits.len();
                    total_favor_hits += favor_hits.len();
                    total_avoid_hits += avoid_hits.len();

                    clusters.insert("recommendation-activities".to_string());

                    activity_views.push(RecommendationEvidenceView {
                        activity_id,
                        label_vi,
                        label_en,
                        favor_hits,
                        avoid_hits,
                        has_hard_stop,
                        source_breakdown,
                    });
                }
                NodeConcept::RecommendationHit => {
                    clusters.insert("recommendation-evidence".to_string());
                }
                NodeConcept::RecommendationSummary => {
                    clusters.insert("recommendation-summary".to_string());
                }
                _ => {}
            }
        }

        activity_views.sort_by(|a, b| a.activity_id.cmp(&b.activity_id));

        RecommendationEvidenceGraphView {
            activities: activity_views,
            total_hits,
            total_favor_hits,
            total_avoid_hits,
            clusters: clusters.into_iter().collect(),
        }
    }

    fn collect_hits_for_activity(
        graph: &SemanticGraph,
        activity_node_id: &str,
    ) -> (Vec<HitView>, Vec<HitView>) {
        let mut favor_hits = Vec::new();
        let mut avoid_hits = Vec::new();

        for edge in graph.edges().values() {
            if edge.to_node_id == activity_node_id
                && matches!(edge.label.concept, EdgeConcept::TargetsActivity)
            {
                if let Some(hit_node) = graph.get_node(&edge.from_node_id) {
                    if matches!(hit_node.concept, NodeConcept::RecommendationHit) {
                        let tags = &hit_node.tags;
                        let is_favor = tags.iter().any(|t| t == "favor");
                        let is_hard_stop = tags.iter().any(|t| t == "hard_stop=true");

                        let source = tags
                            .iter()
                            .find(|t| t.starts_with("source="))
                            .map(|t| t.replace("source=", ""))
                            .unwrap_or_default();

                        let provenance = hit_node
                            .provenance
                            .iter()
                            .map(|p| p.to_reasoning_evidence())
                            .collect();

                        let hit_view = HitView {
                            hit_id: hit_node.node_id.clone(),
                            summary_vi: hit_node.summary_vi.clone(),
                            summary_en: String::new(),
                            source,
                            severity: hit_node.severity.clone().unwrap_or_default(),
                            hard_stop: is_hard_stop,
                            provenance,
                        };

                        if is_favor {
                            favor_hits.push(hit_view);
                        } else {
                            avoid_hits.push(hit_view);
                        }
                    }
                }
            }
        }

        (favor_hits, avoid_hits)
    }

    fn compute_source_breakdown(favor_hits: &[HitView], avoid_hits: &[HitView]) -> SourceBreakdown {
        let mut counts = SourceFamilyCounts::default();
        for hit in favor_hits.iter().chain(avoid_hits.iter()) {
            counts.tally_source(&hit.source);
        }
        SourceBreakdown::from_counts(&counts)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LlmRecommendationSlice {
    pub date: String,
    pub profile: String,
    pub activity_summaries: Vec<LlmActivitySummary>,
    pub top_favor_hits: Vec<String>,
    pub top_avoid_hits: Vec<String>,
    pub hard_stop_activities: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LlmActivitySummary {
    pub activity_id: String,
    pub label_vi: String,
    pub verdict: String,
    pub supporting_sources: Vec<String>,
    pub resisting_sources: Vec<String>,
    pub hard_stop: bool,
}

impl LlmRecommendationSlice {
    pub fn from_view(view: &RecommendationEvidenceGraphView, date: &str, profile: &str) -> Self {
        let mut top_favor_hits = Vec::new();
        let mut top_avoid_hits = Vec::new();
        let mut hard_stop_activities = Vec::new();
        let mut activity_summaries = Vec::new();

        for activity in &view.activities {
            let mut supporting = Vec::new();
            let mut resisting = Vec::new();

            for hit in &activity.favor_hits {
                supporting.push(hit.summary_vi.clone());
            }

            for hit in &activity.avoid_hits {
                resisting.push(hit.summary_vi.clone());
            }

            if activity.has_hard_stop {
                hard_stop_activities.push(activity.activity_id.clone());
            }

            let verdict = if activity.has_hard_stop {
                "Kỵ mạnh".to_string()
            } else if !activity.favor_hits.is_empty() && activity.avoid_hits.is_empty() {
                "Nên".to_string()
            } else if activity.avoid_hits.len() > activity.favor_hits.len() {
                "Tránh".to_string()
            } else {
                "Có thể".to_string()
            };

            activity_summaries.push(LlmActivitySummary {
                activity_id: activity.activity_id.clone(),
                label_vi: activity.label_vi.clone(),
                verdict,
                supporting_sources: supporting,
                resisting_sources: resisting,
                hard_stop: activity.has_hard_stop,
            });

            for hit in &activity.favor_hits {
                if top_favor_hits.len() < 5 {
                    top_favor_hits.push(format!("{}: {}", activity.activity_id, hit.summary_vi));
                }
            }

            for hit in &activity.avoid_hits {
                if top_avoid_hits.len() < 5 {
                    top_avoid_hits.push(format!("{}: {}", activity.activity_id, hit.summary_vi));
                }
            }
        }

        LlmRecommendationSlice {
            date: date.to_string(),
            profile: profile.to_string(),
            activity_summaries,
            top_favor_hits,
            top_avoid_hits,
            hard_stop_activities,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::recommendation::collect_recommendation_hits;
    use crate::almanac::recommendation::RecommendationSynthesisContext;
    use crate::calculate_day_snapshot;
    use crate::semantic_graph::builders::{
        build_day_snapshot_graph, build_recommendation_evidence_graph,
        build_recommendation_evidence_graph_connected,
    };

    #[test]
    fn recommendation_evidence_view_extracts_activities() {
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

        let recommendations =
            crate::almanac::recommendation::synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");

        let graph = build_recommendation_evidence_graph(
            2024,
            2,
            10,
            "default",
            &recommendations.activities,
            &hits,
        );

        let view = RecommendationEvidenceGraphView::from_graph(&graph);

        assert!(!view.activities.is_empty(), "view should have activities");
        assert_eq!(view.total_hits, hits.len(), "total hits should match");
    }

    #[test]
    fn llm_recommendation_slice_generates_summaries() {
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

        let recommendations =
            crate::almanac::recommendation::synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");

        let graph = build_recommendation_evidence_graph(
            2024,
            2,
            10,
            "default",
            &recommendations.activities,
            &hits,
        );

        let view = RecommendationEvidenceGraphView::from_graph(&graph);
        let slice = LlmRecommendationSlice::from_view(&view, "2024-02-10", "default");

        assert!(
            !slice.activity_summaries.is_empty(),
            "slice should have summaries"
        );
    }

    #[test]
    fn graph_activity_count_matches_daily_recommendations() {
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

        let recommendations =
            crate::almanac::recommendation::synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");

        let graph = build_recommendation_evidence_graph(
            2024,
            2,
            10,
            "default",
            &recommendations.activities,
            &hits,
        );

        let activity_count = graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, NodeConcept::Activity))
            .count();

        assert_eq!(
            activity_count,
            recommendations.activities.len(),
            "graph activity count should match daily recommendations activity count"
        );
    }

    #[test]
    fn graph_hit_count_matches_collected_hits() {
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

        let recommendations =
            crate::almanac::recommendation::synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");

        let graph = build_recommendation_evidence_graph(
            2024,
            2,
            10,
            "default",
            &recommendations.activities,
            &hits,
        );

        let hit_count = graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, NodeConcept::RecommendationHit))
            .count();

        assert_eq!(
            hit_count,
            hits.len(),
            "graph hit count should match collected hits count"
        );
    }

    #[test]
    fn graph_hard_stop_activities_are_flagged() {
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

        let recommendations =
            crate::almanac::recommendation::synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");

        let graph = build_recommendation_evidence_graph(
            2024,
            2,
            14,
            "default",
            &recommendations.activities,
            &hits,
        );

        let view = RecommendationEvidenceGraphView::from_graph(&graph);

        let has_hard_stop_in_view = view.activities.iter().any(|a| a.has_hard_stop);
        let has_hard_stop_in_recommendations = recommendations
            .activities
            .iter()
            .any(|a| a.bucket == crate::almanac::recommendation::RecommendationBucket::KyManh);

        assert_eq!(
            has_hard_stop_in_view, has_hard_stop_in_recommendations,
            "hard stop detection in graph view should match daily recommendations bucket"
        );
    }

    #[test]
    fn graph_source_breakdown_covers_all_hit_sources() {
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

        let recommendations =
            crate::almanac::recommendation::synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");

        let graph = build_recommendation_evidence_graph(
            2024,
            2,
            10,
            "default",
            &recommendations.activities,
            &hits,
        );

        let view = RecommendationEvidenceGraphView::from_graph(&graph);

        let total_breakdown: usize = view
            .activities
            .iter()
            .map(|a| {
                let b = &a.source_breakdown;
                b.truc_count
                    + b.taboo_count
                    + b.day_deity_count
                    + b.xung_hop_count
                    + b.stars_count
                    + b.gio_hoang_dao_count
                    + b.travel_count
                    + b.tiet_khi_count
                    + b.other_count
            })
            .sum();

        assert_eq!(
            total_breakdown, view.total_hits,
            "source breakdown total should equal total hits"
        );
    }

    #[test]
    fn connected_graph_merges_day_fact_nodes() {
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

        let recommendations =
            crate::almanac::recommendation::synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");
        let day_graph = build_day_snapshot_graph(&snapshot);

        let connected_graph = build_recommendation_evidence_graph_connected(
            2024,
            2,
            10,
            "default",
            &recommendations.activities,
            &hits,
            &day_graph,
        );

        let day_chi_nodes: Vec<_> = connected_graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, crate::semantic_graph::NodeConcept::Truc))
            .collect();

        assert!(
            !day_chi_nodes.is_empty(),
            "connected graph should contain day fact nodes (truc)"
        );
    }

    #[test]
    fn parity_hits_per_activity_matches_reasons() {
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

        let recommendations =
            crate::almanac::recommendation::synthesize_daily_recommendations(&context);
        let hits = collect_recommendation_hits(&context, &[]).expect("hits should collect");

        let graph = build_recommendation_evidence_graph(
            2024,
            2,
            10,
            "default",
            &recommendations.activities,
            &hits,
        );

        let view = RecommendationEvidenceGraphView::from_graph(&graph);

        for activity in &recommendations.activities {
            let activity_id_str = activity.activity_id.as_str();

            let view_activity = view
                .activities
                .iter()
                .find(|a| a.activity_id == activity_id_str);

            if let Some(view_act) = view_activity {
                let graph_hit_count = view_act.favor_hits.len() + view_act.avoid_hits.len();
                let reason_count = activity.reasons.len();

                assert_eq!(
                    graph_hit_count, reason_count,
                    "hit count for {} should match reason count",
                    activity_id_str
                );
            }
        }
    }
}
