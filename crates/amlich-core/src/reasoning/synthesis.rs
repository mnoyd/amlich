use crate::DaySnapshot;

use super::{
    assemble_action_vector, build_fact_graph, derive_interpreted_signals, ActionId,
    DecisionConfidence, InitiationOpeningDecision, InitiationOpeningVector, PersonalReasoningInput,
    RecommendationBucket,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SynthesisSemantic {
    OverrideAvoid,
    OverrideCautious,
    ConflictedCautious,
    ResistanceLedCautious,
    FavorableClear,
    FavorableContextual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SynthesisState {
    recommendation_bucket: RecommendationBucket,
    confidence: DecisionConfidence,
    context_is_clear: bool,
    semantic: SynthesisSemantic,
}

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
        .fold(Vec::new(), |mut acc, factor| {
            if !acc.contains(&factor) {
                acc.push(factor);
            }
            acc
        });

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
    let state = synthesize_state(&vector, &override_factors, &conflict_notes);

    let primary_conclusion = synthesize_conclusion(
        &state,
        &vector,
        &strongest_supports,
        &strongest_resistances,
        &override_factors,
    );
    let suggested_hours = synthesize_hour_refinements(snapshot, personal_input);
    let suggested_directions = synthesize_direction_refinements(snapshot, personal_input);

    Ok(InitiationOpeningDecision {
        primary_conclusion,
        recommendation_bucket: state.recommendation_bucket,
        strongest_supports,
        strongest_resistances,
        override_factors,
        conflict_notes,
        confidence: state.confidence,
        context_is_clear: state.context_is_clear,
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

fn synthesize_state(
    vector: &InitiationOpeningVector,
    override_factors: &[String],
    conflict_notes: &[String],
) -> SynthesisState {
    let context_is_clear =
        conflict_notes.is_empty() && vector.context_clarity > 0.0 && override_factors.is_empty();
    let semantic = synthesize_semantic(vector, override_factors, conflict_notes, context_is_clear);

    SynthesisState {
        recommendation_bucket: semantic_bucket(semantic),
        confidence: semantic_confidence(semantic, vector),
        context_is_clear,
        semantic,
    }
}

fn synthesize_semantic(
    vector: &InitiationOpeningVector,
    override_factors: &[String],
    conflict_notes: &[String],
    context_is_clear: bool,
) -> SynthesisSemantic {
    if !override_factors.is_empty() {
        let is_single_taboo_pressure = override_factors.len() == 1
            && override_factors
                .first()
                .is_some_and(|factor| factor.starts_with("Kiêng/kỵ:"))
            && vector.support <= 1.0
            && !conflict_notes.is_empty();

        if is_single_taboo_pressure {
            return SynthesisSemantic::OverrideCautious;
        }

        return SynthesisSemantic::OverrideAvoid;
    }

    if vector.support > 0.0 && vector.resistance > 0.0 && !conflict_notes.is_empty() {
        return SynthesisSemantic::ConflictedCautious;
    }

    if vector.resistance > vector.support {
        return SynthesisSemantic::ResistanceLedCautious;
    }

    if context_is_clear {
        SynthesisSemantic::FavorableClear
    } else {
        SynthesisSemantic::FavorableContextual
    }
}

fn semantic_bucket(semantic: SynthesisSemantic) -> RecommendationBucket {
    match semantic {
        SynthesisSemantic::OverrideAvoid => RecommendationBucket::Avoid,
        SynthesisSemantic::OverrideCautious
        | SynthesisSemantic::ConflictedCautious
        | SynthesisSemantic::ResistanceLedCautious => RecommendationBucket::Cautious,
        SynthesisSemantic::FavorableClear | SynthesisSemantic::FavorableContextual => {
            RecommendationBucket::Favorable
        }
    }
}

fn semantic_confidence(
    semantic: SynthesisSemantic,
    vector: &InitiationOpeningVector,
) -> DecisionConfidence {
    match semantic {
        SynthesisSemantic::OverrideAvoid | SynthesisSemantic::OverrideCautious => {
            DecisionConfidence::High
        }
        SynthesisSemantic::ConflictedCautious
        | SynthesisSemantic::ResistanceLedCautious
        | SynthesisSemantic::FavorableContextual => DecisionConfidence::Low,
        SynthesisSemantic::FavorableClear => {
            if vector.support >= 2.0 && vector.resistance == 0.0 {
                DecisionConfidence::High
            } else {
                DecisionConfidence::Medium
            }
        }
    }
}

fn synthesize_conclusion(
    state: &SynthesisState,
    vector: &InitiationOpeningVector,
    strongest_supports: &[String],
    strongest_resistances: &[String],
    override_factors: &[String],
) -> String {
    match state.semantic {
        SynthesisSemantic::OverrideAvoid => format!(
            "Không nên khởi sự/mở việc vì có yếu tố cấm kỵ nổi bật: {}",
            override_factors.join(", ")
        ),
        SynthesisSemantic::OverrideCautious => format!(
            "Bối cảnh khởi sự còn vướng yếu tố kiêng/kỵ nên chỉ có thể cân nhắc rất thận trọng: {}",
            override_factors.join(", ")
        ),
        SynthesisSemantic::ConflictedCautious => format!(
            "Bối cảnh khởi sự còn trái chiều nên cần giữ thế thận trọng: thuận {} nhưng vẫn có lực cản {}",
            strongest_supports
                .first()
                .cloned()
                .unwrap_or_else(|| format!("{:.0} tín hiệu thuận", vector.support)),
            strongest_resistances
                .first()
                .cloned()
                .unwrap_or_else(|| format!("{:.0} tín hiệu cản", vector.resistance))
        ),
        SynthesisSemantic::ResistanceLedCautious => format!(
            "Có thể cân nhắc rất thận trọng vì lực cản đang nhỉnh hơn lực thuận ({:.0} vs {:.0})",
            vector.resistance, vector.support
        ),
        SynthesisSemantic::FavorableClear => format!(
            "Có thể khởi sự/mở việc, nổi bật là {}",
            strongest_supports
                .first()
                .cloned()
                .unwrap_or_else(|| "nền ngày đang khá thuận".to_string())
        ),
        SynthesisSemantic::FavorableContextual => {
            "Có tín hiệu thuận cho khởi sự/mở việc nhưng vẫn cần đọc bối cảnh tổng thể"
                .to_string()
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
