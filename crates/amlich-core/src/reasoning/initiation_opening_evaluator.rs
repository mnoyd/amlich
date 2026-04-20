use crate::semantic_graph::{NodeConcept, SemanticGraph, SemanticNode};

use crate::almanac::recommendation::evidence::{BaseDirection, collect_truc_hits};
use crate::insight_data::find_truc_insight;
use crate::reasoning::action_evaluator::{ActionEvaluation, ActionEvaluator};
use crate::reasoning::types::{
    ActionId, DecisionConfidence, InterpretedAxis, ReasoningAxisScore,
    ReasoningConclusionSemantic, ReasoningNodeSeverity, ReasoningNote, RecommendationBucket,
    interpret_severity,
};
use crate::DaySnapshot;
use crate::reasoning::PersonalReasoningInput;

pub struct InitiationOpeningEvaluator;

#[derive(Debug, Clone, Copy, Default)]
struct TrucOpeningSignal {
    opening_avoid_count: usize,
}

impl InitiationOpeningEvaluator {
    pub fn new() -> Self {
        Self
    }

    fn extract_support_evidence(&self, graph: &SemanticGraph, _snapshot: &DaySnapshot) -> Vec<ReasoningNote> {
        let mut notes = Vec::new();

        if let Some(truc_node) = self.find_node_by_concept(graph, NodeConcept::Truc) {
            if let Some(quality) = &truc_node.severity {
                if quality == "cat" {
                    notes.push(ReasoningNote {
                        node_id: Some(truc_node.node_id.clone()),
                        summary_vi: truc_node.summary_vi.clone(),
                        tags: vec!["support".to_string(), "truc".to_string()],
                    });
                }
            }
        }

        if let Some(deity_node) = self.find_node_by_concept(graph, NodeConcept::DayDeity) {
            if let Some(severity) = &deity_node.severity {
                if severity == "hoang_dao" {
                    notes.push(ReasoningNote {
                        node_id: Some(deity_node.node_id.clone()),
                        summary_vi: deity_node.summary_vi.clone(),
                        tags: vec!["support".to_string(), "day_deity".to_string()],
                    });
                }
            }
        }

        if let Some(star_node) = self.find_node_by_concept(graph, NodeConcept::Star) {
            if self.is_star_supportive(star_node) {
                notes.push(ReasoningNote {
                    node_id: Some(star_node.node_id.clone()),
                    summary_vi: star_node.summary_vi.clone(),
                    tags: vec!["support".to_string(), "star".to_string()],
                });
            }
        }

        notes
    }

    fn extract_resistance_evidence(&self, graph: &SemanticGraph) -> Vec<ReasoningNote> {
        let mut notes = Vec::new();

        if let Some((truc_node, signal)) = self.truc_opening_signal(graph) {
            if signal.opening_avoid_count > 0 {
                notes.push(ReasoningNote {
                    node_id: Some(truc_node.node_id.clone()),
                    summary_vi: truc_node.summary_vi.clone(),
                    tags: vec!["resistance".to_string(), "truc".to_string()],
                });
            }
        }

        if let Some(xung_hop_node) = self.find_node_by_concept(graph, NodeConcept::XungHop) {
            if xung_hop_node.summary_vi.contains("Xung") && !xung_hop_node.summary_vi.contains(", hợp ") {
                notes.push(ReasoningNote {
                    node_id: Some(xung_hop_node.node_id.clone()),
                    summary_vi: xung_hop_node.summary_vi.clone(),
                    tags: vec!["resistance".to_string(), "xung_hop".to_string()],
                });
            }
        }

        for (_, node) in graph.nodes() {
            if node.concept == NodeConcept::Taboo {
                if node.severity.is_some() {
                    notes.push(ReasoningNote {
                        node_id: Some(node.node_id.clone()),
                        summary_vi: node.summary_vi.clone(),
                        tags: vec!["resistance".to_string(), "taboo".to_string()],
                    });
                }
            }
        }

        if let Some(node) = self.most_unfavorable_direction_row(graph) {
            notes.push(ReasoningNote {
                node_id: Some(node.node_id.clone()),
                summary_vi: format!("Hướng cá nhân có điểm xung: {}", node.summary_vi),
                tags: vec!["resistance".to_string(), "personal_direction".to_string()],
            });
        }

        notes
    }

