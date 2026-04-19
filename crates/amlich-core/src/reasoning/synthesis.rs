use crate::DaySnapshot;

use super::{
    assemble_action_vector, build_fact_graph, derive_interpreted_signals, export_reasoning_graph,
    ActionId, DecisionConfidence, InitiationOpeningDecision, InitiationOpeningReasoningBundle,
    InitiationOpeningVector, InterpretedAxis, PersonalReasoningInput, ReasoningAxisScore,
    ReasoningConclusionSemantic, ReasoningNote, RecommendationBucket,
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

impl SynthesisSemantic {
    fn export(self) -> ReasoningConclusionSemantic {
        match self {
            SynthesisSemantic::OverrideAvoid => ReasoningConclusionSemantic::OverrideAvoid,
            SynthesisSemantic::OverrideCautious => ReasoningConclusionSemantic::OverrideCautious,
            SynthesisSemantic::ConflictedCautious => {
                ReasoningConclusionSemantic::ConflictedCautious
            }
            SynthesisSemantic::ResistanceLedCautious => {
                ReasoningConclusionSemantic::ResistanceLedCautious
            }
            SynthesisSemantic::FavorableClear => ReasoningConclusionSemantic::FavorableClear,
            SynthesisSemantic::FavorableContextual => {
                ReasoningConclusionSemantic::FavorableContextual
            }
        }
    }
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
    Ok(build_initiation_opening_reasoning_bundle(snapshot, personal_input)?.decision)
}

pub fn build_initiation_opening_reasoning_bundle(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<InitiationOpeningReasoningBundle, String> {
    let graph = build_fact_graph(ActionId::InitiationOpening, snapshot, personal_input)?;
    let graph = derive_interpreted_signals(graph)?;
    let vector = assemble_action_vector(&graph)?;

    let override_factors = graph
        .edges
        .iter()
        .filter(|edge| edge.effect.is_override())
        .filter_map(|edge| note_from_node_id(&graph, &edge.from_node_id, &["override"]))
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
        .map(|edge| {
            note_from_node_id(&graph, &edge.from_node_id, &["conflict"]).unwrap_or(ReasoningNote {
                node_id: Some(edge.from_node_id.clone()),
                summary_vi: format!("{} tạo tín hiệu mâu thuẫn", edge.from_node_id),
                tags: vec!["conflict".to_string()],
            })
        })
        .collect::<Vec<_>>();

    let strongest_supports = collect_notes(
        vector.strongest_support_id.clone(),
        vector.strongest_support_note.clone(),
        &["support"],
        vector.support > 0.0,
    );
    let strongest_resistances = collect_notes(
        vector.strongest_resistance_id.clone(),
        vector.strongest_resistance_note.clone(),
        &["resistance"],
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

    let decision = InitiationOpeningDecision {
        primary_conclusion: primary_conclusion.clone(),
        recommendation_bucket: state.recommendation_bucket,
        strongest_supports: strongest_supports
            .iter()
            .map(|note| note.summary_vi.clone())
            .collect(),
        strongest_resistances: strongest_resistances
            .iter()
            .map(|note| note.summary_vi.clone())
            .collect(),
        override_factors: override_factors
            .iter()
            .map(|note| note.summary_vi.clone())
            .collect(),
        conflict_notes: conflict_notes
            .iter()
            .map(|note| note.summary_vi.clone())
            .collect(),
        confidence: state.confidence,
        context_is_clear: state.context_is_clear,
        suggested_hours: suggested_hours.clone(),
        suggested_directions: suggested_directions.clone(),
    };

    Ok(InitiationOpeningReasoningBundle {
        decision,
        decision_export: super::InitiationOpeningDecisionExport {
            primary_conclusion: primary_conclusion.clone(),
            recommendation_bucket: state.recommendation_bucket,
            confidence: state.confidence,
            context_is_clear: state.context_is_clear,
            semantic: state.semantic.export(),
            strongest_supports,
            strongest_resistances,
            override_factors,
            conflict_notes,
            suggested_hours: suggested_hours.clone(),
            suggested_directions: suggested_directions.clone(),
            axis_scores: axis_scores(&vector),
        },
        graph: export_reasoning_graph(&graph),
    })
}

fn collect_notes(
    node_id: Option<String>,
    note: Option<String>,
    tags: &[&str],
    include: bool,
) -> Vec<ReasoningNote> {
    if include {
        note.into_iter()
            .map(|summary_vi| ReasoningNote {
                node_id: node_id.clone(),
                summary_vi,
                tags: tags.iter().map(|tag| (*tag).to_string()).collect(),
            })
            .collect()
    } else {
        Vec::new()
    }
}

fn synthesize_state(
    vector: &InitiationOpeningVector,
    override_factors: &[ReasoningNote],
    conflict_notes: &[ReasoningNote],
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
    override_factors: &[ReasoningNote],
    conflict_notes: &[ReasoningNote],
    context_is_clear: bool,
) -> SynthesisSemantic {
    if !override_factors.is_empty() {
        let is_single_taboo_pressure = override_factors.len() == 1
            && override_factors
                .first()
                .is_some_and(|factor| factor.summary_vi.starts_with("Kiêng/kỵ:"))
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
    strongest_supports: &[ReasoningNote],
    strongest_resistances: &[ReasoningNote],
    override_factors: &[ReasoningNote],
) -> String {
    match state.semantic {
        SynthesisSemantic::OverrideAvoid => format!(
            "Không nên khởi sự/mở việc vì có yếu tố cấm kỵ nổi bật: {}",
            join_notes(override_factors)
        ),
        SynthesisSemantic::OverrideCautious => format!(
            "Bối cảnh khởi sự còn vướng yếu tố kiêng/kỵ nên chỉ có thể cân nhắc rất thận trọng: {}",
            join_notes(override_factors)
        ),
        SynthesisSemantic::ConflictedCautious => format!(
            "Bối cảnh khởi sự còn trái chiều nên cần giữ thế thận trọng: thuận {} nhưng vẫn có lực cản {}",
            strongest_supports
                .first()
                .map(|note| note.summary_vi.clone())
                .unwrap_or_else(|| format!("{:.0} tín hiệu thuận", vector.support)),
            strongest_resistances
                .first()
                .map(|note| note.summary_vi.clone())
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
                .map(|note| note.summary_vi.clone())
                .unwrap_or_else(|| "nền ngày đang khá thuận".to_string())
        ),
        SynthesisSemantic::FavorableContextual => {
            "Có tín hiệu thuận cho khởi sự/mở việc nhưng vẫn cần đọc bối cảnh tổng thể"
                .to_string()
        }
    }
}

fn note_from_node_id(
    graph: &super::ReasoningGraph,
    node_id: &str,
    tags: &[&str],
) -> Option<ReasoningNote> {
    graph
        .nodes
        .iter()
        .find(|node| node.id == node_id)
        .map(|node| ReasoningNote {
            node_id: Some(node.id.clone()),
            summary_vi: node.summary_vi.clone(),
            tags: tags.iter().map(|tag| (*tag).to_string()).collect(),
        })
        .or_else(|| {
            Some(ReasoningNote {
                node_id: Some(node_id.to_string()),
                summary_vi: node_id.to_string(),
                tags: tags.iter().map(|tag| (*tag).to_string()).collect(),
            })
        })
}

fn join_notes(notes: &[ReasoningNote]) -> String {
    notes
        .iter()
        .map(|note| note.summary_vi.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

fn axis_scores(vector: &InitiationOpeningVector) -> Vec<ReasoningAxisScore> {
    vec![
        score_axis(
            InterpretedAxis::Support,
            vector.support,
            vector.strongest_support_id.clone(),
            vector.strongest_support_note.clone(),
        ),
        score_axis(
            InterpretedAxis::Resistance,
            vector.resistance,
            vector.strongest_resistance_id.clone(),
            vector.strongest_resistance_note.clone(),
        ),
        score_axis(InterpretedAxis::Stability, vector.stability, None, None),
        score_axis(
            InterpretedAxis::PersonalAlignment,
            vector.personal_alignment,
            None,
            None,
        ),
        score_axis(InterpretedAxis::TimingFit, vector.timing_fit, None, None),
        score_axis(
            InterpretedAxis::ContextClarity,
            vector.context_clarity,
            None,
            None,
        ),
    ]
}

fn score_axis(
    axis: InterpretedAxis,
    score: f32,
    strongest_node_id: Option<String>,
    strongest_summary_vi: Option<String>,
) -> ReasoningAxisScore {
    ReasoningAxisScore {
        axis,
        score,
        strongest_node_id,
        strongest_summary_vi,
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
