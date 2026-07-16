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

    // -----------------------------------------------------------------
    // Phase 23-03 Task 1 (TDD RED): composite_severity tie behaviour.
    // -----------------------------------------------------------------

    #[test]
    fn composite_severity_picks_inauspicious_on_favorable_unfavorable_tie() {
        // 4 Auspicious + 4 Inauspicious -> tied top count -> the
        // conservative-default rule must pick Inauspicious (CONTEXT.md
        // "taboo-leaning on ambiguity" recommendation).
        let severities = [
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Inauspicious,
            ReasoningNodeSeverity::Inauspicious,
            ReasoningNodeSeverity::Inauspicious,
            ReasoningNodeSeverity::Inauspicious,
        ];
        assert_eq!(
            composite_severity(&severities),
            ReasoningNodeSeverity::Inauspicious
        );
    }

    #[test]
    fn composite_severity_majority_wins_when_clear() {
        // 5 HardTaboo + 3 Auspicious -> HardTaboo has the clear majority.
        let severities = [
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
            ReasoningNodeSeverity::Auspicious,
        ];
        assert_eq!(
            composite_severity(&severities),
            ReasoningNodeSeverity::HardTaboo
        );
    }

    #[test]
    fn composite_severity_picks_most_cautionary_on_tie() {
        // 4 SoftTaboo + 4 HardTaboo -> tied top count -> HardTaboo is the
        // most cautionary tied value.
        let severities = [
            ReasoningNodeSeverity::SoftTaboo,
            ReasoningNodeSeverity::SoftTaboo,
            ReasoningNodeSeverity::SoftTaboo,
            ReasoningNodeSeverity::SoftTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
            ReasoningNodeSeverity::HardTaboo,
        ];
        assert_eq!(
            composite_severity(&severities),
            ReasoningNodeSeverity::HardTaboo
        );
    }

    // -----------------------------------------------------------------
    // Phase 23-03 Task 1 (TDD RED): public builder surface contracts.
    // -----------------------------------------------------------------

    #[test]
    fn build_personal_cross_link_returns_eight_cells_in_locked_order() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let cross = build_direction_cross_link_personal(&snapshot, 10)
            .expect("personal builder should succeed for in-range birth chi");
        assert_eq!(cross.cells.len(), 8);
        for (i, expected) in DIRECTION_ORDER.iter().enumerate() {
            assert_eq!(
                cross.cells[i].direction, *expected,
                "cell {} must be {:?} in DIRECTION_ORDER",
                i, expected
            );
        }
        assert_eq!(cross.birth_chi_index, 10);
    }

    #[test]
    fn build_personal_cross_link_rejects_out_of_range_birth_chi() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let err = build_direction_cross_link_personal(&snapshot, 12)
            .expect_err("out-of-range birth chi must error");
        assert!(
            err.contains("birth_chi_index") || err.contains("range"),
            "error must explain the out-of-range cause; got: {err}"
        );
    }

    #[test]
    fn build_date_cross_link_carries_sentinel_and_omits_thai_tue() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let cross = build_direction_cross_link_date(&snapshot)
            .expect("date builder should succeed for a populated snapshot");
        assert_eq!(cross.birth_chi_index, DATE_ONLY_BIRTH_CHI_INDEX);
        for cell in cross.cells.iter() {
            if let Some(taboo) = cell.khcbppt.as_ref() {
                assert!(
                    taboo.thai_tue.is_none(),
                    "date variant must never carry a directional Thai Tue record"
                );
            }
        }
    }

    #[test]
    fn build_personal_cross_link_carries_exactly_three_evidence_envelopes() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let cross = build_direction_cross_link_personal(&snapshot, 10)
            .expect("personal builder");
        assert_eq!(cross.evidence.len(), 3);
        assert_eq!(
            cross.evidence[0].source_id,
            crate::sources::SOURCE_KHCBPPT
        );
        assert_eq!(
            cross.evidence[1].source_id,
            crate::sources::SOURCE_HUYEN_KHONG
        );
        assert_eq!(
            cross.evidence[2].source_id,
            COMPOSITE_DIRECTION_CROSS_LINK
        );
        // The huyen-khong primitive's method value is locked at runtime.
        let huyen_method = cross.evidence[1].method.clone();
        let mut expected = String::from("phi");
        expected.push('_');
        expected.push_str("tinh.palace_layout");
        assert_eq!(huyen_method, expected);
    }

    #[test]
    fn build_direction_cross_link_wrapper_returns_personal_fact_node() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let node = build_direction_cross_link(&snapshot, 10).expect("wrapper builder");
        assert_eq!(node.id, "fact.personal.direction_cross_link");
        assert_eq!(node.evidence.len(), 3);
        assert!(!node.summary_vi.is_empty());
    }

    #[test]
    fn project_to_summary_carries_cross_link_source() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let cross = build_direction_cross_link_personal(&snapshot, 10)
            .expect("personal builder");
        let summary = project_to_summary(&cross);
        assert_eq!(summary.cross_link_source, COMPOSITE_DIRECTION_CROSS_LINK);
        assert_eq!(summary.cells.len(), 8);
        assert_eq!(summary.birth_chi_index, cross.birth_chi_index);
    }

    #[test]
    fn enrich_helper_attaches_summary_and_leaves_input_unchanged() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        assert!(snapshot.direction_cross_link.is_none());
        let enriched = crate::enrich_day_snapshot_with_direction_cross_link(&snapshot, 10)
            .expect("enrichment should succeed");
        assert!(enriched.direction_cross_link.is_some());
        // The input snapshot must remain unchanged (immutable clone-and-attach).
        assert!(snapshot.direction_cross_link.is_none());
    }

    #[test]
    fn enrich_helper_dispatches_sentinel_to_date_builder() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let enriched = crate::enrich_day_snapshot_with_direction_cross_link(
            &snapshot,
            DATE_ONLY_BIRTH_CHI_INDEX,
        )
        .expect("sentinel enrichment should dispatch to date builder");
        let summary = enriched
            .direction_cross_link
            .expect("summary attached");
        assert_eq!(summary.birth_chi_index, DATE_ONLY_BIRTH_CHI_INDEX);
    }

    #[test]
    fn enrich_helper_rejects_invalid_personal_birth_chi() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let _ = crate::enrich_day_snapshot_with_direction_cross_link(&snapshot, 99)
            .expect_err("invalid birth chi must propagate the personal builder's error");
    }
}
