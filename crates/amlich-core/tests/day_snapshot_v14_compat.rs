//! INT-05 — DaySnapshot v1.4 backward-compatibility round-trip tests.
//!
//! These tests prove:
//!   1. v15_round_trip_byte_equal        — serialize→deserialize→serialize yields byte-equal JSON.
//!   2. additive_fields_absent_when_none — when flying_stars and applicable_rituals are None,
//!                                         neither key appears in the serialized JSON.
//!   3. v14_json_deserializes_into_v15   — a JSON object lacking the two new keys (mimicking a
//!                                         v1.4 payload) deserializes successfully into the v1.5
//!                                         DaySnapshot, confirming `#[serde(default)]` lenience.
//!
//! Imports via `use amlich_core::...` as an external consumer would.

use amlich_core::calculate_day_snapshot;
use amlich_core::enrich_day_snapshot_with_direction_cross_link;
use amlich_core::DaySnapshot;
use amlich_core::iching::{enrich_day_snapshot_with_iching, IChingQuery};
use amlich_core::rituals::OfferingRef;
use amlich_core::sources::{
    SOURCE_HUYEN_KHONG, SOURCE_KHCBPPT, SOURCE_KINH_DICH, SOURCE_MAI_HOA_DICH_SO,
    SOURCE_VN_FOLK_RITUAL,
};

// ---------------------------------------------------------------------------
// Test 1 — v15 round-trip byte-equal
// ---------------------------------------------------------------------------

/// Serialize a default-populated DaySnapshot (which now includes the two new
/// additive fields when Some), deserialize it, re-serialize, and assert the
/// two JSON strings are byte-equal.
#[test]
fn v15_round_trip_byte_equal() {
    let snap = calculate_day_snapshot(10, 2, 2024);

    let json = serde_json::to_string(&snap).expect("serialization failed");
    let round_tripped: DaySnapshot =
        serde_json::from_str(&json).expect("deserialization failed");
    let json2 = serde_json::to_string(&round_tripped).expect("re-serialization failed");

    assert_eq!(
        json, json2,
        "v1.5 DaySnapshot round-trip: first and second serializations must be byte-equal"
    );
}

// ---------------------------------------------------------------------------
// Test 2 — additive fields absent in JSON when None
// ---------------------------------------------------------------------------

/// When both new optional fields are explicitly set to None, neither
/// "flying_stars" nor "applicable_rituals" should appear as keys in the
/// serialized JSON output.
#[test]
fn additive_fields_absent_when_none() {
    let mut snap = calculate_day_snapshot(10, 2, 2024);
    snap.flying_stars = None;
    snap.applicable_rituals = None;

    let json = serde_json::to_string(&snap).expect("serialization failed");

    assert!(
        !json.contains("\"flying_stars\""),
        "flying_stars must NOT appear in JSON when the field is None; got:\n{json}"
    );
    assert!(
        !json.contains("\"applicable_rituals\""),
        "applicable_rituals must NOT appear in JSON when the field is None; got:\n{json}"
    );
}

// ---------------------------------------------------------------------------
// Test 3 — v1.4 JSON (missing new keys) deserializes into v1.5 DaySnapshot
// ---------------------------------------------------------------------------

