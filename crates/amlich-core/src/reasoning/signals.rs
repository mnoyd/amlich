use crate::{
    almanac::{recommendation::evidence::collect_truc_hits, types::DayDeityClassification},
    insight_data::find_truc_insight,
};

use super::{EdgeEffect, InterpretedAxis, NodeKind, ReasoningEdge, ReasoningGraph, ReasoningNode};

pub fn derive_interpreted_signals(mut graph: ReasoningGraph) -> Result<ReasoningGraph, String> {
    let fact_summary_count = graph.nodes.len();
    let has_favorable_fact = graph.nodes.iter().any(is_favorable_fact);
    let has_unfavorable_fact = graph.nodes.iter().any(is_unfavorable_fact);

    graph.nodes.extend(
        InterpretedAxis::core_axes()
            .into_iter()
            .map(build_signal_node),
    );

    for node in graph
        .nodes
        .clone()
        .into_iter()
        .filter(|node| node.kind == NodeKind::Fact)
    {
        match node.id.as_str() {
            "fact.day.truc" => {
                if node.severity.as_deref() == Some("cat") {
                    graph.edges.push(ReasoningEdge::new(
                        node.id.clone(),
                        InterpretedAxis::Support.signal_node_id(),
                        EdgeEffect::Supports,
                    ));
                }
                if let Some(truc_name) = node.summary_vi.strip_prefix("Trực ") {
                    if let Some(truc) = find_truc_insight(truc_name) {
                        let opening_hits = collect_truc_hits(truc)
                            .into_iter()
                            .filter(|hit| {
                                hit.activity_id == crate::almanac::recommendation::ActivityId::OpeningStart
                            })
                            .collect::<Vec<_>>();
                        let opening_avoid_count = opening_hits
                            .iter()
                            .filter(|hit| matches!(hit.direction, crate::almanac::recommendation::evidence::BaseDirection::Avoid))
                            .count();
                        let has_opening_avoid = opening_avoid_count > 0;
                        let has_opening_favor = opening_hits
                            .iter()
                            .any(|hit| matches!(hit.direction, crate::almanac::recommendation::evidence::BaseDirection::Favor));

                        if has_opening_avoid {
                            graph.edges.push(ReasoningEdge::new(
                                node.id.clone(),
                                InterpretedAxis::Resistance.signal_node_id(),
                                if opening_avoid_count > 1 {
                                    EdgeEffect::Overrides
                                } else {
                                    EdgeEffect::Supports
                                },
                            ));
                            graph.edges.push(ReasoningEdge::new(
                                node.id.clone(),
                                InterpretedAxis::ContextClarity.signal_node_id(),
                                EdgeEffect::ConflictsWith,
                            ));
                        }

                        if has_opening_favor {
                            graph.edges.push(ReasoningEdge::new(
                                node.id.clone(),
                                InterpretedAxis::Support.signal_node_id(),
                                EdgeEffect::Supports,
                            ));
                        }
                    }
                }
            }
            "fact.day.day_deity" => {
                if node.severity.as_deref() == Some(day_deity_tag(DayDeityClassification::HoangDao))
                {
                    graph.edges.push(ReasoningEdge::new(
                        node.id.clone(),
                        InterpretedAxis::Support.signal_node_id(),
                        EdgeEffect::Supports,
                    ));
                }
            }
            "fact.day.nhi_thap_bat_tu" => {
                if is_star_supportive(&node.summary_vi) {
                    graph.edges.push(ReasoningEdge::new(
                        node.id.clone(),
                        InterpretedAxis::Support.signal_node_id(),
                        EdgeEffect::Supports,
                    ));
                }
            }
            "fact.day.taboos" => {
                let effect = if node.severity.as_deref() == Some("hard") {
                    EdgeEffect::Overrides
                } else {
                    EdgeEffect::Supports
                };
                graph.edges.push(ReasoningEdge::new(
                    node.id.clone(),
                    InterpretedAxis::Resistance.signal_node_id(),
                    effect,
                ));
                if node.severity.is_some() {
                    graph.edges.push(ReasoningEdge::new(
                        node.id.clone(),
                        InterpretedAxis::Stability.signal_node_id(),
                        EdgeEffect::Weakens,
                    ));
                }
                if node.severity.as_deref() == Some("hard") {
                    graph.edges.push(ReasoningEdge::new(
                        node.id.clone(),
                        InterpretedAxis::ContextClarity.signal_node_id(),
                        EdgeEffect::Overrides,
                    ));
                }
            }
            "fact.day.xung_hop" => {
                if looks_clash_heavy(&node.summary_vi) {
                    graph.edges.push(ReasoningEdge::new(
                        node.id.clone(),
                        InterpretedAxis::Resistance.signal_node_id(),
                        EdgeEffect::Supports,
                    ));
                    graph.edges.push(ReasoningEdge::new(
                        node.id.clone(),
                        InterpretedAxis::Stability.signal_node_id(),
                        EdgeEffect::Weakens,
                    ));
                }
            }
            "fact.day.hoang_dao_hours" => {
                if has_good_hour_capacity(&node.severity) {
                    graph.edges.push(ReasoningEdge::new(
                        node.id.clone(),
                        InterpretedAxis::TimingFit.signal_node_id(),
                        EdgeEffect::Supports,
                    ));
                }
            }
            "fact.personal.day_person_matrix" => {
                graph.edges.push(ReasoningEdge::new(
                    node.id.clone(),
                    InterpretedAxis::PersonalAlignment.signal_node_id(),
                    personal_alignment_effect(&node.summary_vi),
                ));
            }
            "fact.personal.personal_hour_matrix" => {
                graph.edges.push(ReasoningEdge::new(
                    node.id.clone(),
                    InterpretedAxis::PersonalAlignment.signal_node_id(),
                    personal_hour_effect(&node.summary_vi),
                ));
            }
            _ => {}
        }
    }

    if has_favorable_fact && has_unfavorable_fact {
        graph.edges.push(ReasoningEdge::new(
            "fact.graph.mixed_day_signals",
            InterpretedAxis::ContextClarity.signal_node_id(),
            EdgeEffect::ConflictsWith,
        ));
    }

    let clarity_edge_count = graph
        .edges
        .iter()
        .filter(|edge| edge.to_node_id == InterpretedAxis::ContextClarity.signal_node_id())
        .count();
    if clarity_edge_count == 0 && fact_summary_count > 0 {
        graph.edges.push(ReasoningEdge::new(
            "fact.graph.available_context",
            InterpretedAxis::ContextClarity.signal_node_id(),
            EdgeEffect::Supports,
        ));
    }

    Ok(graph)
}

