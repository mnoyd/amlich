use crate::sun::get_sun_longitude;
/**
 * Lunar Calendar Conversion
 *
 * Algorithms from "Astronomical Algorithms" by Jean Meeus, 1998
 * Based on implementation by Ho Ngoc Duc
 */
use std::f64::consts::PI;

/// Compute the time of the k-th new moon after the new moon of 1/1/1900 13:52 UCT
///
/// Measured as the number of days since 1/1/4713 BC noon UCT.
/// For example, 2415079.9758617813 for k=2 or 2414961.935157746 for k=-2
///
/// # Arguments
/// * `k` - Number of new moons after 1/1/1900 13:52 UCT
///
/// # Returns
/// Julian day number (floating point)
pub fn new_moon(k: i32) -> f64 {
    // Time in Julian centuries from 1900 January 0.5
    let t = k as f64 / 1236.85;
    let t2 = t * t;
    let t3 = t2 * t;
    let dr = PI / 180.0;

    // Mean new moon
    let mut jd1 = 2415020.75933 + 29.53058868 * k as f64 + 0.0001178 * t2 - 0.000000155 * t3;
    jd1 += 0.00033 * ((166.56 + 132.87 * t - 0.009173 * t2) * dr).sin();

    // Sun's mean anomaly
    let m = 359.2242 + 29.10535608 * k as f64 - 0.0000333 * t2 - 0.00000347 * t3;

    // Moon's mean anomaly
    let mpr = 306.0253 + 385.81691806 * k as f64 + 0.0107306 * t2 + 0.00001236 * t3;

    // Moon's argument of latitude
    let f = 21.2964 + 390.67050646 * k as f64 - 0.0016528 * t2 - 0.00000239 * t3;

    // Corrections
    let mut c1 = (0.1734 - 0.000393 * t) * (m * dr).sin() + 0.0021 * (2.0 * dr * m).sin();
    c1 -= 0.4068 * (mpr * dr).sin() + 0.0161 * (dr * 2.0 * mpr).sin();
    c1 -= 0.0004 * (dr * 3.0 * mpr).sin();
    c1 += 0.0104 * (dr * 2.0 * f).sin() - 0.0051 * (dr * (m + mpr)).sin();
    c1 -= 0.0074 * (dr * (m - mpr)).sin() + 0.0004 * (dr * (2.0 * f + m)).sin();
    c1 -= 0.0004 * (dr * (2.0 * f - m)).sin() - 0.0006 * (dr * (2.0 * f + mpr)).sin();
    c1 += 0.0010 * (dr * (2.0 * f - mpr)).sin() + 0.0005 * (dr * (2.0 * mpr + m)).sin();

    let deltat = if t < -11.0 {
        0.001 + 0.000839 * t + 0.0002261 * t2 - 0.00000845 * t3 - 0.000000081 * t * t3
    } else {
        -0.000278 + 0.000265 * t + 0.000262 * t2
    };

    jd1 + c1 - deltat
}

/// Compute the day of the k-th new moon in the given time zone
///
/// # Arguments
/// * `k` - New moon index
/// * `time_zone` - Time zone offset (7.0 for UTC+7:00)
///
/// # Returns
/// Julian day number (integer)
pub fn get_new_moon_day(k: i32, time_zone: f64) -> i32 {
    (new_moon(k) + 0.5 + time_zone / 24.0).floor() as i32
}

/// Find the day that starts the lunar month 11 of the given year
///
/// # Arguments
/// * `year` - Solar year
/// * `time_zone` - Time zone offset (7.0 for UTC+7:00)
///
/// # Returns
/// Julian day number of lunar month 11
pub fn get_lunar_month11(year: i32, time_zone: f64) -> i32 {
    use crate::julian::jd_from_date;

    let off = jd_from_date(31, 12, year) - 2415021;
    let k = (off as f64 / 29.530588853).floor() as i32;
    let mut nm = get_new_moon_day(k, time_zone);
    let sun_long = get_sun_longitude(nm, time_zone);

    if sun_long >= 9 {
        nm = get_new_moon_day(k - 1, time_zone);
    }

    nm
}

