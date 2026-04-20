use super::types::{InterpretedAxis, ReasoningNodeSeverity, interpret_severity};

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
    summary_vi: &str,
) -> Option<ReasoningNodeSeverity> {
    let concept_key = match node_id {
        "fact.day.truc" => "truc",
        "fact.day.day_deity" => "day_deity",
        "fact.day.taboos" => "taboo",
        "fact.day.nhi_thap_bat_tu" => "star",
        "fact.day.hoang_dao_hours" => "hoang_dao_hours",
        "fact.day.xung_hop" => "xung_hop",
        _ => return None,
    };
    interpret_severity(concept_key, severity, summary_vi)
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
