use super::types::{BranchRef, PunishmentKind, TriadElement, XungHopResult};
/// Xung/Hợp Domain — Earthly Branch Conflict and Harmony Relations
///
/// Implements the three classical branch-relationship groups used in daily
/// almanac calculations:
///
/// - **Lục xung (六冲):** Each branch clashes with the one 6 positions away.
/// - **Tam hợp (三合):** Three branches form a harmony triad (4 groups).
/// - **Tứ hành xung (四行冲):** Four branches in a mutual-clash square (3 groups).
///
/// Also implements the canonical Tương hình (相刑) taxonomy as typed
/// `PunishmentKind` facts — see
/// `docs/architecture/personal-day-audit/branch-relation-decision.md` for
/// the source-cited decision.
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
        xiang_xing: xiang_xing_group,
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

/// Canonical 3-branch Tương hình (相刑) groups.
///
/// Source-cited taxonomy per
/// `docs/architecture/personal-day-audit/branch-relation-decision.md` §2.5:
///
/// - `[2, 5, 8]` — 寅巳申 (Dần, Tỵ, Thân) — Vô ân chi hình (Fire element)
/// - `[1, 7, 10]` — 丑未戌 (Sửu, Mùi, Tuất) — Trì thế chi hình (Earth element)
/// - `[3, 0]` — 卯子 (Mão, Tý) — Vô lễ chi hình (directed pair)
/// - `[4, 6, 9, 11]` — 辰午酉亥 (Thìn, Ngọ, Dậu, Hợi) — Tự hình (self-punishment)
///
/// This replaces the previous (incorrect) `[[usize; 3]; 4]` constant that
/// conflated membership with completed-group claims and mis-grouped
/// 寅卯巳 as a punishment triad when it is not. The directed pair and
/// self-punishment groups use arrays of distinct lengths; the typed
/// [`xiang_xing_pair`] / [`xiang_xing_self`] functions are the
/// supported lookup surface.
pub mod xiang_xing_groups {
    /// 寅巳申 — Vô ân chi hình (mutual 3-branch Fire triad).
    pub const INVISIBLE_FIRE: [usize; 3] = [2, 5, 8];
    /// 丑未戌 — Trì thế chi hình (mutual 3-branch Earth triad).
    pub const EARTH_POWER: [usize; 3] = [1, 7, 10];
    /// 卯子 — Vô lễ chi hình (directed Tý → Mão pair).
    pub const DIRECTED_RUDE: [usize; 2] = [3, 0];
    /// 辰午酉亥 — Tự hình (self-punishment singletons).
    pub const SELF_PUNISHMENT: [usize; 4] = [4, 6, 9, 11];
}

/// Return the day-level 3-branch Tương hình group for `chi_index`, or
/// `None` when the branch is not in any canonical 3-branch group.
///
/// This is the day-level projection used by `XungHopResult.xiang_xing`.
/// Pair-level, self-punishment, directed, and disputed cases are
/// surfaced via [`xiang_xing_pair`] / [`xiang_xing_self`].
pub fn get_xiang_xing(chi_index: usize) -> Option<Vec<String>> {
    if xiang_xing_groups::INVISIBLE_FIRE.contains(&chi_index) {
        return Some(
            xiang_xing_groups::INVISIBLE_FIRE
                .iter()
                .map(|&i| CHI[i].to_string())
                .collect(),
        );
    }
    if xiang_xing_groups::EARTH_POWER.contains(&chi_index) {
        return Some(
            xiang_xing_groups::EARTH_POWER
                .iter()
                .map(|&i| CHI[i].to_string())
                .collect(),
        );
    }
    None
}

