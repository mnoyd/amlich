use crate::almanac::recommendation::{BaseDirection, RecommendationHit, SynthesizedRecommendation};
use crate::semantic_graph::{
    EdgeConcept, NodeConcept, NodeOrigin, ProvenanceEntry, SemanticEdge, SemanticGraph, SemanticId,
    SemanticNode,
};

pub struct RecommendationEvidenceGraphBuilder {
    graph: SemanticGraph,
    profile: String,
    day_fact_ids: DayFactIds,
}

struct DayFactIds {
    truc: Option<String>,
    day_deity: Option<String>,
    taboo_prefix: String,
    xung_hop: String,
    travel: String,
    gio_hoang_dao: String,
}

impl RecommendationEvidenceGraphBuilder {
    pub fn new(year: i32, month: i32, day: i32, profile: &str) -> Self {
        let tz_suffix = "+7".to_string();
        let date_str = format!("{:04}-{:02}-{:02}", year, month, day);

        let day_fact_ids = DayFactIds {
            truc: None,
            day_deity: None,
            taboo_prefix: format!("taboo:day:{}:{}:", date_str, tz_suffix),
            xung_hop: format!("xung_hop:day:{}:{}", date_str, tz_suffix),
            travel: format!("direction:travel:day:{}:all", tz_suffix),
            gio_hoang_dao: format!("hoang_dao_hours:day:{}:hoang_dao", tz_suffix),
        };

        let builder = Self {
            graph: SemanticGraph::new(),
            profile: profile.to_string(),
            day_fact_ids,
        };

        builder
    }

    pub fn with_day_snapshot_graph(mut self, snapshot_graph: &SemanticGraph) -> Self {
        self.graph.merge(snapshot_graph.clone()).ok();

        if let Some(node_id) = self.find_truc_node(snapshot_graph) {
            self.day_fact_ids.truc = Some(node_id);
        }
        if let Some(node_id) = self.find_day_deity_node(snapshot_graph) {
            self.day_fact_ids.day_deity = Some(node_id);
        }

        self
    }

    fn find_truc_node(&self, graph: &SemanticGraph) -> Option<String> {
        graph
            .nodes()
            .keys()
            .find(|id| id.starts_with("truc:"))
            .cloned()
    }

    fn find_day_deity_node(&self, graph: &SemanticGraph) -> Option<String> {
        graph
            .nodes()
            .keys()
            .find(|id| id.starts_with("day_deity:"))
            .cloned()
    }

    pub fn with_activities(mut self, activities: &[SynthesizedRecommendation]) -> Self {
        self.add_activity_nodes(activities);
        self
    }

    fn add_activity_nodes(&mut self, activities: &[SynthesizedRecommendation]) {
        for activity in activities {
            let node_id = format!("activity:{}", activity.activity_id.as_str());
            let label = format!("{} / {}", activity.label.vi, activity.label.en);

            let provenance =
                ProvenanceEntry::snapshot(node_id.clone(), "recommendation_activity_v1")
                    .with_profile(self.profile.clone());

            let node = SemanticNode::new(
                SemanticId::new("activity", activity.activity_id.as_str()),
                NodeConcept::Activity,
                NodeOrigin::Interpreted,
                label,
            )
            .with_tags(vec![format!("bucket={:?}", activity.bucket)])
            .with_provenance(provenance);

            self.graph.add_node(node);
        }
    }

    pub fn add_hit_nodes(&mut self, hits: &[RecommendationHit]) {
        for hit in hits {
            let hit_key = hit.hit_id.strip_prefix("hit:").unwrap_or(&hit.hit_id);
            let node_id = format!("hit:{}", hit_key);

            let provenance = ProvenanceEntry::snapshot(node_id.clone(), "recommendation_hit_v1")
                .with_profile(self.profile.clone());

            let direction_tag = match hit.direction {
                BaseDirection::Favor => "favor",
                BaseDirection::Avoid => "avoid",
            };

            let mut tags = vec![
                format!("source={:?}", hit.source),
                direction_tag.to_string(),
            ];

            if hit.hard_stop {
                tags.push("hard_stop=true".to_string());
            }

            let node = SemanticNode::new(
                SemanticId::new("hit", hit_key),
                NodeConcept::RecommendationHit,
                NodeOrigin::Interpreted,
                hit.summary_vi.clone(),
            )
            .with_tags(tags)
            .with_severity(format!("{:?}", hit.severity))
            .with_provenance(provenance);

            self.graph.add_node(node);

            let activity_node_id = format!("activity:{}", hit.activity_id.as_str());
            if self.graph.has_node(&activity_node_id) {
                let edge =
                    SemanticEdge::new(&node_id, &activity_node_id, EdgeConcept::TargetsActivity);
                self.graph.add_edge(edge);
            }

            self.add_origin_edges(&node_id, hit);
        }
    }

