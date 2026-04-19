use crate::bazi::analysis::{BaziAnalysisReport, ChartInteraction, DayMasterStrength, ElementDistribution, TenGodDistribution};
use crate::bazi::types::BaziChart;
use crate::semantic_graph::{
    EdgeConcept, NodeConcept, NodeOrigin, ProvenanceEntry, ProvenanceSource,
    SemanticEdge, SemanticGraph, SemanticId, SemanticNode,
};

pub struct BaziGraphBuilder {
    graph: SemanticGraph,
    profile_root_id: String,
    tz_suffix: String,
    profile: String,
}

impl BaziGraphBuilder {
    pub fn new(chart: &BaziChart, analysis: &BaziAnalysisReport) -> Self {
        let tz_suffix = format!("tz{:.1}", chart.input.timezone);
        let dob_str = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        );
        let profile_root_id = SemanticId::bazi_profile(&dob_str, &tz_suffix).to_node_id();

        let mut builder = Self {
            graph: SemanticGraph::new(),
            profile_root_id,
            tz_suffix,
            profile: "bazi".to_string(),
        };

        builder.add_profile_root(chart);
        builder.add_pillars(chart);
        builder.add_day_master(chart);
        builder.add_element_distribution(chart, analysis);
        builder.add_ten_god_distribution(chart, analysis);
        builder.add_day_master_strength(chart, analysis);
        builder.add_chart_interactions(chart, analysis);

