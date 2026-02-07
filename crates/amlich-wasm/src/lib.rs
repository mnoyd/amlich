// amlich-wasm - WASM bindings for web usage

use amlich_core::{
    format_day_info, get_day_info, get_day_info_with_timezone,
    holidays::{get_major_holidays, get_vietnamese_holidays},
    lunar::{convert_lunar_to_solar, convert_solar_to_lunar},
    VIETNAM_TIMEZONE,
};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct WasmLunarDate {
    day: i32,
    month: i32,
    year: i32,
    is_leap: bool,
}

#[derive(Serialize)]
struct WasmSolarDate {
    day: i32,
    month: i32,
    year: i32,
}

#[derive(Serialize)]
struct WasmDayInfo {
    solar: WasmSolarInfo,
    lunar: WasmLunarInfo,
    jd: i32,
    canchi: WasmCanChiInfo,
    tiet_khi: WasmTietKhi,
    gio_hoang_dao: WasmGioHoangDao,
}

#[derive(Serialize)]
struct WasmSolarInfo {
    day: i32,
    month: i32,
    year: i32,
    day_of_week: usize,
    day_of_week_name: String,
    date_string: String,
}

#[derive(Serialize)]
struct WasmLunarInfo {
    day: i32,
    month: i32,
    year: i32,
    is_leap_month: bool,
    date_string: String,
}

#[derive(Serialize)]
struct WasmCanChi {
    can_index: usize,
    chi_index: usize,
    can: String,
    chi: String,
    full: String,
    con_giap: String,
    ngu_hanh_can: String,
    ngu_hanh_chi: String,
}

#[derive(Serialize)]
struct WasmCanChiInfo {
    day: WasmCanChi,
    month: WasmCanChi,
    year: WasmCanChi,
    full: String,
}

#[derive(Serialize)]
struct WasmTietKhi {
    index: usize,
    name: String,
    description: String,
    longitude: i32,
    current_longitude: f64,
    season: String,
}

#[derive(Serialize)]
struct WasmHourInfo {
    hour_index: usize,
    hour_chi: String,
    time_range: String,
    star: String,
    is_good: bool,
}

#[derive(Serialize)]
struct WasmGioHoangDao {
    day_chi: String,
    good_hour_count: usize,
    good_hours: Vec<WasmHourInfo>,
    all_hours: Vec<WasmHourInfo>,
    summary: String,
}

#[derive(Serialize)]
struct WasmHoliday {
    name: String,
    description: String,
    solar_day: i32,
    solar_month: i32,
    solar_year: i32,
    lunar_day: Option<i32>,
    lunar_month: Option<i32>,
    lunar_year: Option<i32>,
    is_solar: bool,
}

fn convert_canchi(cc: &amlich_core::CanChi) -> WasmCanChi {
    WasmCanChi {
        can_index: cc.can_index,
        chi_index: cc.chi_index,
        can: cc.can.clone(),
        chi: cc.chi.clone(),
        full: cc.full.clone(),
        con_giap: cc.con_giap.clone(),
        ngu_hanh_can: cc.ngu_hanh.can.clone(),
        ngu_hanh_chi: cc.ngu_hanh.chi.clone(),
    }
}

