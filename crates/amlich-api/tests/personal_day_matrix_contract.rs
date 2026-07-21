use amlich_api::{get_personal_day_matrix_report, BaziQuery, DateQuery};

fn sample_birth_datetime() -> BaziQuery {
    BaziQuery {
        day: 1,
        month: 1,
        year: 1990,
        hour: 9,
        minute: 30,
        time_known: None,
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
        time_known: None,
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

#[test]
fn personal_day_matrix_canonical_shape_locks_required_fields() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let value = serde_json::to_value(&report).expect("serialize");
    let obj = value.as_object().expect("object");

    let required = [
        "input",
        "tier",
        "day_person",
        "element_resonance",
        "direction_merge",
        "domain_day_boost",
    ];
    for key in &required {
        assert!(obj.contains_key(*key), "matrix report missing key: {key}");
    }

    let input = obj["input"].as_object().expect("input object");
    assert!(input.contains_key("birth"));
    assert!(input.contains_key("date"));

    let day_person = obj["day_person"].as_object().expect("day_person object");
    assert!(day_person.contains_key("day_canchi"));
    assert!(day_person.contains_key("day_master"));
    assert!(day_person.contains_key("pillars"));
}

#[test]
fn personal_day_matrix_day_person_has_four_pillars() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    assert_eq!(report.day_person.pillars.len(), 4);
    for pillar in &report.day_person.pillars {
        match pillar.pillar {
            amlich_core::PillarKind::Year
            | amlich_core::PillarKind::Month
            | amlich_core::PillarKind::Day
            | amlich_core::PillarKind::Hour => {}
        }
        assert!(!pillar.pillar_canchi.is_empty());
    }
}

#[test]
fn personal_day_matrix_element_resonance_has_five_elements() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    assert_eq!(report.element_resonance.entries.len(), 5);
}

#[test]
fn personal_day_matrix_personal_hours_has_twelve_entries() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let hours = report.personal_hours.expect("personal hours");
    assert_eq!(hours.hours.len(), 12);
    assert!(hours.hours.iter().any(|h| h.is_hoang_dao));
}

#[test]
fn personal_day_matrix_direction_merge_has_eight_directions() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let directions = report.direction_merge.expect("direction merge");
    assert_eq!(directions.entries.len(), 8);
}

#[test]
fn personal_day_matrix_domain_day_boost_has_five_domains() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let domains = report.domain_day_boost.expect("domain day boost");
    assert_eq!(domains.entries.len(), 5);
}
