use std::collections::BTreeMap;

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::dto::{
    CanChiInfoDto, DailyRecommendationsDto, DateQuery, DayFortuneDto, DayInsightDto,
    GioHoangDaoDto, KuaResultDto, LunarDto, NaAmResponseDto, SolarDto, ThapThanResultDto,
    TietKhiDto, UpcomingEventDto,
};

const SCHEMA_VERSION: &str = "amlich.engine/v1";

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
    pub profile: String,
    pub generated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayBundleDto {
    pub schema_version: String,
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub generated_at: String,
    pub solar: SolarDto,
    pub lunar: LunarDto,
    pub jd: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canchi: Option<CanChiInfoDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiet_khi: Option<TietKhiDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gio_hoang_dao: Option<GioHoangDaoDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_fortune: Option<DayFortuneDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_recommendations: Option<DailyRecommendationsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextual_recommendations: Option<DailyRecommendationsDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight: Option<DayInsightDto>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upcoming_events: Vec<UpcomingEventDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayRangeDto {
    pub schema_version: String,
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub generated_at: String,
    pub start: String,
    pub end: String,
    pub days: Vec<DayBundleDto>,
}

impl From<&crate::dto::DayInfoDto> for ApiMetaDto {
    fn from(info: &crate::dto::DayInfoDto) -> Self {
        Self {
            schema_version: SCHEMA_VERSION.to_string(),
            ruleset_id: info.ruleset_id.clone(),
            ruleset_version: info.ruleset_version.clone(),
            profile: info.profile.clone(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl DayBundleDto {
    fn from_parts(
        info: crate::dto::DayInfoDto,
        insight: Option<DayInsightDto>,
        includes: &[Include],
    ) -> Result<Self, String> {
        let meta = ApiMetaDto::from(&info);

        let upcoming_events =
            amlich_core::holidays::get_upcoming_events(info.jd, info.solar.year, 14)
                .into_iter()
                .map(|e| UpcomingEventDto {
                    name: e.name,
                    days_left: e.days_left,
                    is_lunar: e.is_lunar,
                })
                .collect();

        Ok(Self {
            schema_version: meta.schema_version,
            ruleset_id: meta.ruleset_id,
            ruleset_version: meta.ruleset_version,
            profile: meta.profile,
            generated_at: meta.generated_at,
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
            daily_recommendations: includes
                .contains(&Include::Fortune)
                .then_some(info.daily_recommendations),
            contextual_recommendations: includes
                .contains(&Include::Fortune)
                .then_some(info.contextual_recommendations)
                .flatten(),
            insight,
            upcoming_events,
        })
    }

    pub fn meta(&self) -> ApiMetaDto {
        ApiMetaDto {
            schema_version: self.schema_version.clone(),
            ruleset_id: self.ruleset_id.clone(),
            ruleset_version: self.ruleset_version.clone(),
            profile: self.profile.clone(),
            generated_at: self.generated_at.clone(),
        }
    }
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

    DayBundleDto::from_parts(info, insight, &includes)
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
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
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
        let query = DateQuery {
            day: cursor.day() as i32,
            month: cursor.month() as i32,
            year: cursor.year(),
            timezone: start.timezone,
            ruleset_id: start.ruleset_id.clone(),
            event_kind: start.event_kind.clone(),
            enabled_pack_ids: start.enabled_pack_ids.clone(),
        };
        days.push(get_day_bundle(&query, includes)?);
        cursor = cursor
            .succ_opt()
            .ok_or_else(|| "failed to iterate date range".to_string())?;
    }

    let first = days
        .first()
        .ok_or_else(|| "empty range after processing".to_string())?;
    Ok(DayRangeDto {
        schema_version: first.schema_version.clone(),
        ruleset_id: first.ruleset_id.clone(),
        ruleset_version: first.ruleset_version.clone(),
        profile: first.profile.clone(),
        generated_at: first.generated_at.clone(),
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

pub fn get_insight_with_profile(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Result<DayInsightDto, String> {
    crate::get_day_insight_with_profile(query, birth_year, birth_month, birth_day, gender)
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
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
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
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
        };
        let bundle = get_day_bundle(&query, &[Include::Base, Include::CanChi]).expect("bundle");
        assert!(bundle.day_fortune.is_none());
        assert!(bundle.daily_recommendations.is_none());
        assert!(bundle.canchi.is_some());
    }

    #[test]
    fn day_bundle_exposes_top_level_metadata() {
        let query = DateQuery {
            day: 10,
            month: 2,
            year: 2024,
            timezone: Some(7.0),
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
        };

        let bundle = get_day_bundle(&query, &[]).expect("bundle");
        assert_eq!(bundle.schema_version, SCHEMA_VERSION);
        assert_eq!(bundle.ruleset_id, "vn_baseline_v1");
        assert_eq!(bundle.ruleset_version, "v1");
        assert_eq!(bundle.profile, "baseline");
        assert!(!bundle.generated_at.is_empty());
        assert_eq!(bundle.meta().ruleset_id, bundle.ruleset_id);
    }
}
