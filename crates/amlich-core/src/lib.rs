// amlich-core - Vietnamese Lunar Calendar Core Library
//
// This library provides comprehensive Vietnamese lunar calendar calculations including:
// - Solar ↔ Lunar date conversion
// - Can Chi (Heavenly Stems & Earthly Branches) calculations
// - Tiết Khí (24 Solar Terms)
// - Giờ Hoàng Đạo (Auspicious Hours)
// - Vietnamese holidays and festivals

pub mod canchi;
pub mod gio_hoang_dao;
pub mod holidays;
pub mod julian;
pub mod lunar;
pub mod sun;
pub mod tietkhi;
pub mod types;

// Re-export main types
pub use types::*;

use canchi::{get_day_canchi, get_month_canchi, get_year_canchi};
use gio_hoang_dao::{get_gio_hoang_dao, GioHoangDao, HourInfo};
use julian::jd_from_date;
use lunar::{convert_solar_to_lunar, LunarDate};
use tietkhi::{get_tiet_khi, SolarTerm};

/// Solar date information
#[derive(Debug, Clone)]
pub struct SolarInfo {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub day_of_week: usize,
    pub day_of_week_name: String,
    pub date_string: String,
}

/// Lunar date information
#[derive(Debug, Clone)]
pub struct LunarInfo {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub is_leap_month: bool,
    pub date_string: String,
}

/// Can Chi information for day, month, and year
#[derive(Debug, Clone)]
pub struct CanChiInfo {
    pub day: CanChi,
    pub month: CanChi,
    pub year: CanChi,
    pub full: String,
}

/// Complete information about a day
#[derive(Debug, Clone)]
pub struct DayInfo {
    pub solar: SolarInfo,
    pub lunar: LunarInfo,
    pub jd: i32,
    pub canchi: CanChiInfo,
    pub tiet_khi: SolarTerm,
    pub gio_hoang_dao: GioHoangDao,
}

/// Get comprehensive information for a given solar date
///
/// # Arguments
/// * `day` - Day (1-31)
/// * `month` - Month (1-12)
/// * `year` - Year
///
/// # Returns
/// Complete day information including solar, lunar, Can Chi, solar terms, and auspicious hours
///
/// # Example
/// ```
/// use amlich_core::get_day_info;
///
/// // Get info for Tết 2024 (February 10, 2024)
/// let info = get_day_info(10, 2, 2024);
/// println!("Lunar date: {}/{}/{}", info.lunar.day, info.lunar.month, info.lunar.year);
/// println!("Day Can Chi: {}", info.canchi.day.full);
/// ```
pub fn get_day_info(day: i32, month: i32, year: i32) -> DayInfo {
    get_day_info_with_timezone(day, month, year, 7.0)
}

/// Get comprehensive information for a given solar date with custom timezone
///
/// # Arguments
/// * `day` - Day (1-31)
/// * `month` - Month (1-12)
/// * `year` - Year
/// * `time_zone` - Timezone offset (default: 7.0 for Vietnam UTC+7)
///
/// # Returns
/// Complete day information
pub fn get_day_info_with_timezone(day: i32, month: i32, year: i32, time_zone: f64) -> DayInfo {
    // Calculate Julian Day Number
    let jd = jd_from_date(day, month, year);

    // Convert to lunar date
    let lunar_date = convert_solar_to_lunar(day, month, year, time_zone);

    // Calculate day of week (JD + 1 because JD 0 was Monday)
    let day_of_week = ((jd + 1) % 7) as usize;

    // Calculate Can Chi for day, month, year
    let day_canchi = get_day_canchi(jd);
    let month_canchi = get_month_canchi(lunar_date.month, lunar_date.year, lunar_date.is_leap);
    let year_canchi = get_year_canchi(lunar_date.year);

    // Calculate Solar Term (Tiết Khí)
    let tiet_khi = get_tiet_khi(jd, time_zone);

    // Calculate Auspicious Hours (Giờ Hoàng Đạo)
    let gio_hoang_dao = get_gio_hoang_dao(day_canchi.chi_index);

    // Build solar info
    let solar = SolarInfo {
        day,
        month,
        year,
        day_of_week,
        day_of_week_name: THU[day_of_week].to_string(),
        date_string: format!("{}-{:02}-{:02}", year, month, day),
    };

    // Build lunar info
    let lunar = LunarInfo {
        day: lunar_date.day,
        month: lunar_date.month,
        year: lunar_date.year,
        is_leap_month: lunar_date.is_leap,
        date_string: format!(
            "{}/{}/{}{}",
            lunar_date.day,
            lunar_date.month,
            lunar_date.year,
            if lunar_date.is_leap { " (nhuận)" } else { "" }
        ),
    };

    // Build Can Chi info
    let canchi = CanChiInfo {
        day: day_canchi.clone(),
        month: month_canchi.clone(),
        year: year_canchi.clone(),
        full: format!(
            "{}, tháng {}, năm {}",
            day_canchi.full, month_canchi.full, year_canchi.full
        ),
    };

    DayInfo {
        solar,
        lunar,
        jd,
        canchi,
        tiet_khi,
        gio_hoang_dao,
    }
}