/// Build a JSON representation that does NOT contain "flying_stars" or
/// "applicable_rituals" keys (as a v1.4 producer would emit), then confirm
/// it deserializes successfully into the v1.5 struct.
///
/// This proves `#[serde(default)]` tolerates the absence of both new fields.
#[test]
fn v14_json_without_new_fields_deserializes() {
    // Obtain a v1.5 snapshot, strip the two new fields, use that as a
    // stand-in for a v1.4 producer payload.
    let mut snap = calculate_day_snapshot(10, 2, 2024);
    snap.flying_stars = None;
    snap.applicable_rituals = None;

    let v14_json =
        serde_json::to_string(&snap).expect("serialization of None-fielded snapshot failed");

    // Confirm neither key is present (we're testing what v1.4 would have emitted).
    assert!(
        !v14_json.contains("\"flying_stars\""),
        "test precondition: flying_stars must not be in the stripped JSON"
    );
    assert!(
        !v14_json.contains("\"applicable_rituals\""),
        "test precondition: applicable_rituals must not be in the stripped JSON"
    );

    // Deserializing this v1.4-shaped JSON must succeed with the v1.5 struct.
    let recovered: DaySnapshot = serde_json::from_str(&v14_json)
        .expect("v1.4 JSON (without new fields) must deserialize into v1.5 DaySnapshot");

    // The two additive fields must have been defaulted to None.
    assert!(
        recovered.flying_stars.is_none(),
        "flying_stars must default to None when absent from JSON"
    );
    assert!(
        recovered.applicable_rituals.is_none(),
        "applicable_rituals must default to None when absent from JSON"
    );

    // Core fields must survive round-trip intact.
    assert_eq!(
        recovered.context.solar.day,
        snap.context.solar.day,
        "solar day must survive round-trip"
    );
    assert_eq!(
        recovered.context.solar.month,
        snap.context.solar.month,
        "solar month must survive round-trip"
    );
    assert_eq!(
        recovered.context.solar.year,
        snap.context.solar.year,
        "solar year must survive round-trip"
    );
    assert_eq!(
        recovered.ruleset_id,
        snap.ruleset_id,
        "ruleset_id must survive round-trip"
    );
}

// ---------------------------------------------------------------------------
// Test 4 — v1.5 JSON (without daily_flying_stars) deserializes into v1.6 DaySnapshot
// ---------------------------------------------------------------------------

/// A v1.5 DaySnapshot JSON (with flying_stars and applicable_rituals populated
/// but NO daily_flying_stars key) must deserialize cleanly into the v1.6
/// DaySnapshot struct. The new daily_flying_stars field is additive with
/// #[serde(default)] so the missing key defaults to None.
#[test]
fn v15_json_without_daily_flying_stars_deserializes() {
    let mut snap = calculate_day_snapshot(10, 2, 2024);
    snap.daily_flying_stars = None;

    let v15_json = serde_json::to_string(&snap).expect("v1.5-shaped serialization failed");
    assert!(!v15_json.contains("\"daily_flying_stars\""),
        "test precondition: daily_flying_stars must not be in the stripped JSON");

    let recovered: DaySnapshot = serde_json::from_str(&v15_json)
        .expect("v1.5 JSON must deserialize into v1.6 DaySnapshot");
    assert!(recovered.daily_flying_stars.is_none(),
        "daily_flying_stars must default to None when absent from JSON");
    assert!(recovered.flying_stars.is_some());
    assert!(recovered.applicable_rituals.is_some());
}

// ---------------------------------------------------------------------------
// Test 5 — daily_flying_stars byte-equal round-trip
// ---------------------------------------------------------------------------

#[test]
fn daily_flying_stars_byte_equal_round_trip() {
    let snap = calculate_day_snapshot(10, 2, 2024);
    let daily = snap.daily_flying_stars.as_ref().expect("daily_flying_stars must be populated");

    let json = serde_json::to_string(&snap).expect("v1.6 serialization failed");
    let round_tripped: DaySnapshot = serde_json::from_str(&json).expect("v1.6 deserialization failed");
    let json2 = serde_json::to_string(&round_tripped).expect("v1.6 re-serialization failed");

    assert_eq!(json, json2, "v1.6 DaySnapshot round-trip must be byte-equal");
    assert_eq!(
        round_tripped.daily_flying_stars.as_ref().map(|d| d.center_star as u8),
        Some(daily.center_star as u8),
        "daily_flying_stars.center_star must survive round-trip"
    );
}

// ---------------------------------------------------------------------------
// Test 6 — daily_flying_stars absent in JSON when None
// ---------------------------------------------------------------------------

#[test]
fn daily_flying_stars_absent_when_none() {
    let mut snap = calculate_day_snapshot(10, 2, 2024);
    snap.daily_flying_stars = None;

    let json = serde_json::to_string(&snap).expect("serialization failed");
    assert!(!json.contains("\"daily_flying_stars\""),
        "daily_flying_stars must NOT appear in JSON when None; got: {json}");
}

