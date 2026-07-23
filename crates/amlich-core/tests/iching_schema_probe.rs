//! FND-11 CRIT-1 schema-lock gate: 1-entry serde round-trip probe.
//!
//! This is the **CRIT-1 schema-lock-first gate**. It exercises the locked
//! `HexagramEntry` shape against the trickiest case in the corpus — hexagram
//! #2 Khôn — BEFORE any of the 64 corpus entries are authored in Phase 21.
//!
//! Hexagram #2 Khôn is chosen per 20-RESEARCH.md §"Pitfall 5 / Probe fixture
//! choice" because it exercises ALL of the schema's edge cases simultaneously:
//!   1. **7 `hao_tu` entries** (the "dụng lục" seventh line — unique to
//!      hexagrams #1 Kiền and #2 Khôn; all other hexagrams have 6).
//!   2. **NFC-sensitive Vietnamese diacritics** in `vi_name`, `thoai_tu`, and
//!      `cat_hung` (proves the schema does not mangle precomposed Unicode).
//!   3. **`pending_review: Some(DeferralMarker { ... })`** — proves the
//!      `Option<DeferralMarker>` field round-trips (the v1.6 RIT-14 reuse).
//!   4. **`reviewer` free-text marker** in the
//!      `ExternalReviewPending(reason="..."; expected_review_date="...";
//!      assigned_to="...")` shape — mirrors the Văn khấn corpus pattern.
//!   5. **Reserved `*_en` fields omitted** — proves additive `Option<T>`
//!      discipline deserialises absent keys as `None` (no silent default
//!      value drift).
//!   6. **`#[serde(deny_unknown_fields)]`** active — deserialising a JSON
//!      with a spurious `"bogus_field"` returns `Err`, proving field-name
//!      typos fail during Phase 21 corpus authoring (not silently coerced).
//!
//! This test passing means Phase 21 can author the 64 corpus entries against
//! a frozen schema.

use amlich_core::almanac::fengshui::golden::DeferralMarker;
use amlich_core::iching::{HauThienTrigram, HexagramEntry, KingWenHexagram};

/// Round-trip the trickiest single entry: hexagram #2 Khôn with 7 hao_tu,
/// NFC diacritics, a reviewer free-text marker, and a populated
/// `pending_review`. Asserts field equality + the 7-hao_tu length rule +
/// the DeferralMarker survives serialisation.
#[test]
fn hexagram_entry_one_entry_serde_round_trip() {
    let entry = HexagramEntry {
        king_wen_index: KingWenHexagram::new(2).expect("King Wen #2 must construct"),
        vi_name: "Khôn / Địa".to_string(), // NFC-sensitive diacritics
        vi_name_en: None,                  // reserved — additive Option<T>
        upper_trigram: HauThienTrigram::Khon, // Lo Shu 2 (Pitfall 1 pin)
        lower_trigram: HauThienTrigram::Khon,
        thoai_tu: "Nguyên hanh, lợi mã chi trinh".to_string(),
        thoai_tu_en: None,
        hao_tu: vec![
            "Lý sương, kiên băng chí".to_string(),
            "Trực phương, đại, bất tập vô bất lợi".to_string(),
            "Hàm chương, khả trinh".to_string(),
            "Quát nang, vô cữu vô dự".to_string(),
            "Hoàng thường, nguyên cát".to_string(),
            "Long chiến dã, kỳ huyết huyền hoàng".to_string(),
            "Lợi vĩnh trinh".to_string(), // 7th "dụng lục" — proves length rule
        ],
        hao_tu_en: None,
        cat_hung: "Thuận phục, hanh thông, tốt cho nuôi dưỡng".to_string(), // NFC diacritics
        reviewer: r#"ExternalReviewPending(reason="Ngô Tất Tố source gap for #2 Khôn dụng lục; pending corpus authoring"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer")"#
            .to_string(),
        pending_review: Some(DeferralMarker {
            reason: "probe fixture — Phase 21 corpus will populate".to_string(),
            expected_review_date: "2026-12-31".to_string(),
            assigned_to: None,
        }),
    };

    let json = serde_json::to_string(&entry).expect("serialize");
    let roundtripped: HexagramEntry = serde_json::from_str(&json).expect("deserialize");

    // 1. Field equality on NFC-sensitive strings.
    assert_eq!(entry.vi_name, roundtripped.vi_name, "vi_name round-trip");
    assert_eq!(entry.thoai_tu, roundtripped.thoai_tu, "thoai_tu round-trip");
    assert_eq!(entry.cat_hung, roundtripped.cat_hung, "cat_hung round-trip");

    // 2. King Wen index round-trips as the newtype wrapper.
    assert_eq!(
        entry.king_wen_index, roundtripped.king_wen_index,
        "king_wen_index round-trip"
    );

    // 3. Hậu Thiên trigram round-trips.
    assert_eq!(entry.upper_trigram, roundtripped.upper_trigram);
    assert_eq!(entry.lower_trigram, roundtripped.lower_trigram);

    // 4. The 7-hao_tu rule for hexagrams 1 & 2 (Pitfall 5 fixture choice).
    assert_eq!(
        roundtripped.hao_tu.len(),
        7,
        "7-hao_tu rule for hexagrams 1 & 2 — got {}",
        roundtripped.hao_tu.len()
    );
    assert_eq!(
        entry.hao_tu, roundtripped.hao_tu,
        "hao_tu entries round-trip"
    );

    // 5. `pending_review: Some(DeferralMarker { ... })` survives serialisation
    //    (the v1.6 RIT-14 reuse).
    let pending = roundtripped
        .pending_review
        .as_ref()
        .expect("pending_review must round-trip as Some");
    assert_eq!(
        pending.expected_review_date, "2026-12-31",
        "DeferralMarker.expected_review_date round-trip"
    );
    assert!(
        pending.reason.contains("Phase 21"),
        "DeferralMarker.reason round-trip: {:?}",
        pending.reason
    );
    assert!(
        pending.assigned_to.is_none(),
        "DeferralMarker.assigned_to=None round-trip"
    );

    // 6. Reviewer free-text marker survives verbatim.
    assert!(
        roundtripped.reviewer.contains("ExternalReviewPending("),
        "reviewer marker shape preserved: {:?}",
        roundtripped.reviewer
    );
    assert!(
        roundtripped
            .reviewer
            .contains("expected_review_date=\"2026-12-31\""),
        "reviewer marker sub-fields preserved: {:?}",
        roundtripped.reviewer
    );
}

