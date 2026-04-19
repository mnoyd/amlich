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

    pub fn day_root(date: &str, tz: &str) -> Self {
        Self::new("day", format!("{}:{}", date, tz))
    }

    pub fn day_child(child_kind: &str, date: &str, tz: &str) -> Self {
        Self::new(child_kind, format!("day:{}:{}", date, tz))
    }

    pub fn solar_term_day(date: &str, tz: &str) -> Self {
        Self::day_child("solar_term", date, tz)
    }

    pub fn truc_day(date: &str, tz: &str) -> Self {
        Self::day_child("truc", date, tz)
    }

    pub fn day_deity_day(date: &str, tz: &str) -> Self {
        Self::day_child("day_deity", date, tz)
    }

    pub fn star_day(date: &str, tz: &str) -> Self {
        Self::day_child("star", date, tz)
    }

    pub fn taboo_day(date: &str, tz: &str, taboo_name: &str) -> Self {
        Self::new("taboo", format!("day:{}:{}:{}", date, tz, taboo_name))
    }

    pub fn xung_hop_day(date: &str, tz: &str) -> Self {
        Self::day_child("xung_hop", date, tz)
    }

    pub fn travel_day(date: &str, tz: &str) -> Self {
        Self::day_child("travel", date, tz)
    }

    pub fn hoang_dao_hours_day(date: &str, tz: &str) -> Self {
        Self::day_child("hoang_dao_hours", date, tz)
    }

    pub fn bazi_profile(dob: &str, tz: &str) -> Self {
        Self::new("bazi_profile", format!("{}:{}", dob, tz))
    }

    pub fn pillar_bazi(pillar_kind: &str, dob: &str, tz: &str) -> Self {
        Self::new(
            "pillar",
            format!("bazi_profile:{}:{}:{}", dob, tz, pillar_kind),
        )
    }

    pub fn element_distribution_bazi(dob: &str, tz: &str) -> Self {
        Self::new("element_distribution", format!("bazi_profile:{}:{}", dob, tz))
    }

    pub fn ten_god_distribution_bazi(dob: &str, tz: &str) -> Self {
        Self::new("ten_god_distribution", format!("bazi_profile:{}:{}", dob, tz))
    }

    pub fn day_master_strength_bazi(dob: &str, tz: &str) -> Self {
        Self::new("day_master_strength", format!("bazi_profile:{}:{}", dob, tz))
    }

    pub fn chart_interaction_bazi(dob: &str, tz: &str, index: usize) -> Self {
        Self::new(
            "chart_interaction",
            format!("bazi_profile:{}:{}:{}", dob, tz, index),
        )
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
