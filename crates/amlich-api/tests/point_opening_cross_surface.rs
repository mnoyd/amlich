//! v1.11 `amlich-xlag.2.3.4` cross-surface contract suite for the
//! point-opening citation.
//!
//! Locks:
//!
//! - The citation rendered from the API-transported context is
//!   byte-equal to the committed core goldens
//!   (`terminal-citation-open.txt` / `terminal-citation-closed.txt`)
//!   for the representative open (甲/戌 19:30 → 竅陰 GB44) and
//!   explicit-closed (甲/子 00:30) fixtures — the same bytes the core
//!   suite, the terminal widget, and the desktop inspector lock.
//! - The `DayInfoDto.point_opening` field is exactly additive: the
//!   serialized key set grows by that one key and nothing else, the
//!   transported context carries the full cross-surface contract
//!   field set (the Rust-side mirror of the TypeScript `keyof`
//!   locks), and no other report DTO grows the field.
//! - The serialized point-opening surface (context + provenance,
//!   every hourly state of the fixture day) stays free of clinical
//!   field names and action/efficacy phrasing (BOUND-02); the
//!   disclaimer v2 negation frames are byte-locked separately and
//!   stripped before scanning.

use std::collections::BTreeSet;

use amlich_api::{get_day_info, get_day_info_with_point_opening, DateQuery, DayInfoDto};
use amlich_core::point_opening::{point_opening_citation_text, PointOpeningSlotState};

/// The committed canonical goldens — the single byte-locked citation
/// wording every surface renders verbatim.
const GOLDEN_OPEN: &str =
    include_str!("../../amlich-core/data/ty-ngo-luu-chu/terminal-citation-open.txt");
const GOLDEN_CLOSED: &str =
    include_str!("../../amlich-core/data/ty-ngo-luu-chu/terminal-citation-closed.txt");

