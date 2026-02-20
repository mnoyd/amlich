use amlich_core::holidays::get_vietnamese_holidays;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Verbosity {
    Basic,
    #[default]
    Standard,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DateQuery {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub timezone: Option<f64>,
    pub locale: Option<String>,
    pub verbosity: Option<Verbosity>,
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

fn convert_canchi(cc: &amlich_core::CanChi) -> CanChiDto {
    CanChiDto {
        can_index: cc.can_index,
        chi_index: cc.chi_index,
        can: cc.can.clone(),
        chi: cc.chi.clone(),
        full: cc.full.clone(),
        con_giap: cc.con_giap.clone(),
        ngu_hanh: NguHanhDto {
            can: cc.ngu_hanh.can.clone(),
            chi: cc.ngu_hanh.chi.clone(),
        },
    }
}

fn convert_day_info(info: &amlich_core::DayInfo) -> DayInfoDto {
    DayInfoDto {
        solar: SolarDto {
            day: info.solar.day,
            month: info.solar.month,
            year: info.solar.year,
            day_of_week: info.solar.day_of_week,
            day_of_week_name: info.solar.day_of_week_name.clone(),
            date_string: info.solar.date_string.clone(),
        },
        lunar: LunarDto {
            day: info.lunar.day,
            month: info.lunar.month,
            year: info.lunar.year,
            is_leap_month: info.lunar.is_leap_month,
            date_string: info.lunar.date_string.clone(),
        },
        jd: info.jd,
        canchi: CanChiInfoDto {
            day: convert_canchi(&info.canchi.day),
            month: convert_canchi(&info.canchi.month),
            year: convert_canchi(&info.canchi.year),
            full: info.canchi.full.clone(),
        },
        tiet_khi: TietKhiDto {
            index: info.tiet_khi.index,
            name: info.tiet_khi.name.clone(),
            description: info.tiet_khi.description.clone(),
            longitude: info.tiet_khi.longitude,
            current_longitude: info.tiet_khi.current_longitude,
            season: info.tiet_khi.season.clone(),
        },
        gio_hoang_dao: GioHoangDaoDto {
            day_chi: info.gio_hoang_dao.day_chi.clone(),
            good_hour_count: info.gio_hoang_dao.good_hour_count,
            good_hours: info
                .gio_hoang_dao
                .good_hours
                .iter()
                .map(|h| HourInfoDto {
                    hour_index: h.hour_index,
                    hour_chi: h.hour_chi.clone(),
                    time_range: h.time_range.clone(),
                    star: h.star.clone(),
                    is_good: h.is_good,
                })
                .collect(),
            all_hours: info
                .gio_hoang_dao
                .all_hours
                .iter()
                .map(|h| HourInfoDto {
                    hour_index: h.hour_index,
                    hour_chi: h.hour_chi.clone(),
                    time_range: h.time_range.clone(),
                    star: h.star.clone(),
                    is_good: h.is_good,
                })
                .collect(),
            summary: info.gio_hoang_dao.summary.clone(),
        },
    }
}

pub fn get_day_info(query: &DateQuery) -> Result<DayInfoDto, String> {
    if !(1..=12).contains(&query.month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&query.day) {
        return Err("day must be 1-31".to_string());
    }

    let tz = query.timezone.unwrap_or(amlich_core::VIETNAM_TIMEZONE);
    let info = amlich_core::get_day_info_with_timezone(query.day, query.month, query.year, tz);
    Ok(convert_day_info(&info))
}

pub fn get_day_info_for_date(day: i32, month: i32, year: i32) -> Result<DayInfoDto, String> {
    get_day_info(&DateQuery {
        day,
        month,
        year,
        timezone: None,
        locale: Some("vi-VN".to_string()),
        verbosity: Some(Verbosity::Standard),
    })
}

pub fn get_holidays(year: i32, major_only: bool) -> Vec<HolidayDto> {
    get_vietnamese_holidays(year)
        .into_iter()
        .filter(|h| !major_only || h.is_major)
        .map(|h| HolidayDto {
            name: h.name,
            description: h.description,
            solar_day: h.solar_day,
            solar_month: h.solar_month,
            solar_year: h.solar_year,
            lunar_day: h.lunar_date.as_ref().map(|d| d.day),
            lunar_month: h.lunar_date.as_ref().map(|d| d.month),
            lunar_year: h.lunar_date.as_ref().map(|d| d.year),
            is_solar: h.is_solar,
            category: h.category,
            is_major: h.is_major,
        })
        .collect()
}
