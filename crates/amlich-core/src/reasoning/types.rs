#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionId {
    InitiationOpening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Fact,
    InterpretedSignal,
    DecisionTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningNode {
    pub id: String,
    pub kind: NodeKind,
    pub summary_vi: String,
    pub severity: Option<String>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningEdge {
    pub from_node_id: String,
    pub to_node_id: String,
    pub effect: EdgeEffect,
}

impl ReasoningEdge {
    pub fn new(from_node_id: impl Into<String>, to_node_id: impl Into<String>, effect: EdgeEffect) -> Self {
        Self {
            from_node_id: from_node_id.into(),
            to_node_id: to_node_id.into(),
            effect,
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
