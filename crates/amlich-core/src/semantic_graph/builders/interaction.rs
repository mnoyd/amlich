use crate::almanac::types::{FiveElement, FiveElementRelation, RuleEvidence, ThapThanLabel};
use crate::interaction::types::{
    BranchRelation, DayPersonMatrix, DirectionMergeMatrix, DirectionSignal, ElementInteraction,
    PersonalHourMatrix,
};
use crate::semantic_graph::{
    EdgeConcept, NodeConcept, NodeOrigin, ProvenanceEntry, SemanticEdge, SemanticGraph, SemanticId,
    SemanticNode,
};

pub struct InteractionGraphBuilder {
    graph: SemanticGraph,
    day_id: String,
    profile_id: String,
    profile: String,
}

impl InteractionGraphBuilder {
    pub fn new(day_id: &str, profile_id: &str) -> Self {
        Self {
            graph: SemanticGraph::new(),
            day_id: day_id.to_string(),
            profile_id: profile_id.to_string(),
            profile: "baseline".to_string(),
        }
    }

    fn provenance(&self, source_id: &str, method: &str) -> ProvenanceEntry {
        ProvenanceEntry::interaction(source_id, method).with_profile(self.profile.clone())
    }

    pub fn build(self) -> SemanticGraph {
        self.graph
    }

    pub fn add_matrix_root(
        &mut self,
        _matrix_id: &str,
        matrix_kind: NodeConcept,
        summary: &str,
    ) -> String {
        let node_concept_label = matrix_kind
            .as_str()
            .unwrap_or("matrix")
            .replace("_matrix", "");
        let stable_key = format!("{}:{}", self.day_id, self.profile_id);
        let id = SemanticId::new(&node_concept_label, &stable_key);
        let node_id = id.to_node_id();
        let node = SemanticNode::new(id, matrix_kind, NodeOrigin::Interpreted, summary)
            .with_tags(vec![format!(
                "matrix:{}/{}",
                node_concept_label, self.day_id
            )])
            .with_provenance(self.provenance(&stable_key, "interaction_graph_builder"));

        self.graph.add_node(node);
        node_id
    }

    pub fn add_matrix_root_with_tags(
        &mut self,
        matrix_id: &str,
        matrix_kind: NodeConcept,
        summary: &str,
        tags: Vec<String>,
    ) -> String {
        let node_id = self.add_matrix_root(matrix_id, matrix_kind, summary);
        self.extend_node_tags(&node_id, tags);
        node_id
    }

    pub fn extend_node_tags(&mut self, node_id: &str, tags: Vec<String>) {
        if let Some(existing) = self.graph.get_node(node_id).cloned() {
            let mut updated = existing;
            updated.tags.extend(tags);
            self.graph.add_node(updated);
        }
    }

    pub fn add_row_node(
        &mut self,
        row_id: &str,
        concept: NodeConcept,
        summary: &str,
        score: Option<u8>,
        polarity: Option<&str>,
    ) -> String {
        let provenance = self.provenance(row_id, "interaction_row");

        let mut tags = vec![];
        if let Some(s) = polarity {
            tags.push(s.to_string());
        }

        let stable_key = row_id.replace("matrix_row:", "");
        let stable_key = stable_key.replace("row:", "");
        let id = SemanticId::new("row", stable_key);
        let node_id = id.to_node_id();
        let mut node = SemanticNode::new(id, concept, NodeOrigin::Interpreted, summary)
            .with_tags(tags)
            .with_provenance(provenance);

        if let Some(s) = score {
            node.tags.push(format!("score:{}", s));
        }

        self.graph.add_node(node);
        node_id
    }

    pub fn add_has_row_edge(&mut self, matrix_node_id: &str, row_node_id: &str) {
        let edge = SemanticEdge::new(matrix_node_id, row_node_id, EdgeConcept::HasRow);
        self.graph.add_edge(edge);
    }

