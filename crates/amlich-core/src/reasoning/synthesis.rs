use crate::DaySnapshot;

use super::{
    assemble_action_vector, build_fact_graph, derive_interpreted_signals, ActionId,
    DecisionConfidence, InitiationOpeningDecision, InitiationOpeningVector, PersonalReasoningInput,
    RecommendationBucket,
};

pub fn build_initiation_opening_decision(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<InitiationOpeningDecision, String> {
    let graph = build_fact_graph(ActionId::InitiationOpening, snapshot, personal_input)?;
    let graph = derive_interpreted_signals(graph)?;
    let vector = assemble_action_vector(&graph)?;

    let override_factors = graph
        .edges
        .iter()
        .filter(|edge| edge.effect.is_override())
        .filter_map(|edge| {
            graph
                .nodes
                .iter()
                .find(|node| node.id == edge.from_node_id)
                .map(|node| node.summary_vi.clone())
                .or_else(|| Some(edge.from_node_id.clone()))
        })
        .collect::<Vec<_>>();

    let conflict_notes = graph
        .edges
        .iter()
        .filter(|edge| matches!(edge.effect, super::EdgeEffect::ConflictsWith))
        .map(|edge| format!("{} tạo tín hiệu mâu thuẫn", edge.from_node_id))
        .collect::<Vec<_>>();

    let strongest_supports =
        collect_notes(vector.strongest_support_note.clone(), vector.support > 0.0);
    let strongest_resistances = collect_notes(
        vector.strongest_resistance_note.clone(),
        vector.resistance > 0.0 || !override_factors.is_empty(),
    );
    let context_is_clear =
        conflict_notes.is_empty() && vector.context_clarity > 0.0 && override_factors.is_empty();

    let recommendation_bucket = synthesize_bucket(&vector, &override_factors, &conflict_notes);
    let confidence = synthesize_confidence(
        &vector,
        &override_factors,
        &conflict_notes,
        context_is_clear,
    );
    let primary_conclusion = synthesize_conclusion(
        recommendation_bucket,
        &vector,
        &strongest_supports,
        &strongest_resistances,
        &override_factors,
        context_is_clear,
    );
    let suggested_hours = synthesize_hour_refinements(snapshot, personal_input);
    let suggested_directions = synthesize_direction_refinements(snapshot, personal_input);

    Ok(InitiationOpeningDecision {
        primary_conclusion,
        recommendation_bucket,
        strongest_supports,
        strongest_resistances,
        override_factors,
        conflict_notes,
        confidence,
        context_is_clear,
        suggested_hours,
        suggested_directions,
    })
}

fn collect_notes(note: Option<String>, include: bool) -> Vec<String> {
    if include {
        note.into_iter().collect()
    } else {
        Vec::new()
    }
}

fn synthesize_bucket(
    vector: &InitiationOpeningVector,
    override_factors: &[String],
    conflict_notes: &[String],
) -> RecommendationBucket {
    if !override_factors.is_empty() {
        return RecommendationBucket::Avoid;
    }

    if vector.support > 0.0 && vector.resistance > 0.0 && !conflict_notes.is_empty() {
        return RecommendationBucket::Mixed;
    }

    if vector.resistance > vector.support {
        return RecommendationBucket::Cautious;
    }

    RecommendationBucket::Favorable
}

fn synthesize_confidence(
    vector: &InitiationOpeningVector,
    override_factors: &[String],
    conflict_notes: &[String],
    context_is_clear: bool,
) -> DecisionConfidence {
    if !override_factors.is_empty() {
        return DecisionConfidence::High;
    }

    if !context_is_clear || !conflict_notes.is_empty() {
        return DecisionConfidence::Low;
    }

    if vector.support >= 2.0 && vector.resistance == 0.0 {
        DecisionConfidence::High
    } else {
        DecisionConfidence::Medium
    }
}

fn synthesize_conclusion(
    bucket: RecommendationBucket,
    vector: &InitiationOpeningVector,
    strongest_supports: &[String],
    strongest_resistances: &[String],
    override_factors: &[String],
    context_is_clear: bool,
) -> String {
    match bucket {
        RecommendationBucket::Avoid => format!(
            "Không nên khởi sự/mở việc vì có yếu tố cấm kỵ nổi bật: {}",
            override_factors.join(", ")
        ),
        RecommendationBucket::Mixed => format!(
            "Bối cảnh khởi sự đang trái chiều: thuận {} nhưng vẫn có lực cản {}",
            strongest_supports
                .first()
                .cloned()
                .unwrap_or_else(|| format!("{:.0} tín hiệu thuận", vector.support)),
            strongest_resistances
                .first()
                .cloned()
                .unwrap_or_else(|| format!("{:.0} tín hiệu cản", vector.resistance))
        ),
        RecommendationBucket::Cautious => format!(
            "Có thể cân nhắc rất thận trọng vì lực cản đang nhỉnh hơn lực thuận ({:.0} vs {:.0})",
            vector.resistance, vector.support
        ),
        RecommendationBucket::Favorable => {
            if context_is_clear {
                format!(
                    "Có thể khởi sự/mở việc, nổi bật là {}",
                    strongest_supports
                        .first()
                        .cloned()
                        .unwrap_or_else(|| "nền ngày đang khá thuận".to_string())
                )
            } else {
                "Có tín hiệu thuận cho khởi sự/mở việc nhưng vẫn cần đọc bối cảnh tổng thể"
                    .to_string()
            }
        }
    }
}

fn synthesize_hour_refinements(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Vec<String> {
    if let Some(personal_input) = personal_input {
        let personal_hours = personal_input.suggested_hours(snapshot);
        if !personal_hours.is_empty() {
            return personal_hours;
        }
    }

    snapshot
        .context
        .gio_hoang_dao
        .good_hours
        .iter()
        .take(3)
        .map(|hour| {
            format!(
                "Nếu vẫn tiến hành, ưu tiên giờ {} ({})",
                hour.hour_chi, hour.time_range
            )
        })
        .collect()
}

fn synthesize_direction_refinements(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Vec<String> {
    if let Some(personal_input) = personal_input {
        let personal_directions = personal_input.suggested_directions(snapshot);
        if !personal_directions.is_empty() {
            return personal_directions;
        }
    }

    [
        ("Tài Thần", snapshot.day_fortune.travel.tai_than.as_str()),
        ("Hỷ Thần", snapshot.day_fortune.travel.hy_than.as_str()),
        (
            "Xuất hành",
            snapshot.day_fortune.travel.xuat_hanh_huong.as_str(),
        ),
    ]
    .into_iter()
    .map(|(label, direction)| {
        format!(
            "Nếu vẫn tiến hành, ưu tiên hướng {} theo {}",
            direction, label
        )
    })
    .collect()
}
