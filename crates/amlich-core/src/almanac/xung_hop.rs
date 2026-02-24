/// Xung/Hợp Domain — Earthly Branch Conflict and Harmony Relations
///
/// Implements the three classical branch-relationship groups used in daily
/// almanac calculations:
///
/// - **Lục xung (六冲):** Each branch clashes with the one 6 positions away.
/// - **Tam hợp (三合):** Three branches form a harmony triad (4 groups).
/// - **Tứ hành xung (四行冲):** Four branches in a mutual-clash square (3 groups).

use crate::types::CHI;
use super::types::XungHopResult;

/// Return the lục-xung (direct-conflict) branch for `chi_index`.
///
/// Each branch clashes with the branch 6 positions ahead in the 12-branch cycle.
pub fn luc_xung(chi_index: usize) -> &'static str {
    CHI[(chi_index + 6) % 12]
}

/// Return the tam-hợp triad for `chi_index` (sorted by branch order).
///
/// The four triads:
/// - Thân(8) · Tý(0) · Thìn(4)  — Water (Thủy)
/// - Tỵ(5)  · Dậu(9) · Sửu(1)  — Metal (Kim)
/// - Dần(2) · Ngọ(6) · Tuất(10) — Fire (Hỏa)
/// - Hợi(11)· Mão(3) · Mùi(7)  — Wood (Mộc)
///
/// Pattern: branches with the same `chi_index % 4` form one triad.
pub fn tam_hop(chi_index: usize) -> [&'static str; 3] {
    let group = chi_index % 4;
    // Collect 3 members: chi_index, chi_index+4, chi_index+8 (mod 12), sorted
    let mut members = [
        group,
        (group + 4) % 12,
        (group + 8) % 12,
    ];
    members.sort_unstable();
    [CHI[members[0]], CHI[members[1]], CHI[members[2]]]
}

/// Return the tứ-hành-xung (four-clash) square for `chi_index` (sorted).
///
/// The three squares:
/// - Tý(0) · Mão(3) · Ngọ(6)  · Dậu(9)  — `chi % 3 == 0`
/// - Sửu(1)· Thìn(4)· Mùi(7) · Tuất(10) — `chi % 3 == 1`
/// - Dần(2)· Tỵ(5) · Thân(8) · Hợi(11) — `chi % 3 == 2`
pub fn tu_hanh_xung(chi_index: usize) -> [&'static str; 4] {
    let group = chi_index % 3;
    let mut members = [
        group,
        (group + 3) % 12,
        (group + 6) % 12,
        (group + 9) % 12,
    ];
    members.sort_unstable();
    [
        CHI[members[0]],
        CHI[members[1]],
        CHI[members[2]],
        CHI[members[3]],
    ]
}

/// Compute the full xung/hợp result for a day branch.
pub fn get_xung_hop(chi_index: usize) -> XungHopResult {
    XungHopResult {
        luc_xung: luc_xung(chi_index).to_string(),
        tam_hop: tam_hop(chi_index).iter().map(|s| s.to_string()).collect(),
        tu_hanh_xung: tu_hanh_xung(chi_index)
            .iter()
            .map(|s| s.to_string())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- lục xung ---

    #[test]
    fn luc_xung_ty_is_ngo() {
        // Tý (0) clashes with Ngọ (6)
        assert_eq!(luc_xung(0), "Ngọ");
    }

    #[test]
    fn luc_xung_dan_is_than() {
        // Dần (2) clashes with Thân (8)
        assert_eq!(luc_xung(2), "Thân");
    }

    #[test]
    fn luc_xung_all_pairs_symmetric() {
        // Every branch's xung must point back to itself
        for i in 0..12 {
            let j = CHI.iter().position(|c| *c == luc_xung(i)).unwrap();
            assert_eq!(
                luc_xung(j),
                CHI[i],
                "luc_xung must be symmetric: {} <-> {}",
                CHI[i],
                CHI[j]
            );
        }
    }

    // --- tam hợp ---

    #[test]
    fn tam_hop_ty_is_water_group() {
        // Tý(0) belongs to Thân·Tý·Thìn (Water)
        let group = tam_hop(0);
        assert!(group.contains(&"Tý"), "Tý must be in its own triad");
        assert!(group.contains(&"Thìn"), "Thìn must be in Tý's triad");
        assert!(group.contains(&"Thân"), "Thân must be in Tý's triad");
    }

    #[test]
    fn tam_hop_dan_is_fire_group() {
        // Dần(2) belongs to Dần·Ngọ·Tuất (Fire)
        let group = tam_hop(2);
        assert!(group.contains(&"Dần"));
        assert!(group.contains(&"Ngọ"));
        assert!(group.contains(&"Tuất"));
    }

    #[test]
    fn tam_hop_groups_cover_all_12_branches() {
        // All 12 branches must appear across the 12 tam_hop calls
        let mut seen = std::collections::HashSet::new();
        for i in 0..12 {
            for &c in tam_hop(i).iter() {
                seen.insert(c);
            }
        }
        assert_eq!(seen.len(), 12, "All 12 branches must appear in tam_hop groups");
    }

    // --- tứ hành xung ---

    #[test]
    fn tu_hanh_xung_ty_group() {
        // Tý(0): group {Tý, Mão, Ngọ, Dậu}
        let group = tu_hanh_xung(0);
        assert!(group.contains(&"Tý"));
        assert!(group.contains(&"Mão"));
        assert!(group.contains(&"Ngọ"));
        assert!(group.contains(&"Dậu"));
    }

    #[test]
    fn tu_hanh_xung_dan_group() {
        // Dần(2): group {Dần, Tỵ, Thân, Hợi}
        let group = tu_hanh_xung(2);
        assert!(group.contains(&"Dần"));
        assert!(group.contains(&"Tỵ"));
        assert!(group.contains(&"Thân"));
        assert!(group.contains(&"Hợi"));
    }

    #[test]
    fn tu_hanh_xung_suu_group() {
        // Sửu(1): group {Sửu, Thìn, Mùi, Tuất}
        let group = tu_hanh_xung(1);
        assert!(group.contains(&"Sửu"));
        assert!(group.contains(&"Thìn"));
        assert!(group.contains(&"Mùi"));
        assert!(group.contains(&"Tuất"));
    }

    #[test]
    fn tu_hanh_xung_groups_cover_all_12_branches() {
        let mut seen = std::collections::HashSet::new();
        for i in 0..12 {
            for &c in tu_hanh_xung(i).iter() {
                seen.insert(c);
            }
        }
        assert_eq!(
            seen.len(),
            12,
            "All 12 branches must appear in tu_hanh_xung groups"
        );
    }

    // --- get_xung_hop integration ---

    #[test]
    fn get_xung_hop_ty_day() {
        let result = get_xung_hop(0);
        assert_eq!(result.luc_xung, "Ngọ");
        assert_eq!(result.tam_hop.len(), 3);
        assert_eq!(result.tu_hanh_xung.len(), 4);
        assert!(result.tam_hop.contains(&"Tý".to_string()));
        assert!(result.tu_hanh_xung.contains(&"Tý".to_string()));
    }
}