        builder
    }

    fn add_profile_root(&mut self, chart: &BaziChart) {
        let provenance = ProvenanceEntry::bazi(self.profile_root_id.clone(), "bazi_chart_v1");

        let dob_str = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        );
        let label = format!(
            "Bazi {} tuổi, giờ {}",
            chart.input.year, chart.input.hour
        );

        let node = SemanticNode::new(
            SemanticId::bazi_profile(&dob_str, &self.tz_suffix),
            NodeConcept::PersonalAlignment,
            NodeOrigin::Fact,
            label,
        )
        .with_tags(vec![
            format!("year={}", chart.input.year),
            format!("month={}", chart.input.month),
            format!("day={}", chart.input.day),
            format!("hour={}", chart.input.hour),
            chart.input.gender.map(|g| format!("{:?}", g)).unwrap_or_else(|| "unknown".to_string()),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);
    }

    fn add_pillars(&mut self, chart: &BaziChart) {
        self.add_year_pillar(chart);
        self.add_month_pillar(chart);
        self.add_day_pillar(chart);
        if let Some(ref hour_pillar) = chart.hour_pillar {
            self.add_hour_pillar(chart, hour_pillar);
        }
    }

    fn add_year_pillar(&mut self, chart: &BaziChart) {
        let pillar = &chart.year_pillar;
        let node_id = SemanticId::pillar_bazi("year", &format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        ), &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::bazi(node_id.clone(), "bazi_pillar_year");

        let node = SemanticNode::new(
            SemanticId::chart_pillar("year", pillar.can_chi.can_index, pillar.can_chi.chi_index),
            NodeConcept::ChartPillar,
            NodeOrigin::Fact,
            format!("Năm: {}", pillar.can_chi.full),
        )
        .with_tags(vec![
            pillar.can_chi.can.clone(),
            pillar.can_chi.chi.clone(),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_month_pillar(&mut self, chart: &BaziChart) {
        let pillar = &chart.month_pillar;
        let node_id = SemanticId::pillar_bazi("month", &format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        ), &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::bazi(node_id.clone(), "bazi_pillar_month");

        let node = SemanticNode::new(
            SemanticId::chart_pillar("month", pillar.can_chi.can_index, pillar.can_chi.chi_index),
            NodeConcept::ChartPillar,
            NodeOrigin::Fact,
            format!("Tháng: {}", pillar.can_chi.full),
        )
        .with_tags(vec![
            pillar.can_chi.can.clone(),
            pillar.can_chi.chi.clone(),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_day_pillar(&mut self, chart: &BaziChart) {
        let pillar = &chart.day_pillar;
        let node_id = SemanticId::pillar_bazi("day", &format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        ), &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::bazi(node_id.clone(), "bazi_pillar_day");

        let node = SemanticNode::new(
            SemanticId::chart_pillar("day", pillar.can_chi.can_index, pillar.can_chi.chi_index),
            NodeConcept::ChartPillar,
            NodeOrigin::Fact,
            format!("Ngày: {}", pillar.can_chi.full),
        )
        .with_tags(vec![
            pillar.can_chi.can.clone(),
            pillar.can_chi.chi.clone(),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_hour_pillar(&mut self, chart: &BaziChart, pillar: &crate::bazi::types::BaziPillar) {
        let node_id = SemanticId::pillar_bazi("hour", &format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        ), &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::bazi(node_id.clone(), "bazi_pillar_hour");

        let node = SemanticNode::new(
            SemanticId::chart_pillar("hour", pillar.can_chi.can_index, pillar.can_chi.chi_index),
            NodeConcept::ChartPillar,
            NodeOrigin::Fact,
            format!("Giờ: {}", pillar.can_chi.full),
        )
        .with_tags(vec![
            pillar.can_chi.can.clone(),
            pillar.can_chi.chi.clone(),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_day_master(&mut self, chart: &BaziChart) {
        let dm = &chart.day_master;
        let node_id = SemanticId::new("day_master", format!("bazi:{}:{}", self.tz_suffix, "self")).to_node_id();

        let provenance = ProvenanceEntry::bazi(node_id.clone(), "bazi_day_master");

        let node = SemanticNode::new(
            SemanticId::new("day_master", format!("bazi:{}", self.tz_suffix)),
            NodeConcept::DayCanchi,
            NodeOrigin::Fact,
            format!("Thiên Can Ngày: {}", dm.full),
        )
        .with_tags(vec![dm.can.clone(), dm.chi.clone()])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_element_distribution(&mut self, chart: &BaziChart, analysis: &BaziAnalysisReport) {
        let ed = &analysis.element_distribution;
        let node_id = SemanticId::element_distribution_bazi(&format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        ), &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::bazi(node_id.clone(), "compute_element_distribution");

        let summary = format!(
            "Mộc={}, Hỏa={}, Thổ={}, Kim={}, Thủy={}",
            ed.moc, ed.hoa, ed.tho, ed.kim, ed.thuy
        );

        let node = SemanticNode::new(
            SemanticId::new("element", format!("bazi:{}:element_dist", self.tz_suffix)),
            NodeConcept::Element,
            NodeOrigin::Fact,
            summary,
        )
        .with_tags(vec![
            format!("moc={}", ed.moc),
            format!("hoa={}", ed.hoa),
            format!("tho={}", ed.tho),
            format!("kim={}", ed.kim),
            format!("thuy={}", ed.thuy),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_ten_god_distribution(&mut self, chart: &BaziChart, analysis: &BaziAnalysisReport) {
        let td = &analysis.ten_god_distribution;
        let node_id = SemanticId::ten_god_distribution_bazi(&format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        ), &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::bazi(node_id.clone(), "compute_ten_god_distribution");

        let summary = format!(
            "Tỷ Kiến={}, Kiếp Tai={}, Thực Thần={}, Thương Quan={}, Chính Tài={}, Thiên Tài={}, Chính Quân={}, Thất Sát={}, Chính An={}, Thiên An={}",
            td.ty_kien, td.kiep_tai, td.thuc_than, td.thuong_quan, td.chinh_tai, td.thien_tai, td.chinh_quan, td.that_sat, td.chinh_an, td.thien_an
        );

        let node = SemanticNode::new(
            SemanticId::new("ten_god", format!("bazi:{}:ten_god_dist", self.tz_suffix)),
            NodeConcept::AxisSignal,
            NodeOrigin::Fact,
            summary,
        )
        .with_tags(vec![
            format!("ty_kien={}", td.ty_kien),
            format!("kiep_tai={}", td.kiep_tai),
            format!("thuc_than={}", td.thuc_than),
            format!("thuong_quan={}", td.thuong_quan),
            format!("chinh_tai={}", td.chinh_tai),
            format!("thien_tai={}", td.thien_tai),
            format!("chinh_quan={}", td.chinh_quan),
            format!("that_sat={}", td.that_sat),
            format!("chinh_an={}", td.chinh_an),
            format!("thien_an={}", td.thien_an),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_day_master_strength(&mut self, chart: &BaziChart, analysis: &BaziAnalysisReport) {
        let dms = &analysis.day_master_strength;
        let node_id = SemanticId::day_master_strength_bazi(&format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            chart.input.year, chart.input.month, chart.input.day,
            chart.input.hour, chart.input.minute
        ), &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::bazi(node_id.clone(), "evaluate_day_master_strength");

        let label = match dms.label {
            crate::bazi::analysis::DayMasterStrengthLabel::Strong => "Mạnh",
            crate::bazi::analysis::DayMasterStrengthLabel::Balanced => "Cân Bằng",
            crate::bazi::analysis::DayMasterStrengthLabel::Weak => "Yếu",
        };

        let node = SemanticNode::new(
            SemanticId::new("day_master_strength", format!("bazi:{}:dms", self.tz_suffix)),
            NodeConcept::PersonalAlignment,
            NodeOrigin::Interpreted,
            format!("{} (score={})", label, dms.score),
        )
        .with_tags(vec![label.to_lowercase(), format!("score={}", dms.score)])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_chart_interactions(&mut self, chart: &BaziChart, analysis: &BaziAnalysisReport) {
        for (i, interaction) in analysis.interactions.iter().enumerate() {
            let node_id = SemanticId::chart_interaction_bazi(&format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}",
                chart.input.year, chart.input.month, chart.input.day,
                chart.input.hour, chart.input.minute
            ), &self.tz_suffix, i).to_node_id();

            let provenance = ProvenanceEntry::bazi(node_id.clone(), "detect_chart_interactions");

            let kind_str = match interaction.kind {
                crate::bazi::analysis::ChartInteractionKind::BranchClash => "clash",
                crate::bazi::analysis::ChartInteractionKind::BranchHarmony => "harmony",
                crate::bazi::analysis::ChartInteractionKind::BranchHarm => "harm",
            };

            let node = SemanticNode::new(
                SemanticId::new("chart_interaction", format!("bazi:{}:{}", self.tz_suffix, i)),
                NodeConcept::InteractionSignal,
                NodeOrigin::Interpreted,
                interaction.summary_vi.clone(),
            )
            .with_tags(vec![
                kind_str.to_string(),
                interaction.participants.join(","),
            ])
            .with_provenance(provenance);

            self.graph.add_node(node);

            let edge = SemanticEdge::new(&self.profile_root_id, &node_id, EdgeConcept::Composes);
            self.graph.add_edge(edge);
        }
    }

    pub fn build(self) -> SemanticGraph {
        self.graph
    }
}

pub fn build_bazi_profile_graph(
    chart: &BaziChart,
    analysis: &BaziAnalysisReport,
) -> SemanticGraph {
    BaziGraphBuilder::new(chart, analysis).build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bazi::{analyze_bazi_chart, build_bazi_chart, BaziInput};

    fn make_test_input() -> BaziInput {
        BaziInput {
            day: 15,
            month: 8,
            year: 1990,
            hour: 9,
            minute: 30,
            timezone: 7.0,
            longitude: None,
            use_solar_time: false,
            gender: Some(crate::almanac::tu_menh::Gender::Male),
        }
    }

    #[test]
    fn bazi_graph_contains_profile_root() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        let dob_str = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            input.year, input.month, input.day, input.hour, input.minute
        );
        let tz = format!("tz{:.1}", input.timezone);
        let profile_root_id = SemanticId::bazi_profile(&dob_str, &tz).to_node_id();

        assert!(graph.has_node(&profile_root_id), "profile root should exist");
        assert!(graph.node_count() >= 5, "should have profile + pillars + analysis nodes");
    }

    #[test]
    fn bazi_graph_contains_pillars() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        let pillar_ids: Vec<_> = graph.nodes().keys()
            .filter(|id| id.contains("pillar:") || id.starts_with("chart_pillar"))
            .collect();

        assert!(pillar_ids.len() >= 3, "should have year, month, day pillars minimum, got: {:?}", pillar_ids);
    }

    #[test]
    fn bazi_graph_contains_analysis_nodes() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        let has_element_dist = graph.nodes().keys().any(|id| id.contains("element"));
        let has_ten_god = graph.nodes().keys().any(|id| id.contains("ten_god"));
        let has_dms = graph.nodes().keys().any(|id| id.contains("day_master_strength"));

        assert!(has_element_dist, "should have element distribution node");
        assert!(has_ten_god, "should have ten god distribution node");
        assert!(has_dms, "should have day master strength node");
    }

    #[test]
    fn bazi_graph_provenance_preserved() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        for (_, node) in graph.nodes() {
            assert!(!node.provenance.is_empty(), "node {} should have provenance", node.node_id);
        }
    }

    #[test]
    fn bazi_graph_deterministic() {
        let input = make_test_input();
        let chart1 = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis1 = analyze_bazi_chart(&chart1);
        let chart2 = build_bazi_chart(input).expect("valid chart");
        let analysis2 = analyze_bazi_chart(&chart2);

        let graph1 = build_bazi_profile_graph(&chart1, &analysis1);
        let graph2 = build_bazi_profile_graph(&chart2, &analysis2);

        assert_eq!(graph1.node_count(), graph2.node_count(), "node count should be deterministic");
        assert_eq!(graph1.edge_count(), graph2.edge_count(), "edge count should be deterministic");
    }

    #[test]
    fn bazi_graph_different_inputs_different_ids() {
        let input1 = make_test_input();
        let chart1 = build_bazi_chart(input1).expect("valid chart");
        let analysis1 = analyze_bazi_chart(&chart1);

        let mut input2 = make_test_input();
        input2.year = 1991;
        let chart2 = build_bazi_chart(input2).expect("valid chart");
        let analysis2 = analyze_bazi_chart(&chart2);

        let graph1 = build_bazi_profile_graph(&chart1, &analysis1);
        let graph2 = build_bazi_profile_graph(&chart2, &analysis2);

        let ids1: Vec<_> = graph1.nodes().keys().collect();
        let ids2: Vec<_> = graph2.nodes().keys().collect();

        assert_ne!(ids1, ids2, "different inputs should produce different node IDs");
    }

    #[test]
    fn bazi_graph_stable_profile_root_id() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        let dob_str = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            input.year, input.month, input.day, input.hour, input.minute
        );
        let tz = format!("tz{:.1}", input.timezone);
        let expected_profile_id = format!("bazi_profile:{}:{}", dob_str, tz);

        assert!(graph.has_node(&expected_profile_id), "profile root should be at expected stable ID: {}", expected_profile_id);
    }

    #[test]
    fn bazi_graph_edge_kind_is_composes() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        let dob_str = format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}",
            input.year, input.month, input.day, input.hour, input.minute
        );
        let tz = format!("tz{:.1}", input.timezone);
        let profile_root_id = format!("bazi_profile:{}:{}", dob_str, tz);

        for edge in graph.outgoing_edges(&profile_root_id) {
            assert_eq!(edge.label.concept, EdgeConcept::Composes, "edges from profile root should be Composes");
        }
    }

    #[test]
    fn bazi_graph_provenance_has_source_and_method() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        for (_, node) in graph.nodes() {
            assert!(!node.provenance.is_empty(), "node {} should have provenance", node.node_id);
            for prov in &node.provenance {
                assert!(!prov.source_id.is_empty(), "provenance should have source_id");
                assert!(!prov.method.is_empty(), "provenance should have method");
            }
        }
    }

    #[test]
    fn bazi_graph_contains_expected_fact_kinds() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        let has_chart_pillar = graph.nodes().values().any(|n| matches!(n.concept, NodeConcept::ChartPillar));
        let has_element = graph.nodes().values().any(|n| matches!(n.concept, NodeConcept::Element));
        let has_personal_alignment = graph.nodes().values().any(|n| matches!(n.concept, NodeConcept::PersonalAlignment));

        assert!(has_chart_pillar, "should have chart pillar nodes");
        assert!(has_element, "should have element distribution node");
        assert!(has_personal_alignment, "should have personal alignment (profile/day master strength) nodes");
    }

    #[test]
    fn bazi_graph_no_duplicate_node_ids() {
        let input = make_test_input();
        let chart = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis = analyze_bazi_chart(&chart);

        let graph = build_bazi_profile_graph(&chart, &analysis);

        let node_ids: Vec<_> = graph.nodes().keys().collect();
        let unique_ids: std::collections::HashSet<_> = node_ids.iter().collect();
        assert_eq!(node_ids.len(), unique_ids.len(), "node IDs should be unique");
    }

    #[test]
    fn bazi_graph_determinism_across_multiple_builds() {
        let input = make_test_input();
        let chart1 = build_bazi_chart(input.clone()).expect("valid chart");
        let analysis1 = analyze_bazi_chart(&chart1);
        let chart2 = build_bazi_chart(input).expect("valid chart");
        let analysis2 = analyze_bazi_chart(&chart2);

        let graph1 = build_bazi_profile_graph(&chart1, &analysis1);
        let graph2 = build_bazi_profile_graph(&chart2, &analysis2);

        let mut ids1: Vec<_> = graph1.nodes().keys().collect();
        let mut ids2: Vec<_> = graph2.nodes().keys().collect();
        ids1.sort();
        ids2.sort();
        assert_eq!(ids1, ids2, "same inputs should produce identical node IDs across builds");
    }
}