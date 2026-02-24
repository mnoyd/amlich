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
    assert_eq!(fortune.profile, "baseline");
    assert!(!fortune.day_element.na_am.is_empty());
    assert!(!fortune.conflict.tuoi_xung.is_empty());
    assert!(!fortune.stars.cat_tinh.is_empty());
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
