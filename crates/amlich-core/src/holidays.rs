/**
 * Vietnamese Holidays Module
 *
 * Provides functions to get Vietnamese lunar holidays for a given year
 */
use crate::lunar::{convert_lunar_to_solar, LunarDate};

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
}

// Helper function to create a lunar holiday
fn create_lunar_holiday(
    name: &str,
    lunar_day: i32,
    lunar_month: i32,
    lunar_year: i32,
    description: &str,
    time_zone: f64,
) -> Option<Holiday> {
    let solar = convert_lunar_to_solar(lunar_day, lunar_month, lunar_year, false, time_zone);
    if solar.0 > 0 {
        Some(Holiday {
            name: name.to_string(),
            description: description.to_string(),
            lunar_date: Some(LunarDate {
                day: lunar_day,
                month: lunar_month,
                year: lunar_year,
                is_leap: false,
            }),
            solar_day: solar.0,
            solar_month: solar.1,
            solar_year: solar.2,
            is_solar: false,
        })
    } else {
        None
    }
}

/// Get all Vietnamese lunar holidays for a given solar year
///
/// # Arguments
/// * `solar_year` - Solar year
///
/// # Returns
/// Vector of holidays sorted by date
pub fn get_vietnamese_holidays(solar_year: i32) -> Vec<Holiday> {
    let time_zone = 7.0;
    let mut holidays = Vec::new();

    // Add major holidays
    let major = vec![
        (
            "Tết Nguyên Đán (Mùng 1 Tết)",
            1,
            1,
            solar_year,
            "Vietnamese New Year - First day",
        ),
        ("Mùng 2 Tết", 2, 1, solar_year, "Second day of Tết"),
        ("Mùng 3 Tết", 3, 1, solar_year, "Third day of Tết"),
        (
            "Tết Nguyên Tiêu (Rằm tháng Giêng)",
            15,
            1,
            solar_year,
            "Lantern Festival",
        ),
        ("Tết Hàn Thực", 3, 3, solar_year, "Cold Food Festival"),
        (
            "Phật Đản (Rằm tháng Tư)",
            15,
            4,
            solar_year,
            "Buddha's Birthday",
        ),
        ("Tết Đoan Ngọ", 5, 5, solar_year, "Dragon Boat Festival"),
        (
            "Vu Lan (Rằm tháng Bảy)",
            15,
            7,
            solar_year,
            "Parents' Day / Wandering Souls",
        ),
        (
            "Tết Trung Thu (Rằm tháng Tám)",
            15,
            8,
            solar_year,
            "Mid-Autumn Festival / Children's Festival",
        ),
        ("Tết Trùng Cửu", 9, 9, solar_year, "Double Ninth Festival"),
        (
            "Tết Hạ Nguyên (Rằm tháng Mười)",
            15,
            10,
            solar_year,
            "Lower Nguyên Festival",
        ),
        (
            "Ông Táo chầu trời",
            23,
            12,
            solar_year - 1,
            "Kitchen Gods go to Heaven",
        ),
        (
            "Giao Thừa (Đêm giao thừa)",
            30,
            12,
            solar_year - 1,
            "New Year's Eve",
        ),
    ];

    for (name, day, month, year, desc) in major {
        if let Some(h) = create_lunar_holiday(name, day, month, year, desc, time_zone) {
            holidays.push(h);
        }
    }

    // Thanh Minh (solar-based)
    holidays.push(Holiday {
        name: "Tết Thanh Minh".to_string(),
        description: "Tomb Sweeping Day (Solar calendar)".to_string(),
        lunar_date: None,
        solar_day: 5,
        solar_month: 4,
        solar_year,
        is_solar: true,
    });

    // Add all Rằm (15th) and Mùng 1 (1st) of each lunar month
    for month in 1..=12 {
        if let Some(h) = create_lunar_holiday(
            &format!("Mùng 1 tháng {}", month),
            1,
            month,
            solar_year,
            "First day of lunar month",
            time_zone,
        ) {
            holidays.push(h);
        }

        if let Some(h) = create_lunar_holiday(
            &format!("Rằm tháng {}", month),
            15,
            month,
            solar_year,
            "Full moon day",
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
    let all_holidays = get_vietnamese_holidays(solar_year);
    all_holidays
        .into_iter()
        .filter(|h| {
            // Filter out the monthly Mùng 1 and Rằm except the special ones
            !(h.name.starts_with("Mùng 1 tháng") && !h.name.contains("Tết"))
                && !(h.name.starts_with("Rằm tháng") && !h.description.contains("Festival"))
        })
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
}