// ---------------------------------------------------------------------------
// Test 7 — v1.5 JSON (without daily_flying_stars + offering_refs + offerings)
//          deserializes into v1.6 DaySnapshot + byte-equal round-trip
// ---------------------------------------------------------------------------
//
// Mirrors Phase 18-04 Test 4 (`v15_json_without_daily_flying_stars_deserializes`)
// but EXTENDED for the new `offering_refs` + `offerings` additive fields
// added by Plan 19-01 (INT-08). The v1.5 fixture pattern is:
//   has `flying_stars`, no `daily_flying_stars`, no `offering_refs`, no `offerings`.
//
// BLOCKER 5 FIX: this test strips ALL THREE new fields together (not just
// `offering_refs` + `offerings`), re-serializes the recovered v1.6 value,
// and asserts byte-equal round-trip + no unexpected fields appear. The
// original Phase 18-04 Test 4 only stripped `daily_flying_stars`; the
// combined strip proves that v1.5 JSON without ANY of the v1.6 fields
// round-trips cleanly through the v1.6 struct.
#[test]
fn v15_json_without_v16_fields_deserializes_and_round_trips() {
    let snap = calculate_day_snapshot(17, 2, 2026); // Tết 2026 — guarantees ALL fields populated

    // Sanity: all v1.6 surfaces populated before strip
    assert!(snap.flying_stars.is_some(), "flying_stars must be Some for Tết 2026");
    assert!(snap.applicable_rituals.is_some(), "applicable_rituals must be Some");
    assert!(snap.daily_flying_stars.is_some(), "daily_flying_stars must be Some");
    assert!(snap.offering_refs.is_some(), "offering_refs must be Some for Tết 2026");
    assert!(snap.offerings.is_some(), "offerings (legacy flat-string) must be Some");

    // Round-trip the full v1.6 snapshot first (sanity: byte-equal baseline)
    let v16_json = serde_json::to_string(&snap).expect("v1.6 serialization failed");
    let v16_recovered: DaySnapshot = serde_json::from_str(&v16_json)
        .expect("v1.6 JSON must deserialize into v1.6 DaySnapshot");
    assert_eq!(serde_json::to_string(&v16_recovered).expect("re-serialize"), v16_json,
               "v1.6 round-trip must be byte-equal");

    // Now strip ALL THREE v1.6-new fields together to simulate v1.5 JSON
    let mut v15_json: serde_json::Value = serde_json::from_str(&v16_json).expect("parse");
    let obj = v15_json.as_object_mut().expect("must be object");
    obj.remove("daily_flying_stars");
    obj.remove("offering_refs");
    obj.remove("offerings");
    let v15_str = serde_json::to_string(&v15_json).expect("re-serialize v1.5-shaped");

    // Verify test precondition: all three new keys absent (v1.5 fixture shape)
    assert!(!v15_str.contains("\"daily_flying_stars\""),
            "test precondition: daily_flying_stars must not be in the stripped JSON");
    assert!(!v15_str.contains("\"offering_refs\""),
            "test precondition: offering_refs must not be in the stripped JSON");
    assert!(!v15_str.contains("\"offerings\""),
            "test precondition: offerings must not be in the stripped JSON");
    // Verify `flying_stars` (v1.5 field) IS present — the v1.5 fixture pattern
    assert!(v15_str.contains("\"flying_stars\""),
            "test precondition: flying_stars MUST be in the v1.5-shaped JSON (v1.5 fixture pattern)");

    let recovered: DaySnapshot = serde_json::from_str(&v15_str)
        .expect("v1.5 JSON (without daily_flying_stars + offering_refs + offerings) must deserialize into v1.6 DaySnapshot");

    // The three stripped fields must default to None
    assert!(recovered.daily_flying_stars.is_none(),
            "daily_flying_stars must default to None when absent from JSON");
    assert!(recovered.offering_refs.is_none(),
            "offering_refs must default to None when absent from JSON");
    assert!(recovered.offerings.is_none(),
            "offerings must default to None when absent from JSON");

    // Existing v1.5 surfaces must survive the strip
    assert!(recovered.flying_stars.is_some(), "flying_stars must survive strip");
    assert!(recovered.applicable_rituals.is_some(), "applicable_rituals must survive strip");

    // BLOCKER 5 FIX: re-serialize the recovered v1.6 value and assert byte-equal
    // (no unexpected fields appear after the round-trip). Also verify that
    // the three stripped fields do NOT appear in the re-serialization.
    let re_serialized = serde_json::to_string(&recovered).expect("re-serialization failed");
    // NOTE: byte-equal comparison is impossible here because `v15_str` was
    // produced via `serde_json::Value::to_string()` which alphabetizes map
    // keys, while `re_serialized` uses struct-declaration field order.
    // Compare semantically instead by parsing both into `serde_json::Value`.
    let v15_value: serde_json::Value =
        serde_json::from_str(&v15_str).expect("v15_str must reparse as JSON value");
    let re_value: serde_json::Value =
        serde_json::from_str(&re_serialized).expect("re_serialized must reparse as JSON value");
    assert_eq!(v15_value, re_value,
               "v1.5 → v1.6 → v1.5 round-trip must be semantically equal (no unexpected fields)");
    assert!(!re_serialized.contains("\"daily_flying_stars\""),
            "re-serialized JSON must NOT contain daily_flying_stars when None");
    assert!(!re_serialized.contains("\"offering_refs\""),
            "re-serialized JSON must NOT contain offering_refs when None");
    assert!(!re_serialized.contains("\"offerings\""),
            "re-serialized JSON must NOT contain offerings when None");
}

