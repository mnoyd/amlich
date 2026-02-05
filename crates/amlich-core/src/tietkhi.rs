use crate::sun::sun_longitude;
/**
 * Tiết Khí (24 Solar Terms) Calculations
 *
 * Solar terms are based on the sun's ecliptic longitude:
 * - Each term corresponds to 15° (360° / 24)
 * - 0° = Xuân Phân (Spring Equinox)
 * - 15° = Thanh Minh
 * - ...and so on
 *
 * References:
 * - Based on astronomical calculations from Jean Meeus
 * - Traditional Vietnamese naming
 */
use std::f64::consts::PI;

/// Information about a solar term
#[derive(Debug, Clone, PartialEq)]
pub struct SolarTerm {
    pub index: usize,
    pub name: String,
    pub description: String,
    pub longitude: i32,
    pub current_longitude: f64,
    pub season: String,
}

/// Solar term definition
#[derive(Debug, Clone)]
pub struct SolarTermDef {
    pub name: &'static str,
    pub description: &'static str,
    pub longitude: i32,
}

// 24 Solar Terms in Vietnamese (starting from 0° = Xuân Phân)
pub const TIET_KHI: [SolarTermDef; 24] = [
    SolarTermDef {
        name: "Xuân Phân",
        description: "Spring Equinox (Xuân Phân)",
        longitude: 0,
    },
    SolarTermDef {
        name: "Thanh Minh",
        description: "Pure Brightness (Thanh Minh)",
        longitude: 15,
    },
    SolarTermDef {
        name: "Cốc Vũ",
        description: "Grain Rain (Cốc Vũ)",
        longitude: 30,
    },
    SolarTermDef {
        name: "Lập Hạ",
        description: "Start of Summer (Lập Hạ)",
        longitude: 45,
    },
    SolarTermDef {
        name: "Tiểu Mãn",
        description: "Grain Buds (Tiểu Mãn)",
        longitude: 60,
    },
    SolarTermDef {
        name: "Mang Chủng",
        description: "Grain in Ear (Mang Chủng)",
        longitude: 75,
    },
    SolarTermDef {
        name: "Hạ Chí",
        description: "Summer Solstice (Hạ Chí)",
        longitude: 90,
    },
    SolarTermDef {
        name: "Tiểu Thử",
        description: "Slight Heat (Tiểu Thử)",
        longitude: 105,
    },
    SolarTermDef {
        name: "Đại Thử",
        description: "Great Heat (Đại Thử)",
        longitude: 120,
    },
    SolarTermDef {
        name: "Lập Thu",
        description: "Start of Autumn (Lập Thu)",
        longitude: 135,
    },
    SolarTermDef {
        name: "Xử Thử",
        description: "End of Heat (Xử Thử)",
        longitude: 150,
    },
    SolarTermDef {
        name: "Bạch Lộ",
        description: "White Dew (Bạch Lộ)",
        longitude: 165,
    },
    SolarTermDef {
        name: "Thu Phân",
        description: "Autumn Equinox (Thu Phân)",
        longitude: 180,
    },
    SolarTermDef {
        name: "Hàn Lộ",
        description: "Cold Dew (Hàn Lộ)",
        longitude: 195,
    },
    SolarTermDef {
        name: "Sương Giáng",
        description: "Frost Descent (Sương Giáng)",
        longitude: 210,
    },
    SolarTermDef {
        name: "Lập Đông",
        description: "Start of Winter (Lập Đông)",
        longitude: 225,
    },
    SolarTermDef {
        name: "Tiểu Tuyết",
        description: "Slight Snow (Tiểu Tuyết)",
        longitude: 240,
    },
    SolarTermDef {
        name: "Đại Tuyết",
        description: "Great Snow (Đại Tuyết)",
        longitude: 255,
    },
    SolarTermDef {
        name: "Đông Chí",
        description: "Winter Solstice (Đông Chí)",
        longitude: 270,
    },
    SolarTermDef {
        name: "Tiểu Hàn",
        description: "Slight Cold (Tiểu Hàn)",
        longitude: 285,
    },
    SolarTermDef {
        name: "Đại Hàn",
        description: "Great Cold (Đại Hàn)",
        longitude: 300,
    },
    SolarTermDef {
        name: "Lập Xuân",
        description: "Start of Spring (Lập Xuân)",
        longitude: 315,
    },
    SolarTermDef {
        name: "Vũ Thủy",
        description: "Rain Water (Vũ Thủy)",
        longitude: 330,
    },
    SolarTermDef {
        name: "Kinh Trập",
        description: "Awakening of Insects (Kinh Trập)",
        longitude: 345,
    },
];

