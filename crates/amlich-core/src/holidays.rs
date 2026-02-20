use crate::holiday_data::{lunar_festivals, solar_holidays};
/**
 * Vietnamese Holidays Module
 *
 * Provides functions to get Vietnamese lunar holidays for a given year.
 * Holiday data is loaded from shared JSON files at compile time.
 */
use crate::julian::{jd_from_date, jd_to_date};
use crate::lunar::{convert_lunar_to_solar, LunarDate};
use crate::tietkhi::get_all_tiet_khi_for_year;
use crate::types::VIETNAM_TIMEZONE;

/// Information about a Vietnamese holiday
#[derive(Debug, Clone)]
pub struct Holiday {
    pub name: String,
    pub description: String,
    pub lunar_date: Option<LunarDate>,
    pub solar_day: i32,
    pub solar_month: i32,
    pub solar_year: i32,
    pub is_solar: bool,
    pub category: String,
    pub is_major: bool,
}

struct LunarHolidayInput<'a> {
    name: &'a str,
    lunar_day: i32,
    lunar_month: i32,
    lunar_year: i32,
    description: &'a str,
    category: &'a str,
    is_major: bool,
}

// Helper function to create a lunar holiday
fn create_lunar_holiday(input: LunarHolidayInput<'_>, time_zone: f64) -> Option<Holiday> {
    let solar = convert_lunar_to_solar(
        input.lunar_day,
        input.lunar_month,
        input.lunar_year,
        false,
        time_zone,
    );
    if solar.0 > 0 {
        Some(Holiday {
            name: input.name.to_string(),
            description: input.description.to_string(),
            lunar_date: Some(LunarDate {
                day: input.lunar_day,
                month: input.lunar_month,
                year: input.lunar_year,
                is_leap: false,
            }),
            solar_day: solar.0,
            solar_month: solar.1,
            solar_year: solar.2,
            is_solar: false,
            category: input.category.to_string(),
            is_major: input.is_major,
        })
    } else {
        None
    }
}

fn nth_weekday_of_month(year: i32, month: i32, weekday: usize, nth: i32) -> (i32, i32, i32) {
    let first_jd = jd_from_date(1, month, year);
    let first_weekday = (first_jd + 1) % 7;
    let target_weekday = weekday as i32;
    let offset = (7 + target_weekday - first_weekday) % 7;
    let day = 1 + offset + 7 * (nth - 1);
    (day, month, year)
}

/// Get all Vietnamese lunar holidays for a given solar year
///
/// # Arguments
/// * `solar_year` - Solar year
///
/// # Returns
/// Vector of holidays sorted by date
pub fn get_vietnamese_holidays(solar_year: i32) -> Vec<Holiday> {
    let time_zone = VIETNAM_TIMEZONE;
    let mut holidays = Vec::new();

    // -- Lunar festivals from shared JSON data --
    for festival in lunar_festivals() {
        // Skip solar-based entries (e.g. Thanh Minh) — handled separately via solar term computation
        if festival.is_solar {
            continue;
        }

        let name = &festival.names.vi[0];
        let description = &festival.names.en[0];
        let lunar_year = solar_year + festival.year_offset;

        if let Some(h) = create_lunar_holiday(
            LunarHolidayInput {
                name,
                lunar_day: festival.lunar_day,
                lunar_month: festival.lunar_month,
                lunar_year,
                description,
                category: &festival.category,
                is_major: festival.is_major,
            },
            time_zone,
        ) {
            holidays.push(h);
        }
    }

    // Thanh Minh (solar-based, computed from solar term transition)
    let thanh_minh_date = get_all_tiet_khi_for_year(solar_year, time_zone)
        .into_iter()
        .find(|t| t.name == "Thanh Minh")
        .map(|t| jd_to_date(t.jd))
        .unwrap_or((5, 4, solar_year));

    holidays.push(Holiday {
        name: "Tết Thanh Minh".to_string(),
        description: "Tomb Sweeping Day (Solar calendar)".to_string(),
        lunar_date: None,
        solar_day: thanh_minh_date.0,
        solar_month: thanh_minh_date.1,
        solar_year: thanh_minh_date.2,
        is_solar: true,
        category: "festival".to_string(),
        is_major: true,
    });

    // -- Solar holidays from shared JSON data --
    for holiday_data in solar_holidays() {
        let name = &holiday_data.names.vi[0];
        let description = &holiday_data.names.en[0];

        holidays.push(Holiday {
            name: name.clone(),
            description: description.clone(),
            lunar_date: None,
            solar_day: holiday_data.solar_day,
            solar_month: holiday_data.solar_month,
            solar_year,
            is_solar: true,
            category: holiday_data.category.clone(),
            is_major: holiday_data.is_major,
        });
    }

    let mothers_day = nth_weekday_of_month(solar_year, 5, 0, 2);
    holidays.push(Holiday {
        name: "Ngày của Mẹ".to_string(),
        description: "Mother's Day (2nd Sunday of May)".to_string(),
        lunar_date: None,
        solar_day: mothers_day.0,
        solar_month: mothers_day.1,
        solar_year: mothers_day.2,
        is_solar: true,
        category: "social".to_string(),
        is_major: true,
    });

    let fathers_day = nth_weekday_of_month(solar_year, 6, 0, 3);
    holidays.push(Holiday {
        name: "Ngày của Cha".to_string(),
        description: "Father's Day (3rd Sunday of June)".to_string(),
        lunar_date: None,
        solar_day: fathers_day.0,
        solar_month: fathers_day.1,
        solar_year: fathers_day.2,
        is_solar: true,
        category: "social".to_string(),
        is_major: true,
    });

    // Add all Rằm (15th) and Mùng 1 (1st) of each lunar month
    for month in 1..=12 {
        let mung_mot_name = format!("Mùng 1 tháng {}", month);
        if let Some(h) = create_lunar_holiday(
            LunarHolidayInput {
                name: &mung_mot_name,
                lunar_day: 1,
                lunar_month: month,
                lunar_year: solar_year,
                description: "First day of lunar month",
                category: "lunar-cycle",
                is_major: false,
            },
            time_zone,
        ) {
            holidays.push(h);
        }

        let ram_name = format!("Rằm tháng {}", month);
        if let Some(h) = create_lunar_holiday(
            LunarHolidayInput {
                name: &ram_name,
                lunar_day: 15,
                lunar_month: month,
                lunar_year: solar_year,
                description: "Full moon day",
                category: "lunar-cycle",
                is_major: false,
            },
            time_zone,
        ) {
            holidays.push(h);
        }
    }

    // Sort by date
    holidays.sort_by(|a, b| {
        let date_a = (a.solar_year, a.solar_month, a.solar_day);
        let date_b = (b.solar_year, b.solar_month, b.solar_day);
        date_a.cmp(&date_b)
    });

    holidays
}

