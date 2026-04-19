use crate::almanac::types::{
    FiveElement, FiveElementRelation, RuleEvidence, ThapThanLabel, ThapThanResult,
};
use crate::bazi::types::PillarKind;
use crate::interaction::types::{
    BranchRelation, DayPersonMatrix, DirectionEntry, DirectionMergeMatrix, DirectionSignal,
    DomainDayBoostEntry, DomainDayBoostMatrix, ElementInteraction, ElementResonanceEntry,
    ElementResonanceMatrix, PersonalHourEntry, PersonalHourMatrix, PillarInteraction,
};
use crate::semantic_graph::{
    EdgeConcept, NodeConcept, NodeOrigin, ProvenanceEntry, SemanticEdge, SemanticGraph, SemanticId,
    SemanticNode,
};

pub struct InteractionGraphBuilder {
    graph: SemanticGraph,
    day_id: String,
    profile_id: String,
    ruleset_id: String,
    ruleset_version: String,
    profile: String,
}

impl InteractionGraphBuilder {
    pub fn new(day_id: &str, profile_id: &str) -> Self {
        Self {
            graph: SemanticGraph::new(),
            day_id: day_id.to_string(),
            profile_id: profile_id.to_string(),
            ruleset_id: "baseline".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
        }
    }

    pub fn with_ruleset(mut self, ruleset_id: &str, ruleset_version: &str, profile: &str) -> Self {
        self.ruleset_id = ruleset_id.to_string();
        self.ruleset_version = ruleset_version.to_string();
        self.profile = profile.to_string();
        self
    }

    fn provenance(&self, source_id: &str, method: &str) -> ProvenanceEntry {
        ProvenanceEntry::interaction(source_id, method).with_profile(self.profile.clone())
    }

