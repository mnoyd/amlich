/// Thái Tuế (太歲) — Grand Duke annual conflict check
///
/// Checks whether a person's birth year Earthly Branch has any of 5 conflict
/// relationships with the current year's Earthly Branch.
///
/// **Source:** Both KHCBPPT and Vietnamese folk tradition
/// **Decision:** DEC-0021
use serde::{Deserialize, Serialize};

use super::tu_menh::Direction;
use super::types::RuleEvidence;
use super::xung_hop;
use crate::sources::SOURCE_KHCBPPT;
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

/// Directional Thái Tuế (太歲) result — the year-branch → eight-direction
/// projection of the Grand Duke, distinct from the personal-conflict
/// computation in [`compute_thai_tue`].
///
/// This sibling API was added in Phase 23 Plan 23-01 (XLK-01) so the
/// reasoning cross-link can derive the year's Thái Tuế direction without
/// any birth context. The existing personal-conflict API is unchanged.
///
/// **Source:** KHCBPPT — directional Thái Tuế at the year's Earthly Branch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThaiTueDirectionResult {
    /// 0-based Earthly Branch index (0=Tý .. 11=Hợi).
    pub year_chi_index: usize,
    /// Vietnamese branch name (e.g. "Tý", "Sửu", ...).
    pub year_chi: String,
    /// Eight-direction mapping (reuses the existing `tu_menh::Direction` enum;
    /// no new directional type is minted).
    pub direction: Direction,
    /// Non-optional KHCBPPT provenance. Always populated for this result.
    pub evidence: RuleEvidence,
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
        evidence: Some(RuleEvidence {
            source_id: SOURCE_KHCBPPT.to_string(),
            method: "thai_tue_year_branch_conflict".to_string(),
            profile: "baseline".to_string(),
        }),
    }
}

/// Look up the eight-direction cell for a year Earthly Branch.
///
/// Classical rule: Thái Tuế sits at the direction of the year's Earthly
/// Branch. The 12 branches collapse onto the 8-point `Direction` enum as:
///
/// | Branch(es)            | Direction   |
/// | --------------------- | ----------- |
/// | Tý(0)                 | North       |
/// | Sửu(1), Dần(2)        | Northeast   |
/// | Mão(3)                | East        |
/// | Thìn(4), Tỵ(5)        | Southeast   |
/// | Ngọ(6)                | South       |
/// | Mùi(7), Thân(8)       | Southwest   |
/// | Dậu(9)                | West        |
/// | Tuất(10), Hợi(11)     | Northwest   |
fn direction_for_year_chi(year_chi_index: usize) -> Direction {
    match year_chi_index {
        0 => Direction::North,
        1 | 2 => Direction::Northeast,
        3 => Direction::East,
        4 | 5 => Direction::Southeast,
        6 => Direction::South,
        7 | 8 => Direction::Southwest,
        9 => Direction::West,
        10 | 11 => Direction::Northwest,
        other => panic!(
            "year_chi_index {} not in 0..=11 (Earthly Branch range)",
            other
        ),
    }
}

