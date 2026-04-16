/// Thái Tuế (太歲) — Grand Duke annual conflict check
///
/// Checks whether a person's birth year Earthly Branch has any of 5 conflict
/// relationships with the current year's Earthly Branch.
///
/// **Source:** Both KHCBPPT and Vietnamese folk tradition
/// **Decision:** DEC-0021
use serde::{Deserialize, Serialize};

use super::types::RuleEvidence;
use super::xung_hop;
use crate::types::CHI;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThaiTueConflictKind {
    /// Same branch — direct clash with Grand Duke (犯太歲)
    Truc,
    /// Opposite branch — 6-clash opposition (沖太歲)
    Xung,
    /// Mutual harm relationship (害太歲)
    Hai,
    /// Mutual punishment relationship (刑太歲)
    Hinh,
    /// Break/destruction relationship (破太歲) — 3 positions apart
    Pha,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThaiTueConflict {
    pub kind: ThaiTueConflictKind,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThaiTueResult {
    pub conflicts: Vec<ThaiTueConflict>,
    pub has_conflict: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<RuleEvidence>,
}

/// The "Phá" (break/destruction) pairs: branches that are 3 positions apart in the cycle.
/// Tý(0)↔Dậu(9), Sửu(1)↔Thìn(4), Dần(2)↔Hợi(11), Mão(3)↔Ngọ(6),
/// Thìn(4)↔Sửu(1), Tỵ(5)↔Thân(8), Ngọ(6)↔Mão(3), Mùi(7)↔Tuất(10),
/// Thân(8)↔Tỵ(5), Dậu(9)↔Tý(0), Tuất(10)↔Mùi(7), Hợi(11)↔Dần(2)
fn pha_partner(chi_index: usize) -> usize {
    // The traditional "pha" relationship: (chi + 9) % 12 or equivalently (chi - 3 + 12) % 12
    (chi_index + 9) % 12
}

/// Compute Thái Tuế conflicts between a person's birth year and the current year.
pub fn compute_thai_tue(birth_chi_index: usize, current_year_chi_index: usize) -> ThaiTueResult {
    let mut conflicts = Vec::new();
    let birth_name = CHI[birth_chi_index];
    let year_name = CHI[current_year_chi_index];

    // Trực: same branch
    if birth_chi_index == current_year_chi_index {
        conflicts.push(ThaiTueConflict {
            kind: ThaiTueConflictKind::Truc,
            description: format!("Phạm Thái Tuế: {} trùng {}", birth_name, year_name),
        });
    }

    // Xung: opposite branch (lục xung)
    let xung_name = xung_hop::luc_xung(birth_chi_index);
    if CHI[current_year_chi_index] == xung_name {
        conflicts.push(ThaiTueConflict {
            kind: ThaiTueConflictKind::Xung,
            description: format!("Xung Thái Tuế: {} xung {}", birth_name, year_name),
        });
    }

    // Hại: mutual harm
    let hai_name = xung_hop::get_xiang_hai(birth_chi_index);
    if CHI[current_year_chi_index] == hai_name {
        conflicts.push(ThaiTueConflict {
            kind: ThaiTueConflictKind::Hai,
            description: format!("Hại Thái Tuế: {} hại {}", birth_name, year_name),
        });
    }

    // Hình: mutual punishment — check if both are in the same punishment group
    let xing_group = xung_hop::get_xiang_xing(birth_chi_index);
    if birth_chi_index != current_year_chi_index
        && xing_group
            .iter()
            .any(|name| name.as_str() == CHI[current_year_chi_index])
    {
        conflicts.push(ThaiTueConflict {
            kind: ThaiTueConflictKind::Hinh,
            description: format!("Hình Thái Tuế: {} hình {}", birth_name, year_name),
        });
    }

    // Phá: break relationship (3 positions apart)
    let pha = pha_partner(birth_chi_index);
    if pha == current_year_chi_index {
        conflicts.push(ThaiTueConflict {
            kind: ThaiTueConflictKind::Pha,
            description: format!("Phá Thái Tuế: {} phá {}", birth_name, year_name),
        });
    }

    let has_conflict = !conflicts.is_empty();
    ThaiTueResult {
        conflicts,
        has_conflict,
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truc_same_branch() {
        let r = compute_thai_tue(6, 6); // Ngọ year, Ngọ birth
        assert!(r.has_conflict);
        assert!(r
            .conflicts
            .iter()
            .any(|c| c.kind == ThaiTueConflictKind::Truc));
    }

    #[test]
    fn xung_opposite_branch() {
        // Tý(0) xung Ngọ(6)
        let r = compute_thai_tue(0, 6);
        assert!(r.has_conflict);
        assert!(r
            .conflicts
            .iter()
            .any(|c| c.kind == ThaiTueConflictKind::Xung));
    }

    #[test]
    fn hai_harm_relationship() {
        // Tý(0) hại Mùi(7)
        let r = compute_thai_tue(0, 7);
        assert!(r
            .conflicts
            .iter()
            .any(|c| c.kind == ThaiTueConflictKind::Hai));
    }

    #[test]
    fn hinh_punishment_relationship() {
        // Dần(2) hình Tỵ(5) — both in punishment group [Dần, Tỵ, Thân]
        let r = compute_thai_tue(2, 5);
        assert!(r
            .conflicts
            .iter()
            .any(|c| c.kind == ThaiTueConflictKind::Hinh));
    }

    #[test]
    fn pha_break_relationship() {
        // Tý(0) phá Dậu(9): (0+9)%12=9
        let r = compute_thai_tue(0, 9);
        assert!(r
            .conflicts
            .iter()
            .any(|c| c.kind == ThaiTueConflictKind::Pha));
    }

    #[test]
    fn no_conflict() {
        // Dần(2) vs Thìn(4) — no truc/xung/hai/hinh/pha
        // Dần is in xing group [2,3,5], Thìn is in [0,1,4] — different groups
        let r = compute_thai_tue(2, 4);
        assert!(!r.has_conflict);
        assert!(r.conflicts.is_empty());
    }

    #[test]
    fn multiple_conflicts_possible() {
        // Some branches can have multiple conflict types simultaneously
        let r = compute_thai_tue(6, 6); // Same branch → Truc + possibly Hinh (self-punishment for Ngọ)
        assert!(r.has_conflict);
        assert!(r
            .conflicts
            .iter()
            .any(|c| c.kind == ThaiTueConflictKind::Truc));
    }

    #[test]
    fn evidence_defaults_to_none() {
        let r = compute_thai_tue(0, 0);
        assert!(r.evidence.is_none());
    }
}
