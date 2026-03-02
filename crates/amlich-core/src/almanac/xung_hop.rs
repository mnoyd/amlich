use super::types::XungHopResult;
/// Xung/Hợp Domain — Earthly Branch Conflict and Harmony Relations
///
/// Implements the three classical branch-relationship groups used in daily
/// almanac calculations:
///
/// - **Lục xung (六冲):** Each branch clashes with the one 6 positions away.
/// - **Tam hợp (三合):** Three branches form a harmony triad (4 groups).
/// - **Tứ hành xung (四行冲):** Four branches in a mutual-clash square (3 groups).
use crate::types::CHI;

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
    let mut members = [group, (group + 4) % 12, (group + 8) % 12];
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
    let mut members = [group, (group + 3) % 12, (group + 6) % 12, (group + 9) % 12];
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
    let xiang_xing_group = get_xiang_xing(chi_index);

    XungHopResult {
        luc_xung: luc_xung(chi_index).to_string(),
        tam_hop: tam_hop(chi_index).iter().map(|s| s.to_string()).collect(),
        tu_hanh_xung: tu_hanh_xung(chi_index)
            .iter()
            .map(|s| s.to_string())
            .collect(),
        liu_he: Some(get_liu_he(chi_index).to_string()),
        xiang_hai: Some(get_xiang_hai(chi_index).to_string()),
        xiang_xing: if xiang_xing_group.is_empty() {
            None
        } else {
            Some(xiang_xing_group)
        },
    }
}

// --- Lục hợp ---

/// Lục hợp pairs - 6 harmony pairs covering all 12 branches
pub const LIUHE: [(usize, usize); 6] = [
    (0, 1),  // 子丑
    (2, 11), // 寅亥
    (3, 10), // 卯戌
    (4, 9),  // 辰酉
    (5, 8),  // 巳申
    (6, 7),  // 午未
];

/// Return the Lục hợp harmony partner for `chi_index`.
///
/// Returns the branch's harmony partner if one exists, otherwise returns the branch itself.
pub fn get_liu_he(chi_index: usize) -> &'static str {
    for (a, b) in LIUHE.iter() {
        if *a == chi_index {
            return CHI[*b];
        }
        if *b == chi_index {
            return CHI[*a];
        }
    }
    CHI[chi_index] // No partner (should not happen with full coverage)
}

// --- Tương hại ---

/// Tương hại pairs - 6 harm pairs covering all 12 branches
pub const XIANGHAI: [(usize, usize); 6] = [
    (0, 7),  // 子未
    (1, 6),  // 丑午
    (2, 9),  // 寅酉
    (3, 8),  // 卯申
    (4, 11), // 辰亥
    (5, 10), // 巳戌
];

/// Return the Tương hại harm partner for `chi_index`.
///
/// Returns the branch's harm partner if one exists, otherwise returns the branch itself.
pub fn get_xiang_hai(chi_index: usize) -> &'static str {
    for (a, b) in XIANGHAI.iter() {
        if *a == chi_index {
            return CHI[*b];
        }
        if *b == chi_index {
            return CHI[*a];
        }
    }
    CHI[chi_index]
}

// --- Tương hình ---

/// Tương hình punishment groups - 4 groups with 3-4 members each
/// Note: Group [6, 6, 6] represents Ngọ Ngọ (self-punishment)
pub const XIANGXING: [[usize; 3]; 4] = [
    [2, 3, 5],  // 寅卯巳 (Vô恩之刑 - ungrateful punishment)
    [0, 1, 4],  // 子辰丑 (恃势之刑 - relying on power punishment)
    [8, 9, 11], // 申酉亥 (无礼之刑 - disrespectful punishment)
    [6, 6, 6],  // 午午 (自刑 - self-punishment)
];

