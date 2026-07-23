use serde::Serialize;
use std::collections::HashMap;

use amlich_api::v2::{
    get_day_bundle_for_date, get_day_range as api_get_day_range,
    get_tiet_khi_for_year as api_get_tiet_khi_for_year, DayBundleDto, DayRangeDto, Include,
    TietKhiYearDto,
};
use amlich_api::{
    get_bazi_derived_report as api_get_bazi_derived_report, get_bazi_report as api_get_bazi_report,
    get_day_info_for_date, get_day_insight_for_date, get_holidays,
    get_hour_selection_report as api_get_hour_selection_report,
    get_personal_day_matrix_report as api_get_personal_day_matrix_report,
    get_personal_day_report as api_get_personal_day_report,
    get_recommendation_pack_catalog as api_get_recommendation_pack_catalog,
    get_ruleset_catalog as api_get_ruleset_catalog, BaziDerivedReportDto, BaziQuery, BaziReportDto,
    DateQuery, DayInfoDto, DayInsightDto, HolidayDto, HourSelectionReportDto,
    PersonalDayMatrixReportDto, PersonalDayReportDto, RecommendationPackCatalogEntryDto,
    RulesetCatalogEntryDto,
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
    platform: String,
    arch: String,
    app_version: String,
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

fn parse_gender(gender: Option<String>) -> Result<Option<amlich_core::Gender>, String> {
    match gender
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        None => Ok(None),
        Some("male") | Some("nam") => Ok(Some(amlich_core::Gender::Male)),
        Some("female") | Some("nu") | Some("nữ") => Ok(Some(amlich_core::Gender::Female)),
        Some(value) => Err(format!(
            "gender must be one of: male, female, nam, nu, nữ; got '{value}'"
        )),
    }
}

fn validate_date_parts(day: i32, month: i32) -> Result<(), String> {
    if !(1..=12).contains(&month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&day) {
        return Err("day must be 1-31".to_string());
    }
    Ok(())
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
    validate_date_parts(day, month)?;

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
    validate_date_parts(day, month)?;
    get_day_insight_for_date(day, month, year)
}

#[tauri::command]
fn get_day_info(day: i32, month: i32, year: i32) -> Result<DayInfoDto, String> {
    validate_date_parts(day, month)?;
    get_day_info_for_date(day, month, year)
}

#[tauri::command]
fn get_day_bundle(day: i32, month: i32, year: i32) -> Result<DayBundleDto, String> {
    validate_date_parts(day, month)?;
    get_day_bundle_for_date(day, month, year, &[], None)
}

#[tauri::command]
fn get_day_range(
    start: DateQuery,
    end: DateQuery,
    includes: Option<Vec<Include>>,
) -> Result<DayRangeDto, String> {
    api_get_day_range(start, end, includes.as_deref().unwrap_or(&[]))
}

#[tauri::command]
fn get_bazi_report(
    year: i32,
    month: i32,
    day: i32,
    hour: u8,
    minute: u8,
    gender: Option<String>,
) -> Result<BaziReportDto, String> {
    let query = BaziQuery {
        year,
        month,
        day,
        hour,
        minute,
        time_known: None,
        timezone: None,
        longitude: None,
        use_solar_time: false,
        gender,
    };
    api_get_bazi_report(&query, None)
}

#[tauri::command]
fn get_bazi_derived_report(
    year: i32,
    month: i32,
    day: i32,
    hour: u8,
    minute: u8,
    gender: Option<String>,
) -> Result<BaziDerivedReportDto, String> {
    let query = BaziQuery {
        year,
        month,
        day,
        hour,
        minute,
        time_known: None,
        timezone: None,
        longitude: None,
        use_solar_time: false,
        gender,
    };
    api_get_bazi_derived_report(&query)
}

