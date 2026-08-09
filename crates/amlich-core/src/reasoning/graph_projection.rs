use crate::almanac::recommendation::evidence::{collect_truc_hits, BaseDirection};
use crate::almanac::recommendation::ActivityId;
use crate::insight_data::find_truc_insight;
use crate::semantic_graph::{NodeConcept, SemanticFact, SemanticGraph, SemanticPolarity};
use crate::DaySnapshot;

use super::export::{axis_for_node, severity_for_node, tags_for_node};
use super::personal::{PersonalAssessmentFacts, PersonalReasoningInput};
use super::types::{
    ActionId, EdgeEffect, InterpretedAxis, NodeKind, ReasoningEdgeExport,
    ReasoningEdgeJustification, ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily,
    ReasoningGraphExport, ReasoningNodeExport, ReasoningNodeSeverity,
};

/// Project the reasoning graph reusing a precomputed
/// [`PersonalAssessmentFacts`]. Per-request request paths must use this
/// variant so the fact-node projection does not rebuild the chart and
/// matrices — see REPAIR-PLAN.md P2 (`amlich-mwbp.8` finding A-R11).
pub fn project_semantic_graph_export_with_facts(
    graph: &SemanticGraph,
    _evaluation: &super::action_evaluator::ActionEvaluation,
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
    facts: Option<&PersonalAssessmentFacts>,
) -> ReasoningGraphExport {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    let solar_term_node = build_solar_term_node(snapshot);
    let truc_node = build_truc_node(snapshot);
    let star_node = build_star_node(snapshot);
    let deity_node = build_day_deity_node(snapshot);
    let taboo_node = build_taboo_node(snapshot);
    let xung_hop_node = build_xung_hop_node(snapshot);
    let travel_node = build_travel_direction_node(snapshot);
    let hours_node = build_hoang_dao_hours_node(snapshot);

    nodes.push(solar_term_node.clone());
    nodes.push(truc_node.clone());
    nodes.push(star_node.clone());
    nodes.push(deity_node.clone());
    nodes.push(taboo_node.clone());
    nodes.push(xung_hop_node.clone());
    nodes.push(travel_node.clone());
    nodes.push(hours_node.clone());

    for axis in InterpretedAxis::core_axes() {
        nodes.push(build_signal_node(axis));
    }

    add_truc_edges(&truc_node, &mut edges, snapshot);
    add_deity_edges(&deity_node, &mut edges);
    add_star_edges(&star_node, &mut edges);
    add_taboo_edges(&taboo_node, &mut edges);
    add_xung_hop_edges(&xung_hop_node, &mut edges);
    add_hours_edges(&hours_node, &mut edges);

    let has_favorable = has_favorable_fact(graph);
    let has_unfavorable = has_unfavorable_fact(graph);

    if has_favorable && has_unfavorable {
        edges.push(ReasoningEdgeExport {
            from_node_id: "fact.graph.mixed_day_signals".to_string(),
            to_node_id: InterpretedAxis::ContextClarity.signal_node_id().to_string(),
            effect: EdgeEffect::ConflictsWith,
            weight: 1,
            justification: ReasoningEdgeJustification::MixedSignalConflict,
            evidence: vec![ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::Derived,
                source_id: "fact.graph.mixed_day_signals".to_string(),
                method: "mixed_fact_detection".to_string(),
                note: None,
            }],
            tags: vec![],
        });
    }

    let clarity_edge_count = edges
        .iter()
        .filter(|e| e.to_node_id == InterpretedAxis::ContextClarity.signal_node_id())
        .count();
    if clarity_edge_count == 0 {
        edges.push(ReasoningEdgeExport {
            from_node_id: "fact.graph.available_context".to_string(),
            to_node_id: InterpretedAxis::ContextClarity.signal_node_id().to_string(),
            effect: EdgeEffect::Supports,
            weight: 1,
            justification: ReasoningEdgeJustification::AvailableContextSupport,
            evidence: vec![ReasoningEvidenceEnvelope {
                source_family: ReasoningEvidenceSourceFamily::Derived,
                source_id: "fact.graph.available_context".to_string(),
                method: "context_availability".to_string(),
                note: None,
            }],
            tags: vec![],
        });
    }

    if let Some(personal) = personal_input {
        let personal_nodes = if let Some(facts) = facts {
            personal.build_fact_nodes_from_facts(facts)
        } else {
            personal.build_fact_nodes(snapshot).unwrap_or_default()
        };
        add_personal_node_edges(&mut nodes, &mut edges, personal_nodes);
    }

    ReasoningGraphExport {
        action_id: ActionId::InitiationOpening,
        nodes,
        edges,
    }
}