fn fixture_query() -> DateQuery {
    // 2024-02-10 is the frozen 甲-day fixture shared by every v1.11 suite.
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

// ---------------------------------------------------------------------------
// 1. Cross-surface golden parity
// ---------------------------------------------------------------------------

/// The API-transported context re-renders the committed goldens byte
/// for byte: transport cannot reinterpret the canonical wording.
#[test]
fn api_transport_renders_the_committed_goldens_byte_for_byte() {
    let open = get_day_info_with_point_opening(&fixture_query(), 19, 30)
        .expect("open point-opening day info");
    assert!(matches!(
        open.point_opening.as_ref().unwrap().context.state,
        PointOpeningSlotState::Open { .. }
    ));
    assert_eq!(
        point_opening_citation_text(open.point_opening.as_ref().unwrap()) + "\n",
        GOLDEN_OPEN,
        "the API surface must render the exact committed open golden"
    );

    let closed = get_day_info_with_point_opening(&fixture_query(), 0, 30)
        .expect("closed point-opening day info");
    assert!(matches!(
        closed.point_opening.as_ref().unwrap().context.state,
        PointOpeningSlotState::Closed { .. }
    ));
    assert_eq!(
        point_opening_citation_text(closed.point_opening.as_ref().unwrap()) + "\n",
        GOLDEN_CLOSED,
        "the API surface must render the exact committed closed golden"
    );
}

// ---------------------------------------------------------------------------
// 2. Additive DTO discipline
// ---------------------------------------------------------------------------

/// `point_opening` is the exactly-one additive key on `DayInfoDto`:
/// every pre-existing serialized key stays byte-identical and only
/// the new key appears.
#[test]
fn day_info_dto_grows_exactly_the_point_opening_key() {
    let ordinary = get_day_info(&fixture_query()).expect("ordinary day info");
    let with =
        get_day_info_with_point_opening(&fixture_query(), 19, 30).expect("point-opening day info");

    let ordinary_keys: BTreeSet<String> = serde_json::to_value(&ordinary)
        .expect("serialize ordinary")
        .as_object()
        .expect("DayInfoDto must serialize to an object")
        .keys()
        .cloned()
        .collect();
    let with_keys: BTreeSet<String> = serde_json::to_value(&with)
        .expect("serialize with point opening")
        .as_object()
        .expect("DayInfoDto must serialize to an object")
        .keys()
        .cloned()
        .collect();

    let mut expected = ordinary_keys.clone();
    assert!(expected.insert("point_opening".to_string()));
    assert_eq!(
        with_keys, expected,
        "the explicit point-opening path must add exactly the `point_opening` key"
    );
}

/// The transported context carries the full cross-surface contract
/// field set — the Rust-side serde mirror of the TypeScript `keyof`
/// locks in `classical-surface.types.test.ts`.
#[test]
fn transported_context_carries_the_cross_surface_contract_field_set() {
    let info =
        get_day_info_with_point_opening(&fixture_query(), 19, 30).expect("point-opening day info");
    let context =
        serde_json::to_value(info.point_opening.as_ref().unwrap()).expect("serialize context");
    let carrier = context
        .as_object()
        .expect("DayPointOpeningContext must serialize to an object");

    let expected_carrier: BTreeSet<&str> = [
        "day_stem_zh",
        "hour_branch_zh",
        "hour_pillar_zh",
        "hour_branch_vi",
        "hour_time_range",
        "hour_slot_index",
        "civil_day_canchi",
        "slot_day_canchi",
        "late_night_day_transition",
        "cross_day_spillover",
        "context",
        "provenance",
        "method_evidence",
        "calendar_evidence",
    ]
    .into_iter()
    .collect();
    let actual_carrier: BTreeSet<&str> = carrier.keys().map(String::as_str).collect();
    assert_eq!(
        actual_carrier, expected_carrier,
        "the transported carrier field set must mirror the TS keyof lock"
    );

    let inner = carrier
        .get("context")
        .expect("context field")
        .as_object()
        .expect("inner context must serialize to an object");
    let expected_inner: BTreeSet<&str> = [
        "policy_id",
        "state",
        "disclaimer",
        "review_state",
        "nomenclature_review_state",
        "safety_class",
        "time_basis",
        "known_divergence_ids",
    ]
    .into_iter()
    .collect();
    let actual_inner: BTreeSet<&str> = inner.keys().map(String::as_str).collect();
    assert_eq!(
        actual_inner, expected_inner,
        "the transported context field set must mirror the TS keyof lock"
    );
}

/// The additive field does not leak into any other report DTO — the
/// point-opening context lives on `DayInfoDto` (explicit path) and the
/// desktop `ClassicalSurfaceDto` only.
#[test]
fn personal_day_chart_dto_does_not_carry_point_opening() {
    use amlich_api::dto::PersonalDayChartDto;
    let chart = PersonalDayChartDto {
        input: amlich_api::dto::PersonalDayQueryDto {
            date: amlich_api::DateQuery {
                day: 10,
                month: 2,
                year: 2024,
                timezone: Some(7.0),
                ruleset_id: None,
                event_kind: None,
                enabled_pack_ids: vec![],
            },
            birth_year: None,
            birth_month: None,
            birth_day: None,
            gender: None,
        },
        tier: amlich_api::dto::BirthDataTierDto::Anonymous,
        solar: amlich_api::dto::SolarDto {
            day: 10,
            month: 2,
            year: 2024,
            day_of_week: 0,
            day_of_week_name: "Sat".to_string(),
            date_string: "2024-02-10".to_string(),
        },
        lunar: amlich_api::dto::LunarDto {
            day: 1,
            month: 1,
            year: 2024,
            is_leap_month: false,
            date_string: "2024-01-01".to_string(),
        },
        canchi: None,
        tiet_khi: None,
    };
    let json = serde_json::to_string(&chart).expect("serialise PersonalDayChartDto");
    assert!(
        !json.contains("point_opening"),
        "PersonalDayChartDto must not carry point_opening; got {json}"
    );
}

// ---------------------------------------------------------------------------
// 3. Safety scan of the serialized API surface
// ---------------------------------------------------------------------------

/// The extended clinical/technique field lexicon (identical to the
/// finite-core golden guard; extends the contract-guard list).
const FORBIDDEN_KEYS: &[&str] = &[
    "technique",
    "techniques",
    "depth",
    "needle_depth",
    "depth_cun",
    "manipulation",
    "indication",
    "indications",
    "contraindication",
    "contraindications",
    "efficacy",
    "effect",
    "effects",
    "recommended_point",
    "best_time",
    "best_time_to_treat",
    "point_to_press",
    "treats",
    "cures",
    "heals",
    "diagnosis",
    "prescription",
    "dosage",
    "moxa_protocol",
    "physiological_flow",
    "stimulation",
    "needle_retention",
    "needle",
    "needles",
    "needling",
    "moxa",
    "moxibustion",
    "pressure_point",
    "therapeutic",
    "therapy",
    "physiology",
];

/// Action/efficacy phrasing forbidden on every serialized surface.
const FORBIDDEN_PHRASES: &[&str] = &[
    "best time to treat",
    "best hour to treat",
    "best hour",
    "should be needled",
    "should be pressed",
    "should needle",
    "should stimulate",
    "recommended point",
    "optimal point",
    "nên châm",
    "nên bấm",
    "nên cứu",
    "nên kích thích",
    "hãy châm",
    "hãy bấm",
    "điểm nên ",
    "công dụng",
    "chữa",
    "điều trị",
    "thải độc",
    "liều lượng",
    "hoạt động mạnh nhất",
    "đạt đỉnh",
];

fn collect_keys(value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, child) in map {
                out.push(key.clone());
                collect_keys(child, out);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_keys(item, out);
            }
        }
        _ => {}
    }
}

