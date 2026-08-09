use super::types::{InterpretedAxis, ReasoningNodeSeverity};

pub(super) fn axis_for_node(node_id: &str) -> Option<InterpretedAxis> {
    match node_id {
        "signal.support" => Some(InterpretedAxis::Support),
        "signal.resistance" => Some(InterpretedAxis::Resistance),
        "signal.stability" => Some(InterpretedAxis::Stability),
        "signal.personal_alignment" => Some(InterpretedAxis::PersonalAlignment),
        "signal.timing_fit" => Some(InterpretedAxis::TimingFit),
        "signal.context_clarity" => Some(InterpretedAxis::ContextClarity),
        _ => None,
    }
}

pub(super) fn severity_for_node(
    node_id: &str,
    severity: Option<&str>,
) -> Option<ReasoningNodeSeverity> {
    match node_id {
        "fact.day.truc" => match severity {
            Some("cat") => Some(ReasoningNodeSeverity::Auspicious),
            Some("hung") => Some(ReasoningNodeSeverity::Inauspicious),
            _ => None,
        },
        "fact.day.day_deity" => match severity {
            Some("hoang_dao") => Some(ReasoningNodeSeverity::HoangDao),
            Some("hac_dao") => Some(ReasoningNodeSeverity::HacDao),
            _ => None,
        },
        "fact.day.taboos" => match severity {
            Some("hard") => Some(ReasoningNodeSeverity::HardTaboo),
            Some("soft") => Some(ReasoningNodeSeverity::SoftTaboo),
            _ => None,
        },
        "fact.day.hoang_dao_hours" if severity.is_some() => Some(ReasoningNodeSeverity::Auspicious),
        _ => None,
    }
}

pub(super) fn tags_for_node(node_id: &str) -> Vec<String> {
    let mut tags = Vec::new();
    if node_id.starts_with("fact.personal.") {
        tags.push("personal".to_string());
    }
    if node_id.starts_with("fact.day.") {
        tags.push("day".to_string());
    }
    if node_id.starts_with("signal.") {
        tags.push("signal".to_string());
    }
    match node_id {
        "fact.day.taboos" | "fact.day.xung_hop" | "signal.resistance" => {
            tags.push("resistance".to_string());
        }
        "fact.day.truc"
        | "fact.day.day_deity"
        | "fact.day.nhi_thap_bat_tu"
        | "fact.day.hoang_dao_hours"
        | "signal.support" => {
            tags.push("support".to_string());
        }
        "fact.day.travel_directions" | "signal.timing_fit" => {
            tags.push("timing".to_string());
        }
        "fact.day.solar_term" => {
            tags.push("context".to_string());
        }
        _ => {}
    }
    tags
}
