use amlich_core::almanac::data::{
    get_ruleset_data, get_ruleset_descriptor_doc, DEFAULT_RULESET_ID,
};
use amlich_core::get_day_info;

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
fn day_output_is_deterministic_for_same_input() {
    let a = get_day_info(10, 2, 2024);
    let b = get_day_info(10, 2, 2024);

    assert_eq!(a.ruleset_id, "vn_baseline_v1");
    assert_eq!(a.ruleset_id, b.ruleset_id);
    assert_eq!(a.ruleset_version, b.ruleset_version);
    assert_eq!(a.canchi.day.full, b.canchi.day.full);
    assert_eq!(a.lunar.date_string, b.lunar.date_string);

    let fortune_a = serde_json::to_string(&a.day_fortune).expect("serialize day fortune A");
    let fortune_b = serde_json::to_string(&b.day_fortune).expect("serialize day fortune B");
    assert_eq!(fortune_a, fortune_b);
}