/// `#[serde(deny_unknown_fields)]` is active: an unknown field in the JSON
/// returns `Err` on deserialise. This is the CRIT-1 gate against silent
/// field-name typos during Phase 21 corpus authoring — a typo'd
/// `"king_wen_ndex"` MUST fail loudly, not coerce to `KingWenHexagram(0)`.
#[test]
fn hexagram_entry_rejects_unknown_fields() {
    // Minimal valid JSON shape plus a spurious `bogus_field`.
    let json = serde_json::json!({
        "king_wen_index": 2,
        "vi_name": "Khôn / Địa",
        "upper_trigram": "khon",
        "lower_trigram": "khon",
        "thoai_tu": "Nguyên hanh, lợi mã chi trinh",
        "hao_tu": ["Lý sương, kiên băng chí"],
        "cat_hung": "thuận phục",
        "reviewer": "ExternalReviewPending(reason=\"x\")",
        "bogus_field": 123
    })
    .to_string();

    let result: Result<HexagramEntry, _> = serde_json::from_str(&json);
    assert!(
        result.is_err(),
        "deny_unknown_fields MUST reject bogus_field; got Ok({:?})",
        result.ok()
    );

    let err_msg = format!("{}", result.unwrap_err());
    assert!(
        err_msg.contains("unknown field") || err_msg.contains("bogus_field"),
        "error should name the unknown field; got: {err_msg}"
    );
}

/// Reserved `*_en` additive `Option<T>` discipline: when the keys are absent
/// from JSON, they deserialise as `None` (no silent default-value drift).
/// Also confirms a complete-but-`*_en`-omitted payload round-trips without
/// the `*_en` keys re-appearing in the serialised output.
#[test]
fn hexagram_entry_reserved_en_fields_absent_round_trip_as_none() {
    let json = serde_json::json!({
        "king_wen_index": 2,
        "vi_name": "Khôn / Địa",
        "upper_trigram": "khon",
        "lower_trigram": "khon",
        "thoai_tu": "Nguyên hanh, lợi mã chi trinh",
        "hao_tu": ["Lý sương, kiên băng chí"],
        "cat_hung": "thuận phục",
        "reviewer": "ExternalReviewPending(reason=\"x\")"
        // NOTE: vi_name_en, thoai_tu_en, hao_tu_en, pending_review all absent.
    })
    .to_string();

    let entry: HexagramEntry = serde_json::from_str(&json).expect("deserialize");

    // All reserved *_en fields default to None when absent.
    assert!(
        entry.vi_name_en.is_none(),
        "vi_name_en must be None when absent"
    );
    assert!(
        entry.thoai_tu_en.is_none(),
        "thoai_tu_en must be None when absent"
    );
    assert!(
        entry.hao_tu_en.is_none(),
        "hao_tu_en must be None when absent"
    );
    assert!(
        entry.pending_review.is_none(),
        "pending_review must be None when absent"
    );

    // Re-serialise: the absent keys MUST NOT re-appear (skip_serializing_if
    // discipline — additive fields stay absent in JSON when None).
    let reserialised = serde_json::to_string(&entry).expect("re-serialize");
    assert!(
        !reserialised.contains("vi_name_en"),
        "vi_name_en must not appear in JSON when None: {reserialised}"
    );
    assert!(
        !reserialised.contains("thoai_tu_en"),
        "thoai_tu_en must not appear in JSON when None: {reserialised}"
    );
    assert!(
        !reserialised.contains("hao_tu_en"),
        "hao_tu_en must not appear in JSON when None: {reserialised}"
    );
    assert!(
        !reserialised.contains("pending_review"),
        "pending_review must not appear in JSON when None: {reserialised}"
    );
}

/// `HauThienTrigram` deserialises from the locked Lo Shu snake_case names
/// (Pitfall 1 pin). Spot-check that the corpus's `upper_trigram` /
/// `lower_trigram` keys accept the canonical Khôn value `"khon"` (NOT a
/// number, NOT a Tiên Thiên name in a different arrangement).
#[test]
fn hau_thien_trigram_deserialises_from_snake_case_name() {
    let json = serde_json::json!("khon").to_string();
    let trigram: HauThienTrigram = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(trigram, HauThienTrigram::Khon);
    assert_eq!(
        trigram as u8, 2,
        "Hậu Thiên Khôn = Lo Shu 2 (Pitfall 1 pin)"
    );
}
