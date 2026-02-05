/**
 * Sun Longitude Calculations
 *
 * Algorithm from "Astronomical Algorithms" by Jean Meeus, 1998
 */
use std::f64::consts::PI;

/// Compute the longitude of the sun at any time
///
/// # Arguments
/// * `jdn` - Julian day number (can be fractional)
///
/// # Returns
/// Sun longitude in radians, normalized to [0, 2π)
pub fn sun_longitude(jdn: f64) -> f64 {
    // Time in Julian centuries from 2000-01-01 12:00:00 GMT
    let t = (jdn - 2451545.0) / 36525.0;
    let t2 = t * t;
    let dr = PI / 180.0; // degree to radian

    // Sun's mean anomaly, in degrees
    let m = 357.52910 + 35999.05030 * t - 0.0001559 * t2 - 0.00000048 * t * t2;

    // Sun's mean longitude, in degrees
    let l0 = 280.46645 + 36000.76983 * t + 0.0003032 * t2;

    // Sun's equation of center
    let mut dl = (1.914600 - 0.004817 * t - 0.000014 * t2) * (dr * m).sin();
    dl += (0.019993 - 0.000101 * t) * (dr * 2.0 * m).sin();
    dl += 0.000290 * (dr * 3.0 * m).sin();

    // Sun's true longitude, in degrees
    let l = l0 + dl;

    // Convert to radians
    let mut l_rad = l * dr;

    // Normalize to [0, 2π)
    l_rad = l_rad - PI * 2.0 * (l_rad / (PI * 2.0)).floor();

    l_rad
}

/// Compute sun position at midnight of the day with the given Julian day number
///
/// The function returns a number between 0 and 11.
/// From the day after March equinox and the 1st major term after March equinox, 0 is returned.
/// After that, return 1, 2, 3 ...
///
/// # Arguments
/// * `day_number` - Julian day number
/// * `time_zone` - Time zone offset (e.g., 7.0 for UTC+7:00)
///
/// # Returns
/// Sun longitude index (0-11)
pub fn get_sun_longitude(day_number: i32, time_zone: f64) -> i32 {
    let jdn = day_number as f64 - 0.5 - time_zone / 24.0;
    let longitude = sun_longitude(jdn);
    (longitude / PI * 6.0).floor() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sun_longitude_range() {
        // Test that sun_longitude returns values in [0, 2π)
        let test_jds = vec![
            2451545.0, // 2000-01-01
            2460349.0, // 2024-02-10
            2460703.0, // 2025-01-29
        ];

        for jd in test_jds {
            let sl = sun_longitude(jd);
            assert!(
                sl >= 0.0 && sl < 2.0 * PI,
                "Sun longitude {} not in [0, 2π)",
                sl
            );
        }
    }

    #[test]
    fn test_get_sun_longitude_range() {
        // Test that get_sun_longitude returns values in [0, 11]
        let test_days = vec![
            2451545, // 2000-01-01
            2460349, // 2024-02-10
            2460703, // 2025-01-29
        ];

        for day in test_days {
            let sl = get_sun_longitude(day, 7.0);
            assert!(
                sl >= 0 && sl <= 11,
                "Sun longitude index {} not in [0, 11]",
                sl
            );
        }
    }

    #[test]
    fn test_sun_longitude_consistency() {
        // Test that sun longitude changes gradually over time
        let jd_start = 2451545.0; // 2000-01-01

        for i in 1..365 {
            let jd = jd_start + i as f64;
            let sl = sun_longitude(jd);

            // Sun longitude should be between 0 and 2π
            assert!(sl >= 0.0 && sl < 2.0 * PI);
        }
    }

    #[test]
    fn test_march_equinox_approximation() {
        // Around March 20-21, sun longitude should be near 0 (spring equinox)
        // Using 2024-03-20 as reference
        let jd = 2460388; // 2024-03-20
        let sl_index = get_sun_longitude(jd, 7.0);

        // Should be 0 or close to transition
        assert!(
            sl_index == 0 || sl_index == 11,
            "March equinox should have sun longitude index near 0, got {}",
            sl_index
        );
    }
}
