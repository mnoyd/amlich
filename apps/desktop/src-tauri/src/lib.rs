use serde::Serialize;
use std::collections::HashMap;

use amlich_api::{
    get_day_info_for_date, get_day_insight_for_date, get_holidays, DayInfoDto, DayInsightDto,
    HolidayDto,
};

#[derive(Debug, Serialize, Clone)]
struct GoodHour {
    hour_chi: String,
    time_range: String,
    star: String,
}

#[derive(Debug, Serialize, Clone)]
struct HolidayInfo {
    name: String,
    description: String,
    is_solar: bool,
    lunar_day: Option<i32>,
    lunar_month: Option<i32>,
    category: String,
    is_major: bool,
}

#[derive(Debug, Serialize, Clone)]
struct DayCell {
    day: i32,
    month: i32,
    year: i32,
    day_of_week_index: usize,
    day_of_week: String,
    solar_date: String,
    lunar_day: i32,
    lunar_month: i32,
    lunar_year: i32,
    lunar_leap: bool,
    lunar_date: String,
    canchi_day: String,
    canchi_month: String,
    canchi_year: String,
    tiet_khi: String,
    tiet_khi_description: String,
    tiet_khi_season: String,
    good_hours: Vec<GoodHour>,
    holidays: Vec<HolidayInfo>,
}

#[derive(Debug, Serialize, Clone)]
struct MonthData {
    month: u32,
    year: i32,
    first_weekday: usize,
    days: Vec<DayCell>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct InstallContext {
    executable_path: Option<String>,
    is_system_install: bool,
    can_self_update: bool,
}

fn to_day_cell(day_info: DayInfoDto, holidays: Vec<HolidayInfo>) -> DayCell {
    let good_hours = day_info
        .gio_hoang_dao
        .good_hours
        .iter()
        .map(|h| GoodHour {
            hour_chi: h.hour_chi.clone(),
            time_range: h.time_range.clone(),
            star: h.star.clone(),
        })
        .collect::<Vec<_>>();

    DayCell {
        day: day_info.solar.day,
        month: day_info.solar.month,
        year: day_info.solar.year,
        day_of_week_index: day_info.solar.day_of_week,
        day_of_week: day_info.solar.day_of_week_name,
        solar_date: day_info.solar.date_string,
        lunar_day: day_info.lunar.day,
        lunar_month: day_info.lunar.month,
        lunar_year: day_info.lunar.year,
        lunar_leap: day_info.lunar.is_leap_month,
        lunar_date: day_info.lunar.date_string,
        canchi_day: day_info.canchi.day.full,
        canchi_month: day_info.canchi.month.full,
        canchi_year: day_info.canchi.year.full,
        tiet_khi: day_info.tiet_khi.name,
        tiet_khi_description: day_info.tiet_khi.description,
        tiet_khi_season: day_info.tiet_khi.season,
        good_hours,
        holidays,
    }
}

fn holiday_to_info(holiday: &HolidayDto) -> HolidayInfo {
    HolidayInfo {
        name: holiday.name.clone(),
        description: holiday.description.clone(),
        is_solar: holiday.is_solar,
        lunar_day: holiday.lunar_day,
        lunar_month: holiday.lunar_month,
        category: holiday.category.clone(),
        is_major: holiday.is_major,
    }
}

#[tauri::command]
fn get_month_data(month: u32, year: i32) -> Result<MonthData, String> {
    if !(1..=12).contains(&month) {
        return Err("month must be 1-12".to_string());
    }

    let mut days = Vec::new();
    let mut first_weekday = 0;
    let mut holidays_by_day: HashMap<i32, Vec<HolidayInfo>> = HashMap::new();
    let holidays = get_holidays(year, false);
    for holiday in holidays {
        if holiday.solar_year == year && holiday.solar_month == month as i32 {
            holidays_by_day
                .entry(holiday.solar_day)
                .or_default()
                .push(holiday_to_info(&holiday));
        }
    }

    for day in 1..=31 {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day as u32);
        if date.is_none() {
            break;
        }

        let holiday_list = holidays_by_day.remove(&day).unwrap_or_default();
        let info = get_day_info_for_date(day, month as i32, year)?;
        if day == 1 {
            first_weekday = info.solar.day_of_week;
        }
        days.push(to_day_cell(info, holiday_list));
    }

    Ok(MonthData {
        month,
        year,
        first_weekday,
        days,
    })
}

#[tauri::command]
fn get_day_detail(day: i32, month: i32, year: i32) -> Result<DayCell, String> {
    if !(1..=12).contains(&month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&day) {
        return Err("day must be 1-31".to_string());
    }

    let holidays = get_holidays(year, false)
        .into_iter()
        .filter(|h| h.solar_year == year && h.solar_month == month && h.solar_day == day)
        .map(|h| holiday_to_info(&h))
        .collect::<Vec<_>>();

    Ok(to_day_cell(
        get_day_info_for_date(day, month, year)?,
        holidays,
    ))
}

#[tauri::command]
fn get_day_insight(day: i32, month: i32, year: i32) -> Result<DayInsightDto, String> {
    if !(1..=12).contains(&month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&day) {
        return Err("day must be 1-31".to_string());
    }
    get_day_insight_for_date(day, month, year)
}

#[tauri::command]
fn get_install_context() -> InstallContext {
    let executable_path = std::env::current_exe()
        .ok()
        .map(|path| path.display().to_string());

    #[cfg(target_os = "linux")]
    let is_system_install = executable_path.as_ref().is_some_and(|path| {
        path.starts_with("/usr/") || path.starts_with("/opt/") || path.starts_with("/nix/store/")
    });

    #[cfg(not(target_os = "linux"))]
    let is_system_install = false;

    InstallContext {
        executable_path,
        is_system_install,
        can_self_update: !is_system_install,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            get_month_data,
            get_day_detail,
            get_day_insight,
            get_install_context
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
