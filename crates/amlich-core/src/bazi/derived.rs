//! Derived Bazi Computations — Thai Nguyên, Mệnh Cung, Thân Cung
//!
//! These are "virtual pillars" / palaces computed from existing chart data.
//! They enrich a Bazi reading beyond the four core pillars.
//!
//! Source: Classical Bazi methodology (子平真詮, 三命通會).

use crate::almanac::types::RuleEvidence;
use crate::types::CanChi;

use super::types::{MenhCungResult, ThaiNguyenResult};

const EVIDENCE_SOURCE: &str = "bazi-classical";
const EVIDENCE_PROFILE: &str = "baseline";

// ---------------------------------------------------------------------------
// Thai Nguyên (胎元 — Conception Pillar)
// ---------------------------------------------------------------------------

/// Compute the Conception Pillar (Thai Nguyên / 胎元).
///
/// Represents the month of conception, approximately 9-10 months before
/// birth.  The classical formula advances the month pillar by 3 positions:
///
/// - Stem:   month_can + 1 (mod 10)
/// - Branch: month_chi + 3 (mod 12)
///
/// # Arguments
/// * `month_can_index` — Heavenly Stem index of the month pillar (0-9)
/// * `month_chi_index` — Earthly Branch index of the month pillar (0-11)
pub fn compute_thai_nguyen(month_can_index: usize, month_chi_index: usize) -> ThaiNguyenResult {
    let can_index = (month_can_index + 1) % 10;
    let chi_index = (month_chi_index + 3) % 12;

    ThaiNguyenResult {
        can_chi: CanChi::new(can_index, chi_index),
        evidence: RuleEvidence {
            source_id: EVIDENCE_SOURCE.to_string(),
            method: "thai-nguyen-month-plus-3".to_string(),
            profile: EVIDENCE_PROFILE.to_string(),
        },
    }
}

// ---------------------------------------------------------------------------
// Mệnh Cung + Thân Cung (命宮 / 身宮)
// ---------------------------------------------------------------------------

/// Year-stem-to-first-month-stem mapping (Ngũ Hổ Độn / 五虎遁).
///
/// Same table used by `canchi::get_month_canchi`.
const FIRST_MONTH_CAN_TABLE: [usize; 10] = [2, 4, 6, 8, 0, 2, 4, 6, 8, 0];

