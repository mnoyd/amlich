/// Tam Tai (三災 / 三殺) — Three Calamities
///
/// A 3-year affliction period recurring every 12 years. Based on Tam Hợp (三合) triads
/// and directional opposition: each triad encounters Tam Tai in the 3 years of the
/// opposite directional group.
///
/// **Source:** KHCBPPT (三殺) + Vietnamese adaptation
/// **Decision:** DEC-0021

use serde::{Deserialize, Serialize};

use super::types::RuleEvidence;
use crate::types::CHI;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TamTaiSeverity {
    /// Year 1 — entering calamity (lightest)
    Nhap,
    /// Year 2 — residing in calamity (heaviest)
    Cu,
    /// Year 3 — exiting calamity (recovering)
    Xuat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TamTaiResult {
    pub in_tam_tai: bool,
    /// 1-based position within the 3-year cycle (1=nhập, 2=cư, 3=xuất)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year_position: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<TamTaiSeverity>,
    /// The Tam Hợp group the birth year belongs to (3 branch names)
    pub tam_hop_group: Vec<String>,
    /// The 3 calamity years (branch names)
    pub tai_years: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<RuleEvidence>,
}

/// Tam Hợp triads by chi_index. Each triad shares the same Tam Tai years.
/// Group 0 (Water): Thân(8), Tý(0), Thìn(4)
/// Group 1 (Wood):  Hợi(11), Mão(3), Mùi(7)
/// Group 2 (Fire):  Dần(2), Ngọ(6), Tuất(10)
/// Group 3 (Metal): Tỵ(5), Dậu(9), Sửu(1)
const TAM_HOP_TRIADS: [[usize; 3]; 4] = [
    [8, 0, 4],   // Water: Thân Tý Thìn
    [11, 3, 7],  // Wood:  Hợi Mão Mùi
    [2, 6, 10],  // Fire:  Dần Ngọ Tuất
    [5, 9, 1],   // Metal: Tỵ Dậu Sửu
];

/// The 3 calamity years for each Tam Hợp group (opposite directional group).
/// Water triad → East years (Dần Mão Thìn)
/// Wood triad  → South years (Tỵ Ngọ Mùi)
/// Fire triad  → West years (Thân Dậu Tuất)
/// Metal triad → North years (Hợi Tý Sửu)
const TAI_YEARS: [[usize; 3]; 4] = [
    [2, 3, 4],    // East: Dần Mão Thìn
    [5, 6, 7],    // South: Tỵ Ngọ Mùi
    [8, 9, 10],   // West: Thân Dậu Tuất
    [11, 0, 1],   // North: Hợi Tý Sửu
];

/// Find which Tam Hợp group a chi_index belongs to.
fn find_triad_group(chi_index: usize) -> usize {
    for (group_idx, triad) in TAM_HOP_TRIADS.iter().enumerate() {
        if triad.contains(&chi_index) {
            return group_idx;
        }
    }
    unreachable!("chi_index {} not in any triad", chi_index)
}

/// Compute Tam Tai status for a person given their birth year branch and current year branch.
pub fn compute_tam_tai(birth_chi_index: usize, current_year_chi_index: usize) -> TamTaiResult {
    let group = find_triad_group(birth_chi_index);
    let tai = &TAI_YEARS[group];
    let triad = &TAM_HOP_TRIADS[group];

    let tam_hop_group: Vec<String> = triad.iter().map(|&i| CHI[i].to_string()).collect();
    let tai_years: Vec<String> = tai.iter().map(|&i| CHI[i].to_string()).collect();

    let position = tai
        .iter()
        .position(|&y| y == current_year_chi_index)
        .map(|p| (p + 1) as u8);

    let severity = position.map(|p| match p {
        1 => TamTaiSeverity::Nhap,
        2 => TamTaiSeverity::Cu,
        3 => TamTaiSeverity::Xuat,
        _ => unreachable!(),
    });

    TamTaiResult {
        in_tam_tai: position.is_some(),
        year_position: position,
        severity,
        tam_hop_group,
        tai_years,
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn water_triad_tai_in_east_years() {
        // Thân(8) in Water triad → Tam Tai in Dần(2), Mão(3), Thìn(4)
        let r = compute_tam_tai(8, 2);
        assert!(r.in_tam_tai);
        assert_eq!(r.year_position, Some(1));
        assert_eq!(r.severity, Some(TamTaiSeverity::Nhap));

        let r = compute_tam_tai(0, 3); // Tý in Water → Mão is year 2
        assert!(r.in_tam_tai);
        assert_eq!(r.year_position, Some(2));
        assert_eq!(r.severity, Some(TamTaiSeverity::Cu));

        let r = compute_tam_tai(4, 4); // Thìn in Water → Thìn is year 3
        assert!(r.in_tam_tai);
        assert_eq!(r.year_position, Some(3));
        assert_eq!(r.severity, Some(TamTaiSeverity::Xuat));
    }

    #[test]
    fn not_in_tam_tai() {
        // Thân(8) in Water triad → NOT in Tam Tai during Ngọ(6) year
        let r = compute_tam_tai(8, 6);
        assert!(!r.in_tam_tai);
        assert_eq!(r.year_position, None);
        assert_eq!(r.severity, None);
    }

    #[test]
    fn all_four_groups_covered() {
        // Fire triad: Dần(2) → Tam Tai in Thân(8), Dậu(9), Tuất(10)
        let r = compute_tam_tai(2, 9);
        assert!(r.in_tam_tai);
        assert_eq!(r.year_position, Some(2));

        // Metal triad: Tỵ(5) → Tam Tai in Hợi(11), Tý(0), Sửu(1)
        let r = compute_tam_tai(5, 11);
        assert!(r.in_tam_tai);
        assert_eq!(r.year_position, Some(1));

        // Wood triad: Mão(3) → Tam Tai in Tỵ(5), Ngọ(6), Mùi(7)
        let r = compute_tam_tai(3, 7);
        assert!(r.in_tam_tai);
        assert_eq!(r.year_position, Some(3));
    }

    #[test]
    fn result_contains_group_names() {
        let r = compute_tam_tai(0, 2); // Tý in Water triad
        assert_eq!(r.tam_hop_group, vec!["Thân", "Tý", "Thìn"]);
        assert_eq!(r.tai_years, vec!["Dần", "Mão", "Thìn"]);
    }

    #[test]
    fn evidence_defaults_to_none() {
        let r = compute_tam_tai(0, 0);
        assert!(r.evidence.is_none());
    }
}
