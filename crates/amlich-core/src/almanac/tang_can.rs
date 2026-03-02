use super::types::TangCan;
use crate::types::CHI;

/// Càng Ngàn (Hidden Stems) for each Địa Chi
///
/// Each Địa Chi contains hidden Heavenly Stems (Càng Ngàn) that influence
/// its astrological properties. Some branches have 1 hidden stem (100% strength),
/// while others have 3 stems with varying strengths.
///
/// Indexed by CHI order: ["Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi"]
pub const CANGAN: [[&str; 3]; 12] = [
    // [Chính, Trung, Dư]
    ["癸", "", ""],     // Tý - Chỉ có Quý
    ["己", "癸", "辛"], // Sửu - Kỷ chính, Quý trung, Tân dư
    ["甲", "丙", "戊"], // Dần - Giáp chính, Bính trung, Mậu dư
    ["乙", "", ""],     // Mão - Chỉ có Ất
    ["戊", "乙", "癸"], // Thìn - Mậu chính, Ất trung, Quý dư
    ["丙", "庚", "戊"], // Tỵ - Bính chính, Canh trung, Mậu dư
    ["丁", "己", ""],   // Ngọ - Đinh chính, Kỷ trung
    ["己", "丁", "乙"], // Mùi - Kỷ chính, Đinh trung, Ất dư
    ["庚", "壬", "戊"], // Thân - Canh chính, Nhâm trung, Mậu dư
    ["辛", "", ""],     // Dậu - Chỉ có Tân
    ["戊", "辛", "丁"], // Tuất - Mậu chính, Tân trung, Đinh dư
    ["壬", "甲", ""],   // Hợi - Nhâm chính, Giáp trung
];

/// Strength values for hidden stems [main, central, residual]
///
/// Strength patterns:
/// - [100, 0, 0]: Single hidden stem (100% strength)
/// - [60, 25, 15]: Three hidden stems with decreasing strength
/// - [70, 30, 0]: Two hidden stems (main + central)
pub const CANGAN_STRENGTH: [[u8; 3]; 12] = [
    [100, 0, 0],  // 子
    [60, 25, 15], // 丑
    [60, 25, 15], // 寅
    [100, 0, 0],  // 卯
    [60, 25, 15], // 辰
    [60, 25, 15], // 巳
    [70, 30, 0],  // 午
    [60, 25, 15], // 未
    [60, 25, 15], // 申
    [100, 0, 0],  // 酉
    [60, 25, 15], // 戌
    [70, 30, 0],  // 亥
];

/// Get Tàng Can (Hidden Stems) for a given Địa Chi
///
/// # Arguments
/// * `chi_name` - Name of the Địa Chi (e.g., "子", "丑", "寅")
///
/// # Returns
/// TangCan struct containing hidden stems and their strength values
///
/// # Panics
/// Panics if `chi_name` is not a valid Địa Chi name
pub fn get_tang_can(chi_name: &str) -> TangCan {
    let chi_idx = CHI
        .iter()
        .position(|&c| c == chi_name)
        .unwrap_or_else(|| panic!("tang_can: unknown chi '{}'", chi_name));

    let stems = CANGAN[chi_idx];
    let strengths = CANGAN_STRENGTH[chi_idx];

    TangCan {
        main: stems[0].to_string(),
        central: stems[1].to_string(),
        residual: stems[2].to_string(),
        strength: strengths,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tang_can_ty_has_only_quy() {
        let tc = get_tang_can("Tý");
        assert_eq!(tc.main, "癸");
        assert_eq!(tc.central, "");
        assert_eq!(tc.residual, "");
        assert_eq!(tc.strength, [100, 0, 0]);
    }

    #[test]
    fn test_tang_can_suu_has_all_three() {
        let tc = get_tang_can("Sửu");
        assert_eq!(tc.main, "己");
        assert_eq!(tc.central, "癸");
        assert_eq!(tc.residual, "辛");
        assert_eq!(tc.strength, [60, 25, 15]);
    }

    #[test]
    fn test_tang_can_strength_values_correct() {
        let tc = get_tang_can("Ngọ");
        assert_eq!(tc.strength, [70, 30, 0]);
    }

    #[test]
    fn test_all_12_branches_have_tang_can() {
        for chi in CHI.iter() {
            let tc = get_tang_can(chi);
            assert!(!tc.main.is_empty(), "{chi}: main should not be empty");
        }
    }

    #[test]
    fn test_tang_can_serializes() {
        let tc = get_tang_can("Sửu");
        let json = serde_json::to_string(&tc).unwrap();
        assert!(json.contains("\"己\""));
        assert!(json.contains("\"癸\""));
        assert!(json.contains("\"辛\""));
    }
}