/// Legacy snapshot-based projection kept for callers that go through
/// `build_initiation_opening_reasoning_bundle` (no precomputed facts).
/// Per-request paths must use [`project_semantic_graph_export_with_facts`]
/// instead — see REPAIR-PLAN.md P2 (`amlich-mwbp.8` finding A-R11).
#[allow(dead_code)]
pub fn project_semantic_graph_export(
    graph: &SemanticGraph,
    _evaluation: &super::action_evaluator::ActionEvaluation,
    snapshot: &DaySnapshot,
    personal_input: Option<&super::personal::PersonalReasoningInput>,
) -> ReasoningGraphExport {
    project_semantic_graph_export_with_facts(graph, _evaluation, snapshot, personal_input, None)
}

fn add_personal_node_edges(
    nodes: &mut Vec<ReasoningNodeExport>,
    edges: &mut Vec<ReasoningEdgeExport>,
    personal_nodes: Vec<super::personal::PersonalFactNode>,
) {
    for raw_node in personal_nodes {
        let id = raw_node.id.clone();
        let effect = raw_node.effect;
        let node_export = ReasoningNodeExport {
            id: id.clone(),
            kind: NodeKind::Fact,
            axis: axis_for_node(&id),
            // Personal fact nodes carry no severity classification: their
            // favorability is expressed via typed edges/summary, not via an
            // overloaded severity slot (amlich-0q2f).
            severity: None,
            tags: tags_for_node(&id),
            summary_vi: raw_node.summary_vi,
            evidence: raw_node.evidence,
        };

        match id.as_str() {
            "fact.personal.day_person_matrix" => {
                let effect = effect.unwrap_or(EdgeEffect::Weakens);
                edges.push(make_edge(
                    &id,
                    InterpretedAxis::PersonalAlignment.signal_node_id(),
                    effect,
                    if effect == EdgeEffect::Overrides {
                        2
                    } else {
                        1
                    },
                    ReasoningEdgeJustification::PersonalDayAlignment,
                    node_export.evidence.clone(),
                ));
            }
            "fact.personal.personal_hour_matrix" => {
                let effect = effect.unwrap_or(EdgeEffect::Weakens);
                edges.push(make_edge(
                    &id,
                    InterpretedAxis::PersonalAlignment.signal_node_id(),
                    effect,
                    1,
                    ReasoningEdgeJustification::PersonalHourAlignment,
                    node_export.evidence.clone(),
                ));
            }
            _ => {}
        }

        nodes.push(node_export);
    }
}

fn build_solar_term_node(snapshot: &DaySnapshot) -> ReasoningNodeExport {
    ReasoningNodeExport {
        id: "fact.day.solar_term".to_string(),
        kind: NodeKind::Fact,
        axis: None,
        severity: None,
        tags: vec!["context".to_string()],
        summary_vi: snapshot.context.tiet_khi.name.clone(),
        evidence: vec![snapshot_evidence(
            "snapshot.context.tiet_khi",
            "context.tiet_khi",
        )],
    }
}

fn build_truc_node(snapshot: &DaySnapshot) -> ReasoningNodeExport {
    let summary_vi = format!("Trực {}", snapshot.day_fortune.truc.name);
    let quality = &snapshot.day_fortune.truc.quality;
    ReasoningNodeExport {
        id: "fact.day.truc".to_string(),
        kind: NodeKind::Fact,
        axis: axis_for_node("fact.day.truc"),
        severity: severity_for_node("fact.day.truc", Some(quality.as_str())),
        tags: tags_for_node("fact.day.truc"),
        summary_vi,
        evidence: vec![snapshot_evidence(
            "snapshot.day_fortune.truc",
            "day_fortune.truc",
        )],
    }
}

