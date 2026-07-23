//! Tam Sát (三殺) — Classical three-direction yearly killing direction.
//!
//! Distinct from `tam_tai.rs` (which encodes the three-year Tam Tai
//! affliction period sharing the same Chinese name 三殺). This module
//! encodes the *directional* Tam Sát rule: for each year Earthly Branch,
//! three directions are taboo — the three branches of the lục-xung opposite
//! Tam Hợp triad, each mapped to an 8-point `Direction` cell.
//!
//! ## Locked mapping (CONTEXT.md §"Tam Sát triad → 3-direction mapping")
//!
//! | Tam Hợp triad        | Element | Tam Sát (opposite)   | Tam Sát directions           |
//! | -------------------- | ------- | -------------------- | ---------------------------- |
//! | Thân · Tý · Thìn     | Thủy    | Dần · Ngọ · Tuất     | Đông Bắc, Nam, Tây Bắc       |
//! | Hợi · Mão · Mùi      | Mộc     | Tỵ · Dậu · Sửu       | Đông Nam, Tây, Đông Bắc      |
//! | Dần · Ngọ · Tuất     | Hỏa     | Thân · Tý · Thìn     | Tây Nam, Bắc, Đông Nam       |
//! | Tỵ · Dậu · Sửu       | Kim     | Hợi · Mão · Mùi      | Tây Bắc, Đông, Tây Nam       |
//!
//! **Source:** KHCBPPT (三殺). Exact edition/page citation pending external
//! review — see `crates/amlich-core/data/almanac/tam_sat_provenance.md`
//! (discoverable artifact, NOT loaded at runtime).

use serde::{Deserialize, Serialize};

use super::tu_menh::Direction;
use super::types::RuleEvidence;
use crate::sources::SOURCE_KHCBPPT;
use crate::types::CHI;

/// Tradition-ordered Tam Hợp triad rows, each paired with its opposite
/// (lục-xung) Tam Sát triad and the locked 8-point direction mapping.
///
/// Order mirrors CONTEXT.md §"Tam Sát triad → 3-direction mapping"
/// (Water → Wood → Fire → Metal). Each row is indexed by the year-chi's
/// Tam Hợp group membership (computed by `find_triad_row`), NOT by
/// `year_chi_index % 4` directly — the latter would reorder rows by
/// branch-index residue and lose the tradition-ordered triad labels
/// (the existing `xung_hop::tam_hop` returns branches sorted by index;
/// here we preserve tradition order to match the source convention).
const TAM_SAT_ROWS: [TamSatRow; 4] = [
    // Water triad: Thân(8), Tý(0), Thìn(4) → opposite Dần(2), Ngọ(6), Tuất(10)
    TamSatRow {
        tam_hop_branches: [8, 0, 4],
        tam_sat_branches: [2, 6, 10],
        tam_sat_directions: [
            Direction::Northeast, // Dần
            Direction::South,     // Ngọ
            Direction::Northwest, // Tuất
        ],
    },
    // Wood triad: Hợi(11), Mão(3), Mùi(7) → opposite Tỵ(5), Dậu(9), Sửu(1)
    TamSatRow {
        tam_hop_branches: [11, 3, 7],
        tam_sat_branches: [5, 9, 1],
        tam_sat_directions: [
            Direction::Southeast, // Tỵ
            Direction::West,      // Dậu
            Direction::Northeast, // Sửu
        ],
    },
    // Fire triad: Dần(2), Ngọ(6), Tuất(10) → opposite Thân(8), Tý(0), Thìn(4)
    TamSatRow {
        tam_hop_branches: [2, 6, 10],
        tam_sat_branches: [8, 0, 4],
        tam_sat_directions: [
            Direction::Southwest, // Thân
            Direction::North,     // Tý
            Direction::Southeast, // Thìn
        ],
    },
    // Metal triad: Tỵ(5), Dậu(9), Sửu(1) → opposite Hợi(11), Mão(3), Mùi(7)
    TamSatRow {
        tam_hop_branches: [5, 9, 1],
        tam_sat_branches: [11, 3, 7],
        tam_sat_directions: [
            Direction::Northwest, // Hợi
            Direction::East,      // Mão
            Direction::Southwest, // Mùi
        ],
    },
];

struct TamSatRow {
    tam_hop_branches: [usize; 3],
    tam_sat_branches: [usize; 3],
    tam_sat_directions: [Direction; 3],
}

/// Phase 23 (XLK-02) classical Tam Sát result.
///
/// Carries the year's Earthly Branch, its Tam Hợp triad (tradition order),
/// the opposite (Tam Sát) triad branches, the corresponding 8-point
/// directions, and the KHCBPPT evidence envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TamSatDirectionResult {
    /// 0-based Earthly Branch index (0=Tý .. 11=Hợi).
    pub year_chi_index: usize,
    /// Vietnamese branch name (e.g. "Tý").
    pub year_chi: String,
    /// Tam Hợp triad the year-chi belongs to, in tradition order
    /// (e.g. Water: `["Thân", "Tý", "Thìn"]`).
    pub tam_hop_group: [String; 3],
    /// Opposite (lục-xung) triad — the three Tam Sát branches.
    pub tam_sat_branches: [String; 3],
    /// The three 8-point directions of the Tam Sát (reuses `Direction`).
    pub tam_sat_directions: [Direction; 3],
    /// KHCBPPT provenance; method/profile text references the discoverable
    /// `data/almanac/tam_sat_provenance.md` artifact + PendingExternalReview
    /// marker (exact page citation deferred).
    pub evidence: RuleEvidence,
}

