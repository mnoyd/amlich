use crate::semantic_graph::{
    EdgeConcept, NodeConcept, NodeOrigin, ProvenanceEntry, ProvenanceSource, SemanticEdge,
    SemanticGraph, SemanticId, SemanticNode,
};
use crate::sources::{SOURCE_HUYEN_KHONG, SOURCE_VN_FOLK_RITUAL};
use crate::DaySnapshot;

pub struct DaySnapshotGraphBuilder {
    graph: SemanticGraph,
    day_root_id: String,
    tz_suffix: String,
    profile: String,
}

impl DaySnapshotGraphBuilder {
    pub fn new(snapshot: &DaySnapshot) -> Self {
        let tz_suffix = "+7".to_string();
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );
        let day_root_id = SemanticId::day_root(&date_str, &tz_suffix).to_node_id();

        let mut builder = Self {
            graph: SemanticGraph::new(),
            day_root_id,
            tz_suffix,
            profile: snapshot.profile.clone(),
        };

        builder.add_day_root(snapshot);
        builder.add_canchi_facts(snapshot);
        builder.add_solar_term_fact(snapshot);
        builder.add_truc_fact(snapshot);
        builder.add_day_deity_fact(snapshot);
        builder.add_stars_facts(snapshot);
        builder.add_taboo_facts(snapshot);
        builder.add_xung_hop_facts(snapshot);
        builder.add_travel_direction_fact(snapshot);
        builder.add_hoang_dao_hours_fact(snapshot);

