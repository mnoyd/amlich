mod convert;
mod dto;
pub mod v2;

use std::collections::HashMap;

use amlich_core::almanac::data::{get_ruleset, ruleset_registry};
use amlich_core::almanac::recommendation::pack::recommendation_pack_descriptors;
use amlich_core::holiday_data::{lunar_festivals, solar_holidays};
use amlich_core::holidays::get_vietnamese_holidays;
use amlich_core::insight_data::{
    all_elements, find_can, find_chi, find_deity_classification_insight, find_deity_insight,
    find_na_am_insight, find_ten_gods_insight, find_tiet_khi_insight, find_truc_insight,
    get_day_guidance,
};

pub use dto::*;
pub use dto::{NaAmErrorDto, NaAmLookupResultDto, NaAmResponseDto};

fn parse_bazi_gender(value: Option<&str>) -> Result<Option<amlich_core::Gender>, String> {
    match value.map(str::trim).filter(|value| !value.is_empty()) {
        None => Ok(None),
        Some("male" | "nam" | "Male" | "Nam") => Ok(Some(amlich_core::Gender::Male)),
        Some("female" | "nu" | "nữ" | "Female" | "Nu" | "Nữ") => {
            Ok(Some(amlich_core::Gender::Female))
        }
        Some(other) => Err(format!(
            "unsupported gender: {other}. supported values: male, female"
        )),
    }
}

fn to_bazi_input(query: &BaziQuery) -> Result<amlich_core::BaziInput, String> {
    if !(1..=12).contains(&query.month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&query.day) {
        return Err("day must be 1-31".to_string());
    }
    if query.hour > 23 {
        return Err("hour must be 0-23".to_string());
    }
    if query.minute > 59 {
        return Err("minute must be 0-59".to_string());
    }

    Ok(amlich_core::BaziInput {
        day: query.day,
        month: query.month,
        year: query.year,
        hour: query.hour,
        minute: query.minute,
        timezone: query.timezone.unwrap_or(amlich_core::VIETNAM_TIMEZONE),
        longitude: query.longitude,
        use_solar_time: query.use_solar_time,
        gender: parse_bazi_gender(query.gender.as_deref())?,
    })
}

fn require_bazi_gender(query: &BaziQuery) -> Result<amlich_core::Gender, String> {
    parse_bazi_gender(query.gender.as_deref())?.ok_or_else(|| {
        "gender is required for bazi timing/advisory. supported values: male, female".to_string()
    })
}

/// Convert snake_case to PascalCase (e.g. "ty_kien" -> "TyKien")
fn snake_to_pascal(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    upper + chars.as_str()
                }
            }
        })
        .collect()
}

pub fn get_day_info(query: &DateQuery) -> Result<DayInfoDto, String> {
    if !(1..=12).contains(&query.month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&query.day) {
        return Err("day must be 1-31".to_string());
    }

    let normalized_ruleset_id = normalize_ruleset_id(query.ruleset_id.as_deref())?;
    let normalized_event_kind = normalize_event_kind(query.event_kind.as_deref())?;
    let enabled_pack_ids = normalize_enabled_pack_ids(&query.enabled_pack_ids)?;

    let tz = query.timezone.unwrap_or(amlich_core::VIETNAM_TIMEZONE);
    let snapshot = amlich_core::calculate_day_snapshot_with_recommendation_request(
        query.day,
        query.month,
        query.year,
        tz,
        normalized_ruleset_id.as_deref(),
        normalized_event_kind.as_deref(),
        &enabled_pack_ids,
    )?;
    Ok(DayInfoDto::from(&snapshot))
}

pub fn get_bazi_chart(query: &BaziQuery) -> Result<BaziChartDto, String> {
    let input = to_bazi_input(query)?;
    let report = amlich_core::build_bazi_report(input, None)?;
    let response = report
        .chart_response
        .ok_or_else(|| "missing bazi chart response".to_string())?;
    Ok(BaziChartDto::from((query, &response)))
}

