/// Thần Hướng Domain — Directional Deity Positions
///
/// Provides named accessors for the three directional deities associated
/// with each Heavenly Stem (Can) day:
///
/// - **Tài thần (財神):** Direction to face for wealth.
/// - **Hỷ thần (喜神):** Direction for auspicious events.
/// - **Xuất hành hướng:** Recommended travel direction.
///
/// Mapping source: Khâm Định Hiệp Kỷ Biện Phương Thư (欽定協紀辨方書),
/// bài quyết (甲艮乙坤丙丁兑…).
/// Source tag already applied via `AlmanacData::travel_meta` (Batch 1).
use super::data::baseline_data;
use super::types::TravelDirection;

/// Return the directional-deity information for the given Heavenly Stem name.
///
/// Panics if `can` is not one of the 10 canonical stems — that would indicate
/// a programming error, not a runtime condition.
pub fn get_than_huong(can: &str) -> TravelDirection {
    let rule = baseline_data()
        .travel_by_can
        .get(can)
        .unwrap_or_else(|| panic!("than_huong: unknown can '{can}'"));

    TravelDirection {
        xuat_hanh_huong: rule.xuat_hanh_huong.clone(),
        tai_than: rule.tai_than.clone(),
        hy_than: rule.hy_than.clone(),
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: assert three direction fields at once
    fn check(can: &str, xuat: &str, tai: &str, hy: &str) {
        let r = get_than_huong(can);
        assert_eq!(r.xuat_hanh_huong, xuat, "{can}: xuat_hanh_huong");
        assert_eq!(r.tai_than, tai, "{can}: tai_than");
        assert_eq!(r.hy_than, hy, "{can}: hy_than");
    }

    // One test per stem, asserting all three directions.
    // Values derived from the classical bài quyết cross-checked against
    // 欽定協紀辨方書; data corrected in baseline.json commit 0f29f3f.

    #[test]
    fn giap_directions() {
        check("Giáp", "Đông Nam", "Đông Bắc", "Đông Bắc");
    }

    #[test]
    fn at_directions() {
        check("Ất", "Đông", "Tây Nam", "Tây Bắc");
    }

    #[test]
    fn binh_directions() {
        check("Bính", "Nam", "Tây", "Tây Nam");
    }

    #[test]
    fn dinh_directions() {
        check("Đinh", "Nam", "Tây", "Nam");
    }

    #[test]
    fn mau_directions() {
        check("Mậu", "Đông Bắc", "Bắc", "Đông Nam");
    }

    #[test]
    fn ky_directions() {
        check("Kỷ", "Tây Nam", "Bắc", "Đông Bắc");
    }

    #[test]
    fn canh_directions() {
        check("Canh", "Tây Bắc", "Đông", "Tây Bắc");
    }

    #[test]
    fn tan_directions() {
        check("Tân", "Tây", "Đông", "Tây Nam");
    }

    #[test]
    fn nham_directions() {
        check("Nhâm", "Bắc", "Nam", "Nam");
    }

    #[test]
    fn quy_directions() {
        check("Quý", "Tây", "Nam", "Đông Nam");
    }

    #[test]
    fn all_10_stems_covered() {
        // Every canonical stem must return valid directions without panic
        let stems = [
            "Giáp", "Ất", "Bính", "Đinh", "Mậu", "Kỷ", "Canh", "Tân", "Nhâm", "Quý",
        ];
        for can in stems {
            let r = get_than_huong(can);
            assert!(
                !r.xuat_hanh_huong.is_empty(),
                "{can}: xuat must not be empty"
            );
            assert!(!r.tai_than.is_empty(), "{can}: tai must not be empty");
            assert!(!r.hy_than.is_empty(), "{can}: hy must not be empty");
        }
    }
}
