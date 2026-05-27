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
