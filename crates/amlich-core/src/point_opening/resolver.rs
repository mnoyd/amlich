//! Pure lookup from an existing day stem and hour-branch slot to one frozen
//! Tý Ngọ Lưu Chú record.
//!
//! This module deliberately performs no civil-time or day-pillar calculation.
//! Those boundary rules are integrated in [`super::civil_time`].  It translates
//! the existing core vocabulary to the frozen corpus keys and returns the
//! exact open or explicit-closed record only.

use crate::almanac::hour_pillar::HourBranchSlot;
use crate::almanac::types::HeavenlyStem;
use crate::types::CHI;

use super::corpus::{frozen_point_opening_record, FrozenPointOpeningRecord};

const STEMS_ZH: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const BRANCHES_ZH: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

/// Return the exact frozen result for a valid day stem and Vietnamese hour
/// branch label. Unknown branch labels return `None`; they never select a
/// neighboring slot.
pub fn resolve_frozen_point_opening(
    day_stem: HeavenlyStem,
    hour_branch: &str,
) -> Option<&'static FrozenPointOpeningRecord> {
    let branch_index = CHI.iter().position(|branch| *branch == hour_branch)?;
    frozen_point_opening_record(stem_zh(day_stem), BRANCHES_ZH[branch_index])
}

/// Resolve from the existing local-civil hour-branch primitive.
///
/// A malformed manually-created slot is unavailable rather than being coerced
/// to an adjacent corpus cell.
pub fn resolve_frozen_point_opening_slot(
    day_stem: HeavenlyStem,
    hour_slot: &HourBranchSlot,
) -> Option<&'static FrozenPointOpeningRecord> {
    if hour_slot.branch_index >= BRANCHES_ZH.len()
        || CHI[hour_slot.branch_index] != hour_slot.branch
    {
        return None;
    }
    frozen_point_opening_record(stem_zh(day_stem), BRANCHES_ZH[hour_slot.branch_index])
}

fn stem_zh(stem: HeavenlyStem) -> &'static str {
    let index = match stem {
        HeavenlyStem::Giap => 0,
        HeavenlyStem::At => 1,
        HeavenlyStem::Binh => 2,
        HeavenlyStem::Dinh => 3,
        HeavenlyStem::Mau => 4,
        HeavenlyStem::Ky => 5,
        HeavenlyStem::Canh => 6,
        HeavenlyStem::Tan => 7,
        HeavenlyStem::Nham => 8,
        HeavenlyStem::Quy => 9,
    };
    STEMS_ZH[index]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point_opening::PointOpeningSlotState;

    #[test]
    fn every_valid_stem_and_branch_returns_exactly_one_frozen_record() {
        for stem in HeavenlyStem::ALL {
            for branch in CHI {
                assert!(
                    resolve_frozen_point_opening(stem, branch).is_some(),
                    "{stem:?}/{branch} must resolve"
                );
            }
        }
    }

    #[test]
    fn open_substitution_and_closed_states_remain_the_frozen_truth() {
        let open = resolve_frozen_point_opening(HeavenlyStem::Quy, "Tý").unwrap();
        assert_eq!(open.day_stem_zh, "癸");
        assert_eq!(open.hour_branch_zh, "子");
        assert!(matches!(
            &open.context.state,
            PointOpeningSlotState::Open { substitution, .. }
                if substitution.as_deref() == Some("qi_na_san_jiao")
        ));

        let closed = resolve_frozen_point_opening(HeavenlyStem::Giap, "Tý").unwrap();
        assert!(matches!(
            closed.context.state,
            PointOpeningSlotState::Closed { .. }
        ));
    }

    #[test]
    fn slot_resolver_uses_existing_hour_branch_and_rejects_bad_slots() {
        let slot = HourBranchSlot {
            slot_index: 0,
            branch_index: 0,
            branch: "Tý".to_string(),
            time_range: "23:00-00:59".to_string(),
        };
        assert!(resolve_frozen_point_opening_slot(HeavenlyStem::Quy, &slot).is_some());

        let malformed = HourBranchSlot {
            branch: "Sửu".to_string(),
            ..slot
        };
        assert!(resolve_frozen_point_opening_slot(HeavenlyStem::Quy, &malformed).is_none());
        assert!(resolve_frozen_point_opening(HeavenlyStem::Quy, "not-a-branch").is_none());
    }
}
