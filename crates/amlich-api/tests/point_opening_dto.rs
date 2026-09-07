//! v1.11 additive point-opening transport contract.
//!
//! The API keeps the canonical core context byte-identical and only exposes it
//! through the explicit time-aware path. Ordinary day payloads remain unchanged.

use amlich_api::{get_day_info, get_day_info_with_point_opening, DateQuery, DayInfoDto};
use amlich_core::point_opening::PointOpeningSlotState;

fn fixture_query() -> DateQuery {
    // 2024-02-10 is the frozen 甲-day fixture used by the core corpus tests.
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

#[test]
fn ordinary_day_info_omits_and_accepts_the_additive_field() {
    let info = get_day_info(&fixture_query()).expect("ordinary day info");
    assert!(info.point_opening.is_none());

    let json = serde_json::to_string(&info).expect("serialize ordinary DTO");
    assert!(
        !json.contains("point_opening"),
        "ordinary payload must retain its existing shape; got {json}"
    );

    let recovered: DayInfoDto = serde_json::from_str(&json).expect("deserialize legacy payload");
    assert!(recovered.point_opening.is_none());
    assert_eq!(json, serde_json::to_string(&recovered).unwrap());
}

#[test]
fn api_transport_preserves_the_frozen_open_context_byte_for_byte() {
    // 甲/戌 is a frozen open row; 19:30 belongs to the 戌 branch.
    let info =
        get_day_info_with_point_opening(&fixture_query(), 19, 30).expect("point-opening day info");
    let transported = info
        .point_opening
        .expect("explicit API path populates context");
    assert!(matches!(
        transported.context.state,
        PointOpeningSlotState::Open { .. }
    ));

    let snapshot = amlich_core::calculate_day_snapshot(10, 2, 2024);
    let core = amlich_core::enrich_day_snapshot_with_point_opening(&snapshot, 19, 30)
        .expect("core enrichment")
        .point_opening
        .expect("core context");
    assert_eq!(
        serde_json::to_string(&core).unwrap(),
        serde_json::to_string(&transported).unwrap(),
        "API transport must not alter the canonical open context"
    );
}

#[test]
fn api_transport_preserves_the_explicit_closed_context_byte_for_byte() {
    // 甲/子 is a frozen closed row; 00:30 belongs to the 子 branch.
    let info =
        get_day_info_with_point_opening(&fixture_query(), 0, 30).expect("point-opening day info");
    let transported = info
        .point_opening
        .expect("explicit API path populates context");
    assert!(matches!(
        transported.context.state,
        PointOpeningSlotState::Closed { .. }
    ));

    let snapshot = amlich_core::calculate_day_snapshot(10, 2, 2024);
    let core = amlich_core::enrich_day_snapshot_with_point_opening(&snapshot, 0, 30)
        .expect("core enrichment")
        .point_opening
        .expect("core context");
    assert_eq!(
        serde_json::to_string(&core).unwrap(),
        serde_json::to_string(&transported).unwrap(),
        "API transport must not alter the canonical closed context"
    );
}
