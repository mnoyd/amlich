use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeConcept {
    DayCanchi,
    MonthCanchi,
    YearCanchi,
    SolarTerm,
    HourCanchi,
    Truc,
    DayDeity,
    NaAm,
    Star,
    Element,
    Direction,
    PersonalAlignment,
    InteractionSignal,
    Recommendation,
    Taboo,
    ChartPillar,
    AxisSignal,
}

impl NodeConcept {
    pub fn label(&self) -> ConceptLabel {
        match self {
            Self::DayCanchi => ConceptLabel::DayCanchi,
            Self::MonthCanchi => ConceptLabel::MonthCanchi,
            Self::YearCanchi => ConceptLabel::YearCanchi,
            Self::SolarTerm => ConceptLabel::SolarTerm,
            Self::HourCanchi => ConceptLabel::HourCanchi,
            Self::Truc => ConceptLabel::Truc,
            Self::DayDeity => ConceptLabel::DayDeity,
            Self::NaAm => ConceptLabel::NaAm,
            Self::Star => ConceptLabel::Star,
            Self::Element => ConceptLabel::Element,
            Self::Direction => ConceptLabel::Direction,
            Self::PersonalAlignment => ConceptLabel::PersonalAlignment,
            Self::InteractionSignal => ConceptLabel::InteractionSignal,
            Self::Recommendation => ConceptLabel::Recommendation,
            Self::Taboo => ConceptLabel::Taboo,
            Self::ChartPillar => ConceptLabel::ChartPillar,
            Self::AxisSignal => ConceptLabel::AxisSignal,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeConcept {
    Resonates,
    Conflicts,
    Conditions,
    Supports,
    Weakens,
    Overrides,
    Composes,
    Projects,
    Derives,
}

impl EdgeConcept {
    pub fn label(&self) -> ConceptLabel {
        match self {
            Self::Resonates => ConceptLabel::Resonates,
            Self::Conflicts => ConceptLabel::Conflicts,
            Self::Conditions => ConceptLabel::Conditions,
            Self::Supports => ConceptLabel::Supports,
            Self::Weakens => ConceptLabel::Weakens,
            Self::Overrides => ConceptLabel::Overrides,
            Self::Composes => ConceptLabel::Composes,
            Self::Projects => ConceptLabel::Projects,
            Self::Derives => ConceptLabel::Derives,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConceptLabel {
    DayCanchi,
    MonthCanchi,
    YearCanchi,
    SolarTerm,
    HourCanchi,
    Truc,
    DayDeity,
    NaAm,
    Star,
    Element,
    Direction,
    PersonalAlignment,
    InteractionSignal,
    Recommendation,
    Taboo,
    ChartPillar,
    AxisSignal,
    Resonates,
    Conflicts,
    Conditions,
    Supports,
    Weakens,
    Overrides,
    Composes,
    Projects,
    Derives,
}

pub struct GraphOntology;

impl GraphOntology {
    pub fn node_concepts() -> &'static [NodeConcept] {
        &[
            NodeConcept::DayCanchi,
            NodeConcept::MonthCanchi,
            NodeConcept::YearCanchi,
            NodeConcept::SolarTerm,
            NodeConcept::HourCanchi,
            NodeConcept::Truc,
            NodeConcept::DayDeity,
            NodeConcept::NaAm,
            NodeConcept::Star,
            NodeConcept::Element,
            NodeConcept::Direction,
            NodeConcept::PersonalAlignment,
            NodeConcept::InteractionSignal,
            NodeConcept::Recommendation,
            NodeConcept::Taboo,
            NodeConcept::ChartPillar,
            NodeConcept::AxisSignal,
        ]
    }

    pub fn edge_concepts() -> &'static [EdgeConcept] {
        &[
            EdgeConcept::Resonates,
            EdgeConcept::Conflicts,
            EdgeConcept::Conditions,
            EdgeConcept::Supports,
            EdgeConcept::Weakens,
            EdgeConcept::Overrides,
            EdgeConcept::Composes,
            EdgeConcept::Projects,
            EdgeConcept::Derives,
        ]
    }
}
