//! Contract test for event-specific figures vs. the day's almanac deity.
//!
//! `FestivalInsightDto.figures` / `HolidayInsightDto.figures` are event-specific
//! associated persons/deities (e.g. Mục Kiền Liên for Vu Lan, Táo Quân for Ông
//! Táo) and must remain independent of `DayInsightDto.day_deity`, which is the
//! day's almanac deity (Hoàng Đạo / Hắc Đạo). This contract pins the
//! distinction so the TUI can consume event figures without falling back to
//! `day_deity` as a workaround.

use amlich_api::{
    get_day_insight, DateQuery, FestivalInsightDto, FigureInsightDto, HolidayInsightDto,
};

fn tet_query() -> DateQuery {
    DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

fn phat_dan_query() -> DateQuery {
    DateQuery {
        day: 12,
        month: 5,
        year: 2025,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

fn hcm_birthday_query() -> DateQuery {
    DateQuery {
        day: 19,
        month: 5,
        year: 2025,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

#[test]
fn festival_figures_round_trip_independently_from_day_deity() {
    let insight = get_day_insight(&phat_dan_query()).expect("phat-dan insight must work");
    let festival = insight
        .festival
        .clone()
        .expect("Rằm tháng Tư must match a festival");

    let festival_json = serde_json::to_string(&festival).expect("festival must serialize");
    let festival_back: FestivalInsightDto =
        serde_json::from_str(&festival_json).expect("festival must round-trip");

    assert_eq!(
        festival_back.figures.len(),
        festival.figures.len(),
        "figure count must round-trip"
    );
    let figures_back_json = serde_json::to_string(&festival_back.figures).unwrap();
    let figures_orig_json = serde_json::to_string(&festival.figures).unwrap();
    assert_eq!(
        figures_back_json, figures_orig_json,
        "event figures must serialize/deserialize byte-for-byte"
    );

    assert!(
        !festival.figures.is_empty(),
        "Rằm tháng Tư must expose event-specific figures (Phật)"
    );
    let buddhafound = festival
        .figures
        .iter()
        .any(|f| f.name.vi.contains("Phật") || f.name.en.contains("Buddha"));
    assert!(
        buddhafound,
        "Phật Đản must list a Buddha-related figure, got: {:?}",
        festival
            .figures
            .iter()
            .map(|f| &f.name.vi)
            .collect::<Vec<_>>()
    );
}

#[test]
fn festival_figures_dto_carries_no_day_deity_field() {
    let festival: FestivalInsightDto = serde_json::from_value(serde_json::json!({
        "names": { "vi": ["a"], "en": ["b"] },
        "origin": null,
        "activities": null,
        "food": [],
        "taboos": [],
        "proverbs": [],
        "regions": null,
        "figures": [{
            "name": { "vi": "Mục Kiền Liên", "en": "Moggallana" },
            "role": { "vi": "Đệ tử Phật", "en": "Buddha disciple" },
            "description": { "vi": "v", "en": "e" }
        }],
        "category": "festival",
        "is_major": true
    }))
    .expect("festival should accept its canonical shape");

    let json = serde_json::to_value(&festival).expect("festival must serialize");
    let obj = json
        .as_object()
        .expect("festival must serialize as a JSON object");
    let allowed: std::collections::BTreeSet<&str> = [
        "activities",
        "category",
        "figures",
        "food",
        "is_major",
        "names",
        "origin",
        "proverbs",
        "regions",
        "taboos",
    ]
    .into_iter()
    .collect();
    for key in obj.keys() {
        assert!(
            allowed.contains(key.as_str()),
            "FestivalInsightDto must not carry a day_deity field; found '{key}'"
        );
    }
}

#[test]
fn holiday_figures_round_trip_independently_from_day_deity() {
    let insight = get_day_insight(&hcm_birthday_query()).expect("hcm birthday insight must work");
    let holiday = insight
        .holiday
        .clone()
        .expect("Ngày sinh Hồ Chí Minh must match a holiday");

    let holiday_json = serde_json::to_string(&holiday).expect("holiday must serialize");
    let holiday_back: HolidayInsightDto =
        serde_json::from_str(&holiday_json).expect("holiday must round-trip");

    assert!(
        !holiday.figures.is_empty(),
        "Ngày sinh Hồ Chí Minh must expose event-specific figures"
    );
    let hcm = holiday
        .figures
        .iter()
        .find(|f| f.name.vi.contains("Hồ Chí Minh"))
        .expect("holiday must list Hồ Chí Minh as a figure");

    assert!(
        !hcm.role.vi.is_empty() && !hcm.role.en.is_empty(),
        "figure role must be bilingual"
    );
    assert!(
        !hcm.description.vi.is_empty() && !hcm.description.en.is_empty(),
        "figure description must be bilingual"
    );

    let figures_back_json = serde_json::to_string(&holiday_back.figures).unwrap();
    let figures_orig_json = serde_json::to_string(&holiday.figures).unwrap();
    assert_eq!(
        figures_back_json, figures_orig_json,
        "holiday figures must serialize/deserialize byte-for-byte"
    );
}

#[test]
fn tet_festival_has_figures_and_day_deity_distinct() {
    let insight = get_day_insight(&tet_query()).expect("tet insight must work");
    let festival = insight.festival.clone().expect("Tết must match a festival");

    assert!(
        !festival.figures.is_empty(),
        "Tết must expose event-specific figures (Ông bà, Thần Tài...)"
    );

    if let Some(day_deity) = insight.day_deity.as_ref() {
        let figure_names = festival
            .figures
            .iter()
            .map(|f| f.name.vi.clone())
            .collect::<Vec<_>>();

        let day_deity_vi = &day_deity.name;
        let overlap = figure_names.iter().any(|n| n == day_deity_vi);
        assert!(
            !overlap,
            "event figures and day_deity must be distinct sources: figure={figure_names:?}, day_deity={day_deity_vi}"
        );
    }
}

#[test]
fn figure_insight_dto_carries_only_canonical_fields() {
    let figure: FigureInsightDto = serde_json::from_value(serde_json::json!({
        "name": { "vi": "Táo Quân", "en": "Kitchen Gods" },
        "role": { "vi": "Thần bếp", "en": "Kitchen deities" },
        "description": { "vi": "v", "en": "e" }
    }))
    .expect("figure should accept its canonical shape");

    let json = serde_json::to_value(&figure).expect("figure must serialize");
    let obj = json
        .as_object()
        .expect("figure must serialize as a JSON object");
    let allowed: std::collections::BTreeSet<&str> =
        ["description", "name", "role"].into_iter().collect();
    for key in obj.keys() {
        assert!(
            allowed.contains(key.as_str()),
            "FigureInsightDto must not carry extra fields like '{key}'"
        );
    }
}
