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
    assert!(report.direction_assessment.is_some());
    assert!(report.domain_day_boost.is_some());
    assert!(report.unavailable_sections.is_empty());
}

#[test]
fn direction_assessment_has_its_own_axes_and_explicit_missing_location() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let direction = report.direction_assessment.expect("direction assessment");

    assert_eq!(direction.entries.len(), 8);
    assert!(direction.entries.iter().all(|entry| {
        entry.axes.directional_constraints.score.is_some()
            && entry.axes.flying_star_overlay.score.is_none()
    }));
    assert!(direction
        .unavailable_sections
        .iter()
        .any(|warning| warning.code == "location_unavailable"));
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
    // amlich-mwbp.5 regression: domain_day_boost must also be None when
    // gender is missing (previously emitted a silent-zero Hạn penalty).
    assert!(
        report.domain_day_boost.is_none(),
        "domain_day_boost must be None when gender is missing"
    );
    assert!(report
        .unavailable_sections
        .iter()
        .any(|section| section.section == "domain_day_boost"));
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
        "direction_assessment",
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

/// Regression for amlich-mwbp.5: domain_day_boost.day_canchi must
/// serialize the canonical day Can Chi label ("Bính Thân" / "Giáp Thìn"
/// / etc.), not the ngũ hành element names ("Mộc Thổ") that the previous
/// implementation derived from day_fortune.day_element. End-to-end check
/// across the matrix builder → API DTO hop.
#[test]
fn personal_day_matrix_domain_day_boost_serializes_real_can_chi() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let boost = report.domain_day_boost.expect("domain_day_boost present");
    // The sample date 2024-02-10 has day_canchi = "Giáp Thìn" on the
    // day_person surface; the domain_day_boost.day_canchi must agree.
    assert_eq!(
        boost.day_canchi, report.day_person.day_canchi,
        "domain_day_boost.day_canchi must match the canonical day Can Chi"
    );
    // Negative regression: must not contain element names.
    for element in ["Mộc", "Hỏa", "Thổ", "Kim", "Thủy"] {
        assert!(
            !boost.day_canchi.contains(element),
            "domain_day_boost.day_canchi must not contain element name {element}; got {}",
            boost.day_canchi
        );
    }
}

/// Regression for amlich-mwbp.5: with full birth profile (gender
/// present), domain_day_boost must still be Some and advertise its
/// availability. This guards against accidentally flipping the gender
/// gate to "always None".
#[test]
fn personal_day_matrix_domain_day_boost_available_with_full_profile() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    assert!(
        report.domain_day_boost.is_some(),
        "domain_day_boost must be Some when gender is supplied"
    );
    assert!(
        !report
            .unavailable_sections
            .iter()
            .any(|section| section.section == "domain_day_boost"),
        "domain_day_boost must NOT be in unavailable_sections when gender is supplied"
    );
}

/// Regression for amlich-mwbp.2: the first personal-hour row exposed by
/// the API must be Tý (23:00-01:00) with a Can Chi whose chi is "Tý",
/// and its star must be the same Hoàng Đạo star the row's chi_index
/// implies. End-to-end check across the core builder → API DTO hop.
#[test]
fn personal_day_matrix_personal_hours_slot_0_is_ty() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let hours = report.personal_hours.expect("personal hours present");
    let row = &hours.hours[0];
    assert_eq!(row.chi, "Tý", "API slot 0 chi");
    assert_eq!(row.chi_index, 0, "API slot 0 chi_index");
    assert_eq!(row.time_range, "23:00-01:00", "API slot 0 time range");
    assert!(
        row.canchi.ends_with("Tý"),
        "API slot 0 canchi must end with 'Tý'; got {}",
        row.canchi
    );
}

/// Regression for amlich-mwbp.2: every personal-hour row in the API
/// response must carry matching chi_index → chi label → time_range →
/// star (no off-by-one between any of them).
#[test]
fn personal_day_matrix_personal_hours_rows_align_index_chi_time_star() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let hours = report.personal_hours.expect("personal hours present");
    let day_canchi: amlich_core::CanChi = amlich_core::CanChi::new(0, 0); // unused; we only need the Hoàng Đạo table for star parity
    let _ = day_canchi;
    // The matrix report does not expose the day_canchi chi_index directly,
    // so we re-derive the expected Hoàng Đạo layout from the day-person
    // surface's day_canchi string ("Giáp Thìn" → chi_index 4 for Thìn).
    let day_chi_index = report
        .day_person
        .day_canchi
        .split_whitespace()
        .nth(1)
        .and_then(|chi| match chi {
            "Tý" => Some(0),
            "Sửu" => Some(1),
            "Dần" => Some(2),
            "Mão" => Some(3),
            "Thìn" => Some(4),
            "Tỵ" => Some(5),
            "Ngọ" => Some(6),
            "Mùi" => Some(7),
            "Thân" => Some(8),
            "Dậu" => Some(9),
            "Tuất" => Some(10),
            "Hợi" => Some(11),
            _ => None,
        })
        .expect("day_canchi chi must be a known branch");
    let hoang_dao = amlich_core::gio_hoang_dao::get_gio_hoang_dao(day_chi_index);
    for (slot, row) in hours.hours.iter().enumerate() {
        assert_eq!(
            row.chi_index, slot,
            "API slot {slot} chi_index must equal slot"
        );
        assert_eq!(
            &row.star_name, &hoang_dao.all_hours[slot].star,
            "API slot {slot} star must align with chi_index"
        );
    }
}
