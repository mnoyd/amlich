use amlich_api::{get_bazi_derived_report, BaziQuery};

fn sample_query(hour: u8, minute: u8) -> BaziQuery {
    BaziQuery {
        day: 10,
        month: 2,
        year: 2024,
        hour,
        minute,
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
