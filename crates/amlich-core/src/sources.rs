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
    }
}
