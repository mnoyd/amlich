/**
 * Giờ Hoàng Đạo (Auspicious Hours) Calculations
 *
 * Based on the traditional 12-Star System (Thập Nhị Kiến Trừ)
 * Each day has 12 hours, and each hour is governed by one of 12 stars.
 * The cycle starts at different hours depending on the Day's Branch (Chi).
 *
 * The 12 Stars:
 * - Good Stars (Hoàng Đạo - 6 stars): Thanh Long, Minh Đường, Kim Quỹ, Bảo Quang, Ngọc Đường, Tư Mệnh
 * - Bad Stars (Hắc Đạo - 6 stars): Thiên Hình, Chu Tước, Bạch Hổ, Thiên Lao, Nguyên Vũ, Câu Trận
 */
use crate::types::CHI;

/// Star type (good or bad)
#[derive(Debug, Clone, PartialEq)]
pub enum StarType {
    Good,
    Bad,
}

/// Information about one of the 12 stars
#[derive(Debug, Clone)]
pub struct Star {
    pub name: &'static str,
    pub star_type: StarType,
    pub description: &'static str,
}

// 12 Stars in order (Good/Bad alternating pattern with variations)
pub const TWELVE_STARS: [Star; 12] = [
    Star {
        name: "Thanh Long",
        star_type: StarType::Good,
        description: "Azure Dragon - Very auspicious",
    },
    Star {
        name: "Minh Đường",
        star_type: StarType::Good,
        description: "Bright Hall - Auspicious",
    },
    Star {
        name: "Thiên Hình",
        star_type: StarType::Bad,
        description: "Heavenly Punishment - Ominous",
    },
    Star {
        name: "Chu Tước",
        star_type: StarType::Bad,
        description: "Vermilion Bird - Ominous",
    },
    Star {
        name: "Kim Quỹ",
        star_type: StarType::Good,
        description: "Golden Coffer - Auspicious",
    },
    Star {
        name: "Bảo Quang",
        star_type: StarType::Good,
        description: "Precious Light - Auspicious",
    },
    Star {
        name: "Bạch Hổ",
        star_type: StarType::Bad,
        description: "White Tiger - Very ominous",
    },
    Star {
        name: "Ngọc Đường",
        star_type: StarType::Good,
        description: "Jade Hall - Auspicious",
    },
    Star {
        name: "Thiên Lao",
        star_type: StarType::Bad,
        description: "Heavenly Prison - Ominous",
    },
    Star {
        name: "Nguyên Vũ",
        star_type: StarType::Bad,
        description: "Black Tortoise - Ominous",
    },
    Star {
        name: "Tư Mệnh",
        star_type: StarType::Good,
        description: "Life Star - Auspicious",
    },
    Star {
        name: "Câu Trận",
        star_type: StarType::Bad,
        description: "Hook Array - Ominous",
    },
];

// Start hour offset for each Day Branch (where Thanh Long appears)
// Index: Day Branch index (0-11), Value: Hour Branch index where cycle starts
const DAY_TO_START_HOUR: [usize; 12] = [
    8,  // Tý day → Thanh Long at Thân hour
    10, // Sửu day → Thanh Long at Tuất hour
    0,  // Dần day → Thanh Long at Tý hour
    2,  // Mão day → Thanh Long at Dần hour
    4,  // Thìn day → Thanh Long at Thìn hour
    6,  // Tỵ day → Thanh Long at Ngọ hour
    8,  // Ngọ day → Thanh Long at Thân hour
    10, // Mùi day → Thanh Long at Tuất hour
    0,  // Thân day → Thanh Long at Tý hour
    2,  // Dậu day → Thanh Long at Dần hour
    4,  // Tuất day → Thanh Long at Thìn hour
    6,  // Hợi day → Thanh Long at Ngọ hour
];

