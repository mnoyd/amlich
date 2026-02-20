use amlich_api::{get_day_info, get_day_insight, DateQuery};

#[test]
fn insight_datasets_have_expected_sizes() {
    assert_eq!(amlich_core::insight_data::all_can().len(), 10);
    assert_eq!(amlich_core::insight_data::all_chi().len(), 12);
    assert_eq!(amlich_core::insight_data::all_elements().len(), 5);
    assert_eq!(amlich_core::insight_data::all_day_guidance().len(), 12);
    assert_eq!(amlich_core::insight_data::all_tiet_khi_insights().len(), 24);
}

#[test]
fn tiet_khi_records_have_required_fields() {
    for term in amlich_core::insight_data::all_tiet_khi_insights() {
        assert!(!term.name.vi.trim().is_empty());
        assert!(!term.name.en.trim().is_empty());
        assert!(!term.meaning.vi.trim().is_empty());
        assert!(!term.astronomy.vi.trim().is_empty());
        assert!(!term.agriculture.vi.is_empty());
        assert!(!term.health.vi.is_empty());
        assert!(!term.weather.vi.trim().is_empty());
    }
}

#[test]
fn tet_2024_has_festival_and_guidance_insight() {
    let insight = get_day_insight(&DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: None,
    })
    .expect("day insight should be available");

    let festival = insight.festival.expect("Tết should match festival insight");
    assert!(festival.names.vi.iter().any(|n| n.contains("Tết")));
    assert!(festival.origin.is_some());
    assert!(!festival.food.is_empty());
    assert!(insight.day_guidance.is_some());
    assert!(insight.tiet_khi.is_some());
}

#[test]
fn teachers_day_has_holiday_insight_fields() {
    let insight = get_day_insight(&DateQuery {
        day: 20,
        month: 11,
        year: 2024,
        timezone: None,
    })
    .expect("day insight should be available");

    let holiday = insight
        .holiday
        .expect("Teachers' day should match holiday insight");
    assert!(holiday
        .names
        .vi
        .iter()
        .any(|n| n.contains("Ngày Nhà Giáo Việt Nam")));
    assert!(holiday.significance.is_some());
    assert!(holiday.traditions.is_some());
    assert!(!holiday.proverbs.is_empty());
    assert!(holiday.regions.is_some());
}

#[test]
fn day_info_and_day_insight_remain_consistent() {
    let query = DateQuery {
        day: 29,
        month: 1,
        year: 2025,
        timezone: None,
    };

    let info = get_day_info(&query).expect("day info should work");
    let insight = get_day_insight(&query).expect("day insight should work");

    assert_eq!(info.solar.day, insight.solar.day);
    assert_eq!(info.solar.month, insight.solar.month);
    assert_eq!(info.lunar.day, insight.lunar.day);
    assert_eq!(info.lunar.month, insight.lunar.month);
    assert_eq!(
        info.canchi.day.can,
        insight
            .canchi
            .as_ref()
            .expect("canchi insight should exist")
            .can
            .name
    );
    assert_eq!(
        info.canchi.day.chi,
        insight
            .canchi
            .as_ref()
            .expect("canchi insight should exist")
            .chi
            .name
    );
}