    pub fn add_ten_god_relation_node(
        &mut self,
        _matrix_id: &str,
        row_node_id: &str,
        pillar_key: &str,
        label: &str,
        relation: FiveElementRelation,
        same_polarity: bool,
        evidence: &RuleEvidence,
        is_favorable: bool,
    ) {
        let stable_key = format!(
            "{}:{}:{}:{}",
            row_node_id, self.day_id, self.profile_id, pillar_key
        );
        let provenance = self.provenance(&stable_key, "ten_god_relation");

        let polarity = if is_favorable {
            "favorable"
        } else {
            "unfavorable"
        };
        let node = SemanticNode::new(
            SemanticId::new("ten_god", &stable_key),
            NodeConcept::TenGodRelation,
            NodeOrigin::Interpreted,
            format!("Thập Thần: {}", label),
        )
        .with_tags(vec![
            label.to_string(),
            polarity.to_string(),
            format!(
                "label:{}",
                thap_than_label_tag(parse_thap_than_label(label).unwrap_or(ThapThanLabel::TyKien))
            ),
            format!("relation:{}", five_element_relation_tag(relation)),
            format!("same_polarity:{}", same_polarity),
            format!("evidence_source:{}", evidence.source_id),
            format!("evidence_method:{}", evidence.method),
            format!("evidence_profile:{}", evidence.profile),
        ])
        .with_provenance(provenance);
        let node_id = node.node_id.clone();

        self.graph.add_node(node);

        let edge = SemanticEdge::new(row_node_id, &node_id, EdgeConcept::HasTenGodRelation);
        self.graph.add_edge(edge);
    }

    pub fn add_branch_relation_node(
        &mut self,
        _matrix_id: &str,
        row_node_id: &str,
        pillar_key: &str,
        branch_rel: &BranchRelation,
    ) {
        let stable_key = format!(
            "{}:{}:{}:{}",
            row_node_id, self.day_id, self.profile_id, pillar_key
        );
        let provenance = self.provenance(&stable_key, "branch_relation");

        let polarity = if branch_rel.has_conflict() {
            "conflict"
        } else if branch_rel.has_harmony() {
            "harmony"
        } else {
            "neutral"
        };

        let mut tags = vec![polarity.to_string()];
        if branch_rel.luc_xung {
            tags.push("luc_xung".to_string());
        }
        if branch_rel.luc_hop {
            tags.push("luc_hop".to_string());
        }
        if branch_rel.tam_hop {
            tags.push("tam_hop".to_string());
        }
        if branch_rel.tuong_hai {
            tags.push("tuong_hai".to_string());
        }
        if branch_rel.tuong_hinh {
            tags.push("tuong_hinh".to_string());
        }

        let summary = format!(
            "Xung: {} | Hợp: {} | Hại: {} | Hình: {}",
            branch_rel.luc_xung,
            branch_rel.luc_hop || branch_rel.tam_hop,
            branch_rel.tuong_hai,
            branch_rel.tuong_hinh
        );

        let node = SemanticNode::new(
            SemanticId::new("branch", &stable_key),
            NodeConcept::BranchRelationNode,
            NodeOrigin::Interpreted,
            summary,
        )
        .with_tags(tags)
        .with_provenance(provenance);
        let node_id = node.node_id.clone();

        self.graph.add_node(node);

        let edge = SemanticEdge::new(row_node_id, &node_id, EdgeConcept::HasBranchRelation);
        self.graph.add_edge(edge);
    }

    pub fn add_element_relation_node(
        &mut self,
        _matrix_id: &str,
        row_node_id: &str,
        pillar_key: &str,
        interaction: ElementInteraction,
    ) {
        let stable_key = format!(
            "{}:{}:{}:{}",
            row_node_id, self.day_id, self.profile_id, pillar_key
        );
        let provenance = self.provenance(&stable_key, "element_interaction");

        let (label, polarity) = match interaction {
            ElementInteraction::Same => ("same".to_string(), "neutral".to_string()),
            ElementInteraction::DayGeneratesPillar => {
                ("day_generates_pillar".to_string(), "favorable".to_string())
            }
            ElementInteraction::PillarGeneratesDay => (
                "pillar_generates_day".to_string(),
                "mild_favorable".to_string(),
            ),
            ElementInteraction::DayControlsPillar => {
                ("day_controls_pillar".to_string(), "challenging".to_string())
            }
            ElementInteraction::PillarControlsDay => {
                ("pillar_controls_day".to_string(), "challenging".to_string())
            }
        };

        let node = SemanticNode::new(
            SemanticId::new("element", &stable_key),
            NodeConcept::ElementRelationNode,
            NodeOrigin::Interpreted,
            format!("Element: {}", label),
        )
        .with_tags(vec![label.clone(), polarity, format!("relation:{}", label)])
        .with_provenance(provenance);
        let node_id = node.node_id.clone();

        self.graph.add_node(node);

        let edge = SemanticEdge::new(row_node_id, &node_id, EdgeConcept::HasElementRelation);
        self.graph.add_edge(edge);
    }

