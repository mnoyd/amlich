use crate::{
    almanac::types::{DayDeityClassification, DayFortune, DayStars, DayTaboo},
    DaySnapshot,
};

use super::{
    ActionId, NodeKind, PersonalReasoningInput, ReasoningEvidenceEnvelope,
    ReasoningEvidenceSourceFamily, ReasoningGraph, ReasoningNode,
};

pub fn build_fact_graph(
    action_id: ActionId,
    snapshot: &DaySnapshot,
    personal_context: Option<&PersonalReasoningInput>,
) -> Result<ReasoningGraph, String> {
    let mut graph = ReasoningGraph::new(action_id);

    graph.nodes.push(ReasoningNode {
        id: "fact.day.solar_term".to_string(),
        kind: NodeKind::Fact,
        summary_vi: snapshot.context.tiet_khi.name.clone(),
        severity: None,
        evidence: vec![snapshot_evidence("snapshot.context.tiet_khi", "context.tiet_khi")],
    });
    graph.nodes.push(ReasoningNode {
        id: "fact.day.truc".to_string(),
        kind: NodeKind::Fact,
        summary_vi: format!("Trực {}", snapshot.day_fortune.truc.name),
        severity: Some(snapshot.day_fortune.truc.quality.clone()),
        evidence: vec![snapshot_evidence("snapshot.day_fortune.truc", "day_fortune.truc")],
    });
    graph.nodes.push(ReasoningNode {
        id: "fact.day.nhi_thap_bat_tu".to_string(),
        kind: NodeKind::Fact,
        summary_vi: summarize_stars(&snapshot.day_fortune.stars),
        severity: None,
        evidence: vec![snapshot_evidence("snapshot.day_fortune.stars", "day_fortune.stars")],
    });
    graph.nodes.push(ReasoningNode {
        id: "fact.day.day_deity".to_string(),
        kind: NodeKind::Fact,
        summary_vi: summarize_day_deity(&snapshot.day_fortune),
        severity: snapshot
            .day_fortune
            .day_deity
            .as_ref()
            .map(|deity| match deity.classification {
                DayDeityClassification::HoangDao => "hoang_dao".to_string(),
                DayDeityClassification::HacDao => "hac_dao".to_string(),
            }),
        evidence: vec![snapshot_evidence(
            "snapshot.day_fortune.day_deity",
            "day_fortune.day_deity",
        )],
    });
    graph.nodes.push(ReasoningNode {
        id: "fact.day.taboos".to_string(),
        kind: NodeKind::Fact,
        summary_vi: summarize_taboos(&snapshot.day_fortune.taboos),
        severity: taboo_severity(&snapshot.day_fortune.taboos),
        evidence: snapshot
            .day_fortune
            .taboos
            .iter()
            .take(3)
            .map(|taboo| almanac_rule_evidence(&taboo.rule_id))
            .collect(),
    });
    graph.nodes.push(ReasoningNode {
        id: "fact.day.xung_hop".to_string(),
        kind: NodeKind::Fact,
        summary_vi: format!(
            "Xung {}{}",
            snapshot.day_fortune.conflict.opposing_chi,
            snapshot
                .day_fortune
                .xung_hop
                .liu_he
                .as_ref()
                .map(|partner| format!(", hợp {partner}"))
                .unwrap_or_default()
        ),
        severity: None,
        evidence: vec![
            snapshot_evidence("snapshot.day_fortune.conflict", "day_fortune.conflict"),
            snapshot_evidence("snapshot.day_fortune.xung_hop", "day_fortune.xung_hop"),
        ],
    });
    graph.nodes.push(ReasoningNode {
        id: "fact.day.travel_directions".to_string(),
        kind: NodeKind::Fact,
        summary_vi: format!(
            "Xuất hành {}, Tài Thần {}, Hỷ Thần {}",
            snapshot.day_fortune.travel.xuat_hanh_huong,
            snapshot.day_fortune.travel.tai_than,
            snapshot.day_fortune.travel.hy_than
        ),
        severity: None,
        evidence: vec![snapshot_evidence("snapshot.day_fortune.travel", "day_fortune.travel")],
    });
    graph.nodes.push(ReasoningNode {
        id: "fact.day.hoang_dao_hours".to_string(),
        kind: NodeKind::Fact,
        summary_vi: snapshot.context.gio_hoang_dao.summary.clone(),
        severity: Some(snapshot.context.gio_hoang_dao.good_hour_count.to_string()),
        evidence: vec![snapshot_evidence(
            "snapshot.context.gio_hoang_dao",
            "context.gio_hoang_dao",
        )],
    });

    if let Some(personal_context) = personal_context {
        graph
            .nodes
            .extend(personal_context.build_fact_nodes(snapshot)?);
    }

    Ok(graph)
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

fn summarize_stars(stars: &DayStars) -> String {
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

fn summarize_day_deity(day_fortune: &DayFortune) -> String {
    match &day_fortune.day_deity {
        Some(deity) => deity.name.clone(),
        None => "Không có thần sát ngày".to_string(),
    }
}

fn summarize_taboos(taboos: &[DayTaboo]) -> String {
    if taboos.is_empty() {
        return "Không có điều kiêng kỵ nổi bật".to_string();
    }

    format!(
        "Kiêng/kỵ: {}",
        taboos
            .iter()
            .map(|taboo| taboo.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn taboo_severity(taboos: &[DayTaboo]) -> Option<String> {
    if taboos.iter().any(|taboo| taboo.severity == "hard") {
        return Some("hard".to_string());
    }

    if taboos.iter().any(|taboo| taboo.severity == "soft") {
        return Some("soft".to_string());
    }

    None
}
