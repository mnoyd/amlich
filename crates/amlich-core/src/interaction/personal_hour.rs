use crate::almanac::hour_pillar::compute_hour_pillar;
use crate::almanac::thap_than::get_thap_than;
use crate::almanac::types::{FiveElement, HeavenlyStem, RuleEvidence};
use crate::bazi::analysis::ElementDistribution;
use crate::bazi::types::BaziChart;
use crate::gio_hoang_dao::{get_gio_hoang_dao, get_hour_time_range};
use crate::types::{CanChi, CHI};

use super::day_person::compute_branch_relation;
use super::types::{ElementInteraction, PersonalHourEntry, PersonalHourMatrix};

/// Find the weakest element (lowest score) in a distribution.
pub fn weakest_element(dist: &ElementDistribution) -> FiveElement {
    let scores = [
        (FiveElement::Moc, dist.moc),
        (FiveElement::Hoa, dist.hoa),
        (FiveElement::Tho, dist.tho),
        (FiveElement::Kim, dist.kim),
        (FiveElement::Thuy, dist.thuy),
    ];
    scores
        .iter()
        .min_by_key(|(_, s)| *s)
        .map(|(e, _)| *e)
        .unwrap()
}

/// Compute the Personal Hour Matrix for a given day and person.
///
/// Ranks each of 12 traditional hours by personal compatibility using:
/// - Generic Hoàng Đạo quality (12-star system)
/// - Thập Thần: hour stem → person's Nhật Chủ
/// - Branch relation: hour chi × person's birth hour chi
/// - Element support: whether the hour's stem element matches the person's weakest element
pub fn compute_personal_hour_matrix(
    day_canchi: &CanChi,
    chart: &BaziChart,
    element_dist: &ElementDistribution,
) -> Option<PersonalHourMatrix> {
    crate::build_count::personal_hour_matrix_built();
    let day_stem = HeavenlyStem::ALL[day_canchi.can_index];
    let day_master_stem = HeavenlyStem::ALL[chart.day_master.can_index];
    let birth_hour_chi_index = chart.hour_pillar.as_ref()?.can_chi.chi_index;
    let weak = weakest_element(element_dist);

    let hoang_dao = get_gio_hoang_dao(day_canchi.chi_index);

    let hours: Vec<PersonalHourEntry> = (0usize..12)
        .map(|slot| {
            // Slot 0 is Tý (23:00-01:00). For slot s (s>0) the wall-clock
            // start hour is 2s-1 (Sửu=01:00, Dần=03:00, ...). The previous
            // formula `(slot * 2 + 1)` started at hour=1 for slot=0, which
            // `resolve_hour_branch_slot` mapped to Sửu — leaving row 0
            // with Sửu chi but Tý's Hoàng Đạo star. See amlich-mwbp.2.
            let wall_hour = if slot == 0 { 23 } else { (2 * slot - 1) as u8 };
            let hour_pillar =
                compute_hour_pillar(day_stem, wall_hour, 0).expect("slot 0-11 always resolves");
            let hour_stem = HeavenlyStem::ALL[hour_pillar.can_chi.can_index];
            let hour_chi_index = hour_pillar.can_chi.chi_index;

            let is_hoang_dao = hoang_dao.all_hours[slot].is_good;
            let star_name = hoang_dao.all_hours[slot].star.clone();

            let thap_than = get_thap_than(hour_stem, day_master_stem);
            let branch_rel = compute_branch_relation(hour_chi_index, birth_hour_chi_index);
            let element_interaction = ElementInteraction::from(thap_than.relation);

            let supports_weak = hour_stem.element() == weak;

            let score = compute_hour_score(is_hoang_dao, &thap_than, &branch_rel, supports_weak);

            PersonalHourEntry {
                chi_index: hour_chi_index,
                chi: CHI[hour_chi_index].to_string(),
                canchi: hour_pillar.can_chi.full.clone(),
                time_range: get_hour_time_range(hour_chi_index).to_string(),
                is_hoang_dao,
                star_name,
                thap_than_to_day_master: thap_than,
                branch_relation_to_birth_hour: branch_rel,
                element_interaction,
                supports_weak_element: supports_weak,
                score,
            }
        })
        .collect();

    Some(PersonalHourMatrix {
        day_canchi: day_canchi.full.clone(),
        day_master: chart.day_master.full.clone(),
        birth_hour_chi: CHI[birth_hour_chi_index].to_string(),
        weak_element: weak,
        hours,
        evidence: RuleEvidence {
            source_id: crate::sources::SOURCE_KHCBPPT.to_string(),
            method: "personal-hour-matrix".to_string(),
            profile: "baseline".to_string(),
        },
    })
}

