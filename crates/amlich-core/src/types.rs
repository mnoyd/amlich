/**
 * Core Type Definitions and Constants for Vietnamese Lunar Calendar
 *
 * Conventions:
 * - Indices are 0-based
 * - Timezone: UTC+7 (Vietnam)
 * - Lunar month 1 branch = Dần (index 2)
 */

// Thiên Can (Heavenly Stems) - 10 elements
pub const CAN: [&str; 10] = [
    "Giáp", "Ất", "Bính", "Đinh", "Mậu", "Kỷ", "Canh", "Tân", "Nhâm", "Quý",
];

// Địa Chi (Earthly Branches) - 12 elements
pub const CHI: [&str; 12] = [
    "Tý", "Sửu", "Dần", "Mão", "Thìn", "Tỵ", "Ngọ", "Mùi", "Thân", "Dậu", "Tuất", "Hợi",
];

// Vietnamese zodiac animals (Con Giáp) - aligned with CHI
pub const CON_GIAP: [&str; 12] = [
    "Tý (Chuột)",
    "Sửu (Trâu)",
    "Dần (Hổ)",
    "Mão (Mèo)",
    "Thìn (Rồng)",
    "Tỵ (Rắn)",
    "Ngọ (Ngựa)",
    "Mùi (Dê)",
    "Thân (Khỉ)",
    "Dậu (Gà)",
    "Tuất (Chó)",
    "Hợi (Lợn)",
];

// Ngũ Hành (Five Elements) - aligned with CAN (2 stems per element)
pub const NGU_HANH_CAN: [&str; 10] = [
    "Mộc", "Mộc", "Hỏa", "Hỏa", "Thổ", "Thổ", "Kim", "Kim", "Thủy", "Thủy",
];

// Ngũ Hành for CHI
pub const NGU_HANH_CHI: [&str; 12] = [
    "Thủy", "Thổ", "Mộc", "Mộc", "Thổ", "Hỏa", "Hỏa", "Thổ", "Kim", "Kim", "Thổ", "Thủy",
];

// Days of week (Vietnamese)
pub const THU: [&str; 7] = [
    "Chủ Nhật",
    "Thứ Hai",
    "Thứ Ba",
    "Thứ Tư",
    "Thứ Năm",
    "Thứ Sáu",
    "Thứ Bảy",
];

/// Represents the Ngũ Hành (Five Elements) for both Can and Chi
#[derive(Debug, Clone, PartialEq)]
pub struct NguHanh {
    pub can: String,
    pub chi: String,
}

/// Represents a Can Chi combination with full information
#[derive(Debug, Clone, PartialEq)]
pub struct CanChi {
    pub can_index: usize,
    pub chi_index: usize,
    pub can: String,
    pub chi: String,
    pub full: String,
    pub con_giap: String,
    pub ngu_hanh: NguHanh,
    pub sexagenary_index: usize,
}

impl CanChi {
    /// Create a new CanChi object with full information
    ///
    /// # Arguments
    /// * `can_index` - Stem index (0-9)
    /// * `chi_index` - Branch index (0-11)
    ///
    /// # Returns
    /// A CanChi struct with all information populated
    pub fn new(can_index: usize, chi_index: usize) -> Self {
        let can_idx = can_index % 10;
        let chi_idx = chi_index % 12;

        // Calculate sexagenary cycle index (0-59)
        let sexagenary_index = ((can_idx * 6) + (chi_idx / 2)) % 60;

        let can = CAN[can_idx].to_string();
        let chi = CHI[chi_idx].to_string();
        let full = format!("{} {}", can, chi);
        let con_giap = CON_GIAP[chi_idx].to_string();
        let ngu_hanh = NguHanh {
            can: NGU_HANH_CAN[can_idx].to_string(),
            chi: NGU_HANH_CHI[chi_idx].to_string(),
        };

        CanChi {
            can_index: can_idx,
            chi_index: chi_idx,
            can,
            chi,
            full,
            con_giap,
            ngu_hanh,
            sexagenary_index,
        }
    }
}

/// Normalize an index to 0-based range
///
/// # Arguments
/// * `value` - Value to normalize
/// * `modulo` - Modulo value (10 for Can, 12 for Chi)
///
/// # Returns
/// Normalized index in range [0, modulo)
pub fn normalize_index(value: i32, modulo: i32) -> usize {
    (((value % modulo) + modulo) % modulo) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_length() {
        assert_eq!(CAN.len(), 10);
        assert_eq!(CHI.len(), 12);
        assert_eq!(CON_GIAP.len(), 12);
        assert_eq!(NGU_HANH_CAN.len(), 10);
        assert_eq!(NGU_HANH_CHI.len(), 12);
        assert_eq!(THU.len(), 7);
    }

    #[test]
    fn test_canchi_creation() {
        // Test Giáp Tý (first of 60-year cycle)
        let cc = CanChi::new(0, 0);
        assert_eq!(cc.can, "Giáp");
        assert_eq!(cc.chi, "Tý");
        assert_eq!(cc.full, "Giáp Tý");
        assert_eq!(cc.con_giap, "Tý (Chuột)");
        assert_eq!(cc.ngu_hanh.can, "Mộc");
        assert_eq!(cc.ngu_hanh.chi, "Thủy");
        assert_eq!(cc.sexagenary_index, 0);
    }

    #[test]
    fn test_canchi_giap_thin() {
        // Tết 2024: Giáp Thìn
        let cc = CanChi::new(0, 4);
        assert_eq!(cc.can, "Giáp");
        assert_eq!(cc.chi, "Thìn");
        assert_eq!(cc.full, "Giáp Thìn");
        assert_eq!(cc.con_giap, "Thìn (Rồng)");
        assert_eq!(cc.ngu_hanh.can, "Mộc");
        assert_eq!(cc.ngu_hanh.chi, "Thổ");
    }

    #[test]
    fn test_canchi_modulo_wrapping() {
        // Test that indices wrap correctly
        let cc1 = CanChi::new(10, 12); // Should wrap to 0, 0
        assert_eq!(cc1.can_index, 0);
        assert_eq!(cc1.chi_index, 0);

        let cc2 = CanChi::new(13, 25); // Should wrap to 3, 1
        assert_eq!(cc2.can_index, 3);
        assert_eq!(cc2.chi_index, 1);
    }

    #[test]
    fn test_normalize_index() {
        assert_eq!(normalize_index(0, 10), 0);
        assert_eq!(normalize_index(10, 10), 0);
        assert_eq!(normalize_index(13, 10), 3);
        assert_eq!(normalize_index(-1, 10), 9);
        assert_eq!(normalize_index(-11, 10), 9);

        assert_eq!(normalize_index(0, 12), 0);
        assert_eq!(normalize_index(12, 12), 0);
        assert_eq!(normalize_index(-1, 12), 11);
    }

    #[test]
    fn test_sexagenary_cycle() {
        // The 60-year cycle: Giáp Tý starts at 0
        let cc0 = CanChi::new(0, 0);
        assert_eq!(cc0.sexagenary_index, 0);

        // Ất Sửu should be index 6 (not all combinations are valid in 60-year cycle)
        let cc1 = CanChi::new(1, 1);
        assert_eq!(cc1.sexagenary_index, 6);
    }
}