    pub fn add_direction_signal_node(
        &mut self,
        matrix_id: &str,
        direction: &str,
        row_node_id: &str,
        signal: DirectionSignal,
    ) {
        let signal_label = direction_signal_tag(signal);
        let stable_key = format!(
            "{}:{}:{}:{}:{}",
            matrix_id, self.day_id, self.profile_id, direction, signal_label
        );
        let provenance = self.provenance(&stable_key, "direction_signal");

        let polarity = if signal.is_favorable() {
            "favorable"
        } else {
            "unfavorable"
        };

        let node = SemanticNode::new(
            SemanticId::new("direction_signal", &stable_key),
            NodeConcept::DirectionSignalNode,
            NodeOrigin::Interpreted,
            format!("Signal: {}", signal_label),
        )
        .with_tags(vec![
            signal_label.to_string(),
            polarity.to_string(),
            format!("signal:{}", signal_label),
        ])
        .with_provenance(provenance);
        let node_id = node.node_id.clone();

        self.graph.add_node(node);

        let edge = SemanticEdge::new(row_node_id, &node_id, EdgeConcept::RelatesTo);
        self.graph.add_edge(edge);
    }
}

impl NodeConcept {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            NodeConcept::DayPersonMatrix => Some("day_person_matrix"),
            NodeConcept::PersonalHourMatrix => Some("personal_hour_matrix"),
            NodeConcept::ElementResonanceMatrix => Some("element_resonance_matrix"),
            NodeConcept::DirectionMergeMatrix => Some("direction_merge_matrix"),
            NodeConcept::DomainDayBoostMatrix => Some("domain_day_boost_matrix"),
            _ => None,
        }
    }
}

pub fn build_day_person_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &DayPersonMatrix,
) -> Result<SemanticGraph, String> {
    let matrix_kind = "day_person";
    let matrix_id = format!("{}:{}:{}", matrix_kind, day_id, profile_id);

    let mut builder = InteractionGraphBuilder::new(day_id, profile_id);
    let matrix_node_id = builder.add_matrix_root_with_tags(
        &matrix_id,
        NodeConcept::DayPersonMatrix,
        &format!(
            "Day-Person Matrix: {} vs {}",
            matrix.day_canchi, matrix.day_master
        ),
        vec![
            format!("day_canchi:{}", matrix.day_canchi),
            format!("day_master:{}", matrix.day_master),
            format!(
                "day_to_day_master_label:{}",
                thap_than_label_tag(matrix.day_to_day_master.label)
            ),
            format!(
                "day_to_day_master_relation:{}",
                five_element_relation_tag(matrix.day_to_day_master.relation)
            ),
            format!(
                "day_to_day_master_same_polarity:{}",
                matrix.day_to_day_master.same_polarity
            ),
            format!(
                "day_to_day_master_evidence_source:{}",
                matrix.day_to_day_master.evidence.source_id
            ),
            format!(
                "day_to_day_master_evidence_method:{}",
                matrix.day_to_day_master.evidence.method
            ),
            format!(
                "day_to_day_master_evidence_profile:{}",
                matrix.day_to_day_master.evidence.profile
            ),
            format!("evidence_source:{}", matrix.evidence.source_id),
            format!("evidence_method:{}", matrix.evidence.method),
            format!("evidence_profile:{}", matrix.evidence.profile),
        ],
    );

    for pillar in &matrix.pillars {
        let pillar_key = match pillar.pillar {
            crate::bazi::types::PillarKind::Year => "year",
            crate::bazi::types::PillarKind::Month => "month",
            crate::bazi::types::PillarKind::Day => "day",
            crate::bazi::types::PillarKind::Hour => "hour",
        };

        let row_id = format!("{}:{}:{}:{}", matrix_kind, day_id, profile_id, pillar_key);
        let polarity = polarity_for_pillar(&pillar.branch_relation, &pillar.element_interaction);
        let row_node_id = builder.add_row_node(
            &row_id,
            NodeConcept::InteractionRow,
            &format!("{} pillar: {}", pillar_key, pillar.pillar_canchi),
            None,
            Some(polarity),
        );
        builder.add_has_row_edge(&matrix_node_id, &row_node_id);

        builder.add_ten_god_relation_node(
            &matrix_id,
            &row_node_id,
            pillar_key,
            &format!("{:?}", pillar.thap_than.label),
            pillar.thap_than.relation,
            pillar.thap_than.same_polarity,
            &pillar.thap_than.evidence,
            thap_than_is_favorable(pillar.thap_than.label),
        );

        builder.add_branch_relation_node(
            &matrix_id,
            &row_node_id,
            pillar_key,
            &pillar.branch_relation,
        );

        builder.add_element_relation_node(
            &matrix_id,
            &row_node_id,
            pillar_key,
            pillar.element_interaction,
        );
    }

    Ok(builder.build())
}

