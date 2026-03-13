use amlich_api::{get_day_insight, get_day_insight_with_profile, DateQuery, DayInsightDto};

#[test]
fn enriched_insight_has_all_day_only_fields() {
    let query = DateQuery {
        day: 1,
        month: 1,
        year: 2025,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    let insight = get_day_insight(&query).unwrap();

    // Existing fields still work
    assert!(insight.canchi.is_some());
    assert!(insight.tiet_khi.is_some());

    // New day-only fields populated
    assert!(insight.truc.is_some());
    assert!(insight.na_am.is_some());
    assert!(insight.stars.is_some());
    assert!(insight.travel.is_some());
    assert!(insight.hours.is_some());

    // Birth-dependent fields absent without profile
    assert!(insight.tu_menh.is_none());
    assert!(insight.dai_van.is_none());
}

#[test]
fn enriched_insight_bilingual_non_empty() {
    let query = DateQuery {
        day: 15,
        month: 6,
        year: 2025,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    let insight = get_day_insight(&query).unwrap();

    if let Some(truc) = &insight.truc {
        assert!(!truc.meaning.vi.is_empty());
        assert!(!truc.meaning.en.is_empty());
    }
    if let Some(na_am) = &insight.na_am {
        assert!(!na_am.meaning.vi.is_empty());
        assert!(!na_am.meaning.en.is_empty());
    }
}

#[test]
fn enriched_insight_with_profile_populates_birth_fields() {
    let query = DateQuery {
        day: 1,
        month: 1,
        year: 2025,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    let insight = get_day_insight_with_profile(
        &query,
        Some(1990),
        Some(5),
        Some(15),
        Some(amlich_core::almanac::tu_menh::Gender::Male),
    )
    .unwrap();

    assert!(insight.tu_menh.is_some());
    let tu_menh = insight.tu_menh.unwrap();
    assert!(tu_menh.kua > 0);
    assert!(!tu_menh.meaning.vi.is_empty());

    assert!(insight.dai_van.is_some());
    let dai_van = insight.dai_van.unwrap();
    assert!(!dai_van.all_pillars.is_empty());
}

#[test]
fn enriched_insight_json_roundtrip() {
    let query = DateQuery {
        day: 10,
        month: 3,
        year: 2025,
        timezone: None,
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    };
    let insight = get_day_insight(&query).unwrap();
    let json = serde_json::to_string(&insight).unwrap();
    let parsed: DayInsightDto = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.solar.day, insight.solar.day);
}
