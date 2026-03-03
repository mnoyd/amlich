use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DateQuery {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub timezone: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarDto {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub day_of_week: usize,
    pub day_of_week_name: String,
    pub date_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarDto {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub is_leap_month: bool,
    pub date_string: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NguHanhDto {
    pub can: String,
    pub chi: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanChiDto {
    pub can_index: usize,
    pub chi_index: usize,
    pub can: String,
    pub chi: String,
    pub full: String,
    pub con_giap: String,
    pub ngu_hanh: NguHanhDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanChiInfoDto {
    pub day: CanChiDto,
    pub month: CanChiDto,
    pub year: CanChiDto,
    pub full: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TietKhiDto {
    pub index: usize,
    pub name: String,
    pub description: String,
    pub longitude: i32,
    pub current_longitude: f64,
    pub season: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourInfoDto {
    pub hour_index: usize,
    pub hour_chi: String,
    pub time_range: String,
    pub star: String,
    pub is_good: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GioHoangDaoDto {
    pub day_chi: String,
    pub good_hour_count: usize,
    pub good_hours: Vec<HourInfoDto>,
    pub all_hours: Vec<HourInfoDto>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayElementDto {
    pub na_am: String,
    pub element: String,
    pub can_element: String,
    pub chi_element: String,
    pub evidence: Option<RuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayConflictDto {
    pub opposing_chi: String,
    pub opposing_con_giap: String,
    pub tuoi_xung: Vec<String>,
    pub sat_huong: String,
    pub evidence: Option<RuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelDirectionDto {
    pub xuat_hanh_huong: String,
    pub tai_than: String,
    pub hy_than: String,
    pub evidence: Option<RuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleEvidenceDto {
    pub source_id: String,
    pub method: String,
    pub profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayStarDto {
    pub system: String,
    pub index: usize,
    pub name: String,
    pub quality: String,
    pub evidence: Option<RuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarRuleEvidenceDto {
    pub name: String,
    pub quality: String,
    pub category: String,
    pub source_id: String,
    pub method: String,
    pub profile: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayStarsDto {
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
    pub day_star: Option<DayStarDto>,
    pub star_system: Option<String>,
    pub evidence: Option<RuleEvidenceDto>,
    pub matched_rules: Vec<StarRuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayDeityDto {
    pub name: String,
    pub classification: String,
    pub evidence: Option<RuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XungHopDto {
    pub luc_xung: String,
    pub tam_hop: Vec<String>,
    pub tu_hanh_xung: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrucDto {
    pub index: usize,
    pub name: String,
    pub quality: String,
    pub evidence: Option<RuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayTenGodsDto {
    /// Ten Gods relation from day stem to year stem
    pub to_year_stem: Option<ThapThanResultDto>,
    /// Ten Gods relation from day stem to self (day stem to day stem)
    pub to_self: Option<ThapThanResultDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThapThanResultDto {
    pub label: String,
    pub relation: String,
    pub same_polarity: bool,
    pub evidence: RuleEvidenceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayTabooDto {
    pub rule_id: String,
    pub name: String,
    pub severity: String,
    pub reason: String,
    pub evidence: Option<RuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayFortuneDto {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub day_element: DayElementDto,
    pub conflict: DayConflictDto,
    pub travel: TravelDirectionDto,
    pub stars: DayStarsDto,
    pub day_deity: Option<DayDeityDto>,
    pub taboos: Vec<DayTabooDto>,
    pub xung_hop: XungHopDto,
    pub truc: TrucDto,
    /// Ten Gods relations for predefined targets (populated when day stem available)
    pub ten_gods: Option<DayTenGodsDto>,
    /// Kua (Tu Mến) result (populated only when birth year and gender provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu_menh: Option<KuaResultDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KuaResultDto {
    pub kua: u8,
    pub group: String,
    pub favorable_directions: Vec<String>,
    pub unfavorable_directions: Vec<String>,
    pub convention: ConventionMetadataDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConventionMetadataDto {
    pub year_basis: String,
    pub kua_five_resolution: String,
    pub gender_encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayInfoDto {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub jd: i32,
    pub canchi: CanChiInfoDto,
    pub tiet_khi: TietKhiDto,
    pub gio_hoang_dao: GioHoangDaoDto,
    pub day_fortune: Option<DayFortuneDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayDto {
    pub name: String,
    pub description: String,
    pub solar_day: i32,
    pub solar_month: i32,
    pub solar_year: i32,
    pub lunar_day: Option<i32>,
    pub lunar_month: Option<i32>,
    pub lunar_year: Option<i32>,
    pub is_solar: bool,
    pub category: String,
    pub is_major: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedTextDto {
    pub vi: String,
    pub en: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalizedListDto {
    pub vi: Vec<String>,
    pub en: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodInsightDto {
    pub name: LocalizedTextDto,
    pub description: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabooInsightDto {
    pub action: LocalizedTextDto,
    pub reason: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProverbInsightDto {
    pub text: String,
    pub meaning: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionsInsightDto {
    pub north: LocalizedTextDto,
    pub central: LocalizedTextDto,
    pub south: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FestivalInsightDto {
    pub names: LocalizedListDto,
    pub origin: Option<LocalizedTextDto>,
    pub activities: Option<LocalizedListDto>,
    pub food: Vec<FoodInsightDto>,
    pub taboos: Vec<TabooInsightDto>,
    pub proverbs: Vec<ProverbInsightDto>,
    pub regions: Option<RegionsInsightDto>,
    pub category: String,
    pub is_major: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayInsightDto {
    pub names: LocalizedListDto,
    pub origin: Option<LocalizedTextDto>,
    pub significance: Option<LocalizedTextDto>,
    pub activities: Option<LocalizedListDto>,
    pub traditions: Option<LocalizedListDto>,
    pub food: Vec<FoodInsightDto>,
    pub taboos: Vec<TabooInsightDto>,
    pub proverbs: Vec<ProverbInsightDto>,
    pub regions: Option<RegionsInsightDto>,
    pub category: String,
    pub is_major: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInsightDto {
    pub key: String,
    pub name: LocalizedTextDto,
    pub nature: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanInsightDto {
    pub name: String,
    pub element: String,
    pub meaning: LocalizedTextDto,
    pub nature: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChiInsightDto {
    pub name: String,
    pub animal: LocalizedTextDto,
    pub element: String,
    pub meaning: LocalizedTextDto,
    pub hours: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanChiInsightDto {
    pub can: CanInsightDto,
    pub chi: ChiInsightDto,
    pub element: Option<ElementInsightDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayGuidanceDto {
    pub good_for: LocalizedListDto,
    pub avoid_for: LocalizedListDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TietKhiInsightDto {
    pub id: String,
    pub name: LocalizedTextDto,
    pub longitude: i32,
    pub meaning: LocalizedTextDto,
    pub astronomy: LocalizedTextDto,
    pub agriculture: LocalizedListDto,
    pub health: LocalizedListDto,
    pub weather: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayInsightDto {
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub festival: Option<FestivalInsightDto>,
    pub holiday: Option<HolidayInsightDto>,
    pub canchi: Option<CanChiInsightDto>,
    pub day_guidance: Option<DayGuidanceDto>,
    pub tiet_khi: Option<TietKhiInsightDto>,
}

/// Na Am lookup result with cycle index and evidence metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaAmLookupResultDto {
    /// 1-based cycle index (1-60)
    pub cycle_index: u8,
    /// Heavenly stem (Vietnamese name)
    pub can: String,
    /// Earthly branch (Vietnamese name)
    pub chi: String,
    /// Na Am value (e.g., "Hải Trung Kim")
    pub na_am: String,
    /// Five element (last word of na_am)
    pub element: String,
    /// Evidence: source identifier
    pub source_id: String,
    /// Evidence: computation method
    pub method: String,
    /// Evidence: ruleset profile
    pub profile: String,
}

/// Na Am lookup error with deterministic error type and human-readable message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaAmErrorDto {
    /// Error type: "invalid_cycle_index" | "invalid_stem_branch_pair" | "unknown_stem" | "unknown_branch"
    pub error: String,
    /// Human-readable error description
    pub message: String,
}

/// Na Am API response with success or error variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NaAmResponseDto {
    Success(NaAmLookupResultDto),
    Error(NaAmErrorDto),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_fortune_dto_has_optional_ten_gods_and_tu_menh_fields() {
        // Test 1: DTO has matching optional ten_gods and tu_menh fields
        let dto = DayFortuneDto {
            ruleset_id: "test".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            day_element: DayElementDto {
                na_am: "test".to_string(),
                element: "Kim".to_string(),
                can_element: "Mộc".to_string(),
                chi_element: "Thổ".to_string(),
                evidence: None,
            },
            conflict: DayConflictDto {
                opposing_chi: "Tuất".to_string(),
                opposing_con_giap: "Tuất (Chó)".to_string(),
                tuoi_xung: vec![],
                sat_huong: "Nam".to_string(),
                evidence: None,
            },
            travel: TravelDirectionDto {
                xuat_hanh_huong: "Đông Nam".to_string(),
                tai_than: "Tây Nam".to_string(),
                hy_than: "Đông Bắc".to_string(),
                evidence: None,
            },
            stars: DayStarsDto {
                cat_tinh: vec![],
                sat_tinh: vec![],
                day_star: None,
                star_system: None,
                evidence: None,
                matched_rules: vec![],
            },
            day_deity: None,
            taboos: vec![],
            xung_hop: XungHopDto {
                luc_xung: "Tuất".to_string(),
                tam_hop: vec![],
                tu_hanh_xung: vec![],
            },
            truc: TrucDto {
                index: 0,
                name: "Kiến".to_string(),
                quality: "cat".to_string(),
                evidence: None,
            },
            ten_gods: None,
            tu_menh: None,
        };

        // Verify fields exist and are optional
        let _ = dto.ten_gods;
        let _ = dto.tu_menh;
    }

    #[test]
    fn day_ten_gods_dto_serializes_with_snake_case() {
        // Test 3: JSON serialization matches expected stable field names (snake_case)
        let ten_gods = DayTenGodsDto {
            to_year_stem: None,
            to_self: None,
        };

        let json = serde_json::to_string(&ten_gods).expect("serialize");
        // Verify snake_case field names
        assert!(json.contains("\"to_year_stem\""));
        assert!(json.contains("\"to_self\""));
    }
}