pub fn build_personal_hour_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &PersonalHourMatrix,
) -> Result<SemanticGraph, String> {
    let matrix_kind = "personal_hour";
    let matrix_id = format!("{}:{}:{}", matrix_kind, day_id, profile_id);

    let mut builder = InteractionGraphBuilder::new(day_id, profile_id);
    let matrix_node_id = builder.add_matrix_root_with_tags(
        &matrix_id,
        NodeConcept::PersonalHourMatrix,
        &format!(
            "Personal Hour Matrix: {} for {}",
            matrix.day_canchi, matrix.day_master
        ),
        vec![
            format!("day_canchi:{}", matrix.day_canchi),
            format!("day_master:{}", matrix.day_master),
            format!("birth_hour_chi:{}", matrix.birth_hour_chi),
            format!("weak_element:{}", five_element_tag(matrix.weak_element)),
            format!("evidence_source:{}", matrix.evidence.source_id),
            format!("evidence_method:{}", matrix.evidence.method),
            format!("evidence_profile:{}", matrix.evidence.profile),
        ],
    );

    for (slot, hour) in matrix.hours.iter().enumerate() {
        let slot_key = slot.to_string();
        let row_id = format!("{}:{}:{}:{}", matrix_kind, day_id, profile_id, slot_key);
        let polarity = if hour.is_hoang_dao {
            "favorable"
        } else {
            "unfavorable"
        };
        let row_node_id = builder.add_row_node(
            &row_id,
            NodeConcept::HourSlot,
            &format!("{} {} ({})", hour.chi, hour.canchi, hour.time_range),
            Some(hour.score),
            Some(polarity),
        );
        builder.extend_node_tags(
            &row_node_id,
            vec![
                format!("chi_index:{}", hour.chi_index),
                format!("chi:{}", hour.chi),
                format!("canchi:{}", hour.canchi),
                format!("time_range:{}", hour.time_range),
                format!("is_hoang_dao:{}", hour.is_hoang_dao),
                format!("star_name:{}", hour.star_name),
                format!("supports_weak_element:{}", hour.supports_weak_element),
            ],
        );
        builder.add_has_row_edge(&matrix_node_id, &row_node_id);

        builder.add_ten_god_relation_node(
            &matrix_id,
            &row_node_id,
            &slot_key,
            &format!("{:?}", hour.thap_than_to_day_master.label),
            hour.thap_than_to_day_master.relation,
            hour.thap_than_to_day_master.same_polarity,
            &hour.thap_than_to_day_master.evidence,
            thap_than_is_favorable(hour.thap_than_to_day_master.label),
        );

        builder.add_branch_relation_node(
            &matrix_id,
            &row_node_id,
            &slot_key,
            &hour.branch_relation_to_birth_hour,
        );

        builder.add_element_relation_node(
            &matrix_id,
            &row_node_id,
            &slot_key,
            hour.element_interaction,
        );
    }

    Ok(builder.build())
}

