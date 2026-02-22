// amlich-wasm - WASM bindings for web usage

use amlich_api::DayInfoDto;
use amlich_api::{get_day_info, get_holidays, DateQuery};
use amlich_core::{
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

fn format_day_info(info: &DayInfoDto) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "📅 Ngày {} ({})",
        info.solar.date_string, info.solar.day_of_week_name
    ));
    lines.push(format!("🌙 Âm lịch: {}", info.lunar.date_string));
    lines.push("📜 Can Chi:".to_string());
    lines.push(format!(
        "   • Ngày: {} ({})",
        info.canchi.day.full, info.canchi.day.con_giap
    ));
    lines.push(format!("   • Tháng: {}", info.canchi.month.full));
    lines.push(format!(
        "   • Năm: {} ({})",
        info.canchi.year.full, info.canchi.year.con_giap
    ));
    lines.push("🌟 Ngũ hành:".to_string());
    lines.push(format!(
        "   • Ngày: {} (Can) - {} (Chi)",
        info.canchi.day.ngu_hanh.can, info.canchi.day.ngu_hanh.chi
    ));
    lines.push(format!(
        "🌤️  Tiết khí: {} - {}",
        info.tiet_khi.name, info.tiet_khi.season
    ));
    lines.push(format!("   • {}", info.tiet_khi.description));
    lines.push(format!(
        "   • Kinh độ mặt trời: {}°",
        info.tiet_khi.current_longitude
    ));
    lines.push(format!(
        "⏰ Giờ Hoàng Đạo ({} giờ tốt):",
        info.gio_hoang_dao.good_hour_count
    ));
    for h in &info.gio_hoang_dao.good_hours {
        lines.push(format!(
            "   • {} ({}) - {}",
            h.hour_chi, h.time_range, h.star
        ));
    }

    lines.join("\n")
}

/// Get comprehensive day information for a solar date.
#[wasm_bindgen]
pub fn get_day_info_js(day: i32, month: i32, year: i32) -> JsValue {
    let query = DateQuery {
        day,
        month,
        year,
        timezone: Some(VIETNAM_TIMEZONE),
    };
    match get_day_info(&query) {
        Ok(info) => serde_wasm_bindgen::to_value(&info).unwrap_or(JsValue::NULL),
        Err(_) => JsValue::NULL,
    }
}

/// Get comprehensive day information with custom timezone.
#[wasm_bindgen]
pub fn get_day_info_with_timezone_js(day: i32, month: i32, year: i32, time_zone: f64) -> JsValue {
    let query = DateQuery {
        day,
        month,
        year,
        timezone: Some(time_zone),
    };
    match get_day_info(&query) {
        Ok(info) => serde_wasm_bindgen::to_value(&info).unwrap_or(JsValue::NULL),
        Err(_) => JsValue::NULL,
    }
}

/// Get formatted day info as a string.
#[wasm_bindgen]
pub fn format_day_info_js(day: i32, month: i32, year: i32) -> String {
    let query = DateQuery {
        day,
        month,
        year,
        timezone: Some(VIETNAM_TIMEZONE),
    };

    match get_day_info(&query) {
        Ok(info) => format_day_info(&info),
        Err(_) => String::new(),
    }
}

/// Convert solar date to lunar date.
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

/// Convert solar date to lunar date with custom timezone.
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

/// Convert lunar date to solar date.
#[wasm_bindgen]
pub fn lunar_to_solar(day: i32, month: i32, year: i32, is_leap: bool) -> JsValue {
    let (d, m, y) = convert_lunar_to_solar(day, month, year, is_leap, VIETNAM_TIMEZONE);
    if d == 0 {
        return JsValue::NULL;
    }
    let result = WasmSolarDate {
        day: d,
        month: m,
        year: y,
    };
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Convert lunar date to solar date with custom timezone.
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
    let result = WasmSolarDate {
        day: d,
        month: m,
        year: y,
    };
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Get all holidays for a given year.
#[wasm_bindgen]
pub fn get_holidays_js(year: i32) -> JsValue {
    let result = get_holidays(year, false);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Get major holidays for a given year.
#[wasm_bindgen]
pub fn get_major_holidays_js(year: i32) -> JsValue {
    let result = get_holidays(year, true);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

/// Health check function.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solar_to_lunar_conversion() {
        let lunar = convert_solar_to_lunar(10, 2, 2024, VIETNAM_TIMEZONE);
        assert_eq!(lunar.day, 1);
        assert_eq!(lunar.month, 1);
        assert_eq!(lunar.year, 2024);
    }

    #[test]
    fn test_lunar_to_solar_conversion() {
        let (d, m, y) = convert_lunar_to_solar(1, 1, 2024, false, VIETNAM_TIMEZONE);
        assert_eq!((d, m, y), (10, 2, 2024));
    }
}