fn convert_day_info(info: &amlich_core::DayInfo) -> WasmDayInfo {
    WasmDayInfo {
        solar: WasmSolarInfo {
            day: info.solar.day,
            month: info.solar.month,
            year: info.solar.year,
            day_of_week: info.solar.day_of_week,
            day_of_week_name: info.solar.day_of_week_name.clone(),
            date_string: info.solar.date_string.clone(),
        },
        lunar: WasmLunarInfo {
            day: info.lunar.day,
            month: info.lunar.month,
            year: info.lunar.year,
            is_leap_month: info.lunar.is_leap_month,
            date_string: info.lunar.date_string.clone(),
        },
        jd: info.jd,
        canchi: WasmCanChiInfo {
            day: convert_canchi(&info.canchi.day),
            month: convert_canchi(&info.canchi.month),
            year: convert_canchi(&info.canchi.year),
            full: info.canchi.full.clone(),
        },
        tiet_khi: WasmTietKhi {
            index: info.tiet_khi.index,
            name: info.tiet_khi.name.clone(),
            description: info.tiet_khi.description.clone(),
            longitude: info.tiet_khi.longitude,
            current_longitude: info.tiet_khi.current_longitude,
            season: info.tiet_khi.season.clone(),
        },
        gio_hoang_dao: WasmGioHoangDao {
            day_chi: info.gio_hoang_dao.day_chi.clone(),
            good_hour_count: info.gio_hoang_dao.good_hour_count,
            good_hours: info
                .gio_hoang_dao
                .good_hours
                .iter()
                .map(|h| WasmHourInfo {
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
                .map(|h| WasmHourInfo {
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

/// Get comprehensive day information for a solar date
///
/// Returns a JavaScript object with solar, lunar, canchi, tiet_khi, and gio_hoang_dao info
#[wasm_bindgen]
pub fn get_day_info_js(day: i32, month: i32, year: i32) -> JsValue {
    let info = get_day_info(day, month, year);
    let wasm_info = convert_day_info(&info);
    serde_wasm_bindgen::to_value(&wasm_info).unwrap_or(JsValue::NULL)
}

/// Get comprehensive day information with custom timezone
#[wasm_bindgen]
pub fn get_day_info_with_timezone_js(day: i32, month: i32, year: i32, time_zone: f64) -> JsValue {
    let info = get_day_info_with_timezone(day, month, year, time_zone);
    let wasm_info = convert_day_info(&info);
    serde_wasm_bindgen::to_value(&wasm_info).unwrap_or(JsValue::NULL)
}

/// Get formatted day info as a string
#[wasm_bindgen]
pub fn format_day_info_js(day: i32, month: i32, year: i32) -> String {
    let info = get_day_info(day, month, year);
    format_day_info(&info)
}

/// Convert solar date to lunar date
///
/// Returns { day, month, year, is_leap }
#[wasm_bindgen]
pub fn solar_to_lunar(day: i32, month: i32, year: i32) -> JsValue {
    let lunar = convert_solar_to_lunar(day, month, year, VIETNAM_TIMEZONE);
    let result = WasmLunarDate {
        day: lunar.day,
        month: lunar.month,
        year: lunar.year,
        is_leap: lunar.is_leap,
    };
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Convert solar date to lunar date with custom timezone
#[wasm_bindgen]
pub fn solar_to_lunar_with_timezone(day: i32, month: i32, year: i32, time_zone: f64) -> JsValue {
    let lunar = convert_solar_to_lunar(day, month, year, time_zone);
    let result = WasmLunarDate {
        day: lunar.day,
        month: lunar.month,
        year: lunar.year,
        is_leap: lunar.is_leap,
    };
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Convert lunar date to solar date
///
/// Returns { day, month, year } or null if invalid
#[wasm_bindgen]
pub fn lunar_to_solar(day: i32, month: i32, year: i32, is_leap: bool) -> JsValue {
    let (d, m, y) = convert_lunar_to_solar(day, month, year, is_leap, VIETNAM_TIMEZONE);
    if d == 0 {
        return JsValue::NULL;
    }
    let result = WasmSolarDate { day: d, month: m, year: y };
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Convert lunar date to solar date with custom timezone
#[wasm_bindgen]
pub fn lunar_to_solar_with_timezone(
    day: i32,
    month: i32,
    year: i32,
    is_leap: bool,
    time_zone: f64,
) -> JsValue {
    let (d, m, y) = convert_lunar_to_solar(day, month, year, is_leap, time_zone);
    if d == 0 {
        return JsValue::NULL;
    }
    let result = WasmSolarDate { day: d, month: m, year: y };
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Get Vietnamese holidays for a given solar year
#[wasm_bindgen]
pub fn get_holidays(solar_year: i32) -> JsValue {
    let holidays = get_vietnamese_holidays(solar_year);
    let result: Vec<WasmHoliday> = holidays
        .into_iter()
        .map(|h| WasmHoliday {
            name: h.name,
            description: h.description,
            solar_day: h.solar_day,
            solar_month: h.solar_month,
            solar_year: h.solar_year,
            lunar_day: h.lunar_date.as_ref().map(|l| l.day),
            lunar_month: h.lunar_date.as_ref().map(|l| l.month),
            lunar_year: h.lunar_date.as_ref().map(|l| l.year),
            is_solar: h.is_solar,
        })
        .collect();
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Get major Vietnamese holidays for a given solar year (excludes monthly Mùng 1/Rằm)
#[wasm_bindgen]
pub fn get_major_holidays_js(solar_year: i32) -> JsValue {
    let holidays = get_major_holidays(solar_year);
    let result: Vec<WasmHoliday> = holidays
        .into_iter()
        .map(|h| WasmHoliday {
            name: h.name,
            description: h.description,
            solar_day: h.solar_day,
            solar_month: h.solar_month,
            solar_year: h.solar_year,
            lunar_day: h.lunar_date.as_ref().map(|l| l.day),
            lunar_month: h.lunar_date.as_ref().map(|l| l.month),
            lunar_year: h.lunar_date.as_ref().map(|l| l.year),
            is_solar: h.is_solar,
        })
        .collect();
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Get Vietnam timezone constant
#[wasm_bindgen]
pub fn get_vietnam_timezone() -> f64 {
    VIETNAM_TIMEZONE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_to_lunar_conversion() {
        // Tết 2024: February 10, 2024 = 1/1/2024 lunar
        let lunar = convert_solar_to_lunar(10, 2, 2024, VIETNAM_TIMEZONE);
        assert_eq!(lunar.day, 1);
        assert_eq!(lunar.month, 1);
        assert_eq!(lunar.year, 2024);
    }

    #[test]
    fn test_lunar_to_solar_conversion() {
        // 1/1/2024 lunar = February 10, 2024
        let (d, m, y) = convert_lunar_to_solar(1, 1, 2024, false, VIETNAM_TIMEZONE);
        assert_eq!((d, m, y), (10, 2, 2024));
    }
}