/// Derive the year-only directional Thái Tuế (太歲) for a given Earthly Branch.
///
/// This is a **sibling** of [`compute_thai_tue`]: the personal-conflict API
/// is unchanged. The directional sibling is used by the Phase 23 reasoning
/// cross-link (`reasoning/direction_composite.rs`) to project each year's
/// Thái Tuế onto a single 8-point direction without any birth context.
///
/// # Panics
/// Panics if `year_chi_index` is outside the 0..=11 Earthly Branch range —
/// this matches the contract-violation discipline of the existing almanac
/// table APIs (e.g. `xung_hop::luc_xung`).
pub fn thai_tue_direction(year_chi_index: usize) -> ThaiTueDirectionResult {
    // Validate the index via the directional match BEFORE indexing `CHI`.
    // `direction_for_year_chi` panics with a useful message on out-of-range
    // input; the natural `CHI[year_chi_index]` indexing would panic later
    // with the less informative "index out of bounds" message.
    let direction = direction_for_year_chi(year_chi_index);
    let year_chi = CHI[year_chi_index].to_string();
    ThaiTueDirectionResult {
        year_chi_index,
        year_chi,
        direction,
        evidence: RuleEvidence {
            source_id: SOURCE_KHCBPPT.to_string(),
            method: "thai_tue_year_branch_to_direction".to_string(),
            profile: "baseline".to_string(),
        },
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
        // XLK-01 backfill: evidence is now populated with KHCBPPT provenance.
        let r = compute_thai_tue(0, 0);
        let evidence = r
            .evidence
            .as_ref()
            .expect("compute_thai_tue evidence must be populated after XLK-01 backfill");
        assert_eq!(evidence.source_id, SOURCE_KHCBPPT);
        assert_eq!(evidence.method, "thai_tue_year_branch_conflict");
        assert_eq!(evidence.profile, "baseline");
    }
}

#[cfg(test)]
mod direction_tests {
    use super::*;
    use crate::almanac::tu_menh::Direction;

    #[test]
    fn cardinal_branches_map_to_unique_directions() {
        assert_eq!(thai_tue_direction(0).direction, Direction::North); // Tý
        assert_eq!(thai_tue_direction(3).direction, Direction::East); // Mão
        assert_eq!(thai_tue_direction(6).direction, Direction::South); // Ngọ
        assert_eq!(thai_tue_direction(9).direction, Direction::West); // Dậu
    }

    #[test]
    fn intercardinal_branches_collapse_in_pairs() {
        // Sửu + Dần → Northeast
        assert_eq!(thai_tue_direction(1).direction, Direction::Northeast);
        assert_eq!(thai_tue_direction(2).direction, Direction::Northeast);
        // Thìn + Tỵ → Southeast
        assert_eq!(thai_tue_direction(4).direction, Direction::Southeast);
        assert_eq!(thai_tue_direction(5).direction, Direction::Southeast);
        // Mùi + Thân → Southwest
        assert_eq!(thai_tue_direction(7).direction, Direction::Southwest);
        assert_eq!(thai_tue_direction(8).direction, Direction::Southwest);
        // Tuất + Hợi → Northwest
        assert_eq!(thai_tue_direction(10).direction, Direction::Northwest);
        assert_eq!(thai_tue_direction(11).direction, Direction::Northwest);
    }

    #[test]
    fn all_12_year_branches_covered() {
        for i in 0..12 {
            let r = thai_tue_direction(i);
            assert_eq!(r.year_chi_index, i);
            assert_eq!(r.year_chi, CHI[i]);
            assert_eq!(r.evidence.source_id, SOURCE_KHCBPPT);
            assert_eq!(r.evidence.method, "thai_tue_year_branch_to_direction");
            assert_eq!(r.evidence.profile, "baseline");
        }
    }

    #[test]
    fn directional_sibling_does_not_alter_personal_conflict_api() {
        // The personal Thái Tuế API is unchanged by the sibling directional
        // addition. compute_thai_tue(0, 6) still flags the Xung conflict.
        let personal = compute_thai_tue(0, 6);
        assert!(personal.has_conflict);
        assert!(personal
            .conflicts
            .iter()
            .any(|c| matches!(c.kind, ThaiTueConflictKind::Xung)));

        // And the directional sibling for year 6 (Ngọ) returns South.
        let directional = thai_tue_direction(6);
        assert_eq!(directional.direction, Direction::South);
        assert_ne!(
            directional.evidence.method,
            personal.evidence.as_ref().unwrap().method,
            "directional sibling uses a distinct evidence method from the personal API"
        );
    }

    #[test]
    #[should_panic(expected = "not in 0..=11")]
    fn out_of_range_year_chi_index_panics() {
        let _ = thai_tue_direction(12);
    }
}
