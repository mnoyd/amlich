use std::collections::HashMap;

use crate::semantic_graph::{NodeConcept, SemanticNode};

pub(crate) fn cluster_for_node_id(_node_id: &str, concept: NodeConcept) -> String {
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
    }
}

pub(crate) fn format_node_summary_point(node: &SemanticNode) -> String {
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
    parts.join(" | ")
}

pub(crate) fn opt_map(map: HashMap<String, usize>) -> Option<HashMap<String, usize>> {
    if map.is_empty() {
        None
    } else {
        Some(map)
    }
}

pub(crate) struct NodeViewAccumulator {
    pub cluster_counts: HashMap<String, usize>,
    pub semantic_kind_counts: HashMap<String, usize>,
    pub severity_counts: HashMap<String, usize>,
    pub summary_points: Vec<String>,
}

impl NodeViewAccumulator {
    pub fn new() -> Self {
        Self {
            cluster_counts: HashMap::new(),
            semantic_kind_counts: HashMap::new(),
            severity_counts: HashMap::new(),
            summary_points: Vec::new(),
        }
    }

    pub fn accumulate(&mut self, node: &SemanticNode) {
        let cluster = cluster_for_node_id(&node.node_id, node.concept);
        *self.cluster_counts.entry(cluster).or_insert(0) += 1;

        let kind_label = node.concept.label().as_str().to_string();
        *self.semantic_kind_counts.entry(kind_label).or_insert(0) += 1;

        if let Some(ref sev) = node.severity {
            *self.severity_counts.entry(sev.clone()).or_insert(0) += 1;
        }

        self.summary_points.push(format_node_summary_point(node));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::semantic_graph::ids::SemanticId;
    use crate::semantic_graph::provenance::{ProvenanceEntry, ProvenanceSource};
    use crate::semantic_graph::{NodeConcept, NodeOrigin, SemanticNode};

    #[test]
    fn cluster_day_core_concepts() {
        for concept in [
            NodeConcept::Truc,
            NodeConcept::DayDeity,
            NodeConcept::Taboo,
            NodeConcept::XungHop,
            NodeConcept::HoangDaoHour,
            NodeConcept::Star,
            NodeConcept::Direction,
            NodeConcept::Element,
        ] {
            assert_eq!(
                cluster_for_node_id("test:1", concept),
                "day-core",
                "{:?} should be day-core",
                concept
            );
        }
    }

    #[test]
    fn cluster_interaction_core_concepts() {
        for concept in [
            NodeConcept::ChartPillar,
            NodeConcept::DayPersonMatrix,
            NodeConcept::InteractionRow,
            NodeConcept::TenGodRelation,
        ] {
            assert_eq!(
                cluster_for_node_id("test:1", concept),
                "interaction-core",
                "{:?} should be interaction-core",
                concept
            );
        }
    }

    #[test]
    fn cluster_recommendation_concepts() {
        assert_eq!(
            cluster_for_node_id("activity:1", NodeConcept::Activity),
            "recommendation-evidence"
        );
        assert_eq!(
            cluster_for_node_id("rec:1", NodeConcept::Recommendation),
            "recommendation-summary"
        );
    }

    #[test]
    fn cluster_fallback_by_id_prefix() {
        assert_eq!(
            cluster_for_node_id("bazi_profile:foo", NodeConcept::ChartPillar),
            "interaction-core"
        );
    }

    #[test]
    fn format_summary_point_basic() {
        let node = SemanticNode::new(
            SemanticId::new("truc", "test:1"),
            NodeConcept::Truc,
            NodeOrigin::Fact,
            "Truc Cat",
        );
        let point = format_node_summary_point(&node);
        assert!(point.starts_with("[truc] Truc Cat"));
    }

    #[test]
    fn format_summary_point_with_severity() {
        let node = SemanticNode::new(
            SemanticId::new("truc", "test:1"),
            NodeConcept::Truc,
            NodeOrigin::Fact,
            "Truc Cat",
        )
        .with_severity("cat");
        let point = format_node_summary_point(&node);
        assert!(point.contains("severity=cat"));
    }

    #[test]
    fn format_summary_point_with_provenance() {
        let node = SemanticNode::new(
            SemanticId::new("truc", "test:1"),
            NodeConcept::Truc,
            NodeOrigin::Fact,
            "Truc Cat",
        )
        .with_provenance(ProvenanceEntry::snapshot("s1", "compute"));
        let point = format_node_summary_point(&node);
        assert!(point.contains("sources="));
    }

    #[test]
    fn accumulator_counts_clusters() {
        let mut acc = NodeViewAccumulator::new();
        acc.accumulate(&SemanticNode::new(
            SemanticId::new("truc", "truc:1"),
            NodeConcept::Truc,
            NodeOrigin::Fact,
            "Cat",
        ));
        acc.accumulate(&SemanticNode::new(
            SemanticId::new("taboo", "taboo:1"),
            NodeConcept::Taboo,
            NodeOrigin::Fact,
            "Hard",
        ));
        assert_eq!(acc.cluster_counts.get("day-core"), Some(&2));
        assert_eq!(acc.summary_points.len(), 2);
    }

    #[test]
    fn opt_map_returns_none_when_empty() {
        let map: HashMap<String, usize> = HashMap::new();
        assert!(opt_map(map).is_none());
    }

    #[test]
    fn opt_map_returns_some_when_nonempty() {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert("day-core".to_string(), 5);
        assert!(opt_map(map).is_some());
    }
}
