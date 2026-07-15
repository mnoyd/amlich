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
use amlich_core::DaySnapshot;
use amlich_core::rituals::OfferingRef;
use amlich_core::sources::SOURCE_VN_FOLK_RITUAL;

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