fn build_star_node(snapshot: &DaySnapshot) -> ReasoningNodeExport {
    let summary_vi = summarize_stars(&snapshot.day_fortune.stars);
    ReasoningNodeExport {
        id: "fact.day.nhi_thap_bat_tu".to_string(),
        kind: NodeKind::Fact,
        axis: axis_for_node("fact.day.nhi_thap_bat_tu"),
        severity: star_severity(&snapshot.day_fortune.stars),
        tags: tags_for_node("fact.day.nhi_thap_bat_tu"),
        summary_vi,
        evidence: vec![snapshot_evidence(
            "snapshot.day_fortune.stars",
            "day_fortune.stars",
        )],
    }
}

fn build_day_deity_node(snapshot: &DaySnapshot) -> ReasoningNodeExport {
    let summary_vi = match &snapshot.day_fortune.day_deity {
        Some(deity) => deity.name.clone(),
        None => "Không có thần sát ngày".to_string(),
    };
    let severity_str =
        snapshot
            .day_fortune
            .day_deity
            .as_ref()
            .map(|deity| match deity.classification {
                crate::almanac::types::DayDeityClassification::HoangDao => "hoang_dao",
                crate::almanac::types::DayDeityClassification::HacDao => "hac_dao",
            });
    ReasoningNodeExport {
        id: "fact.day.day_deity".to_string(),
        kind: NodeKind::Fact,
        axis: axis_for_node("fact.day.day_deity"),
        severity: severity_for_node("fact.day.day_deity", severity_str),
        tags: tags_for_node("fact.day.day_deity"),
        summary_vi,
        evidence: vec![snapshot_evidence(
            "snapshot.day_fortune.day_deity",
            "day_fortune.day_deity",
        )],
    }
}

