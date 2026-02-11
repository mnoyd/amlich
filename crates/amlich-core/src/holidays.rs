/**
 * Vietnamese Holidays Module
 *
 * Provides functions to get Vietnamese lunar holidays for a given year
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

// Helper function to create a lunar holiday
fn create_lunar_holiday(
    name: &str,
    lunar_day: i32,
    lunar_month: i32,
    lunar_year: i32,
    description: &str,
    category: &str,
    is_major: bool,
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
            category: category.to_string(),
            is_major,
        })
    } else {
        None
    }
}

fn nth_weekday_of_month(year: i32, month: i32, weekday: usize, nth: i32) -> (i32, i32, i32) {
    let first_jd = jd_from_date(1, month, year);
    let first_weekday = ((first_jd + 1) % 7) as i32;
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

    // Add major holidays
    let major = vec![
        (
            "Tết Nguyên Đán (Mùng 1 Tết)",
            1,
            1,
            solar_year,
            "Vietnamese New Year - First day",
            "festival",
            true,
        ),
        (
            "Mùng 2 Tết",
            2,
            1,
            solar_year,
            "Second day of Tết",
            "festival",
            true,
        ),
        (
            "Mùng 3 Tết",
            3,
            1,
            solar_year,
            "Third day of Tết",
            "festival",
            true,
        ),
        (
            "Tết Nguyên Tiêu (Rằm tháng Giêng)",
            15,
            1,
            solar_year,
            "Lantern Festival",
            "festival",
            true,
        ),
        (
            "Tết Hàn Thực",
            3,
            3,
            solar_year,
            "Cold Food Festival",
            "festival",
            true,
        ),
        (
            "Phật Đản (Rằm tháng Tư)",
            15,
            4,
            solar_year,
            "Buddha's Birthday",
            "festival",
            true,
        ),
        (
            "Tết Đoan Ngọ",
            5,
            5,
            solar_year,
            "Dragon Boat Festival",
            "festival",
            true,
        ),
        (
            "Vu Lan (Rằm tháng Bảy)",
            15,
            7,
            solar_year,
            "Parents' Day / Wandering Souls",
            "festival",
            true,
        ),
        (
            "Tết Trung Thu (Rằm tháng Tám)",
            15,
            8,
            solar_year,
            "Mid-Autumn Festival / Children's Festival",
            "festival",
            true,
        ),
        (
            "Tết Trùng Cửu",
            9,
            9,
            solar_year,
            "Double Ninth Festival",
            "festival",
            true,
        ),
        (
            "Tết Hạ Nguyên (Rằm tháng Mười)",
            15,
            10,
            solar_year,
            "Lower Nguyên Festival",
            "festival",
            true,
        ),
        (
            "Ông Táo chầu trời",
            23,
            12,
            solar_year - 1,
            "Kitchen Gods go to Heaven",
            "festival",
            true,
        ),
        (
            "Giao Thừa (Đêm giao thừa)",
            30,
            12,
            solar_year - 1,
            "New Year's Eve",
            "festival",
            true,
        ),
    ];

    for (name, day, month, year, desc, category, is_major) in major {
        if let Some(h) =
            create_lunar_holiday(name, day, month, year, desc, category, is_major, time_zone)
        {
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

    // National / Solar holidays
    let national_holidays = vec![
        // Public holidays (ngày lễ chính thức, nghỉ lễ)
        (
            "Tết Dương Lịch",
            1,
            1,
            "New Year's Day",
            "public-holiday",
            true,
        ),
        (
            "Ngày Giải Phóng Miền Nam",
            30,
            4,
            "Reunification Day",
            "public-holiday",
            true,
        ),
        (
            "Ngày Quốc Tế Lao Động",
            1,
            5,
            "International Workers' Day",
            "public-holiday",
            true,
        ),
        (
            "Ngày Quốc Khánh",
            2,
            9,
            "National Day / Independence Day",
            "public-holiday",
            true,
        ),
        // Commemorative days (ngày kỷ niệm)
        (
            "Ngày Thành Lập Đảng",
            3,
            2,
            "Founding of the Communist Party of Vietnam",
            "commemorative",
            true,
        ),
        (
            "Lễ Tình Nhân (Valentine)",
            14,
            2,
            "Valentine's Day",
            "social",
            true,
        ),
        (
            "Ngày Quyền Của Người Tiêu Dùng Việt Nam",
            15,
            3,
            "Vietnam Consumer Rights Day",
            "social",
            true,
        ),
        (
            "Ngày Chiến Thắng Điện Biên Phủ",
            7,
            5,
            "Điện Biên Phủ Victory Day",
            "commemorative",
            true,
        ),
        (
            "Ngày Quốc Tế Gia Đình",
            15,
            5,
            "International Day of Families",
            "international",
            true,
        ),
        (
            "Ngày Sinh Chủ Tịch Hồ Chí Minh",
            19,
            5,
            "President Hồ Chí Minh's Birthday",
            "commemorative",
            true,
        ),
        (
            "Ngày Thương Binh - Liệt Sĩ",
            27,
            7,
            "War Invalids and Martyrs Day",
            "commemorative",
            true,
        ),
        (
            "Ngày Cách Mạng Tháng Tám",
            19,
            8,
            "August Revolution Day",
            "commemorative",
            true,
        ),
        (
            "Ngày Truyền Thống Công An Nhân Dân",
            19,
            8,
            "People's Public Security Traditional Day",
            "commemorative",
            true,
        ),
        (
            "Ngày Giải Phóng Thủ Đô",
            10,
            10,
            "Liberation Day of the Capital",
            "commemorative",
            true,
        ),
        (
            "Ngày Thành Lập Quân Đội",
            22,
            12,
            "Vietnam People's Army Founding Day",
            "commemorative",
            true,
        ),
        // Professional / tradition days (ngày truyền thống ngành)
        (
            "Ngày Học Sinh - Sinh Viên",
            9,
            1,
            "Vietnamese Students' Day",
            "professional",
            true,
        ),
        (
            "Ngày Thầy Thuốc Việt Nam",
            27,
            2,
            "Vietnamese Doctors' Day",
            "professional",
            true,
        ),
        (
            "Ngày Thành Lập Đoàn TNCS",
            26,
            3,
            "Founding of Hồ Chí Minh Communist Youth Union",
            "professional",
            true,
        ),
        (
            "Ngày Khoa Học và Công Nghệ Việt Nam",
            18,
            5,
            "Vietnam Science and Technology Day",
            "professional",
            true,
        ),
        (
            "Ngày Quốc Tế Hạnh Phúc",
            20,
            3,
            "International Day of Happiness",
            "international",
            true,
        ),
        (
            "Ngày Sách Việt Nam",
            21,
            4,
            "Vietnamese Book and Reading Culture Day",
            "professional",
            true,
        ),
        ("Ngày Cá Tháng Tư", 1, 4, "April Fools' Day", "social", true),
        (
            "Ngày Kiến Trúc Sư Việt Nam",
            27,
            4,
            "Vietnamese Architects' Day",
            "professional",
            true,
        ),
        ("Ngày Trái Đất", 22, 4, "Earth Day", "international", true),
        (
            "Ngày Gia Đình Việt Nam",
            28,
            6,
            "Vietnamese Family Day",
            "social",
            true,
        ),
        (
            "Ngày Dân Số Thế Giới",
            11,
            7,
            "World Population Day",
            "international",
            true,
        ),
        (
            "Ngày Quốc Tế Thanh Niên",
            12,
            8,
            "International Youth Day",
            "international",
            true,
        ),
        (
            "Ngày Doanh Nhân Việt Nam",
            13,
            10,
            "Vietnamese Entrepreneurs' Day",
            "professional",
            true,
        ),
        (
            "Ngày Chuyển Đổi Số Quốc Gia",
            10,
            10,
            "National Digital Transformation Day",
            "professional",
            true,
        ),
        (
            "Ngày Quốc Tế Người Cao Tuổi",
            1,
            10,
            "International Day of Older Persons",
            "international",
            true,
        ),
        (
            "Ngày Quốc Tế Hòa Bình",
            21,
            9,
            "International Day of Peace",
            "international",
            true,
        ),
        ("Halloween", 31, 10, "Halloween", "international", true),
        (
            "Ngày Phụ Nữ Việt Nam",
            20,
            10,
            "Vietnamese Women's Day",
            "social",
            true,
        ),
        (
            "Ngày Pháp Luật Việt Nam",
            9,
            11,
            "Vietnam Law Day",
            "commemorative",
            true,
        ),
        (
            "Ngày Nhà Giáo Việt Nam",
            20,
            11,
            "Vietnamese Teachers' Day",
            "professional",
            true,
        ),
        (
            "Ngày Quốc Tế Nam Giới",
            19,
            11,
            "International Men's Day",
            "international",
            true,
        ),
        (
            "Ngày Thành Lập Hội Chữ Thập Đỏ Việt Nam",
            23,
            11,
            "Vietnam Red Cross Society Founding Day",
            "social",
            true,
        ),
        (
            "Ngày Di Sản Văn Hóa",
            23,
            11,
            "Vietnamese Cultural Heritage Day",
            "commemorative",
            true,
        ),
        (
            "Ngày Thế Giới Phòng Chống AIDS",
            1,
            12,
            "World AIDS Day",
            "international",
            true,
        ),
        (
            "Ngày Quốc Tế Người Khuyết Tật",
            3,
            12,
            "International Day of Persons with Disabilities",
            "international",
            true,
        ),
        (
            "Lễ Giáng Sinh",
            25,
            12,
            "Christmas Day",
            "international",
            true,
        ),
        (
            "Ngày Toàn Quốc Kháng Chiến",
            19,
            12,
            "National Resistance Day",
            "commemorative",
            true,
        ),
        // International days observed in Vietnam
        (
            "Ngày Quốc Tế Phụ Nữ",
            8,
            3,
            "International Women's Day",
            "international",
            true,
        ),
        (
            "Ngày Nước Thế Giới",
            22,
            3,
            "World Water Day",
            "international",
            true,
        ),
        (
            "Ngày Sức Khỏe Thế Giới",
            7,
            4,
            "World Health Day",
            "international",
            true,
        ),
        (
            "Ngày Quốc Tế Thiếu Nhi",
            1,
            6,
            "International Children's Day",
            "international",
            true,
        ),
        (
            "Ngày Xóa Mù Chữ Quốc Tế",
            8,
            9,
            "International Literacy Day",
            "international",
            true,
        ),
        (
            "Ngày Môi Trường Thế Giới",
            5,
            6,
            "World Environment Day",
            "international",
            true,
        ),
        (
            "Ngày Chiến Thắng Phát Xít",
            9,
            5,
            "Victory Day over Fascism",
            "international",
            true,
        ),
    ];

    for (name, day, month, desc, category, is_major) in national_holidays {
        holidays.push(Holiday {
            name: name.to_string(),
            description: desc.to_string(),
            lunar_date: None,
            solar_day: day,
            solar_month: month,
            solar_year,
            is_solar: true,
            category: category.to_string(),
            is_major,
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
        if let Some(h) = create_lunar_holiday(
            &format!("Mùng 1 tháng {}", month),
            1,
            month,
            solar_year,
            "First day of lunar month",
            "lunar-cycle",
            false,
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
            "lunar-cycle",
            false,
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
