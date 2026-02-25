use amlich_api::{get_day_info, DateQuery};

fn tet_2024_fortune() -> amlich_api::DayFortuneDto {
    let info = get_day_info(&DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
    })
    .expect("day info should be available");
    info.day_fortune.expect("day_fortune should exist")
}

#[test]
fn day_info_exposes_day_fortune_contract() {
    let fortune = tet_2024_fortune();
    assert_eq!(fortune.ruleset_id, "vn_baseline_v1");
    assert_eq!(fortune.ruleset_version, "v1");
    assert_eq!(fortune.profile, "baseline");
    assert!(!fortune.day_element.na_am.is_empty());
    assert!(!fortune.conflict.tuoi_xung.is_empty());
    assert!(!fortune.stars.cat_tinh.is_empty());
    let deity = fortune.day_deity.as_ref().expect("day_deity should exist");
    assert!(!deity.name.is_empty());
    assert!(matches!(
        deity.classification.as_str(),
        "hoang_dao" | "hac_dao"
    ));
    assert!(fortune.taboos.iter().all(|t| !t.rule_id.is_empty()));
}

// --- xung_hop contract ---

#[test]
fn day_fortune_exposes_xung_hop() {
    let fortune = tet_2024_fortune();
    // Tết 2024: day chi = Thìn(4), lục xung = Tuất
    assert_eq!(fortune.xung_hop.luc_xung, "Tuất");
    assert_eq!(fortune.xung_hop.tam_hop.len(), 3);
    assert_eq!(fortune.xung_hop.tu_hanh_xung.len(), 4);
    assert!(fortune.xung_hop.tam_hop.contains(&"Thìn".to_string()));
    assert!(fortune.xung_hop.tu_hanh_xung.contains(&"Thìn".to_string()));
}

// --- trực contract ---

#[test]
fn day_fortune_exposes_truc() {
    let fortune = tet_2024_fortune();
    // Tết 2024: lunar month 1, day chi Thìn(4) → trực Mãn (index 2)
    assert_eq!(fortune.truc.name, "Mãn");
    assert_eq!(fortune.truc.index, 2);
    assert_eq!(fortune.truc.quality, "hung");
}

// --- domain-level star evidence ---

#[test]
fn day_fortune_exposes_day_star_evidence() {
    let fortune = tet_2024_fortune();
    let day_star = fortune.stars.day_star.expect("day_star must be present");
    assert!(!day_star.name.is_empty());
    assert!(day_star.index < 28);
    assert!(
        matches!(day_star.quality.as_str(), "cat" | "hung" | "binh"),
        "day_star quality must be cat/hung/binh, got: {}",
        day_star.quality
    );
    assert_eq!(day_star.system, "nhi-thap-bat-tu");
}

#[test]
fn day_fortune_applies_seeded_star_precedence_at_api_layer() {
    let info = get_day_info(&DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
    })
    .expect("day info should be available");

    assert_eq!(info.canchi.day.full, "Giáp Thìn");
    assert_eq!(info.canchi.year.can, "Giáp");
    assert_eq!(info.lunar.month, 1);
    assert_eq!(info.tiet_khi.name, "Lập Xuân");

    let stars = info.day_fortune.expect("day_fortune should exist").stars;

    // Seeded overlaps in baseline.json verify precedence:
    // ByTietKhi > ByMonth > ByYear > FixedByCanChi > FixedByChi.
    assert!(stars.cat_tinh.contains(&"Bạch Hổ".to_string()));
    assert!(!stars.sat_tinh.contains(&"Bạch Hổ".to_string()));

    assert!(stars.cat_tinh.contains(&"Thiên Quý".to_string()));
    assert!(!stars.sat_tinh.contains(&"Thiên Quý".to_string()));

    assert!(stars.sat_tinh.contains(&"Phúc Sinh".to_string()));
    assert!(!stars.cat_tinh.contains(&"Phúc Sinh".to_string()));

    // Binh stars are filtered from final cat/hung lists.
    assert!(!stars.cat_tinh.contains(&"Nguyệt Không".to_string()));
    assert!(!stars.sat_tinh.contains(&"Nguyệt Không".to_string()));
}

#[test]
fn day_fortune_exposes_source_evidence_metadata() {
    let fortune = tet_2024_fortune();

    let day_element_evidence = fortune
        .day_element
        .evidence
        .expect("day_element evidence should exist");
    assert_eq!(day_element_evidence.source_id, "tam-menh-thong-hoi");
    assert_eq!(day_element_evidence.method, "table-lookup");
    assert_eq!(day_element_evidence.profile, "baseline");

    let conflict_evidence = fortune
        .conflict
        .evidence
        .expect("conflict evidence should exist");
    assert_eq!(conflict_evidence.source_id, "khcbppt");

    let travel_evidence = fortune
        .travel
        .evidence
        .expect("travel evidence should exist");
    assert_eq!(travel_evidence.source_id, "khcbppt");
    assert_eq!(travel_evidence.method, "bai-quyet");

    let stars_evidence = fortune.stars.evidence.expect("stars evidence should exist");
    assert_eq!(stars_evidence.source_id, "khcbppt");

    let day_star = fortune.stars.day_star.expect("day_star must be present");
    let day_star_evidence = day_star.evidence.expect("day_star evidence should exist");
    assert_eq!(day_star_evidence.source_id, "nhi-thap-bat-tu");
    assert_eq!(day_star_evidence.method, "jd-cycle");

    let truc_evidence = fortune.truc.evidence.expect("truc evidence should exist");
    assert_eq!(truc_evidence.source_id, "formula");

    let deity = fortune.day_deity.as_ref().expect("day_deity should exist");
    let deity_evidence = deity
        .evidence
        .as_ref()
        .expect("day_deity evidence should exist");
    assert_eq!(deity_evidence.source_id, "khcbppt");
    assert_eq!(deity_evidence.method, "table-lookup");
    assert_eq!(deity_evidence.profile, "baseline");

    assert!(
        !fortune.stars.matched_rules.is_empty(),
        "matched_rules evidence should be populated"
    );
    assert!(fortune
        .stars
        .matched_rules
        .iter()
        .any(|r| r.category == "by_tiet_khi" && r.name == "Bạch Hổ"));

    if let Some(taboo) = fortune.taboos.first() {
        let taboo_evidence = taboo
            .evidence
            .as_ref()
            .expect("taboo evidence should exist");
        assert_eq!(taboo_evidence.method, "table-lookup");
        assert_eq!(taboo_evidence.profile, "baseline");
        assert!(!taboo.reason.is_empty());
    }
}

#[test]
fn day_fortune_exposes_structured_taboos() {
    let info = get_day_info(&DateQuery {
        day: 14,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
    })
    .expect("day info should be available");

    let fortune = info.day_fortune.expect("day_fortune should exist");
    assert!(
        !fortune.taboos.is_empty(),
        "expected taboo hits on selected day"
    );
    assert!(fortune
        .taboos
        .iter()
        .any(|t| t.rule_id == "nguyet_ky" && t.severity == "hard"));
    assert!(fortune
        .taboos
        .iter()
        .all(|t| !t.name.is_empty() && !t.reason.is_empty()));
}

#[test]
fn day_info_exposes_ruleset_metadata() {
    let info = get_day_info(&DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
    })
    .expect("day info should be available");

    assert_eq!(info.ruleset_id, "vn_baseline_v1");
    assert_eq!(info.ruleset_version, "v1");
    let fortune = info.day_fortune.expect("day_fortune should exist");
    assert_eq!(fortune.ruleset_id, info.ruleset_id);
    assert_eq!(fortune.ruleset_version, info.ruleset_version);
}