pub fn build_direction_merge_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &DirectionMergeMatrix,
) -> Result<SemanticGraph, String> {
    let matrix_kind = "direction_merge";
    let matrix_id = format!("{}:{}:{}", matrix_kind, day_id, profile_id);

    let mut builder = InteractionGraphBuilder::new(day_id, profile_id);
    let matrix_node_id = builder.add_matrix_root_with_tags(
        &matrix_id,
        NodeConcept::DirectionMergeMatrix,
        &format!(
            "Direction Merge: {} kua={}",
            matrix.day_canchi, matrix.kua_number
        ),
        vec![
            format!("day_canchi:{}", matrix.day_canchi),
            format!("kua_number:{}", matrix.kua_number),
            format!("evidence_source:{}", matrix.evidence.source_id),
            format!("evidence_method:{}", matrix.evidence.method),
            format!("evidence_profile:{}", matrix.evidence.profile),
        ],
    );

    for entry in &matrix.entries {
        let row_id = format!(
            "{}:{}:{}:{}",
            matrix_kind, day_id, profile_id, entry.direction
        );
        let polarity = if entry.net_score > 0 {
            "favorable"
        } else if entry.net_score < 0 {
            "unfavorable"
        } else {
            "neutral"
        };
        let row_node_id = builder.add_row_node(
            &row_id,
            NodeConcept::InteractionRow,
            &format!("{} direction: net={}", entry.direction, entry.net_score),
            None,
            Some(polarity),
        );
        builder.extend_node_tags(
            &row_node_id,
            vec![
                format!("direction:{}", entry.direction),
                format!("favorable_count:{}", entry.favorable_count),
                format!("unfavorable_count:{}", entry.unfavorable_count),
                format!("net_score:{}", entry.net_score),
            ],
        );
        builder.add_has_row_edge(&matrix_node_id, &row_node_id);

        for signal in &entry.signals {
            builder.add_direction_signal_node(&matrix_id, &entry.direction, &row_node_id, *signal);
        }
    }

    Ok(builder.build())
}

fn polarity_for_pillar<'a>(
    branch_rel: &BranchRelation,
    elem_interaction: &ElementInteraction,
) -> &'a str {
    if branch_rel.has_conflict() {
        return "conflict";
    }
    if branch_rel.has_harmony() {
        return "harmony";
    }
    match elem_interaction {
        ElementInteraction::DayGeneratesPillar => "favorable",
        ElementInteraction::PillarGeneratesDay => "mild_favorable",
        ElementInteraction::DayControlsPillar | ElementInteraction::PillarControlsDay => {
            "challenging"
        }
        ElementInteraction::Same => "neutral",
    }
}

fn thap_than_is_favorable(label: crate::almanac::types::ThapThanLabel) -> bool {
    use crate::almanac::types::ThapThanLabel::*;
    matches!(
        label,
        ChinhAn | ThienAn | TyKien | KiepTai | ThucThan | ChinhQuan
    )
}

fn parse_thap_than_label(value: &str) -> Result<ThapThanLabel, String> {
    match value {
        "ty_kien" | "TyKien" => Ok(ThapThanLabel::TyKien),
        "kiep_tai" | "KiepTai" => Ok(ThapThanLabel::KiepTai),
        "thuc_than" | "ThucThan" => Ok(ThapThanLabel::ThucThan),
        "thuong_quan" | "ThuongQuan" => Ok(ThapThanLabel::ThuongQuan),
        "chinh_tai" | "ChinhTai" => Ok(ThapThanLabel::ChinhTai),
        "thien_tai" | "ThienTai" => Ok(ThapThanLabel::ThienTai),
        "chinh_quan" | "ChinhQuan" => Ok(ThapThanLabel::ChinhQuan),
        "that_sat" | "ThatSat" => Ok(ThapThanLabel::ThatSat),
        "chinh_an" | "ChinhAn" => Ok(ThapThanLabel::ChinhAn),
        "thien_an" | "ThienAn" => Ok(ThapThanLabel::ThienAn),
        other => Err(format!("invalid thap than label: {other}")),
    }
}