// ---------------------------------------------------------------------------
// Test 8 — offering_refs byte-equal round-trip with field-shape assertions
// ---------------------------------------------------------------------------
//
// Mirrors Phase 18-04 Test 5 (`daily_flying_stars_byte_equal_round_trip`) plus
// the field-by-field discipline from v1.5 Test 1. Asserts:
//   1. byte-equal round-trip (with offering_refs populated)
//   2. offering_refs[0].offering_id is non-empty AND follows ritual.<id>.offering.<idx>
//   3. offering_refs[0].name_vi is non-empty
//   4. offering_refs[0].source_id == SOURCE_VN_FOLK_RITUAL ("vn-folk-ritual")
//   5. offerings flat-string has at least one entry matching offering_refs[0].name_vi
#[test]
fn offering_refs_byte_equal_round_trip() {
    let snap = calculate_day_snapshot(17, 2, 2026);
    let refs = snap.offering_refs.as_ref()
        .expect("offering_refs must be populated for Tết 2026");
    assert!(!refs.is_empty(), "offering_refs must be non-empty for Tết 2026");
    let flat = snap.offerings.as_ref()
        .expect("offerings (flat-string) must be populated for Tết 2026");
    assert!(!flat.is_empty(), "offerings (flat-string) must be non-empty for Tết 2026");

    // Field-by-field assertion on the first OfferingRef (Pitfall P-11)
    let first: &OfferingRef = &refs[0];
    assert!(!first.offering_id.is_empty(),
            "OfferingRef.offering_id must be non-empty");
    assert!(first.offering_id.starts_with("ritual."),
            "OfferingRef.offering_id must follow ritual.<id>.offering.<idx> pattern; got {:?}",
            first.offering_id);
    assert!(!first.name_vi.is_empty(),
            "OfferingRef.name_vi must be non-empty");
    assert_eq!(first.source_id, SOURCE_VN_FOLK_RITUAL,
               "OfferingRef.source_id must equal vn-folk-ritual");

    // Round-trip byte-equal
    let json = serde_json::to_string(&snap).expect("serialization failed");
    let round_tripped: DaySnapshot = serde_json::from_str(&json)
        .expect("deserialization failed");
    let json2 = serde_json::to_string(&round_tripped).expect("re-serialization failed");
    assert_eq!(json, json2,
               "v1.6 DaySnapshot round-trip (with offering_refs) must be byte-equal");

    // Field-shape after round-trip
    assert_eq!(round_tripped.offering_refs, snap.offering_refs,
               "offering_refs must survive round-trip");
    assert_eq!(round_tripped.offerings, snap.offerings,
               "offerings flat-string must survive round-trip");

    // Cross-check: every OfferingRef.name_vi must appear in the flat-string offerings
    for r in refs.iter() {
        assert!(flat.contains(&r.name_vi),
                "flat-string offerings must contain OfferingRef.name_vi = {:?}", r.name_vi);
    }
}

