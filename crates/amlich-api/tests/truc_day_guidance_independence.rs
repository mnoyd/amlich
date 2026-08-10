//! Contract test for the independent Trực / Ngũ Hành boundary.
//!
//! See `docs/adr/0002-independent-truc-and-ngu-hanh-boundaries.md`.
//! `TrucInsightDto` and `DayGuidanceDto` are deliberately independent surfaces on
//! `DayInsightDto`; this test pins that contract so future contributors cannot
//! quietly introduce a typed interaction without a contract-test change.

use amlich_api::{get_day_insight, DateQuery, DayGuidanceDto, TrucInsightDto};

fn query_with_both_populated() -> DateQuery {
    DateQuery {
        day: 1,
        month: 1,
        year: 2025,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

#[test]
fn truc_and_day_guidance_round_trip_independently() {
    let insight =
        get_day_insight(&query_with_both_populated()).expect("day insight should be available");
    let truc = insight
        .truc
        .clone()
        .expect("truc insight should be populated on this date");
    let day_guidance = insight
        .day_guidance
        .clone()
        .expect("day_guidance insight should be populated on this date");
    let truc_json = serde_json::to_string(&truc).expect("truc must serialize");
    let guidance_json = serde_json::to_string(&day_guidance).expect("day_guidance must serialize");

    let truc_back: TrucInsightDto = serde_json::from_str(&truc_json).expect("truc must round-trip");
    let guidance_back: DayGuidanceDto =
        serde_json::from_str(&guidance_json).expect("day_guidance must round-trip");

    let truc_back_json = serde_json::to_string(&truc_back).expect("truc must re-serialize");
    let guidance_back_json =
        serde_json::to_string(&guidance_back).expect("day_guidance must re-serialize");
    assert_eq!(truc_back_json, truc_json);
    assert_eq!(guidance_back_json, guidance_json);
}

#[test]
fn truc_dto_carries_no_day_element_interaction_field() {
    let truc: TrucInsightDto = serde_json::from_value(serde_json::json!({
        "name": "Kiến",
        "quality": "cat",
        "meaning": { "vi": "v", "en": "e" },
        "good_for": { "vi": ["a"], "en": ["b"] },
        "avoid_for": { "vi": ["c"], "en": ["d"] }
    }))
    .expect("truc should accept its canonical shape");

    let json = serde_json::to_value(&truc).expect("truc must serialize");
    let obj = json
        .as_object()
        .expect("truc should serialize as a JSON object");
    let allowed: std::collections::BTreeSet<&str> =
        ["avoid_for", "good_for", "meaning", "name", "quality"]
            .into_iter()
            .collect();
    for key in obj.keys() {
        assert!(
            allowed.contains(key.as_str()),
            "TrucInsightDto must not carry a day-element interaction field; found '{key}'"
        );
    }
}

#[test]
fn full_insight_carries_truc_and_day_guidance_as_independent_options() {
    let insight =
        get_day_insight(&query_with_both_populated()).expect("day insight should be available");
    let full = serde_json::to_value(&insight).expect("insight must serialize");
    let obj = full
        .as_object()
        .expect("insight must serialize as a JSON object");

    assert!(
        obj.contains_key("truc"),
        "truc is a top-level independent option on DayInsightDto"
    );
    assert!(
        obj.contains_key("day_guidance"),
        "day_guidance is a top-level independent option on DayInsightDto"
    );

    let truc_json = serde_json::to_value(insight.truc.as_ref().expect("truc should be populated"))
        .expect("truc must serialize independently");
    let guidance_json = serde_json::to_value(
        insight
            .day_guidance
            .as_ref()
            .expect("day_guidance should be populated"),
    )
    .expect("day_guidance must serialize independently");

    assert_eq!(
        truc_json,
        obj.get("truc")
            .expect("top-level truc must equal truc-only serialization")
            .clone()
    );
    assert_eq!(
        guidance_json,
        obj.get("day_guidance")
            .expect("top-level day_guidance must equal day_guidance-only serialization")
            .clone()
    );
}