fn build_taboo_node(snapshot: &DaySnapshot) -> ReasoningNodeExport {
    let taboos = &snapshot.day_fortune.taboos;
    let summary_vi = if taboos.is_empty() {
        "Không có điều kiêng kỵ nổi bật".to_string()
    } else {
        format!(
            "Kiêng/kỵ: {}",
            taboos
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };
    let severity_str = taboo_severity_val(taboos);
    let evidence: Vec<_> = taboos
        .iter()
        .take(3)
        .map(|t| almanac_rule_evidence(&t.rule_id))
        .collect();
    ReasoningNodeExport {
        id: "fact.day.taboos".to_string(),
        kind: NodeKind::Fact,
        axis: axis_for_node("fact.day.taboos"),
        severity: severity_for_node("fact.day.taboos", severity_str.as_deref()),
        tags: tags_for_node("fact.day.taboos"),
        summary_vi,
        evidence,
    }
}

fn build_xung_hop_node(snapshot: &DaySnapshot) -> ReasoningNodeExport {
    let summary_vi = format!(
        "Xung {}{}",
        snapshot.day_fortune.conflict.opposing_chi,
        snapshot
            .day_fortune
            .xung_hop
            .liu_he
            .as_ref()
            .map(|p| format!(", hợp {p}"))
            .unwrap_or_default()
    );
    ReasoningNodeExport {
        id: "fact.day.xung_hop".to_string(),
        kind: NodeKind::Fact,
        axis: axis_for_node("fact.day.xung_hop"),
        severity: if snapshot.day_fortune.xung_hop.liu_he.is_none() {
            Some(ReasoningNodeSeverity::Inauspicious)
        } else {
            None
        },
        tags: tags_for_node("fact.day.xung_hop"),
        summary_vi,
        evidence: vec![
            snapshot_evidence("snapshot.day_fortune.conflict", "day_fortune.conflict"),
            snapshot_evidence("snapshot.day_fortune.xung_hop", "day_fortune.xung_hop"),
        ],
    }
}

fn build_travel_direction_node(snapshot: &DaySnapshot) -> ReasoningNodeExport {
    let summary_vi = format!(
        "Xuất hành {}, Tài Thần {}, Hỷ Thần {}",
        snapshot.day_fortune.travel.xuat_hanh_huong,
        snapshot.day_fortune.travel.tai_than,
        snapshot.day_fortune.travel.hy_than
    );
    ReasoningNodeExport {
        id: "fact.day.travel_directions".to_string(),
        kind: NodeKind::Fact,
        axis: axis_for_node("fact.day.travel_directions"),
        severity: None,
        tags: tags_for_node("fact.day.travel_directions"),
        summary_vi,
        evidence: vec![snapshot_evidence(
            "snapshot.day_fortune.travel",
            "day_fortune.travel",
        )],
    }
}

fn build_hoang_dao_hours_node(snapshot: &DaySnapshot) -> ReasoningNodeExport {
    let summary_vi = snapshot.context.gio_hoang_dao.summary.clone();
    // Drive favorability from the boolean count check, not by encoding the
    // count as a severity string (amlich-0q2f: no overloaded numeric
    // severities).
    let severity_str =
        (snapshot.context.gio_hoang_dao.good_hour_count > 0).then_some("has_good_hours");
    ReasoningNodeExport {
        id: "fact.day.hoang_dao_hours".to_string(),
        kind: NodeKind::Fact,
        axis: axis_for_node("fact.day.hoang_dao_hours"),
        severity: severity_for_node("fact.day.hoang_dao_hours", severity_str),
        tags: tags_for_node("fact.day.hoang_dao_hours"),
        summary_vi,
        evidence: vec![snapshot_evidence(
            "snapshot.context.gio_hoang_dao",
            "context.gio_hoang_dao",
        )],
    }
}

fn build_signal_node(axis: InterpretedAxis) -> ReasoningNodeExport {
    let summary = signal_summary(axis);
    ReasoningNodeExport {
        id: axis.signal_node_id().to_string(),
        kind: NodeKind::InterpretedSignal,
        axis: Some(axis),
        severity: None,
        tags: vec!["signal".to_string()],
        summary_vi: summary.to_string(),
        evidence: vec![ReasoningEvidenceEnvelope {
            source_family: ReasoningEvidenceSourceFamily::Axis,
            source_id: format!("axis::{axis:?}"),
            method: "axis_registration".to_string(),
            note: None,
        }],
    }
}

fn add_truc_edges(
    truc_node: &ReasoningNodeExport,
    edges: &mut Vec<ReasoningEdgeExport>,
    snapshot: &DaySnapshot,
) {
    let evidence = &truc_node.evidence;
    if truc_node.severity.is_some() {
        if let Some(ReasoningNodeSeverity::Auspicious) = truc_node.severity {
            edges.push(make_edge(
                "fact.day.truc",
                InterpretedAxis::Support.signal_node_id(),
                EdgeEffect::Supports,
                1,
                ReasoningEdgeJustification::FavorableDaySignal,
                evidence.clone(),
            ));
        }
    }

    if let Some(truc) = find_truc_insight(&snapshot.day_fortune.truc.name) {
        let opening_hits: Vec<_> = collect_truc_hits(truc)
            .into_iter()
            .filter(|hit| hit.activity_id == ActivityId::OpeningStart)
            .collect();
        let opening_avoid_count = opening_hits
            .iter()
            .filter(|hit| matches!(hit.direction, BaseDirection::Avoid))
            .count();
        let has_opening_favor = opening_hits
            .iter()
            .any(|hit| matches!(hit.direction, BaseDirection::Favor));

        if opening_avoid_count > 0 {
            let effect = if opening_avoid_count > 1 {
                EdgeEffect::Overrides
            } else {
                EdgeEffect::Supports
            };
            edges.push(make_edge(
                "fact.day.truc",
                InterpretedAxis::Resistance.signal_node_id(),
                effect,
                if effect == EdgeEffect::Overrides {
                    2
                } else {
                    1
                },
                ReasoningEdgeJustification::TrucActivityConflict,
                truc_evidence(truc.id.as_str(), "opening_start"),
            ));
            edges.push(make_edge(
                "fact.day.truc",
                InterpretedAxis::ContextClarity.signal_node_id(),
                EdgeEffect::ConflictsWith,
                1,
                ReasoningEdgeJustification::TrucActivityConflict,
                truc_evidence(truc.id.as_str(), "opening_start"),
            ));
        }

        if has_opening_favor {
            edges.push(make_edge(
                "fact.day.truc",
                InterpretedAxis::Support.signal_node_id(),
                EdgeEffect::Supports,
                1,
                ReasoningEdgeJustification::TrucActivitySupport,
                truc_evidence(truc.id.as_str(), "opening_start"),
            ));
        }
    }
}

fn add_deity_edges(deity_node: &ReasoningNodeExport, edges: &mut Vec<ReasoningEdgeExport>) {
    if deity_node.severity == Some(ReasoningNodeSeverity::HoangDao) {
        edges.push(make_edge(
            "fact.day.day_deity",
            InterpretedAxis::Support.signal_node_id(),
            EdgeEffect::Supports,
            1,
            ReasoningEdgeJustification::DayDeitySupport,
            deity_node.evidence.clone(),
        ));
    }
}

fn add_star_edges(star_node: &ReasoningNodeExport, edges: &mut Vec<ReasoningEdgeExport>) {
    if star_node.severity == Some(ReasoningNodeSeverity::Auspicious) {
        edges.push(make_edge(
            "fact.day.nhi_thap_bat_tu",
            InterpretedAxis::Support.signal_node_id(),
            EdgeEffect::Supports,
            1,
            ReasoningEdgeJustification::StarSupport,
            star_node.evidence.clone(),
        ));
    }
}

fn add_taboo_edges(taboo_node: &ReasoningNodeExport, edges: &mut Vec<ReasoningEdgeExport>) {
    let effect = if taboo_node.severity == Some(ReasoningNodeSeverity::HardTaboo) {
        EdgeEffect::Overrides
    } else {
        EdgeEffect::Supports
    };
    edges.push(make_edge(
        "fact.day.taboos",
        InterpretedAxis::Resistance.signal_node_id(),
        effect,
        if effect == EdgeEffect::Overrides {
            2
        } else {
            1
        },
        ReasoningEdgeJustification::TabooPressure,
        taboo_node.evidence.clone(),
    ));
    if taboo_node.severity.is_some() {
        edges.push(make_edge(
            "fact.day.taboos",
            InterpretedAxis::Stability.signal_node_id(),
            EdgeEffect::Weakens,
            1,
            ReasoningEdgeJustification::TabooStabilityPenalty,
            taboo_node.evidence.clone(),
        ));
    }
    if taboo_node.severity == Some(ReasoningNodeSeverity::HardTaboo) {
        edges.push(make_edge(
            "fact.day.taboos",
            InterpretedAxis::ContextClarity.signal_node_id(),
            EdgeEffect::Overrides,
            2,
            ReasoningEdgeJustification::TabooContextPenalty,
            taboo_node.evidence.clone(),
        ));
    }
}

fn add_xung_hop_edges(xung_hop_node: &ReasoningNodeExport, edges: &mut Vec<ReasoningEdgeExport>) {
    if xung_hop_node.severity == Some(ReasoningNodeSeverity::Inauspicious) {
        edges.push(make_edge(
            "fact.day.xung_hop",
            InterpretedAxis::Resistance.signal_node_id(),
            EdgeEffect::Supports,
            1,
            ReasoningEdgeJustification::ClashPressure,
            xung_hop_node.evidence.clone(),
        ));
        edges.push(make_edge(
            "fact.day.xung_hop",
            InterpretedAxis::Stability.signal_node_id(),
            EdgeEffect::Weakens,
            1,
            ReasoningEdgeJustification::ClashStabilityPenalty,
            xung_hop_node.evidence.clone(),
        ));
    }
}

fn add_hours_edges(hours_node: &ReasoningNodeExport, edges: &mut Vec<ReasoningEdgeExport>) {
    if hours_node.severity.is_some() {
        edges.push(make_edge(
            "fact.day.hoang_dao_hours",
            InterpretedAxis::TimingFit.signal_node_id(),
            EdgeEffect::Supports,
            1,
            ReasoningEdgeJustification::HoangDaoHourSupport,
            hours_node.evidence.clone(),
        ));
    }
}

fn make_edge(
    from: &str,
    to: &str,
    effect: EdgeEffect,
    weight: i32,
    justification: ReasoningEdgeJustification,
    evidence: Vec<ReasoningEvidenceEnvelope>,
) -> ReasoningEdgeExport {
    let mut tags = Vec::new();
    if effect.is_override() {
        tags.push("override".to_string());
    }
    if matches!(effect, EdgeEffect::ConflictsWith) {
        tags.push("conflict".to_string());
    }
    if to == InterpretedAxis::Support.signal_node_id() {
        tags.push("support".to_string());
    }
    if to == InterpretedAxis::Resistance.signal_node_id() {
        tags.push("resistance".to_string());
    }
    if to == InterpretedAxis::ContextClarity.signal_node_id() {
        tags.push("context".to_string());
    }
    ReasoningEdgeExport {
        from_node_id: from.to_string(),
        to_node_id: to.to_string(),
        effect,
        weight,
        justification,
        evidence,
        tags,
    }
}

fn snapshot_evidence(source_id: &str, note: &str) -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::Snapshot,
        source_id: source_id.to_string(),
        method: "field_lookup".to_string(),
        note: Some(note.to_string()),
    }
}

