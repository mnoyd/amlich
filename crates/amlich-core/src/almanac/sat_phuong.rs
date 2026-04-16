/// Sát Phương (煞方) — Killing Direction by day/year branch
///
/// Maps the day's (or year's) Earthly Branch to its killing direction via
/// the Tam Hợp triad grouping → opposite cardinal direction.
///
/// **Source:** KHCBPPT, Quyển 9, Lập Thành (立成)
/// **Decision:** DEC-0018
use serde::{Deserialize, Serialize};

use super::types::RuleEvidence;

/// Sát direction by triad group:
/// Group 0 (chi%4=0): Thân(8), Tý(0), Thìn(4)  → Sát Nam (South)
/// Group 1 (chi%4=1): Tỵ(5), Dậu(9), Sửu(1)    → Sát Đông (East)
/// Group 2 (chi%4=2): Dần(2), Ngọ(6), Tuất(10)   → Sát Bắc (North)
/// Group 3 (chi%4=3): Hợi(11), Mão(3), Mùi(7)    → Sát Tây (West)
///
/// Note: The triad grouping by chi%4 aligns branches into their directional families,
/// then the killing direction is the OPPOSITE cardinal direction.
///
/// However, the standard Tam Hợp mapping uses a different modular grouping
/// than simple chi%4. We use the explicit lookup instead.
const SAT_PHUONG_BY_CHI: [&str; 12] = [
    "Nam",  // Tý(0)  — Water triad → Sát Nam
    "Đông", // Sửu(1) — Metal triad → Sát Đông
    "Bắc",  // Dần(2) — Fire triad  → Sát Bắc
    "Tây",  // Mão(3) — Wood triad  → Sát Tây
    "Nam",  // Thìn(4)— Water triad → Sát Nam
    "Đông", // Tỵ(5)  — Metal triad → Sát Đông
    "Bắc",  // Ngọ(6) — Fire triad  → Sát Bắc
    "Tây",  // Mùi(7) — Wood triad  → Sát Tây
    "Nam",  // Thân(8)— Water triad → Sát Nam
    "Đông", // Dậu(9) — Metal triad → Sát Đông
    "Bắc",  // Tuất(10)— Fire triad → Sát Bắc
    "Tây",  // Hợi(11)— Wood triad  → Sát Tây
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SatPhuongResult {
    pub direction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<RuleEvidence>,
}

/// Get the Sát Phương (killing direction) for a given Earthly Branch.
///
/// # Arguments
/// * `chi_index` — 0-based Earthly Branch index (0=Tý .. 11=Hợi)
pub fn get_sat_phuong(chi_index: usize) -> SatPhuongResult {
    SatPhuongResult {
        direction: SAT_PHUONG_BY_CHI[chi_index].to_string(),
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_triad_sat_south() {
        // Thân(8), Tý(0), Thìn(4) → Sát Nam
        assert_eq!(get_sat_phuong(8).direction, "Nam");
        assert_eq!(get_sat_phuong(0).direction, "Nam");
        assert_eq!(get_sat_phuong(4).direction, "Nam");
    }

    #[test]
    fn metal_triad_sat_east() {
        // Tỵ(5), Dậu(9), Sửu(1) → Sát Đông
        assert_eq!(get_sat_phuong(5).direction, "Đông");
        assert_eq!(get_sat_phuong(9).direction, "Đông");
        assert_eq!(get_sat_phuong(1).direction, "Đông");
    }

    #[test]
    fn fire_triad_sat_north() {
        // Dần(2), Ngọ(6), Tuất(10) → Sát Bắc
        assert_eq!(get_sat_phuong(2).direction, "Bắc");
        assert_eq!(get_sat_phuong(6).direction, "Bắc");
        assert_eq!(get_sat_phuong(10).direction, "Bắc");
    }

    #[test]
    fn wood_triad_sat_west() {
        // Hợi(11), Mão(3), Mùi(7) → Sát Tây
        assert_eq!(get_sat_phuong(11).direction, "Tây");
        assert_eq!(get_sat_phuong(3).direction, "Tây");
        assert_eq!(get_sat_phuong(7).direction, "Tây");
    }

    #[test]
    fn all_12_branches_covered() {
        for i in 0..12 {
            let r = get_sat_phuong(i);
            assert!(!r.direction.is_empty());
        }
    }

    #[test]
    fn evidence_defaults_to_none() {
        assert!(get_sat_phuong(0).evidence.is_none());
    }
}