pub fn get_bazi_analysis(query: &BaziQuery) -> Result<BaziAnalysisDto, String> {
    let input = to_bazi_input(query)?;
    let report = amlich_core::build_bazi_report(input, None)?;
    let response = report
        .analysis_response
        .ok_or_else(|| "missing bazi analysis response".to_string())?;
    Ok(BaziAnalysisDto::from(&response))
}

pub fn get_bazi_timing(
    query: &BaziQuery,
    timing: &BaziTimingQuery,
) -> Result<BaziTimingDto, String> {
    let input = to_bazi_input(query)?;
    require_bazi_gender(query)?;
    let report = amlich_core::build_bazi_report(
        input,
        Some(amlich_core::BaziTimingInput {
            current_age: timing.current_age,
            target_year: timing.target_year,
            months: timing.months.clone(),
        }),
    )?;
    let response = report
        .timing_response
        .ok_or_else(|| "missing bazi timing response".to_string())?;
    Ok(BaziTimingDto::from(&response))
}

pub fn get_bazi_advisory(
    query: &BaziQuery,
    timing: Option<&BaziTimingQuery>,
) -> Result<BaziAdvisoryDto, String> {
    let input = to_bazi_input(query)?;
    let timing_input = match timing {
        Some(timing) => {
            require_bazi_gender(query)?;
            Some(amlich_core::BaziTimingInput {
                current_age: timing.current_age,
                target_year: timing.target_year,
                months: timing.months.clone(),
            })
        }
        None => None,
    };
    let report = amlich_core::build_bazi_report(input, timing_input)?;
    let response = report
        .advisory_response
        .ok_or_else(|| "missing bazi advisory response".to_string())?;
    Ok(BaziAdvisoryDto::from(&response))
}

pub fn get_bazi_metrics(
    query: &BaziQuery,
    timing: Option<&BaziTimingQuery>,
) -> Result<BaziComputedMetricsDto, String> {
    let input = to_bazi_input(query)?;
    let timing_input = match timing {
        Some(timing) => {
            require_bazi_gender(query)?;
            Some(amlich_core::BaziTimingInput {
                current_age: timing.current_age,
                target_year: timing.target_year,
                months: timing.months.clone(),
            })
        }
        None => None,
    };
    let report = amlich_core::build_bazi_report(input, timing_input)?;
    Ok(BaziComputedMetricsDto::from(&report.computed_metrics))
}

pub fn get_bazi_report(
    query: &BaziQuery,
    timing: Option<&BaziTimingQuery>,
) -> Result<BaziReportDto, String> {
    let input = to_bazi_input(query)?;
    let timing_input = match timing {
        Some(timing) => {
            require_bazi_gender(query)?;
            Some(amlich_core::BaziTimingInput {
                current_age: timing.current_age,
                target_year: timing.target_year,
                months: timing.months.clone(),
            })
        }
        None => None,
    };
    let report = amlich_core::build_bazi_report(input, timing_input)?;
    Ok(BaziReportDto::from((query, &report)))
}

fn normalize_ruleset_id(ruleset_id: Option<&str>) -> Result<Option<String>, String> {
    let Some(ruleset_id) = ruleset_id.map(str::trim).filter(|id| !id.is_empty()) else {
        return Ok(None);
    };

    let entry = get_ruleset(ruleset_id).map_err(|err| err.to_string())?;
    Ok(Some(entry.descriptor.id.to_string()))
}

fn normalize_event_kind(event_kind: Option<&str>) -> Result<Option<String>, String> {
    let Some(event_kind) = event_kind.map(str::trim).filter(|kind| !kind.is_empty()) else {
        return Ok(None);
    };

    match event_kind {
        "contract_signing" | "medical_checkup" | "travel" => {
            Ok(Some(event_kind.to_string()))
        }
        other => Err(format!(
            "unsupported recommendation event_kind: {other}. supported values: contract_signing, medical_checkup, travel"
        )),
    }
}

