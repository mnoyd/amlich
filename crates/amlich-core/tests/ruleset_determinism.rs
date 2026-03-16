use amlich_core::almanac::calc::calculate_day_fortune;
use amlich_core::almanac::data::{
    get_ruleset_data, get_ruleset_descriptor_doc, DEFAULT_RULESET_ID,
};
use amlich_core::canchi::{get_day_canchi, get_year_canchi};
use amlich_core::julian::jd_from_date;
use amlich_core::lunar::convert_solar_to_lunar;
use amlich_core::tietkhi::get_tiet_khi;
use amlich_core::VIETNAM_TIMEZONE;

#[test]
fn ruleset_descriptor_is_deterministic_for_same_id() {
    let a = get_ruleset_descriptor_doc(DEFAULT_RULESET_ID).expect("descriptor A");
    let b = get_ruleset_descriptor_doc(DEFAULT_RULESET_ID).expect("descriptor B");
    assert_eq!(a, b);
}

#[test]
fn ruleset_descriptor_alias_matches_canonical_id() {
    let canonical = get_ruleset_descriptor_doc(DEFAULT_RULESET_ID).expect("canonical descriptor");
    let alias = get_ruleset_descriptor_doc("baseline").expect("alias descriptor");
    assert_eq!(alias, canonical);
}

#[test]
fn ruleset_data_alias_points_to_same_loaded_ruleset() {
    let canonical = get_ruleset_data(DEFAULT_RULESET_ID).expect("canonical data");
    let alias = get_ruleset_data("baseline").expect("alias data");
    assert!(std::ptr::eq(canonical, alias));
}

#[test]
fn unknown_ruleset_id_returns_explicit_error() {
    let err = get_ruleset_data("not-a-ruleset").expect_err("unknown ruleset must fail");
    assert_eq!(err.to_string(), "unknown almanac ruleset id: not-a-ruleset");
}

#[test]
fn day_fortune_is_deterministic_for_same_explicit_inputs() {
    let jd = jd_from_date(10, 2, 2024);
    let lunar = convert_solar_to_lunar(10, 2, 2024, VIETNAM_TIMEZONE);
    let day_canchi = get_day_canchi(jd);
    let year_canchi = get_year_canchi(lunar.year);
    let tiet_khi = get_tiet_khi(jd, VIETNAM_TIMEZONE);

    let a = calculate_day_fortune(
        jd,
        &day_canchi,
        lunar.day,
        lunar.month,
        &year_canchi.can,
        &tiet_khi.name,
    );
    let b = calculate_day_fortune(
        jd,
        &day_canchi,
        lunar.day,
        lunar.month,
        &year_canchi.can,
        &tiet_khi.name,
    );

    assert_eq!(a.ruleset_id, "vn_baseline_v1");
    assert_eq!(a.ruleset_id, b.ruleset_id);
    assert_eq!(a.ruleset_version, b.ruleset_version);

    let fortune_a = serde_json::to_string(&a).expect("serialize day fortune A");
    let fortune_b = serde_json::to_string(&b).expect("serialize day fortune B");
    assert_eq!(fortune_a, fortune_b);
}