/// Remove every `disclaimer` node — the bilingual negation frames are
/// the only permitted clinical-verb contexts and are byte-locked
/// separately by `point_opening_contract_guard.rs`.
fn strip_disclaimers(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            map.remove("disclaimer");
            for child in map.values_mut() {
                strip_disclaimers(child);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                strip_disclaimers(item);
            }
        }
        _ => {}
    }
}

fn assert_surface_is_safe(label: &str, mut payload: serde_json::Value) {
    strip_disclaimers(&mut payload);
    let mut keys = Vec::new();
    collect_keys(&payload, &mut keys);
    for key in &keys {
        assert!(
            !FORBIDDEN_KEYS.contains(&key.as_str()),
            "{label}: prohibited clinical field `{key}` on the API surface"
        );
    }
    let text = payload.to_string().to_lowercase();
    for phrase in FORBIDDEN_PHRASES {
        assert!(
            !text.contains(phrase),
            "{label}: prohibited action/efficacy phrasing `{phrase}` on the API surface"
        );
    }
}

/// Every hourly point-opening state the explicit API path can emit
/// stays free of clinical fields and action/efficacy phrasing.
#[test]
fn every_api_point_opening_state_is_free_of_clinical_or_action_language() {
    for hour in 0u8..=23 {
        let info = get_day_info_with_point_opening(&fixture_query(), hour, 30)
            .expect("point-opening day info");
        let payload = serde_json::to_value(info.point_opening.as_ref().unwrap())
            .expect("serialize transported context");
        assert_surface_is_safe(&format!("api hour {hour}:30"), payload);
    }
}

/// Legacy deserialization keeps accepting pre-v1.11 payloads and the
/// ordinary path never grows the field.
#[test]
fn ordinary_payload_round_trips_without_the_field() {
    let info = get_day_info(&fixture_query()).expect("ordinary day info");
    let json = serde_json::to_string(&info).expect("serialize ordinary DTO");
    assert!(!json.contains("point_opening"));
    let recovered: DayInfoDto = serde_json::from_str(&json).expect("deserialize legacy payload");
    assert!(recovered.point_opening.is_none());
    assert_eq!(json, serde_json::to_string(&recovered).unwrap());
}