/// Compute Mệnh Cung (Life Palace) and Thân Cung (Body Palace).
///
/// ## Mệnh Cung 命宮
/// Starting from Dần (branch 2) for lunar month 1, count forward by the
/// birth month, then count *backward* by the birth hour:
///
/// ```text
/// menh_branch = (2 + (lunar_month - 1) - hour_chi_index + 12) % 12
///             = (lunar_month - hour_chi_index + 13) % 12
/// ```
///
/// Simplified:  Dần is month 1.  Month N starts at branch `N + 1`.
/// Then subtract the hour branch index (Tý = 0 … Hợi = 11).
///
/// ## Thân Cung 身宮
/// Same starting point but count *forward* by hour:
///
/// ```text
/// than_branch = (2 + (lunar_month - 1) + hour_chi_index) % 12
///             = (lunar_month + hour_chi_index + 1) % 12
/// ```
///
/// ## Stem derivation
/// The stem is derived from the year stem using the Ngũ Hổ Độn table,
/// with the branch offset from Dần applied as for month stems.
///
/// # Arguments
/// * `lunar_month`     — Lunar month of birth (1-12)
/// * `hour_chi_index`  — Earthly Branch index of birth hour (0-11)
/// * `year_can_index`  — Heavenly Stem index of birth year (0-9)
pub fn compute_menh_than_cung(
    lunar_month: i32,
    hour_chi_index: usize,
    year_can_index: usize,
) -> MenhCungResult {
    // Branch of Mệnh Cung
    let menh_branch = ((lunar_month as isize) - (hour_chi_index as isize) + 13 + 12) as usize % 12;

    // Branch of Thân Cung
    let than_branch = ((lunar_month as isize) + (hour_chi_index as isize) + 1) as usize % 12;

    // Derive stems via Ngũ Hổ Độn: stem for Dần (branch 2) is given by the
    // table, then each subsequent branch advances the stem by 1.
    let dan_stem = FIRST_MONTH_CAN_TABLE[year_can_index];

    let menh_stem = (dan_stem + (menh_branch + 12 - 2) % 12) % 10;
    let than_stem = (dan_stem + (than_branch + 12 - 2) % 12) % 10;

    MenhCungResult {
        menh_cung: CanChi::new(menh_stem, menh_branch),
        than_cung: CanChi::new(than_stem, than_branch),
        evidence: RuleEvidence {
            source_id: EVIDENCE_SOURCE.to_string(),
            method: "menh-cung-month-hour-counter".to_string(),
            profile: EVIDENCE_PROFILE.to_string(),
        },
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- Thai Nguyên ----

    #[test]
    fn thai_nguyen_giap_dan_yields_at_ty() {
        // Month pillar: Giáp(0) Dần(2) → Conception: Ất(1) Tỵ(5)
        let r = compute_thai_nguyen(0, 2);
        assert_eq!(r.can_chi.can, "Ất");
        assert_eq!(r.can_chi.chi, "Tỵ");
    }

    #[test]
    fn thai_nguyen_quy_suu_wraps_correctly() {
        // Month: Quý(9) Sửu(1) → Can: (9+1)%10=0=Giáp, Chi: (1+3)%12=4=Thìn
        let r = compute_thai_nguyen(9, 1);
        assert_eq!(r.can_chi.can, "Giáp");
        assert_eq!(r.can_chi.chi, "Thìn");
    }

    #[test]
    fn thai_nguyen_wraps_chi_past_hoi() {
        // Month: Mậu(4) Tuất(10) → Can: 5=Kỷ, Chi: (10+3)%12=1=Sửu
        let r = compute_thai_nguyen(4, 10);
        assert_eq!(r.can_chi.can, "Kỷ");
        assert_eq!(r.can_chi.chi, "Sửu");
    }

    #[test]
    fn thai_nguyen_evidence() {
        let r = compute_thai_nguyen(0, 0);
        assert_eq!(r.evidence.source_id, "bazi-classical");
        assert_eq!(r.evidence.method, "thai-nguyen-month-plus-3");
    }

    // ---- Mệnh Cung / Thân Cung ----

    #[test]
    fn menh_cung_month1_hour_ty() {
        // Lunar month 1, hour Tý(0), year stem Giáp(0)
        // menh_branch = (1 - 0 + 13) % 12 = 14 % 12 = 2 = Dần
        // than_branch = (1 + 0 + 1) % 12 = 2 = Dần
        // Both land on Dần when month=1, hour=Tý
        let r = compute_menh_than_cung(1, 0, 0);
        assert_eq!(r.menh_cung.chi, "Dần");
        assert_eq!(r.than_cung.chi, "Dần");
    }

    #[test]
    fn menh_cung_month1_hour_ngo() {
        // Lunar month 1, hour Ngọ(6), year stem Giáp(0)
        // menh_branch = (1 - 6 + 13 + 12) % 12 = 20 % 12 = 8 = Thân
        // than_branch = (1 + 6 + 1) % 12 = 8 = Thân
        let r = compute_menh_than_cung(1, 6, 0);
        assert_eq!(r.menh_cung.chi, "Thân");
        assert_eq!(r.than_cung.chi, "Thân");
    }

    #[test]
    fn menh_cung_month6_hour_dan() {
        // Lunar month 6, hour Dần(2), year stem Giáp(0)
        // menh_branch = (6 - 2 + 13) % 12 = 17 % 12 = 5 = Tỵ
        // than_branch = (6 + 2 + 1) % 12 = 9 = Dậu
        let r = compute_menh_than_cung(6, 2, 0);
        assert_eq!(r.menh_cung.chi, "Tỵ");
        assert_eq!(r.than_cung.chi, "Dậu");
    }

    #[test]
    fn menh_cung_stems_follow_ngu_ho_don() {
        // Year Giáp(0) → Dần stem = Bính(2) from table
        // Month 1, hour Tý(0) → menh_branch = Dần(2)
        // menh_stem = (2 + (2-2)%12) % 10 = 2 = Bính
        let r = compute_menh_than_cung(1, 0, 0);
        assert_eq!(r.menh_cung.can, "Bính");

        // Year Ất(1) → Dần stem = Mậu(4)
        let r2 = compute_menh_than_cung(1, 0, 1);
        assert_eq!(r2.menh_cung.can, "Mậu");
    }

    #[test]
    fn menh_cung_symmetric_at_noon() {
        // When hour is Ngọ (index 6), Mệnh and Thân should have the same
        // branch because forward and backward by 6 from the same point
        // differ by 12 (≡ 0 mod 12).
        for month in 1..=12 {
            let r = compute_menh_than_cung(month, 6, 0);
            assert_eq!(
                r.menh_cung.chi_index, r.than_cung.chi_index,
                "month={month}: menh and than branches should match at hour Ngọ"
            );
        }
    }

    #[test]
    fn menh_cung_evidence() {
        let r = compute_menh_than_cung(1, 0, 0);
        assert_eq!(r.evidence.source_id, "bazi-classical");
        assert_eq!(r.evidence.method, "menh-cung-month-hour-counter");
    }
}