fn normalize_enabled_pack_ids(enabled_pack_ids: &[String]) -> Result<Vec<&str>, String> {
    let mut normalized = Vec::with_capacity(enabled_pack_ids.len());
    for pack_id in enabled_pack_ids {
        let trimmed = pack_id.trim();
        if trimmed.is_empty() {
            return Err("recommendation pack id must not be empty".to_string());
        }
        normalized.push(trimmed);
    }
    Ok(normalized)
}

pub fn get_day_info_for_date(day: i32, month: i32, year: i32) -> Result<DayInfoDto, String> {
    get_day_info(&DateQuery {
        day,
        month,
        year,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    })
}

pub fn get_ruleset_catalog() -> Vec<RulesetCatalogEntryDto> {
    ruleset_registry()
        .iter()
        .map(RulesetCatalogEntryDto::from)
        .collect()
}

pub fn get_recommendation_pack_catalog() -> Vec<RecommendationPackCatalogEntryDto> {
    recommendation_pack_descriptors()
        .iter()
        .map(RecommendationPackCatalogEntryDto::from)
        .collect()
}

pub fn get_holidays(year: i32, major_only: bool) -> Vec<HolidayDto> {
    get_vietnamese_holidays(year)
        .iter()
        .filter(|h| !major_only || h.is_major)
        .map(HolidayDto::from)
        .collect()
}

pub fn get_day_insight(query: &DateQuery) -> Result<DayInsightDto, String> {
    get_day_insight_with_profile(query, None, None, None, None)
}