/// Get season name from term index
///
/// # Arguments
/// * `term_index` - Solar term index (0-23)
///
/// # Returns
/// Season name in Vietnamese
pub fn get_season(term_index: usize) -> &'static str {
    match term_index {
        0..=5 => "Xuân (Spring)",
        6..=11 => "Hạ (Summer)",
        12..=17 => "Thu (Autumn)",
        _ => "Đông (Winter)",
    }
}

/// Get Solar Term (Tiết Khí) for a given date
///
/// # Arguments
/// * `jd` - Julian Day Number
/// * `time_zone` - Timezone offset (default: 7 for Vietnam)
///
/// # Returns
/// Solar term information
pub fn get_tiet_khi(jd: i32, time_zone: f64) -> SolarTerm {
    // Calculate sun longitude at local midnight
    let sun_long_rad = sun_longitude(jd as f64 - 0.5 - time_zone / 24.0);

    // Convert radians to degrees
    let sun_long_deg = (sun_long_rad * 180.0 / PI) % 360.0;

    // Calculate term index (0-23)
    let term_index = (sun_long_deg / 15.0).floor() as usize;

    let term = &TIET_KHI[term_index];

    SolarTerm {
        index: term_index,
        name: term.name.to_string(),
        description: term.description.to_string(),
        longitude: term.longitude,
        current_longitude: (sun_long_deg * 100.0).round() / 100.0, // Round to 2 decimal places
        season: get_season(term_index).to_string(),
    }
}

/// Solar term with date information
#[derive(Debug, Clone)]
pub struct SolarTermWithDate {
    pub jd: i32,
    pub index: usize,
    pub name: String,
    pub description: String,
    pub longitude: i32,
    pub current_longitude: f64,
    pub season: String,
}