/// Get major Vietnamese holidays only (no monthly Mùng 1/Rằm)
///
/// # Arguments
/// * `solar_year` - Solar year
///
/// # Returns
/// Vector of major holidays only
pub fn get_major_holidays(solar_year: i32) -> Vec<Holiday> {
    get_vietnamese_holidays(solar_year)
        .into_iter()
        .filter(|h| h.is_major)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_vietnamese_holidays_2024() {
        let holidays = get_vietnamese_holidays(2024);

        // Should have holidays (at least Tết, major festivals, + monthly dates)
        assert!(holidays.len() > 20, "Should have many holidays");

        // Check that holidays are sorted
        for i in 1..holidays.len() {
            let date_prev = (
                holidays[i - 1].solar_year,
                holidays[i - 1].solar_month,
                holidays[i - 1].solar_day,
            );
            let date_curr = (
                holidays[i].solar_year,
                holidays[i].solar_month,
                holidays[i].solar_day,
            );
            assert!(date_curr >= date_prev, "Holidays should be sorted by date");
        }
    }

    #[test]
    fn test_tet_nguyen_dan_present() {
        let holidays = get_vietnamese_holidays(2024);

        // Should have Tết Nguyên Đán
        let tet = holidays.iter().find(|h| h.name.contains("Tết Nguyên Đán"));
        assert!(tet.is_some(), "Should have Tết Nguyên Đán");

        let tet = tet.unwrap();
        // Tết 2024 is February 10, 2024
        assert_eq!(tet.solar_day, 10);
        assert_eq!(tet.solar_month, 2);
        assert_eq!(tet.solar_year, 2024);
    }

    #[test]
    fn test_thanh_minh_is_solar() {
        let holidays = get_vietnamese_holidays(2024);

        let thanh_minh = holidays.iter().find(|h| h.name.contains("Thanh Minh"));
        assert!(thanh_minh.is_some(), "Should have Thanh Minh");

        let thanh_minh = thanh_minh.unwrap();
        assert!(thanh_minh.is_solar, "Thanh Minh should be solar-based");
        assert_eq!(thanh_minh.solar_month, 4);
        assert_eq!(thanh_minh.solar_day, 5);
    }

    #[test]
    fn test_trung_thu_present() {
        let holidays = get_vietnamese_holidays(2024);

        let trung_thu = holidays.iter().find(|h| h.name.contains("Trung Thu"));
        assert!(trung_thu.is_some(), "Should have Tết Trung Thu");

        // Trung Thu is 15/8 lunar
        let trung_thu = trung_thu.unwrap();
        assert!(trung_thu.lunar_date.is_some());
        let lunar = trung_thu.lunar_date.as_ref().unwrap();
        assert_eq!(lunar.day, 15);
        assert_eq!(lunar.month, 8);
    }

    #[test]
    fn test_get_major_holidays() {
        let all = get_vietnamese_holidays(2024);
        let major = get_major_holidays(2024);

        // Major holidays should be fewer than all holidays
        assert!(major.len() < all.len(), "Major holidays should be a subset");

        // Should still have Tết
        assert!(major.iter().any(|h| h.name.contains("Tết Nguyên Đán")));

        // Should still have Trung Thu
        assert!(major.iter().any(|h| h.name.contains("Trung Thu")));
    }

    #[test]
    fn test_lunar_dates_populated() {
        let holidays = get_vietnamese_holidays(2024);

        // Most holidays should have lunar dates (except Thanh Minh)
        let with_lunar = holidays.iter().filter(|h| h.lunar_date.is_some()).count();
        assert!(with_lunar > 20, "Most holidays should have lunar dates");
    }

    #[test]
    fn test_floating_mother_father_days() {
        let holidays = get_vietnamese_holidays(2024);

        let mother_day = holidays.iter().find(|h| h.name == "Ngày của Mẹ");
        assert!(mother_day.is_some(), "Should have Mother's Day");
        let mother_day = mother_day.unwrap();
        assert_eq!(mother_day.solar_day, 12);
        assert_eq!(mother_day.solar_month, 5);

        let father_day = holidays.iter().find(|h| h.name == "Ngày của Cha");
        assert!(father_day.is_some(), "Should have Father's Day");
        let father_day = father_day.unwrap();
        assert_eq!(father_day.solar_day, 16);
        assert_eq!(father_day.solar_month, 6);
    }
}
