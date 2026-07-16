//! Tam Sát (三殺) — Classical three-direction yearly killing direction.
//!
//! Distinct from `tam_tai.rs` (which encodes the three-year Tam Tai
//! affliction period sharing the same Chinese name). This module encodes
//! the *directional* Tam Sát rule: for each year Earthly Branch, three
//! contiguous directions are taboo — the three branches of the lục-xung
//! opposite Tam Hợp triad, each mapped to an 8-point `Direction` cell.
//!
//! **Source:** KHCBPPT (三殺). Exact edition/page citation pending — see
//! `crates/amlich-core/data/almanac/tam_sat_provenance.md`.

use serde::{Deserialize, Serialize};

use super::tu_menh::Direction;
use super::types::RuleEvidence;

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

/// Derive the classical three-direction Tam Sát (三殺) for a given year
/// Earthly Branch.
///
/// Implementation pending — RED phase stub.
pub fn tam_sat_direction(_year_chi_index: usize) -> TamSatDirectionResult {
    unimplemented!("RED phase: tam_sat_direction not yet implemented")
}