fn almanac_rule_evidence(rule_id: &str) -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: rule_id.to_string(),
        method: "rule_match".to_string(),
        note: None,
    }
}

fn truc_evidence(truc_id: &str, note: &str) -> Vec<ReasoningEvidenceEnvelope> {
    vec![ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::Insight,
        source_id: format!("truc.{truc_id}"),
        method: "insight_lookup".to_string(),
        note: Some(note.to_string()),
    }]
}

fn signal_summary(axis: InterpretedAxis) -> &'static str {
    match axis {
        InterpretedAxis::Support => "Tín hiệu thuận cho khởi sự/mở việc",
        InterpretedAxis::Resistance => "Tín hiệu cản trở cần lưu ý",
        InterpretedAxis::Stability => "Độ ổn định tổng thể của bối cảnh ngày",
        InterpretedAxis::PersonalAlignment => "Mức hợp giữa ngày và dữ liệu cá nhân",
        InterpretedAxis::TimingFit => "Độ thuận theo khung giờ hành sự",
        InterpretedAxis::ContextClarity => "Độ rõ ràng hay mâu thuẫn của bối cảnh",
    }
}

fn has_favorable_fact(graph: &SemanticGraph) -> bool {
    graph.nodes().values().any(|n| match n.concept {
        NodeConcept::Truc => n.severity.as_deref() == Some("cat"),
        NodeConcept::DayDeity => n.severity.as_deref() == Some("hoang_dao"),
        NodeConcept::HoangDaoHour => n.severity.is_some(),
        NodeConcept::Star => matches!(
            n.fact,
            Some(SemanticFact::Star {
                polarity: SemanticPolarity::Favorable
            })
        ),
        _ => false,
    })
}