/// Find the index of the leap month after the month starting on day a11
///
/// # Arguments
/// * `a11` - Julian day of lunar month 11
/// * `time_zone` - Time zone offset (7.0 for UTC+7:00)
///
/// # Returns
/// Leap month offset (1-based)
pub fn get_leap_month_offset(a11: i32, time_zone: f64) -> i32 {
    let k = ((a11 as f64 - 2415021.076998695) / 29.530588853 + 0.5).floor() as i32;
    let mut last;
    let mut i = 1; // Start with the month following lunar month 11
    let mut arc = get_sun_longitude(get_new_moon_day(k + i, time_zone), time_zone);

    loop {
        last = arc;
        i += 1;
        arc = get_sun_longitude(get_new_moon_day(k + i, time_zone), time_zone);
        if !(arc != last && i < 14) {
            break;
        }
    }

    i - 1
}

/// Lunar date representation
#[derive(Debug, Clone, PartialEq)]
pub struct LunarDate {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub is_leap: bool,
}

/// Convert solar date to lunar date
///
/// # Arguments
/// * `day` - Solar day (1-31)
/// * `month` - Solar month (1-12)
/// * `year` - Solar year
/// * `time_zone` - Time zone offset (7.0 for UTC+7:00)
///
/// # Returns
/// Lunar date
pub fn convert_solar_to_lunar(day: i32, month: i32, year: i32, time_zone: f64) -> LunarDate {
    use crate::julian::jd_from_date;

    let day_number = jd_from_date(day, month, year);
    let k = ((day_number as f64 - 2415021.076998695) / 29.530588853).floor() as i32;
    let mut month_start = get_new_moon_day(k + 1, time_zone);

    if month_start > day_number {
        month_start = get_new_moon_day(k, time_zone);
    }

    let mut a11 = get_lunar_month11(year, time_zone);
    let mut b11 = a11;

    let mut lunar_year: i32;
    if a11 >= month_start {
        lunar_year = year;
        a11 = get_lunar_month11(year - 1, time_zone);
    } else {
        lunar_year = year + 1;
        b11 = get_lunar_month11(year + 1, time_zone);
    }

    let lunar_day = day_number - month_start + 1;
    let diff = ((month_start - a11) as f64 / 29.0).floor() as i32;
    let mut lunar_leap = false;
    let mut lunar_month = diff + 11;

    if b11 - a11 > 365 {
        let leap_month_diff = get_leap_month_offset(a11, time_zone);
        if diff >= leap_month_diff {
            lunar_month = diff + 10;
            if diff == leap_month_diff {
                lunar_leap = true;
            }
        }
    }

    if lunar_month > 12 {
        lunar_month -= 12;
    }

    if lunar_month >= 11 && diff < 4 {
        lunar_year -= 1;
    }

    LunarDate {
        day: lunar_day,
        month: lunar_month,
        year: lunar_year,
        is_leap: lunar_leap,
    }
}

