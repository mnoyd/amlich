use amlich_api::{get_personal_day_matrix_report, BaziQuery, DateQuery};

fn sample_birth_datetime() -> BaziQuery {
    BaziQuery {
        day: 1,
        month: 1,
        year: 1990,
        hour: 9,
        minute: 30,
        timezone: Some(7.0),
        longitude: None,
        use_solar_time: false,
        gender: Some("male".to_string()),
    }
}

fn sample_birth_date_only() -> BaziQuery {
    BaziQuery {
        hour: 0,
        minute: 0,
        ..sample_birth_datetime()
    }
}

fn sample_date() -> DateQuery {
    DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

#[test]
fn personal_day_matrix_report_exposes_datetime_surface() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");

    assert_eq!(report.tier, amlich_api::BirthDataTierDto::Datetime);
    assert_eq!(report.input.birth.year, 1990);
    assert_eq!(report.input.date.year, 2024);
    assert_eq!(report.day_person.day_canchi, "Giáp Thìn");
    assert!(report.personal_hours.is_some());
    assert!(report.direction_merge.is_some());
    assert!(report.domain_day_boost.is_some());
    assert!(report.unavailable_sections.is_empty());
}

#[test]
fn personal_day_matrix_report_marks_personal_hours_unavailable_for_date_tier() {
    let report =
        get_personal_day_matrix_report(&sample_birth_date_only(), &sample_date()).expect("report");

    assert_eq!(report.tier, amlich_api::BirthDataTierDto::Date);
    assert!(report.personal_hours.is_none());
    assert!(report.direction_merge.is_some());
    assert!(report.domain_day_boost.is_some());
    assert!(report
        .unavailable_sections
        .iter()
        .any(|section| section.section == "personal_hours"));
}

#[test]
fn personal_day_matrix_report_omits_direction_merge_without_gender() {
    let mut birth = sample_birth_datetime();
    birth.gender = None;

    let report = get_personal_day_matrix_report(&birth, &sample_date()).expect("report");

    assert_eq!(report.tier, amlich_api::BirthDataTierDto::Datetime);
    assert!(report.personal_hours.is_some());
    assert!(report.direction_merge.is_none());
    assert!(report.domain_day_boost.is_some());
}