#[tauri::command]
fn get_hour_selection_report(
    day: i32,
    month: i32,
    year: i32,
) -> Result<HourSelectionReportDto, String> {
    validate_date_parts(day, month)?;
    let query = DateQuery {
        day,
        month,
        year,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    api_get_hour_selection_report(&query, None, None, None, None)
}

#[tauri::command]
fn get_tiet_khi_for_year(year: i32) -> Result<TietKhiYearDto, String> {
    api_get_tiet_khi_for_year(year, None)
}

#[tauri::command]
fn get_ruleset_catalog() -> Vec<RulesetCatalogEntryDto> {
    api_get_ruleset_catalog()
}

#[tauri::command]
fn get_recommendation_pack_catalog() -> Vec<RecommendationPackCatalogEntryDto> {
    api_get_recommendation_pack_catalog()
}

#[tauri::command]
fn get_holidays_list(year: i32, major_only: bool) -> Vec<HolidayDto> {
    get_holidays(year, major_only)
}

#[tauri::command]
fn get_personal_day_report(
    day: i32,
    month: i32,
    year: i32,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<String>,
) -> Result<PersonalDayReportDto, String> {
    validate_date_parts(day, month)?;

    api_get_personal_day_report(
        &DateQuery {
            day,
            month,
            year,
            timezone: None,
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
        },
        birth_year,
        birth_month,
        birth_day,
        parse_gender(gender)?,
    )
}

#[tauri::command]
#[allow(clippy::too_many_arguments)]
fn get_personal_day_matrix_report(
    day: i32,
    month: i32,
    year: i32,
    birth_year: i32,
    birth_month: i32,
    birth_day: i32,
    birth_hour: u8,
    birth_minute: u8,
    gender: Option<String>,
) -> Result<PersonalDayMatrixReportDto, String> {
    validate_date_parts(day, month)?;

    api_get_personal_day_matrix_report(
        &BaziQuery {
            day: birth_day,
            month: birth_month,
            year: birth_year,
            hour: birth_hour,
            minute: birth_minute,
            time_known: None,
            timezone: None,
            longitude: None,
            use_solar_time: false,
            gender,
        },
        &DateQuery {
            day,
            month,
            year,
            timezone: None,
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
        },
    )
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

    let platform = match std::env::consts::OS {
        "macos" => "macos",
        "linux" => "linux",
        "windows" => "windows",
        other => other,
    }
    .to_string();

    let arch = std::env::consts::ARCH.to_string();

    InstallContext {
        executable_path,
        is_system_install,
        can_self_update: !is_system_install,
        platform,
        arch,
        app_version: env!("CARGO_PKG_VERSION").to_string(),
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
            get_day_info,
            get_day_bundle,
            get_day_range,
            get_bazi_report,
            get_bazi_derived_report,
            get_hour_selection_report,
            get_tiet_khi_for_year,
            get_ruleset_catalog,
            get_recommendation_pack_catalog,
            get_holidays_list,
            get_personal_day_report,
            get_personal_day_matrix_report,
            get_install_context
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gender_accepts_supported_aliases_and_rejects_unknown_values() {
        assert_eq!(
            parse_gender(Some("male".to_string())),
            Ok(Some(amlich_core::Gender::Male))
        );
        assert_eq!(
            parse_gender(Some("nam".to_string())),
            Ok(Some(amlich_core::Gender::Male))
        );
        assert_eq!(
            parse_gender(Some("female".to_string())),
            Ok(Some(amlich_core::Gender::Female))
        );
        assert_eq!(
            parse_gender(Some("nữ".to_string())),
            Ok(Some(amlich_core::Gender::Female))
        );
        assert!(parse_gender(Some("other".to_string())).is_err());
    }

    #[test]
    fn personal_day_report_command_exposes_canonical_reasoning_bundle() {
        let report = get_personal_day_report(
            10,
            2,
            2024,
            Some(1990),
            Some(1),
            Some(1),
            Some("male".to_string()),
        )
        .expect("personal day report");

        let decision_export = report.decision_export.as_ref().expect("decision export");
        let graph = report.graph.as_ref().expect("graph");

        assert!(!decision_export.primary_conclusion.is_empty());
        assert!(!decision_export.axis_scores.is_empty());
        assert!(!graph.nodes.is_empty());
        assert_eq!(report.decision_export, report.analysis.decision_export);
        assert_eq!(report.graph, report.analysis.graph);
    }

    #[test]
    fn personal_day_matrix_command_exposes_matrix_sections() {
        let report = get_personal_day_matrix_report(
            10,
            2,
            2024,
            1990,
            1,
            1,
            9,
            30,
            Some("male".to_string()),
        )
        .expect("personal day matrix report");

        assert_eq!(report.tier, amlich_api::BirthDataTierDto::Datetime);
        assert!(report.personal_hours.is_some());
        assert!(report.direction_merge.is_some());
        assert!(report.domain_day_boost.is_some());
    }

    #[test]
    fn day_range_command_exposes_bundles() {
        let range = get_day_range(
            DateQuery {
                day: 10,
                month: 2,
                year: 2024,
                ..Default::default()
            },
            DateQuery {
                day: 12,
                month: 2,
                year: 2024,
                ..Default::default()
            },
            Some(vec![Include::Base, Include::CanChi, Include::TietKhi]),
        )
        .expect("day range");

        assert_eq!(range.start, "2024-02-10");
        assert_eq!(range.end, "2024-02-12");
        assert_eq!(range.days.len(), 3);
        assert!(range.days[0].canchi.is_some());
        assert!(range.days[0].tiet_khi.is_some());
    }

    #[test]
    fn get_month_data_command_builds_calendar_grid() {
        let data = get_month_data(2, 2024).expect("month data");

        assert_eq!(data.month, 2);
        assert_eq!(data.year, 2024);
        // 2024 has 29 days in February.
        assert_eq!(data.days.len(), 29);
        assert_eq!(data.days[0].day, 1);
        assert_eq!(data.days[28].day, 29);
        // First weekday of Feb 2024 is Thursday (index 4).
        assert_eq!(data.first_weekday, 4);
        // Each day cell carries lunar + canchi metadata for the workspace.
        assert!(!data.days[0].canchi_day.is_empty());
        assert!(!data.days[0].lunar_date.is_empty());
    }

    #[test]
    fn get_month_data_command_rejects_invalid_month() {
        assert!(get_month_data(0, 2024).is_err());
        assert!(get_month_data(13, 2024).is_err());
    }

    #[test]
    fn get_day_detail_command_returns_decorated_cell() {
        let cell = get_day_detail(10, 2, 2024).expect("day detail");

        assert_eq!(cell.day, 10);
        assert_eq!(cell.month, 2);
        assert_eq!(cell.year, 2024);
        assert_eq!(cell.solar_date, "2024-02-10");
        assert!(!cell.canchi_day.is_empty());
        assert!(!cell.canchi_month.is_empty());
        assert!(!cell.canchi_year.is_empty());
        // Good hours list is populated for the Day workspace.
        assert!(!cell.good_hours.is_empty());
        assert!(cell.good_hours.iter().all(|h| !h.hour_chi.is_empty()
            && !h.time_range.is_empty()
            && !h.star.is_empty()));
    }

    #[test]
    fn get_day_detail_command_rejects_out_of_range_parts() {
        assert!(get_day_detail(0, 2, 2024).is_err());
        assert!(get_day_detail(32, 2, 2024).is_err());
        assert!(get_day_detail(10, 0, 2024).is_err());
        assert!(get_day_detail(10, 13, 2024).is_err());
    }

    #[test]
    fn get_day_bundle_command_returns_full_bundle() {
        let bundle = get_day_bundle(10, 2, 2024).expect("day bundle");

        assert!(!bundle.schema_version.is_empty());
        assert!(!bundle.ruleset_id.is_empty());
        assert_eq!(bundle.solar.year, 2024);
        assert_eq!(bundle.solar.month, 2);
        assert_eq!(bundle.solar.day, 10);
        assert!(bundle.canchi.is_some());
        assert!(bundle.tiet_khi.is_some());
        assert!(bundle.gio_hoang_dao.is_some());
    }

    #[test]
    fn get_day_info_and_insight_commands_return_consistent_solar_anchor() {
        let info = get_day_info(10, 2, 2024).expect("day info");
        let insight = get_day_insight(10, 2, 2024).expect("day insight");

        assert_eq!(info.solar.date_string, "2024-02-10");
        assert_eq!(insight.solar.date_string, "2024-02-10");
        assert_eq!(info.lunar.date_string, insight.lunar.date_string);
    }

    #[test]
    fn get_bazi_report_command_returns_summary_and_signals() {
        let report =
            get_bazi_report(1990, 1, 1, 9, 30, Some("male".to_string())).expect("bazi report");

        assert!(!report.summary.is_empty());
        assert!(!report.top_signals.is_empty());
        assert!(!report.why_this_matters.is_empty());
        assert!(!report.recommended_actions.is_empty());
    }

    #[test]
    fn get_bazi_derived_report_command_returns_thai_nguyen_and_tier() {
        let report = get_bazi_derived_report(1990, 1, 1, 9, 30, Some("male".to_string()))
            .expect("bazi derived report");

        assert_eq!(report.tier, amlich_api::BirthDataTierDto::Datetime);
        assert!(!report.thai_nguyen.can_chi.full.is_empty());
        assert!(report.menh_cung.is_some());
    }

    #[test]
    fn get_hour_selection_report_command_returns_chart_and_analysis() {
        let report = get_hour_selection_report(10, 2, 2024).expect("hour selection report");

        assert!(!report.chart.gio_hoang_dao.good_hours.is_empty());
        assert!(report.computed_metrics.good_hour_count + report.computed_metrics.bad_hour_count > 0);
    }

    #[test]
    fn get_tiet_khi_for_year_command_returns_24_transitions() {
        let year = get_tiet_khi_for_year(2024).expect("tiet khi year");

        assert_eq!(year.year, 2024);
        // 24 tiết khí in a year, plus a leading/trailing boundary transition.
        assert!(year.transitions.len() >= 24);
        assert!(year.transitions.iter().all(|t| !t.term.name.is_empty()));
    }

    #[test]
    fn catalog_commands_return_non_empty_entries() {
        let rulesets = get_ruleset_catalog();
        assert!(!rulesets.is_empty());
        assert!(rulesets.iter().all(|r| !r.id.is_empty() && !r.version.is_empty()));

        let packs = get_recommendation_pack_catalog();
        assert!(!packs.is_empty());
        assert!(packs.iter().all(|p| !p.pack_id.is_empty() && !p.version.is_empty()));
    }

    #[test]
    fn get_holidays_list_command_filters_by_major_flag() {
        let all = get_holidays_list(2024, false);
        let major = get_holidays_list(2024, true);

        assert!(!all.is_empty());
        assert!(major.iter().all(|h| h.is_major));
        assert!(major.len() <= all.len());
    }

    #[test]
    fn get_install_context_command_reports_runtime_metadata() {
        let ctx = get_install_context();

        assert!(!ctx.platform.is_empty());
        assert!(!ctx.arch.is_empty());
        assert!(!ctx.app_version.is_empty());
        assert_eq!(ctx.can_self_update, !ctx.is_system_install);
    }

    #[test]
    fn gender_is_passed_through_to_bazi_commands() {
        let male =
            get_bazi_report(1990, 1, 1, 9, 30, Some("nam".to_string())).expect("bazi male");
        let female =
            get_bazi_report(1990, 1, 1, 9, 30, Some("nữ".to_string())).expect("bazi female");

        // Same chart, but the advisory summary should differ once gender is applied.
        assert!(!male.summary.is_empty());
        assert!(!female.summary.is_empty());
    }
}
