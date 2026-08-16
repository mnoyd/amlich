//! Canonical source_id constants for all classical traditions in amlich-core.
//!
//! Every `ProvenanceEntry::almanac_rule(source_id, method)` call-site in this
//! crate MUST use one of these constants. Bare string literals are forbidden
//! in `src/` outside this module (enforced by `tests/source_id_guard.rs`).

use serde::{Deserialize, Serialize};
use std::fmt;

/// Khâm Định Hiệp Kỷ Biện Phương Thư — primary Vietnamese almanac reference.
pub const SOURCE_KHCBPPT: &str = "khcbppt";

/// Ngọc Hạp Ký — secondary classical reference for directional compatibility.
pub const SOURCE_NGOC_HAP_KY: &str = "ngoc-hap-ky";

/// Vietnamese folk tradition (Hoàng Ốc and similar).
pub const SOURCE_VN_FOLK: &str = "vn-folk";

/// Cửu Diệu (九曜) — Buddhist/Indian astronomical tradition.
pub const SOURCE_CUU_DIEU: &str = "cuu-dieu";

/// Tam Mệnh Thông Hội — Na Am / sexagenary sound source.
pub const SOURCE_TAM_MENH_THONG_HOI: &str = "tam-menh-thong-hoi";

/// Văn khấn cổ truyền Việt Nam — ritual content corpus (new in v1.5).
pub const SOURCE_VN_FOLK_RITUAL: &str = "vn-folk-ritual";

/// Thẩm Thị Huyền Không Học — Phi Tinh / Flying Stars source (new in v1.5).
pub const SOURCE_HUYEN_KHONG: &str = "huyen-khong";

/// Kinh Dịch (I-Ching) — Ngô Tất Tố hexagram text corpus (new in v1.7).
pub const SOURCE_KINH_DICH: &str = "kinh-dich";

/// Mai Hoa Dịch Số — Thiệu Khang Tiết casting algorithm (new in v1.7).
pub const SOURCE_MAI_HOA_DICH_SO: &str = "mai-hoa-dich-so";

/// Thập nhị kinh nạp địa chi (十二經納地支) — fixed twelve Earthly Branch to
/// Traditional Channel historical association table. New in v1.10 (Phase
/// 01-01, ASSOC-01 / SOURCE-01). Source: Xu Feng, *Zhenjiu Daquan*,
/// volume 5, `論子午流注之法` lines 3–9, section `十二經納地支歌`.
/// **Not** full Tý Ngọ Lưu Chú (`子午流注`) — the latter is reserved for a
/// future, separately reviewed milestone and must never be emitted (see
/// ADR-0003).
pub const SOURCE_SHI_ER_JING_NA_DI_ZHI: &str = "shi-er-jing-na-di-zhi";

/// Hoàng Đế Nội Kinh – Tố Vấn (黃帝內經 · 素問) — four-season cultivation
/// profiles paraphrased from chapter `四氣調神大論篇第二` only. New in
/// v1.10 (Phase 02-01, SEASON-01 / SOURCE-01). Scope is strictly the
/// four seasonal routine paraphrases; the astronomical solar-term
/// computation keeps its existing source and is never retagged as
/// Suwen. The term-to-season join is an Amlich composition emitted as
/// the composite `rule.composite.seasonal_wellness`, never as a
/// primitive source.
pub const SOURCE_HUANGDI_NEIJING_SUWEN: &str = "huangdi-neijing-suwen";

/// Typed identifier for a classical or derived provenance source.
///
/// The transparent serde representation preserves the existing JSON string
/// contract while preventing source identifiers from being confused with
/// unrelated strings inside Rust code.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SourceId(String);

impl SourceId {
    /// Construct a non-empty source identifier.
    pub fn new(value: impl Into<String>) -> Self {
        let value = value.into();
        assert!(!value.is_empty(), "SourceId must not be empty");
        Self(value)
    }

    /// Borrow the source identifier as text.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper and return its string representation.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for SourceId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl From<String> for SourceId {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for SourceId {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl PartialEq<str> for SourceId {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for SourceId {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_constants_have_expected_values() {
        assert_eq!(SOURCE_KHCBPPT, "khcbppt");
        assert_eq!(SOURCE_NGOC_HAP_KY, "ngoc-hap-ky");
        assert_eq!(SOURCE_VN_FOLK, "vn-folk");
        assert_eq!(SOURCE_CUU_DIEU, "cuu-dieu");
        assert_eq!(SOURCE_TAM_MENH_THONG_HOI, "tam-menh-thong-hoi");
        assert_eq!(SOURCE_VN_FOLK_RITUAL, "vn-folk-ritual");
        assert_eq!(SOURCE_HUYEN_KHONG, "huyen-khong");
        // v1.7 (Phase 20-01): IChing + Mai Hoa Dịch Số source IDs (FND-09).
        assert_eq!(SOURCE_KINH_DICH, "kinh-dich");
        assert_eq!(SOURCE_MAI_HOA_DICH_SO, "mai-hoa-dich-so");
        // v1.10 (Phase 01-01): Thập nhị kinh nạp địa chi historical
        // association (ASSOC-01 / SOURCE-01).
        assert_eq!(SOURCE_SHI_ER_JING_NA_DI_ZHI, "shi-er-jing-na-di-zhi");
        // v1.10 (Phase 02-01): Hoàng Đế Nội Kinh – Tố Vấn four-season
        // cultivation paraphrases (SEASON-01 / SOURCE-01).
        assert_eq!(SOURCE_HUANGDI_NEIJING_SUWEN, "huangdi-neijing-suwen");
    }

    #[test]
    fn source_id_is_a_transparent_string_newtype() {
        let source_id = SourceId::new(SOURCE_VN_FOLK_RITUAL);

        assert_eq!(source_id.as_str(), "vn-folk-ritual");
        assert_eq!(source_id.as_ref(), "vn-folk-ritual");
        assert_eq!(source_id.to_string(), "vn-folk-ritual");
        assert_eq!(
            serde_json::to_string(&source_id).unwrap(),
            "\"vn-folk-ritual\""
        );
        assert_eq!(
            serde_json::from_str::<SourceId>("\"vn-folk-ritual\"").unwrap(),
            source_id
        );
    }

    #[test]
    #[should_panic(expected = "SourceId must not be empty")]
    fn source_id_rejects_empty_construction() {
        SourceId::new("");
    }
}