    fn add_origin_edges(&mut self, hit_node_id: &str, hit: &RecommendationHit) {
        match hit.source {
            crate::almanac::recommendation::RecommendationEvidenceSource::Truc => {
                if let Some(truc_node_id) = &self.day_fact_ids.truc {
                    let edge =
                        SemanticEdge::new(truc_node_id, hit_node_id, EdgeConcept::OriginatesFrom);
                    self.graph.add_edge(edge);
                }
            }
            crate::almanac::recommendation::RecommendationEvidenceSource::DayDeity => {
                if let Some(deity_node_id) = &self.day_fact_ids.day_deity {
                    let edge =
                        SemanticEdge::new(deity_node_id, hit_node_id, EdgeConcept::OriginatesFrom);
                    self.graph.add_edge(edge);
                }
            }
            crate::almanac::recommendation::RecommendationEvidenceSource::Taboo => {
                let fact_node_id = format!(
                    "{}{}",
                    self.day_fact_ids.taboo_prefix,
                    hit.source_code.strip_prefix("taboo.").unwrap_or("unknown")
                );
                if self.graph.has_node(&fact_node_id) {
                    let edge =
                        SemanticEdge::new(&fact_node_id, hit_node_id, EdgeConcept::OriginatesFrom);
                    self.graph.add_edge(edge);
                }
            }
            crate::almanac::recommendation::RecommendationEvidenceSource::XungHop => {
                if self.graph.has_node(&self.day_fact_ids.xung_hop) {
                    let edge = SemanticEdge::new(
                        &self.day_fact_ids.xung_hop,
                        hit_node_id,
                        EdgeConcept::OriginatesFrom,
                    );
                    self.graph.add_edge(edge);
                }
            }
            crate::almanac::recommendation::RecommendationEvidenceSource::Travel => {
                if self.graph.has_node(&self.day_fact_ids.travel) {
                    let edge = SemanticEdge::new(
                        &self.day_fact_ids.travel,
                        hit_node_id,
                        EdgeConcept::OriginatesFrom,
                    );
                    self.graph.add_edge(edge);
                }
            }
            crate::almanac::recommendation::RecommendationEvidenceSource::GioHoangDao => {
                if self.graph.has_node(&self.day_fact_ids.gio_hoang_dao) {
                    let edge = SemanticEdge::new(
                        &self.day_fact_ids.gio_hoang_dao,
                        hit_node_id,
                        EdgeConcept::OriginatesFrom,
                    );
                    self.graph.add_edge(edge);
                }
            }
            _ => {}
        }
    }

    pub fn build(self) -> SemanticGraph {
        self.graph
    }
}

pub fn build_recommendation_evidence_graph(
    year: i32,
    month: i32,
    day: i32,
    profile: &str,
    activities: &[SynthesizedRecommendation],
    hits: &[RecommendationHit],
) -> SemanticGraph {
    let mut builder = RecommendationEvidenceGraphBuilder::new(year, month, day, profile);
    builder = builder.with_activities(activities);
    builder.add_hit_nodes(hits);
    builder.build()
}

pub fn build_recommendation_evidence_graph_connected(
    year: i32,
    month: i32,
    day: i32,
    profile: &str,
    activities: &[SynthesizedRecommendation],
    hits: &[RecommendationHit],
    day_fact_graph: &SemanticGraph,
) -> SemanticGraph {
    let mut builder = RecommendationEvidenceGraphBuilder::new(year, month, day, profile);
    builder = builder.with_day_snapshot_graph(day_fact_graph);
    builder = builder.with_activities(activities);
    builder.add_hit_nodes(hits);
    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::recommendation::{
        collect_recommendation_hits, RecommendationSynthesisContext,
    };
    use crate::calculate_day_snapshot;

    #[test]
    fn recommendation_evidence_graph_contains_activity_nodes() {
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

        let activity_nodes: Vec<_> = graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, NodeConcept::Activity))
            .collect();

        assert!(
            !activity_nodes.is_empty(),
            "graph should contain activity nodes"
        );
    }

    #[test]
    fn recommendation_evidence_graph_contains_hit_nodes() {
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

        let hit_nodes: Vec<_> = graph
            .nodes()
            .values()
            .filter(|n| matches!(n.concept, NodeConcept::RecommendationHit))
            .collect();

        assert!(!hit_nodes.is_empty(), "graph should contain hit nodes");
    }

    #[test]
    fn hit_nodes_target_activity_nodes() {
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

        for (node_id, node) in graph.nodes() {
            if matches!(node.concept, NodeConcept::RecommendationHit) {
                let outgoing = graph.outgoing_edges(node_id);
                assert!(
                    outgoing
                        .iter()
                        .any(|e| matches!(e.label.concept, EdgeConcept::TargetsActivity)),
                    "hit node {} should have TargetsActivity edge",
                    node_id
                );
            }
        }
    }

    #[test]
    fn recommendation_evidence_graph_provenance_preserved() {
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

        for (_, node) in graph.nodes() {
            assert!(
                !node.provenance.is_empty(),
                "node {} should have provenance",
                node.node_id
            );
        }
    }
}