// ---------------------------------------------------------------------------
// Test 9 — offering_refs absent in JSON when None
// ---------------------------------------------------------------------------
//
// Mirrors Phase 18-04 Test 6 (`daily_flying_stars_absent_when_none`). The
// `#[serde(skip_serializing_if = "Option::is_none")]` discipline must be
// honored — when both `offering_refs` and `offerings` are None, neither key
// appears in the serialized JSON.
#[test]
fn offering_refs_absent_when_none() {
    let mut snap = calculate_day_snapshot(17, 2, 2026);
    snap.offering_refs = None;
    snap.offerings = None;

    let json = serde_json::to_string(&snap).expect("serialization failed");
    assert!(!json.contains("\"offering_refs\""),
            "offering_refs must NOT appear in JSON when None; got: {json}");
    assert!(!json.contains("\"offerings\""),
            "offerings must NOT appear in JSON when None; got: {json}");
}

// ---------------------------------------------------------------------------
// Test 10 — v1.6 JSON (without iching_cast + direction_cross_link) deserializes
//           into v1.7 DaySnapshot + semantic-equality round-trip
// ---------------------------------------------------------------------------
//
// Mirrors Phase 19-03 Test 7 (`v15_json_without_v16_fields_deserializes_and_round_trips`)
// but extends it to the two v1.7 additive fields: `iching_cast` and
// `direction_cross_link`. The v1.6 fixture keeps the v1.6 fields populated
// (`flying_stars`, `daily_flying_stars`, `offering_refs`, `offerings`) while
// omitting both v1.7 fields. Re-serialization must not introduce either key
// or any `null` values.
#[test]
fn v16_json_without_v17_iching_fields_deserializes_and_round_trips() {
    let snap = calculate_day_snapshot(17, 2, 2026); // Tết 2026 — guarantees v1.6 fields populated

    // Sanity: all v1.6 surfaces populated before strip.
    assert!(
        snap.flying_stars.is_some(),
        "flying_stars must be Some for Tết 2026"
    );
    assert!(
        snap.applicable_rituals.is_some(),
        "applicable_rituals must be Some"
    );
    assert!(
        snap.daily_flying_stars.is_some(),
        "daily_flying_stars must be Some"
    );
    assert!(
        snap.offering_refs.is_some(),
        "offering_refs must be Some for Tết 2026"
    );
    assert!(
        snap.offerings.is_some(),
        "offerings (legacy flat-string) must be Some"
    );

    // Ordinary snapshots do not implicitly populate either v1.7 surface.
    assert!(
        snap.iching_cast.is_none(),
        "iching_cast must default to None on an ordinary snapshot"
    );
    assert!(
        snap.direction_cross_link.is_none(),
        "direction_cross_link must default to None on an ordinary snapshot"
    );

    // Round-trip the full snapshot first (sanity: byte-equal baseline).
    let v17_json = serde_json::to_string(&snap).expect("v1.7 serialization failed");
    let v17_recovered: DaySnapshot =
        serde_json::from_str(&v17_json).expect("v1.7 JSON must deserialize into DaySnapshot");
    assert_eq!(
        serde_json::to_string(&v17_recovered).expect("re-serialize"),
        v17_json,
        "ordinary v1.7 snapshot round-trip must be byte-equal"
    );

    // Strip BOTH v1.7-new fields together to simulate v1.6 JSON.
    let mut v16_json_value: serde_json::Value = serde_json::from_str(&v17_json).expect("parse");
    let obj = v16_json_value.as_object_mut().expect("must be object");
    obj.remove("iching_cast");
    obj.remove("direction_cross_link");
    let v16_str = serde_json::to_string(&v16_json_value).expect("re-serialize v1.6-shaped");

    assert!(
        !v16_str.contains("\"iching_cast\""),
        "test precondition: iching_cast must not be in the stripped JSON"
    );
    assert!(
        !v16_str.contains("\"direction_cross_link\""),
        "test precondition: direction_cross_link must not be in the stripped JSON"
    );
    assert!(
        v16_str.contains("\"flying_stars\""),
        "test precondition: flying_stars MUST be in the v1.6-shaped JSON"
    );
    assert!(
        v16_str.contains("\"daily_flying_stars\""),
        "test precondition: daily_flying_stars MUST be in the v1.6-shaped JSON"
    );
    assert!(
        v16_str.contains("\"offering_refs\""),
        "test precondition: offering_refs MUST be in the v1.6-shaped JSON"
    );
    assert!(
        v16_str.contains("\"offerings\""),
        "test precondition: offerings MUST be in the v1.6-shaped JSON"
    );

    let recovered: DaySnapshot = serde_json::from_str(&v16_str)
        .expect("v1.6 JSON (without iching_cast + direction_cross_link) must deserialize into v1.7 DaySnapshot");

    assert!(
        recovered.iching_cast.is_none(),
        "iching_cast must default to None when absent from JSON"
    );
    assert!(
        recovered.direction_cross_link.is_none(),
        "direction_cross_link must default to None when absent from JSON"
    );
    assert!(
        recovered.flying_stars.is_some(),
        "flying_stars must survive strip"
    );
    assert!(
        recovered.applicable_rituals.is_some(),
        "applicable_rituals must survive strip"
    );
    assert!(
        recovered.daily_flying_stars.is_some(),
        "daily_flying_stars must survive strip"
    );
    assert!(
        recovered.offering_refs.is_some(),
        "offering_refs must survive strip"
    );
    assert!(
        recovered.offerings.is_some(),
        "offerings must survive strip"
    );

    let re_serialized = serde_json::to_string(&recovered).expect("re-serialization failed");
    let v16_value: serde_json::Value =
        serde_json::from_str(&v16_str).expect("v16_str must reparse as JSON value");
    let re_value: serde_json::Value =
        serde_json::from_str(&re_serialized).expect("re_serialized must reparse as JSON value");
    assert_eq!(
        v16_value, re_value,
        "v1.6 → v1.7 → v1.6 round-trip must be semantically equal"
    );

    assert_eq!(
        re_serialized.matches("null").count(),
        v16_str.matches("null").count(),
        "re-serialized JSON must not ADD null values beyond the v1.6-shaped fixture"
    );
    assert!(
        !re_serialized.contains("\"iching_cast\":null"),
        "re-serialized JSON must NOT contain iching_cast:null; got: {re_serialized}"
    );
    assert!(
        !re_serialized.contains("\"direction_cross_link\":null"),
        "re-serialized JSON must NOT contain direction_cross_link:null; got: {re_serialized}"
    );
    assert!(
        !re_serialized.contains("\"iching_cast\""),
        "re-serialized JSON must NOT contain iching_cast when None; got: {re_serialized}"
    );
    assert!(
        !re_serialized.contains("\"direction_cross_link\""),
        "re-serialized JSON must NOT contain direction_cross_link when None; got: {re_serialized}"
    );
}

