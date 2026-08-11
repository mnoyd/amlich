//! v1.10 Tier-0 Traditional Wellness Context — bilingual cultural-information
//! disclaimer.
//!
//! The disclaimer text is the single source of truth for the safety
//! classification that travels with every Traditional Wellness Context
//! surface. The exact strings here are byte-identical to
//! `.planning/milestones/v1.10-phases/01-hour-branch-channel-association/REVIEWER-PACK.md`
//! §A.1 (Vietnamese) and §A.2 (English); the prohibited-language guard
//! (`tests/prohibited_language_guard.rs`) enforces that byte equality.
//!
//! The disclaimer is **stable** — clients are contractually required to
//! render this string (or its localized variant) verbatim whenever the
//! `DisclaimerId("cultural_information_v1")` is exposed.

use serde::{Deserialize, Serialize};

/// Stable identifier for a disclaimer text. Serde-transparent so the
/// serialized form is the inner string, matching the
/// [`crate::sources::SourceId`] newtype discipline.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DisclaimerId(String);

impl DisclaimerId {
    /// Construct a disclaimer identifier from a static string literal.
    pub fn new(value: &'static str) -> Self {
        Self(value.to_string())
    }

    /// Borrow the identifier as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for DisclaimerId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisclaimerId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl From<&'static str> for DisclaimerId {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

/// Stable identifier for the cultural-information disclaimer. Any serializer
/// that exposes the Traditional Wellness Context must render this string or
/// its localized equivalent.
pub fn disclaimer_id_cultural_information() -> DisclaimerId {
    DisclaimerId::new("cultural_information_v1")
}

/// Backwards-compatible alias for the cultural-information disclaimer id.
/// Prefer [`disclaimer_id_cultural_information`] for new call-sites.
pub const DISCLAIMER_ID_CULTURAL_INFORMATION_STR: &str = "cultural_information_v1";

/// Vietnamese disclaimer text. Byte-identical to REVIEWER-PACK.md §A.1.
///
/// Includes a U+2013 EN DASH between "văn hóa" and "lịch sử" — do not
/// normalize to a hyphen.
pub const DISCLAIMER_CULTURAL_INFORMATION_VN: &str = "Thông tin văn hóa–lịch sử về quan niệm dưỡng sinh truyền thống; không phải tư vấn y khoa, chẩn đoán, phòng ngừa hay điều trị. Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn.";

/// English disclaimer text. Byte-identical to REVIEWER-PACK.md §A.2.
pub const DISCLAIMER_CULTURAL_INFORMATION_EN: &str = "Historical and cultural information about a traditional wellness system; not medical advice, diagnosis, prevention, or treatment. Do not use it to delay or replace care from a qualified health professional.";

/// The bilingual payload any Traditional Wellness serializer can emit.
///
/// Carries the stable [`DisclaimerId`] so clients can detect renames and
/// render the correct localized string. The fields serialize as
/// `{ "id": "cultural_information_v1", "vi": "...", "en": "..." }`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalizedDisclaimer {
    pub id: DisclaimerId,
    pub vi: String,
    pub en: String,
}

/// Single source of truth for the cultural-information disclaimer. The
/// returned value owns its strings (rather than borrowing the `pub const`
/// above) so callers can clone and serialize freely.
pub fn cultural_information_disclaimer() -> LocalizedDisclaimer {
    LocalizedDisclaimer {
        id: disclaimer_id_cultural_information(),
        vi: DISCLAIMER_CULTURAL_INFORMATION_VN.to_string(),
        en: DISCLAIMER_CULTURAL_INFORMATION_EN.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn disclaimer_id_serializes_as_inner_string() {
        let id = disclaimer_id_cultural_information();
        assert_eq!(id.as_str(), "cultural_information_v1");
        assert_eq!(
            serde_json::to_string(&id).unwrap(),
            "\"cultural_information_v1\""
        );
        let roundtrip: DisclaimerId =
            serde_json::from_str("\"cultural_information_v1\"").unwrap();
        assert_eq!(roundtrip, id);
    }

    #[test]
    fn localized_disclaimer_round_trip_byte_equal() {
        let original = cultural_information_disclaimer();
        let json = serde_json::to_string(&original).unwrap();
        let recovered: LocalizedDisclaimer = serde_json::from_str(&json).unwrap();
        assert_eq!(recovered, original);

        // Re-serialize and confirm byte equality with the first encoding —
        // this is the additive-serialization round-trip contract that the
        // prohibited-language guard also asserts against the reviewer pack.
        let json2 = serde_json::to_string(&recovered).unwrap();
        assert_eq!(json, json2);
    }

    #[test]
    fn bilingual_text_matches_reviewer_pack_byte_for_byte() {
        // REVIEWER-PACK.md §A.1 (Vietnamese) and §A.2 (English) are the
        // contract surface. The prohibited-language guard re-asserts this
        // from the file side; here we assert it from the source side.
        assert_eq!(
            DISCLAIMER_CULTURAL_INFORMATION_VN,
            "Thông tin văn hóa–lịch sử về quan niệm dưỡng sinh truyền thống; \
             không phải tư vấn y khoa, chẩn đoán, phòng ngừa hay điều trị. \
             Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn."
        );
        assert_eq!(
            DISCLAIMER_CULTURAL_INFORMATION_EN,
            "Historical and cultural information about a traditional wellness system; \
             not medical advice, diagnosis, prevention, or treatment. \
             Do not use it to delay or replace care from a qualified health professional."
        );
    }

    #[test]
    fn vietnamese_text_preserves_en_dash_not_hyphen() {
        // Guard against accidental normalization to a hyphen-minus or
        // em-dash; the reviewer pack uses U+2013 EN DASH exactly.
        assert!(
            DISCLAIMER_CULTURAL_INFORMATION_VN.contains('\u{2013}'),
            "Vietnamese disclaimer must contain U+2013 EN DASH between 'văn hóa' and 'lịch sử'"
        );
        assert!(
            !DISCLAIMER_CULTURAL_INFORMATION_VN.contains("văn hóa-lịch sử"),
            "Vietnamese disclaimer must NOT contain ASCII hyphen between 'văn hóa' and 'lịch sử'"
        );
    }
}
