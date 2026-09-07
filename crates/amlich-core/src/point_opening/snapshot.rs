//! DaySnapshot projection for the frozen point-opening context
//! (bead `amlich-xlag.2.2.6`).
//!
//! The civil-time resolver (`amlich-xlag.2.2.4`) returns a
//! [`LocalCivilPointOpening`] borrowing the static frozen record — the
//! right shape for engine internals but not for a serializable
//! snapshot field. This module freezes that moment into the additive
//! [`DayPointOpeningContext`] DTO:
//!
//! - the slot identity (day stem, hour branch, hour pillar) as
//!   printed in the corpus;
//! - the civil/slot day-pillar pair and the TNLC-DIV-03 late-night
//!   transition marker from the existing calendar conventions;
//! - the full [`PointOpeningContext`] carrier (policy id, exactly-one
//!   open-or-explicit-closed state, disclaimer v2, review states,
//!   safety class, time basis, divergences);
//! - the per-row provenance block plus the two never-merged evidence
//!   vectors from bead `amlich-xlag.2.2.5`: method evidence
//!   (`ty-ngo-luu-chu` only) and calendar evidence
//!   (`amlich-calendar-engine` + `khcbppt` only).
//!
//! The context is attached to `DaySnapshot` as its own additive
//! `point_opening` field — separate from v1.10 `traditional_wellness`,
//! never feeding Day Assessment, Hour Ranking, or Direction Assessment
//! (ADR-0003 / ADR-0004).

use serde::{Deserialize, Serialize};

use crate::reasoning::ReasoningEvidenceEnvelope;
use crate::types::CanChi;

use super::civil_time::resolve_frozen_point_opening_at_local_civil_time;
use super::provenance::PointOpeningProvenance;
use super::state::PointOpeningContext;

/// The serializable point-opening context for one local civil moment
/// on a `DaySnapshot` (bead `amlich-xlag.2.2.6`). Carries the frozen
/// slot's identity, disclosures, provenance, and evidence so surfaces
/// never need to re-resolve the corpus.
///
/// Separation from v1.10: this type never embeds a
/// `TraditionalWellnessContext` and rides its own
/// `DaySnapshot.point_opening` field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DayPointOpeningContext {
    /// Day stem of the cell-owning day pillar, exactly as printed
    /// (e.g. 甲).
    pub day_stem_zh: String,
    /// Hour branch of the slot, exactly as printed (e.g. 子).
    pub hour_branch_zh: String,
    /// Full hour pillar of the slot as printed (e.g. 甲戌).
    pub hour_pillar_zh: String,
    /// Vietnamese hour-branch label from the existing hour-branch slot
    /// convention (e.g. Tý).
    pub hour_branch_vi: String,
    /// Local civil time range of the hour block (e.g. 23:00-00:59).
    pub hour_time_range: String,
    /// Slot index of the hour block within the twelve-branch cycle
    /// (0 = Tý … 11 = Hợi).
    pub hour_slot_index: usize,
    /// Day pillar of the snapshot's civil date (existing
    /// `get_day_canchi` over `jd`).
    pub civil_day_canchi: CanChi,
    /// Day pillar owning the cell: equal to `civil_day_canchi` except
    /// at 23:00–23:59, where it advances one Julian day (TNLC-DIV-03).
    pub slot_day_canchi: CanChi,
    /// True exactly at 23:00–23:59 — the Tý block attributed to the
    /// upcoming civil date (TNLC-DIV-03).
    pub late_night_day_transition: bool,
    /// The frozen cross-day spillover marker of the resolved row.
    pub cross_day_spillover: bool,
    /// The full disclosure carrier: policy id, exactly-one open or
    /// explicit-closed state, disclaimer v2, review states, safety
    /// class, time basis, and divergences.
    pub context: PointOpeningContext,
    /// Per-row provenance block (work citations + table evidence).
    pub provenance: PointOpeningProvenance,
    /// Method evidence: the frozen point-opening lookup, citing only
    /// the reserved `ty-ngo-luu-chu` primitive source (bead
    /// `amlich-xlag.2.2.5`). Absent from JSON when empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub method_evidence: Vec<ReasoningEvidenceEnvelope>,
    /// Calendar evidence: the day pillar and hour-branch slot computed
    /// by the existing calendar conventions, never citing the TNLC
    /// primitive source. Absent from JSON when empty.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub calendar_evidence: Vec<ReasoningEvidenceEnvelope>,
}