/// Convert lunar date to solar date
///
/// # Arguments
/// * `lunar_day` - Lunar day (1-30)
/// * `lunar_month` - Lunar month (1-12)
/// * `lunar_year` - Lunar year
/// * `lunar_leap` - Is leap month
/// * `time_zone` - Time zone offset (7.0 for UTC+7:00)
///
/// # Returns
/// Tuple of (day, month, year) or (0, 0, 0) if invalid
pub fn convert_lunar_to_solar(
    lunar_day: i32,
    lunar_month: i32,
    lunar_year: i32,
    lunar_leap: bool,
    time_zone: f64,
) -> (i32, i32, i32) {
    use crate::julian::jd_to_date;

    let a11: i32;
    let b11: i32;

    if lunar_month < 11 {
        a11 = get_lunar_month11(lunar_year - 1, time_zone);
        b11 = get_lunar_month11(lunar_year, time_zone);
    } else {
        a11 = get_lunar_month11(lunar_year, time_zone);
        b11 = get_lunar_month11(lunar_year + 1, time_zone);
    }

    let k = (0.5 + (a11 as f64 - 2415021.076998695) / 29.530588853).floor() as i32;
    let mut off = lunar_month - 11;

    if off < 0 {
        off += 12;
    }

    if b11 - a11 > 365 {
        let leap_off = get_leap_month_offset(a11, time_zone);
        let mut leap_month = leap_off - 2;

        if leap_month < 0 {
            leap_month += 12;
        }

        if lunar_leap && lunar_month != leap_month {
            return (0, 0, 0);
        } else if lunar_leap || off >= leap_off {
            off += 1;
        }
    }

    let month_start = get_new_moon_day(k + off, time_zone);
    jd_to_date(month_start + lunar_day - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_moon_calculation() {
        // Test that new_moon returns reasonable values
        // New moon k=0 corresponds to a date around 1900
        let nm = new_moon(0);
        assert!(nm > 2415000.0 && nm < 2416000.0, "new_moon(0) = {}", nm);

        // Test a few more values
        let nm2 = new_moon(1);
        assert!(nm2 > nm, "new_moon should increase with k");
    }

    #[test]
    fn test_solar_to_lunar_tet_2024() {
        // Tết 2024: February 10, 2024 = 1/1/2024 lunar
        let lunar = convert_solar_to_lunar(10, 2, 2024, 7.0);
        assert_eq!(lunar.day, 1);
        assert_eq!(lunar.month, 1);
        assert_eq!(lunar.year, 2024);
        assert!(!lunar.is_leap);
    }

    #[test]
    fn test_solar_to_lunar_tet_2025() {
        // Tết 2025: January 29, 2025 = 1/1/2025 lunar
        let lunar = convert_solar_to_lunar(29, 1, 2025, 7.0);
        assert_eq!(lunar.day, 1);
        assert_eq!(lunar.month, 1);
        assert_eq!(lunar.year, 2025);
        assert!(!lunar.is_leap);
    }

    #[test]
    fn test_solar_to_lunar_tet_2023() {
        // Tết 2023: January 22, 2023 = 1/1/2023 lunar
        let lunar = convert_solar_to_lunar(22, 1, 2023, 7.0);
        assert_eq!(lunar.day, 1);
        assert_eq!(lunar.month, 1);
        assert_eq!(lunar.year, 2023);
        assert!(!lunar.is_leap);
    }

    #[test]
    fn test_solar_to_lunar_y2k() {
        // Y2K: January 1, 2000 = 25/11/1999 lunar
        let lunar = convert_solar_to_lunar(1, 1, 2000, 7.0);
        assert_eq!(lunar.day, 25);
        assert_eq!(lunar.month, 11);
        assert_eq!(lunar.year, 1999);
    }

    #[test]
    fn test_lunar_to_solar_tet_2024() {
        // 1/1/2024 lunar = February 10, 2024
        let (d, m, y) = convert_lunar_to_solar(1, 1, 2024, false, 7.0);
        assert_eq!((d, m, y), (10, 2, 2024));
    }

    #[test]
    fn test_lunar_to_solar_tet_2025() {
        // 1/1/2025 lunar = January 29, 2025
        let (d, m, y) = convert_lunar_to_solar(1, 1, 2025, false, 7.0);
        assert_eq!((d, m, y), (29, 1, 2025));
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Test that converting back and forth works
        let test_dates = vec![
            (10, 2, 2024), // Tết 2024
            (29, 1, 2025), // Tết 2025
            (1, 1, 2000),  // Y2K
            (15, 8, 2024), // Random date
        ];

        for (d, m, y) in test_dates {
            let lunar = convert_solar_to_lunar(d, m, y, 7.0);
            let (d2, m2, y2) =
                convert_lunar_to_solar(lunar.day, lunar.month, lunar.year, lunar.is_leap, 7.0);
            assert_eq!(
                (d, m, y),
                (d2, m2, y2),
                "Roundtrip failed for {}/{}/{}",
                d,
                m,
                y
            );
        }
    }
}
