use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionId {
    InitiationOpening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeKind {
    Fact,
    InterpretedSignal,
    DecisionTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InterpretedAxis {
    Support,
    Resistance,
    Stability,
    PersonalAlignment,
    TimingFit,
    ContextClarity,
}

impl InterpretedAxis {
    pub fn core_axes() -> [Self; 6] {
        [
            Self::Support,
            Self::Resistance,
            Self::Stability,
            Self::PersonalAlignment,
            Self::TimingFit,
            Self::ContextClarity,
        ]
    }

    pub fn signal_node_id(self) -> &'static str {
        match self {
            Self::Support => "signal.support",
            Self::Resistance => "signal.resistance",
            Self::Stability => "signal.stability",
            Self::PersonalAlignment => "signal.personal_alignment",
            Self::TimingFit => "signal.timing_fit",
            Self::ContextClarity => "signal.context_clarity",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeEffect {
    Supports,
    Weakens,
    Overrides,
    ConflictsWith,
    Conditions,
}

impl EdgeEffect {
    pub fn is_override(self) -> bool {
        matches!(self, Self::Overrides)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationBucket {
    Avoid,
    Cautious,
    Mixed,
    Favorable,
}

impl RecommendationBucket {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Avoid => "avoid",
            Self::Cautious => "cautious",
            Self::Mixed => "mixed",
            Self::Favorable => "favorable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitiationOpeningDecision {
    pub primary_conclusion: String,
    pub recommendation_bucket: RecommendationBucket,
    pub strongest_supports: Vec<String>,
    pub strongest_resistances: Vec<String>,
    pub override_factors: Vec<String>,
    pub conflict_notes: Vec<String>,
    pub confidence: DecisionConfidence,
    pub context_is_clear: bool,
    pub suggested_hours: Vec<String>,
    pub suggested_directions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningConclusionSemantic {
    OverrideAvoid,
    OverrideCautious,
    ConflictedCautious,
    ResistanceLedCautious,
    FavorableClear,
    FavorableContextual,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningNote {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    pub summary_vi: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningNode {
    pub id: String,
    pub kind: NodeKind,
    pub summary_vi: String,
    pub severity: Option<String>,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEvidenceSourceFamily {
    Snapshot,
    Interaction,
    Bazi,
    Axis,
    AlmanacRule,
    Insight,
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningEvidenceEnvelope {
    pub source_family: ReasoningEvidenceSourceFamily,
    pub source_id: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningEdge {
    pub from_node_id: String,
    pub to_node_id: String,
    pub effect: EdgeEffect,
    pub justification: ReasoningEdgeJustification,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

impl ReasoningEdge {
    pub fn new(
        from_node_id: impl Into<String>,
        to_node_id: impl Into<String>,
        effect: EdgeEffect,
        justification: ReasoningEdgeJustification,
        evidence: Vec<ReasoningEvidenceEnvelope>,
    ) -> Self {
        Self {
            from_node_id: from_node_id.into(),
            to_node_id: to_node_id.into(),
            effect,
            justification,
            evidence,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningGraph {
    pub action_id: ActionId,
    pub nodes: Vec<ReasoningNode>,
    pub edges: Vec<ReasoningEdge>,
}

impl ReasoningGraph {
    pub fn new(action_id: ActionId) -> Self {
        Self {
            action_id,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningNodeSeverity {
    Auspicious,
    Inauspicious,
    HardTaboo,
    SoftTaboo,
    HoangDao,
    HacDao,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningNodeExport {
    pub id: String,
    pub kind: NodeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub axis: Option<InterpretedAxis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<ReasoningNodeSeverity>,
    #[serde(default)]
    pub tags: Vec<String>,
    pub summary_vi: String,
    #[serde(default)]
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningEdgeExport {
    pub from_node_id: String,
    pub to_node_id: String,
    pub effect: EdgeEffect,
    pub weight: i32,
    pub justification: ReasoningEdgeJustification,
    #[serde(default)]
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReasoningGraphExport {
    pub action_id: ActionId,
    pub nodes: Vec<ReasoningNodeExport>,
    pub edges: Vec<ReasoningEdgeExport>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEdgeJustification {
    FavorableDaySignal,
    TrucActivitySupport,
    TrucActivityConflict,
    DayDeitySupport,
    StarSupport,
    TabooPressure,
    TabooStabilityPenalty,
    TabooContextPenalty,
    ClashPressure,
    ClashStabilityPenalty,
    HoangDaoHourSupport,
    PersonalDayAlignment,
    PersonalHourAlignment,
    MixedSignalConflict,
    AvailableContextSupport,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasoningAxisScore {
    pub axis: InterpretedAxis,
    pub score: f32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strongest_node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strongest_summary_vi: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitiationOpeningDecisionExport {
    pub primary_conclusion: String,
    pub recommendation_bucket: RecommendationBucket,
    pub confidence: DecisionConfidence,
    pub context_is_clear: bool,
    pub semantic: ReasoningConclusionSemantic,
    #[serde(default)]
    pub strongest_supports: Vec<ReasoningNote>,
    #[serde(default)]
    pub strongest_resistances: Vec<ReasoningNote>,
    #[serde(default)]
    pub override_factors: Vec<ReasoningNote>,
    #[serde(default)]
    pub conflict_notes: Vec<ReasoningNote>,
    #[serde(default)]
    pub suggested_hours: Vec<String>,
    #[serde(default)]
    pub suggested_directions: Vec<String>,
    #[serde(default)]
    pub axis_scores: Vec<ReasoningAxisScore>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InitiationOpeningReasoningBundle {
    pub decision: InitiationOpeningDecision,
    pub decision_export: InitiationOpeningDecisionExport,
    pub graph: ReasoningGraphExport,
}
