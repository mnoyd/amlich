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

    // Birth-dependent: Yearly Han (composite assessment)
    let yearly_han = match (birth_year, gender) {
        (Some(by), Some(g)) => {
            use amlich_core::almanac::yearly_han::{compute_yearly_han, YearlyHanInput};
            let current_year = query.year;
            // Derive chi indices: (year - 4) % 12 aligns with canchi convention
            let birth_chi = ((by - 4).rem_euclid(12)) as usize;
            let year_chi = ((current_year - 4).rem_euclid(12)) as usize;
            let input = YearlyHanInput {
                birth_lunar_year: by,
                current_lunar_year: current_year,
                gender: g,
            };
            let han = compute_yearly_han(&input, birth_chi, year_chi);
            Some(YearlyHanInsightDto {
                sao_han: CuuDieuInsightDto {
                    star_index: han.sao_han.star_index,
                    star_name: han.sao_han.star_name,
                    quality: format!("{:?}", han.sao_han.quality).to_lowercase(),
                    is_han: han.sao_han.is_han,
                    element: han.sao_han.element,
                },
                tam_tai: TamTaiInsightDto {
                    in_tam_tai: han.tam_tai.in_tam_tai,
                    year_position: han.tam_tai.year_position,
                    severity: han.tam_tai.severity.map(|s| format!("{:?}", s).to_lowercase()),
                    tam_hop_group: han.tam_tai.tam_hop_group,
                    tai_years: han.tam_tai.tai_years,
                },
                kim_lau: KimLauInsightDto {
                    in_kim_lau: han.kim_lau.in_kim_lau,
                    category: han.kim_lau.category.map(|c| format!("{:?}", c).to_lowercase()),
                    remainder: han.kim_lau.remainder,
                    tuoi_mu: han.kim_lau.tuoi_mu,
                },
                hoang_oc: HoangOcInsightDto {
                    position: han.hoang_oc.position,
                    position_name: han.hoang_oc.position_name,
                    is_good: han.hoang_oc.is_good,
                    tuoi_mu: han.hoang_oc.tuoi_mu,
                },
                thai_tue: ThaiTueInsightDto {
                    conflicts: han
                        .thai_tue
                        .conflicts
                        .iter()
                        .map(|c| ThaiTueConflictDto {
                            kind: format!("{:?}", c.kind).to_lowercase(),
                            description: c.description.clone(),
                        })
                        .collect(),
                    has_conflict: han.thai_tue.has_conflict,
                },
                han_count: han.han_count,
                is_chong_han: han.is_chong_han,
                severity: format!("{:?}", han.severity).to_lowercase(),
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
        yearly_han,
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

fn profile_gender_label(
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Option<String> {
    gender.map(|value| match value {
        amlich_core::almanac::tu_menh::Gender::Male => "male".to_string(),
        amlich_core::almanac::tu_menh::Gender::Female => "female".to_string(),
    })
}

fn personal_day_query(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> PersonalDayQueryDto {
    PersonalDayQueryDto {
        date: query.clone(),
        birth_year,
        birth_month,
        birth_day,
        gender: profile_gender_label(gender),
    }
}

pub fn get_personal_day_chart(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Result<PersonalDayChartDto, String> {
    let insight = get_day_insight_with_profile(query, birth_year, birth_month, birth_day, gender)?;
    Ok(PersonalDayChartDto {
        input: personal_day_query(query, birth_year, birth_month, birth_day, gender),
        solar: insight.solar,
        lunar: insight.lunar,
        canchi: insight.canchi,
        tiet_khi: insight.tiet_khi,
    })
}

pub fn get_personal_day_analysis(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Result<PersonalDayAnalysisDto, String> {
    let insight = get_day_insight_with_profile(query, birth_year, birth_month, birth_day, gender)?;
    Ok(PersonalDayAnalysisDto {
        ten_gods: insight.ten_gods,
        xung_hop: insight.xung_hop,
        tang_can: insight.tang_can,
        tu_menh: insight.tu_menh,
        dai_van: insight.dai_van,
        yearly_han: insight.yearly_han,
    })
}

pub fn get_personal_day_metrics(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Result<PersonalDayMetricsDto, String> {
    let insight = get_day_insight_with_profile(query, birth_year, birth_month, birth_day, gender)?;
    let mut available_sections = Vec::new();
    if insight.ten_gods.is_some() {
        available_sections.push("ten_gods".to_string());
    }
    if insight.xung_hop.is_some() {
        available_sections.push("xung_hop".to_string());
    }
    if insight.tang_can.is_some() {
        available_sections.push("tang_can".to_string());
    }
    if insight.tu_menh.is_some() {
        available_sections.push("tu_menh".to_string());
    }
    if insight.dai_van.is_some() {
        available_sections.push("dai_van".to_string());
    }
    if insight.yearly_han.is_some() {
        available_sections.push("yearly_han".to_string());
    }

    let profile_completeness = [
        birth_year.is_some(),
        birth_month.is_some(),
        birth_day.is_some(),
        gender.is_some(),
    ]
    .into_iter()
    .filter(|value| *value)
    .count() as u8;

    Ok(PersonalDayMetricsDto {
        profile_completeness,
        has_personal_recommendations: insight.tu_menh.is_some()
            || insight.dai_van.is_some()
            || insight.yearly_han.is_some(),
        available_sections,
    })
}

pub fn get_personal_day_advisory(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Result<PersonalDayAdvisoryDto, String> {
    let insight = get_day_insight_with_profile(query, birth_year, birth_month, birth_day, gender)?;
    let mut highlights = Vec::new();
    let mut cautions = Vec::new();

    if let Some(tu_menh) = &insight.tu_menh {
        highlights.push(format!("kua {} {}", tu_menh.kua, tu_menh.group));
    } else {
        cautions.push("missing kua profile context".to_string());
    }

    if let Some(dai_van) = &insight.dai_van {
        highlights.push(format!("dai_van {}", dai_van.direction));
    } else {
        cautions.push("missing dai van timing context".to_string());
    }

    if insight.ten_gods.is_none() {
        cautions.push("ten gods analysis unavailable".to_string());
    }

    if let Some(han) = &insight.yearly_han {
        if han.han_count == 0 {
            highlights.push("no yearly han active".to_string());
        } else {
            if han.sao_han.is_han {
                cautions.push(format!("sao han: {} ({})", han.sao_han.star_name, han.sao_han.quality));
            }
            if han.tam_tai.in_tam_tai {
                let sev = han.tam_tai.severity.as_deref().unwrap_or("unknown");
                cautions.push(format!("tam tai year {} ({})", han.tam_tai.year_position.unwrap_or(0), sev));
            }
            if han.kim_lau.in_kim_lau {
                let cat = han.kim_lau.category.as_deref().unwrap_or("unknown");
                cautions.push(format!("kim lau: {}", cat));
            }
            if !han.hoang_oc.is_good {
                cautions.push(format!("hoang oc: {}", han.hoang_oc.position_name));
            }
            if han.thai_tue.has_conflict {
                let kinds: Vec<&str> = han.thai_tue.conflicts.iter().map(|c| c.kind.as_str()).collect();
                cautions.push(format!("thai tue: {}", kinds.join(", ")));
            }
            if han.is_chong_han {
                cautions.push(format!("han chong han: {} active ({})", han.han_count, han.severity));
            }
        }
    }

    Ok(PersonalDayAdvisoryDto { highlights, cautions })
}

pub fn get_personal_day_report(
    query: &DateQuery,
    birth_year: Option<i32>,
    birth_month: Option<i32>,
    birth_day: Option<i32>,
    gender: Option<amlich_core::almanac::tu_menh::Gender>,
) -> Result<PersonalDayReportDto, String> {
    Ok(PersonalDayReportDto {
        chart: get_personal_day_chart(query, birth_year, birth_month, birth_day, gender)?,
        analysis: get_personal_day_analysis(query, birth_year, birth_month, birth_day, gender)?,
        computed_metrics: get_personal_day_metrics(
            query, birth_year, birth_month, birth_day, gender,
        )?,
        advisory: get_personal_day_advisory(query, birth_year, birth_month, birth_day, gender)?,
    })
}

fn get_hour_selection_day_info(query: &DateQuery) -> Result<DayInfoDto, String> {
    get_day_info(query)
}

pub fn get_hour_selection_chart(query: &DateQuery) -> Result<HourSelectionChartDto, String> {
    let info = get_hour_selection_day_info(query)?;
    Ok(HourSelectionChartDto {
        input: HourSelectionQueryDto {
            date: query.clone(),
        },
        solar: info.solar,
        lunar: info.lunar,
        gio_hoang_dao: info.gio_hoang_dao,
    })
}

pub fn get_hour_selection_analysis(query: &DateQuery) -> Result<HourSelectionAnalysisDto, String> {
    let info = get_hour_selection_day_info(query)?;
    let bad_hours = info
        .gio_hoang_dao
        .all_hours
        .iter()
        .filter(|hour| !hour.is_good)
        .cloned()
        .collect();
    Ok(HourSelectionAnalysisDto {
        good_hours: info.gio_hoang_dao.good_hours.clone(),
        bad_hours,
        summary: info.gio_hoang_dao.summary,
    })
}

pub fn get_hour_selection_metrics(query: &DateQuery) -> Result<HourSelectionMetricsDto, String> {
    let info = get_hour_selection_day_info(query)?;
    let total = info.gio_hoang_dao.all_hours.len();
    let good = info.gio_hoang_dao.good_hour_count;
    Ok(HourSelectionMetricsDto {
        good_hour_count: good,
        bad_hour_count: total.saturating_sub(good),
        good_hour_ratio: if total == 0 {
            0.0
        } else {
            good as f32 / total as f32
        },
    })
}

pub fn get_hour_selection_advisory(query: &DateQuery) -> Result<HourSelectionAdvisoryDto, String> {
    let info = get_hour_selection_day_info(query)?;
    Ok(HourSelectionAdvisoryDto {
        best_windows: info
            .gio_hoang_dao
            .good_hours
            .iter()
            .map(|hour| format!("{} {}", hour.hour_chi, hour.time_range))
            .collect(),
        caution_windows: info
            .gio_hoang_dao
            .all_hours
            .iter()
            .filter(|hour| !hour.is_good)
            .map(|hour| format!("{} {}", hour.hour_chi, hour.time_range))
            .collect(),
    })
}

pub fn get_hour_selection_report(query: &DateQuery) -> Result<HourSelectionReportDto, String> {
    Ok(HourSelectionReportDto {
        chart: get_hour_selection_chart(query)?,
        analysis: get_hour_selection_analysis(query)?,
        computed_metrics: get_hour_selection_metrics(query)?,
        advisory: get_hour_selection_advisory(query)?,
    })
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