fn five_element_tag(value: FiveElement) -> &'static str {
    match value {
        FiveElement::Moc => "moc",
        FiveElement::Hoa => "hoa",
        FiveElement::Tho => "tho",
        FiveElement::Kim => "kim",
        FiveElement::Thuy => "thuy",
    }
}

fn five_element_relation_tag(value: FiveElementRelation) -> &'static str {
    match value {
        FiveElementRelation::Same => "same",
        FiveElementRelation::DayGeneratesTarget => "day_generates_target",
        FiveElementRelation::TargetGeneratesDay => "target_generates_day",
        FiveElementRelation::DayControlsTarget => "day_controls_target",
        FiveElementRelation::TargetControlsDay => "target_controls_day",
    }
}

fn thap_than_label_tag(value: ThapThanLabel) -> &'static str {
    match value {
        ThapThanLabel::TyKien => "ty_kien",
        ThapThanLabel::KiepTai => "kiep_tai",
        ThapThanLabel::ThucThan => "thuc_than",
        ThapThanLabel::ThuongQuan => "thuong_quan",
        ThapThanLabel::ChinhTai => "chinh_tai",
        ThapThanLabel::ThienTai => "thien_tai",
        ThapThanLabel::ChinhQuan => "chinh_quan",
        ThapThanLabel::ThatSat => "that_sat",
        ThapThanLabel::ChinhAn => "chinh_an",
        ThapThanLabel::ThienAn => "thien_an",
    }
}