/// Get hour time range from Chi index
/// Vietnamese traditional hours: Each Chi covers 2 modern hours
///
/// # Arguments
/// * `chi_index` - Hour Branch index (0-11)
///
/// # Returns
/// Time range string (e.g., "23:00-01:00")
pub fn get_hour_time_range(chi_index: usize) -> &'static str {
    const RANGES: [&str; 12] = [
        "23:00-01:00", // Tý
        "01:00-03:00", // Sửu
        "03:00-05:00", // Dần
        "05:00-07:00", // Mão
        "07:00-09:00", // Thìn
        "09:00-11:00", // Tỵ
        "11:00-13:00", // Ngọ
        "13:00-15:00", // Mùi
        "15:00-17:00", // Thân
        "17:00-19:00", // Dậu
        "19:00-21:00", // Tuất
        "21:00-23:00", // Hợi
    ];
    RANGES[chi_index % 12]
}

/// Information about a single hour
#[derive(Debug, Clone)]
pub struct HourInfo {
    pub hour_index: usize,
    pub hour_chi: String,
    pub time_range: String,
    pub star: String,
    pub star_description: String,
    pub star_type: StarType,
    pub is_good: bool,
}

/// Complete information about auspicious hours for a day
#[derive(Debug, Clone)]
pub struct GioHoangDao {
    pub day_chi_index: usize,
    pub day_chi: String,
    pub all_hours: Vec<HourInfo>,
    pub good_hours: Vec<HourInfo>,
    pub good_hour_count: usize,
    pub summary: String,
}

/// Get Auspicious Hours (Giờ Hoàng Đạo) for a given day
///
/// # Arguments
/// * `day_chi_index` - Day's Branch index (0-11)
///
/// # Returns
/// Complete hour information with stars
pub fn get_gio_hoang_dao(day_chi_index: usize) -> GioHoangDao {
    let start_hour = DAY_TO_START_HOUR[day_chi_index % 12];
    let mut hours = Vec::new();
    let mut good_hours = Vec::new();

    // Calculate star for each of the 12 hours
    for i in 0..12 {
        let hour_chi_index = i;
        let hour_chi = CHI[hour_chi_index].to_string();

        // Calculate which star governs this hour
        // The star cycle starts at startHour with Thanh Long (index 0)
        let star_index = (hour_chi_index + 12 - start_hour) % 12;
        let star = &TWELVE_STARS[star_index];

        let is_good = star.star_type == StarType::Good;

        let hour_info = HourInfo {
            hour_index: hour_chi_index,
            hour_chi: hour_chi.clone(),
            time_range: get_hour_time_range(hour_chi_index).to_string(),
            star: star.name.to_string(),
            star_description: star.description.to_string(),
            star_type: star.star_type.clone(),
            is_good,
        };

        if is_good {
            good_hours.push(hour_info.clone());
        }

        hours.push(hour_info);
    }

    let summary = good_hours
        .iter()
        .map(|h| format!("{} ({})", h.hour_chi, h.time_range))
        .collect::<Vec<_>>()
        .join(", ");

    GioHoangDao {
        day_chi_index: day_chi_index % 12,
        day_chi: CHI[day_chi_index % 12].to_string(),
        all_hours: hours,
        good_hour_count: good_hours.len(),
        good_hours,
        summary,
    }
}

