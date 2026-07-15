//! Canonical source_id constants for all classical traditions in amlich-core.
//!
//! Every `ProvenanceEntry::almanac_rule(source_id, method)` call-site in this
//! crate MUST use one of these constants. Bare string literals are forbidden
//! in `src/` outside this module (enforced by `tests/source_id_guard.rs`).

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

/// Typed alias for `source_id` fields on semantic-graph / corpus types.
///
/// Zero-cost newtype over `String` introduced in Phase 19-01 to satisfy
/// INT-07's literal "source_id: SourceId" discipline. The underlying
/// representation is still `String` (consistent with `tests/source_id_guard.rs`,
/// which greps for bare-string literals across `src/` — the alias is a
/// transparent type marker, not a semantic constraint). Future phases MAY
/// tighten this into a true newtype that enforces `SOURCE_*` membership at
/// construction, but for now it is documentation-only.
///
/// All call-sites continue to use `pub const SOURCE_*: &str` from this module
/// and `.to_string()` them into `SourceId` values — the discipline is
/// unchanged.
pub type SourceId = String;

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
    }

    #[test]
    fn source_id_alias_is_string() {
        // Phase 19 INT-07: SourceId is a zero-cost newtype over String.
        let s: crate::sources::SourceId = crate::sources::SOURCE_VN_FOLK_RITUAL.to_string();
        let s_str: &str = s.as_str();
        assert_eq!(s_str, "vn-folk-ritual");
    }
}