// ---------------------------------------------------------------------------
// Test 11 — populated v1.7 iching_cast + direction_cross_link byte-equal round-trip
// ---------------------------------------------------------------------------
//
// Mirrors Phase 19-03 Test 8 (`offering_refs_byte_equal_round_trip`) plus the
// v1.7 field-shape discipline: an explicitly enriched Tết 2026 snapshot carries
// BOTH `iching_cast` and `direction_cross_link`, both keys survive a byte-equal
// round-trip, and each summary keeps its source/evidence shape intact.
#[test]
fn v17_iching_cast_and_direction_cross_link_byte_equal_round_trip() {
    let snap = calculate_day_snapshot(17, 2, 2026);
    let query = IChingQuery::from_snapshot(&snap, Some("việc làm".to_string()), 9)
        .expect("IChingQuery::from_snapshot must succeed for a valid snapshot + valid hour");
    let iching_enriched = enrich_day_snapshot_with_iching(&snap, query)
        .expect("enrich_day_snapshot_with_iching must succeed for a valid query");
    let enriched = enrich_day_snapshot_with_direction_cross_link(&iching_enriched, 0)
        .expect("enrich_day_snapshot_with_direction_cross_link must succeed for a valid branch");

    let iching = enriched
        .iching_cast
        .as_ref()
        .expect("iching_cast must be populated after enrichment");
    let cross = enriched
        .direction_cross_link
        .as_ref()
        .expect("direction_cross_link must be populated after enrichment");

    assert!(
        (1..=64).contains(&iching.cast.chu_que.0),
        "iching_cast.cast.chu_que must be a valid King Wen index 1..=64; got {}",
        iching.cast.chu_que.0
    );
    assert!(
        (1..=64).contains(&iching.bien_que.king_wen.0),
        "iching_cast.bien_que.king_wen must be a valid King Wen index 1..=64; got {}",
        iching.bien_que.king_wen.0
    );
    assert!(
        (1..=6).contains(&iching.cast.dong_hao),
        "iching_cast.cast.dong_hao must be in 1..=6; got {}",
        iching.cast.dong_hao
    );
    assert!(
        (1..=6).contains(&iching.moving_line),
        "iching_cast.moving_line must be in 1..=6; got {}",
        iching.moving_line
    );
    assert!(
        !iching.chu_hexagram_vi_name.is_empty(),
        "iching_cast.chu_hexagram_vi_name must be non-empty"
    );
    assert!(
        !iching.bien_hexagram_vi_name.is_empty(),
        "iching_cast.bien_hexagram_vi_name must be non-empty"
    );

    let primitive_source_ids: std::collections::HashSet<&str> = iching
        .evidence
        .iter()
        .filter(|e| e.source_id != "rule.composite.iching_consultation")
        .map(|e| e.source_id.as_str())
        .collect();
    assert!(
        primitive_source_ids.contains(SOURCE_MAI_HOA_DICH_SO),
        "iching_cast.evidence must include SOURCE_MAI_HOA_DICH_SO primitive source_id"
    );
    assert!(
        primitive_source_ids.contains(SOURCE_KINH_DICH),
        "iching_cast.evidence must include SOURCE_KINH_DICH primitive source_id"
    );
    assert!(
        primitive_source_ids.len() >= 2,
        "iching_cast.evidence must contain at least 2 distinct primitive source_ids; got {:?}",
        primitive_source_ids
    );
    let composite_count = iching
        .evidence
        .iter()
        .filter(|e| e.source_id == "rule.composite.iching_consultation")
        .count();
    assert_eq!(
        composite_count, 1,
        "iching_cast.evidence must contain EXACTLY 1 composite envelope; got {}",
        composite_count
    );

    assert_eq!(
        cross.birth_chi_index, 0,
        "direction_cross_link.birth_chi_index must survive the requested personal branch"
    );
    assert_eq!(
        cross.day_chi_index as usize, enriched.context.canchi.day.chi_index,
        "direction_cross_link.day_chi_index must match the snapshot day chi"
    );
    assert_eq!(
        cross.cells.len(),
        8,
        "direction_cross_link must carry the locked eight-direction cell surface"
    );
    assert!(
        !cross.summary_vi.is_empty(),
        "direction_cross_link.summary_vi must be non-empty"
    );
    let cross_source_ids: std::collections::HashSet<&str> = cross
        .evidence
        .iter()
        .map(|e| e.source_id.as_str())
        .collect();
    assert!(
        cross_source_ids.contains(SOURCE_KHCBPPT),
        "direction_cross_link.evidence must include SOURCE_KHCBPPT; got {:?}",
        cross_source_ids
    );
    assert!(
        cross_source_ids.contains(SOURCE_HUYEN_KHONG),
        "direction_cross_link.evidence must include SOURCE_HUYEN_KHONG; got {:?}",
        cross_source_ids
    );
    assert!(
        cross_source_ids.contains(cross.cross_link_source.as_str()),
        "direction_cross_link.evidence must include its composite cross_link_source; got {:?}",
        cross_source_ids
    );
    assert!(
        cross.cross_link_source.starts_with("rule.composite."),
        "direction_cross_link.cross_link_source must be a composite source id; got {}",
        cross.cross_link_source
    );

    let json = serde_json::to_string(&enriched).expect("serialization failed");
    assert!(
        json.contains("\"iching_cast\""),
        "iching_cast must appear in JSON when Some; got: {json}"
    );
    assert!(
        json.contains("\"direction_cross_link\""),
        "direction_cross_link must appear in JSON when Some; got: {json}"
    );

    let round_tripped: DaySnapshot = serde_json::from_str(&json).expect("deserialization failed");
    let json2 = serde_json::to_string(&round_tripped).expect("re-serialization failed");
    assert_eq!(
        json, json2,
        "v1.7 DaySnapshot round-trip with BOTH new fields populated must be byte-equal"
    );

    let iching_rt = round_tripped
        .iching_cast
        .as_ref()
        .expect("iching_cast must survive round-trip");
    assert_eq!(
        iching_rt.cast.chu_que.0, iching.cast.chu_que.0,
        "iching_cast.cast.chu_que must survive round-trip"
    );
    assert_eq!(
        iching_rt.bien_que.king_wen.0, iching.bien_que.king_wen.0,
        "iching_cast.bien_que.king_wen must survive round-trip"
    );
    assert_eq!(
        iching_rt.moving_line, iching.moving_line,
        "iching_cast.moving_line must survive round-trip"
    );
    assert_eq!(
        iching_rt.question_vi, iching.question_vi,
        "iching_cast.question_vi must survive round-trip"
    );

    let cross_rt = round_tripped
        .direction_cross_link
        .as_ref()
        .expect("direction_cross_link must survive round-trip");
    assert_eq!(
        cross_rt.cross_link_kind, cross.cross_link_kind,
        "direction_cross_link.cross_link_kind must survive round-trip"
    );
    assert_eq!(
        cross_rt.cross_link_source, cross.cross_link_source,
        "direction_cross_link.cross_link_source must survive round-trip"
    );
    assert_eq!(
        cross_rt.cells, cross.cells,
        "direction_cross_link.cells must survive round-trip"
    );

    assert_eq!(
        round_tripped
            .daily_flying_stars
            .as_ref()
            .map(|d| d.center_star as u8),
        enriched
            .daily_flying_stars
            .as_ref()
            .map(|d| d.center_star as u8),
        "daily_flying_stars.center_star must survive round-trip with v1.7 fields added"
    );
    assert_eq!(
        round_tripped.offering_refs, enriched.offering_refs,
        "offering_refs must survive round-trip with v1.7 fields added"
    );
}

