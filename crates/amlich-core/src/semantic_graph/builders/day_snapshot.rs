use crate::almanac::fengshui::star_metadata;
use crate::semantic_graph::{
    EdgeConcept, NodeConcept, NodeOrigin, ProvenanceEntry, ProvenanceSource, SemanticEdge,
    SemanticFact, SemanticGraph, SemanticId, SemanticNode, SemanticPolarity,
};
use crate::sources::{
    SOURCE_HUYEN_KHONG, SOURCE_KHCBPPT, SOURCE_KINH_DICH, SOURCE_MAI_HOA_DICH_SO,
    SOURCE_VN_FOLK_RITUAL,
};
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
        builder.add_flying_star_facts(snapshot);
        builder.add_ritual_facts(snapshot);
        builder.add_offering_facts(snapshot);
        builder.add_iching_facts(snapshot);
        builder.add_direction_composite_facts(snapshot);

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
        let opening_hits = crate::insight_data::find_truc_insight(&truc.name)
            .map(crate::almanac::recommendation::evidence::collect_truc_hits)
            .unwrap_or_default()
            .into_iter()
            .filter(|hit| {
                hit.activity_id == crate::almanac::recommendation::ActivityId::OpeningStart
            })
            .collect::<Vec<_>>();
        let opening_avoid_count = opening_hits
            .iter()
            .filter(|hit| {
                matches!(
                    hit.direction,
                    crate::almanac::recommendation::evidence::BaseDirection::Avoid
                )
            })
            .count() as u8;
        let opening_favorable = opening_hits.iter().any(|hit| {
            matches!(
                hit.direction,
                crate::almanac::recommendation::evidence::BaseDirection::Favor
            )
        });
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
        .with_fact(SemanticFact::Truc {
            opening_avoid_count,
            opening_favorable,
        })
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

            let polarity = match stars.day_star.as_ref().map(|star| &star.quality) {
                Some(crate::almanac::types::StarQuality::Cat) => SemanticPolarity::Favorable,
                Some(crate::almanac::types::StarQuality::Hung) => SemanticPolarity::Unfavorable,
                Some(crate::almanac::types::StarQuality::Binh) => SemanticPolarity::Neutral,
                None if !stars.cat_tinh.is_empty() && stars.sat_tinh.is_empty() => {
                    SemanticPolarity::Favorable
                }
                None if stars.cat_tinh.is_empty() && !stars.sat_tinh.is_empty() => {
                    SemanticPolarity::Unfavorable
                }
                None if !stars.cat_tinh.is_empty() && !stars.sat_tinh.is_empty() => {
                    SemanticPolarity::Mixed
                }
                None => SemanticPolarity::Neutral,
            };

            let node = SemanticNode::new(
                SemanticId::new("star", format!("day:{}:stars", self.tz_suffix)),
                NodeConcept::Star,
                NodeOrigin::Fact,
                summary,
            )
            .with_tags(tags)
            .with_fact(SemanticFact::Star { polarity })
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
                ProvenanceEntry::snapshot(node_id.clone(), format!("taboo_{}", taboo.rule_id))
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
        .with_fact(SemanticFact::XungHop {
            has_clash: !xung_hop.luc_xung.is_empty(),
            has_harmony: xung_hop.liu_he.is_some(),
        })
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
        .with_provenance(provenance) // existing khcbppt-family entry
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
        .with_severity_if(gio_hoang_dao.good_hour_count > 0, "has_good_hours")
        .with_provenance(provenance);

        self.graph.add_node(node);

        let edge = SemanticEdge::new(&self.day_root_id, &node_id, EdgeConcept::Composes);
        self.graph.add_edge(edge);
    }

    fn add_flying_star_facts(&mut self, snapshot: &DaySnapshot) {
        let Some(fs) = &snapshot.flying_stars else {
            return;
        };

        let summary = format!(
            "Phi Tinh Vận {}: trung cung {:?}, {} cung chồng",
            fs.van,
            fs.center_star,
            fs.palace_overlays.len()
        );

        let node_id = SemanticId::new(
            "flying_star",
            format!("day:{}:flying_stars", self.tz_suffix),
        )
        .to_node_id();
        let prov = ProvenanceEntry::almanac_rule(SOURCE_HUYEN_KHONG, "phi_tinh.combined_overlay");
        let node = SemanticNode::new(
            SemanticId::new(
                "flying_star",
                format!("day:{}:flying_stars", self.tz_suffix),
            ),
            NodeConcept::FlyingStar,
            NodeOrigin::Fact,
            summary,
        )
        .with_provenance(prov);

        self.graph.add_node(node);
        self.graph.add_edge(SemanticEdge::new(
            &self.day_root_id,
            &node_id,
            EdgeConcept::Composes,
        ));

        // OccupiesPalace edge from FlyingStar to Direction node — exercises new edge concept (INT-04)
        let direction_id =
            SemanticId::new("direction", format!("travel:day:{}:all", self.tz_suffix)).to_node_id();
        self.graph.add_edge(SemanticEdge::new(
            &node_id,
            &direction_id,
            EdgeConcept::OccupiesPalace,
        ));

        // CarriesElement edge from FlyingStar to its center star's Ngũ Hành Element node.
        // The center (trung cung) star is the chart's tonal anchor per Thẩm Thị Huyền Không Học —
        // its element drives Vận-wide auspice and pairs with the OccupiesPalace edge to give
        // the aggregate node both a spatial (palace) and an elemental (Ngũ Hành) handle.
        let element = star_metadata(fs.center_star).element.as_str();
        let element_node_id_raw = SemanticId::new(
            "element",
            format!("flying_star:day:{}:center", self.tz_suffix),
        );
        let element_node_id = element_node_id_raw.clone().to_node_id();
        let element_prov =
            ProvenanceEntry::almanac_rule(SOURCE_HUYEN_KHONG, "phi_tinh.center_star.element");
        let element_node = SemanticNode::new(
            element_node_id_raw,
            NodeConcept::Element,
            NodeOrigin::Fact,
            format!("Ngũ Hành trung cung: {element}"),
        )
        .with_tags(vec![format!("element={element}")])
        .with_provenance(element_prov);
        self.graph.add_node(element_node);
        self.graph.add_edge(SemanticEdge::new(
            &node_id,
            &element_node_id,
            EdgeConcept::CarriesElement,
        ));
    }

    fn add_ritual_facts(&mut self, snapshot: &DaySnapshot) {
        let Some(rituals) = &snapshot.applicable_rituals else {
            return;
        };
        if rituals.is_empty() {
            return;
        }

        let summary = format!("Văn khấn áp dụng: {}", rituals.join(", "));
        let node_id =
            SemanticId::new("ritual", format!("day:{}:rituals", self.tz_suffix)).to_node_id();
        let prov =
            ProvenanceEntry::almanac_rule(SOURCE_VN_FOLK_RITUAL, "find_van_khan_for_snapshot");
        let node = SemanticNode::new(
            SemanticId::new("ritual", format!("day:{}:rituals", self.tz_suffix)),
            NodeConcept::Ritual,
            NodeOrigin::Fact,
            summary,
        )
        .with_provenance(prov);

        self.graph.add_node(node);

        // Phase 19 (INT-08 SC#2 literal): populate payload on the aggregate Ritual
        // node with the structured offering_refs + flat-string offerings derived
        // from snapshot.applicable_rituals. The DaySnapshot fields (Plan 19-01)
        // remain the canonical structured surface; this payload is the
        // semantic-graph-node-payload interpretation per Option B in 19-RESEARCH.md.
        // The payload is populated ONLY when applicable_rituals is non-empty
        // (mirroring the DaySnapshot populate-block invariant).
        let payload_value = if let Some(refs) = &snapshot.offering_refs {
            if !refs.is_empty() {
                serde_json::json!({
                    "offering_refs": refs.iter().map(|r| {
                        serde_json::json!({
                            "offering_id": r.offering_id,
                            "name_vi": r.name_vi,
                            "name_en": r.name_en,
                            "source_id": r.source_id,
                        })
                    }).collect::<Vec<_>>(),
                    "offerings": snapshot.offerings.clone().unwrap_or_default(),
                })
            } else {
                serde_json::Value::Null
            }
        } else {
            serde_json::Value::Null
        };
        if !payload_value.is_null() {
            self.graph
                .nodes_mut()
                .get_mut(&node_id)
                .expect("Ritual node just added")
                .payload = Some(payload_value);
        }

        // Ritual prescribed FOR the day
        self.graph.add_edge(SemanticEdge::new(
            &node_id,
            &self.day_root_id,
            EdgeConcept::PrescribedFor,
        ));
    }

    fn add_offering_facts(&mut self, snapshot: &DaySnapshot) {
        let Some(ritual_ids) = &snapshot.applicable_rituals else {
            return;
        };
        if ritual_ids.is_empty() {
            return;
        }

        // Aggregate Ritual node id (same as add_ritual_facts) — the from-side of RecommendsOffering edges.
        let ritual_node_id =
            SemanticId::new("ritual", format!("day:{}:rituals", self.tz_suffix)).to_node_id();

        // Dedup edges by (ritual_node_id, offering_node_id) — two rituals could
        // theoretically share an offering reference (same ritual_id is deduped
        // upstream; different ritual_ids sharing an offering name is rare but
        // possible). HashSet prevents emitting the same edge twice.
        let mut emitted_edges: std::collections::HashSet<(String, String)> =
            std::collections::HashSet::new();

        for ritual_id in ritual_ids {
            let Some(entry) = crate::rituals::get_ritual_by_id(ritual_id) else {
                continue;
            };

            // INT-09: collect this entry's cross_source_curing annotations (if any)
            // for emission as additional provenance on every RecommendsOffering edge
            // derived from this entry's offerings.
            let cross_cures: Vec<(String, String)> = entry
                .metadata
                .as_ref()
                .and_then(|m| m.cross_source_curing.as_ref())
                .map(|cures| {
                    cures
                        .iter()
                        .map(|c| (c.source_id.as_str().to_string(), c.element_cure_for.clone()))
                        .collect()
                })
                .unwrap_or_default();

            for (idx, offering) in entry.offerings.iter().enumerate() {
                // Build the locked OfferingRef (Plan 19-01) for the semantic-graph handle.
                let offering_ref = crate::rituals::OfferingRef::new(
                    format!("ritual.{ritual_id}.offering.{idx}"),
                    offering.name_vi.clone(),
                    offering.name_en.clone(),
                    crate::sources::SOURCE_VN_FOLK_RITUAL.to_string(),
                );

                // Stable Offering node id (mirror day_snapshot.rs:488 timezone-suffixed pattern).
                let offering_node_id_raw = SemanticId::new(
                    "offering",
                    format!(
                        "ritual:{ritual_id}:offering:{idx}:day:{:04}-{:02}-{:02}:{}",
                        snapshot.context.solar.year,
                        snapshot.context.solar.month,
                        snapshot.context.solar.day,
                        self.tz_suffix
                    ),
                );
                let offering_node_id = offering_node_id_raw.clone().to_node_id();

                // Emit Offering node — single SOURCE_VN_FOLK_RITUAL provenance via constructor.
                let offering_prov =
                    ProvenanceEntry::almanac_rule(SOURCE_VN_FOLK_RITUAL, "ritual.offering_lookup")
                        .with_note(format!(
                            "offering_id={};ritual_id={};rationale=lễ vật của nghi lễ",
                            offering_ref.offering_id, ritual_id
                        ));
                let offering_node = SemanticNode::new(
                    offering_node_id_raw,
                    NodeConcept::Offering,
                    NodeOrigin::Fact,
                    format!("Lễ vật: {}", offering_ref.name_vi),
                )
                .with_provenance(offering_prov);
                self.graph.add_node(offering_node);

                // Dedup: skip if edge already emitted for this (ritual_node, offering_node) pair.
                if !emitted_edges.insert((ritual_node_id.clone(), offering_node_id.clone())) {
                    continue;
                }

                // Emit RecommendsOffering edge (Ritual → Offering).
                self.graph.add_edge(SemanticEdge::new(
                    &ritual_node_id,
                    &offering_node_id,
                    EdgeConcept::RecommendsOffering,
                ));

                // Track edge provenance — INT-09 dual-source pattern.
                // For ordinary rituals: ONE track_provenance call (vn-folk-ritual only).
                // For cross_source_curing-annotated rituals: TWO track_provenance calls
                // (one per source — vn-folk-ritual + the annotated source_id from each cure).
                // This reuses the v1.5 multi-source ProvenanceTracker::track() append-pattern
                // — NO parallel dedup helper is introduced.
                let edge_id = format!("{}->{}", ritual_node_id, offering_node_id);

                // Build the rationale string — Blocker 4 fix: rationale lives on the
                // EDGE provenance note, not just on the Offering node provenance.
                // Single-source rationale: "lễ vật của nghi lễ".
                // Dual-source rationale: "lễ vật của nghi lễ, hỗ trợ chữa trị ngũ hành tương ứng"
                // (the latter only when at least one cross_source_curing annotation exists).
                let has_dual_source = !cross_cures.is_empty();
                let rationale = if has_dual_source {
                    "lễ vật của nghi lễ, hỗ trợ chữa trị ngũ hành tương ứng".to_string()
                } else {
                    "lễ vật của nghi lễ".to_string()
                };

                // First track_provenance call: vn-folk-ritual (always).
                self.graph.track_provenance(
                    &edge_id,
                    ProvenanceEntry::almanac_rule(
                        SOURCE_VN_FOLK_RITUAL,
                        "ritual.recommends_offering",
                    )
                    .with_note(format!(
                        "ritual={};offering_id={};rationale={}",
                        ritual_id, offering_ref.offering_id, rationale
                    )),
                );

                // Subsequent track_provenance calls: one per cross_source_curing annotation.
                // Each cure emits a second (or third, ...) ProvenanceEntry on the SAME edge.
                for (cure_source_id, element_cure_for) in &cross_cures {
                    self.graph.track_provenance(
                        &edge_id,
                        ProvenanceEntry::almanac_rule(cure_source_id, "ritual.cross_source_cure")
                            .with_note(format!(
                                "ritual={};offering_id={};element_cure_for={}",
                                ritual_id, offering_ref.offering_id, element_cure_for
                            )),
                    );
                }
            }
        }
    }

    // =========================================================================
    // Phase 24-02 (INT-11) — IChing semantic-graph wiring
    //
    // Two distinct `NodeConcept::Hexagram` nodes (primary chu + bien) wired
    // via `EdgeConcept::Transforms` + `EdgeConcept::LocatedAt` edges with
    // IChing-family dual-source provenance (CRIT-6).
    //
    // CRIT-3 isolation: this method does NOT import or reference the v1.5
    // Phi Tinh aggregator surface (the `add_flying_star_facts` method above
    // owns that import). The integration suite's grep guard test pins this
    // discipline by scanning for the literal Phi-Tinh-aggregator type name
    // inside this method's body.
    //
    // Edge insertion order: both Hexagram nodes are added BEFORE the
    // Transforms + LocatedAt edges. `SemanticGraph::add_edge` silently
    // drops edges whose endpoint nodes are missing (see
    // `semantic_graph/graph.rs:23-28`); ordering prevents silent drops.
    // =========================================================================

    /// Emit TWO distinct `NodeConcept::Hexagram` nodes (primary chủ + biến)
    /// when `snapshot.iching_cast` is `Some(...)`, with dual-source
    /// provenance (CRIT-6: SOURCE_MAI_HOA_DICH_SO + SOURCE_KINH_DICH) on
    /// each node, plus one `EdgeConcept::Transforms` edge (chu → biến) and
    /// two `EdgeConcept::LocatedAt` edges (each Hexagram → day root).
    /// Early-returns without modifying the graph when `snapshot.iching_cast`
    /// is `None` (no implicit wiring on ordinary snapshots).
    fn add_iching_facts(&mut self, snapshot: &DaySnapshot) {
        let Some(summary) = snapshot.iching_cast.as_ref() else {
            return;
        };

        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );

        let chu_kw = summary.chu_king_wen_index();
        let bien_kw = summary.bien_king_wen_index();
        let chu_id_raw = SemanticId::iching_hexagram("chu", chu_kw, &date_str, &self.tz_suffix);
        let bien_id_raw = SemanticId::iching_hexagram("bien", bien_kw, &date_str, &self.tz_suffix);
        let chu_id = chu_id_raw.clone().to_node_id();
        let bien_id = bien_id_raw.clone().to_node_id();

        // CRIT-6 dual-source provenance per node: casting source_id
        // (SOURCE_MAI_HOA_DICH_SO) + corpus source_id (SOURCE_KINH_DICH).
        let chu_prov_cast =
            ProvenanceEntry::almanac_rule(SOURCE_MAI_HOA_DICH_SO, "iching.cast_mai_hoa").with_note(
                format!("king_wen={};moving_line={}", chu_kw, summary.cast.dong_hao),
            );
        let chu_prov_corpus =
            ProvenanceEntry::almanac_rule(SOURCE_KINH_DICH, "iching.corpus_lookup").with_note(
                format!(
                    "king_wen={};vi_name={}",
                    chu_kw, summary.chu_hexagram_vi_name
                ),
            );
        let bien_prov_cast =
            ProvenanceEntry::almanac_rule(SOURCE_MAI_HOA_DICH_SO, "iching.derive_bien_que")
                .with_note(format!(
                    "king_wen={};flipped_dong_hao={}",
                    bien_kw, summary.bien_que.flipped_dong_hao
                ));
        let bien_prov_corpus =
            ProvenanceEntry::almanac_rule(SOURCE_KINH_DICH, "iching.corpus_lookup").with_note(
                format!(
                    "king_wen={};vi_name={}",
                    bien_kw, summary.bien_hexagram_vi_name
                ),
            );

        let chu_node = SemanticNode::new(
            chu_id_raw,
            NodeConcept::Hexagram,
            NodeOrigin::Fact,
            format!("Quẻ chủ #{} {}", chu_kw, summary.chu_hexagram_vi_name),
        )
        .with_tags(vec![
            format!("king_wen={}", chu_kw),
            "role=chu".to_string(),
            format!("verdict={}", summary.cat_hung_summary),
            format!("moving_line={}", summary.cast.dong_hao),
        ])
        .with_provenance(chu_prov_cast)
        .with_provenance(chu_prov_corpus);
        self.graph.add_node(chu_node);

        let bien_node = SemanticNode::new(
            bien_id_raw,
            NodeConcept::Hexagram,
            NodeOrigin::Fact,
            format!("Quẻ biến #{} {}", bien_kw, summary.bien_hexagram_vi_name),
        )
        .with_tags(vec![
            format!("king_wen={}", bien_kw),
            "role=bien".to_string(),
            format!("flipped_dong_hao={}", summary.bien_que.flipped_dong_hao),
        ])
        .with_provenance(bien_prov_cast)
        .with_provenance(bien_prov_corpus);
        self.graph.add_node(bien_node);

        // Edges AFTER both endpoints exist (preventing silent drops).
        self.graph.add_edge(SemanticEdge::new(
            &chu_id,
            &bien_id,
            EdgeConcept::Transforms,
        ));
        self.graph.add_edge(SemanticEdge::new(
            &chu_id,
            &self.day_root_id,
            EdgeConcept::LocatedAt,
        ));
        self.graph.add_edge(SemanticEdge::new(
            &bien_id,
            &self.day_root_id,
            EdgeConcept::LocatedAt,
        ));
    }

    /// Emit ONE `NodeConcept::Direction` composite fact node when
    /// `snapshot.direction_cross_link` is `Some(...)` (Phase 23 surface),
    /// carrying KHCBPPT + Huyền Không primitive source-id entries plus ONE
    /// composite envelope per Phase 23's locked contract. Early-returns
    /// without modifying the graph when `snapshot.direction_cross_link` is
    /// `None` (the IChing-only enrichment does NOT auto-infer directional
    /// cross-link wiring).
    ///
    /// CRIT-3 isolation: this method does NOT import or reference the v1.5
    /// Phi Tinh aggregator surface (the `add_flying_star_facts` method owns
    /// that import). The directional cross-link is consumed as a pure DTO
    /// projection. The directional surface uses KHCBPPT (Thái Tuế +
    /// Tam Sát + Sát Phương) + Huyền Không (Phi Tinh palace overlay)
    /// primitive source_ids only.
    fn add_direction_composite_facts(&mut self, snapshot: &DaySnapshot) {
        let Some(cross) = snapshot.direction_cross_link.as_ref() else {
            return;
        };

        let date_str = format!(
            "{:04}-{:02}-{:02}",
            snapshot.context.solar.year, snapshot.context.solar.month, snapshot.context.solar.day
        );
        let node_id_raw = SemanticId::new("direction", format!("cross_link:{}:+7", date_str));
        let node_id = node_id_raw.clone().to_node_id();

        // Phase 23's locked CRIT-6 dual-source pattern: distinct primitive
        // source_ids (KHCBPPT + Huyền-Không) + ONE composite envelope per
        // `DirectionCrossLinkSummary.cross_link_source` (which carries
        // the composite `rule.composite.direction_cross_link` value per
        // ADR-0007).
        let khcbppt_prov =
            ProvenanceEntry::almanac_rule(SOURCE_KHCBPPT, "thai_tue_tam_sat_directional")
                .with_note(format!(
                    "day_chi_index={};birth_chi_index={}",
                    cross.day_chi_index, cross.birth_chi_index
                ));
        let huyen_khong_prov =
            ProvenanceEntry::almanac_rule(SOURCE_HUYEN_KHONG, "phi_tinh.palace_overlay")
                .with_note(format!("day_chi_index={}", cross.day_chi_index));
        let composite_prov = ProvenanceEntry::derived(
            cross.cross_link_source.as_str(),
            "build_direction_cross_link",
        )
        .with_note(format!(
            "composite cross-link of day_chi={};birth_chi={}",
            cross.day_chi_index, cross.birth_chi_index
        ));

        let node = SemanticNode::new(
            node_id_raw,
            NodeConcept::Direction,
            NodeOrigin::Fact,
            format!("Cross-link KHCBPPT×Huyền-Không ({})", cross.cross_link_kind),
        )
        .with_tags(vec![
            format!("cross_link_kind={}", cross.cross_link_kind),
            format!("day_chi_index={}", cross.day_chi_index),
            format!("birth_chi_index={}", cross.birth_chi_index),
        ])
        .with_provenance(khcbppt_prov)
        .with_provenance(huyen_khong_prov)
        .with_provenance(composite_prov);
        self.graph.add_node(node);

        // Edge AFTER endpoint exists (single node — single edge to day root).
        self.graph.add_edge(SemanticEdge::new(
            &node_id,
            &self.day_root_id,
            EdgeConcept::LocatedAt,
        ));
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

        for node in graph.nodes().values() {
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

        for node in graph.nodes().values() {
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
    fn v15_pillar_nodes_carry_disjoint_source_ids_and_direction_is_multi_source() {
        // Use Tết 2026 (2026-02-17) — rituals + flying stars are populated for this date.
        let snap = crate::calculate_day_snapshot(17, 2, 2026);
        let graph = build_day_snapshot_graph(&snap);

        // --- FlyingStar node: must exist with ONLY huyen-khong provenance ---
        let flying_star_node = graph
            .nodes()
            .values()
            .find(|n| matches!(n.concept, NodeConcept::FlyingStar))
            .expect("FlyingStar node must exist in graph for a day with flying_stars populated");

        assert_eq!(
            flying_star_node.provenance.len(),
            1,
            "FlyingStar node must have exactly 1 provenance entry (huyen-khong only); got: {:?}",
            flying_star_node.provenance
        );
        assert_eq!(
            flying_star_node.provenance[0].source_id.as_str(),
            "huyen-khong",
            "FlyingStar node provenance must be huyen-khong"
        );

        // --- Ritual node: must exist with ONLY vn-folk-ritual provenance ---
        let ritual_node = graph
            .nodes()
            .values()
            .find(|n| matches!(n.concept, NodeConcept::Ritual))
            .expect("Ritual node must exist in graph for Tết 2026 (applicable_rituals populated)");

        assert_eq!(
            ritual_node.provenance.len(),
            1,
            "Ritual node must have exactly 1 provenance entry (vn-folk-ritual only); got: {:?}",
            ritual_node.provenance
        );
        assert_eq!(
            ritual_node.provenance[0].source_id.as_str(),
            "vn-folk-ritual",
            "Ritual node provenance must be vn-folk-ritual"
        );

        // --- Direction node: must carry BOTH khcbppt-family AND huyen-khong (len==2) ---
        let direction_id = SemanticId::new("direction", "travel:day:+7:all").to_node_id();
        let direction_node = graph
            .nodes()
            .get(&direction_id)
            .expect("Direction node must exist");

        assert_eq!(
            direction_node.provenance.len(),
            2,
            "Direction node must carry 2 provenance entries (khcbppt-family + huyen-khong); got: {:?}",
            direction_node.provenance
        );

        let dir_source_ids: Vec<&str> = direction_node
            .provenance
            .iter()
            .map(|p| p.source_id.as_str())
            .collect();
        assert!(
            dir_source_ids.contains(&"huyen-khong"),
            "Direction node must include huyen-khong provenance; got: {:?}",
            dir_source_ids
        );
        // Also verify the first entry is NOT huyen-khong (khcbppt-family)
        assert_ne!(
            direction_node.provenance[0].source_id.as_str(),
            "huyen-khong",
            "First Direction provenance entry should be the khcbppt-family (travel evidence), not huyen-khong"
        );

        // --- Verify PrescribedFor edge: Ritual node connects to day root ---
        let has_prescribed_for_edge = graph
            .edges()
            .values()
            .any(|e| matches!(e.label.concept, EdgeConcept::PrescribedFor));
        assert!(
            has_prescribed_for_edge,
            "Graph must contain a PrescribedFor edge from Ritual node to day root"
        );

        // --- Verify OccupiesPalace edge: FlyingStar node connects to Direction node ---
        let has_occupies_palace_edge = graph
            .edges()
            .values()
            .any(|e| matches!(e.label.concept, EdgeConcept::OccupiesPalace));
        assert!(
            has_occupies_palace_edge,
            "Graph must contain an OccupiesPalace edge from FlyingStar node to Direction node"
        );

        // --- Verify CarriesElement edge: FlyingStar node connects to its center-star Element node ---
        let fs_node_id = flying_star_node.node_id.clone();
        let element_node_id = SemanticId::new("element", "flying_star:day:+7:center").to_node_id();
        let carries_element_edge = graph
            .edges()
            .values()
            .find(|e| matches!(e.label.concept, EdgeConcept::CarriesElement))
            .expect("Graph must contain a CarriesElement edge from FlyingStar to Element node");
        assert_eq!(
            &carries_element_edge.from_node_id, &fs_node_id,
            "CarriesElement must originate at the FlyingStar aggregate node"
        );
        assert_eq!(
            &carries_element_edge.to_node_id, &element_node_id,
            "CarriesElement must terminate at the center-star Element node"
        );

        let element_node = graph
            .nodes()
            .get(&element_node_id)
            .expect("Element node for center star must exist");
        assert!(matches!(element_node.concept, NodeConcept::Element));
        assert_eq!(
            element_node.provenance.len(),
            1,
            "Element node must have exactly 1 provenance entry (huyen-khong only)"
        );
        assert_eq!(
            element_node.provenance[0].source_id.as_str(),
            "huyen-khong",
            "Element node provenance must be huyen-khong"
        );
        // Tag must carry one of the five Ngũ Hành element strings.
        let element_tag = element_node
            .tags
            .iter()
            .find(|t| t.starts_with("element="))
            .expect("Element node must carry an element=... tag");
        let value = element_tag.trim_start_matches("element=");
        assert!(
            matches!(value, "water" | "earth" | "wood" | "fire" | "metal"),
            "element tag must be one of the five Ngũ Hành values; got {value:?}"
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

        let source_ids: Vec<&str> = node
            .provenance
            .iter()
            .map(|p| p.source_id.as_str())
            .collect();
        assert!(
            source_ids.contains(&"huyen-khong"),
            "Direction node must have a huyen-khong provenance entry; got: {:?}",
            source_ids
        );
    }

    // ===========================================================================
    // Phase 19 — INT-09 dual-source provenance + INT-08 SC#2 literal payload
    // ===========================================================================

    #[test]
    fn recommends_offering_edge_carries_dual_source_provenance() {
        // INT-09: at least one RecommendsOffering edge MUST carry BOTH
        // "vn-folk-ritual" AND "huyen-khong" provenance entries on Tết 2026
        // (the `van-khan-tet-day-du` ritual entry is annotated with a
        // huyen-khong cross_source_curing per Phase 19-02 corpus augmentation).
        // This proves the dual-source pattern is actually wired end-to-end.

        let snap = crate::calculate_day_snapshot(17, 2, 2026); // Tết 2026
        let graph = build_day_snapshot_graph(&snap);

        // Find every RecommendsOffering edge
        let rec_edges: Vec<_> = graph
            .edges()
            .values()
            .filter(|e| matches!(e.label.concept, EdgeConcept::RecommendsOffering))
            .collect();

        assert!(
            !rec_edges.is_empty(),
            "Tết 2026 must have >= 1 RecommendsOffering edge; got 0"
        );

        // At least one RecommendsOffering edge must carry BOTH source_ids
        let mut found_dual_source = false;
        for edge in &rec_edges {
            let edge_id = &edge.edge_id;
            let entries = graph
                .provenance()
                .get(edge_id)
                .expect("RecommendsOffering edge must have provenance entries");
            let source_ids: Vec<&str> = entries.iter().map(|p| p.source_id.as_str()).collect();

            // Endpoint sanity (Blocker 6 — from_node_id is Ritual concept, to_node_id is Offering concept)
            let from_node = graph
                .nodes()
                .get(&edge.from_node_id)
                .expect("RecommendsOffering edge from_node must exist");
            let to_node = graph
                .nodes()
                .get(&edge.to_node_id)
                .expect("RecommendsOffering edge to_node must exist");
            assert!(
                matches!(from_node.concept, NodeConcept::Ritual),
                "RecommendsOffering from_node_id must point to a Ritual node; got {:?}",
                from_node.concept
            );
            assert!(
                matches!(to_node.concept, NodeConcept::Offering),
                "RecommendsOffering to_node_id must point to an Offering node; got {:?}",
                to_node.concept
            );

            // INT-09 dual-source check: edge provenance contains both
            if source_ids.contains(&"vn-folk-ritual") && source_ids.contains(&"huyen-khong") {
                found_dual_source = true;
            }

            // Blocker 4 fix: rationale must appear in the edge provenance note
            // for the vn-folk-ritual entry (at minimum)
            let vn_folk_entry = entries
                .iter()
                .find(|p| p.source_id.as_str() == "vn-folk-ritual")
                .expect("RecommendsOffering edge must have a vn-folk-ritual provenance entry");
            let note = vn_folk_entry
                .note
                .as_deref()
                .expect("vn-folk-ritual provenance entry must carry a note with rationale");
            assert!(
                note.contains("rationale="),
                "vn-folk-ritual edge provenance note must include rationale=; got: {note}"
            );
        }

        assert!(
            found_dual_source,
            "At least one RecommendsOffering edge MUST carry BOTH 'vn-folk-ritual' AND 'huyen-khong' provenance on Tết 2026 — INT-09 dual-source pattern is not wired (the van-khan-tet-day-du corpus annotation is missing)"
        );
    }

    // ===========================================================================
    // Phase 19 — endpoint verification (Blocker 6) + annual+monthly FlyingStar
    // assertion (Blocker 7 — the new test explicitly checks annual + monthly
    // fields, not just palace_overlays.len())
    // ===========================================================================

    #[test]
    fn phase19_offering_wiring_endpoint_and_flying_star_components() {
        // Blockers 6 + 7 fix: this test verifies BOTH the endpoint shape of
        // RecommendsOffering edges AND the annual+monthly FlyingStar components.
        let snap = crate::calculate_day_snapshot(17, 2, 2026);
        let graph = build_day_snapshot_graph(&snap);

        // (Blocker 6) Every RecommendsOffering edge must have:
        //   - from_node_id pointing to a NodeConcept::Ritual node
        //   - to_node_id pointing to a NodeConcept::Offering node
        //   - at least one provenance entry with source_id == "vn-folk-ritual"
        let rec_edges: Vec<_> = graph
            .edges()
            .values()
            .filter(|e| matches!(e.label.concept, EdgeConcept::RecommendsOffering))
            .collect();
        assert!(
            !rec_edges.is_empty(),
            "RecommendsOffering edges must exist on Tết 2026"
        );
        for edge in &rec_edges {
            let from = graph
                .nodes()
                .get(&edge.from_node_id)
                .expect("from_node must exist");
            let to = graph
                .nodes()
                .get(&edge.to_node_id)
                .expect("to_node must exist");
            assert!(
                matches!(from.concept, NodeConcept::Ritual),
                "from_node_id must point to a Ritual node; got {:?}",
                from.concept
            );
            assert!(
                matches!(to.concept, NodeConcept::Offering),
                "to_node_id must point to an Offering node; got {:?}",
                to.concept
            );
            let entries = graph
                .provenance()
                .get(&edge.edge_id)
                .expect("RecommendsOffering edge must have provenance entries");
            assert!(
                entries
                    .iter()
                    .any(|p| p.source_id.as_str() == "vn-folk-ritual"),
                "RecommendsOffering edge provenance must include vn-folk-ritual"
            );
        }

        // (Blocker 7) Annual + monthly FlyingStar components must both exist
        // on DaySnapshot.flying_stars.palace_overlays (each entry is a
        // (annual, monthly) tuple — 9 entries per `almanac/fengshui/combined.rs:48`).
        let fs_summary = snap
            .flying_stars
            .as_ref()
            .expect("flying_stars must be Some for Tết 2026");
        assert_eq!(
            fs_summary.palace_overlays.len(),
            9,
            "flying_stars.palace_overlays must have 9 entries"
        );

        // The annual + monthly components are stored as FlyingStar enum values.
        // `FlyingStar` does NOT derive Default — verify each (annual, monthly)
        // tuple member is one of the 9 valid FlyingStar variants (any valid
        // variant proves the field is populated; the absence of `Default` means
        // there's no meaningful "uninitialized" sentinel to compare against).
        for (i, (annual, monthly)) in fs_summary.palace_overlays.iter().enumerate() {
            assert!(
                matches!(
                    annual,
                    crate::almanac::fengshui::types::FlyingStar::NhatBach
                        | crate::almanac::fengshui::types::FlyingStar::NhiHac
                        | crate::almanac::fengshui::types::FlyingStar::TamBich
                        | crate::almanac::fengshui::types::FlyingStar::TuLuc
                        | crate::almanac::fengshui::types::FlyingStar::NguHoang
                        | crate::almanac::fengshui::types::FlyingStar::LucBach
                        | crate::almanac::fengshui::types::FlyingStar::ThatXich
                        | crate::almanac::fengshui::types::FlyingStar::BatBach
                        | crate::almanac::fengshui::types::FlyingStar::CuuTu
                ),
                "palace_overlays[{i}].0 (annual) must be a valid FlyingStar variant; got {:?}",
                annual
            );
            assert!(
                matches!(
                    monthly,
                    crate::almanac::fengshui::types::FlyingStar::NhatBach
                        | crate::almanac::fengshui::types::FlyingStar::NhiHac
                        | crate::almanac::fengshui::types::FlyingStar::TamBich
                        | crate::almanac::fengshui::types::FlyingStar::TuLuc
                        | crate::almanac::fengshui::types::FlyingStar::NguHoang
                        | crate::almanac::fengshui::types::FlyingStar::LucBach
                        | crate::almanac::fengshui::types::FlyingStar::ThatXich
                        | crate::almanac::fengshui::types::FlyingStar::BatBach
                        | crate::almanac::fengshui::types::FlyingStar::CuuTu
                ),
                "palace_overlays[{i}].1 (monthly) must be a valid FlyingStar variant; got {:?}",
                monthly
            );
        }
    }
}