    pub fn graph(&self) -> &SemanticGraph {
        &self.graph
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

pub fn build_element_resonance_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &ElementResonanceMatrix,
) -> Result<SemanticGraph, String> {
    let matrix_kind = "element_resonance";
    let matrix_id = format!("{}:{}:{}", matrix_kind, day_id, profile_id);

    let mut builder = InteractionGraphBuilder::new(day_id, profile_id);
    let matrix_node_id = builder.add_matrix_root_with_tags(
        &matrix_id,
        NodeConcept::ElementResonanceMatrix,
        &format!(
            "Element Resonance: {} in month {}",
            matrix.day_canchi, matrix.month_chi
        ),
        vec![
            format!("day_canchi:{}", matrix.day_canchi),
            format!("day_element:{}", five_element_tag(matrix.day_element)),
            format!("month_chi:{}", matrix.month_chi),
            format!("season_factor:{}", matrix.season_factor),
            format!("net_resonance:{}", matrix.net_resonance),
            format!("evidence_source:{}", matrix.evidence.source_id),
            format!("evidence_method:{}", matrix.evidence.method),
            format!("evidence_profile:{}", matrix.evidence.profile),
        ],
    );

    for entry in &matrix.entries {
        let elem_key = format!("{:?}", entry.element).to_lowercase();
        let row_id = format!("{}:{}:{}:{}", matrix_kind, day_id, profile_id, elem_key);
        let polarity = if entry.effective_resonance > 0.0 {
            "supportive"
        } else if entry.effective_resonance < 0.0 {
            "depleting"
        } else {
            "neutral"
        };
        let row_node_id = builder.add_row_node(
            &row_id,
            NodeConcept::InteractionRow,
            &format!(
                "{} element: score={} resonance={:.2}",
                elem_key, entry.personal_score, entry.effective_resonance
            ),
            None,
            Some(polarity),
        );
        builder.extend_node_tags(
            &row_node_id,
            vec![
                format!("element:{}", five_element_tag(entry.element)),
                format!("personal_score:{}", entry.personal_score),
                format!("relation_to_day:{}", entry.relation_to_day),
                format!("season_factor:{}", entry.season_factor),
                format!("effective_resonance:{}", entry.effective_resonance),
                format!("is_deficit:{}", entry.is_deficit),
                format!("day_helps_deficit:{}", entry.day_helps_deficit),
            ],
        );
        builder.add_has_row_edge(&matrix_node_id, &row_node_id);
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

pub fn build_domain_day_boost_matrix_graph(
    day_id: &str,
    profile_id: &str,
    matrix: &DomainDayBoostMatrix,
) -> Result<SemanticGraph, String> {
    let matrix_kind = "domain_day_boost";
    let matrix_id = format!("{}:{}:{}", matrix_kind, day_id, profile_id);

    let mut builder = InteractionGraphBuilder::new(day_id, profile_id);
    let matrix_node_id = builder.add_matrix_root_with_tags(
        &matrix_id,
        NodeConcept::DomainDayBoostMatrix,
        &format!("Domain-Day Boost: {}", matrix.day_canchi),
        vec![
            format!("day_canchi:{}", matrix.day_canchi),
            format!("evidence_source:{}", matrix.evidence.source_id),
            format!("evidence_method:{}", matrix.evidence.method),
            format!("evidence_profile:{}", matrix.evidence.profile),
        ],
    );

    for entry in &matrix.entries {
        let row_id = format!("{}:{}:{}:{}", matrix_kind, day_id, profile_id, entry.domain);
        let polarity = if entry.boosted_score > entry.base_score {
            "boosted"
        } else if entry.boosted_score < entry.base_score {
            "reduced"
        } else {
            "unchanged"
        };
        let row_node_id = builder.add_row_node(
            &row_id,
            NodeConcept::InteractionRow,
            &format!(
                "{} domain: {} → {}",
                entry.domain, entry.base_score, entry.boosted_score
            ),
            None,
            Some(polarity),
        );
        builder.extend_node_tags(
            &row_node_id,
            vec![
                format!("domain:{}", entry.domain),
                format!("base_score:{}", entry.base_score),
                format!("day_modifier:{}", entry.day_modifier),
                format!("han_penalty:{}", entry.han_penalty),
                format!("boosted_score:{}", entry.boosted_score),
            ],
        );
        builder.add_has_row_edge(&matrix_node_id, &row_node_id);
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

pub fn project_day_person_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<DayPersonMatrix, String> {
    let root = matrix_root_node(graph, matrix_id)?;
    let evidence = rule_evidence_from_tags(root)?;
    let day_to_day_master = thap_than_from_matrix_root(root, "day_to_day_master")?;

    let mut pillars: Vec<PillarInteraction> = row_nodes(graph, &root.node_id)?
        .into_iter()
        .map(|row| project_day_person_pillar(graph, row))
        .collect::<Result<Vec<_>, _>>()?;
    pillars.sort_by_key(|pillar| pillar_order(pillar.pillar));

    Ok(DayPersonMatrix {
        day_canchi: required_tag(root, "day_canchi")?.to_string(),
        day_master: required_tag(root, "day_master")?.to_string(),
        day_to_day_master,
        pillars,
        evidence,
    })
}

pub fn project_personal_hour_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<PersonalHourMatrix, String> {
    let root = matrix_root_node(graph, matrix_id)?;
    let evidence = rule_evidence_from_tags(root)?;

    let mut hours: Vec<(usize, PersonalHourEntry)> = row_nodes(graph, &root.node_id)?
        .into_iter()
        .map(|row| -> Result<(usize, PersonalHourEntry), String> {
            let slot = row_key(row)?.parse::<usize>().map_err(|err| {
                format!("invalid personal hour row key on {}: {err}", row.node_id)
            })?;
            Ok((slot, project_personal_hour_entry(graph, row)?))
        })
        .collect::<Result<Vec<_>, _>>()?;
    hours.sort_by_key(|(slot, _)| *slot);

    Ok(PersonalHourMatrix {
        day_canchi: required_tag(root, "day_canchi")?.to_string(),
        day_master: required_tag(root, "day_master")?.to_string(),
        birth_hour_chi: required_tag(root, "birth_hour_chi")?.to_string(),
        weak_element: parse_five_element(required_tag(root, "weak_element")?)?,
        hours: hours.into_iter().map(|(_, entry)| entry).collect(),
        evidence,
    })
}

pub fn project_element_resonance_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<ElementResonanceMatrix, String> {
    let root = matrix_root_node(graph, matrix_id)?;
    let evidence = rule_evidence_from_tags(root)?;

    let mut entries: Vec<ElementResonanceEntry> = row_nodes(graph, &root.node_id)?
        .into_iter()
        .map(project_element_resonance_entry)
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| five_element_order(entry.element));

    Ok(ElementResonanceMatrix {
        day_canchi: required_tag(root, "day_canchi")?.to_string(),
        day_element: parse_five_element(required_tag(root, "day_element")?)?,
        month_chi: required_tag(root, "month_chi")?.to_string(),
        season_factor: parse_f32_tag(root, "season_factor")?,
        entries,
        net_resonance: parse_f32_tag(root, "net_resonance")?,
        evidence,
    })
}

pub fn project_direction_merge_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<DirectionMergeMatrix, String> {
    let root = matrix_root_node(graph, matrix_id)?;
    let evidence = rule_evidence_from_tags(root)?;

    let mut entries: Vec<DirectionEntry> = row_nodes(graph, &root.node_id)?
        .into_iter()
        .map(|row| project_direction_entry(graph, row))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| direction_order(&entry.direction));

    Ok(DirectionMergeMatrix {
        day_canchi: required_tag(root, "day_canchi")?.to_string(),
        kua_number: parse_u8_tag(root, "kua_number")?,
        entries,
        evidence,
    })
}

pub fn project_domain_day_boost_matrix(
    graph: &SemanticGraph,
    matrix_id: &str,
) -> Result<DomainDayBoostMatrix, String> {
    let root = matrix_root_node(graph, matrix_id)?;
    let evidence = rule_evidence_from_tags(root)?;

    let mut entries: Vec<DomainDayBoostEntry> = row_nodes(graph, &root.node_id)?
        .into_iter()
        .map(project_domain_day_boost_entry)
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| domain_order(&entry.domain));

    Ok(DomainDayBoostMatrix {
        day_canchi: required_tag(root, "day_canchi")?.to_string(),
        entries,
        evidence,
    })
}

fn matrix_root_node<'a>(
    graph: &'a SemanticGraph,
    matrix_id: &str,
) -> Result<&'a SemanticNode, String> {
    let normalized = matrix_id.strip_prefix("matrix:").unwrap_or(matrix_id);
    graph
        .get_node(normalized)
        .or_else(|| graph.get_node(matrix_id))
        .ok_or_else(|| format!("matrix root not found: {matrix_id}"))
}

