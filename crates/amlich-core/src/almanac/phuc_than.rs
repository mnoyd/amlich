/// Phúc Thần (福神) — Fortune God direction by day stem
///
/// Maps the day's Heavenly Stem to the Fortune God's direction.
/// Stems pair by Hợp relationship: Giáp-Kỷ, Ất-Canh, Bính-Tân, Đinh-Nhâm, Mậu-Quý.
///
/// **Source:** KHCBPPT, Quyển 9, Lập Thành (立成)
/// **Decision:** DEC-0018
/// **Mnemonic:** 甲己正北是福神，丙辛西北乾宮存，乙庚坤位戊癸艮，丁壬巽上妙追尋

use serde::{Deserialize, Serialize};

use super::types::RuleEvidence;

/// Direction by stem-pair index (Hợp pairing):
/// Pair 0: Giáp(0)-Kỷ(5) → Bắc (N)
/// Pair 1: Ất(1)-Canh(6) → Tây Nam (SW)
/// Pair 2: Bính(2)-Tân(7) → Tây Bắc (NW)
/// Pair 3: Đinh(3)-Nhâm(8) → Đông Nam (SE)
/// Pair 4: Mậu(4)-Quý(9) → Đông Bắc (NE)
const PHUC_THAN_DIRECTIONS: [&str; 5] = [
    "Bắc",       // Giáp-Kỷ
    "Tây Nam",    // Ất-Canh
    "Tây Bắc",   // Bính-Tân
    "Đông Nam",   // Đinh-Nhâm
    "Đông Bắc",  // Mậu-Quý
];

/// Hợp pairing: can_index maps to pair_index via (can_index % 5)
/// Giáp(0)→0, Ất(1)→1, Bính(2)→2, Đinh(3)→3, Mậu(4)→4,
/// Kỷ(5)→0, Canh(6)→1, Tân(7)→2, Nhâm(8)→3, Quý(9)→4
fn hop_pair_index(can_index: usize) -> usize {
    can_index % 5
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhucThanResult {
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<RuleEvidence>,
}

/// Get the Phúc Thần direction for a given day stem.
///
/// # Arguments
/// * `can_index` — 0-based Heavenly Stem index (0=Giáp .. 9=Quý)
pub fn get_phuc_than(can_index: usize) -> PhucThanResult {
    let pair = hop_pair_index(can_index);
    PhucThanResult {
        direction: PHUC_THAN_DIRECTIONS[pair].to_string(),
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn giap_ky_north() {
        assert_eq!(get_phuc_than(0).direction, "Bắc");  // Giáp
        assert_eq!(get_phuc_than(5).direction, "Bắc");  // Kỷ
    }

    #[test]
    fn at_canh_southwest() {
        assert_eq!(get_phuc_than(1).direction, "Tây Nam"); // Ất
        assert_eq!(get_phuc_than(6).direction, "Tây Nam"); // Canh
    }

    #[test]
    fn binh_tan_northwest() {
        assert_eq!(get_phuc_than(2).direction, "Tây Bắc"); // Bính
        assert_eq!(get_phuc_than(7).direction, "Tây Bắc"); // Tân
    }

    #[test]
    fn dinh_nham_southeast() {
        assert_eq!(get_phuc_than(3).direction, "Đông Nam"); // Đinh
        assert_eq!(get_phuc_than(8).direction, "Đông Nam"); // Nhâm
    }

    #[test]
    fn mau_quy_northeast() {
        assert_eq!(get_phuc_than(4).direction, "Đông Bắc"); // Mậu
        assert_eq!(get_phuc_than(9).direction, "Đông Bắc"); // Quý
    }

    #[test]
    fn evidence_defaults_to_none() {
        assert!(get_phuc_than(0).evidence.is_none());
    }
}