    fn extract_override_evidence(&self, graph: &SemanticGraph) -> Vec<ReasoningNote> {
        let mut notes = Vec::new();

        for (_, node) in graph.nodes() {
            if node.concept == NodeConcept::Taboo {
                if let Some(ref severity) = node.severity {
                    if severity == "hard" {
                        notes.push(ReasoningNote {
                            node_id: Some(node.node_id.clone()),
                            summary_vi: node.summary_vi.clone(),
                            tags: vec!["override".to_string(), "hard_taboo".to_string()],
                        });
                    }
                }
            }
        }

        notes
    }

    fn has_favorable_fact(&self, graph: &SemanticGraph) -> bool {
        graph.nodes().values().any(|n| {
            let concept_key = match n.concept {
                NodeConcept::Truc => "truc",
                NodeConcept::DayDeity => "day_deity",
                NodeConcept::HoangDaoHour => "hoang_dao_hours",
                NodeConcept::Star => return self.is_star_supportive(n),
                _ => return false,
            };
            interpret_severity(concept_key, n.severity.as_deref(), &n.summary_vi)
                .is_some_and(ReasoningNodeSeverity::is_favorable)
        })
    }

    fn has_unfavorable_fact(&self, graph: &SemanticGraph) -> bool {
        graph.nodes().values().any(|n| match n.concept {
                NodeConcept::Taboo => n.severity.is_some(),
                NodeConcept::InteractionRow => {
                    n.summary_vi.contains(" direction: net=")
                        && self.node_has_tag(n, "unfavorable")
                }
                _ => false,
        })
    }

    fn extract_conflict_evidence(&self, graph: &SemanticGraph) -> Vec<ReasoningNote> {
        let mut notes = Vec::new();

        let has_favorable = self.has_favorable_fact(graph);
        let has_unfavorable = self.has_unfavorable_fact(graph);

        if let Some((truc_node, signal)) = self.truc_opening_signal(graph) {
            if signal.opening_avoid_count > 0 {
                notes.push(ReasoningNote {
                    node_id: Some(truc_node.node_id.clone()),
                    summary_vi: truc_node.summary_vi.clone(),
                    tags: vec!["conflict".to_string(), "truc".to_string()],
                });
            }
        }

        if has_favorable && has_unfavorable {
            notes.push(ReasoningNote {
                node_id: None,
                summary_vi: "Bối cảnh có cả yếu tố thuận và cản trở".to_string(),
                tags: vec!["conflict".to_string()],
            });
        }

        if self.has_mixed_direction_merge(graph) {
            notes.push(ReasoningNote {
                node_id: None,
                summary_vi: "Hướng hợp cá nhân còn phân hóa giữa thuận và nghịch".to_string(),
                tags: vec!["conflict".to_string(), "personal_direction".to_string()],
            });
        }

        notes
    }

    fn has_mixed_direction_merge(&self, graph: &SemanticGraph) -> bool {
        let mut has_favorable = false;
        let mut has_unfavorable = false;

        for node in self.direction_rows(graph) {
            if self.node_has_tag(node, "favorable") {
                has_favorable = true;
            }
            if self.node_has_tag(node, "unfavorable") {
                has_unfavorable = true;
            }
        }

        has_favorable && has_unfavorable
    }

