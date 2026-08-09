use crate::semantic_graph::{
    NodeConcept, SemanticFact, SemanticGraph, SemanticNode, SemanticPolarity,
};

use crate::advisory::ConsultationIntent;
use crate::assessment::PersonalDayAssessment;
use crate::birth::BirthProfile;
use crate::reasoning::action_evaluator::{ActionEvaluation, ActionEvaluator};
use crate::reasoning::types::{ActionId, ReasoningConclusionSemantic, ReasoningNote};
use crate::reasoning::{PersonalAssessmentFacts, PersonalReasoningInput};
use crate::types::VIETNAM_TIMEZONE;
use crate::DaySnapshot;

pub struct InitiationOpeningEvaluator;

/// Concepts the initiation/opening action is allowed to read. Finding A-R04
/// (amlich-mwbp.8): the evaluator must consume exactly these day-fact
/// concepts, so [`InitiationOpeningEvaluator::select_subgraph`] filters the
/// merged reasoning graph down to this allowlist. Anything outside it — solar
/// term, ritual, hexagram, matrix debug rows, future builder output — must
/// not be able to alter the decision.
const INITIATION_OPENING_ALLOWED_CONCEPTS: &[NodeConcept] = &[
    NodeConcept::Truc,
    NodeConcept::DayDeity,
    NodeConcept::Star,
    NodeConcept::XungHop,
    NodeConcept::Taboo,
    NodeConcept::InteractionRow,
    NodeConcept::HoangDaoHour,
];

fn concept_is_allowed(concept: NodeConcept) -> bool {
    INITIATION_OPENING_ALLOWED_CONCEPTS.contains(&concept)
}

