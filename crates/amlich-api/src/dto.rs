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
pub struct DayInfoDto {
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub jd: i32,
    pub canchi: CanChiInfoDto,
    pub tiet_khi: TietKhiDto,
    pub gio_hoang_dao: GioHoangDaoDto,
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
