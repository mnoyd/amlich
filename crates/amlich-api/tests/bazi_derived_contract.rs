use amlich_api::{get_bazi_derived_report, BaziQuery};

fn sample_query(hour: u8, minute: u8) -> BaziQuery {
    BaziQuery {
        day: 10,
        month: 2,
        year: 2024,
        hour,
        minute,
        time_known: None,
        timezone: None,
        longitude: None,
        use_solar_time: false,
        gender: Some("male".to_string()),
    }
}

#[test]
fn bazi_derived_report_omits_menh_cung_without_birth_time() {
    let report = get_bazi_derived_report(&sample_query(0, 0)).expect("derived report");
    assert_eq!(report.tier, amlich_api::BirthDataTierDto::Date);
    assert!(report.menh_cung.is_none());
    assert!(!report.unavailable_sections.is_empty());
}

#[test]
fn bazi_derived_report_includes_menh_cung_with_birth_time() {
    let report = get_bazi_derived_report(&sample_query(9, 30)).expect("derived report");
    assert_eq!(report.tier, amlich_api::BirthDataTierDto::Datetime);
    assert!(report.menh_cung.is_some());
    assert!(report.unavailable_sections.is_empty());
}

/// Regression for amlich-mwbp.1: a real midnight birth (00:00 with explicit
/// `time_known: true`) must reach Datetime tier and produce mệnh cung,
/// distinct from the legacy `0/0 == unknown` sentinel path that demotes the
/// same wall-clock time to Date tier.
#[test]
fn bazi_derived_report_real_midnight_birth_is_datetime() {
    let mut query = sample_query(0, 0);
    query.time_known = Some(true);
    let report = get_bazi_derived_report(&query).expect("real-midnight derived report");
    assert_eq!(report.tier, amlich_api::BirthDataTierDto::Datetime);
    assert!(report.menh_cung.is_some());
    assert!(report.unavailable_sections.is_empty());
}

/// Companion regression: the same `0/0` wall-clock time WITHOUT the
/// explicit `time_known` override must continue to demote to Date tier
/// (backward-compatible sentinel behavior).
#[test]
fn bazi_derived_report_legacy_midnight_sentinel_still_demotes() {
    let report = get_bazi_derived_report(&sample_query(0, 0)).expect("legacy derived report");
    assert_eq!(report.tier, amlich_api::BirthDataTierDto::Date);
    assert!(report.menh_cung.is_none());
}
