//! Civil-time boundary integration for the frozen Tý Ngọ Lưu Chú
//! resolver (bead `amlich-xlag.2.2.4`).
//!
//! Connects [`super::resolver`] — the pure (day stem × hour branch)
//! lookup — to Amlich's existing local-civil conventions without
//! introducing any independent timezone, DST, or day-boundary
//! arithmetic:
//!
//! - the hour block comes from
//!   [`crate::almanac::hour_pillar::resolve_hour_branch_slot`] (Tý
//!   23:00–00:59 … Hợi 21:00–22:59), reused unchanged;
//! - the day pillar comes from [`crate::canchi::get_day_canchi`] over
//!   the caller's civil-date Julian day number, with the single frozen
//!   exception of the corpus `day_attribution_rule` (TNLC-DIV-03): the
//!   23:00–01:00 Tý block belongs to the civil date containing its
//!   00:00–01:00 half, so at 23:00–23:59 the cell day stem is the one
//!   Julian day later.
//!
//! Every result carries the disclosure contract: `time_basis =
//! local_civil_hour_branch` plus the applicable `TNLC-DIV-*` ids
//! (always including TNLC-DIV-03 on grid cells) ride the frozen
//! record's [`super::state::PointOpeningContext`].

use crate::almanac::hour_pillar::{resolve_hour_branch_slot, HourBranchSlot};
use crate::almanac::types::HeavenlyStem;
use crate::canchi::get_day_canchi;
use crate::traditional_wellness::divergence::TimeBasis;
use crate::types::CanChi;

use super::corpus::FrozenPointOpeningRecord;
use super::resolver::resolve_frozen_point_opening_slot;
use super::state::PointOpeningSlotState;

/// The frozen day-attribution boundary (corpus metadata
/// `day_attribution_rule`, TNLC-DIV-03): from 23:00 the Tý block
/// belongs to the upcoming civil date, one Julian day later.
const LATE_NIGHT_ROLLOVER_HOUR: u8 = 23;

/// One resolved local-civil moment: the civil-date day pillar, the
/// day pillar of the cell that owns the hour block, the existing
/// hour-branch slot, and the one frozen record with its disclosure
/// context.
#[derive(Debug, Clone, PartialEq)]
pub struct LocalCivilPointOpening {
    /// Day pillar of the given civil date (existing `get_day_canchi`
    /// over `jd`).
    pub civil_day_canchi: CanChi,
    /// Day pillar owning the cell: equal to `civil_day_canchi` except
    /// at 23:00–23:59, where it advances one Julian day.
    pub slot_day_canchi: CanChi,
    /// True exactly at 23:00–23:59 — the Tý block attributed to the
    /// upcoming civil date (TNLC-DIV-03).
    pub late_night_day_transition: bool,
    /// The existing local-civil hour-branch slot from
    /// `resolve_hour_branch_slot`, never redefined here.
    pub hour_slot: HourBranchSlot,
    /// The one frozen record: an open state or the explicit closed
    /// state, exactly as frozen.
    pub record: &'static FrozenPointOpeningRecord,
}

impl LocalCivilPointOpening {
    /// Disclosed time basis (TNLC-DIV-03); always
    /// `local_civil_hour_branch`.
    pub fn time_basis(&self) -> &TimeBasis {
        &self.record.context.time_basis
    }

    /// Applicable `TNLC-DIV-*` divergences riding the frozen record;
    /// grid cells always include `TNLC-DIV-03`.
    pub fn known_divergence_ids(&self) -> &[String] {
        &self.record.context.known_divergence_ids
    }

    /// The resolved slot state — exactly one open or explicit closed.
    pub fn state(&self) -> &PointOpeningSlotState {
        &self.record.context.state
    }
}

