//! v1.11 Point-Opening Context — disclaimer v2 (bilingual historical
//! procedural citation).
//!
//! The exact strings here are byte-identical to
//! `.planning/milestones/v1.11-phases/01-najia-table-freeze/REVIEWER-PACK.md`
//! §A.4.1 (Vietnamese) and §A.4.2 (English) and to the
//! `disclaimer_v2_draft` block of the frozen corpus
//! (`crates/amlich-core/data/ty-ngo-luu-chu/najia-open-points.json`);
//! `tests/point_opening_contract_guard.rs` enforces both byte
//! equalities.
//!
//! Status: **draft pending Gate 4** (product/legal sign-off). The
//! identifier is the contract clients must honor once the surfaces
//! render it (bead `amlich-xlag.2.3`); the text must not change without
//! product/legal re-sign-off.
//!
//! Reuses the v1.10 [`DisclaimerId`] / [`LocalizedDisclaimer`]
//! *primitive newtypes* (shared wire vocabulary), not any v1.10
//! Traditional Wellness Context value — the two disclaimer ids and
//! payloads are distinct and never interchangeable.

use crate::traditional_wellness::disclaimer::{DisclaimerId, LocalizedDisclaimer};

/// Stable identifier for disclaimer v2. Proposed by the Track-1 freeze;
/// any serializer that exposes the Point-Opening Context must render
/// this string or its localized equivalent.
pub fn disclaimer_id_historical_procedural_citation() -> DisclaimerId {
    DisclaimerId::new("historical_procedural_citation_v1")
}

/// Backwards-friendly alias mirroring the v1.10 naming convention.
pub const DISCLAIMER_ID_HISTORICAL_PROCEDURAL_CITATION_STR: &str =
    "historical_procedural_citation_v1";

/// Vietnamese disclaimer v2 text. Byte-identical to REVIEWER-PACK §A.4.1
/// and to the corpus `disclaimer_v2_draft.vi`. Includes a U+2013 EN DASH
/// between "văn hóa" and "lịch sử" — do not normalize to a hyphen.
pub const DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN: &str = "Trích dẫn thuật ngữ y học cổ truyền từ văn bản Châm Cứu Đại Thành; chỉ mang tính văn hóa – lịch sử. Đây không phải hướng dẫn châm, bấm, cứu hay tự điều trị tại bất kỳ thời điểm nào. Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn.";

/// English disclaimer v2 text. Byte-identical to REVIEWER-PACK §A.4.2
/// and to the corpus `disclaimer_v2_draft.en`.
pub const DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN: &str = "Citations of classical acupuncture terminology from Zhenjiu Dacheng; provided as historical and cultural information only. This is not instruction or encouragement to needle, press, moxibust, or self-treat at any time. Do not use it to delay or replace care from a qualified health professional.";

/// Single source of truth for the disclaimer v2 payload emitted by every
/// Point-Opening Context serializer.
pub fn historical_procedural_citation_disclaimer() -> LocalizedDisclaimer {
    LocalizedDisclaimer {
        id: disclaimer_id_historical_procedural_citation(),
        vi: DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN.to_string(),
        en: DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disclaimer_id_serializes_as_inner_string() {
        let id = disclaimer_id_historical_procedural_citation();
        assert_eq!(
            id.as_str(),
            DISCLAIMER_ID_HISTORICAL_PROCEDURAL_CITATION_STR
        );
        assert_eq!(
            serde_json::to_string(&id).unwrap(),
            "\"historical_procedural_citation_v1\""
        );
        let roundtrip: DisclaimerId =
            serde_json::from_str("\"historical_procedural_citation_v1\"").unwrap();
        assert_eq!(roundtrip, id);
    }

    #[test]
    fn localized_disclaimer_round_trip_byte_equal() {
        let original = historical_procedural_citation_disclaimer();
        let json = serde_json::to_string(&original).unwrap();
        let recovered: LocalizedDisclaimer = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered, original);
        let json2 = serde_json::to_string(&recovered).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn vietnamese_text_preserves_en_dash_not_hyphen() {
        assert!(DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN.contains('\u{2013}'));
        assert!(!DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN.contains("văn hóa-lịch sử"));
    }

    #[test]
    fn disclaimer_wording_stays_negation_not_instruction() {
        // The clinical verbs may appear ONLY inside the negation frames
        // ("không phải hướng dẫn …", "not instruction or encouragement
        // to …"). The imperative forms are forbidden lexemes policed by
        // the extended prohibited-language guard.
        assert!(DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN.contains("không phải hướng dẫn"));
        assert!(DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN.contains("not instruction"));
    }

    #[test]
    fn v2_id_is_distinct_from_the_v1_10_cultural_information_id() {
        // Explicit separation from v1.10 Traditional Wellness Context:
        // the two disclaimer identifiers must never collide, or a
        // client could render the weaker v1.10 text for point-opening
        // output.
        assert_ne!(
            DISCLAIMER_ID_HISTORICAL_PROCEDURAL_CITATION_STR,
            crate::traditional_wellness::DISCLAIMER_ID_CULTURAL_INFORMATION_STR
        );
        assert_ne!(
            historical_procedural_citation_disclaimer(),
            crate::traditional_wellness::cultural_information_disclaimer()
        );
    }
}
