use serde::{Deserialize, Serialize};

use crate::{
    almanac::{
        hour_pillar::HourPillarResult,
        types::{RuleEvidence, ThapThanResult},
        tu_menh::Gender,
    },
    lunar::LunarDate,
    types::CanChi,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PillarKind {
    Year,
    Month,
    Day,
    Hour,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziInput {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub hour: u8,
    pub minute: u8,
    pub timezone: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(default)]
    pub use_solar_time: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StemRelationSet {
    pub stem: ThapThanResult,
    #[serde(default)]
    pub hidden_stems: Vec<ThapThanResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenStemEntry {
    pub stem_symbol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stem_name: Option<String>,
    pub strength: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ten_god_to_day_master: Option<ThapThanResult>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaziPillar {
    pub kind: PillarKind,
    pub can_chi: CanChi,
    pub hidden_stems: Vec<HiddenStemEntry>,
    pub na_am: Option<String>,
    pub stem_relation_to_day_master: Option<ThapThanResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziChartMetadata {
    pub timezone: f64,
    pub use_solar_time: bool,
    pub year_basis: String,
    pub month_basis: String,
    pub day_basis: String,
    pub hour_basis: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour_evidence: Option<RuleEvidence>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaziChart {
    pub input: BaziInput,
    pub lunar_date: LunarDate,
    pub year_pillar: BaziPillar,
    pub month_pillar: BaziPillar,
    pub day_pillar: BaziPillar,
    pub hour_pillar: BaziPillar,
    pub day_master: CanChi,
    pub pillars: Vec<BaziPillar>,
    pub metadata: BaziChartMetadata,
}

impl BaziChartMetadata {
    pub fn new(input: &BaziInput, hour_pillar: &HourPillarResult) -> Self {
        Self {
            timezone: input.timezone,
            use_solar_time: input.use_solar_time,
            year_basis: "lunar_year_from_solar_birth_date".to_string(),
            month_basis: "lunar_month_with_year_stem_mapping".to_string(),
            day_basis: "julian_day_cycle".to_string(),
            hour_basis: "day_stem_seed_table".to_string(),
            hour_evidence: Some(hour_pillar.evidence.clone()),
        }
    }
}