/// Build the [`BirthProfile`] the canonical assessment consumes, mirroring the
/// construction in `reasoning::synthesis` so the evaluator and the export
/// orchestrator see the same normalized profile. `personal_input = None`
/// yields an anonymous date-only profile (the inquiry date with no birth
/// time/gender), which the assessment builder supports via its capability
/// tiers. The resulting assessment is snapshot-derived, so it is invariant to
/// graph-node multiplicity — the property `amlich-zakn` relies on to make
/// axis scores duplicate-monotone and provenance-backed.
///
/// Kept as a documentation seam for the legacy `evaluate` trait method;
/// per-request paths now reuse the cached canonical assessment via
/// `evaluate_with_facts` (amlich-mwbp.8 P2 finding A-R11).
#[allow(dead_code)]
fn assessment_profile(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> BirthProfile {
    let (timezone, gender) = match personal_input {
        Some(p) => (p.birth.timezone, p.birth.gender),
        None => (VIETNAM_TIMEZONE, None),
    };
    BirthProfile {
        day: snapshot.context.solar.day,
        month: snapshot.context.solar.month,
        year: snapshot.context.solar.year,
        time: None,
        timezone,
        longitude: None,
        use_solar_time: false,
        gender,
        location_name: None,
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct TrucOpeningSignal {
    opening_avoid_count: usize,
}

impl InitiationOpeningEvaluator {
    pub fn new() -> Self {
        Self
    }

    fn extract_support_evidence(
        &self,
        graph: &SemanticGraph,
        _snapshot: &DaySnapshot,
    ) -> Vec<ReasoningNote> {
        let mut notes = Vec::new();

        if let Some(truc_node) = self.find_node_by_concept(graph, NodeConcept::Truc) {
            if let Some(quality) = &truc_node.severity {
                if quality == "cat" {
                    notes.push(ReasoningNote {
                        node_id: Some(truc_node.node_id.clone()),
                        summary_vi: truc_node.summary_vi.clone(),
                        tags: vec!["support".to_string(), "truc".to_string()],
                        provenance: truc_node
                            .provenance
                            .iter()
                            .map(|p| p.to_reasoning_evidence())
                            .collect(),
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
                        provenance: deity_node
                            .provenance
                            .iter()
                            .map(|p| p.to_reasoning_evidence())
                            .collect(),
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
                    provenance: star_node
                        .provenance
                        .iter()
                        .map(|p| p.to_reasoning_evidence())
                        .collect(),
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
                    provenance: truc_node
                        .provenance
                        .iter()
                        .map(|p| p.to_reasoning_evidence())
                        .collect(),
                });
            }
        }

        if let Some(xung_hop_node) = self.find_node_by_concept(graph, NodeConcept::XungHop) {
            if matches!(
                xung_hop_node.fact,
                Some(SemanticFact::XungHop {
                    has_clash: true,
                    has_harmony: false,
                })
            ) {
                notes.push(ReasoningNote {
                    node_id: Some(xung_hop_node.node_id.clone()),
                    summary_vi: xung_hop_node.summary_vi.clone(),
                    tags: vec!["resistance".to_string(), "xung_hop".to_string()],
                    provenance: xung_hop_node
                        .provenance
                        .iter()
                        .map(|p| p.to_reasoning_evidence())
                        .collect(),
                });
            }
        }

        for node in graph.nodes().values() {
            if node.concept == NodeConcept::Taboo && node.severity.is_some() {
                notes.push(ReasoningNote {
                    node_id: Some(node.node_id.clone()),
                    summary_vi: node.summary_vi.clone(),
                    tags: vec!["resistance".to_string(), "taboo".to_string()],
                    provenance: node
                        .provenance
                        .iter()
                        .map(|p| p.to_reasoning_evidence())
                        .collect(),
                });
            }
        }

        if let Some(node) = self.most_unfavorable_direction_row(graph) {
            notes.push(ReasoningNote {
                node_id: Some(node.node_id.clone()),
                summary_vi: format!("Hướng cá nhân có điểm xung: {}", node.summary_vi),
                tags: vec!["resistance".to_string(), "personal_direction".to_string()],
                provenance: node
                    .provenance
                    .iter()
                    .map(|p| p.to_reasoning_evidence())
                    .collect(),
            });
        }

        notes
    }

    fn extract_override_evidence(&self, graph: &SemanticGraph) -> Vec<ReasoningNote> {
        let mut notes = Vec::new();

        for node in graph.nodes().values() {
            if node.concept == NodeConcept::Taboo {
                if let Some(ref severity) = node.severity {
                    if severity == "hard" {
                        notes.push(ReasoningNote {
                            node_id: Some(node.node_id.clone()),
                            summary_vi: node.summary_vi.clone(),
                            tags: vec!["override".to_string(), "hard_taboo".to_string()],
                            provenance: node
                                .provenance
                                .iter()
                                .map(|p| p.to_reasoning_evidence())
                                .collect(),
                        });
                    }
                }
            }
        }

        notes
    }

    fn has_favorable_fact(&self, graph: &SemanticGraph) -> bool {
        graph.nodes().values().any(|n| match n.concept {
            NodeConcept::Truc => n.severity.as_deref() == Some("cat"),
            NodeConcept::DayDeity => n.severity.as_deref() == Some("hoang_dao"),
            NodeConcept::HoangDaoHour => n.severity.is_some(),
            NodeConcept::Star => self.is_star_supportive(n),
            _ => false,
        })
    }

    fn has_unfavorable_fact(&self, graph: &SemanticGraph) -> bool {
        graph.nodes().values().any(|n| match n.concept {
            NodeConcept::Taboo => n.severity.is_some(),
            NodeConcept::InteractionRow => matches!(
                n.fact,
                Some(SemanticFact::Direction { net_score }) if net_score < 0
            ),
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
                    provenance: truc_node
                        .provenance
                        .iter()
                        .map(|p| p.to_reasoning_evidence())
                        .collect(),
                });
            }
        }

        if has_favorable && has_unfavorable {
            notes.push(ReasoningNote {
                node_id: None,
                summary_vi: "Bối cảnh có cả yếu tố thuận và cản trở".to_string(),
                tags: vec!["conflict".to_string()],
                provenance: vec![],
            });
        }

        if self.has_mixed_direction_merge(graph) {
            notes.push(ReasoningNote {
                node_id: None,
                summary_vi: "Hướng hợp cá nhân còn phân hóa giữa thuận và nghịch".to_string(),
                tags: vec!["conflict".to_string(), "personal_direction".to_string()],
                provenance: vec![],
            });
        }

        notes
    }

    fn has_mixed_direction_merge(&self, graph: &SemanticGraph) -> bool {
        let mut has_favorable = false;
        let mut has_unfavorable = false;

        for node in self.direction_rows(graph) {
            if matches!(node.fact, Some(SemanticFact::Direction { net_score }) if net_score > 0) {
                has_favorable = true;
            }
            if matches!(node.fact, Some(SemanticFact::Direction { net_score }) if net_score < 0) {
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
            .filter(|node| {
                matches!(node.fact, Some(SemanticFact::Direction { net_score }) if net_score < 0)
            })
            .min_by(|left, right| {
                let left_score = match left.fact {
                    Some(SemanticFact::Direction { net_score }) => net_score,
                    _ => 0,
                };
                let right_score = match right.fact {
                    Some(SemanticFact::Direction { net_score }) => net_score,
                    _ => 0,
                };
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
                && matches!(node.fact, Some(SemanticFact::Direction { .. }))
        })
    }

    fn canonical_semantic(value: &str) -> Result<ReasoningConclusionSemantic, String> {
        match value {
            "override_avoid" => Ok(ReasoningConclusionSemantic::OverrideAvoid),
            "override_cautious" => Ok(ReasoningConclusionSemantic::OverrideCautious),
            "conflicted_cautious" => Ok(ReasoningConclusionSemantic::ConflictedCautious),
            "resistance_led_cautious" => Ok(ReasoningConclusionSemantic::ResistanceLedCautious),
            "favorable_clear" => Ok(ReasoningConclusionSemantic::FavorableClear),
            "favorable_contextual" => Ok(ReasoningConclusionSemantic::FavorableContextual),
            other => Err(format!("unsupported canonical reasoning semantic: {other}")),
        }
    }

    /// Legacy snapshot-based variant. Per-request paths call
    /// `suggested_hours_with_facts` directly (amlich-mwbp.8 P2 finding A-R11).
    #[allow(dead_code)]
    fn suggested_hours(
        &self,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
    ) -> Vec<String> {
        self.suggested_hours_with_facts(snapshot, personal_input, None)
    }

    fn suggested_hours_with_facts(
        &self,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
        facts: Option<&PersonalAssessmentFacts>,
    ) -> Vec<String> {
        if let Some(personal_input) = personal_input {
            let personal_hours = match facts {
                Some(facts) => personal_input.suggested_hours_from_facts(facts),
                None => personal_input.suggested_hours(snapshot),
            };
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

    /// Legacy snapshot-based variant. Per-request paths call
    /// `suggested_directions_with_facts` directly (amlich-mwbp.8 P2 finding A-R11).
    #[allow(dead_code)]
    fn suggested_directions(
        &self,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
    ) -> Vec<String> {
        self.suggested_directions_with_facts(snapshot, personal_input, None)
    }

    fn suggested_directions_with_facts(
        &self,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
        facts: Option<&PersonalAssessmentFacts>,
    ) -> Vec<String> {
        if let Some(personal_input) = personal_input {
            let personal_directions = match facts {
                Some(facts) => personal_input.suggested_directions_from_facts(facts),
                None => personal_input.suggested_directions(snapshot),
            };
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

    fn find_node_by_concept<'a>(
        &self,
        graph: &'a SemanticGraph,
        concept: NodeConcept,
    ) -> Option<&'a SemanticNode> {
        graph.nodes().values().find(|n| n.concept == concept)
    }

    fn is_star_supportive(&self, node: &SemanticNode) -> bool {
        matches!(
            node.fact,
            Some(SemanticFact::Star {
                polarity: SemanticPolarity::Favorable
            })
        )
    }

    fn truc_opening_signal<'a>(
        &self,
        graph: &'a SemanticGraph,
    ) -> Option<(&'a SemanticNode, TrucOpeningSignal)> {
        let truc_node = self.find_node_by_concept(graph, NodeConcept::Truc)?;
        let SemanticFact::Truc {
            opening_avoid_count,
            ..
        } = truc_node.fact.as_ref()?
        else {
            return None;
        };

        Some((
            truc_node,
            TrucOpeningSignal {
                opening_avoid_count: usize::from(*opening_avoid_count),
            },
        ))
    }

    /// Evaluate reusing a precomputed [`PersonalAssessmentFacts`] and
    /// [`PersonalDayAssessment`] so the chart, the matrices, and the
    /// canonical assessment are not rebuilt alongside the evaluation.
    /// Per-request request paths must use this entry point — see
    /// REPAIR-PLAN.md P2 (`amlich-mwbp.8` finding A-R11).
    pub fn evaluate_with_facts(
        &self,
        graph: &SemanticGraph,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
        facts: Option<&PersonalAssessmentFacts>,
        canonical_assessment: Option<&PersonalDayAssessment>,
    ) -> Result<ActionEvaluation, String> {
        // amlich-mwbp.8 (finding A-R04): evaluate over the allowlisted
        // subgraph so unrelated context nodes cannot alter the
        // initiation/opening decision. `select_subgraph` is idempotent, so
        // re-evaluating an already-filtered graph is safe.
        let subgraph = self.select_subgraph(graph, snapshot, personal_input)?;
        let graph = &subgraph;
        let assessment = canonical_assessment.ok_or_else(|| {
            "canonical PersonalDayAssessment is required for initiation/opening reasoning"
                .to_string()
        })?;

        let support_notes = self.extract_support_evidence(graph, snapshot);
        let resistance_notes = self.extract_resistance_evidence(graph);
        let override_notes = self.extract_override_evidence(graph);
        let conflict_notes = self.extract_conflict_evidence(graph);
        let semantic = Self::canonical_semantic(&assessment.decision.semantic)?;
        let bucket = assessment.decision.bucket;
        let confidence = assessment.decision.confidence;
        let context_is_clear = assessment.decision.context_is_clear;
        let primary_conclusion = assessment.decision.primary_conclusion.clone();

        let suggested_hours = self.suggested_hours_with_facts(snapshot, personal_input, facts);
        let suggested_directions =
            self.suggested_directions_with_facts(snapshot, personal_input, facts);

        // All verdict fields and numerical axes come from the same canonical
        // assessment. The graph only supplies typed evidence projections and
        // presentation notes; changing or translating those summaries cannot
        // alter the decision.
        let mut axis_scores = assessment.axis_scores();
        for axis in axis_scores.iter_mut() {
            if axis.strongest_node_id.is_none() {
                axis.score = 0.0;
            }
        }

        let mut referenced_node_ids = Vec::new();
        for note in support_notes
            .iter()
            .chain(resistance_notes.iter())
            .chain(override_notes.iter())
        {
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
        })
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
        // amlich-mwbp.8 (finding A-R04): isolate the initiation/opening action
        // to its allowlisted subgraph instead of returning a full clone. The
        // evaluator only consumes the day-fact concepts listed in
        // [`INITIATION_OPENING_ALLOWED_CONCEPTS`]; filtering the rest out
        // means future graph builders cannot leak action-irrelevant facts
        // (solar term, ritual, hexagram, matrix debug rows, ...) into the
        // decision. [`SemanticGraph::add_edge`] silently drops any edge that
        // references a removed node, so the subgraph stays internally
        // consistent without an explicit edge filter.
        let mut subgraph = SemanticGraph::new();
        for node in graph.nodes().values() {
            if concept_is_allowed(node.concept) {
                subgraph.add_node(node.clone());
            }
        }
        for edge in graph.edges().values() {
            subgraph.add_edge(edge.clone());
        }
        Ok(subgraph)
    }

    fn evaluate(
        &self,
        graph: &SemanticGraph,
        snapshot: &DaySnapshot,
        personal_input: Option<&PersonalReasoningInput>,
    ) -> Result<ActionEvaluation, String> {
        // amlich-mwbp.8 P2: the trait-level `evaluate` keeps its legacy
        // signature and rebuilds the assessment internally so callers
        // that go through the trait do not duplicate the consumer API. The
        // consolidated per-request path uses `evaluate_with_facts` directly
        // and supplies the cached canonical assessment.
        let profile = personal_input
            .map(PersonalReasoningInput::to_birth_profile)
            .unwrap_or(BirthProfile {
                day: snapshot.context.solar.day,
                month: snapshot.context.solar.month,
                year: snapshot.context.solar.year,
                time: None,
                timezone: VIETNAM_TIMEZONE,
                longitude: None,
                use_solar_time: false,
                gender: None,
                location_name: None,
            });
        let canonical_assessment = PersonalDayAssessment::assess(
            snapshot.clone(),
            profile,
            ConsultationIntent::OpeningBusiness,
        );
        self.evaluate_with_facts(
            graph,
            snapshot,
            personal_input,
            None,
            Some(&canonical_assessment),
        )
    }
}