        builder
    }

    fn add_day_root(&mut self, snapshot: &DaySnapshot) {
        let provenance = ProvenanceEntry::snapshot(self.day_root_id.clone(), "day_snapshot_v1")
            .with_profile(self.profile.clone());

        let node = SemanticNode::new(
            SemanticId::day_root(
                &format!(
                    "{:04}-{:02}-{:02}",
                    snapshot.context.solar.year,
                    snapshot.context.solar.month,
                    snapshot.context.solar.day
                ),
                &self.tz_suffix,
            ),
            NodeConcept::DayCanchi,
            NodeOrigin::Fact,
            format!(
                "{} {} AL {} {}",
                snapshot.context.lunar.day,
                snapshot.context.lunar.month,
                snapshot.context.canchi.day.full,
                snapshot.context.tiet_khi.name
            ),
        )
        .with_tags(vec!["day-root".to_string(), "lunar".to_string()])
        .with_provenance(provenance);

        self.graph.add_node(node);
    }

    fn add_canchi_facts(&mut self, snapshot: &DaySnapshot) {
        self.add_day_canchi(snapshot);
        self.add_month_canchi(snapshot);
        self.add_year_canchi(snapshot);
    }

    fn add_day_canchi(&mut self, snapshot: &DaySnapshot) {
        let day_canchi = &snapshot.context.canchi.day;
        let node_id =
            SemanticId::day_canchi(day_canchi.can_index, day_canchi.chi_index).to_node_id();

        let provenance = ProvenanceEntry::snapshot(node_id.clone(), "get_day_canchi")
            .with_profile(self.profile.clone());

        let node = SemanticNode::new(
            SemanticId::day_canchi(day_canchi.can_index, day_canchi.chi_index),
            NodeConcept::DayCanchi,
            NodeOrigin::Fact,
            day_canchi.full.clone(),
        )
        .with_tags(vec![
            day_canchi.can.clone(),
            day_canchi.chi.clone(),
            day_canchi.ngu_hanh.can.clone(),
            day_canchi.ngu_hanh.chi.clone(),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_month_canchi(&mut self, snapshot: &DaySnapshot) {
        let month_canchi = &snapshot.context.canchi.month;
        let node_id =
            SemanticId::month_canchi(month_canchi.can_index, month_canchi.chi_index).to_node_id();

        let provenance = ProvenanceEntry::snapshot(node_id.clone(), "get_month_canchi")
            .with_profile(self.profile.clone());

        let node = SemanticNode::new(
            SemanticId::month_canchi(month_canchi.can_index, month_canchi.chi_index),
            NodeConcept::MonthCanchi,
            NodeOrigin::Fact,
            month_canchi.full.clone(),
        )
        .with_tags(vec![month_canchi.can.clone(), month_canchi.chi.clone()])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_year_canchi(&mut self, snapshot: &DaySnapshot) {
        let year_canchi = &snapshot.context.canchi.year;
        let node_id =
            SemanticId::year_canchi(year_canchi.can_index, year_canchi.chi_index).to_node_id();

        let provenance = ProvenanceEntry::snapshot(node_id.clone(), "get_year_canchi")
            .with_profile(self.profile.clone());

        let node = SemanticNode::new(
            SemanticId::year_canchi(year_canchi.can_index, year_canchi.chi_index),
            NodeConcept::YearCanchi,
            NodeOrigin::Fact,
            year_canchi.full.clone(),
        )
        .with_tags(vec![year_canchi.can.clone(), year_canchi.chi.clone()])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_solar_term_fact(&mut self, snapshot: &DaySnapshot) {
        let tiet_khi = &snapshot.context.tiet_khi;
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );
        let node_id = SemanticId::solar_term_day(&date_str, &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::snapshot(node_id.clone(), "get_tiet_khi")
            .with_profile(self.profile.clone());

        let node = SemanticNode::new(
            SemanticId::solar_term(&tiet_khi.name),
            NodeConcept::SolarTerm,
            NodeOrigin::Fact,
            format!("Tiết {}", tiet_khi.name),
        )
        .with_tags(vec![
            tiet_khi.name.clone(),
            format!("lon={}", tiet_khi.longitude),
        ])
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_truc_fact(&mut self, snapshot: &DaySnapshot) {
        let truc = &snapshot.day_fortune.truc;
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );
        let node_id = SemanticId::truc_day(&date_str, &self.tz_suffix).to_node_id();

        let provenance =
            ProvenanceEntry::from_rule_evidence_opt(ProvenanceSource::AlmanacRule, &truc.evidence)
                .unwrap_or_else(|| ProvenanceEntry::snapshot(node_id.clone(), "day_fortune_truc"));

        let node = SemanticNode::new(
            SemanticId::truc(&truc.name),
            NodeConcept::Truc,
            NodeOrigin::Fact,
            format!("Trực {}", truc.name),
        )
        .with_tags(vec![
            truc.name.clone(),
            truc.quality.clone(),
            format!("index={}", truc.index),
        ])
        .with_severity(truc.quality.clone())
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_day_deity_fact(&mut self, snapshot: &DaySnapshot) {
        if let Some(ref day_deity) = snapshot.day_fortune.day_deity {
            let date_str = format!(
                "{:04}-{:02}-{:02}",
                snapshot.context.solar.year,
                snapshot.context.solar.month,
                snapshot.context.solar.day
            );
            let node_id = SemanticId::day_deity_day(&date_str, &self.tz_suffix).to_node_id();

            let provenance = ProvenanceEntry::from_rule_evidence_opt(
                ProvenanceSource::AlmanacRule,
                &day_deity.evidence,
            )
            .unwrap_or_else(|| ProvenanceEntry::snapshot(node_id.clone(), "day_deity"));

            let classification = match day_deity.classification {
                crate::almanac::types::DayDeityClassification::HoangDao => "hoang_dao",
                crate::almanac::types::DayDeityClassification::HacDao => "hac_dao",
            };

            let node = SemanticNode::new(
                SemanticId::day_deity(&day_deity.name),
                NodeConcept::DayDeity,
                NodeOrigin::Fact,
                format!("Ngày {} - {}", day_deity.name, classification),
            )
            .with_tags(vec![day_deity.name.clone(), classification.to_string()])
            .with_severity(classification.to_string())
            .with_provenance(provenance);

            self.graph.add_node(node);

            let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
            self.graph.add_edge(edge);
        }
    }

    fn add_stars_facts(&mut self, snapshot: &DaySnapshot) {
        let stars = &snapshot.day_fortune.stars;

        if !stars.cat_tinh.is_empty() || !stars.sat_tinh.is_empty() || stars.day_star.is_some() {
            let date_str = format!(
                "{:04}-{:02}-{:02}",
                snapshot.context.solar.year,
                snapshot.context.solar.month,
                snapshot.context.solar.day
            );
            let node_id = SemanticId::day_child("stars", &date_str, &self.tz_suffix).to_node_id();

            let provenance = ProvenanceEntry::from_rule_evidence_opt(
                ProvenanceSource::AlmanacRule,
                &stars.evidence,
            )
            .unwrap_or_else(|| ProvenanceEntry::snapshot(node_id.clone(), "day_stars"));

            let summary = if let Some(ref ds) = stars.day_star {
                format!("Ngôi sao chính: {}", ds.name)
            } else {
                let cat = if !stars.cat_tinh.is_empty() {
                    stars.cat_tinh.join(", ")
                } else {
                    "none".to_string()
                };
                let sat = if !stars.sat_tinh.is_empty() {
                    stars.sat_tinh.join(", ")
                } else {
                    "none".to_string()
                };
                format!("Cát tinh: {} | Sát tinh: {}", cat, sat)
            };

            let mut tags = Vec::new();
            if let Some(ref ds) = stars.day_star {
                tags.push(ds.name.clone());
                tags.push(format!("{:?}", ds.quality).to_lowercase());
            }

            let node = SemanticNode::new(
                SemanticId::new("star", format!("day:{}:stars", self.tz_suffix)),
                NodeConcept::Star,
                NodeOrigin::Fact,
                summary,
            )
            .with_tags(tags)
            .with_provenance(provenance);

            self.graph.add_node(node);

            let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
            self.graph.add_edge(edge);
        }
    }

    fn add_taboo_facts(&mut self, snapshot: &DaySnapshot) {
        for taboo in &snapshot.day_fortune.taboos {
            let date_str = format!(
                "{:04}-{:02}-{:02}",
                snapshot.context.solar.year,
                snapshot.context.solar.month,
                snapshot.context.solar.day
            );
            let node_id =
                SemanticId::taboo_day(&date_str, &self.tz_suffix, &taboo.name).to_node_id();

            let provenance = ProvenanceEntry::from_rule_evidence_opt(
                ProvenanceSource::AlmanacRule,
                &taboo.evidence,
            )
            .unwrap_or_else(|| {
                ProvenanceEntry::snapshot(node_id.clone(), &format!("taboo_{}", taboo.rule_id))
            });

            let node = SemanticNode::new(
                SemanticId::taboo(&taboo.rule_id),
                NodeConcept::Taboo,
                NodeOrigin::Fact,
                format!("{}: {}", taboo.name, taboo.reason),
            )
            .with_tags(vec![taboo.rule_id.clone(), taboo.severity.clone()])
            .with_severity(taboo.severity.clone())
            .with_provenance(provenance);

            self.graph.add_node(node);

            let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
            self.graph.add_edge(edge);
        }
    }

    fn add_xung_hop_facts(&mut self, snapshot: &DaySnapshot) {
        let xung_hop = &snapshot.day_fortune.xung_hop;
        let conflict = &snapshot.day_fortune.conflict;
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );

        let node_id = SemanticId::xung_hop_day(&date_str, &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::snapshot(node_id.clone(), "xung_hop_calculate");

        let mut tags = vec![format!("luc_xung={}", xung_hop.luc_xung)];
        if !xung_hop.tam_hop.is_empty() {
            tags.push(format!("tam_hop={}", xung_hop.tam_hop.join(",")));
        }
        if !xung_hop.tu_hanh_xung.is_empty() {
            tags.push(format!("tu_hanh_xung={}", xung_hop.tu_hanh_xung.join(",")));
        }
        if let Some(ref liu_he) = xung_hop.liu_he {
            tags.push(format!("liu_he={}", liu_he));
        }

        let liu_he_part = xung_hop
            .liu_he
            .as_ref()
            .map(|partner| format!(", hợp {}", partner))
            .unwrap_or_default();

        let summary = format!("Xung {}{}", conflict.opposing_chi, liu_he_part);

        let node = SemanticNode::new(
            SemanticId::new("xung_hop", format!("day:{}:{}", date_str, self.tz_suffix)),
            NodeConcept::XungHop,
            NodeOrigin::Fact,
            summary,
        )
        .with_tags(tags)
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_travel_direction_fact(&mut self, snapshot: &DaySnapshot) {
        let travel = &snapshot.day_fortune.travel;
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );
        let node_id = SemanticId::travel_day(&date_str, &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::from_rule_evidence_opt(
            ProvenanceSource::AlmanacRule,
            &travel.evidence,
        )
        .unwrap_or_else(|| ProvenanceEntry::snapshot(node_id.clone(), "travel_direction"));

        let summary = format!(
            "Xuất hành: {} | Tài thần: {} | Hỷ thần: {}",
            travel.xuat_hanh_huong, travel.tai_than, travel.hy_than
        );

        let huyen_khong_prov =
            ProvenanceEntry::almanac_rule(SOURCE_HUYEN_KHONG, "phi_tinh.direction_overlap");

        let node = SemanticNode::new(
            SemanticId::new("direction", format!("travel:day:{}:all", self.tz_suffix)),
            NodeConcept::Direction,
            NodeOrigin::Fact,
            summary,
        )
        .with_tags(vec![
            travel.xuat_hanh_huong.clone(),
            travel.tai_than.clone(),
            travel.hy_than.clone(),
        ])
        .with_provenance(provenance)        // existing khcbppt-family entry
        .with_provenance(huyen_khong_prov); // NEW — INT-04 multi-source

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_hoang_dao_hours_fact(&mut self, snapshot: &DaySnapshot) {
        let gio_hoang_dao = &snapshot.context.gio_hoang_dao;
        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );
        let node_id = SemanticId::hoang_dao_hours_day(&date_str, &self.tz_suffix).to_node_id();

        let provenance = ProvenanceEntry::snapshot(node_id.clone(), "get_gio_hoang_dao");

        let good_hour_names: Vec<_> = gio_hoang_dao
            .good_hours
            .iter()
            .map(|h| h.hour_chi.clone())
            .collect();

        let node = SemanticNode::new(
            SemanticId::new(
                "hoang_dao_hours",
                format!("day:{}:hoang_dao", self.tz_suffix),
            ),
            NodeConcept::HoangDaoHour,
            NodeOrigin::Fact,
            format!(
                "Giờ hoàng đạo: {} ({} giờ)",
                gio_hoang_dao.summary, gio_hoang_dao.good_hour_count
            ),
        )
        .with_tags(good_hour_names)
        .with_severity(gio_hoang_dao.good_hour_count.to_string())
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    pub fn build(self) -> SemanticGraph {
        self.graph
    }
}

pub fn build_day_snapshot_graph(snapshot: &DaySnapshot) -> SemanticGraph {
    DaySnapshotGraphBuilder::new(snapshot).build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calculate_day_snapshot;

    #[test]
    fn day_snapshot_graph_contains_day_root() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let date_str = "2024-02-10";
        let day_root_id = SemanticId::day_root(date_str, "+7").to_node_id();

        assert!(
            graph.has_node(&day_root_id),
            "day root should exist: {}",
            day_root_id
        );
        assert!(
            graph.node_count() > 1,
            "should have day root plus child facts"
        );
    }

    #[test]
    fn day_snapshot_graph_contains_canchi_nodes() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        assert!(
            graph.node_count() >= 3,
            "should have day, month, year canchi nodes"
        );
    }

    #[test]
    fn day_snapshot_graph_contains_truc() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let truc_node_ids: Vec<_> = graph
            .nodes()
            .keys()
            .filter(|id| id.contains("truc"))
            .collect();

        assert!(
            !truc_node_ids.is_empty(),
            "should have at least one truc node"
        );
    }

    #[test]
    fn day_snapshot_graph_edges_connect_to_root() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let day_root_id = SemanticId::day_root("2024-02-10", "+7").to_node_id();

        let root_outgoing = graph.outgoing_edges(&day_root_id);
        assert!(
            !root_outgoing.is_empty(),
            "day root should have outgoing edges to child facts"
        );
    }

    #[test]
    fn day_snapshot_graph_provenance_preserved() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        for (_, node) in graph.nodes() {
            assert!(
                !node.provenance.is_empty(),
                "node {} should have provenance",
                node.node_id
            );
        }
    }

    #[test]
    fn day_snapshot_graph_deterministic() {
        let snapshot1 = calculate_day_snapshot(10, 2, 2024);
        let snapshot2 = calculate_day_snapshot(10, 2, 2024);

        let graph1 = build_day_snapshot_graph(&snapshot1);
        let graph2 = build_day_snapshot_graph(&snapshot2);

        assert_eq!(
            graph1.node_count(),
            graph2.node_count(),
            "node count should be deterministic"
        );
        assert_eq!(
            graph1.edge_count(),
            graph2.edge_count(),
            "edge count should be deterministic"
        );
    }

    #[test]
    fn day_snapshot_graph_different_days_different_ids() {
        let snap1 = calculate_day_snapshot(10, 2, 2024);
        let snap2 = calculate_day_snapshot(11, 2, 2024);

        let graph1 = build_day_snapshot_graph(&snap1);
        let graph2 = build_day_snapshot_graph(&snap2);

        let ids1: Vec<_> = graph1.nodes().keys().collect();
        let ids2: Vec<_> = graph2.nodes().keys().collect();

        assert_ne!(
            ids1, ids2,
            "different days should produce different node IDs"
        );
    }

    #[test]
    fn day_snapshot_graph_stable_day_root_id() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let date_str = "2024-02-10";
        let expected_root = SemanticId::day_root(date_str, "+7").to_node_id();
        assert_eq!(expected_root, "day:2024-02-10:+7");
        assert!(
            graph.has_node(&expected_root),
            "day root should be at expected stable ID"
        );
    }

    #[test]
    fn day_snapshot_graph_edge_kind_is_composes() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let day_root_id = SemanticId::day_root("2024-02-10", "+7").to_node_id();
        for edge in graph.outgoing_edges(&day_root_id) {
            assert_eq!(
                edge.label.concept,
                EdgeConcept::Composes,
                "edges from day root should be Composes"
            );
        }
    }

    #[test]
    fn day_snapshot_graph_provenance_has_source_and_method() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        for (_, node) in graph.nodes() {
            assert!(
                !node.provenance.is_empty(),
                "node {} should have provenance",
                node.node_id
            );
            for prov in &node.provenance {
                assert!(
                    !prov.source_id.is_empty(),
                    "provenance should have source_id"
                );
                assert!(!prov.method.is_empty(), "provenance should have method");
            }
        }
    }

    #[test]
    fn day_snapshot_graph_contains_expected_fact_kinds() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let has_canchi = graph.nodes().values().any(|n| {
            matches!(
                n.concept,
                NodeConcept::DayCanchi | NodeConcept::MonthCanchi | NodeConcept::YearCanchi
            )
        });
        let has_solar_term = graph
            .nodes()
            .values()
            .any(|n| matches!(n.concept, NodeConcept::SolarTerm));
        let has_truc = graph
            .nodes()
            .values()
            .any(|n| matches!(n.concept, NodeConcept::Truc));

        assert!(has_canchi, "should have canchi node");
        assert!(has_solar_term, "should have solar term node");
        assert!(has_truc, "should have truc node");
    }

    #[test]
    fn day_snapshot_graph_no_duplicate_node_ids() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        let node_ids: Vec<_> = graph.nodes().keys().collect();
        let unique_ids: std::collections::HashSet<_> = node_ids.iter().collect();
        assert_eq!(
            node_ids.len(),
            unique_ids.len(),
            "node IDs should be unique"
        );
    }

    #[test]
    fn direction_node_carries_dual_provenance_khcbppt_and_huyen_khong() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let graph = build_day_snapshot_graph(&snapshot);

        // The Direction node has a fixed id
        let direction_id = SemanticId::new("direction", "travel:day:+7:all").to_node_id();
        let node = graph
            .nodes()
            .get(&direction_id)
            .expect("Direction node must exist");

        assert_eq!(
            node.provenance.len(),
            2,
            "Direction node must carry exactly 2 provenance entries (khcbppt-family + huyen-khong); got: {:?}",
            node.provenance
        );

        let source_ids: Vec<&str> = node.provenance.iter().map(|p| p.source_id.as_str()).collect();
        assert!(
            source_ids.contains(&"huyen-khong"),
            "Direction node must have a huyen-khong provenance entry; got: {:?}",
            source_ids
        );
    }
}