/// Check if a specific hour is auspicious
///
/// # Arguments
/// * `day_chi_index` - Day's Branch index
/// * `hour_chi_index` - Hour's Branch index
///
/// # Returns
/// Hour details with star info
pub fn is_hour_auspicious(day_chi_index: usize, hour_chi_index: usize) -> HourInfo {
    let result = get_gio_hoang_dao(day_chi_index);
    result.all_hours[hour_chi_index % 12].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twelve_stars_count() {
        assert_eq!(TWELVE_STARS.len(), 12);

        // Count good and bad stars
        let good_count = TWELVE_STARS
            .iter()
            .filter(|s| s.star_type == StarType::Good)
            .count();
        let bad_count = TWELVE_STARS
            .iter()
            .filter(|s| s.star_type == StarType::Bad)
            .count();

        assert_eq!(good_count, 6, "Should have 6 good stars");
        assert_eq!(bad_count, 6, "Should have 6 bad stars");
    }

    #[test]
    fn test_first_star_is_thanh_long() {
        assert_eq!(TWELVE_STARS[0].name, "Thanh Long");
        assert_eq!(TWELVE_STARS[0].star_type, StarType::Good);
    }

    #[test]
    fn test_hour_time_ranges() {
        assert_eq!(get_hour_time_range(0), "23:00-01:00"); // Tý
        assert_eq!(get_hour_time_range(6), "11:00-13:00"); // Ngọ
        assert_eq!(get_hour_time_range(11), "21:00-23:00"); // Hợi
    }

    #[test]
    fn test_gio_hoang_dao_basic() {
        // Test Tý day (index 0)
        let result = get_gio_hoang_dao(0);

        assert_eq!(result.day_chi_index, 0);
        assert_eq!(result.day_chi, "Tý");
        assert_eq!(result.all_hours.len(), 12);
        assert_eq!(result.good_hour_count, 6);
        assert_eq!(result.good_hours.len(), 6);
    }

    #[test]
    fn test_gio_hoang_dao_ty_day() {
        // For Tý day, Thanh Long starts at Thân hour (index 8)
        let result = get_gio_hoang_dao(0);

        // Hour 8 (Thân) should have Thanh Long (good)
        assert_eq!(result.all_hours[8].star, "Thanh Long");
        assert!(result.all_hours[8].is_good);

        // Good hours: Tý, Sửu, Mão, Ngọ, Thân, Dậu
        let good: Vec<&str> = result.good_hours.iter().map(|h| h.hour_chi.as_str()).collect();
        assert!(good.contains(&"Tý"));
        assert!(good.contains(&"Sửu"));
        assert!(good.contains(&"Mão"));
        assert!(good.contains(&"Ngọ"));
        assert!(good.contains(&"Thân"));
        assert!(good.contains(&"Dậu"));
    }

    #[test]
    fn test_gio_hoang_dao_thin_day() {
        // For Thìn day (index 4), Thanh Long starts at Thìn hour (index 4)
        let result = get_gio_hoang_dao(4);

        assert_eq!(result.day_chi, "Thìn");

        // Hour 4 (Thìn) should have Thanh Long (good)
        assert_eq!(result.all_hours[4].star, "Thanh Long");
        assert!(result.all_hours[4].is_good);
    }

    #[test]
    fn test_is_hour_auspicious() {
        // Tý day, Tý hour (index 0): S=8 → star_index=(0+12-8)%12=4 → Kim Quỹ (good)
        let hour_info = is_hour_auspicious(0, 0);
        assert_eq!(hour_info.star, "Kim Quỹ");
        assert!(hour_info.is_good);

        // Tý day, Dần hour (index 2): star_index=(2+12-8)%12=6 → Bạch Hổ (bad)
        let hour_info2 = is_hour_auspicious(0, 2);
        assert_eq!(hour_info2.star, "Bạch Hổ");
        assert!(!hour_info2.is_good);

        // Thìn day, Thân hour (index 8): S=4 → star_index=(8+12-4)%12=4 → Kim Quỹ (good)
        let hour_info3 = is_hour_auspicious(4, 8);
        assert!(hour_info3.is_good);
    }

    #[test]
    fn test_good_hours_summary() {
        let result = get_gio_hoang_dao(0);

        // Summary should contain good hours
        assert!(!result.summary.is_empty());
        assert!(result.summary.contains("Tý") || result.summary.contains(":"));
    }

    #[test]
    fn test_all_days_have_6_good_hours() {
        // Every day should have exactly 6 good hours
        for day_chi in 0..12 {
            let result = get_gio_hoang_dao(day_chi);
            assert_eq!(
                result.good_hour_count, 6,
                "Day {} should have 6 good hours",
                day_chi
            );
        }
    }
}
