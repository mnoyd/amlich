use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::dto::{
    CanChiInfoDto, DateQuery, DayFortuneDto, DayInsightDto, GioHoangDaoDto, KuaResultDto, LunarDto,
    NaAmResponseDto, SolarDto, ThapThanResultDto, TietKhiDto,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Include {
    Base,
    CanChi,
    TietKhi,
    Hours,
    Fortune,
    Insight,
    Evidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMetaDto {
    pub schema_version: String,
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayBundleDto {
    pub meta: ApiMetaDto,
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub jd: i32,
    pub canchi: Option<CanChiInfoDto>,
    pub tiet_khi: Option<TietKhiDto>,
    pub gio_hoang_dao: Option<GioHoangDaoDto>,
    pub day_fortune: Option<DayFortuneDto>,
    pub insight: Option<DayInsightDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayRangeDto {
    pub meta: ApiMetaDto,
    pub start: String,
    pub end: String,
    pub days: Vec<DayBundleDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TietKhiTransitionDto {
    pub date: String,
    pub term: TietKhiDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TietKhiYearDto {
    pub year: i32,
    pub transitions: Vec<TietKhiTransitionDto>,
}

fn include_set(includes: &[Include]) -> Vec<Include> {
    if includes.is_empty() {
        vec![
            Include::Base,
            Include::CanChi,
            Include::TietKhi,
            Include::Hours,
            Include::Fortune,
        ]
    } else {
        includes.to_vec()
    }
}

fn validate_includes(includes: &[Include]) -> Result<(), String> {
    if includes.contains(&Include::Evidence) && !includes.contains(&Include::Fortune) {
        return Err("include=evidence requires include=fortune".to_string());
    }
    Ok(())
}

pub fn get_day_bundle(query: &DateQuery, includes: &[Include]) -> Result<DayBundleDto, String> {
    let includes = include_set(includes);
    validate_includes(&includes)?;

    let info = crate::get_day_info(query)?;
    let insight = if includes.contains(&Include::Insight) {
        Some(crate::get_day_insight(query)?)
    } else {
        None
    };

    Ok(DayBundleDto {
        meta: ApiMetaDto {
            schema_version: "amlich.api/v2".to_string(),
            ruleset_id: info.ruleset_id.clone(),
            ruleset_version: info.ruleset_version.clone(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        },
        solar: info.solar,
        lunar: info.lunar,
        jd: info.jd,
        canchi: includes.contains(&Include::CanChi).then_some(info.canchi),
        tiet_khi: includes
            .contains(&Include::TietKhi)
            .then_some(info.tiet_khi),
        gio_hoang_dao: includes
            .contains(&Include::Hours)
            .then_some(info.gio_hoang_dao),
        day_fortune: includes.contains(&Include::Fortune).then_some(
            info.day_fortune
                .ok_or_else(|| "missing day_fortune in day info".to_string())?,
        ),
        insight,
    })
}

pub fn get_day_bundle_for_date(
    day: i32,
    month: i32,
    year: i32,
    includes: &[Include],
    timezone: Option<f64>,
) -> Result<DayBundleDto, String> {
    let query = DateQuery {
        day,
        month,
        year,
        timezone,
    };
    get_day_bundle(&query, includes)
}

fn get_path<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for segment in path {
        current = current.get(*segment)?;
    }
    Some(current)
}

fn set_path(root: &mut Map<String, Value>, path: &[&str], leaf: Value) {
    if path.is_empty() {
        return;
    }
    if path.len() == 1 {
        root.insert(path[0].to_string(), leaf);
        return;
    }

    let entry = root
        .entry(path[0].to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if let Value::Object(obj) = entry {
        set_path(obj, &path[1..], leaf);
    }
}

pub fn project_fields(value: &Value, fields: &[String]) -> Result<Value, String> {
    if fields.is_empty() {
        return Ok(value.clone());
    }

    let mut projected = Map::new();
    for field in fields {
        let segments: Vec<&str> = field.split('.').collect();
        let leaf = get_path(value, &segments)
            .ok_or_else(|| format!("unknown field path: {field}"))?
            .clone();
        set_path(&mut projected, &segments, leaf);
    }

    Ok(Value::Object(projected))
}

pub fn get_day_bundle_projected(
    query: &DateQuery,
    includes: &[Include],
    fields: &[String],
) -> Result<Value, String> {
    let bundle = get_day_bundle(query, includes)?;
    let value = serde_json::to_value(bundle).map_err(|e| format!("serialize failed: {e}"))?;
    project_fields(&value, fields)
}

pub fn get_day_range(
    start: DateQuery,
    end: DateQuery,
    includes: &[Include],
) -> Result<DayRangeDto, String> {
    let start_date = NaiveDate::from_ymd_opt(start.year, start.month as u32, start.day as u32)
        .ok_or_else(|| "invalid start date".to_string())?;
    let end_date = NaiveDate::from_ymd_opt(end.year, end.month as u32, end.day as u32)
        .ok_or_else(|| "invalid end date".to_string())?;

    if end_date < start_date {
        return Err("end date must be greater than or equal to start date".to_string());
    }

    if (end_date - start_date).num_days() > 366 {
        return Err("date range is too large (max 366 days)".to_string());
    }

    let mut days = Vec::new();
    let mut cursor = start_date;
    while cursor <= end_date {
        days.push(get_day_bundle_for_date(
            cursor.day() as i32,
            cursor.month() as i32,
            cursor.year(),
            includes,
            start.timezone,
        )?);
        cursor = cursor
            .succ_opt()
            .ok_or_else(|| "failed to iterate date range".to_string())?;
    }

    let first = days
        .first()
        .ok_or_else(|| "empty range after processing".to_string())?;
    Ok(DayRangeDto {
        meta: first.meta.clone(),
        start: format!("{}-{:02}-{:02}", start.year, start.month, start.day),
        end: format!("{}-{:02}-{:02}", end.year, end.month, end.day),
        days,
    })
}

pub fn get_almanac(query: &DateQuery) -> Result<DayFortuneDto, String> {
    let info = crate::get_day_info(query)?;
    info.day_fortune
        .ok_or_else(|| "missing day_fortune for date".to_string())
}

pub fn get_insight(query: &DateQuery) -> Result<DayInsightDto, String> {
    crate::get_day_insight(query)
}

pub fn get_tiet_khi_for_year(year: i32, timezone: Option<f64>) -> Result<TietKhiYearDto, String> {
    let tz = timezone.or(Some(amlich_core::VIETNAM_TIMEZONE));
    let mut by_date: BTreeMap<String, TietKhiDto> = BTreeMap::new();

    let mut cursor =
        NaiveDate::from_ymd_opt(year, 1, 1).ok_or_else(|| "invalid year".to_string())?;
    let end = NaiveDate::from_ymd_opt(year, 12, 31).ok_or_else(|| "invalid year".to_string())?;

    while cursor <= end {
        let query = DateQuery {
            day: cursor.day() as i32,
            month: cursor.month() as i32,
            year: cursor.year(),
            timezone: tz,
        };
        let day = crate::get_day_info(&query)?;
        by_date.insert(cursor.to_string(), day.tiet_khi);
        cursor = cursor
            .succ_opt()
            .ok_or_else(|| "failed to iterate year".to_string())?;
    }

    let mut transitions = Vec::new();
    let mut last_name: Option<String> = None;
    for (date, term) in by_date {
        let changed = last_name
            .as_ref()
            .map(|name| name != &term.name)
            .unwrap_or(true);
        if changed {
            last_name = Some(term.name.clone());
            transitions.push(TietKhiTransitionDto { date, term });
        }
    }

    Ok(TietKhiYearDto { year, transitions })
}

pub fn convert_solar_to_lunar(query: &DateQuery) -> Result<LunarDto, String> {
    let info = crate::get_day_info(query)?;
    Ok(info.lunar)
}

pub fn convert_lunar_to_solar(
    day: i32,
    month: i32,
    year: i32,
    leap: bool,
    timezone: Option<f64>,
) -> Result<SolarDto, String> {
    let tz = timezone.unwrap_or(amlich_core::VIETNAM_TIMEZONE);
    let (solar_day, solar_month, solar_year) =
        amlich_core::lunar::convert_lunar_to_solar(day, month, year, leap, tz);
    if (solar_day, solar_month, solar_year) == (0, 0, 0) {
        return Err("invalid lunar date conversion".to_string());
    }
    Ok(crate::get_day_info_for_date(solar_day, solar_month, solar_year)?.solar)
}

pub fn lookup_na_am_by_index(index: u8) -> NaAmResponseDto {
    crate::get_na_am_by_index(index)
}

pub fn lookup_na_am_by_pair(can: &str, chi: &str) -> NaAmResponseDto {
    crate::get_na_am_by_pair(can, chi)
}

pub fn lookup_ten_gods(day_can: &str, target_can: &str) -> Result<ThapThanResultDto, String> {
    let day = amlich_core::HeavenlyStem::try_from(day_can)?;
    let target = amlich_core::HeavenlyStem::try_from(target_can)?;
    let result = amlich_core::get_thap_than(day, target);
    Ok(ThapThanResultDto::from(&result))
}

pub fn lookup_kua(birth_year: i32, gender: amlich_core::Gender) -> KuaResultDto {
    let result = amlich_core::compute_kua(birth_year, gender);
    KuaResultDto::from(&result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection_rejects_unknown_fields() {
        let value = serde_json::json!({"a": {"b": 1}});
        let err = project_fields(&value, &["a.c".to_string()]).expect_err("should fail");
        assert!(err.contains("unknown field path"));
    }

    #[test]
    fn projection_keeps_nested_shape() {
        let value = serde_json::json!({"a": {"b": 1, "c": 2}, "d": 3});
        let projected =
            project_fields(&value, &["a.b".to_string(), "d".to_string()]).expect("projected");
        assert_eq!(projected["a"]["b"], 1);
        assert_eq!(projected["d"], 3);
        assert!(projected["a"].get("c").is_none());
    }

    #[test]
    fn day_bundle_hides_fortune_when_not_included() {
        let query = DateQuery {
            day: 10,
            month: 2,
            year: 2024,
            timezone: None,
        };
        let bundle = get_day_bundle(&query, &[Include::Base, Include::CanChi]).expect("bundle");
        assert!(bundle.day_fortune.is_none());
        assert!(bundle.canchi.is_some());
    }
}
