//! XLK-01 — Evidence backfill backward-compatibility round-trip tests.
//!
//! Phase 23 Plan 23-01 ships two KHCBPPT-evidence backfills on the existing
//! almanac primitives:
//!   * `compute_thai_tue(...)` — `ThaiTueResult.evidence: None → Some(KHCBPPT)`
//!   * `get_sat_phuong(...)`   — `SatPhuongResult.evidence: None → Some(KHCBPPT)`
//!
//! This test module verifies three properties, mirroring the v1.5 / v1.6
//! `day_snapshot_v14_compat.rs` round-trip discipline:
//!
//!   1. `legacy_*_json_without_evidence_deserializes_to_none`
//!      — A v1.6 JSON object lacking the `evidence` key still deserialises
//!      cleanly because the field is `Option<RuleEvidence>` and serde
//!      defaults absent optionals to `None`. This is the BC guarantee for
//!      any external consumer that persisted v1.6 outputs.
//!
//!   2. `populated_*_result_round_trips_byte_equal`
//!      — A v1.7-populated result (now carrying KHCBPPT evidence) serialises
//!      → deserialises → re-serialises to byte-equal JSON, AND the
//!      populated `evidence.source_id` equals `SOURCE_KHCBPPT` ("khcbppt").
//!
//!   3. `backfill_preserves_*`
//!      — Cross-checks proving the backfills do not change the pre-existing
//!      behavior (`has_conflict`, conflict kinds, day-chi `direction`).
//!
//! Imports via `use amlich_core::...` as an external consumer would (black-box).

use amlich_core::almanac::sat_phuong::{get_sat_phuong, SatPhuongResult};
use amlich_core::almanac::thai_tue::{compute_thai_tue, ThaiTueConflictKind, ThaiTueResult};
use amlich_core::sources::SOURCE_KHCBPPT;

// ---------------------------------------------------------------------------
// ThaiTueResult — backward-compatibility + populated round-trip
// ---------------------------------------------------------------------------

/// A legacy v1.6 JSON object for `ThaiTueResult` that does NOT carry the
/// `evidence` key must still deserialize cleanly into the v1.7 struct, and
/// `evidence` must default to `None`.
#[test]
fn legacy_thai_tue_json_without_evidence_deserializes_to_none() {
    let legacy_json = r#"{
        "conflicts": [
            {"kind":"truc","description":"Phạm Thái Tuế: Tý trùng Tý"}
        ],
        "has_conflict": true
    }"#;
    let decoded: ThaiTueResult =
        serde_json::from_str(legacy_json).expect("legacy v1.6 JSON must deserialize");
    assert!(
        decoded.evidence.is_none(),
        "evidence must default to None when absent from JSON"
    );
    assert!(decoded.has_conflict);
    assert_eq!(decoded.conflicts.len(), 1);
    assert!(matches!(
        decoded.conflicts[0].kind,
        ThaiTueConflictKind::Truc
    ));
}

/// A v1.7-populated `ThaiTueResult` (now carrying KHCBPPT evidence) must
/// serialise → deserialise → re-serialise to byte-equal JSON, AND the
/// `evidence.source_id` must equal `SOURCE_KHCBPPT` ("khcbppt").
#[test]
fn populated_thai_tue_result_round_trips_byte_equal() {
    let current = compute_thai_tue(0, 0); // Tý birth + Tý year → Trực conflict
    let evidence = current
        .evidence
        .as_ref()
        .expect("compute_thai_tue(0, 0) must now populate evidence with KHCBPPT source_id");
    assert_eq!(
        evidence.source_id, SOURCE_KHCBPPT,
        "compute_thai_tue evidence.source_id must equal SOURCE_KHCBPPT (XLK-01 backfill)"
    );
    assert!(
        !evidence.method.is_empty(),
        "compute_thai_tue evidence.method must be non-empty"
    );
    assert!(
        !evidence.profile.is_empty(),
        "compute_thai_tue evidence.profile must be non-empty"
    );

    let json = serde_json::to_string(&current).expect("serialize ThaiTueResult");
    let round_tripped: ThaiTueResult =
        serde_json::from_str(&json).expect("deserialize ThaiTueResult");
    let json2 = serde_json::to_string(&round_tripped).expect("re-serialize ThaiTueResult");
    assert_eq!(
        json, json2,
        "ThaiTueResult round-trip must be byte-equal after the XLK-01 backfill"
    );
    assert_eq!(
        round_tripped.evidence, current.evidence,
        "ThaiTueResult.evidence must survive the round-trip"
    );
}

// ---------------------------------------------------------------------------
// SatPhuongResult — backward-compatibility + populated round-trip
// ---------------------------------------------------------------------------