fn direction_signal_tag(signal: DirectionSignal) -> &'static str {
    match signal {
        DirectionSignal::KuaFavorable => "kua_favorable",
        DirectionSignal::KuaUnfavorable => "kua_unfavorable",
        DirectionSignal::TaiThan => "tai_than",
        DirectionSignal::HyThan => "hy_than",
        DirectionSignal::PhucThan => "phuc_than",
        DirectionSignal::SatPhuong => "sat_phuong",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::tu_menh::{ConventionMetadata, Direction, KuaGroup, KuaResult};
    use crate::bazi::analysis::ElementDistribution;
    use crate::bazi::types::{BaziChart, BaziChartMetadata, BaziInput, BaziPillar, PillarKind};
    use crate::interaction::day_person::compute_day_person_matrix;
    use crate::interaction::direction_merge::compute_direction_merge;
    use crate::interaction::personal_hour::compute_personal_hour_matrix;
    use crate::types::CanChi;

    fn make_pillar(kind: PillarKind, can_index: usize, chi_index: usize) -> BaziPillar {
        BaziPillar {
            kind,
            can_chi: CanChi::new(can_index, chi_index),
            hidden_stems: vec![],
            na_am: None,
            stem_relation_to_day_master: None,
        }
    }

    fn make_chart(
        year: (usize, usize),
        month: (usize, usize),
        day: (usize, usize),
        hour: (usize, usize),
    ) -> BaziChart {
        let year_pillar = make_pillar(PillarKind::Year, year.0, year.1);
        let month_pillar = make_pillar(PillarKind::Month, month.0, month.1);
        let day_pillar = make_pillar(PillarKind::Day, day.0, day.1);
        let hour_pillar = make_pillar(PillarKind::Hour, hour.0, hour.1);
        let day_master = day_pillar.can_chi.clone();
        let pillars = vec![
            year_pillar.clone(),
            month_pillar.clone(),
            day_pillar.clone(),
            hour_pillar.clone(),
        ];
        BaziChart {
            input: BaziInput {
                day: 1,
                month: 1,
                year: 2000,
                hour: 0,
                minute: 0,
                time_known: false,
                timezone: 7.0,
                longitude: None,
                use_solar_time: false,
                gender: None,
            },
            lunar_date: crate::lunar::LunarDate {
                day: 1,
                month: 1,
                year: 2000,
                is_leap: false,
            },
            year_pillar,
            month_pillar,
            day_pillar,
            hour_pillar: Some(hour_pillar),
            day_master,
            pillars,
            metadata: BaziChartMetadata {
                timezone: 7.0,
                use_solar_time: false,
                year_basis: "test".to_string(),
                month_basis: "test".to_string(),
                day_basis: "test".to_string(),
                hour_basis: "test".to_string(),
                hour_evidence: None,
            },
        }
    }

    fn balanced_dist() -> ElementDistribution {
        ElementDistribution {
            moc: 20,
            hoa: 20,
            tho: 20,
            kim: 20,
            thuy: 20,
        }
    }

    fn make_kua() -> KuaResult {
        KuaResult::new(
            1,
            KuaGroup::East,
            [
                Direction::Southeast,
                Direction::East,
                Direction::South,
                Direction::North,
            ],
            [
                Direction::West,
                Direction::Northwest,
                Direction::Southwest,
                Direction::Northeast,
            ],
            ConventionMetadata {
                year_basis: "test".to_string(),
                kua5_resolution: "test".to_string(),
                gender_encoding: "test".to_string(),
            },
        )
    }

    #[test]
    fn day_person_matrix_graph_has_root() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        let graph = build_day_person_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let matrix_id = "day_person:2024-01-01:test-profile";
        assert!(graph.has_node(matrix_id), "should have matrix root node");
    }

    #[test]
    fn day_person_matrix_graph_has_4_rows() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        let graph = build_day_person_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let row_ids = vec![
            "row:day_person:2024-01-01:test-profile:year",
            "row:day_person:2024-01-01:test-profile:month",
            "row:day_person:2024-01-01:test-profile:day",
            "row:day_person:2024-01-01:test-profile:hour",
        ];
        for row_id in row_ids {
            assert!(graph.has_node(row_id), "should have row node: {}", row_id);
        }
    }

    #[test]
    fn day_person_matrix_graph_has_ten_god_nodes() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        let graph = build_day_person_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        assert!(
            graph.node_count() > 5,
            "should have root + 4 rows + relation nodes"
        );
    }

    #[test]
    fn personal_hour_matrix_graph_has_12_rows() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let graph =
            build_personal_hour_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        for slot in 0..12 {
            let row_id = format!("row:personal_hour:2024-01-01:test-profile:{}", slot);
            assert!(
                graph.has_node(&row_id),
                "should have hour slot row: {}",
                row_id
            );
        }
    }

    #[test]
    fn direction_merge_matrix_graph_has_8_rows() {
        let day = CanChi::new(0, 0);
        let matrix = compute_direction_merge(&day, "Tây Nam", "Đông Bắc", &make_kua());
        let graph =
            build_direction_merge_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let directions = vec![
            "Bắc",
            "Đông Bắc",
            "Đông",
            "Đông Nam",
            "Nam",
            "Tây Nam",
            "Tây",
            "Tây Bắc",
        ];
        for dir in directions {
            let row_id = format!("row:direction_merge:2024-01-01:test-profile:{}", dir);
            assert!(
                graph.has_node(&row_id),
                "should have direction row: {}",
                row_id
            );
        }
    }

    #[test]
    fn all_matrix_graphs_build_successfully() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));

        let dpm = compute_day_person_matrix(&day, &chart);
        let dpm_graph = build_day_person_matrix_graph("2024-01-01", "test", &dpm).unwrap();
        assert!(
            dpm_graph.node_count() > 4,
            "day person matrix graph should have root + rows + relations"
        );
        assert!(
            dpm_graph.edge_count() > 0,
            "day person matrix graph should have edges"
        );

        let phm = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let phm_graph = build_personal_hour_matrix_graph("2024-01-01", "test", &phm).unwrap();
        assert!(
            phm_graph.node_count() >= 13,
            "personal hour matrix graph should have root + 12 hour rows"
        );
        assert!(
            phm_graph.edge_count() > 0,
            "personal hour matrix graph should have edges"
        );

        let dmm = compute_direction_merge(&day, "Tây Nam", "Đông Bắc", &make_kua());
        let dmm_graph = build_direction_merge_matrix_graph("2024-01-01", "test", &dmm).unwrap();
        assert!(
            dmm_graph.node_count() >= 9,
            "direction merge graph should have root + 8 direction rows"
        );
    }
}
