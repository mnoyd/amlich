use crate::semantic_graph::{EdgeConcept, NodeConcept, SemanticGraph};

pub struct EvidenceSelectors;

impl EvidenceSelectors {
    pub fn select_hard_taboo_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::Taboo
                    && node.severity.as_deref() == Some("hard")
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_all_taboo_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| node.concept == NodeConcept::Taboo)
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_favorable_truc_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::Truc
                    && node.severity.as_deref() == Some("cat")
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_unfavorable_truc_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::Truc
                    && node.severity.as_deref() == Some("bat")
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_hoang_dao_day_deity_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::DayDeity
                    && node.tags.iter().any(|t| t == "hoang_dao")
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_hac_dao_day_deity_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::DayDeity
                    && node.tags.iter().any(|t| t == "hac_dao")
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_hoang_dao_timing_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::HoangDaoHour
                    && node.severity
                        .as_ref()
                        .and_then(|s| s.parse::<usize>().ok())
                        .is_some_and(|count| count > 0)
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_positive_direction_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::Direction
                    && (node.summary_vi.contains("Tài") || node.summary_vi.contains("Hỷ"))
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_travel_direction_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::Direction && node.node_id.contains("travel")
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_conflict_heavy_xung_hop_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::XungHop
                    && node.tags.iter().any(|t| t.starts_with("luc_xung="))
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_supportive_xung_hop_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::XungHop
                    && node.tags.iter().any(|t| t.starts_with("tam_hop=") || t.starts_with("liu_he="))
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_recommendation_hit_nodes(
        graph: &SemanticGraph,
        direction: Option<SelectHitDirection>,
    ) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::RecommendationHit
                    && match direction {
                        Some(SelectHitDirection::Favor) => {
                            node.tags.iter().any(|t| t == "favor")
                        }
                        Some(SelectHitDirection::Avoid) => {
                            node.tags.iter().any(|t| t == "avoid")
                        }
                        None => true,
                    }
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_hard_stop_recommendation_hits(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| {
                node.concept == NodeConcept::RecommendationHit
                    && node.tags.iter().any(|t| t == "hard_stop=true")
            })
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_activity_nodes(graph: &SemanticGraph) -> Vec<String> {
        graph
            .nodes()
            .values()
            .filter(|node| node.concept == NodeConcept::Activity)
            .map(|node| node.node_id.clone())
            .collect()
    }

    pub fn select_recommendation_hit_origin_facts(
        graph: &SemanticGraph,
        hit_node_id: &str,
    ) -> Vec<String> {
        graph
            .incoming_edges(hit_node_id)
            .iter()
            .filter(|e| matches!(e.label.concept, EdgeConcept::OriginatesFrom))
            .map(|e| e.from_node_id.clone())
            .collect()
    }

    pub fn count_hits_by_source_family(graph: &SemanticGraph) -> SourceFamilyCounts {
        let mut counts = SourceFamilyCounts::default();

        for node in graph.nodes().values() {
            if node.concept != NodeConcept::RecommendationHit {
                continue;
            }

            let source_tag = node
                .tags
                .iter()
                .find(|t| t.starts_with("source="))
                .map(|t| t.replace("source=", ""));

            if let Some(source) = source_tag {
                match source.as_str() {
                    "Truc" => counts.truc += 1,
                    "DayDeity" => counts.day_deity += 1,
                    "Taboo" => counts.taboo += 1,
                    "XungHop" => counts.xung_hop += 1,
                    "Stars" => counts.stars += 1,
                    "GioHoangDao" => counts.gio_hoang_dao += 1,
                    "Travel" => counts.travel += 1,
                    "TietKhi" => counts.tiet_khi += 1,
                    _ => counts.other += 1,
                }
            }
        }

        counts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectHitDirection {
    Favor,
    Avoid,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SourceFamilyCounts {
    pub truc: usize,
    pub day_deity: usize,
    pub taboo: usize,
    pub xung_hop: usize,
    pub stars: usize,
    pub gio_hoang_dao: usize,
    pub travel: usize,
    pub tiet_khi: usize,
    pub other: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_graph::builders::build_day_snapshot_graph;
    use crate::calculate_day_snapshot;

    #[test]
    fn select_hard_taboo_nodes_finds_hard_severity() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let taboos = graph
            .nodes()
            .values()
            .filter(|n| n.concept == NodeConcept::Taboo)
            .collect::<Vec<_>>();

        if !taboos.is_empty() {
            let hard_count = taboos
                .iter()
                .filter(|n| n.severity.as_deref() == Some("hard"))
                .count();

            let selected = EvidenceSelectors::select_hard_taboo_nodes(&graph);

            assert_eq!(
                selected.len(),
                hard_count,
                "hard taboo selector should find all hard taboos"
            );
        }
    }

    #[test]
    fn select_favorable_truc_finds_cat_quality() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let truc_nodes: Vec<_> = graph
            .nodes()
            .values()
            .filter(|n| n.concept == NodeConcept::Truc)
            .collect();

        let cat_count = truc_nodes
            .iter()
            .filter(|n| n.severity.as_deref() == Some("cat"))
            .count();

        let favorable = EvidenceSelectors::select_favorable_truc_nodes(&graph);

        assert_eq!(
            favorable.len(),
            cat_count,
            "favorable truc selector should match cat quality count"
        );
    }

    #[test]
    fn select_hoang_dao_day_deity_finds_hoang_dao() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let deity_nodes: Vec<_> = graph
            .nodes()
            .values()
            .filter(|n| n.concept == NodeConcept::DayDeity && n.tags.iter().any(|t| t == "hoang_dao"))
            .collect();

        let selected = EvidenceSelectors::select_hoang_dao_day_deity_nodes(&graph);

        assert_eq!(
            selected.len(),
            deity_nodes.len(),
            "hoang dao deity selector should match tagged nodes"
        );
    }

    #[test]
    fn select_hoang_dao_timing_finds_good_hours() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let hoang_dao_count = EvidenceSelectors::select_hoang_dao_timing_nodes(&graph);

        assert_eq!(
            hoang_dao_count.len(),
            1,
            "should have exactly one hoang dao timing node per day"
        );
    }

    #[test]
    fn select_travel_direction_nodes_finds_travel() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let travel_nodes: Vec<_> = graph
            .nodes()
            .values()
            .filter(|n| n.concept == NodeConcept::Direction && n.node_id.contains("travel"))
            .collect();

        let selected = EvidenceSelectors::select_travel_direction_nodes(&graph);

        assert_eq!(
            selected.len(),
            travel_nodes.len(),
            "travel direction selector should match travel nodes"
        );
    }

    #[test]
    fn select_conflict_heavy_xung_hop_finds_luc_xung() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let xung_nodes: Vec<_> = graph
            .nodes()
            .values()
            .filter(|n| {
                n.concept == NodeConcept::XungHop
                    && n.tags.iter().any(|t| t.starts_with("luc_xung="))
            })
            .collect();

        let selected = EvidenceSelectors::select_conflict_heavy_xung_hop_nodes(&graph);

        assert_eq!(
            selected.len(),
            xung_nodes.len(),
            "conflict heavy selector should match luc_xung tagged nodes"
        );
    }

    #[test]
    fn select_activity_nodes_finds_activities() {
        use crate::semantic_graph::builders::build_recommendation_evidence_graph;
        use crate::almanac::recommendation::{synthesize_daily_recommendations, collect_recommendation_hits, RecommendationSynthesisContext};

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

        let graph = build_recommendation_evidence_graph(
            2024, 2, 10, "default",
            &recommendations.activities,
            &hits,
        );

        let selected = EvidenceSelectors::select_activity_nodes(&graph);

        assert!(
            !selected.is_empty(),
            "activity selector should find activities in recommendation graph"
        );
    }

    #[test]
    fn select_recommendation_hit_origin_facts_returns_fact_ids() {
        use crate::semantic_graph::builders::{
            build_day_snapshot_graph,
            build_recommendation_evidence_graph_connected,
        };
        use crate::almanac::recommendation::{synthesize_daily_recommendations, collect_recommendation_hits, RecommendationSynthesisContext};

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

        let hit_nodes: Vec<_> = connected_graph
            .nodes()
            .values()
            .filter(|n| n.concept == NodeConcept::RecommendationHit)
            .collect();

        let mut found_origin = false;
        for hit_node in &hit_nodes {
            let origins = EvidenceSelectors::select_recommendation_hit_origin_facts(
                &connected_graph,
                &hit_node.node_id,
            );
            if !origins.is_empty() {
                found_origin = true;
                break;
            }
        }

        assert!(
            found_origin || hit_nodes.is_empty(),
            "at least one hit should have origin facts in connected graph"
        );
    }

    #[test]
    fn count_hits_by_source_family_sums_correctly() {
        use crate::semantic_graph::builders::build_recommendation_evidence_graph;
        use crate::almanac::recommendation::{synthesize_daily_recommendations, collect_recommendation_hits, RecommendationSynthesisContext};

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

        let graph = build_recommendation_evidence_graph(
            2024, 2, 10, "default",
            &recommendations.activities,
            &hits,
        );

        let counts = EvidenceSelectors::count_hits_by_source_family(&graph);

        let total: usize = counts.truc + counts.day_deity + counts.taboo
            + counts.xung_hop + counts.stars + counts.gio_hoang_dao
            + counts.travel + counts.tiet_khi + counts.other;

        assert_eq!(
            total,
            hits.len(),
            "source family counts total should equal hit count"
        );
    }

    #[test]
    fn select_recommendation_hit_nodes_filters_by_direction() {
        use crate::semantic_graph::builders::build_recommendation_evidence_graph;
        use crate::almanac::recommendation::{synthesize_daily_recommendations, collect_recommendation_hits, RecommendationSynthesisContext};

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

        let graph = build_recommendation_evidence_graph(
            2024, 2, 10, "default",
            &recommendations.activities,
            &hits,
        );

        let favor_hits = EvidenceSelectors::select_recommendation_hit_nodes(
            &graph,
            Some(SelectHitDirection::Favor),
        );
        let avoid_hits = EvidenceSelectors::select_recommendation_hit_nodes(
            &graph,
            Some(SelectHitDirection::Avoid),
        );
        let all_hits = EvidenceSelectors::select_recommendation_hit_nodes(&graph, None);

        assert_eq!(
            favor_hits.len() + avoid_hits.len(),
            all_hits.len(),
            "favor + avoid should equal total hits"
        );
    }

    #[test]
    fn select_hard_stop_recommendation_hits_finds_hard_stops() {
        use crate::semantic_graph::builders::build_recommendation_evidence_graph;
        use crate::almanac::recommendation::{synthesize_daily_recommendations, collect_recommendation_hits, RecommendationSynthesisContext};

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

        let graph = build_recommendation_evidence_graph(
            2024, 2, 10, "default",
            &recommendations.activities,
            &hits,
        );

        let hard_stop_hits = EvidenceSelectors::select_hard_stop_recommendation_hits(&graph);

        let expected_count = graph
            .nodes()
            .values()
            .filter(|n| {
                n.concept == NodeConcept::RecommendationHit
                    && n.tags.iter().any(|t| t == "hard_stop=true")
            })
            .count();

        assert_eq!(
            hard_stop_hits.len(),
            expected_count,
            "hard stop selector should find all hard stop hits"
        );
    }
}