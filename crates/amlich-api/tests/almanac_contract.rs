use amlich_api::{get_day_info, DateQuery};

#[test]
fn day_info_exposes_day_fortune_contract() {
    let info = get_day_info(&DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
    })
    .expect("day info should be available");

    let fortune = info.day_fortune.expect("day_fortune should exist");
    assert_eq!(fortune.profile, "baseline");
    assert!(!fortune.day_element.na_am.is_empty());
    assert!(!fortune.conflict.tuoi_xung.is_empty());
    assert!(!fortune.stars.cat_tinh.is_empty());
}