pub fn get_day_insight_with_profile(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Result<DayInsightDto, String> {
    let day_info = get_day_info(query)?;

    let festival = lunar_festivals()
        .iter()
        .find(|item| {
            if item.is_solar {
                item.solar_day == Some(day_info.solar.day)
                    && item.solar_month == Some(day_info.solar.month)
            } else {
                item.lunar_day == day_info.lunar.day && item.lunar_month == day_info.lunar.month
            }
        })
        .map(FestivalInsightDto::from);

    let holiday = solar_holidays()
        .iter()
        .find(|item| {
            item.solar_day == day_info.solar.day && item.solar_month == day_info.solar.month
        })
        .map(HolidayInsightDto::from);

    let can_info = find_can(&day_info.canchi.day.can);
    let chi_info = find_chi(&day_info.canchi.day.chi);
    let element_index: &HashMap<String, amlich_core::insight_data::ElementInfo> = all_elements();

    let canchi = match (can_info, chi_info) {
        (Some(can), Some(chi)) => {
            let element = element_index
                .get(&can.element)
                .map(|el| ElementInsightDto::from((&can.element, el)));
            Some(CanChiInsightDto {
                can: CanInsightDto::from(can),
                chi: ChiInsightDto::from(chi),
                element,
            })
        }
        _ => None,
    };

    let day_guidance = get_day_guidance(&day_info.canchi.day.chi).map(DayGuidanceDto::from);
    let tiet_khi = find_tiet_khi_insight(&day_info.tiet_khi.name).map(TietKhiInsightDto::from);

    let fortune = day_info.day_fortune.as_ref();

    // Na Am insight
    let na_am = fortune.and_then(|f| {
        find_na_am_insight(&f.day_element.na_am).map(|n| NaAmInsightDto {
            na_am: f.day_element.na_am.clone(),
            element: f.day_element.element.clone(),
            meaning: LocalizedTextDto::from(&n.meaning),
        })
    });

    // Truc insight
    let truc = fortune.and_then(|f| {
        find_truc_insight(&f.truc.name).map(|t| TrucInsightDto {
            name: f.truc.name.clone(),
            quality: f.truc.quality.clone(),
            meaning: LocalizedTextDto::from(&t.meaning),
            good_for: LocalizedListDto::from(&t.good_for),
            avoid_for: LocalizedListDto::from(&t.avoid_for),
        })
    });

    // Day Deity insight
    let day_deity = fortune.and_then(|f| {
        f.day_deity.as_ref().map(|deity| {
            // DTO stores "hoang_dao"/"hac_dao", insight data uses "HoangDao"/"HacDao"
            let cls_id = match deity.classification.as_str() {
                "hoang_dao" => "HoangDao",
                "hac_dao" => "HacDao",
                other => other,
            };
            let cls_meaning = find_deity_classification_insight(cls_id)
                .map(|c| LocalizedTextDto::from(&c.meaning))
                .unwrap_or_else(|| LocalizedTextDto {
                    vi: String::new(),
                    en: String::new(),
                });
            let deity_meaning =
                find_deity_insight(&deity.name).map(|d| LocalizedTextDto::from(&d.meaning));
            DayDeityInsightDto {
                name: deity.name.clone(),
                classification: deity.classification.clone(),
                classification_meaning: cls_meaning,
                deity_meaning,
            }
        })
    });

    // Stars insight
    let stars = fortune.map(|f| StarsInsightDto {
        cat_tinh: f.stars.cat_tinh.clone(),
        sat_tinh: f.stars.sat_tinh.clone(),
        day_star: f.stars.day_star.as_ref().map(|s| s.name.clone()),
        day_star_quality: f.stars.day_star.as_ref().map(|s| s.quality.clone()),
    });

    // Taboos insight
    let taboos = fortune
        .map(|f| {
            f.taboos
                .iter()
                .map(|t| TabooInsightItemDto {
                    name: t.name.clone(),
                    severity: t.severity.clone(),
                    reason: t.reason.clone(),
                })
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty());

    // Travel insight
    let travel = fortune.map(|f| TravelInsightDto {
        xuat_hanh_huong: f.travel.xuat_hanh_huong.clone(),
        tai_than: f.travel.tai_than.clone(),
        hy_than: f.travel.hy_than.clone(),
    });

    // Xung Hop insight
    let xung_hop = fortune.map(|f| XungHopInsightDto {
        luc_xung: f.xung_hop.luc_xung.clone(),
        tam_hop: f.xung_hop.tam_hop.clone(),
        liu_he: f.xung_hop.liu_he.clone(),
        xiang_hai: f.xung_hop.xiang_hai.clone(),
    });

    // Tang Can insight
    let tang_can = fortune.and_then(|f| {
        f.tang_can.as_ref().map(|tc| TangCanInsightDto {
            main: tc.main.clone(),
            central: tc.central.clone(),
            residual: tc.residual.clone(),
            strength: tc.strength,
        })
    });

    // Ten Gods insight
    // DTO labels are snake_case (e.g. "ty_kien"), insight data IDs are PascalCase (e.g. "TyKien")
    let ten_gods = fortune.and_then(|f| {
        f.ten_gods.as_ref().map(|tg| {
            let map_entry = |r: &ThapThanResultDto| -> TenGodsEntryInsightDto {
                let pascal_id = snake_to_pascal(&r.label);
                let insight = find_ten_gods_insight(&pascal_id);
                TenGodsEntryInsightDto {
                    label: r.label.clone(),
                    name: insight
                        .map(|i| LocalizedTextDto::from(&i.name))
                        .unwrap_or_else(|| LocalizedTextDto {
                            vi: String::new(),
                            en: String::new(),
                        }),
                    meaning: insight
                        .map(|i| LocalizedTextDto::from(&i.meaning))
                        .unwrap_or_else(|| LocalizedTextDto {
                            vi: String::new(),
                            en: String::new(),
                        }),
                    relation: r.relation.clone(),
                    same_polarity: r.same_polarity,
                }
            };
            TenGodsInsightDto {
                to_year_stem: tg.to_year_stem.as_ref().map(map_entry),
                to_self: tg.to_self.as_ref().map(map_entry),
            }
        })
    });

    // Hours insight
    let hours = Some(HoursInsightDto {
        good_hour_count: day_info.gio_hoang_dao.good_hours.len(),
        good_hours: day_info
            .gio_hoang_dao
            .good_hours
            .iter()
            .map(|h| HourInsightEntryDto {
                chi: h.hour_chi.clone(),
                time_range: h.time_range.clone(),
                star: h.star.clone(),
            })
            .collect(),
    });

    // Birth-dependent: Tu Menh (Kua)
    let tu_menh = match (birth_year, gender) {
        (Some(by), Some(g)) => {
            use amlich_core::almanac::tu_menh::compute_kua;
            use amlich_core::insight_data::{find_kua_group_insight, find_kua_insight};
            let kua_result = compute_kua(by, g);
            let group_id = format!("{:?}", kua_result.group);
            let kua_insight = find_kua_insight(kua_result.kua);
            let group_insight = find_kua_group_insight(&group_id);
            let empty_text = || LocalizedTextDto {
                vi: String::new(),
                en: String::new(),
            };
            Some(TuMenhInsightDto {
                kua: kua_result.kua,
                group: group_id,
                trigram: kua_insight
                    .map(|k| LocalizedTextDto::from(&k.trigram))
                    .unwrap_or_else(empty_text),
                direction: kua_insight
                    .map(|k| LocalizedTextDto::from(&k.direction))
                    .unwrap_or_else(empty_text),
                meaning: kua_insight
                    .map(|k| LocalizedTextDto::from(&k.meaning))
                    .unwrap_or_else(empty_text),
                group_meaning: group_insight
                    .map(|g| LocalizedTextDto::from(&g.meaning))
                    .unwrap_or_else(empty_text),
                favorable_directions: kua_result
                    .favorable_directions
                    .iter()
                    .map(|d| format!("{:?}", d))
                    .collect(),
                unfavorable_directions: kua_result
                    .unfavorable_directions
                    .iter()
                    .map(|d| format!("{:?}", d))
                    .collect(),
            })
        }
        _ => None,
    };

    // Birth-dependent: Dai Van
    let dai_van = match (birth_day, birth_month, birth_year, gender) {
        (Some(bd), Some(bm), Some(by), Some(g)) => {
            use amlich_core::almanac::dai_van::calculate_dai_van;
            use amlich_core::insight_data::{
                dai_van_phases_insight, find_dai_van_direction_insight,
                find_dai_van_element_insight,
            };
            let dv = calculate_dai_van(bd, bm, by, g);
            let dir_id = format!("{:?}", dv.chieu_thu);
            let dir_insight = find_dai_van_direction_insight(&dir_id);
            let phases = dai_van_phases_insight();
            let empty_text = || LocalizedTextDto {
                vi: String::new(),
                en: String::new(),
            };
            let pillars: Vec<DaiVanPillarInsightDto> = dv
                .pillars
                .iter()
                .map(|p| {
                    let element = amlich_core::almanac::na_am::get_na_am_by_pair(
                        &p.can_chi.can,
                        &p.can_chi.chi,
                    )
                    .map(|e| e.element)
                    .unwrap_or_default();
                    let el_insight = find_dai_van_element_insight(&element);
                    DaiVanPillarInsightDto {
                        index: p.index,
                        can_chi: p.can_chi.full.clone(),
                        start_age: p.start_age,
                        end_age: p.end_age,
                        element,
                        element_meaning: el_insight
                            .map(|e| LocalizedTextDto::from(&e.meaning))
                            .unwrap_or_else(empty_text),
                    }
                })
                .collect();
            Some(DaiVanInsightDto {
                direction: dv.chieu_thu_label.clone(),
                direction_meaning: dir_insight
                    .map(|d| LocalizedTextDto::from(&d.meaning))
                    .unwrap_or_else(empty_text),
                start_age: dv.start_age_display.clone(),
                current_pillar: None,
                all_pillars: pillars,
                phases_meaning: LocalizedTextDto::from(&phases.meaning),
            })
        }
        _ => None,
    };

    Ok(DayInsightDto {
        solar: day_info.solar,
        lunar: day_info.lunar,
        festival,
        holiday,
        canchi,
        day_guidance,
        tiet_khi,
        na_am,
        truc,
        day_deity,
        stars,
        taboos,
        travel,
        xung_hop,
        tang_can,
        ten_gods,
        hours,
        tu_menh,
        dai_van,
    })
}

pub fn get_day_insight_for_date(day: i32, month: i32, year: i32) -> Result<DayInsightDto, String> {
    get_day_insight(&DateQuery {
        day,
        month,
        year,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    })
}

pub fn get_day_insight_for_date_with_profile(
    day: i32,
    month: i32,
    year: i32,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Result<DayInsightDto, String> {
    get_day_insight_with_profile(
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
        gender,
    )
}

/// Lookup Na Am by 1-based cycle index (1-60)
///
/// # Arguments
/// * `index` - 1-based cycle index in range [1, 60]
///
/// # Returns
/// * `NaAmResponseDto::Success` with Na Am details if index is valid
/// * `NaAmResponseDto::Error` with error details if index is invalid
///
/// # Examples
/// ```ignore
/// let response = get_na_am_by_index(1);
/// match response {
///     NaAmResponseDto::Success(result) => {
///         println!("Na Am: {}", result.na_am); // "Hải Trung Kim"
///     }
///     NaAmResponseDto::Error(err) => {
///         eprintln!("Error: {}", err.message);
///     }
/// }
/// ```
pub fn get_na_am_by_index(index: u8) -> NaAmResponseDto {
    use amlich_core::almanac::na_am::get_na_am_by_index;

    match get_na_am_by_index(index) {
        Ok(entry) => NaAmResponseDto::Success(NaAmLookupResultDto::from(&entry)),
        Err(error) => NaAmResponseDto::Error(NaAmErrorDto::from(error)),
    }
}

/// Lookup Na Am by stem-branch pair (Vietnamese names)
///
/// # Arguments
/// * `can` - Vietnamese stem name (e.g., "Giáp", "Ất")
/// * `chi` - Vietnamese branch name (e.g., "Tý", "Sửu")
///
/// # Returns
/// * `NaAmResponseDto::Success` with Na Am details if pair is valid
/// * `NaAmResponseDto::Error` with error details if pair is invalid
///
/// # Examples
///
/// Lookup by cycle index (1-60):
///
/// ```ignore
/// use amlich_api::{get_na_am_by_index, get_na_am_by_pair};
///
/// let result = get_na_am_by_index(1);
/// match result {
///     NaAmResponseDto::Success(data) => {
///         println!("Index {}: {} {} - {} ({})",
///             data.cycle_index, data.can, data.chi, data.na_am, data.element);
///     }
///     NaAmResponseDto::Error(err) => {
///         eprintln!("Error: {}", err.message);
///     }
/// }
/// ```
///
/// Lookup by stem-branch pair:
///
/// ```ignore
/// let result = get_na_am_by_pair("Giáp", "Tý");
/// match result {
///     NaAmResponseDto::Success(data) => {
///         println!("{} {}: {} ({})",
///             data.can, data.chi, data.na_am, data.element);
///     }
///     NaAmResponseDto::Error(err) => {
///         eprintln!("Error: {}", err.message);
///     }
/// }
/// ```
pub fn get_na_am_by_pair(can: &str, chi: &str) -> NaAmResponseDto {
    use amlich_core::almanac::na_am::get_na_am_by_pair;

    match get_na_am_by_pair(can, chi) {
        Ok(entry) => NaAmResponseDto::Success(NaAmLookupResultDto::from(&entry)),
        Err(error) => NaAmResponseDto::Error(NaAmErrorDto::from(error)),
    }
}
