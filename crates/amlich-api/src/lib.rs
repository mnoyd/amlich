mod convert;
mod dto;

use amlich_core::holidays::get_vietnamese_holidays;

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