fn row_nodes<'a>(graph: &'a SemanticGraph, root_id: &str) -> Result<Vec<&'a SemanticNode>, String> {
    let mut rows = Vec::new();
    for edge in graph.outgoing_edges(root_id) {
        if edge.label.concept != EdgeConcept::HasRow {
            continue;
        }
        let row = graph
            .get_node(&edge.to_node_id)
            .ok_or_else(|| format!("missing row node: {}", edge.to_node_id))?;
        rows.push(row);
    }
    Ok(rows)
}

fn project_day_person_pillar(
    graph: &SemanticGraph,
    row: &SemanticNode,
) -> Result<PillarInteraction, String> {
    let pillar = parse_pillar_kind(row_key(row)?)?;
    let thap_than = thap_than_from_relation_node(graph, row, EdgeConcept::HasTenGodRelation)?;
    let branch_relation = branch_relation_from_node(relation_child_node(
        graph,
        row,
        EdgeConcept::HasBranchRelation,
    )?)?;
    let element_interaction = element_interaction_from_node(relation_child_node(
        graph,
        row,
        EdgeConcept::HasElementRelation,
    )?)?;

    Ok(PillarInteraction {
        pillar,
        pillar_canchi: row
            .summary_vi
            .split_once(": ")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| {
                format!(
                    "unable to parse pillar canchi from summary: {}",
                    row.summary_vi
                )
            })?,
        thap_than,
        branch_relation,
        element_interaction,
    })
}

fn project_personal_hour_entry(
    graph: &SemanticGraph,
    row: &SemanticNode,
) -> Result<PersonalHourEntry, String> {
    Ok(PersonalHourEntry {
        chi_index: parse_usize_tag(row, "chi_index")?,
        chi: required_tag(row, "chi")?.to_string(),
        canchi: required_tag(row, "canchi")?.to_string(),
        time_range: required_tag(row, "time_range")?.to_string(),
        is_hoang_dao: parse_bool_tag(row, "is_hoang_dao")?,
        star_name: required_tag(row, "star_name")?.to_string(),
        thap_than_to_day_master: thap_than_from_relation_node(
            graph,
            row,
            EdgeConcept::HasTenGodRelation,
        )?,
        branch_relation_to_birth_hour: branch_relation_from_node(relation_child_node(
            graph,
            row,
            EdgeConcept::HasBranchRelation,
        )?)?,
        element_interaction: element_interaction_from_node(relation_child_node(
            graph,
            row,
            EdgeConcept::HasElementRelation,
        )?)?,
        supports_weak_element: parse_bool_tag(row, "supports_weak_element")?,
        score: parse_u8_tag(row, "score")?,
    })
}