/// Weighted composite score for one hour slot (0-100).
///
/// Weights:
/// - Hoàng Đạo generic:       30 pts
/// - Thập Thần favorability:   30 pts
/// - Branch harmony/conflict:  25 pts
/// - Element support:          15 pts
fn compute_hour_score(
    is_hoang_dao: bool,
    thap_than: &crate::almanac::types::ThapThanResult,
    branch_rel: &super::types::BranchRelation,
    supports_weak: bool,
) -> u8 {
    let mut score: i32 = 50; // neutral baseline

    // Hoàng Đạo: +20 / -10
    if is_hoang_dao {
        score += 20;
    } else {
        score -= 10;
    }

    // Thập Thần favorability
    use crate::almanac::types::ThapThanLabel;
    score += match thap_than.label {
        ThapThanLabel::ChinhAn | ThapThanLabel::ThienAn => 15, // Seal = support
        ThapThanLabel::TyKien | ThapThanLabel::KiepTai => 5,   // Peer = mild support
        ThapThanLabel::ThucThan => 10,                         // Food God = positive
        ThapThanLabel::ThuongQuan => -5,                       // Injured Officer = mild drain
        ThapThanLabel::ChinhTai | ThapThanLabel::ThienTai => 0, // Wealth = neutral
        ThapThanLabel::ChinhQuan => 5,                         // Direct Officer = structure
        ThapThanLabel::ThatSat => -10,                         // Seven Killings = pressure
    };

    // Branch harmony/conflict — consume the typed branch relation.
    if branch_rel.luc_hop {
        score += 15;
    }
    // Tam hợp only counts when the two branches are distinct members
    // of the same triad; same-branch is not a positive Tam hợp pair
    // (see `BranchRelation::is_tam_hop_pair` and the audit doc).
    if branch_rel.is_tam_hop_pair() {
        score += 10;
    }
    if branch_rel.luc_xung {
        score -= 15;
    }
    if branch_rel.tuong_hai {
        score -= 10;
    }
    // Tương hình subtracts only for actual punishments
    // (directed, completed triad, or self-punishment). Unavailable
    // two-branch incomplete triads do not score.
    if matches!(
        branch_rel.tuong_hinh,
        crate::almanac::types::PunishmentKind::DirectedPair { .. }
            | crate::almanac::types::PunishmentKind::CompletedTriad { .. }
            | crate::almanac::types::PunishmentKind::SelfPunishment { .. }
    ) {
        score -= 10;
    }

    // Element support
    if supports_weak {
        score += 15;
    }

    score.clamp(0, 100) as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bazi::types::{BaziChartMetadata, BaziInput, BaziPillar, PillarKind};

    fn make_pillar(kind: PillarKind, can_index: usize, chi_index: usize) -> BaziPillar {
        BaziPillar {
            kind,
            can_chi: CanChi::new(can_index, chi_index),
            hidden_stems: vec![],
            na_am: None,
            stem_relation_to_day_master: None,
        }
    }

    fn make_chart(
        year: (usize, usize),
        month: (usize, usize),
        day: (usize, usize),
        hour: (usize, usize),
    ) -> BaziChart {
        let year_pillar = make_pillar(PillarKind::Year, year.0, year.1);
        let month_pillar = make_pillar(PillarKind::Month, month.0, month.1);
        let day_pillar = make_pillar(PillarKind::Day, day.0, day.1);
        let hour_pillar = make_pillar(PillarKind::Hour, hour.0, hour.1);
        let day_master = day_pillar.can_chi.clone();
        let pillars = vec![
            year_pillar.clone(),
            month_pillar.clone(),
            day_pillar.clone(),
            hour_pillar.clone(),
        ];
        BaziChart {
            input: BaziInput {
                day: 1,
                month: 1,
                year: 2000,
                hour: 0,
                minute: 0,
                time_known: false,
                timezone: 7.0,
                longitude: None,
                use_solar_time: false,
                gender: None,
            },
            lunar_date: crate::lunar::LunarDate {
                day: 1,
                month: 1,
                year: 2000,
                is_leap: false,
            },
            year_pillar,
            month_pillar,
            day_pillar,
            hour_pillar: Some(hour_pillar),
            day_master,
            pillars,
            metadata: BaziChartMetadata {
                timezone: 7.0,
                use_solar_time: false,
                year_basis: "test".to_string(),
                month_basis: "test".to_string(),
                day_basis: "test".to_string(),
                hour_basis: "test".to_string(),
                hour_evidence: None,
            },
        }
    }

    fn balanced_dist() -> ElementDistribution {
        ElementDistribution {
            moc: 20,
            hoa: 20,
            tho: 20,
            kim: 20,
            thuy: 20,
        }
    }

    #[test]
    fn matrix_has_12_hours() {
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        assert_eq!(matrix.hours.len(), 12);
    }

    #[test]
    fn hours_cover_all_12_branches() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let chis: Vec<usize> = matrix.hours.iter().map(|h| h.chi_index).collect();
        for i in 0..12 {
            assert!(chis.contains(&i), "missing chi_index {i}");
        }
    }

    #[test]
    fn scores_are_in_valid_range() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        for h in &matrix.hours {
            assert!(h.score <= 100, "score {} exceeds 100", h.score);
        }
    }

    #[test]
    fn hoang_dao_hours_score_higher_than_hac_dao() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (0, 0), (0, 0)); // day_master = Giáp
        let dist = ElementDistribution {
            moc: 50,
            hoa: 50,
            tho: 50,
            kim: 50,
            thuy: 50,
        };
        let matrix = compute_personal_hour_matrix(&day, &chart, &dist).expect("matrix");
        let avg_hd: f64 = matrix
            .hours
            .iter()
            .filter(|h| h.is_hoang_dao)
            .map(|h| h.score as f64)
            .sum::<f64>()
            / matrix.hours.iter().filter(|h| h.is_hoang_dao).count() as f64;
        let avg_non: f64 = matrix
            .hours
            .iter()
            .filter(|h| !h.is_hoang_dao)
            .map(|h| h.score as f64)
            .sum::<f64>()
            / matrix.hours.iter().filter(|h| !h.is_hoang_dao).count() as f64;
        assert!(
            avg_hd > avg_non,
            "average Hoàng Đạo score ({avg_hd}) should exceed Hắc Đạo ({avg_non})"
        );
    }

    #[test]
    fn weak_element_is_detected() {
        let dist = ElementDistribution {
            moc: 10,
            hoa: 30,
            tho: 20,
            kim: 25,
            thuy: 15,
        };
        assert_eq!(weakest_element(&dist), FiveElement::Moc);
    }

    #[test]
    fn supports_weak_element_flag_is_set() {
        // Weak element = Thủy (lowest). An hour with Nhâm/Quý stem (Thủy) should flag.
        let day = CanChi::new(0, 0); // Giáp Tý → seed=0, so Tý hour = Giáp(Mộc)
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let dist = ElementDistribution {
            moc: 30,
            hoa: 30,
            tho: 30,
            kim: 30,
            thuy: 5, // weakest
        };
        let matrix = compute_personal_hour_matrix(&day, &chart, &dist).expect("matrix");
        assert_eq!(matrix.weak_element, FiveElement::Thuy);
        // At least one hour should have a Thủy stem (Nhâm or Quý)
        let any_supports = matrix.hours.iter().any(|h| h.supports_weak_element);
        assert!(any_supports, "at least one hour should support weak Thủy");
    }

    #[test]
    fn day_master_and_birth_hour_are_recorded() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        assert_eq!(matrix.day_master, chart.day_master.full);
        assert_eq!(matrix.birth_hour_chi, "Ngọ"); // chi_index 6
    }

    #[test]
    fn matrix_serializes_to_json() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let json = serde_json::to_string(&matrix).expect("should serialize");
        assert!(json.contains("\"hours\""));
        assert!(json.contains("\"score\""));
        assert!(json.contains("\"is_hoang_dao\""));
        assert!(json.contains("\"star_name\""));
    }

    #[test]
    fn evidence_metadata_is_set() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        assert_eq!(matrix.evidence.source_id, "khcbppt");
        assert_eq!(matrix.evidence.method, "personal-hour-matrix");
    }

    #[test]
    fn matrix_is_unavailable_without_birth_hour() {
        let day = CanChi::new(0, 0);
        let mut chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        chart.hour_pillar = None;
        chart.pillars.pop();

        assert!(compute_personal_hour_matrix(&day, &chart, &balanced_dist()).is_none());
    }

    /// Regression for amlich-mwbp.2: slot 0 must be Tý (23:00-01:00), not
    /// Sửu (01:00-03:00). The previous formula `(slot * 2 + 1)` produced
    /// hour=1 for slot=0, which `resolve_hour_branch_slot` mapped to the
    /// Sửu branch — leaving row 0 with a Sửu chi but Tý's Hoàng Đạo star.
    #[test]
    fn slot_0_is_ty_with_midnight_range() {
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let row = &matrix.hours[0];
        assert_eq!(row.chi, "Tý", "slot 0 chi");
        assert_eq!(row.chi_index, 0, "slot 0 chi_index");
        assert_eq!(row.time_range, "23:00-01:00", "slot 0 time range");
        // Giáp day stem seeds Tý hour stem as Giáp → "Giáp Tý"
        assert_eq!(row.canchi, "Giáp Tý", "slot 0 canchi");
    }

    /// Slot 1 must be Sửu (01:00-03:00), proving the rotation has no
    /// off-by-one between consecutive rows.
    #[test]
    fn slot_1_is_suu_with_early_morning_range() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let row = &matrix.hours[1];
        assert_eq!(row.chi, "Sửu", "slot 1 chi");
        assert_eq!(row.chi_index, 1, "slot 1 chi_index");
        assert_eq!(row.time_range, "01:00-03:00", "slot 1 time range");
    }

    /// All 12 rows must align chi, chi_index, time_range, and canchi by
    /// the same canonical index. Previously the chi/time_range advanced by
    /// one slot while the star stayed anchored to the outer loop index,
    /// producing desync rows.
    #[test]
    fn all_rows_align_chi_time_range_and_star_index() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");
        let hoang_dao = get_gio_hoang_dao(day.chi_index);
        for (slot, row) in matrix.hours.iter().enumerate() {
            assert_eq!(
                row.chi_index, slot,
                "slot {slot} chi_index must match row index"
            );
            assert_eq!(
                row.chi, CHI[slot],
                "slot {slot} chi label must match CHI[slot]"
            );
            // The star in the row must be the same star the Hoàng Đạo
            // table reports for that canonical chi_index.
            let expected_star = &hoang_dao.all_hours[slot].star;
            assert_eq!(
                &row.star_name, expected_star,
                "slot {slot} star must align with chi_index"
            );
            assert_eq!(
                row.is_hoang_dao, hoang_dao.all_hours[slot].is_good,
                "slot {slot} is_hoang_dao must align with chi_index"
            );
        }
    }

    /// Boundary regression: wall-clock 23:00, 00:00, and 00:01 must all
    /// resolve to Tý (slot 0); 01:00 must resolve to Sửu (slot 1).
    /// Validates `almanac::hour_pillar::resolve_hour_branch_slot` at the
    /// personal-hour boundary.
    #[test]
    fn midnight_boundaries_resolve_to_expected_slots() {
        use crate::almanac::hour_pillar::resolve_hour_branch_slot;
        for (hour, minute, expected) in [(23u8, 0u8, 0usize), (0, 0, 0), (0, 1, 0), (1, 0, 1)] {
            let slot = resolve_hour_branch_slot(hour, minute)
                .unwrap_or_else(|| panic!("({hour}:{minute:02}) should resolve"));
            assert_eq!(
                slot.slot_index, expected,
                "({hour}:{minute:02}) expected slot {expected}, got {}",
                slot.slot_index
            );
        }
    }

    /// Regression for amlich-mwbp.4: the personal-hour row whose chi
    /// equals the birth hour chi (same branch) must not receive the
    /// `+10` Tam hợp score bonus. The old code reported `tam_hop = true`
    /// for same-branch (membership) and the personal-hour scoring
    /// applied the +10, which was the audit's headline defect.
    #[test]
    fn same_branch_hour_row_does_not_get_tam_hop_bonus() {
        // Birth hour = Tý (slot 0, chi_index 0). The day stem Giáp seeds
        // the Tý hour to "Giáp Tý", so the Tý row exists in the matrix.
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 0), (2, 2), (4, 4), (0, 0)); // hour_pillar = Tý(0)
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");

        // Find the row whose chi matches the birth hour chi (Tý).
        let ty_row = matrix
            .hours
            .iter()
            .find(|h| h.chi_index == 0)
            .expect("Tý row must exist");

        // The branch relation for the Tý row vs the Tý birth hour is
        // same-branch. The new contract must mark it as same-branch
        // and must NOT report a friendly tam_hợp pair.
        assert!(ty_row.branch_relation_to_birth_hour.same_branch);
        assert_eq!(
            ty_row.branch_relation_to_birth_hour.tam_hop_member,
            Some(crate::almanac::types::TriadElement::Thuy),
            "same-branch is still a Thủy triad member"
        );
        assert!(
            !ty_row.branch_relation_to_birth_hour.is_tam_hop_pair(),
            "same-branch is NOT a friendly tam_hợp pair"
        );
    }

    /// Regression for amlich-mwbp.4: the 寅巳申 (Vô ân chi hình)
    /// canonical relation is exposed as `CompletedTriad(Hỏa)`. A row
    /// whose chi is in this punishment group and whose birth hour is
    /// also in it must report a `has_conflict()`.
    ///
    /// Note: 寅巳申 (Dần, Tỵ, Thân) is the punishment group, NOT the
    /// Hỏa Tam hợp triad (which is 寅午戌 = Dần, Ngọ, Tuất). The
    /// "Hỏa" label here is the punishment-group element per the
    /// source-cited decision brief.
    #[test]
    fn fire_triad_pair_reports_completed_triad_and_conflict() {
        // Birth hour = Dần (chi_index 2, in 寅巳申). The Tỵ row
        // (chi_index 5) is also in 寅巳申 — it must report a
        // CompletedTriad(Hỏa) Tương hình.
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (0, 2)); // hour_pillar = Dần(2)
        let matrix = compute_personal_hour_matrix(&day, &chart, &balanced_dist()).expect("matrix");

        // The Tỵ row (slot 5).
        let ty_row = matrix
            .hours
            .iter()
            .find(|h| h.chi_index == 5)
            .expect("Tỵ row must exist");
        assert_eq!(
            ty_row.branch_relation_to_birth_hour.tuong_hinh,
            crate::almanac::types::PunishmentKind::CompletedTriad {
                triad: crate::almanac::types::TriadElement::Hoa
            }
        );
        assert!(ty_row.branch_relation_to_birth_hour.has_conflict());
    }
}
