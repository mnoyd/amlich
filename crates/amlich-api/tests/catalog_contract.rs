use amlich_api::{get_day_info, get_recommendation_pack_catalog, get_ruleset_catalog, DateQuery};

fn query(day: i32, month: i32, year: i32) -> DateQuery {
    DateQuery {
        day,
        month,
        year,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

#[test]
fn ruleset_catalog_exposes_canonical_default_and_request_metadata() {
    let catalog = get_ruleset_catalog();
    assert_eq!(catalog.len(), 1);

    let entry = &catalog[0];
    assert_eq!(entry.id, "vn_baseline_v1");
    assert_eq!(entry.canonical_id, entry.id);
    assert_eq!(entry.version, "v1");
    assert_eq!(entry.profile, "baseline");
    assert_eq!(entry.region, "vn");
    assert_eq!(entry.schema_version, "ruleset-descriptor/v1");
    assert!(entry.is_default);
    assert_eq!(entry.aliases, vec!["baseline".to_string()]);
    assert_eq!(entry.defaults.tz_offset, 7.0);
    assert_eq!(entry.defaults.meridian, None);
    assert!(entry
        .source_notes
        .iter()
        .any(|note| note.family == "travel"));
}

#[test]
fn recommendation_pack_catalog_matches_runtime_activation_metadata() {
    let catalog = get_recommendation_pack_catalog();
    assert_eq!(catalog.len(), 1);

    let entry = &catalog[0];
    assert_eq!(entry.pack_id, "pack.nhi_thap_bat_tu.v1");
    assert_eq!(entry.request_field, "enabled_pack_ids");
    assert_eq!(entry.version, "v1");
    assert_eq!(entry.source_family, "nhi_thap_bat_tu");
    assert_eq!(entry.mode, "advisory");

    let mut request = query(10, 2, 2024);
    request.event_kind = Some("contract_signing".to_string());
    request.enabled_pack_ids = vec![entry.pack_id.clone()];

    let info = get_day_info(&request).expect("day info with pack");
    let contextual = info
        .contextual_recommendations
        .expect("contextual recommendations");
    let active = contextual.active_packs.first().expect("active pack");

    assert_eq!(active.pack_id, entry.pack_id);
    assert_eq!(active.version, entry.version);
    assert_eq!(active.source_family, entry.source_family);
    assert_eq!(active.mode, entry.mode);
}

#[test]
fn alias_selection_normalizes_to_catalog_canonical_id() {
    let catalog = get_ruleset_catalog();
    let baseline = catalog.first().expect("ruleset catalog entry");

    let mut request = query(10, 2, 2024);
    request.ruleset_id = Some("baseline".to_string());

    let info = get_day_info(&request).expect("alias request should succeed");
    assert_eq!(info.ruleset_id, baseline.canonical_id);
    assert_eq!(info.ruleset_version, baseline.version);
    assert_eq!(info.profile, baseline.profile);
}

#[test]
fn duplicate_and_unknown_pack_ids_fail_explicitly() {
    let mut duplicate = query(10, 2, 2024);
    duplicate.event_kind = Some("contract_signing".to_string());
    duplicate.enabled_pack_ids = vec![
        "pack.nhi_thap_bat_tu.v1".to_string(),
        "pack.nhi_thap_bat_tu.v1".to_string(),
    ];
    assert_eq!(
        get_day_info(&duplicate).expect_err("duplicate pack must fail"),
        "duplicate recommendation pack id: pack.nhi_thap_bat_tu.v1"
    );

    let mut unknown = query(10, 2, 2024);
    unknown.event_kind = Some("contract_signing".to_string());
    unknown.enabled_pack_ids = vec!["pack.unknown.v1".to_string()];
    assert_eq!(
        get_day_info(&unknown).expect_err("unknown pack must fail"),
        "unknown recommendation pack id: pack.unknown.v1"
    );
}