fn project_element_resonance_entry(row: &SemanticNode) -> Result<ElementResonanceEntry, String> {
    Ok(ElementResonanceEntry {
        element: parse_five_element(required_tag(row, "element")?)?,
        personal_score: parse_u16_tag(row, "personal_score")?,
        relation_to_day: parse_f32_tag(row, "relation_to_day")?,
        season_factor: parse_f32_tag(row, "season_factor")?,
        effective_resonance: parse_f32_tag(row, "effective_resonance")?,
        is_deficit: parse_bool_tag(row, "is_deficit")?,
        day_helps_deficit: parse_bool_tag(row, "day_helps_deficit")?,
    })
}

fn project_direction_entry(
    graph: &SemanticGraph,
    row: &SemanticNode,
) -> Result<DirectionEntry, String> {
    let mut signals: Vec<DirectionSignal> = graph
        .outgoing_edges(&row.node_id)
        .into_iter()
        .filter(|edge| edge.label.concept == EdgeConcept::RelatesTo)
        .map(|edge| {
            let node = graph
                .get_node(&edge.to_node_id)
                .ok_or_else(|| format!("missing direction signal node: {}", edge.to_node_id))?;
            parse_direction_signal(required_tag(node, "signal")?)
        })
        .collect::<Result<Vec<_>, _>>()?;
    signals.sort_by_key(|signal| direction_signal_order(*signal));

    Ok(DirectionEntry {
        direction: required_tag(row, "direction")?.to_string(),
        signals,
        favorable_count: parse_i8_tag(row, "favorable_count")?,
        unfavorable_count: parse_i8_tag(row, "unfavorable_count")?,
        net_score: parse_i8_tag(row, "net_score")?,
    })
}

fn project_domain_day_boost_entry(row: &SemanticNode) -> Result<DomainDayBoostEntry, String> {
    Ok(DomainDayBoostEntry {
        domain: required_tag(row, "domain")?.to_string(),
        base_score: parse_f32_tag(row, "base_score")?,
        day_modifier: parse_f32_tag(row, "day_modifier")?,
        han_penalty: parse_f32_tag(row, "han_penalty")?,
        boosted_score: parse_f32_tag(row, "boosted_score")?,
    })
}

fn relation_child_node<'a>(
    graph: &'a SemanticGraph,
    row: &SemanticNode,
    concept: EdgeConcept,
) -> Result<&'a SemanticNode, String> {
    let edge = graph
        .outgoing_edges(&row.node_id)
        .into_iter()
        .find(|edge| edge.label.concept == concept)
        .ok_or_else(|| {
            format!(
                "missing relation edge {:?} for row {}",
                concept, row.node_id
            )
        })?;
    graph
        .get_node(&edge.to_node_id)
        .ok_or_else(|| format!("missing relation node: {}", edge.to_node_id))
}

fn thap_than_from_relation_node(
    graph: &SemanticGraph,
    row: &SemanticNode,
    concept: EdgeConcept,
) -> Result<ThapThanResult, String> {
    let node = relation_child_node(graph, row, concept)?;
    Ok(ThapThanResult {
        label: parse_thap_than_label(required_tag(node, "label")?)?,
        relation: parse_five_element_relation(required_tag(node, "relation")?)?,
        same_polarity: parse_bool_tag(node, "same_polarity")?,
        evidence: rule_evidence_from_tags(node)?,
    })
}

fn thap_than_from_matrix_root(root: &SemanticNode, prefix: &str) -> Result<ThapThanResult, String> {
    Ok(ThapThanResult {
        label: parse_thap_than_label(required_tag(root, &format!("{prefix}_label"))?)?,
        relation: parse_five_element_relation(required_tag(root, &format!("{prefix}_relation"))?)?,
        same_polarity: parse_bool_tag(root, &format!("{prefix}_same_polarity"))?,
        evidence: RuleEvidence {
            source_id: required_tag(root, &format!("{prefix}_evidence_source"))?.to_string(),
            method: required_tag(root, &format!("{prefix}_evidence_method"))?.to_string(),
            profile: required_tag(root, &format!("{prefix}_evidence_profile"))?.to_string(),
        },
    })
}

fn branch_relation_from_node(node: &SemanticNode) -> Result<BranchRelation, String> {
    Ok(BranchRelation {
        luc_xung: has_tag(node, "luc_xung"),
        luc_hop: has_tag(node, "luc_hop"),
        tam_hop: has_tag(node, "tam_hop"),
        tuong_hai: has_tag(node, "tuong_hai"),
        tuong_hinh: has_tag(node, "tuong_hinh"),
    })
}