fn has_unfavorable_fact(graph: &SemanticGraph) -> bool {
    graph.nodes().values().any(|n| match n.concept {
        NodeConcept::Taboo => n.severity.is_some(),
        _ => false,
    })
}

fn summarize_stars(stars: &crate::almanac::types::DayStars) -> String {
    if let Some(day_star) = &stars.day_star {
        return format!("Nhị thập bát tú {}", day_star.name);
    }
    let mut parts = Vec::new();
    if !stars.cat_tinh.is_empty() {
        parts.push(format!("cát tinh {}", stars.cat_tinh.join(", ")));
    }
    if !stars.sat_tinh.is_empty() {
        parts.push(format!("sát tinh {}", stars.sat_tinh.join(", ")));
    }
    if parts.is_empty() {
        "Nhị thập bát tú".to_string()
    } else {
        parts.join("; ")
    }
}

fn star_severity(stars: &crate::almanac::types::DayStars) -> Option<ReasoningNodeSeverity> {
    use crate::almanac::types::StarQuality;

    match stars.day_star.as_ref().map(|star| &star.quality) {
        Some(StarQuality::Cat) => Some(ReasoningNodeSeverity::Auspicious),
        Some(StarQuality::Hung) => Some(ReasoningNodeSeverity::Inauspicious),
        Some(StarQuality::Binh) => None,
        None if !stars.cat_tinh.is_empty() && stars.sat_tinh.is_empty() => {
            Some(ReasoningNodeSeverity::Auspicious)
        }
        None if stars.cat_tinh.is_empty() && !stars.sat_tinh.is_empty() => {
            Some(ReasoningNodeSeverity::Inauspicious)
        }
        None => None,
    }
}

fn taboo_severity_val(taboos: &[crate::almanac::types::DayTaboo]) -> Option<String> {
    if taboos.iter().any(|t| t.severity == "hard") {
        return Some("hard".to_string());
    }
    if taboos.iter().any(|t| t.severity == "soft") {
        return Some("soft".to_string());
    }
    None
}