/// Resolve the one frozen point-opening record for a local civil
/// moment.
///
/// `jd` is the Julian day number of the local civil date — the same
/// value day snapshots already carry — and `local_hour` /
/// `local_minute` are local civil time, as everywhere else in Amlich.
/// No timezone or DST conversion happens here; callers pass local
/// civil values per the existing snapshot conventions.
///
/// Returns `None` exactly when
/// [`crate::almanac::hour_pillar::resolve_hour_branch_slot`] rejects
/// the time (`local_hour > 23` or `local_minute > 59`); every valid
/// moment resolves to exactly one frozen open or explicit-closed
/// record, never an interpolated or later-school cell.
pub fn resolve_frozen_point_opening_at_local_civil_time(
    jd: i32,
    local_hour: u8,
    local_minute: u8,
) -> Option<LocalCivilPointOpening> {
    let hour_slot = resolve_hour_branch_slot(local_hour, local_minute)?;
    let late_night = local_hour == LATE_NIGHT_ROLLOVER_HOUR;
    let slot_jd = jd + i32::from(late_night);
    let civil_day_canchi = get_day_canchi(jd);
    let slot_day_canchi = get_day_canchi(slot_jd);
    let day_stem = HeavenlyStem::ALL[slot_day_canchi.can_index];
    let record = resolve_frozen_point_opening_slot(day_stem, &hour_slot)?;
    Some(LocalCivilPointOpening {
        civil_day_canchi,
        slot_day_canchi,
        late_night_day_transition: late_night,
        hour_slot,
        record,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point_opening::PointOpeningSlotState;

    /// 2024-02-10, the verified Giáp-day Julian fixture from
    /// `canchi.rs`.
    const JD_GIAP_DAY: i32 = 2460351;
    const STEMS_ZH: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];

    #[test]
    fn day_attribution_follows_the_frozen_convention() {
        for offset in 0..10 {
            let jd = JD_GIAP_DAY + offset;
            for hour in 0u8..=23 {
                for &minute in &[0u8, 59u8] {
                    let result = resolve_frozen_point_opening_at_local_civil_time(jd, hour, minute)
                        .expect("every valid civil moment must resolve");
                    let expected_rollover = hour == LATE_NIGHT_ROLLOVER_HOUR;
                    assert_eq!(
                        result.late_night_day_transition, expected_rollover,
                        "{jd} {hour}:{minute:02}"
                    );
                    assert_eq!(
                        result.slot_day_canchi,
                        get_day_canchi(jd + i32::from(expected_rollover)),
                        "{jd} {hour}:{minute:02}"
                    );
                    assert_eq!(result.civil_day_canchi, get_day_canchi(jd));
                    assert_eq!(
                        result.record.day_stem_zh, STEMS_ZH[result.slot_day_canchi.can_index],
                        "{jd} {hour}:{minute:02}"
                    );
                }
            }
        }
    }

    #[test]
    fn every_valid_moment_discloses_time_basis_and_tnlc_div_03() {
        for offset in 0..10 {
            let jd = JD_GIAP_DAY + offset;
            for hour in [0u8, 1, 12, 22, 23] {
                let result = resolve_frozen_point_opening_at_local_civil_time(jd, hour, 30)
                    .expect("valid civil moment must resolve");
                assert_eq!(result.time_basis(), &TimeBasis::LocalCivilHourBranch);
                assert!(result
                    .known_divergence_ids()
                    .contains(&"TNLC-DIV-03".to_string()));
                assert!(matches!(
                    result.state(),
                    PointOpeningSlotState::Open { .. } | PointOpeningSlotState::Closed { .. }
                ));
            }
        }
    }

    #[test]
    fn invalid_civil_times_are_rejected_by_the_existing_contract() {
        for (hour, minute) in [(24u8, 0u8), (23, 60), (255, 0)] {
            assert!(
                resolve_frozen_point_opening_at_local_civil_time(JD_GIAP_DAY, hour, minute)
                    .is_none(),
                "{hour}:{minute:02} must be rejected by the hour-branch contract"
            );
        }
    }
}