fn element_interaction_from_node(node: &SemanticNode) -> Result<ElementInteraction, String> {
    parse_element_interaction(required_tag(node, "relation")?)
}

fn rule_evidence_from_tags(node: &SemanticNode) -> Result<RuleEvidence, String> {
    Ok(RuleEvidence {
        source_id: required_tag(node, "evidence_source")?.to_string(),
        method: required_tag(node, "evidence_method")?.to_string(),
        profile: required_tag(node, "evidence_profile")?.to_string(),
    })
}

fn required_tag<'a>(node: &'a SemanticNode, key: &str) -> Result<&'a str, String> {
    node.tags
        .iter()
        .find_map(|tag| tag.strip_prefix(&format!("{key}:")))
        .ok_or_else(|| format!("missing tag {key} on {}", node.node_id))
}

fn has_tag(node: &SemanticNode, expected: &str) -> bool {
    node.tags.iter().any(|tag| tag == expected)
}

fn parse_bool_tag(node: &SemanticNode, key: &str) -> Result<bool, String> {
    required_tag(node, key)?
        .parse::<bool>()
        .map_err(|err| format!("invalid bool tag {key} on {}: {err}", node.node_id))
}

fn parse_u8_tag(node: &SemanticNode, key: &str) -> Result<u8, String> {
    required_tag(node, key)?
        .parse::<u8>()
        .map_err(|err| format!("invalid u8 tag {key} on {}: {err}", node.node_id))
}

fn parse_u16_tag(node: &SemanticNode, key: &str) -> Result<u16, String> {
    required_tag(node, key)?
        .parse::<u16>()
        .map_err(|err| format!("invalid u16 tag {key} on {}: {err}", node.node_id))
}

fn parse_usize_tag(node: &SemanticNode, key: &str) -> Result<usize, String> {
    required_tag(node, key)?
        .parse::<usize>()
        .map_err(|err| format!("invalid usize tag {key} on {}: {err}", node.node_id))
}

fn parse_i8_tag(node: &SemanticNode, key: &str) -> Result<i8, String> {
    required_tag(node, key)?
        .parse::<i8>()
        .map_err(|err| format!("invalid i8 tag {key} on {}: {err}", node.node_id))
}

fn parse_f32_tag(node: &SemanticNode, key: &str) -> Result<f32, String> {
    required_tag(node, key)?
        .parse::<f32>()
        .map_err(|err| format!("invalid f32 tag {key} on {}: {err}", node.node_id))
}

fn row_key(row: &SemanticNode) -> Result<&str, String> {
    row.id
        .stable_key
        .rsplit(':')
        .next()
        .ok_or_else(|| format!("invalid row stable key: {}", row.id.stable_key))
}

fn parse_pillar_kind(value: &str) -> Result<PillarKind, String> {
    match value {
        "year" => Ok(PillarKind::Year),
        "month" => Ok(PillarKind::Month),
        "day" => Ok(PillarKind::Day),
        "hour" => Ok(PillarKind::Hour),
        other => Err(format!("invalid pillar key: {other}")),
    }
}

fn parse_five_element(value: &str) -> Result<FiveElement, String> {
    match value {
        "moc" => Ok(FiveElement::Moc),
        "hoa" => Ok(FiveElement::Hoa),
        "tho" => Ok(FiveElement::Tho),
        "kim" => Ok(FiveElement::Kim),
        "thuy" => Ok(FiveElement::Thuy),
        other => Err(format!("invalid five element: {other}")),
    }
}