/// Get all solar terms for a given year
/// Useful for displaying a full year calendar
///
/// # Arguments
/// * `year` - Solar year
/// * `time_zone` - Timezone offset
///
/// # Returns
/// Vector of solar terms with dates
pub fn get_all_tiet_khi_for_year(year: i32, time_zone: f64) -> Vec<SolarTermWithDate> {
    use crate::julian::jd_from_date;

    let mut terms = Vec::new();
    let start_jd = jd_from_date(1, 1, year);
    let end_jd = jd_from_date(31, 12, year);

    let mut prev_term_index: Option<usize> = None;

    for jd in start_jd..=end_jd {
        let tiet_khi = get_tiet_khi(jd, time_zone);

        // Detect when we cross into a new term
        if Some(tiet_khi.index) != prev_term_index {
            terms.push(SolarTermWithDate {
                jd,
                index: tiet_khi.index,
                name: tiet_khi.name.clone(),
                description: tiet_khi.description.clone(),
                longitude: tiet_khi.longitude,
                current_longitude: tiet_khi.current_longitude,
                season: tiet_khi.season.clone(),
            });
            prev_term_index = Some(tiet_khi.index);
        }
    }

    terms
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::julian::jd_from_date;

    #[test]
    fn test_tiet_khi_constants() {
        assert_eq!(TIET_KHI.len(), 24);
        assert_eq!(TIET_KHI[0].name, "Xuân Phân");
        assert_eq!(TIET_KHI[0].longitude, 0);
        assert_eq!(TIET_KHI[12].name, "Thu Phân");
        assert_eq!(TIET_KHI[12].longitude, 180);
    }

    #[test]
    fn test_get_season() {
        assert_eq!(get_season(0), "Xuân (Spring)");
        assert_eq!(get_season(5), "Xuân (Spring)");
        assert_eq!(get_season(6), "Hạ (Summer)");
        assert_eq!(get_season(11), "Hạ (Summer)");
        assert_eq!(get_season(12), "Thu (Autumn)");
        assert_eq!(get_season(17), "Thu (Autumn)");
        assert_eq!(get_season(18), "Đông (Winter)");
        assert_eq!(get_season(23), "Đông (Winter)");
    }

    #[test]
    fn test_get_tiet_khi_basic() {
        // Test a random date
        let jd = jd_from_date(10, 2, 2024);
        let tiet_khi = get_tiet_khi(jd, 7.0);

        // Should return a valid term
        assert!(tiet_khi.index < 24);
        assert!(!tiet_khi.name.is_empty());
        assert!(tiet_khi.current_longitude >= 0.0 && tiet_khi.current_longitude < 360.0);
    }

    #[test]
    fn test_get_tiet_khi_march_equinox() {
        // March 21, 2024 is the spring equinox (Xuân Phân, index 0)
        let jd = jd_from_date(21, 3, 2024);
        let tiet_khi = get_tiet_khi(jd, 7.0);

        // Should be index 0 (Xuân Phân) or possibly index 1 (Thanh Minh) or 23 (just before)
        assert!(
            tiet_khi.index == 0 || tiet_khi.index == 1 || tiet_khi.index == 23,
            "Expected index 0, 1, or 23 for March 21, got {}",
            tiet_khi.index
        );
    }

    #[test]
    fn test_get_tiet_khi_summer_solstice() {
        // Around June 21 should be near Hạ Chí (index 6, longitude 90°)
        let jd = jd_from_date(21, 6, 2024);
        let tiet_khi = get_tiet_khi(jd, 7.0);

        // Should be close to index 6 (Hạ Chí)
        assert!(
            tiet_khi.index >= 5 && tiet_khi.index <= 7,
            "Expected index 6±1, got {}",
            tiet_khi.index
        );
    }

    #[test]
    fn test_get_tiet_khi_winter_solstice() {
        // Around December 21 should be near Đông Chí (index 18, longitude 270°)
        let jd = jd_from_date(21, 12, 2024);
        let tiet_khi = get_tiet_khi(jd, 7.0);

        // Should be close to index 18 (Đông Chí)
        assert!(
            tiet_khi.index >= 17 && tiet_khi.index <= 19,
            "Expected index 18±1, got {}",
            tiet_khi.index
        );
    }

    #[test]
    fn test_get_all_tiet_khi_for_year() {
        let terms = get_all_tiet_khi_for_year(2024, 7.0);

        // Should have around 24 terms (may have 23-25 depending on year boundaries)
        assert!(
            terms.len() >= 23 && terms.len() <= 25,
            "Expected 23-25 terms, got {}",
            terms.len()
        );

        // Terms should be in chronological order
        for i in 1..terms.len() {
            assert!(
                terms[i].jd > terms[i - 1].jd,
                "Terms not in chronological order"
            );
        }
    }

    #[test]
    fn test_solar_term_progression() {
        // Test that solar terms progress correctly through the year
        let jd_spring = jd_from_date(1, 4, 2024);
        let jd_summer = jd_from_date(1, 7, 2024);
        let jd_autumn = jd_from_date(1, 10, 2024);
        let jd_winter = jd_from_date(1, 1, 2024);

        let term_spring = get_tiet_khi(jd_spring, 7.0);
        let term_summer = get_tiet_khi(jd_summer, 7.0);
        let term_autumn = get_tiet_khi(jd_autumn, 7.0);
        let term_winter = get_tiet_khi(jd_winter, 7.0);

        // Verify seasons match expected months
        assert!(term_spring.season.contains("Spring") || term_spring.season.contains("Summer"));
        assert!(term_summer.season.contains("Summer"));
        assert!(term_autumn.season.contains("Autumn"));
        assert!(term_winter.season.contains("Winter") || term_winter.season.contains("Spring"));
    }
}
