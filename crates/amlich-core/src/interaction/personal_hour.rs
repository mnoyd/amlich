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
    let day_stem = HeavenlyStem::ALL[day_canchi.can_index];
    let day_master_stem = HeavenlyStem::ALL[chart.day_master.can_index];
    let birth_hour_chi_index = chart.hour_pillar.as_ref()?.can_chi.chi_index;
    let weak = weakest_element(element_dist);

    let hoang_dao = get_gio_hoang_dao(day_canchi.chi_index);

    let hours: Vec<PersonalHourEntry> = (0usize..12)
        .map(|slot| {
            let hour_pillar = compute_hour_pillar(day_stem, (slot * 2 + 1) as u8, 0)
                .expect("slot 0-11 always resolves");
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

    // Branch harmony/conflict
    if branch_rel.luc_hop {
        score += 15;
    }
    if branch_rel.tam_hop {
        score += 10;
    }
    if branch_rel.luc_xung {
        score -= 15;
    }
    if branch_rel.tuong_hai {
        score -= 10;
    }
    if branch_rel.tuong_hinh {
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
}