fn parse_five_element_relation(value: &str) -> Result<FiveElementRelation, String> {
    match value {
        "same" => Ok(FiveElementRelation::Same),
        "day_generates_target" => Ok(FiveElementRelation::DayGeneratesTarget),
        "target_generates_day" => Ok(FiveElementRelation::TargetGeneratesDay),
        "day_controls_target" => Ok(FiveElementRelation::DayControlsTarget),
        "target_controls_day" => Ok(FiveElementRelation::TargetControlsDay),
        other => Err(format!("invalid five element relation: {other}")),
    }
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

fn parse_element_interaction(value: &str) -> Result<ElementInteraction, String> {
    match value {
        "same" => Ok(ElementInteraction::Same),
        "day_generates_pillar" => Ok(ElementInteraction::DayGeneratesPillar),
        "pillar_generates_day" => Ok(ElementInteraction::PillarGeneratesDay),
        "day_controls_pillar" => Ok(ElementInteraction::DayControlsPillar),
        "pillar_controls_day" => Ok(ElementInteraction::PillarControlsDay),
        other => Err(format!("invalid element interaction: {other}")),
    }
}

fn parse_direction_signal(value: &str) -> Result<DirectionSignal, String> {
    match value {
        "kua_favorable" => Ok(DirectionSignal::KuaFavorable),
        "kua_unfavorable" => Ok(DirectionSignal::KuaUnfavorable),
        "tai_than" => Ok(DirectionSignal::TaiThan),
        "hy_than" => Ok(DirectionSignal::HyThan),
        "phuc_than" => Ok(DirectionSignal::PhucThan),
        "sat_phuong" => Ok(DirectionSignal::SatPhuong),
        other => Err(format!("invalid direction signal: {other}")),
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

fn direction_signal_order(signal: DirectionSignal) -> usize {
    match signal {
        DirectionSignal::KuaFavorable => 0,
        DirectionSignal::KuaUnfavorable => 1,
        DirectionSignal::TaiThan => 2,
        DirectionSignal::HyThan => 3,
        DirectionSignal::PhucThan => 4,
        DirectionSignal::SatPhuong => 5,
    }
}

fn pillar_order(pillar: PillarKind) -> usize {
    match pillar {
        PillarKind::Year => 0,
        PillarKind::Month => 1,
        PillarKind::Day => 2,
        PillarKind::Hour => 3,
    }
}

fn five_element_order(element: FiveElement) -> usize {
    match element {
        FiveElement::Moc => 0,
        FiveElement::Hoa => 1,
        FiveElement::Tho => 2,
        FiveElement::Kim => 3,
        FiveElement::Thuy => 4,
    }
}

fn direction_order(direction: &str) -> usize {
    match direction {
        "Bắc" => 0,
        "Đông Bắc" => 1,
        "Đông" => 2,
        "Đông Nam" => 3,
        "Nam" => 4,
        "Tây Nam" => 5,
        "Tây" => 6,
        "Tây Bắc" => 7,
        _ => usize::MAX,
    }
}

fn domain_order(domain: &str) -> usize {
    match domain {
        "career" => 0,
        "wealth" => 1,
        "relationship" => 2,
        "health" => 3,
        "timing" => 4,
        _ => usize::MAX,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::tu_menh::{ConventionMetadata, Direction, KuaGroup, KuaResult};
    use crate::almanac::types::{
        DayConflict, DayDeity, DayDeityClassification, DayElement, DayFortune, DayStars,
        TravelDirection, TrucInfo, XungHopResult,
    };
    use crate::bazi::analysis::ElementDistribution;
    use crate::bazi::scoring::BaziDomainScores;
    use crate::bazi::types::{BaziChart, BaziChartMetadata, BaziInput, BaziPillar, PillarKind};
    use crate::interaction::day_person::compute_day_person_matrix;
    use crate::interaction::direction_merge::compute_direction_merge;
    use crate::interaction::domain_day_boost::compute_domain_day_boost;
    use crate::interaction::element_resonance::compute_element_resonance;
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

    fn make_fortune() -> DayFortune {
        DayFortune {
            ruleset_id: "test".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            day_element: DayElement {
                na_am: "test".to_string(),
                element: "Kim".to_string(),
                can_element: "Mộc".to_string(),
                chi_element: "Thổ".to_string(),
                evidence: None,
            },
            conflict: DayConflict {
                opposing_chi: "Tuất".to_string(),
                opposing_con_giap: "Tuất (Chó)".to_string(),
                tuoi_xung: vec![],
                sat_huong: "Nam".to_string(),
                evidence: None,
            },
            travel: TravelDirection {
                xuat_hanh_huong: "Đông".to_string(),
                tai_than: "Tây Nam".to_string(),
                hy_than: "Đông Bắc".to_string(),
                evidence: None,
            },
            stars: DayStars {
                cat_tinh: vec!["Star1".to_string()],
                sat_tinh: vec![],
                day_star: None,
                star_system: None,
                evidence: None,
                matched_rules: vec![],
            },
            day_deity: Some(DayDeity {
                name: "Thanh Long".to_string(),
                classification: DayDeityClassification::HoangDao,
                evidence: None,
            }),
            taboos: vec![],
            xung_hop: XungHopResult {
                luc_xung: "Tuất".to_string(),
                tam_hop: vec![],
                tu_hanh_xung: vec![],
                liu_he: None,
                xiang_hai: None,
                xiang_xing: None,
            },
            truc: TrucInfo {
                index: 0,
                name: "Kiến".to_string(),
                quality: "cat".to_string(),
                evidence: None,
            },
            tang_can: None,
            ten_gods: None,
            tu_menh: None,
        }
    }

    fn make_domain_scores() -> BaziDomainScores {
        let score = |s: u8| crate::bazi::scoring::BaziDomainScore {
            score: s,
            label: "moderate".to_string(),
            confidence: 0.7,
            evidence_level: "baseline".to_string(),
            contributors: vec![],
        };
        BaziDomainScores {
            career: score(60),
            wealth: score(50),
            relationship: score(70),
            health: score(55),
            timing: score(45),
        }
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
    fn element_resonance_matrix_graph_has_5_rows() {
        let day = CanChi::new(0, 0);
        let matrix = compute_element_resonance(&day, "Dần", &balanced_dist());
        let graph =
            build_element_resonance_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let elements = vec!["moc", "hoa", "tho", "kim", "thuy"];
        for elem in elements {
            let row_id = format!("row:element_resonance:2024-01-01:test-profile:{}", elem);
            assert!(
                graph.has_node(&row_id),
                "should have element row: {}",
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
    fn domain_day_boost_matrix_graph_has_5_domains() {
        let fortune = make_fortune();
        let matrix = compute_domain_day_boost(&fortune, &make_domain_scores(), 0);
        let graph =
            build_domain_day_boost_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let domains = vec!["career", "wealth", "relationship", "health", "timing"];
        for domain in domains {
            let row_id = format!("row:domain_day_boost:2024-01-01:test-profile:{}", domain);
            assert!(
                graph.has_node(&row_id),
                "should have domain row: {}",
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

        let erm = compute_element_resonance(&day, "Dần", &balanced_dist());
        let erm_graph = build_element_resonance_matrix_graph("2024-01-01", "test", &erm).unwrap();
        assert!(
            erm_graph.node_count() >= 6,
            "element resonance graph should have root + 5 element rows"
        );

        let dmm = compute_direction_merge(&day, "Tây Nam", "Đông Bắc", &make_kua());
        let dmm_graph = build_direction_merge_matrix_graph("2024-01-01", "test", &dmm).unwrap();
        assert!(
            dmm_graph.node_count() >= 9,
            "direction merge graph should have root + 8 direction rows"
        );

        let ddb = compute_domain_day_boost(&make_fortune(), &make_domain_scores(), 0);
        let ddb_graph = build_domain_day_boost_matrix_graph("2024-01-01", "test", &ddb).unwrap();
        assert!(
            ddb_graph.node_count() >= 6,
            "domain day boost graph should have root + 5 domain rows"
        );
    }

    #[test]
    fn day_person_projection_round_trips_with_order_and_relations() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        let graph = build_day_person_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let projected =
            project_day_person_matrix(&graph, "day_person:2024-01-01:test-profile").unwrap();

        assert_eq!(projected, matrix);
        assert_eq!(
            projected
                .pillars
                .iter()
                .map(|pillar| pillar.pillar)
                .collect::<Vec<_>>(),
            vec![
                PillarKind::Year,
                PillarKind::Month,
                PillarKind::Day,
                PillarKind::Hour
            ]
        );
    }

    #[test]
    fn personal_hour_projection_round_trips_with_best_hour_unchanged() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let graph =
            build_personal_hour_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let projected =
            project_personal_hour_matrix(&graph, "personal_hour:2024-01-01:test-profile").unwrap();
        let best_original = matrix.hours.iter().max_by_key(|hour| hour.score).unwrap();
        let best_projected = projected
            .hours
            .iter()
            .max_by_key(|hour| hour.score)
            .unwrap();

        assert_eq!(projected, matrix);
        assert_eq!(best_projected.chi_index, best_original.chi_index);
        assert_eq!(best_projected.score, best_original.score);
        assert_eq!(
            projected
                .hours
                .iter()
                .map(|hour| hour.chi_index)
                .collect::<Vec<_>>(),
            matrix
                .hours
                .iter()
                .map(|hour| hour.chi_index)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn element_resonance_projection_round_trips_with_net_and_flags_preserved() {
        let day = CanChi::new(0, 2);
        let dist = ElementDistribution {
            moc: 8,
            hoa: 26,
            tho: 12,
            kim: 28,
            thuy: 10,
        };
        let matrix = compute_element_resonance(&day, "Dần", &dist);
        let graph =
            build_element_resonance_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let projected =
            project_element_resonance_matrix(&graph, "element_resonance:2024-01-01:test-profile")
                .unwrap();

        assert_eq!(projected, matrix);
        assert_eq!(projected.net_resonance, matrix.net_resonance);
        assert_eq!(
            projected
                .entries
                .iter()
                .filter(|entry| entry.is_deficit)
                .count(),
            matrix
                .entries
                .iter()
                .filter(|entry| entry.is_deficit)
                .count()
        );
    }

    #[test]
    fn direction_merge_projection_round_trips_with_signals_and_counts_preserved() {
        let day = CanChi::new(0, 0);
        let matrix = compute_direction_merge(&day, "Tây Nam", "Đông Bắc", &make_kua());
        let graph =
            build_direction_merge_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let projected =
            project_direction_merge_matrix(&graph, "direction_merge:2024-01-01:test-profile")
                .unwrap();

        assert_eq!(projected, matrix);
        assert_eq!(
            projected
                .entries
                .iter()
                .map(|entry| &entry.direction)
                .collect::<Vec<_>>(),
            matrix
                .entries
                .iter()
                .map(|entry| &entry.direction)
                .collect::<Vec<_>>()
        );
        assert_eq!(
            projected
                .entries
                .iter()
                .max_by_key(|entry| entry.net_score)
                .map(|entry| (&entry.direction, entry.net_score)),
            matrix
                .entries
                .iter()
                .max_by_key(|entry| entry.net_score)
                .map(|entry| (&entry.direction, entry.net_score))
        );
    }

    #[test]
    fn domain_day_boost_projection_round_trips_with_scores_preserved() {
        let matrix = compute_domain_day_boost(&make_fortune(), &make_domain_scores(), 2);
        let graph =
            build_domain_day_boost_matrix_graph("2024-01-01", "test-profile", &matrix).unwrap();

        let projected =
            project_domain_day_boost_matrix(&graph, "domain_day_boost:2024-01-01:test-profile")
                .unwrap();

        assert_eq!(projected, matrix);
        assert_eq!(
            projected
                .entries
                .iter()
                .max_by(|left, right| left
                    .boosted_score
                    .partial_cmp(&right.boosted_score)
                    .unwrap())
                .map(|entry| (&entry.domain, entry.boosted_score)),
            matrix
                .entries
                .iter()
                .max_by(|left, right| left
                    .boosted_score
                    .partial_cmp(&right.boosted_score)
                    .unwrap())
                .map(|entry| (&entry.domain, entry.boosted_score))
        );
    }

    #[test]
    fn all_projection_functions_return_real_matrices() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));

        let day_person = compute_day_person_matrix(&day, &chart);
        let day_person_graph =
            build_day_person_matrix_graph("2024-01-01", "test", &day_person).unwrap();
        assert!(project_day_person_matrix(&day_person_graph, "day_person:2024-01-01:test").is_ok());

        let personal_hour =
            compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let personal_hour_graph =
            build_personal_hour_matrix_graph("2024-01-01", "test", &personal_hour).unwrap();
        assert!(project_personal_hour_matrix(
            &personal_hour_graph,
            "personal_hour:2024-01-01:test"
        )
        .is_ok());

        let element_resonance = compute_element_resonance(&day, "Dần", &balanced_dist());
        let element_resonance_graph =
            build_element_resonance_matrix_graph("2024-01-01", "test", &element_resonance).unwrap();
        assert!(project_element_resonance_matrix(
            &element_resonance_graph,
            "element_resonance:2024-01-01:test"
        )
        .is_ok());

        let direction_merge = compute_direction_merge(&day, "Tây Nam", "Đông Bắc", &make_kua());
        let direction_merge_graph =
            build_direction_merge_matrix_graph("2024-01-01", "test", &direction_merge).unwrap();
        assert!(project_direction_merge_matrix(
            &direction_merge_graph,
            "direction_merge:2024-01-01:test"
        )
        .is_ok());

        let domain_day_boost = compute_domain_day_boost(&make_fortune(), &make_domain_scores(), 0);
        let domain_day_boost_graph =
            build_domain_day_boost_matrix_graph("2024-01-01", "test", &domain_day_boost).unwrap();
        assert!(project_domain_day_boost_matrix(
            &domain_day_boost_graph,
            "domain_day_boost:2024-01-01:test"
        )
        .is_ok());
    }
}