/// Freeze the point-opening context of one local civil moment into
/// the additive [`DayPointOpeningContext`] DTO.
///
/// `jd` is the Julian day number of the local civil date — the same
/// value day snapshots already carry — and `local_hour` /
/// `local_minute` are local civil time, as everywhere else in Amlich.
/// Returns `None` exactly when the existing hour-branch contract
/// rejects the time (`local_hour > 23` or `local_minute > 59`); every
/// valid moment resolves to exactly one frozen open or explicit-closed
/// record.
pub fn resolve_day_point_opening_context(
    jd: i32,
    local_hour: u8,
    local_minute: u8,
) -> Option<DayPointOpeningContext> {
    let moment = resolve_frozen_point_opening_at_local_civil_time(jd, local_hour, local_minute)?;
    let record = moment.record;
    Some(DayPointOpeningContext {
        day_stem_zh: record.day_stem_zh.clone(),
        hour_branch_zh: record.hour_branch_zh.clone(),
        hour_pillar_zh: record.hour_pillar_zh.clone(),
        hour_branch_vi: moment.hour_slot.branch.clone(),
        hour_time_range: moment.hour_slot.time_range.clone(),
        hour_slot_index: moment.hour_slot.slot_index,
        civil_day_canchi: moment.civil_day_canchi.clone(),
        slot_day_canchi: moment.slot_day_canchi.clone(),
        late_night_day_transition: moment.late_night_day_transition,
        cross_day_spillover: record.cross_day_spillover,
        context: record.context.clone(),
        provenance: record.provenance.clone(),
        method_evidence: record.reasoning_evidence(),
        calendar_evidence: moment.calendar_evidence(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point_opening::PointOpeningSlotState;
    use crate::sources::{SOURCE_KHCBPPT, SOURCE_TY_NGO_LUU_CHU};

    /// 2024-02-10, the verified Giáp-day Julian fixture shared with
    /// the civil-time tests.
    const JD_GIAP_DAY: i32 = 2460351;

    #[test]
    fn invalid_civil_times_resolve_to_none() {
        for (hour, minute) in [(24u8, 0u8), (23, 60), (255, 0)] {
            assert!(
                resolve_day_point_opening_context(JD_GIAP_DAY, hour, minute).is_none(),
                "{hour}:{minute:02} must be rejected by the hour-branch contract"
            );
        }
    }

    #[test]
    fn every_valid_moment_freezes_the_slot_identity_and_context() {
        for offset in 0..10 {
            let jd = JD_GIAP_DAY + offset;
            for hour in [0u8, 6, 12, 18, 23] {
                let ctx =
                    resolve_day_point_opening_context(jd, hour, 30).expect("valid moment resolves");
                let record = crate::point_opening::frozen_point_opening_record(
                    &ctx.day_stem_zh,
                    &ctx.hour_branch_zh,
                )
                .expect("slot identity must resolve back to the frozen corpus");
                assert_eq!(ctx.hour_pillar_zh, record.hour_pillar_zh);
                assert_eq!(ctx.context, record.context);
                assert_eq!(ctx.provenance, record.provenance);
                assert_eq!(ctx.cross_day_spillover, record.cross_day_spillover);
                assert!(!ctx.hour_branch_vi.is_empty());
                assert!(!ctx.hour_time_range.is_empty());
            }
        }
    }

    #[test]
    fn method_and_calendar_evidence_stay_source_separated() {
        for hour in [0u8, 12, 23] {
            let ctx =
                resolve_day_point_opening_context(JD_GIAP_DAY, hour, 30).expect("valid moment");
            assert!(!ctx.method_evidence.is_empty());
            assert_eq!(ctx.calendar_evidence.len(), 2);
            for envelope in &ctx.method_evidence {
                assert_eq!(envelope.source_id, SOURCE_TY_NGO_LUU_CHU);
                assert_ne!(envelope.source_id, SOURCE_KHCBPPT);
                assert_ne!(envelope.source_id, "amlich-calendar-engine");
            }
            for envelope in &ctx.calendar_evidence {
                assert_ne!(envelope.source_id, SOURCE_TY_NGO_LUU_CHU);
            }
        }
    }

    #[test]
    fn late_night_transition_rides_the_context() {
        let ordinary =
            resolve_day_point_opening_context(JD_GIAP_DAY, 12, 30).expect("valid moment");
        assert!(!ordinary.late_night_day_transition);
        assert_eq!(ordinary.civil_day_canchi, ordinary.slot_day_canchi);

        let late = resolve_day_point_opening_context(JD_GIAP_DAY, 23, 30).expect("valid moment");
        assert!(late.late_night_day_transition);
        assert_ne!(late.civil_day_canchi, late.slot_day_canchi);
        assert!(late
            .calendar_evidence
            .first()
            .and_then(|e| e.note.as_deref())
            .unwrap_or_default()
            .contains("TNLC-DIV-03"));
    }

    #[test]
    fn contexts_round_trip_byte_equal_for_open_and_closed_slots() {
        let mut seen_open = false;
        let mut seen_closed = false;
        for hour in 0u8..=23 {
            let Some(original) = resolve_day_point_opening_context(JD_GIAP_DAY, hour, 15) else {
                continue;
            };
            match original.context.state {
                PointOpeningSlotState::Open { .. } => seen_open = true,
                PointOpeningSlotState::Closed { .. } => seen_closed = true,
            }
            let json = serde_json::to_string(&original).unwrap();
            let recovered: DayPointOpeningContext = serde_json::from_str(&json).unwrap();
            assert_eq!(recovered, original);
            assert_eq!(serde_json::to_string(&recovered).unwrap(), json);
        }
        assert!(seen_open, "fixture day must cover at least one open slot");
        assert!(
            seen_closed,
            "fixture day must cover at least one closed slot"
        );
    }
}
