use amlich_core::DebugSemanticGraphInspection;
use amlich_core::ReasoningEvidenceEnvelope;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecisionRole {
    Override,
    Resistance,
    Conflict,
    Support,
    Refinement,
}

impl DecisionRole {
    pub fn label(self) -> &'static str {
        match self {
            Self::Override => "Ghi đè",
            Self::Resistance => "Kháng cự",
            Self::Conflict => "Xung đột",
            Self::Support => "Hỗ trợ",
            Self::Refinement => "Bổ sung",
        }
    }

    pub fn sort_order(self) -> u8 {
        match self {
            Self::Override => 0,
            Self::Conflict => 1,
            Self::Resistance => 2,
            Self::Support => 3,
            Self::Refinement => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutgoingTarget {
    pub target_id: String,
    pub target_label: String,
    pub edge_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionStackEntry {
    pub role: DecisionRole,
    pub label: String,
    pub impact: Option<String>,
    pub provenance: Vec<ReasoningEvidenceEnvelope>,
    pub outgoing_targets: Vec<OutgoingTarget>,
}

pub fn extract_decision_stack(
    inspection: &DebugSemanticGraphInspection,
) -> Vec<DecisionStackEntry> {
    let nodes_map: HashMap<_, _> = inspection
        .visualization
        .nodes
        .iter()
        .map(|n| (n.node_id.clone(), n))
        .collect();

    let mut entries = Vec::new();

    for node in &inspection.visualization.nodes {
        let role = match node.semantic_kind.as_str() {
            "truc" => {
                if node.severity.as_deref() == Some("cat") {
                    Some(DecisionRole::Support)
                } else if node.severity.as_deref() == Some("hung") {
                    Some(DecisionRole::Resistance)
                } else {
                    None
                }
            }
            "day_deity" => {
                if node.label.contains("Hoàng Đạo") {
                    Some(DecisionRole::Support)
                } else if node.label.contains("Hạc Đạo") {
                    Some(DecisionRole::Resistance)
                } else {
                    None
                }
            }
            "taboo" => {
                if node.severity.as_deref() == Some("hard") {
                    Some(DecisionRole::Override)
                } else {
                    Some(DecisionRole::Resistance)
                }
            }
            "star" => {
                if node.label.contains("cát tinh") || node.label.contains("Nhị thập bát tú") {
                    Some(DecisionRole::Support)
                } else if node.label.contains("sát tinh") {
                    Some(DecisionRole::Resistance)
                } else {
                    None
                }
            }
            "hoang_dao_hour" => Some(DecisionRole::Support),
            "xung_hop" => {
                if node.label.contains("Xung") && !node.label.contains(", hợp ") {
                    Some(DecisionRole::Conflict)
                } else {
                    None
                }
            }
            "direction" => Some(DecisionRole::Refinement),
            _ => None,
        };

        let role = match role {
            Some(r) => r,
            None => continue,
        };

        let outgoing_targets: Vec<OutgoingTarget> = inspection
            .visualization
            .edges
            .iter()
            .filter(|e| e.from_id == node.node_id)
            .filter_map(|e| {
                nodes_map.get(&e.to_id).map(|target| OutgoingTarget {
                    target_id: target.node_id.clone(),
                    target_label: target.label.clone(),
                    edge_label: e.label.clone(),
                })
            })
            .collect();

        let impact = derive_impact(&node.severity, role);

        entries.push(DecisionStackEntry {
            role,
            label: node.label.clone(),
            impact,
            provenance: node.provenance.clone(),
            outgoing_targets,
        });
    }

    entries.sort_by(|a, b| {
        a.role
            .sort_order()
            .cmp(&b.role.sort_order())
            .then_with(|| a.label.cmp(&b.label))
    });

    entries
}

fn derive_impact(severity: &Option<String>, role: DecisionRole) -> Option<String> {
    match severity.as_deref() {
        Some("cat") | Some("positive") => Some("tích cực".to_string()),
        Some("hung") | Some("negative") => Some("bất lợi".to_string()),
        Some("hard") => Some("cấm kỵ nặng".to_string()),
        Some("soft") => Some("cấm kỵ nhẹ".to_string()),
        _ => match role {
            DecisionRole::Override => Some("ghi đè".to_string()),
            DecisionRole::Conflict => Some("xung đột".to_string()),
            DecisionRole::Refinement => Some("bổ sung".to_string()),
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_core::semantic_graph::{
        DebugInspectionDate, DebugInspectionSummary, DebugSemanticGraphInspection,
        VisualizationEdge, VisualizationGraph, VisualizationNode,
    };
    use std::collections::HashMap;

    fn blank_inspection(nodes: Vec<VisualizationNode>, edges: Vec<VisualizationEdge>) -> DebugSemanticGraphInspection {
        DebugSemanticGraphInspection {
            surface: "test".to_string(),
            date: DebugInspectionDate { year: 2024, month: 2, day: 10 },
            visualization: VisualizationGraph { nodes, edges },
            summary: DebugInspectionSummary {
                total_nodes: 0,
                total_edges: 0,
                clusters: vec![],
                semantic_kinds: vec![],
                has_recommendation_evidence: false,
            },
            cluster_counts: HashMap::new(),
            semantic_kind_counts: HashMap::new(),
            severity_counts: HashMap::new(),
        }
    }

    fn node(id: &str, kind: &str, label: &str, severity: Option<&str>) -> VisualizationNode {
        VisualizationNode {
            node_id: id.to_string(),
            label: label.to_string(),
            cluster: format!("cluster_{}", kind),
            semantic_kind: kind.to_string(),
            severity: severity.map(|s| s.to_string()),
            provenance: vec![],
            shape_hint: None,
        }
    }

    fn edge(id: &str, from: &str, to: &str, label: &str) -> VisualizationEdge {
        VisualizationEdge {
            edge_id: id.to_string(),
            from_id: from.to_string(),
            to_id: to.to_string(),
            label: label.to_string(),
            semantic_kind: "targets_activity".to_string(),
            weight: 1,
        }
    }

    #[test]
    fn truc_cat_maps_to_support() {
        let inspection = blank_inspection(
            vec![node("t1", "truc", "Trực Kiên Tạo (cát)", Some("cat"))],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Support);
        assert_eq!(stack[0].label, "Trực Kiên Tạo (cát)");
    }

    #[test]
    fn truc_hung_maps_to_resistance() {
        let inspection = blank_inspection(
            vec![node("t2", "truc", "Trực Phá (hung)", Some("hung"))],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Resistance);
    }

    #[test]
    fn truc_without_severity_is_skipped() {
        let inspection = blank_inspection(
            vec![node("t3", "truc", "Trực gì đó", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert!(stack.is_empty());
    }

    #[test]
    fn day_deity_hoang_dao_maps_to_support() {
        let inspection = blank_inspection(
            vec![node("dd1", "day_deity", "Giờ Hoàng Đạo", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Support);
    }

    #[test]
    fn day_deity_hac_dao_maps_to_resistance() {
        let inspection = blank_inspection(
            vec![node("dd2", "day_deity", "Giờ Hạc Đạo", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Resistance);
    }

    #[test]
    fn day_deity_other_is_skipped() {
        let inspection = blank_inspection(
            vec![node("dd3", "day_deity", "Tỳ", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert!(stack.is_empty());
    }

    #[test]
    fn taboo_hard_maps_to_override() {
        let inspection = blank_inspection(
            vec![node("tab1", "taboo", "Taboo hard", Some("hard"))],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Override);
    }

    #[test]
    fn taboo_soft_maps_to_resistance() {
        let inspection = blank_inspection(
            vec![node("tab2", "taboo", "Taboo soft", Some("soft"))],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Resistance);
    }

    #[test]
    fn star_cat_tinh_maps_to_support() {
        let inspection = blank_inspection(
            vec![node("s1", "star", "Sao Thiên Đức (cát tinh)", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Support);
    }

    #[test]
    fn star_nhi_thap_bat_tu_maps_to_support() {
        let inspection = blank_inspection(
            vec![node("s2", "star", "Nhị thập bát tú - Giác", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Support);
    }

    #[test]
    fn star_sat_tinh_maps_to_resistance() {
        let inspection = blank_inspection(
            vec![node("s3", "star", "Sao Đại Hao (sát tinh)", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Resistance);
    }

    #[test]
    fn star_other_is_skipped() {
        let inspection = blank_inspection(
            vec![node("s4", "star", "Sao gì đó lạ", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert!(stack.is_empty());
    }

    #[test]
    fn hoang_dao_hour_maps_to_support() {
        let inspection = blank_inspection(
            vec![node("hd1", "hoang_dao_hour", "Giờ Tý (Hoàng Đạo)", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Support);
    }

    #[test]
    fn xung_hop_xung_maps_to_conflict() {
        let inspection = blank_inspection(
            vec![node("xh1", "xung_hop", "Xung Thân", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Conflict);
    }

    #[test]
    fn xung_hop_xung_hop_combined_is_skipped() {
        let inspection = blank_inspection(
            vec![node("xh2", "xung_hop", "Xung Dần, hợp Ngọ", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert!(stack.is_empty());
    }

    #[test]
    fn xung_hop_hop_only_is_skipped() {
        let inspection = blank_inspection(
            vec![node("xh3", "xung_hop", "hợp Mão", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert!(stack.is_empty());
    }

    #[test]
    fn direction_maps_to_refinement() {
        let inspection = blank_inspection(
            vec![node("dir1", "direction", "Hướng Tây Bắc", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].role, DecisionRole::Refinement);
    }

    #[test]
    fn unknown_kind_is_skipped() {
        let inspection = blank_inspection(
            vec![node("u1", "unknown_kind", "Bla", None)],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert!(stack.is_empty());
    }

    #[test]
    fn sorting_prioritizes_override_before_support() {
        let inspection = blank_inspection(
            vec![
                node("hd1", "hoang_dao_hour", "Giờ Tý", None),
                node("tab1", "taboo", "Taboo", Some("hard")),
                node("dir1", "direction", "Hướng Tây", None),
                node("xh1", "xung_hop", "Xung Thân", None),
                node("s1", "star", "cát tinh A", None),
            ],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 5);
        assert_eq!(stack[0].role, DecisionRole::Override);
        assert_eq!(stack[1].role, DecisionRole::Conflict);
        assert_eq!(stack[2].role, DecisionRole::Support);
        assert_eq!(stack[3].role, DecisionRole::Support);
        assert_eq!(stack[4].role, DecisionRole::Refinement);
    }

    #[test]
    fn sorting_prioritizes_conflict_before_resistance() {
        let inspection = blank_inspection(
            vec![
                node("t1", "truc", "Trực Phá", Some("hung")),
                node("xh1", "xung_hop", "Xung Thân", None),
            ],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack[0].role, DecisionRole::Conflict);
        assert_eq!(stack[1].role, DecisionRole::Resistance);
    }

    #[test]
    fn outgoing_targets_populated_from_edges() {
        let inspection = blank_inspection(
            vec![
                node("tab1", "taboo", "Taboo", Some("hard")),
                node("act1", "activity", "Khai trương", None),
            ],
            vec![edge("e1", "tab1", "act1", "targets_activity")],
        );
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack.len(), 1);
        assert_eq!(stack[0].outgoing_targets.len(), 1);
        assert_eq!(stack[0].outgoing_targets[0].target_label, "Khai trương");
        assert_eq!(stack[0].outgoing_targets[0].edge_label, "targets_activity");
    }

    #[test]
    fn provenance_carried_through() {
        use amlich_core::ReasoningEvidenceSourceFamily;
        let prov = ReasoningEvidenceEnvelope {
            source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
            source_id: "truc_table".to_string(),
            method: "lookup".to_string(),
            note: None,
        };
        let mut n = node("t1", "truc", "Trực Kiên Tạo", Some("cat"));
        n.provenance = vec![prov.clone()];
        let inspection = blank_inspection(vec![n], vec![]);
        let stack = extract_decision_stack(&inspection);
        assert_eq!(stack[0].provenance.len(), 1);
        assert_eq!(stack[0].provenance[0].source_id, "truc_table");
    }

    #[test]
    fn impact_derived_from_severity() {
        let inspection = blank_inspection(
            vec![
                node("t1", "truc", "Trực A", Some("cat")),
                node("t2", "truc", "Trực B", Some("hung")),
                node("tab1", "taboo", "Taboo", Some("hard")),
            ],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        let impacts: Vec<_> = stack.iter().map(|e| e.impact.clone()).collect();
        assert_eq!(impacts[0], Some("cấm kỵ nặng".to_string()));
        assert!(impacts.iter().any(|i| i == &Some("tích cực".to_string())));
        assert!(impacts.iter().any(|i| i == &Some("bất lợi".to_string())));
    }

    #[test]
    fn empty_inspection_returns_empty_stack() {
        let inspection = blank_inspection(vec![], vec![]);
        let stack = extract_decision_stack(&inspection);
        assert!(stack.is_empty());
    }

    #[test]
    fn only_non_decision_nodes_returns_empty() {
        let inspection = blank_inspection(
            vec![
                node("a1", "activity", "Khai trương", None),
                node("d1", "day_canchi", "Giáp Tý", None),
            ],
            vec![],
        );
        let stack = extract_decision_stack(&inspection);
        assert!(stack.is_empty());
    }
}
