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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayConflictDto {
    pub opposing_chi: String,
    pub opposing_con_giap: String,
    pub tuoi_xung: Vec<String>,
    pub sat_huong: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelDirectionDto {
    pub xuat_hanh_huong: String,
    pub tai_than: String,
    pub hy_than: String,
    pub ky_than: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayStarDto {
    pub system: String,
    pub index: usize,
    pub name: String,
    pub quality: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayStarsDto {
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
    pub day_star: Option<DayStarDto>,
    pub star_system: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayFortuneDto {
    pub profile: String,
    pub day_element: DayElementDto,
    pub conflict: DayConflictDto,
    pub travel: TravelDirectionDto,
    pub stars: DayStarsDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayInfoDto {
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