/// Tương hình classification for a pair of branches `(a, b)` where
/// `a != b`. Returns the canonical [`PunishmentKind`].
///
/// Canonical mapping (per decision brief §2.5):
/// - Both in `{2, 5, 8}` (寅巳申) → `CompletedTriad { triad: Hoa }`
/// - Both in `{1, 7, 10}` (丑未戌) → `CompletedTriad { triad: Tho }` is
///   **not currently emitted** because `TriadElement` does not yet have
///   an Earth variant; incomplete two-branch occurrences are marked
///   `Unavailable { reason: "incomplete Trì thế triad" }`.
/// - `(0, 3)` or `(3, 0)` (子卯) → `DirectedPair { aggressor: Tý, victim: Mão }`
///   — direction is always Tý → Mão, the reverse input order still
///   reports the same direction.
/// - Otherwise → `None`.
pub fn xiang_xing_pair(a: usize, b: usize) -> PunishmentKind {
    debug_assert!(a < 12 && b < 12, "xiang_xing_pair indices must be in 0..12");
    if a == b {
        return PunishmentKind::None;
    }
    if xiang_xing_groups::INVISIBLE_FIRE.contains(&a)
        && xiang_xing_groups::INVISIBLE_FIRE.contains(&b)
    {
        return PunishmentKind::CompletedTriad {
            triad: TriadElement::Hoa,
        };
    }
    if xiang_xing_groups::EARTH_POWER.contains(&a) && xiang_xing_groups::EARTH_POWER.contains(&b) {
        // Two-branch occurrences of 丑未戌 are canonically disputed —
        // see branch-relation-decision.md §2.5.4. Mark as unavailable
        // rather than promoting to a verdict.
        return PunishmentKind::Unavailable {
            reason: "incomplete Trì thế triad (丑未戌)".to_string(),
        };
    }
    let is_ty = a == 0 || b == 0;
    let is_mao = a == 3 || b == 3;
    if is_ty && is_mao {
        return PunishmentKind::DirectedPair {
            aggressor: BranchRef::new(0),
            victim: BranchRef::new(3),
        };
    }
    PunishmentKind::None
}

/// Tương hình classification for a same-branch comparison
/// `compute_branch_relation(b, b)`. Returns `SelfPunishment` for the four
/// canonical self-punishment branches and `None` otherwise.
///
/// Same-branch is **never** promoted to `CompletedTriad` because the
/// triad completion rule requires three distinct branches.
pub fn xiang_xing_self(b: usize) -> PunishmentKind {
    debug_assert!(b < 12, "xiang_xing_self index must be in 0..12");
    if xiang_xing_groups::SELF_PUNISHMENT.contains(&b) {
        return PunishmentKind::SelfPunishment {
            branch: BranchRef::new(b),
        };
    }
    PunishmentKind::None
}

/// Return the Tam hợp (三合) triad element for a branch, or `None` if
/// the branch is not in any triad. Every branch in 0..12 belongs to
/// exactly one triad today, so this is total over the canonical range.
pub fn triad_element(chi_index: usize) -> Option<TriadElement> {
    debug_assert!(chi_index < 12, "triad_element index must be in 0..12");
    match chi_index % 4 {
        0 => Some(TriadElement::Thuy), // Thân(8) · Tý(0) · Thìn(4)
        1 => Some(TriadElement::Kim),  // Tỵ(5)  · Dậu(9) · Sửu(1)
        2 => Some(TriadElement::Hoa),  // Dần(2) · Ngọ(6) · Tuất(10)
        3 => Some(TriadElement::Moc),  // Hợi(11)· Mão(3) · Mùi(7)
        _ => None,
    }
}

