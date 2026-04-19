use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticId {
    pub concept_label: String,
    pub stable_key: String,
}

impl SemanticId {
    pub fn new(concept_label: impl Into<String>, stable_key: impl Into<String>) -> Self {
        Self {
            concept_label: concept_label.into(),
            stable_key: stable_key.into(),
        }
    }

    pub fn day_canchi(can_index: usize, chi_index: usize) -> Self {
        Self::new("day_canchi", format!("{}_{}", can_index, chi_index))
    }

    pub fn month_canchi(can_index: usize, chi_index: usize) -> Self {
        Self::new("month_canchi", format!("{}_{}", can_index, chi_index))
    }

    pub fn year_canchi(can_index: usize, chi_index: usize) -> Self {
        Self::new("year_canchi", format!("{}_{}", can_index, chi_index))
    }

    pub fn solar_term(name: &str) -> Self {
        Self::new("solar_term", name.to_lowercase().replace(' ', "_"))
    }

    pub fn hour_canchi(can_index: usize, chi_index: usize, hour_index: usize) -> Self {
        Self::new(
            "hour_canchi",
            format!("{}_{}_{}", can_index, chi_index, hour_index),
        )
    }

    pub fn truc(name: &str) -> Self {
        Self::new("truc", name.to_lowercase())
    }

    pub fn day_deity(name: &str) -> Self {
        Self::new("day_deity", name.to_lowercase())
    }

    pub fn na_am(name: &str) -> Self {
        Self::new("na_am", name.to_lowercase())
    }

    pub fn star(name: &str) -> Self {
        Self::new("star", name.to_lowercase())
    }

    pub fn element(name: &str) -> Self {
        Self::new("element", name.to_lowercase())
    }

    pub fn direction(name: &str) -> Self {
        Self::new("direction", name.to_lowercase())
    }

    pub fn personal_alignment(personal_key: &str) -> Self {
        Self::new("personal_alignment", personal_key.to_lowercase())
    }

    pub fn interaction_signal(signal_type: &str, canchi_key: &str) -> Self {
        Self::new(
            "interaction_signal",
            format!("{}_{}", signal_type, canchi_key),
        )
    }

    pub fn recommendation(activity_id: &str) -> Self {
        Self::new("recommendation", activity_id.to_lowercase())
    }

    pub fn taboo(taboo_id: &str) -> Self {
        Self::new("taboo", taboo_id.to_lowercase())
    }

    pub fn chart_pillar(pillar_kind: &str, can_index: usize, chi_index: usize) -> Self {
        Self::new(
            "chart_pillar",
            format!("{}_{}_{}", pillar_kind, can_index, chi_index),
        )
    }

    pub fn axis_signal(axis_name: &str, signal_id: &str) -> Self {
        Self::new("axis_signal", format!("{}_{}", axis_name, signal_id))
    }

    pub fn to_node_id(&self) -> String {
        format!("{}:{}", self.concept_label, self.stable_key)
    }
}

impl std::fmt::Display for SemanticId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_node_id())
    }
}