/// Return the Tương hình punishment group members for `chi_index`.
///
/// Returns a vector of branch names that form the punishment group with `chi_index`.
/// For self-punishment (Ngọ), returns [Ngọ, Ngọ, Ngọ] to indicate special handling.
pub fn get_xiang_xing(chi_index: usize) -> Vec<String> {
    for group in XIANGXING.iter() {
        if group.contains(&chi_index) {
            return group.iter().map(|&idx| CHI[idx].to_string()).collect();
        }
    }
    vec![]
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
        assert_eq!(
            seen.len(),
            12,
            "All 12 branches must appear in tam_hop groups"
        );
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

    // --- Lục hợp ---

    #[test]
    fn test_liu_he_complete_coverage() {
        // All 12 branches should find a partner
        for i in 0..12 {
            let partner = get_liu_he(i);
            assert_ne!(
                partner, CHI[i],
                "Branch {} should have a different harmony partner",
                CHI[i]
            );
        }
    }

    #[test]
    fn test_liu_he_symmetric() {
        // Every pair should be bidirectional
        for i in 0..12 {
            let partner = get_liu_he(i);
            let partner_idx = CHI.iter().position(|c| *c == partner).unwrap();
            let reverse = get_liu_he(partner_idx);
            assert_eq!(
                reverse, CHI[i],
                "Lục hợp should be symmetric: {} <-> {}",
                CHI[i], partner
            );
        }
    }

    #[test]
    fn test_liu_he_returns_correct_partner() {
        // Specific branches return expected results
        assert_eq!(
            get_liu_he(0),
            "Sửu", // 子 -> 丑
            "Tý (0) should harmonize with Sửu (1)"
        );
        assert_eq!(
            get_liu_he(2),
            "Hợi", // 寅 -> 亥
            "Dần (2) should harmonize with Hợi (11)"
        );
        assert_eq!(
            get_liu_he(3),
            "Tuất", // 卯 -> 戌
            "Mão (3) should harmonize with Tuất (10)"
        );
    }

    // --- Tương hại ---

    #[test]
    fn test_xiang_hai_complete_coverage() {
        // All 12 branches should find a harm partner
        for i in 0..12 {
            let partner = get_xiang_hai(i);
            assert_ne!(
                partner, CHI[i],
                "Branch {} should have a different harm partner",
                CHI[i]
            );
        }
    }

    #[test]
    fn test_xiang_hai_symmetric() {
        // Every pair should be bidirectional
        for i in 0..12 {
            let partner = get_xiang_hai(i);
            let partner_idx = CHI.iter().position(|c| *c == partner).unwrap();
            let reverse = get_xiang_hai(partner_idx);
            assert_eq!(
                reverse, CHI[i],
                "Tương hại should be symmetric: {} <-> {}",
                CHI[i], partner
            );
        }
    }

    #[test]
    fn test_xiang_hai_returns_correct_partner() {
        // Specific branches return expected results
        assert_eq!(
            get_xiang_hai(0),
            "Mùi", // 子 -> 未
            "Tý (0) should harm with Mùi (7)"
        );
        assert_eq!(
            get_xiang_hai(2),
            "Dậu", // 寅 -> 酉
            "Dần (2) should harm with Dậu (9)"
        );
        assert_eq!(
            get_xiang_hai(3),
            "Thân", // 卯 -> 申
            "Mão (3) should harm with Thân (8)"
        );
    }

    // --- Tương hình ---

    #[test]
    fn test_xiang_xing_groups_complete() {
        // All branches in punishment groups should appear
        // Note: Not all 12 branches have Tương hình - only those in the 4 groups
        let mut seen = std::collections::HashSet::new();
        for i in 0..12 {
            let group = get_xiang_xing(i);
            for branch in &group {
                seen.insert(branch.clone());
            }
        }

        // Verify the 4 groups are correct: 寅卯巳, 子辰丑, 申酉亥, 午午
        assert!(seen.contains(&"Dần".to_string()));
        assert!(seen.contains(&"Mão".to_string()));
        assert!(seen.contains(&"Tỵ".to_string()));
        assert!(seen.contains(&"Tý".to_string()));
        assert!(seen.contains(&"Thìn".to_string()));
        assert!(seen.contains(&"Sửu".to_string()));
        assert!(seen.contains(&"Thân".to_string()));
        assert!(seen.contains(&"Dậu".to_string()));
        assert!(seen.contains(&"Hợi".to_string()));
        assert!(seen.contains(&"Ngọ".to_string()));

        // Mùi and Tuất should NOT have Tương hình (not in any group)
        let mui_group = get_xiang_xing(7); // Mùi
        assert_eq!(
            mui_group,
            Vec::<String>::new(),
            "Mùi should not have Tương hình"
        );

        let tuat_group = get_xiang_xing(10); // Tuất
        assert_eq!(
            tuat_group,
            Vec::<String>::new(),
            "Tuất should not have Tương hình"
        );
    }

    #[test]
    fn test_xiang_xing_self_punishment() {
        // Ngọ Ngọ self-punishment: should return [Ngọ, Ngọ, Ngọ] not just [Ngọ]
        let ngo_group = get_xiang_xing(6); // Ngọ is index 6
        assert_eq!(
            ngo_group.len(),
            3,
            "Ngọ self-punishment should return 3 entries"
        );
        assert!(
            ngo_group.iter().all(|s| s == "Ngọ"),
            "All members should be Ngọ for self-punishment"
        );
    }

    #[test]
    fn test_xiang_xing_returns_correct_groups() {
        // 寅卯巳 returns [Dần, Mão, Tỵ]
        let dan_group = get_xiang_xing(2); // Dần is index 2
        assert_eq!(dan_group.len(), 3);
        assert!(dan_group.contains(&"Dần".to_string()));
        assert!(dan_group.contains(&"Mão".to_string()));
        assert!(dan_group.contains(&"Tỵ".to_string()));

        // 子辰丑 returns [Tý, Thìn, Sửu]
        let ty_group = get_xiang_xing(0); // Tý is index 0
        assert_eq!(ty_group.len(), 3);
        assert!(ty_group.contains(&"Tý".to_string()));
        assert!(ty_group.contains(&"Thìn".to_string()));
        assert!(ty_group.contains(&"Sửu".to_string()));
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