/// True if two branches belong to the same Tam hợp triad.
///
/// Same-branch pairs are not promoted here — the caller should
/// additionally check `a != b` if the policy requires a distinct pair.
pub fn is_triad_member(a: usize, b: usize) -> bool {
    debug_assert!(a < 12 && b < 12, "is_triad_member indices must be in 0..12");
    triad_element(a) == triad_element(b)
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

    /// get_xiang_xing is the day-level 3-branch group lookup. Only
    /// 寅巳申 and 丑未戌 return groups; the other 6 branches (Tý, Mão,
    /// Thìn, Ngọ, Dậu, Hợi) are not in any canonical 3-branch group.
    #[test]
    fn test_xiang_xing_canonical_3_branch_groups() {
        // 寅巳申 (Dần, Tỵ, Thân) → Vô ân chi hình (Fire)
        for &idx in &[2usize, 5, 8] {
            let group = get_xiang_xing(idx).expect("group");
            assert_eq!(group.len(), 3, "Dần/Tỵ/Thân group must be 3 members");
            assert!(group.contains(&"Dần".to_string()));
            assert!(group.contains(&"Tỵ".to_string()));
            assert!(group.contains(&"Thân".to_string()));
        }
        // 丑未戌 (Sửu, Mùi, Tuất) → Trì thế chi hình (Earth)
        for &idx in &[1usize, 7, 10] {
            let group = get_xiang_xing(idx).expect("group");
            assert_eq!(group.len(), 3, "Sửu/Mùi/Tuất group must be 3 members");
            assert!(group.contains(&"Sửu".to_string()));
            assert!(group.contains(&"Mùi".to_string()));
            assert!(group.contains(&"Tuất".to_string()));
        }
        // The other 6 branches are NOT in any 3-branch group.
        for &idx in &[0usize, 3, 4, 6, 9, 11] {
            assert!(
                get_xiang_xing(idx).is_none(),
                "branch {idx} ({}) should not be in a 3-branch group",
                CHI[idx]
            );
        }
    }

    /// Self-punishment is exposed via the typed `xiang_xing_self` API
    /// (not via `get_xiang_xing`, which is day-level 3-branch only).
    #[test]
    fn test_xiang_xing_self_punishment_canonical() {
        for &idx in &[4usize, 6, 9, 11] {
            let kind = xiang_xing_self(idx);
            assert!(
                matches!(kind, PunishmentKind::SelfPunishment { .. }),
                "branch {idx} ({}) must be self-punishment",
                CHI[idx]
            );
        }
        // The other 8 branches are not self-punishment.
        for &idx in &[0usize, 1, 2, 3, 5, 7, 8, 10] {
            assert_eq!(
                xiang_xing_self(idx),
                PunishmentKind::None,
                "branch {idx} ({}) must not be self-punishment",
                CHI[idx]
            );
        }
    }

    /// xiang_xing_pair produces the canonical pair-level kinds.
    /// Dần-Tỵ, Dần-Thân, Tỵ-Thân → CompletedTriad(Hỏa).
    #[test]
    fn test_xiang_xing_pair_fire_triad() {
        let pairs = [(2usize, 5usize), (2, 8), (5, 8), (5, 2), (8, 2), (8, 5)];
        for (a, b) in pairs {
            let kind = xiang_xing_pair(a, b);
            assert_eq!(
                kind,
                PunishmentKind::CompletedTriad {
                    triad: TriadElement::Hoa
                },
                "pair ({a}, {b}) must be CompletedTriad(Hỏa)"
            );
        }
    }

    /// 丑未戌 two-branch occurrences are canonically Unavailable
    /// (incomplete Trì thế triad), NOT promoted to a punishment.
    #[test]
    fn test_xiang_xing_pair_earth_two_branch_unavailable() {
        let pairs = [(1usize, 7usize), (1, 10), (7, 10), (7, 1), (10, 1), (10, 7)];
        for (a, b) in pairs {
            let kind = xiang_xing_pair(a, b);
            assert!(
                matches!(kind, PunishmentKind::Unavailable { .. }),
                "pair ({a}, {b}) must be Unavailable (incomplete Trì thế triad)"
            );
        }
    }

    /// 子卯 is a directed pair: Tý → Mão. Both input orders
    /// report the same direction.
    #[test]
    fn test_xiang_xing_pair_directed_ty_mao() {
        let expected = PunishmentKind::DirectedPair {
            aggressor: BranchRef::new(0),
            victim: BranchRef::new(3),
        };
        assert_eq!(xiang_xing_pair(0, 3), expected);
        assert_eq!(xiang_xing_pair(3, 0), expected);
    }

    /// All other distinct-branch pairs (not in 寅巳申, not the directed
    /// 子卯 pair, not in 丑未戌) are `None` — not a punishment.
    #[test]
    fn test_xiang_xing_pair_unrelated_is_none() {
        // Pick a few pairs that should clearly NOT be a punishment.
        let pairs = [
            (0usize, 1usize),
            (0, 2),
            (3, 4),
            (4, 5),
            (6, 7),
            (8, 9),
            (9, 10),
            (10, 11),
        ];
        for (a, b) in pairs {
            assert_eq!(
                xiang_xing_pair(a, b),
                PunishmentKind::None,
                "pair ({a}, {b}) must be None"
            );
        }
    }

    /// Same-branch input to xiang_xing_pair is always `None`. The
    /// self-punishment semantic lives in `xiang_xing_self`.
    #[test]
    fn test_xiang_xing_pair_same_branch_is_none() {
        for i in 0..12 {
            assert_eq!(
                xiang_xing_pair(i, i),
                PunishmentKind::None,
                "pair ({i}, {i}) must be None"
            );
        }
    }

    /// Triad element lookup is total over the canonical branch range.
    #[test]
    fn test_triad_element_all_branches() {
        // Thân(8) · Tý(0) · Thìn(4) → Thủy
        assert_eq!(triad_element(0), Some(TriadElement::Thuy));
        assert_eq!(triad_element(4), Some(TriadElement::Thuy));
        assert_eq!(triad_element(8), Some(TriadElement::Thuy));
        // Tỵ(5) · Dậu(9) · Sửu(1) → Kim
        assert_eq!(triad_element(1), Some(TriadElement::Kim));
        assert_eq!(triad_element(5), Some(TriadElement::Kim));
        assert_eq!(triad_element(9), Some(TriadElement::Kim));
        // Dần(2) · Ngọ(6) · Tuất(10) → Hỏa
        assert_eq!(triad_element(2), Some(TriadElement::Hoa));
        assert_eq!(triad_element(6), Some(TriadElement::Hoa));
        assert_eq!(triad_element(10), Some(TriadElement::Hoa));
        // Hợi(11) · Mão(3) · Mùi(7) → Mộc
        assert_eq!(triad_element(3), Some(TriadElement::Moc));
        assert_eq!(triad_element(7), Some(TriadElement::Moc));
        assert_eq!(triad_element(11), Some(TriadElement::Moc));
    }

    // --- branch-relations-golden.json cross-check ---

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct GoldenDataset {
        #[allow(dead_code)]
        metadata: GoldenMetadata,
        entries: Vec<GoldenEntry>,
    }

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    struct GoldenMetadata {
        schema: String,
        entry_count: usize,
    }

    #[derive(Debug, Deserialize)]
    #[serde(tag = "name")]
    enum GoldenEntry {
        #[serde(rename = "luc_xung_pairs")]
        LucXungPairs { pairs: Vec<LucXungPair> },
        #[serde(rename = "tam_hop_triads")]
        TamHopTriads { triads: Vec<TamHopTriad> },
        #[serde(rename = "xiang_xing_canonical_pairs")]
        XiangXingPairs { cases: Vec<XiangXingCase> },
        #[serde(rename = "xiang_xing_self_punishment")]
        XiangXingSelf { branches: Vec<XiangXingSelfBranch> },
    }

    #[derive(Debug, Deserialize)]
    struct LucXungPair {
        day_chi_index: usize,
        #[allow(dead_code)]
        day_chi: String,
        expected_luc_xung_index: usize,
        #[allow(dead_code)]
        expected_luc_xung: String,
    }

    #[derive(Debug, Deserialize)]
    struct TamHopTriad {
        element: String,
        members: Vec<usize>,
        #[allow(dead_code)]
        branches: Vec<String>,
    }

    #[derive(Debug, Deserialize)]
    struct XiangXingCase {
        kind: String,
        a: usize,
        b: usize,
        #[serde(default)]
        #[allow(dead_code)]
        note: String,
        #[serde(default)]
        triad: Option<String>,
        #[serde(default)]
        reason: Option<String>,
        #[serde(default)]
        aggressor: Option<usize>,
        #[serde(default)]
        victim: Option<usize>,
    }

    #[derive(Debug, Deserialize)]
    struct XiangXingSelfBranch {
        index: usize,
        #[allow(dead_code)]
        branch: String,
        expected_kind: String,
    }

    const GOLDEN_JSON: &str = include_str!("../../data/almanac/branch-relations-golden.json");

    #[test]
    fn golden_luc_xung_pairs_match_canonical() {
        let dataset: GoldenDataset =
            serde_json::from_str(GOLDEN_JSON).expect("branch-relations-golden.json must parse");
        for entry in &dataset.entries {
            if let GoldenEntry::LucXungPairs { pairs } = entry {
                for p in pairs {
                    let partner_name = luc_xung(p.day_chi_index);
                    let partner_index = CHI.iter().position(|c| *c == partner_name).unwrap();
                    assert_eq!(
                        partner_index, p.expected_luc_xung_index,
                        "day_chi_index {}: luc_xung partner mismatch",
                        p.day_chi_index
                    );
                }
            }
        }
    }

    #[test]
    fn golden_tam_hop_triads_match_canonical() {
        let dataset: GoldenDataset =
            serde_json::from_str(GOLDEN_JSON).expect("branch-relations-golden.json must parse");
        for entry in &dataset.entries {
            if let GoldenEntry::TamHopTriads { triads } = entry {
                assert_eq!(triads.len(), 4, "expected exactly 4 canonical triads");
                for t in triads {
                    let expected = match t.element.as_str() {
                        "thuy" => TriadElement::Thuy,
                        "kim" => TriadElement::Kim,
                        "hoa" => TriadElement::Hoa,
                        "moc" => TriadElement::Moc,
                        other => panic!("unknown triad element in golden: {other}"),
                    };
                    for &idx in &t.members {
                        assert_eq!(
                            triad_element(idx),
                            Some(expected),
                            "branch {idx} should map to triad element {:?}",
                            expected
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn golden_xiang_xing_pairs_match_canonical() {
        use crate::almanac::types::BranchRef;
        let dataset: GoldenDataset =
            serde_json::from_str(GOLDEN_JSON).expect("branch-relations-golden.json must parse");
        for entry in &dataset.entries {
            if let GoldenEntry::XiangXingPairs { cases } = entry {
                for c in cases {
                    let actual = xiang_xing_pair(c.a, c.b);
                    match c.kind.as_str() {
                        "none" => assert_eq!(
                            actual,
                            PunishmentKind::None,
                            "({a}, {b}) must be None",
                            a = c.a,
                            b = c.b
                        ),
                        "completed_triad" => {
                            let triad = match c.triad.as_deref() {
                                Some("hoa") => TriadElement::Hoa,
                                Some("kim") => TriadElement::Kim,
                                Some("moc") => TriadElement::Moc,
                                Some("thuy") => TriadElement::Thuy,
                                other => panic!("unknown triad in golden: {other:?}"),
                            };
                            assert_eq!(
                                actual,
                                PunishmentKind::CompletedTriad { triad },
                                "({a}, {b}) must be CompletedTriad({triad:?})",
                                a = c.a,
                                b = c.b
                            );
                        }
                        "directed_pair" => {
                            let aggressor = c.aggressor.expect("aggressor in golden");
                            let victim = c.victim.expect("victim in golden");
                            assert_eq!(
                                actual,
                                PunishmentKind::DirectedPair {
                                    aggressor: BranchRef::new(aggressor),
                                    victim: BranchRef::new(victim),
                                },
                                "({a}, {b}) must be DirectedPair",
                                a = c.a,
                                b = c.b
                            );
                        }
                        "unavailable" => {
                            assert!(
                                matches!(actual, PunishmentKind::Unavailable { .. }),
                                "({a}, {b}) must be Unavailable, got {actual:?}",
                                a = c.a,
                                b = c.b
                            );
                            if let Some(expected_reason) = &c.reason {
                                if let PunishmentKind::Unavailable { reason } = &actual {
                                    assert!(
                                        reason.contains(expected_reason)
                                            || expected_reason.contains(reason.as_str()),
                                        "Unavailable reason mismatch: actual={reason:?}, expected={expected_reason:?}"
                                    );
                                }
                            }
                        }
                        other => panic!("unknown xiang_xing kind in golden: {other}"),
                    }
                }
            }
        }
    }

    #[test]
    fn golden_xiang_xing_self_punishment_matches_canonical() {
        use crate::almanac::types::BranchRef;
        let dataset: GoldenDataset =
            serde_json::from_str(GOLDEN_JSON).expect("branch-relations-golden.json must parse");
        for entry in &dataset.entries {
            if let GoldenEntry::XiangXingSelf { branches } = entry {
                assert_eq!(branches.len(), 12, "expected all 12 branches");
                for b in branches {
                    let actual = xiang_xing_self(b.index);
                    match b.expected_kind.as_str() {
                        "self_punishment" => assert_eq!(
                            actual,
                            PunishmentKind::SelfPunishment {
                                branch: BranchRef::new(b.index),
                            },
                            "branch {} ({}) must be SelfPunishment",
                            b.index,
                            b.branch
                        ),
                        "none" => assert_eq!(
                            actual,
                            PunishmentKind::None,
                            "branch {} ({}) must be None",
                            b.index,
                            b.branch
                        ),
                        other => panic!("unknown self kind in golden: {other}"),
                    }
                }
            }
        }
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
