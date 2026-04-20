use crate::reasoning::action_evaluator::ActionEvaluation;
use crate::reasoning::types::{
    InitiationOpeningDecision, InitiationOpeningDecisionExport, ReasoningGraphExport,
};
use crate::semantic_graph::SemanticGraph;
use crate::semantic_graph::NodeConcept;

pub fn project_initiation_opening_decision(
    evaluation: &ActionEvaluation,
) -> InitiationOpeningDecision {
    InitiationOpeningDecision {
        primary_conclusion: evaluation.primary_conclusion.clone(),
        recommendation_bucket: evaluation.bucket,
        strongest_supports: evaluation
            .strongest_supports
            .iter()
            .map(|n| n.summary_vi.clone())
            .collect(),
        strongest_resistances: evaluation
            .strongest_resistances
            .iter()
            .map(|n| n.summary_vi.clone())
            .collect(),
        override_factors: evaluation
            .override_factors
            .iter()
            .map(|n| n.summary_vi.clone())
            .collect(),
        conflict_notes: evaluation
            .conflict_notes
            .iter()
            .map(|n| n.summary_vi.clone())
            .collect(),
        confidence: evaluation.confidence,
        context_is_clear: evaluation.context_is_clear,
        suggested_hours: evaluation.suggested_hours.clone(),
        suggested_directions: evaluation.suggested_directions.clone(),
    }
}

pub fn project_initiation_opening_decision_export(
    evaluation: &ActionEvaluation,
) -> InitiationOpeningDecisionExport {
    InitiationOpeningDecisionExport {
        primary_conclusion: evaluation.primary_conclusion.clone(),
        recommendation_bucket: evaluation.bucket,
        confidence: evaluation.confidence,
        context_is_clear: evaluation.context_is_clear,
        semantic: evaluation.semantic,
        strongest_supports: evaluation.strongest_supports.clone(),
        strongest_resistances: evaluation.strongest_resistances.clone(),
        override_factors: evaluation.override_factors.clone(),
        conflict_notes: evaluation.conflict_notes.clone(),
        suggested_hours: evaluation.suggested_hours.clone(),
        suggested_directions: evaluation.suggested_directions.clone(),
        axis_scores: evaluation.axis_scores.clone(),
    }
}

pub fn project_reasoning_graph_export(
    graph: &SemanticGraph,
    evaluation: &ActionEvaluation,
) -> ReasoningGraphExport {
    use crate::reasoning::types::{
        NodeKind, ReasoningNodeExport, ReasoningNodeSeverity,
    };

    let referenced_node_ids = &evaluation.referenced_node_ids;
    let mut nodes = Vec::new();
    let edges = Vec::new();

    for node_id in referenced_node_ids {
        if let Some(node) = graph.get_node(node_id) {
            let (axis, severity, tags) = match node.concept {
                NodeConcept::Truc => {
                    let axis = if node.severity.as_deref() == Some("cat") {
                        Some(crate::reasoning::types::InterpretedAxis::Support)
                    } else {
                        None
                    };
                    let severity = node.severity.as_ref().map(|s| {
                        if s == "cat" {
                            ReasoningNodeSeverity::Auspicious
                        } else {
                            ReasoningNodeSeverity::Inauspicious
                        }
                    });
                    let mut tags = vec!["day".to_string(), "truc".to_string()];
                    if node.severity.as_deref() == Some("cat") {
                        tags.push("support".to_string());
                    }
                    (axis, severity, tags)
                }
                NodeConcept::DayDeity => {
                    let axis = node.severity.as_ref().map(|s| {
                        if s == "hoang_dao" {
                            crate::reasoning::types::InterpretedAxis::Support
                        } else {
                            crate::reasoning::types::InterpretedAxis::Resistance
                        }
                    });
                    let severity = node.severity.as_ref().map(|s| {
                        if s == "hoang_dao" {
                            ReasoningNodeSeverity::HoangDao
                        } else {
                            ReasoningNodeSeverity::HacDao
                        }
                    });
                    let mut tags = vec!["day".to_string(), "day_deity".to_string()];
                    if node.severity.as_deref() == Some("hoang_dao") {
                        tags.push("support".to_string());
                    }
                    (axis, severity, tags)
                }
                NodeConcept::Taboo => {
                    let severity = node.severity.as_ref().map(|s| {
                        if s == "hard" {
                            ReasoningNodeSeverity::HardTaboo
                        } else {
                            ReasoningNodeSeverity::SoftTaboo
                        }
                    });
                    let mut tags = vec!["day".to_string(), "taboo".to_string(), "resistance".to_string()];
                    if node.severity.as_deref() == Some("hard") {
                        tags.push("override".to_string());
                    }
                    (None, severity, tags)
                }
                NodeConcept::XungHop => {
                    let severity = if node.summary_vi.contains("Xung") && !node.summary_vi.contains(", hợp ") {
                        Some(ReasoningNodeSeverity::Inauspicious)
                    } else {
                        None
                    };
                    let tags = vec!["day".to_string(), "xung_hop".to_string(), "resistance".to_string()];
                    (None, severity, tags)
                }
                NodeConcept::Star => {
                    let severity = if node.summary_vi.contains("cát tinh") {
                        Some(ReasoningNodeSeverity::Auspicious)
                    } else if node.summary_vi.contains("sát tinh") {
                        Some(ReasoningNodeSeverity::Inauspicious)
                    } else {
                        None
                    };
                    let tags = vec!["day".to_string(), "star".to_string(), "support".to_string()];
                    (None, severity, tags)
                }
                NodeConcept::HoangDaoHour => {
                    let severity = node.severity.as_ref().and_then(|s| {
                        s.parse::<usize>().ok().filter(|&c| c > 0).map(|_| ReasoningNodeSeverity::Auspicious)
                    });
                    let tags = vec!["day".to_string(), "hoang_dao".to_string(), "support".to_string()];
                    (None, severity, tags)
                }
                _ => (None, None, vec!["day".to_string()]),
            };

            nodes.push(ReasoningNodeExport {
                id: node.node_id.clone(),
                kind: NodeKind::Fact,
                axis,
                severity,
                tags,
                summary_vi: node.summary_vi.clone(),
                evidence: node.provenance.iter().map(|p| {
                    crate::reasoning::types::ReasoningEvidenceEnvelope {
                        source_family: crate::reasoning::types::ReasoningEvidenceSourceFamily::Snapshot,
                        source_id: p.source_id.clone(),
                        method: p.method.clone(),
                        note: p.note.clone(),
                    }
                }).collect(),
            });
        }
    }

    ReasoningGraphExport {
        action_id: evaluation.action_id,
        nodes,
        edges,
    }
}
