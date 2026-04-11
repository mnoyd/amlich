use serde::{Deserialize, Serialize};

use crate::{
    almanac::{
        hour_pillar::HourPillarResult,
        tu_menh::Gender,
        types::{RuleEvidence, ThapThanResult},
    },
    lunar::LunarDate,
    types::{CanChi, CHI},
};

// ---------------------------------------------------------------------------
// Thai Nguyên (胎元 — Conception Pillar)
// ---------------------------------------------------------------------------

/// The "5th pillar" representing the month of conception (~3 months after
/// the birth month in the stem/branch cycle).
#[derive(Debug, Clone, PartialEq)]
pub struct ThaiNguyenResult {
    pub can_chi: CanChi,
    pub evidence: RuleEvidence,
}

// ---------------------------------------------------------------------------
// Mệnh Cung / Thân Cung (命宮 / 身宮)
// ---------------------------------------------------------------------------

/// Life Palace (pre-heaven, innate) and Body Palace (post-heaven, manifest).
#[derive(Debug, Clone, PartialEq)]
pub struct MenhCungResult {
    /// Mệnh Cung — innate personality / early life.
    pub menh_cung: CanChi,
    /// Thân Cung — manifested destiny / later life (after ~35).
    pub than_cung: CanChi,
    pub evidence: RuleEvidence,
}

// ---------------------------------------------------------------------------
// Không Vong (空亡 — Empty / Void Branches)
// ---------------------------------------------------------------------------

/// The two void branches for a single pillar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KhongVongPair {
    pub branch_indices: [usize; 2],
    pub branch_names: [String; 2],
}

/// Per-pillar void analysis with cross-reference hits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KhongVongPillarEntry {
    pub pillar: PillarKind,
    pub void_pair: KhongVongPair,
    /// Which other pillars' branches fall into this pillar's void pair.
    pub hits: Vec<PillarKind>,
}

/// Complete Không Vong analysis for a chart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KhongVongAnalysis {
    pub entries: Vec<KhongVongPillarEntry>,
    pub evidence: RuleEvidence,
}

// ---------------------------------------------------------------------------
// Thần Sát (神煞 — Auxiliary / Symbolic Stars)
// ---------------------------------------------------------------------------

/// Derivation source for a Thần Sát star.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThanSatSource {
    /// Derived from the day stem (天干).
    DayStem,
    /// Derived from the year branch (地支).
    YearBranch,
    /// Derived from the day branch.
    DayBranch,
    /// Derived from the month branch.
    MonthBranch,
}

/// A single symbolic star and which chart pillars it occupies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThanSatEntry {
    pub name: String,
    pub source: ThanSatSource,
    /// The target branch index this star maps to.
    pub target_branch: usize,
    pub target_branch_name: String,
    /// Which pillars contain the target branch (may be empty).
    pub present_in: Vec<PillarKind>,
}

/// Complete Thần Sát analysis for a chart.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThanSatResult {
    pub stars: Vec<ThanSatEntry>,
    pub evidence: RuleEvidence,
}

// ---------------------------------------------------------------------------
// Bazi Derived Report (aggregates all quick-win computations)
// ---------------------------------------------------------------------------

/// Aggregated report of all derived Bazi computations beyond the 4 pillars.
#[derive(Debug, Clone, PartialEq)]
pub struct BaziDerivedReport {
    pub thai_nguyen: ThaiNguyenResult,
    pub menh_cung: MenhCungResult,
    pub khong_vong: KhongVongAnalysis,
    pub than_sat: ThanSatResult,
}

// ---------------------------------------------------------------------------
// Helper: compute void pair from sexagenary index
// ---------------------------------------------------------------------------

impl KhongVongPair {
    /// Compute the two void branches for a given sexagenary (60-cycle) index.
    ///
    /// Within each 10-pillar Tuần (旬), stems 0-9 pair with 10 consecutive
    /// branches, leaving 2 branches unused.  The first void branch index is
    /// `(10 - sexagenary_index % 10) % 12`; the second is the next branch.
    pub fn from_sexagenary(sexagenary_index: usize) -> Self {
        // The 60-year cycle has 6 groups of 10 (Tuần / 旬).
        // Each Tuần uses 10 of 12 branches, leaving 2 void:
        //   Tuần 0 (Giáp Tý, idx  0- 9): void = Tuất(10), Hợi(11)
        //   Tuần 1 (Giáp Tuất, idx 10-19): void = Thân(8), Dậu(9)
        //   Tuần 2 (Giáp Thân, idx 20-29): void = Ngọ(6), Mùi(7)
        //   Tuần 3 (Giáp Ngọ, idx 30-39): void = Thìn(4), Tỵ(5)
        //   Tuần 4 (Giáp Thìn, idx 40-49): void = Dần(2), Mão(3)
        //   Tuần 5 (Giáp Dần, idx 50-59): void = Tý(0), Sửu(1)
        let tuan_idx = sexagenary_index / 10;
        let (a, b) = match tuan_idx {
            0 => (10, 11),
            1 => (8, 9),
            2 => (6, 7),
            3 => (4, 5),
            4 => (2, 3),
            5 => (0, 1),
            _ => (10, 11), // fallback
        };

        KhongVongPair {
            branch_indices: [a, b],
            branch_names: [CHI[a].to_string(), CHI[b].to_string()],
        }
    }
}

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