/// Format day info as a readable string
///
/// # Arguments
/// * `day_info` - DayInfo struct from get_day_info()
///
/// # Returns
/// Formatted multi-line string
pub fn format_day_info(day_info: &DayInfo) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "📅 Ngày {} ({})",
        day_info.solar.date_string, day_info.solar.day_of_week_name
    ));
    lines.push(format!("🌙 Âm lịch: {}", day_info.lunar.date_string));
    lines.push("📜 Can Chi:".to_string());
    lines.push(format!(
        "   • Ngày: {} ({})",
        day_info.canchi.day.full, day_info.canchi.day.con_giap
    ));
    lines.push(format!("   • Tháng: {}", day_info.canchi.month.full));
    lines.push(format!(
        "   • Năm: {} ({})",
        day_info.canchi.year.full, day_info.canchi.year.con_giap
    ));
    lines.push("🌟 Ngũ hành:".to_string());
    lines.push(format!(
        "   • Ngày: {} (Can) - {} (Chi)",
        day_info.canchi.day.ngu_hanh.can, day_info.canchi.day.ngu_hanh.chi
    ));
    lines.push(format!(
        "🌤️  Tiết khí: {} - {}",
        day_info.tiet_khi.name, day_info.tiet_khi.season
    ));
    lines.push(format!("   • {}", day_info.tiet_khi.description));
    lines.push(format!(
        "   • Kinh độ mặt trời: {}°",
        day_info.tiet_khi.current_longitude
    ));
    lines.push(format!(
        "⏰ Giờ Hoàng Đạo ({} giờ tốt):",
        day_info.gio_hoang_dao.good_hour_count
    ));
    for h in &day_info.gio_hoang_dao.good_hours {
        lines.push(format!(
            "   • {} ({}) - {}",
            h.hour_chi, h.time_range, h.star
        ));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_day_info_tet_2024() {
        // Tết 2024: February 10, 2024
        let info = get_day_info(10, 2, 2024);

        // Check solar date
        assert_eq!(info.solar.day, 10);
        assert_eq!(info.solar.month, 2);
        assert_eq!(info.solar.year, 2024);

        // Check lunar date (should be 1/1/2024)
        assert_eq!(info.lunar.day, 1);
        assert_eq!(info.lunar.month, 1);
        assert_eq!(info.lunar.year, 2024);
        assert!(!info.lunar.is_leap_month);

        // Check Can Chi
        assert_eq!(info.canchi.day.full, "Giáp Thìn");
        assert_eq!(info.canchi.year.full, "Giáp Thìn");
    }

    #[test]
    fn test_get_day_info_tet_2025() {
        // Tết 2025: January 29, 2025
        let info = get_day_info(29, 1, 2025);

        // Check lunar date (should be 1/1/2025)
        assert_eq!(info.lunar.day, 1);
        assert_eq!(info.lunar.month, 1);
        assert_eq!(info.lunar.year, 2025);

        // Check Can Chi
        assert_eq!(info.canchi.day.full, "Mậu Tuất");
        assert_eq!(info.canchi.year.full, "Ất Tỵ");
    }

    #[test]
    fn test_day_of_week() {
        // Test a known day: January 1, 2000 was a Saturday (index 6)
        let info = get_day_info(1, 1, 2000);
        assert_eq!(info.solar.day_of_week, 6);
        assert_eq!(info.solar.day_of_week_name, "Thứ Bảy");
    }

    #[test]
    fn test_gio_hoang_dao_present() {
        let info = get_day_info(10, 2, 2024);

        // Should have 6 good hours
        assert_eq!(info.gio_hoang_dao.good_hour_count, 6);
        assert_eq!(info.gio_hoang_dao.good_hours.len(), 6);
    }

    #[test]
    fn test_format_day_info() {
        let info = get_day_info(10, 2, 2024);
        let formatted = format_day_info(&info);

        // Should contain key information
        assert!(formatted.contains("2024-02-10"));
        assert!(formatted.contains("1/1/2024"));
        assert!(formatted.contains("Giáp Thìn"));
        assert!(formatted.contains("Giờ Hoàng Đạo"));
    }

    #[test]
    fn test_custom_timezone() {
        // Test with different timezone (should work but give potentially different results)
        let info = get_day_info_with_timezone(10, 2, 2024, 8.0);

        // Should still work
        assert_eq!(info.solar.day, 10);
        assert_eq!(info.solar.month, 2);
        assert_eq!(info.solar.year, 2024);
    }
}