// ---------------------------------------------------------------------------
// Test 12 — both new v1.7 fields absent in JSON when None
// ---------------------------------------------------------------------------
//
// Mirrors Phase 19-03 Test 9 (`offering_refs_absent_when_none`). The
// `#[serde(skip_serializing_if = "Option::is_none")]` discipline must be
// honored — when BOTH `iching_cast` AND `direction_cross_link` are None,
// NEITHER key appears in the serialized JSON.
#[test]
fn v17_iching_fields_absent_when_none() {
    let mut snap = calculate_day_snapshot(17, 2, 2026);
    snap.iching_cast = None;
    snap.direction_cross_link = None;

    let json = serde_json::to_string(&snap).expect("serialization failed");
    assert!(
        !json.contains("\"iching_cast\""),
        "iching_cast must NOT appear in JSON when None; got: {json}"
    );
    assert!(
        !json.contains("\"direction_cross_link\""),
        "direction_cross_link must NOT appear in JSON when None; got: {json}"
    );
    assert!(
        !json.contains("\"iching_cast\":null"),
        "iching_cast must NOT serialize as null when None; got: {json}"
    );
    assert!(
        !json.contains("\"direction_cross_link\":null"),
        "direction_cross_link must NOT serialize as null when None; got: {json}"
    );
}
