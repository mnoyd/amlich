mod convert;
mod dto;

use std::collections::HashMap;

use amlich_core::holiday_data::{lunar_festivals, solar_holidays};
use amlich_core::holidays::get_vietnamese_holidays;
use amlich_core::insight_data::{
    all_elements, find_can, find_chi, find_tiet_khi_insight, get_day_guidance,
};

pub use dto::*;

pub fn get_day_info(query: &DateQuery) -> Result<DayInfoDto, String> {
    if !(1..=12).contains(&query.month) {
        return Err("month must be 1-12".to_string());
    }
    if !(1..=31).contains(&query.day) {
        return Err("day must be 1-31".to_string());
    }

    let tz = query.timezone.unwrap_or(amlich_core::VIETNAM_TIMEZONE);
    let info = amlich_core::get_day_info_with_timezone(query.day, query.month, query.year, tz);
    Ok(DayInfoDto::from(&info))
}

pub fn get_day_info_for_date(day: i32, month: i32, year: i32) -> Result<DayInfoDto, String> {
    get_day_info(&DateQuery {
        day,
        month,
        year,
        timezone: None,
    })
}

pub fn get_holidays(year: i32, major_only: bool) -> Vec<HolidayDto> {
    get_vietnamese_holidays(year)
        .iter()
        .filter(|h| !major_only || h.is_major)
        .map(HolidayDto::from)
        .collect()
}

pub fn get_day_insight(query: &DateQuery) -> Result<DayInsightDto, String> {
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

    Ok(DayInsightDto {
        solar: day_info.solar,
        lunar: day_info.lunar,
        festival,
        holiday,
        canchi,
        day_guidance,
        tiet_khi,
    })
}

pub fn get_day_insight_for_date(day: i32, month: i32, year: i32) -> Result<DayInsightDto, String> {
    get_day_insight(&DateQuery {
        day,
        month,
        year,
        timezone: None,
    })
}