fn build_signal_node(axis: InterpretedAxis) -> ReasoningNode {
    ReasoningNode {
        id: axis.signal_node_id().to_string(),
        kind: NodeKind::InterpretedSignal,
        summary_vi: signal_summary(axis).to_string(),
        severity: Some("0".to_string()),
        evidence: vec![format!("axis::{axis:?}")],
    }
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

fn is_favorable_fact(node: &ReasoningNode) -> bool {
    match node.id.as_str() {
        "fact.day.truc" => node.severity.as_deref() == Some("cat"),
        "fact.day.day_deity" => {
            node.severity.as_deref() == Some(day_deity_tag(DayDeityClassification::HoangDao))
        }
        "fact.day.nhi_thap_bat_tu" => is_star_supportive(&node.summary_vi),
        "fact.day.hoang_dao_hours" => has_good_hour_capacity(&node.severity),
        "fact.personal.day_person_matrix" => !node.summary_vi.contains("0 trụ hợp"),
        "fact.personal.personal_hour_matrix" => !node.summary_vi.contains("Chưa có"),
        _ => false,
    }
}

fn is_unfavorable_fact(node: &ReasoningNode) -> bool {
    match node.id.as_str() {
        "fact.day.taboos" => node.severity.is_some(),
        "fact.day.xung_hop" => looks_clash_heavy(&node.summary_vi),
        "fact.personal.day_person_matrix" => node.summary_vi.contains("xung/khắc"),
        _ => false,
    }
}

fn is_star_supportive(summary: &str) -> bool {
    summary.contains("cát tinh") || summary.contains("Nhị thập bát tú")
}

fn looks_clash_heavy(summary: &str) -> bool {
    summary.contains("Xung") && !summary.contains(", hợp ")
}

fn has_good_hour_capacity(severity: &Option<String>) -> bool {
    severity
        .as_deref()
        .and_then(|value| value.parse::<usize>().ok())
        .is_some_and(|count| count > 0)
}

fn personal_alignment_effect(summary: &str) -> EdgeEffect {
    if extract_first_count(summary, "trụ hợp") > extract_first_count(summary, "trụ xung/khắc")
    {
        EdgeEffect::Supports
    } else {
        EdgeEffect::Weakens
    }
}

fn personal_hour_effect(summary: &str) -> EdgeEffect {
    if summary.contains("điểm -") || summary.contains("Chưa có") {
        EdgeEffect::Weakens
    } else {
        EdgeEffect::Supports
    }
}

fn extract_first_count(summary: &str, marker: &str) -> usize {
    summary
        .split(marker)
        .next()
        .and_then(|prefix| prefix.split_whitespace().last())
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(0)
}

const fn day_deity_tag(classification: DayDeityClassification) -> &'static str {
    match classification {
        DayDeityClassification::HoangDao => "hoang_dao",
        DayDeityClassification::HacDao => "hac_dao",
    }
}