/// A legacy v1.6 JSON object for `SatPhuongResult` that does NOT carry the
/// `evidence` key must still deserialize cleanly, with `evidence` defaulting
/// to `None`.
#[test]
fn legacy_sat_phuong_json_without_evidence_deserializes_to_none() {
    let legacy_json = r#"{"direction":"Nam"}"#;
    let decoded: SatPhuongResult =
        serde_json::from_str(legacy_json).expect("legacy v1.6 JSON must deserialize");
    assert!(
        decoded.evidence.is_none(),
        "evidence must default to None when absent from JSON"
    );
    assert_eq!(decoded.direction, "Nam");
}

/// A v1.7-populated `SatPhuongResult` must round-trip byte-equal AND carry
/// `SOURCE_KHCBPPT` evidence (XLK-01 backfill).
#[test]
fn populated_sat_phuong_result_round_trips_byte_equal() {
    let current = get_sat_phuong(0); // Tý day → Sát Nam
    let evidence = current
        .evidence
        .as_ref()
        .expect("get_sat_phuong(0) must now populate evidence with KHCBPPT source_id");
    assert_eq!(
        evidence.source_id, SOURCE_KHCBPPT,
        "get_sat_phuong evidence.source_id must equal SOURCE_KHCBPPT (XLK-01 backfill)"
    );
    assert!(
        !evidence.method.is_empty(),
        "get_sat_phuong evidence.method must be non-empty"
    );
    assert!(
        !evidence.profile.is_empty(),
        "get_sat_phuong evidence.profile must be non-empty"
    );

    let json = serde_json::to_string(&current).expect("serialize SatPhuongResult");
    let round_tripped: SatPhuongResult =
        serde_json::from_str(&json).expect("deserialize SatPhuongResult");
    let json2 = serde_json::to_string(&round_tripped).expect("re-serialize SatPhuongResult");
    assert_eq!(
        json, json2,
        "SatPhuongResult round-trip must be byte-equal after the XLK-01 backfill"
    );
    assert_eq!(
        round_tripped.evidence, current.evidence,
        "SatPhuongResult.evidence must survive the round-trip"
    );
}

// ---------------------------------------------------------------------------
// Cross-checks — backfills preserve pre-existing behavior
// ---------------------------------------------------------------------------

/// The Thái Tuế evidence backfill must NOT change the existing conflict
/// detection logic. Tý(0) vs Ngọ(6) still yields the Xung conflict.
#[test]
fn backfill_preserves_compute_thai_tue_conflict_logic() {
    let r = compute_thai_tue(0, 6);
    assert!(r.has_conflict);
    assert!(r
        .conflicts
        .iter()
        .any(|c| matches!(c.kind, ThaiTueConflictKind::Xung)));
    assert!(
        r.evidence.is_some(),
        "evidence must be populated after the XLK-01 backfill"
    );

    // Sanity: a no-conflict pair still has_conflict == false.
    let r_no = compute_thai_tue(2, 4); // Dần vs Thìn — no truc/xung/hai/hinh/pha
    assert!(!r_no.has_conflict);
    assert!(
        r_no.evidence.is_some(),
        "evidence is populated even when there is no conflict (rule-level provenance)"
    );
}

/// The Sát Phương evidence backfill must NOT change the existing day-chi →
/// direction mapping. The 4 triads still map to Nam / Tây / Bắc / Đông for
/// Tý / Mão / Ngọ / Dậu respectively, and all 12 branches carry evidence.
#[test]
fn backfill_preserves_get_sat_phuong_day_chi_mapping() {
    // Water triad (Thân, Tý, Thìn) → Sát Nam.
    assert_eq!(get_sat_phuong(0).direction, "Nam");
    assert_eq!(get_sat_phuong(4).direction, "Nam");
    assert_eq!(get_sat_phuong(8).direction, "Nam");
    // Wood triad (Hợi, Mão, Mùi) → Sát Tây.
    assert_eq!(get_sat_phuong(3).direction, "Tây");
    assert_eq!(get_sat_phuong(7).direction, "Tây");
    assert_eq!(get_sat_phuong(11).direction, "Tây");
    // Fire triad (Dần, Ngọ, Tuất) → Sát Bắc.
    assert_eq!(get_sat_phuong(2).direction, "Bắc");
    assert_eq!(get_sat_phuong(6).direction, "Bắc");
    assert_eq!(get_sat_phuong(10).direction, "Bắc");
    // Metal triad (Tỵ, Dậu, Sửu) → Sát Đông.
    assert_eq!(get_sat_phuong(1).direction, "Đông");
    assert_eq!(get_sat_phuong(5).direction, "Đông");
    assert_eq!(get_sat_phuong(9).direction, "Đông");

    for i in 0..12 {
        assert!(
            get_sat_phuong(i).evidence.is_some(),
            "get_sat_phuong({}) evidence must be populated after the XLK-01 backfill",
            i
        );
    }
}