    fn most_unfavorable_direction_row<'a>(
        &self,
        graph: &'a SemanticGraph,
    ) -> Option<&'a SemanticNode> {
        self.direction_rows(graph)
            .filter(|node| self.node_has_tag(node, "unfavorable"))
            .min_by(|left, right| {
                let left_score = self.node_tag_i8(left, "net_score").unwrap_or(0);
                let right_score = self.node_tag_i8(right, "net_score").unwrap_or(0);
                left_score
                    .cmp(&right_score)
                    .then_with(|| left.node_id.cmp(&right.node_id))
            })
    }

    fn direction_rows<'a>(
        &self,
        graph: &'a SemanticGraph,
    ) -> impl Iterator<Item = &'a SemanticNode> {
        graph.nodes().values().filter(|node| {
            node.concept == NodeConcept::InteractionRow
                && node.summary_vi.contains(" direction: net=")
        })
    }

    fn node_tag_i8(&self, node: &SemanticNode, prefix: &str) -> Option<i8> {
        let prefix = format!("{prefix}:");
        node.tags
            .iter()
            .find_map(|tag| tag.strip_prefix(&prefix)?.parse::<i8>().ok())
    }

    fn node_has_tag(&self, node: &SemanticNode, tag: &str) -> bool {
        node.tags.iter().any(|node_tag| node_tag == tag)
    }

    fn score_axis(
        &self,
        graph: &SemanticGraph,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
        axis: InterpretedAxis,
    ) -> ReasoningAxisScore {
        match axis {
            InterpretedAxis::Support => {
                let score = self.extract_support_evidence(graph, snapshot).len() as f32;
                let strongest = self.extract_support_evidence(graph, snapshot).into_iter().next();
                ReasoningAxisScore {
                    axis,
                    score,
                    strongest_node_id: strongest.as_ref().and_then(|n| n.node_id.clone()),
                    strongest_summary_vi: strongest.map(|n| n.summary_vi),
                }
            }
            InterpretedAxis::Resistance => {
                let score = self.extract_resistance_evidence(graph).len() as f32;
                let strongest = self.extract_resistance_evidence(graph).into_iter().next();
                ReasoningAxisScore {
                    axis,
                    score,
                    strongest_node_id: strongest.as_ref().and_then(|n| n.node_id.clone()),
                    strongest_summary_vi: strongest.map(|n| n.summary_vi),
                }
            }
            InterpretedAxis::Stability => {
                let taboo_count = graph.nodes().values()
                    .filter(|n| n.concept == NodeConcept::Taboo)
                    .count() as f32;
                ReasoningAxisScore {
                    axis,
                    score: (3.0 - taboo_count).max(0.0),
                    strongest_node_id: None,
                    strongest_summary_vi: None,
                }
            }
            InterpretedAxis::PersonalAlignment => {
                let score = if personal_input.is_some() { 1.0 } else { 0.0 };
                ReasoningAxisScore {
                    axis,
                    score,
                    strongest_node_id: None,
                    strongest_summary_vi: None,
                }
            }
            InterpretedAxis::TimingFit => {
                let hoang_dao_count = self.find_node_by_concept(graph, NodeConcept::HoangDaoHour)
                    .and_then(|n| n.severity.as_ref())
                    .and_then(|s| s.parse::<usize>().ok())
                    .unwrap_or(0);
                ReasoningAxisScore {
                    axis,
                    score: (hoang_dao_count as f32).min(3.0),
                    strongest_node_id: None,
                    strongest_summary_vi: None,
                }
            }
            InterpretedAxis::ContextClarity => {
                let conflict_count = self.extract_conflict_evidence(graph).len() as f32;
                ReasoningAxisScore {
                    axis,
                    score: (2.0 - conflict_count).max(0.0),
                    strongest_node_id: None,
                    strongest_summary_vi: None,
                }
            }
        }
    }

    fn synthesize_semantic(
        &self,
        support_score: f32,
        resistance_score: f32,
        override_notes: &[ReasoningNote],
        conflict_count: usize,
        context_clarity_score: f32,
    ) -> (ReasoningConclusionSemantic, RecommendationBucket, DecisionConfidence, bool) {
        let override_count = override_notes.len();
        let context_is_clear = conflict_count == 0 && context_clarity_score > 0.0 && override_count == 0;

        let semantic = if override_count > 0 {
            let is_single_taboo_pressure = override_count == 1
                && override_notes
                    .first()
                    .is_some_and(|factor| factor.tags.iter().any(|tag| tag == "hard_taboo"))
                && support_score <= 1.0
                && conflict_count > 0;

            if is_single_taboo_pressure {
                ReasoningConclusionSemantic::OverrideCautious
            } else {
                ReasoningConclusionSemantic::OverrideAvoid
            }
        } else if support_score > 0.0 && resistance_score > 0.0 && conflict_count > 0 {
            ReasoningConclusionSemantic::ConflictedCautious
        } else if resistance_score > support_score {
            ReasoningConclusionSemantic::ResistanceLedCautious
        } else if context_is_clear {
            ReasoningConclusionSemantic::FavorableClear
        } else {
            ReasoningConclusionSemantic::FavorableContextual
        };

        let bucket = match semantic {
            ReasoningConclusionSemantic::OverrideAvoid => RecommendationBucket::Avoid,
            ReasoningConclusionSemantic::OverrideCautious
            | ReasoningConclusionSemantic::ConflictedCautious
            | ReasoningConclusionSemantic::ResistanceLedCautious => RecommendationBucket::Cautious,
            ReasoningConclusionSemantic::FavorableClear | ReasoningConclusionSemantic::FavorableContextual => {
                RecommendationBucket::Favorable
            }
        };

        let confidence = match semantic {
            ReasoningConclusionSemantic::OverrideAvoid | ReasoningConclusionSemantic::OverrideCautious => {
                DecisionConfidence::High
            }
            ReasoningConclusionSemantic::ConflictedCautious
            | ReasoningConclusionSemantic::ResistanceLedCautious
            | ReasoningConclusionSemantic::FavorableContextual => DecisionConfidence::Low,
            ReasoningConclusionSemantic::FavorableClear => {
                if support_score >= 2.0 && resistance_score == 0.0 {
                    DecisionConfidence::High
                } else {
                    DecisionConfidence::Medium
                }
            }
        };

        (semantic, bucket, confidence, context_is_clear)
    }

    fn synthesize_primary_conclusion(
        &self,
        semantic: ReasoningConclusionSemantic,
        _bucket: RecommendationBucket,
        support_score: f32,
        resistance_score: f32,
        override_notes: &[ReasoningNote],
        support_notes: &[ReasoningNote],
        resistance_notes: &[ReasoningNote],
    ) -> String {
        match semantic {
            ReasoningConclusionSemantic::OverrideAvoid => format!(
                "Không nên khởi sự/mở việc vì có yếu tố cấm kỵ nổi bật: {}",
                override_notes.iter().map(|n| n.summary_vi.as_str()).collect::<Vec<_>>().join(", ")
            ),
            ReasoningConclusionSemantic::OverrideCautious => format!(
                "Bối cảnh khởi sự còn vướng yếu tố kiêng/kỵ nên chỉ có thể cân nhắc rất thận trọng: {}",
                override_notes.iter().map(|n| n.summary_vi.as_str()).collect::<Vec<_>>().join(", ")
            ),
            ReasoningConclusionSemantic::ConflictedCautious => format!(
                "Bối cảnh khởi sự còn trái chiều nên cần giữ thế thận trọng: thuận {} nhưng vẫn có lực cản {}",
                support_notes.first().map(|n| n.summary_vi.as_str()).unwrap_or(&format!("{:.0} tín hiệu thuận", support_score)),
                resistance_notes.first().map(|n| n.summary_vi.as_str()).unwrap_or(&format!("{:.0} tín hiệu cản", resistance_score))
            ),
            ReasoningConclusionSemantic::ResistanceLedCautious => format!(
                "Có thể cân nhắc rất thận trọng vì lực cản đang nhỉnh hơn lực thuận ({:.0} vs {:.0})",
                resistance_score, support_score
            ),
            ReasoningConclusionSemantic::FavorableClear => format!(
                "Có thể khởi sự/mở việc, nổi bật là {}",
                support_notes.first().map(|n| n.summary_vi.as_str()).unwrap_or("nền ngày đang khá thuận")
            ),
            ReasoningConclusionSemantic::FavorableContextual => {
                "Có tín hiệu thuận cho khởi sự/mở việc nhưng vẫn cần đọc bối cảnh tổng thể".to_string()
            }
        }
    }

    fn suggested_hours(
        &self,
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
            .map(|hour| format!("Nếu vẫn tiến hành, ưu tiên giờ {} ({})", hour.hour_chi, hour.time_range))
            .collect()
    }

    fn suggested_directions(
        &self,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
    ) -> Vec<String> {
        if let Some(personal_input) = personal_input {
            let personal_directions = personal_input.suggested_directions(snapshot);
            if !personal_directions.is_empty() {
                return personal_directions;
            }
        }

        vec![
            format!(
                "Nếu vẫn tiến hành, ưu tiên hướng {} theo Tài Thần",
                snapshot.day_fortune.travel.tai_than
            ),
            format!(
                "Nếu vẫn tiến hành, ưu tiên hướng {} theo Hỷ Thần",
                snapshot.day_fortune.travel.hy_than
            ),
            format!(
                "Nếu vẫn tiến hành, ưu tiên hướng {} theo Xuất hành",
                snapshot.day_fortune.travel.xuat_hanh_huong
            ),
        ]
    }

    fn find_node_by_concept<'a>(&self, graph: &'a SemanticGraph, concept: NodeConcept) -> Option<&'a SemanticNode> {
        graph.nodes().values().find(|n| n.concept == concept)
    }

    fn is_star_supportive(&self, node: &SemanticNode) -> bool {
        node.summary_vi.contains("cát tinh")
            || node.summary_vi.contains("Nhị thập bát tú")
            || node.summary_vi.starts_with("Ngôi sao chính:")
    }

    fn truc_opening_signal<'a>(
        &self,
        graph: &'a SemanticGraph,
    ) -> Option<(&'a SemanticNode, TrucOpeningSignal)> {
        let truc_node = self.find_node_by_concept(graph, NodeConcept::Truc)?;
        let truc_name = truc_node.summary_vi.strip_prefix("Trực ")?;
        let truc = find_truc_insight(truc_name)?;
        let opening_avoid_count = collect_truc_hits(truc)
            .into_iter()
            .filter(|hit| {
                hit.activity_id == crate::almanac::recommendation::ActivityId::OpeningStart
                    && matches!(hit.direction, BaseDirection::Avoid)
            })
            .count();

        Some((
            truc_node,
            TrucOpeningSignal {
                opening_avoid_count,
            },
        ))
    }
}

