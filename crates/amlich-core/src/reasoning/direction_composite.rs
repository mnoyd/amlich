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

use crate::almanac::tu_menh::Direction;

/// Composite rule identifier carried by the join envelope (audit-friendly
/// single named constant; not a corpus source_id).
pub const COMPOSITE_DIRECTION_CROSS_LINK: &str = "rule.composite.direction_cross_link";

/// Sentinel for the date-only variant: every real branch index sits in
/// `0..=11`, so `usize::MAX` carries the explicit "no birth context" meaning.
pub const DATE_ONLY_BIRTH_CHI_INDEX: usize = 0; // RED stub — GREEN swaps to usize::MAX.

/// Locked eight-element ordering matching the existing interaction-layer
/// directional convention (North, Northeast, East, Southeast, South,
/// Southwest, West, Northwest).
pub const DIRECTION_ORDER: [Direction; 8] = [Direction::North; 8]; // RED stub.

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
}
