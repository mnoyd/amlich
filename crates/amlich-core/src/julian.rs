/**
 * Julian Day Number Calculations
 *
 * Formula from http://www.tondering.dk/claus/calendar.html
 * Based on algorithms from "Astronomical Algorithms" by Jean Meeus, 1998
 */

/// Compute the (integral) Julian day number of day dd/mm/yyyy
///
/// This is the number of days between 1/1/4713 BC (Julian calendar) and dd/mm/yyyy.
///
/// # Arguments
/// * `day` - Day of month (1-31)
/// * `month` - Month (1-12)
/// * `year` - Year
///
/// # Returns
/// Julian day number as i32
pub fn jd_from_date(day: i32, month: i32, year: i32) -> i32 {
    let a = (14 - month) / 12;
    let y = year + 4800 - a;
    let m = month + 12 * a - 3;

    let mut jd = day + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;

    // Before October 15, 1582 (Julian calendar)
    if jd < 2299161 {
        jd = day + (153 * m + 2) / 5 + 365 * y + y / 4 - 32083;
    }

    jd
}

/// Convert a Julian day number to day/month/year
///
/// # Arguments
/// * `jd` - Julian day number (integer)
///
/// # Returns
/// Tuple of (day, month, year)
pub fn jd_to_date(jd: i32) -> (i32, i32, i32) {
    let a: i32;
    let b: i32;
    let c: i32;

    if jd > 2299160 {
        // After October 5, 1582, Gregorian calendar
        a = jd + 32044;
        b = (4 * a + 3) / 146097;
        c = a - (b * 146097) / 4;
    } else {
        b = 0;
        c = jd + 32082;
    }

    let d = (4 * c + 3) / 1461;
    let e = c - (1461 * d) / 4;
    let m = (5 * e + 2) / 153;

    let day = e - (153 * m + 2) / 5 + 1;
    let month = m + 3 - 12 * (m / 10);
    let year = b * 100 + d - 4800 + m / 10;

    (day, month, year)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jd_from_date_basic() {
        // Test known Julian day numbers
        // January 1, 2000 = JD 2451545
        assert_eq!(jd_from_date(1, 1, 2000), 2451545);

        // Tết 2024: February 10, 2024 = JD 2460351
        assert_eq!(jd_from_date(10, 2, 2024), 2460351);

        // Tết 2025: January 29, 2025 = JD 2460705
        assert_eq!(jd_from_date(29, 1, 2025), 2460705);
    }

    #[test]
    fn test_jd_to_date_basic() {
        // Test reverse conversion
        let (d, m, y) = jd_to_date(2451545);
        assert_eq!((d, m, y), (1, 1, 2000));

        let (d, m, y) = jd_to_date(2460351);
        assert_eq!((d, m, y), (10, 2, 2024));

        let (d, m, y) = jd_to_date(2460705);
        assert_eq!((d, m, y), (29, 1, 2025));
    }

    #[test]
    fn test_roundtrip_conversion() {
        // Test that converting back and forth works
        let dates = vec![
            (1, 1, 2000),
            (10, 2, 2024),
            (29, 1, 2025),
            (22, 1, 2023),
            (31, 12, 1999),
            (15, 8, 1945),
            (2, 9, 1945),
        ];

        for (d, m, y) in dates {
            let jd = jd_from_date(d, m, y);
            let (d2, m2, y2) = jd_to_date(jd);
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

    #[test]
    fn test_gregorian_calendar_cutoff() {
        // October 15, 1582 is JD 2299161 (first day of Gregorian calendar)
        let jd = jd_from_date(15, 10, 1582);
        assert_eq!(jd, 2299161);

        // Days before should use Julian calendar
        let jd_before = jd_from_date(4, 10, 1582);
        assert!(jd_before < 2299161);
    }
}