impl Default for InitiationOpeningEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionEvaluator for InitiationOpeningEvaluator {
    fn action_id(&self) -> ActionId {
        ActionId::InitiationOpening
    }

    fn select_subgraph(
        &self,
        graph: &SemanticGraph,
        _snapshot: &DaySnapshot,
        _personal_input: Option<&PersonalReasoningInput>,
    ) -> Result<SemanticGraph, String> {
        Ok(graph.clone())
    }

    fn evaluate(
        &self,
        graph: &SemanticGraph,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
    ) -> Result<ActionEvaluation, String> {
        let support_notes = self.extract_support_evidence(graph, snapshot);
        let resistance_notes = self.extract_resistance_evidence(graph);
        let override_notes = self.extract_override_evidence(graph);
        let conflict_notes = self.extract_conflict_evidence(graph);

        let support_score = support_notes.len() as f32;
        let resistance_score = resistance_notes.len() as f32;
        let conflict_count = conflict_notes.len();

        let stability_score = self.score_axis(graph, snapshot, personal_input, InterpretedAxis::Stability);
        let timing_fit_score = self.score_axis(graph, snapshot, personal_input, InterpretedAxis::TimingFit);
        let context_clarity_score = self.score_axis(graph, snapshot, personal_input, InterpretedAxis::ContextClarity);

        let (semantic, bucket, confidence, context_is_clear) = self.synthesize_semantic(
            support_score,
            resistance_score,
            &override_notes,
            conflict_count,
            context_clarity_score.score,
        );

        let primary_conclusion = self.synthesize_primary_conclusion(
            semantic,
            bucket,
            support_score,
            resistance_score,
            &override_notes,
            &support_notes,
            &resistance_notes,
        );

        let suggested_hours = self.suggested_hours(snapshot, personal_input);
        let suggested_directions = self.suggested_directions(snapshot, personal_input);

        let axis_scores = vec![
            ReasoningAxisScore {
                axis: InterpretedAxis::Support,
                score: support_score,
                strongest_node_id: support_notes.first().and_then(|n| n.node_id.clone()),
                strongest_summary_vi: support_notes.first().map(|n| n.summary_vi.clone()),
            },
            ReasoningAxisScore {
                axis: InterpretedAxis::Resistance,
                score: resistance_score,
                strongest_node_id: resistance_notes.first().and_then(|n| n.node_id.clone()),
                strongest_summary_vi: resistance_notes.first().map(|n| n.summary_vi.clone()),
            },
            stability_score,
            ReasoningAxisScore {
                axis: InterpretedAxis::PersonalAlignment,
                score: if personal_input.is_some() { 1.0 } else { 0.0 },
                strongest_node_id: None,
                strongest_summary_vi: None,
            },
            timing_fit_score,
            context_clarity_score,
        ];

        let mut referenced_node_ids = Vec::new();
        for note in support_notes.iter().chain(resistance_notes.iter()).chain(override_notes.iter()) {
            if let Some(ref node_id) = note.node_id {
                referenced_node_ids.push(node_id.clone());
            }
        }

        Ok(ActionEvaluation {
            action_id: ActionId::InitiationOpening,
            bucket,
            confidence,
            semantic,
            context_is_clear,
            primary_conclusion,
            strongest_supports: support_notes,
            strongest_resistances: resistance_notes,
            override_factors: override_notes,
            conflict_notes,
            suggested_hours,
            suggested_directions,
            axis_scores,
            referenced_node_ids,
            referenced_edge_ids: Vec::new(),
        })
    }
}