/// Locate the tradition-ordered Tam Sát row whose Tam Hợp triad contains
/// the given year-chi index. Panics on out-of-range input mirroring the
/// existing almanac table APIs (e.g. `xung_hop::luc_xung`).
fn find_triad_row(year_chi_index: usize) -> &'static TamSatRow {
    for row in TAM_SAT_ROWS.iter() {
        if row.tam_hop_branches.contains(&year_chi_index) {
            return row;
        }
    }
    panic!(
        "year_chi_index {} not in 0..=11 (Earthly Branch range)",
        year_chi_index
    )
}

/// Derive the classical three-direction Tam Sát (三殺) for a given year
/// Earthly Branch.
///
/// The classical rule: the year-chi belongs to one of four Tam Hợp triads
/// (Thân-Tý-Thìn / Hợi-Mão-Mùi / Dần-Ngọ-Tuất / Tỵ-Dậu-Sửu); the Tam Sát
/// is the **opposite** (lục-xung) triad's three branches, each mapped to
/// its 8-point direction. Mirrors the `tam_tai.rs::TAI_YEARS` precedent
/// for the lục-xung opposite-triad concept but is a distinct, year-only
/// directional module (not the 3-year Tam Tai affliction cycle).
///
/// # Panics
/// Panics if `year_chi_index` is outside the 0..=11 Earthly Branch range —
/// this matches the contract-violation discipline of the existing almanac
/// table APIs.
pub fn tam_sat_direction(year_chi_index: usize) -> TamSatDirectionResult {
    let row = find_triad_row(year_chi_index);
    let tam_hop_group = row
        .tam_hop_branches
        .iter()
        .map(|&i| CHI[i].to_string())
        .collect::<Vec<_>>()
        .try_into()
        .expect("tam_hop_branches has exactly 3 entries");
    let tam_sat_branches = row
        .tam_sat_branches
        .iter()
        .map(|&i| CHI[i].to_string())
        .collect::<Vec<_>>()
        .try_into()
        .expect("tam_sat_branches has exactly 3 entries");

    TamSatDirectionResult {
        year_chi_index,
        year_chi: CHI[year_chi_index].to_string(),
        tam_hop_group,
        tam_sat_branches,
        tam_sat_directions: row.tam_sat_directions,
        evidence: RuleEvidence {
            source_id: SOURCE_KHCBPPT.to_string(),
            // The method text encodes the derivation rule; the profile text
            // references the discoverable provenance artifact and carries an
            // explicit PendingExternalReview marker for the exact page citation.
            method: "tam_sat_opposite_triad".to_string(),
            profile:
                "baseline | provenance: data/almanac/tam_sat_provenance.md | PendingExternalReview"
                    .to_string(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_lookup_for_water_triad_year_returns_correct_branches() {
        // Tý(0) is in Water triad → opposite triad is Dần/Ngọ/Tuất.
        let r = tam_sat_direction(0);
        assert_eq!(r.tam_sat_branches.as_slice(), &["Dần", "Ngọ", "Tuất"]);
        assert_eq!(
            r.tam_sat_directions,
            [Direction::Northeast, Direction::South, Direction::Northwest]
        );
    }

    #[test]
    fn row_lookup_for_metal_triad_year_returns_correct_branches() {
        // Dậu(9) is in Metal triad → opposite triad is Hợi/Mão/Mùi.
        let r = tam_sat_direction(9);
        assert_eq!(r.tam_sat_branches.as_slice(), &["Hợi", "Mão", "Mùi"]);
        assert_eq!(
            r.tam_sat_directions,
            [Direction::Northwest, Direction::East, Direction::Southwest]
        );
    }

    #[test]
    fn tradition_order_preserved_for_water_triad() {
        // NOT branch-sorted (which would be [Tý, Thìn, Thân]).
        let r = tam_sat_direction(0);
        assert_eq!(r.tam_hop_group.as_slice(), &["Thân", "Tý", "Thìn"]);
    }

    #[test]
    fn evidence_carries_khcbppt_source_id() {
        let r = tam_sat_direction(6);
        assert_eq!(r.evidence.source_id, SOURCE_KHCBPPT);
        assert!(r.evidence.profile.contains("PendingExternalReview"));
        assert!(r.evidence.profile.contains("tam_sat_provenance"));
    }

    #[test]
    #[should_panic(expected = "not in 0..=11")]
    fn out_of_range_year_chi_index_panics() {
        let _ = tam_sat_direction(12);
    }
}
