use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DateQuery {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub timezone: Option<f64>,
    pub ruleset_id: Option<String>,
    pub event_kind: Option<String>,
    #[serde(default)]
    pub enabled_pack_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziQuery {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub hour: u8,
    pub minute: u8,
    pub timezone: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(default)]
    pub use_solar_time: bool,
    pub gender: Option<String>,
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
    pub liu_he: Option<String>,
    pub xiang_hai: Option<String>,
    pub xiang_xing: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangCanDto {
    pub main: String,
    pub central: String,
    pub residual: String,
    pub strength: [u8; 3],
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
    pub tang_can: Option<TangCanDto>,
    /// Ten Gods relations for predefined targets (populated when day stem available)
    pub ten_gods: Option<DayTenGodsDto>,
    /// Kua (Tu Mến) result (populated only when birth year and gender provided)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu_menh: Option<KuaResultDto>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationScopeDto {
    GeneralDay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationBucketDto {
    Nen,
    CoThe,
    Tranh,
    KyManh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationSeverityDto {
    Primary,
    Supporting,
    Override,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationEvidenceSourceDto {
    DayGuidance,
    Truc,
    Stars,
    DayDeity,
    Taboo,
    XungHop,
    TietKhi,
    GioHoangDao,
    Travel,
    ProductRule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLabelDto {
    pub vi: String,
    pub en: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationEvidenceDto {
    pub source: RecommendationEvidenceSourceDto,
    pub code: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationReasonDto {
    pub rule_id: String,
    pub severity: RecommendationSeverityDto,
    pub summary_vi: String,
    pub summary_en: String,
    pub evidence: RecommendationEvidenceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynthesizedRecommendationDto {
    pub activity_id: String,
    pub label: ActivityLabelDto,
    pub bucket: RecommendationBucketDto,
    #[serde(default)]
    pub reasons: Vec<RecommendationReasonDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRecommendationsDto {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub scope: RecommendationScopeDto,
    pub version: String,
    pub summary_vi: String,
    pub summary_en: String,
    #[serde(default)]
    pub active_packs: Vec<ActiveRecommendationPackDto>,
    #[serde(default)]
    pub activities: Vec<SynthesizedRecommendationDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRecommendationPackDto {
    pub pack_id: String,
    pub version: String,
    pub source_family: String,
    pub mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziCanChiDto {
    pub can: String,
    pub chi: String,
    pub full: String,
    pub can_index: usize,
    pub chi_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziLunarDateDto {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub is_leap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiddenStemEntryDto {
    pub stem_symbol: String,
    pub stem_name: Option<String>,
    pub strength: u8,
    pub ten_god_to_day_master: Option<ThapThanResultDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziPillarDto {
    pub kind: String,
    pub can_chi: BaziCanChiDto,
    pub hidden_stems: Vec<HiddenStemEntryDto>,
    pub na_am: Option<String>,
    pub stem_relation_to_day_master: Option<ThapThanResultDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziChartMetadataDto {
    pub timezone: f64,
    pub use_solar_time: bool,
    pub year_basis: String,
    pub month_basis: String,
    pub day_basis: String,
    pub hour_basis: String,
    pub hour_evidence: Option<RuleEvidenceDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziChartDto {
    pub input: BaziQuery,
    pub tier: BirthDataTierDto,
    pub lunar_date: BaziLunarDateDto,
    pub day_master: BaziCanChiDto,
    pub pillars: Vec<BaziPillarDto>,
    pub metadata: BaziChartMetadataDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementDistributionDto {
    pub moc: u16,
    pub hoa: u16,
    pub tho: u16,
    pub kim: u16,
    pub thuy: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayMasterStrengthDto {
    pub score: i32,
    pub label: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartInteractionDto {
    pub kind: String,
    pub participants: Vec<String>,
    pub summary_vi: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenGodDistributionDto {
    pub ty_kien: u8,
    pub kiep_tai: u8,
    pub thuc_than: u8,
    pub thuong_quan: u8,
    pub chinh_tai: u8,
    pub thien_tai: u8,
    pub chinh_quan: u8,
    pub that_sat: u8,
    pub chinh_an: u8,
    pub thien_an: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziAnalysisDto {
    pub tier: BirthDataTierDto,
    pub element_distribution: ElementDistributionDto,
    pub day_master_strength: DayMasterStrengthDto,
    pub interactions: Vec<ChartInteractionDto>,
    pub ten_god_distribution: TenGodDistributionDto,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unavailable_sections: Vec<UnavailableSectionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziTimingQuery {
    pub current_age: f64,
    pub target_year: i32,
    pub months: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziLuckPillarDto {
    pub index: usize,
    pub can_chi: String,
    pub start_age: f64,
    pub end_age: f64,
    pub ten_god_to_day_master: Option<ThapThanResultDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualPillarDto {
    pub year: i32,
    pub can_chi: String,
    pub branch: String,
    pub ten_god_to_day_master: Option<ThapThanResultDto>,
    pub interactions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyPillarDto {
    pub year: i32,
    pub month: i32,
    pub can_chi: String,
    pub branch: String,
    pub ten_god_to_day_master: Option<ThapThanResultDto>,
    pub interactions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziTimingDto {
    pub dai_van: Vec<BaziLuckPillarDto>,
    pub active_dai_van: Option<BaziLuckPillarDto>,
    pub annual: AnnualPillarDto,
    pub monthly: Vec<MonthlyPillarDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsefulGodDto {
    pub favorable_elements: Vec<String>,
    pub unfavorable_elements: Vec<String>,
    pub tentative_yong_shen: Option<String>,
    pub tentative_xi_shen: Option<String>,
    pub confidence: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziAdvisoryDomainsDto {
    pub career: Vec<String>,
    pub wealth: Vec<String>,
    pub relationship: Vec<String>,
    pub health: Vec<String>,
    pub timing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziAdvisoryDto {
    pub summary: String,
    pub severity: String,
    pub top_signals: Vec<String>,
    pub why_this_matters: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub priority_order: Vec<String>,
    pub useful_god_analysis: UsefulGodDto,
    pub summary_vi: String,
    pub warnings: Vec<String>,
    pub domains: BaziAdvisoryDomainsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziInteractionMetricDto {
    pub kind: String,
    pub participants: Vec<String>,
    pub impact: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziScoreContributorDto {
    pub signal: String,
    pub delta: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziDomainScoreDto {
    pub score: u8,
    pub label: String,
    pub confidence: f32,
    pub contributors: Vec<BaziScoreContributorDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziDomainScoresDto {
    pub career: BaziDomainScoreDto,
    pub wealth: BaziDomainScoreDto,
    pub relationship: BaziDomainScoreDto,
    pub health: BaziDomainScoreDto,
    pub timing: BaziDomainScoreDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziTimingWindowScoreDto {
    pub month: i32,
    pub score: f32,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziTimingMetricsDto {
    pub current_dai_van_alignment: Option<f32>,
    pub annual_alignment: Option<f32>,
    pub monthly_windows: Vec<BaziTimingWindowScoreDto>,
    pub activation_summary: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziCoreMetricsDto {
    pub day_master_strength_score: i32,
    pub day_master_strength_label: String,
    pub season_support_score: f32,
    pub same_element_score: u16,
    pub resource_support_score: u16,
    pub drain_pressure_score: u16,
    pub control_pressure_score: u16,
    pub element_balance_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziStructureMetricsDto {
    pub dominant_elements: Vec<String>,
    pub weak_elements: Vec<String>,
    pub dominant_ten_gods: Vec<String>,
    pub interaction_score: f32,
    pub notable_interactions: Vec<BaziInteractionMetricDto>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziComputedMetricsDto {
    pub tier: BirthDataTierDto,
    pub core_metrics: BaziCoreMetricsDto,
    pub structure_metrics: BaziStructureMetricsDto,
    pub domain_scores: BaziDomainScoresDto,
    pub timing_metrics: BaziTimingMetricsDto,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unavailable_sections: Vec<UnavailableSectionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziReportDto {
    pub summary: String,
    pub severity: String,
    pub top_signals: Vec<String>,
    pub why_this_matters: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub priority_order: Vec<String>,
    pub chart: BaziChartDto,
    pub analysis: BaziAnalysisDto,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timing: Option<BaziTimingDto>,
    pub computed_metrics: BaziComputedMetricsDto,
    pub advisory: BaziAdvisoryDto,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InsightSurface {
    Chart,
    Analysis,
    Timing,
    Advisory,
    Metrics,
    Report,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetDefaultsDto {
    pub tz_offset: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meridian: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetSourceNoteDto {
    pub family: String,
    pub source_id: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulesetCatalogEntryDto {
    pub id: String,
    pub canonical_id: String,
    pub version: String,
    pub region: String,
    pub profile: String,
    pub schema_version: String,
    pub is_default: bool,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub defaults: RulesetDefaultsDto,
    #[serde(default)]
    pub source_notes: Vec<RulesetSourceNoteDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationPackCatalogEntryDto {
    pub pack_id: String,
    pub request_field: String,
    pub version: String,
    pub source_family: String,
    pub mode: String,
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
    pub profile: String,
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub jd: i32,
    pub canchi: CanChiInfoDto,
    pub tiet_khi: TietKhiDto,
    pub gio_hoang_dao: GioHoangDaoDto,
    pub day_fortune: Option<DayFortuneDto>,
    pub daily_recommendations: DailyRecommendationsDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_recommendations: Option<DailyRecommendationsDto>,
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
pub struct UpcomingEventDto {
    pub name: String,
    pub days_left: i32,
    pub is_lunar: bool,
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
pub struct NaAmInsightDto {
    pub na_am: String,
    pub element: String,
    pub meaning: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrucInsightDto {
    pub name: String,
    pub quality: String,
    pub meaning: LocalizedTextDto,
    pub good_for: LocalizedListDto,
    pub avoid_for: LocalizedListDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayDeityInsightDto {
    pub name: String,
    pub classification: String,
    pub classification_meaning: LocalizedTextDto,
    pub deity_meaning: Option<LocalizedTextDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarsInsightDto {
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
    pub day_star: Option<String>,
    pub day_star_quality: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabooInsightItemDto {
    pub name: String,
    pub severity: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelInsightDto {
    pub xuat_hanh_huong: String,
    pub tai_than: String,
    pub hy_than: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XungHopInsightDto {
    pub luc_xung: String,
    pub tam_hop: Vec<String>,
    pub liu_he: Option<String>,
    pub xiang_hai: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TangCanInsightDto {
    pub main: String,
    pub central: String,
    pub residual: String,
    pub strength: [u8; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenGodsEntryInsightDto {
    pub label: String,
    pub name: LocalizedTextDto,
    pub meaning: LocalizedTextDto,
    pub relation: String,
    pub same_polarity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenGodsInsightDto {
    pub to_year_stem: Option<TenGodsEntryInsightDto>,
    pub to_self: Option<TenGodsEntryInsightDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourInsightEntryDto {
    pub chi: String,
    pub time_range: String,
    pub star: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoursInsightDto {
    pub good_hour_count: usize,
    pub good_hours: Vec<HourInsightEntryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TuMenhInsightDto {
    pub kua: u8,
    pub group: String,
    pub trigram: LocalizedTextDto,
    pub direction: LocalizedTextDto,
    pub meaning: LocalizedTextDto,
    pub group_meaning: LocalizedTextDto,
    pub favorable_directions: Vec<String>,
    pub unfavorable_directions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaiVanPillarInsightDto {
    pub index: usize,
    pub can_chi: String,
    pub start_age: f64,
    pub end_age: f64,
    pub element: String,
    pub element_meaning: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaiVanInsightDto {
    pub direction: String,
    pub direction_meaning: LocalizedTextDto,
    pub start_age: String,
    pub current_pillar: Option<DaiVanPillarInsightDto>,
    pub all_pillars: Vec<DaiVanPillarInsightDto>,
    pub phases_meaning: LocalizedTextDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayInsightDto {
    pub solar: SolarDto,
    pub lunar: LunarDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub festival: Option<FestivalInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holiday: Option<HolidayInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canchi: Option<CanChiInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_guidance: Option<DayGuidanceDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiet_khi: Option<TietKhiInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub na_am: Option<NaAmInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truc: Option<TrucInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_deity: Option<DayDeityInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stars: Option<StarsInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taboos: Option<Vec<TabooInsightItemDto>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel: Option<TravelInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xung_hop: Option<XungHopInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tang_can: Option<TangCanInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ten_gods: Option<TenGodsInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours: Option<HoursInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu_menh: Option<TuMenhInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaiVanInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yearly_han: Option<YearlyHanInsightDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TamTaiInsightDto {
    pub in_tam_tai: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_position: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    pub tam_hop_group: Vec<String>,
    pub tai_years: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KimLauInsightDto {
    pub in_kim_lau: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub remainder: u8,
    pub tuoi_mu: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoangOcInsightDto {
    pub position: u8,
    pub position_name: String,
    pub is_good: bool,
    pub tuoi_mu: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuuDieuInsightDto {
    pub star_index: u8,
    pub star_name: String,
    pub quality: String,
    pub is_han: bool,
    pub element: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThaiTueConflictDto {
    pub kind: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThaiTueInsightDto {
    pub conflicts: Vec<ThaiTueConflictDto>,
    pub has_conflict: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyHanInsightDto {
    pub sao_han: CuuDieuInsightDto,
    pub tam_tai: TamTaiInsightDto,
    pub kim_lau: KimLauInsightDto,
    pub hoang_oc: HoangOcInsightDto,
    pub thai_tue: ThaiTueInsightDto,
    pub han_count: u8,
    pub is_chong_han: bool,
    pub severity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDayQueryDto {
    pub date: DateQuery,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BirthDataTierDto {
    Anonymous,
    Date,
    Datetime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UnavailableSectionDto {
    pub section: String,
    pub reason: String,
    pub required_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDayChartDto {
    pub input: PersonalDayQueryDto,
    pub tier: BirthDataTierDto,
    pub solar: SolarDto,
    pub lunar: LunarDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canchi: Option<CanChiInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiet_khi: Option<TietKhiInsightDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDayAnalysisDto {
    pub tier: BirthDataTierDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<amlich_core::reasoning::InitiationOpeningDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_export: Option<amlich_core::reasoning::InitiationOpeningDecisionExport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<amlich_core::reasoning::ReasoningGraphExport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ten_gods: Option<TenGodsInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xung_hop: Option<XungHopInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tang_can: Option<TangCanInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tu_menh: Option<TuMenhInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dai_van: Option<DaiVanInsightDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yearly_han: Option<YearlyHanInsightDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unavailable_sections: Vec<UnavailableSectionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDayMetricsDto {
    pub tier: BirthDataTierDto,
    pub profile_completeness: u8,
    pub available_sections: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unavailable_sections: Vec<UnavailableSectionDto>,
    pub has_personal_recommendations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDayAdvisoryDto {
    pub summary: String,
    pub severity: String,
    pub top_signals: Vec<String>,
    pub why_this_matters: Vec<String>,
    pub recommended_actions: Vec<String>,
    pub priority_order: Vec<String>,
    pub highlights: Vec<String>,
    pub cautions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_bucket: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_confidence: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDayReportDto {
    pub summary: String,
    pub severity: String,
    pub top_signals: Vec<String>,
    pub chart: PersonalDayChartDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision: Option<amlich_core::reasoning::InitiationOpeningDecision>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_export: Option<amlich_core::reasoning::InitiationOpeningDecisionExport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph: Option<amlich_core::reasoning::ReasoningGraphExport>,
    pub analysis: PersonalDayAnalysisDto,
    pub computed_metrics: PersonalDayMetricsDto,
    pub advisory: PersonalDayAdvisoryDto,
}

/// Query for the personal day matrix report.
/// Requires full birth datetime (for Bazi chart) + target date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDayMatrixQueryDto {
    /// Birth data (full datetime required for Bazi chart).
    pub birth: BaziQuery,
    /// Target date to evaluate.
    pub date: DateQuery,
}

/// Unified report containing all interaction matrices.
///
/// Cross-references a day's almanac data with personal Bazi data
/// to produce interconnected personal matrices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalDayMatrixReportDto {
    pub input: PersonalDayMatrixQueryDto,
    pub tier: BirthDataTierDto,
    /// Matrix 1: how today's Can Chi interacts with each personal pillar.
    pub day_person: amlich_core::interaction::types::DayPersonMatrix,
    /// Matrix 2: element resonance between today and person's element distribution.
    pub element_resonance: amlich_core::interaction::types::ElementResonanceMatrix,
    /// Matrix 3: personal ranking of each of 12 traditional hours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_hours: Option<amlich_core::interaction::types::PersonalHourMatrix>,
    /// Matrix 4a: unified direction scores from Kua + day deities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_merge: Option<amlich_core::interaction::types::DirectionMergeMatrix>,
    /// Matrix 4b: domain scores boosted by day-level signals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_day_boost: Option<amlich_core::interaction::types::DomainDayBoostMatrix>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unavailable_sections: Vec<UnavailableSectionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourSelectionQueryDto {
    pub date: DateQuery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourSelectionChartDto {
    pub input: HourSelectionQueryDto,
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub gio_hoang_dao: GioHoangDaoDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourSelectionAnalysisDto {
    pub intent: String,
    pub summary_vi: String,
    pub summary_en: String,
    pub good_hours: Vec<HourInfoDto>,
    pub bad_hours: Vec<HourInfoDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_recommendation: Option<HourInfoDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical: Option<amlich_core::HourSelectionReasoningExport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourSelectionMetricsDto {
    pub good_hour_count: usize,
    pub bad_hour_count: usize,
    pub good_hour_ratio: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourSelectionAdvisoryDto {
    pub intent: String,
    pub summary_vi: String,
    pub summary_en: String,
    pub best_windows: Vec<String>,
    pub caution_windows: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canonical: Option<amlich_core::HourSelectionReasoningExport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourSelectionReportDto {
    pub chart: HourSelectionChartDto,
    pub analysis: HourSelectionAnalysisDto,
    pub computed_metrics: HourSelectionMetricsDto,
    pub advisory: HourSelectionAdvisoryDto,
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

// ---------------------------------------------------------------------------
// Bazi Derived Report DTOs (Thai Nguyên, Mệnh Cung, Không Vong, Thần Sát)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThaiNguyenDto {
    pub can_chi: BaziCanChiDto,
    pub evidence: RuleEvidenceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenhCungDto {
    pub menh_cung: BaziCanChiDto,
    pub than_cung: BaziCanChiDto,
    pub evidence: RuleEvidenceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhongVongPairDto {
    pub branch_indices: [usize; 2],
    pub branch_names: [String; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhongVongPillarEntryDto {
    pub pillar: String,
    pub void_pair: KhongVongPairDto,
    pub hits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KhongVongAnalysisDto {
    pub entries: Vec<KhongVongPillarEntryDto>,
    pub evidence: RuleEvidenceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThanSatEntryDto {
    pub name: String,
    pub source: String,
    pub target_branch: usize,
    pub target_branch_name: String,
    pub present_in: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThanSatResultDto {
    pub stars: Vec<ThanSatEntryDto>,
    pub evidence: RuleEvidenceDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaziDerivedReportDto {
    pub input: BaziQuery,
    pub tier: BirthDataTierDto,
    pub thai_nguyen: ThaiNguyenDto,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menh_cung: Option<MenhCungDto>,
    pub khong_vong: KhongVongAnalysisDto,
    pub than_sat: ThanSatResultDto,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unavailable_sections: Vec<UnavailableSectionDto>,
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
                liu_he: None,
                xiang_hai: None,
                xiang_xing: None,
            },
            truc: TrucDto {
                index: 0,
                name: "Kiến".to_string(),
                quality: "cat".to_string(),
                evidence: None,
            },
            tang_can: None,
            ten_gods: None,
            tu_menh: None,
        };

        // Verify fields exist and are optional
        let _ = dto.ten_gods;
        let _ = dto.tu_menh;
    }

    #[test]
    fn personal_day_matrix_report_dto_exists() {
        let _ = std::mem::size_of::<PersonalDayMatrixReportDto>();
    }

    #[test]
    fn bazi_derived_report_dto_serializes() {
        let report = BaziDerivedReportDto {
            tier: BirthDataTierDto::Datetime,
            input: BaziQuery {
                year: 1990,
                month: 6,
                day: 15,
                hour: 10,
                minute: 30,
                timezone: Some(7.0),
                longitude: None,
                use_solar_time: false,
                gender: Some("male".to_string()),
            },
            thai_nguyen: ThaiNguyenDto {
                can_chi: BaziCanChiDto {
                    can: "Ất".into(),
                    chi: "Tỵ".into(),
                    full: "Ất Tỵ".into(),
                    can_index: 1,
                    chi_index: 5,
                },
                evidence: RuleEvidenceDto {
                    source_id: "bazi-classical".into(),
                    method: "thai-nguyen-month-plus-3".into(),
                    profile: "baseline".into(),
                },
            },
            menh_cung: Some(MenhCungDto {
                menh_cung: BaziCanChiDto {
                    can: "Bính".into(),
                    chi: "Dần".into(),
                    full: "Bính Dần".into(),
                    can_index: 2,
                    chi_index: 2,
                },
                than_cung: BaziCanChiDto {
                    can: "Canh".into(),
                    chi: "Ngọ".into(),
                    full: "Canh Ngọ".into(),
                    can_index: 6,
                    chi_index: 6,
                },
                evidence: RuleEvidenceDto {
                    source_id: "bazi-classical".into(),
                    method: "menh-cung-month-hour-counter".into(),
                    profile: "baseline".into(),
                },
            }),
            khong_vong: KhongVongAnalysisDto {
                entries: vec![],
                evidence: RuleEvidenceDto {
                    source_id: "bazi-classical".into(),
                    method: "khong-vong-tuan-lookup".into(),
                    profile: "baseline".into(),
                },
            },
            than_sat: ThanSatResultDto {
                stars: vec![],
                evidence: RuleEvidenceDto {
                    source_id: "bazi-classical".into(),
                    method: "than-sat-lookup-tables".into(),
                    profile: "baseline".into(),
                },
            },
            unavailable_sections: Vec::new(),
        };

        let json = serde_json::to_string(&report).expect("serialize");
        assert!(json.contains("\"thai_nguyen\""));
        assert!(json.contains("\"menh_cung\""));
        assert!(json.contains("\"khong_vong\""));
        assert!(json.contains("\"than_sat\""));
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
