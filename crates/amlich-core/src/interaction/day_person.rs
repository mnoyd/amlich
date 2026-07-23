use crate::almanac::thap_than::get_thap_than;
use crate::almanac::types::{HeavenlyStem, RuleEvidence};
use crate::almanac::xung_hop;
use crate::bazi::types::{BaziChart, BaziPillar};
use crate::types::CanChi;

use super::types::{BranchRelation, DayPersonMatrix, ElementInteraction, PillarInteraction};

/// Compute the Day-Person Interaction Matrix.
///
/// Cross-references the day's Can Chi against each of the person's 4 Bazi pillars,
/// producing Thập Thần (stem-to-stem), Xung/Hợp (branch-to-branch), and element
/// interaction data for each available pillar.
pub fn compute_day_person_matrix(day_canchi: &CanChi, chart: &BaziChart) -> DayPersonMatrix {
    crate::build_count::day_person_matrix_built();
    let day_stem = HeavenlyStem::ALL[day_canchi.can_index];
    let day_master_stem = HeavenlyStem::ALL[chart.day_master.can_index];

    let day_to_day_master = get_thap_than(day_stem, day_master_stem);

    let pillars = chart
        .pillars
        .iter()
        .map(|pillar| compute_pillar_interaction(day_canchi, day_stem, pillar))
        .collect();

    DayPersonMatrix {
        day_canchi: day_canchi.full.clone(),
        day_master: chart.day_master.full.clone(),
        day_to_day_master,
        pillars,
        evidence: RuleEvidence {
            source_id: crate::sources::SOURCE_KHCBPPT.to_string(),
            method: "day-person-interaction-matrix".to_string(),
            profile: "baseline".to_string(),
        },
    }
}

fn compute_pillar_interaction(
    day_canchi: &CanChi,
    day_stem: HeavenlyStem,
    pillar: &BaziPillar,
) -> PillarInteraction {
    let pillar_stem = HeavenlyStem::ALL[pillar.can_chi.can_index];

    // Thập Thần: day stem → pillar stem
    let thap_than = get_thap_than(day_stem, pillar_stem);

    // Branch relations: day chi × pillar chi
    let branch_relation = compute_branch_relation(day_canchi.chi_index, pillar.can_chi.chi_index);

    // Element interaction from the Thập Thần relation
    let element_interaction = ElementInteraction::from(thap_than.relation);

    PillarInteraction {
        pillar: pillar.kind,
        pillar_canchi: pillar.can_chi.full.clone(),
        thap_than,
        branch_relation,
        element_interaction,
    }
}

/// Compute the typed branch relation between two Earthly Branches.
///
/// All four canonical relations (Lục xung, Lục hợp, Tương hại, Tương
/// hình) plus Tam hợp triad membership are reported as typed facts so
/// downstream code can distinguish direct pair, completed group, and
/// self-punishment semantically instead of as booleans promoted from
/// group membership.
///
/// Implementation note: this function previously reported
/// `tam_hop = true` and `tuong_hinh = true` for any two branches in
/// the same broad group, which the audit
/// (`docs/architecture/personal-day-audit/interaction-almanac.md:69-115`)
/// flagged as a conflation of membership with pair claims. The
/// canonical replacement is:
/// - `tam_hop_member` is set to the branch's triad element whenever
///   both branches are in the same triad (including same-branch).
/// - `tam_hop_completed` is always `false` at the pair level.
/// - `tuong_hinh` uses the typed [`PunishmentKind`] from
///   `almanac::xung_hop::xiang_xing_pair` /
///   `almanac::xung_hop::xiang_xing_self` so that incomplete triads
///   are reported as `Unavailable` rather than promoted.
pub fn compute_branch_relation(day_chi: usize, pillar_chi: usize) -> BranchRelation {
    let day_xung_hop = xung_hop::get_xung_hop(day_chi);
    let pillar_chi_name = crate::types::CHI[pillar_chi];

    let luc_xung = day_xung_hop.luc_xung == pillar_chi_name;

    let luc_hop = day_xung_hop
        .liu_he
        .as_ref()
        .is_some_and(|h| h == pillar_chi_name);

    let tam_hop_member = if xung_hop::is_triad_member(day_chi, pillar_chi) {
        // Both branches are in the same triad (including same-branch).
        // `is_triad_member` is total over 0..12, so this is the branch's
        // own triad element.
        xung_hop::triad_element(day_chi)
    } else {
        None
    };

    let tuong_hai = day_xung_hop
        .xiang_hai
        .as_ref()
        .is_some_and(|h| h == pillar_chi_name);

    let tuong_hinh = if day_chi == pillar_chi {
        xung_hop::xiang_xing_self(day_chi)
    } else {
        xung_hop::xiang_xing_pair(day_chi, pillar_chi)
    };

    BranchRelation {
        same_branch: day_chi == pillar_chi,
        luc_xung,
        luc_hop,
        tam_hop_member,
        tam_hop_completed: false,
        tuong_hai,
        tuong_hinh,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::almanac::types::ThapThanLabel;
    use crate::bazi::types::{BaziChartMetadata, BaziInput, BaziPillar, PillarKind};
    use crate::types::CanChi;

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

    #[test]
    fn matrix_has_4_pillar_rows() {
        // Giáp Tý day, arbitrary chart
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(matrix.pillars.len(), 4);
        assert_eq!(matrix.pillars[0].pillar, PillarKind::Year);
        assert_eq!(matrix.pillars[1].pillar, PillarKind::Month);
        assert_eq!(matrix.pillars[2].pillar, PillarKind::Day);
        assert_eq!(matrix.pillars[3].pillar, PillarKind::Hour);
    }

    #[test]
    fn matrix_omits_hour_row_when_hour_pillar_missing() {
        let day = CanChi::new(0, 0);
        let mut chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        chart.hour_pillar = None;
        chart.pillars.pop();

        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(matrix.pillars.len(), 3);
        assert!(matrix
            .pillars
            .iter()
            .all(|pillar| pillar.pillar != PillarKind::Hour));
    }

    #[test]
    fn day_canchi_and_day_master_are_recorded() {
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(matrix.day_canchi, "Giáp Tý");
        assert_eq!(matrix.day_master, chart.day_master.full);
    }

    #[test]
    fn thap_than_giap_to_giap_is_ty_kien() {
        // Day = Giáp, Year pillar = Giáp → same stem → Tỷ Kiên
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 2), (2, 4), (4, 6), (6, 8));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(matrix.pillars[0].thap_than.label, ThapThanLabel::TyKien);
    }

    #[test]
    fn thap_than_giap_to_canh_is_that_sat() {
        // Day = Giáp(Mộc), pillar = Canh(Kim), Kim khắc Mộc, same polarity → Thất Sát
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((6, 0), (2, 2), (4, 4), (8, 8));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(matrix.pillars[0].thap_than.label, ThapThanLabel::ThatSat);
    }

    #[test]
    fn luc_xung_ty_vs_ngo() {
        // Day chi = Tý(0), pillar chi = Ngọ(6) → lục xung
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 6), (2, 2), (4, 4), (6, 8));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert!(matrix.pillars[0].branch_relation.luc_xung);
        assert!(matrix.pillars[0].branch_relation.has_conflict());
    }

    #[test]
    fn luc_hop_ty_vs_suu() {
        // Day chi = Tý(0), pillar chi = Sửu(1) → lục hợp
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 1), (2, 2), (4, 4), (6, 8));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert!(matrix.pillars[0].branch_relation.luc_hop);
        assert!(matrix.pillars[0].branch_relation.has_harmony());
    }

    #[test]
    fn tam_hop_ty_than_thin() {
        // Day chi = Tý(0), tam hợp group = [Thân(8), Tý(0), Thìn(4)]
        let day = CanChi::new(0, 0); // Giáp Tý
        let chart = make_chart((0, 8), (2, 4), (4, 0), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        // Year pillar chi = Thân(8) → in Tý's tam hợp (Thủy triad).
        assert_eq!(
            matrix.pillars[0].branch_relation.tam_hop_member,
            Some(crate::almanac::types::TriadElement::Thuy)
        );
        assert!(matrix.pillars[0].branch_relation.is_tam_hop_pair());
        // Month pillar chi = Thìn(4) → in Tý's tam hợp (Thủy triad).
        assert_eq!(
            matrix.pillars[1].branch_relation.tam_hop_member,
            Some(crate::almanac::types::TriadElement::Thuy)
        );
        assert!(matrix.pillars[1].branch_relation.is_tam_hop_pair());
        // Day pillar chi = Tý(0) → in Tý's tam hợp (self).
        // Same-branch is `Some(Thuy)` membership but `is_tam_hop_pair`
        // returns false because the two branches are equal.
        assert_eq!(
            matrix.pillars[2].branch_relation.tam_hop_member,
            Some(crate::almanac::types::TriadElement::Thuy)
        );
        assert!(!matrix.pillars[2].branch_relation.is_tam_hop_pair());
        // Hour pillar chi = Ngọ(6) → NOT in Tý's tam hợp (Hỏa triad).
        assert_eq!(matrix.pillars[3].branch_relation.tam_hop_member, None);
        assert!(!matrix.pillars[3].branch_relation.is_tam_hop_pair());
    }

    #[test]
    fn tuong_hai_ty_vs_mui() {
        // Day chi = Tý(0), pillar chi = Mùi(7) → tương hại
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 7), (2, 2), (4, 4), (6, 8));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert!(matrix.pillars[0].branch_relation.tuong_hai);
        assert!(matrix.pillars[0].branch_relation.has_conflict());
    }

    /// Canonical Tương hình (寅巳申 Vô ân) — Dần-Tỵ-Thân mutual triad.
    /// Dần(2) × Tỵ(5) and Dần(2) × Thân(8) must be CompletedTriad(Hỏa).
    /// Mão(3) is NOT in this triad (Mão is in 子卯 and Wood triad), so
    /// Dần(2) × Mão(3) must be `None` (the audit's headline defect).
    #[test]
    fn tuong_hinh_vo_an_can_ty_than() {
        use crate::almanac::types::{PunishmentKind, TriadElement};
        let day = CanChi::new(0, 2); // Giáp Dần
                                     // Year=Mão(3), Month=Tỵ(5), Day=Thân(8), Hour=Tuất(10)
        let chart = make_chart((0, 3), (2, 5), (4, 8), (6, 10));
        let matrix = compute_day_person_matrix(&day, &chart);

        // Dần(2) × Mão(3) — NOT a punishment (audit defect, now fixed).
        assert_eq!(
            matrix.pillars[0].branch_relation.tuong_hinh,
            PunishmentKind::None
        );
        // Dần(2) × Tỵ(5) — CompletedTriad(Hỏa).
        assert_eq!(
            matrix.pillars[1].branch_relation.tuong_hinh,
            PunishmentKind::CompletedTriad {
                triad: TriadElement::Hoa
            }
        );
        // Dần(2) × Thân(8) — CompletedTriad(Hỏa).
        assert_eq!(
            matrix.pillars[2].branch_relation.tuong_hinh,
            PunishmentKind::CompletedTriad {
                triad: TriadElement::Hoa
            }
        );
        // Dần(2) × Tuất(10) — NOT a punishment. (Tuất is Earth triad.)
        assert_eq!(
            matrix.pillars[3].branch_relation.tuong_hinh,
            PunishmentKind::None
        );

        // Both completed-triad rows must register as conflict.
        assert!(matrix.pillars[1].branch_relation.has_conflict());
        assert!(matrix.pillars[2].branch_relation.has_conflict());
        // And neither must register as harmony.
        assert!(!matrix.pillars[1].branch_relation.has_harmony());
        assert!(!matrix.pillars[2].branch_relation.has_harmony());
    }

    /// Canonical Tương hình (丑未戌 Trì thế) — two-branch occurrences
    /// are canonically `Unavailable`, NOT promoted to a punishment.
    #[test]
    fn tuong_hinh_chi_thien_two_branch_unavailable() {
        use crate::almanac::types::PunishmentKind;
        let day = CanChi::new(0, 1); // Giáp Sửu
                                     // Year=Mùi(7), Month=Tuất(10), Day=Sửu(1) self, Hour=Tý(0)
        let chart = make_chart((0, 7), (2, 10), (4, 1), (6, 0));
        let matrix = compute_day_person_matrix(&day, &chart);

        // Sửu(1) × Mùi(7) — Unavailable (incomplete Trì thế triad).
        assert!(matches!(
            matrix.pillars[0].branch_relation.tuong_hinh,
            PunishmentKind::Unavailable { .. }
        ));
        // Sửu(1) × Tuất(10) — Unavailable (incomplete Trì thế triad).
        assert!(matches!(
            matrix.pillars[1].branch_relation.tuong_hinh,
            PunishmentKind::Unavailable { .. }
        ));
        // Sửu(1) × Sửu(1) — NOT a punishment (Sửu is NOT self-punishment).
        assert_eq!(
            matrix.pillars[2].branch_relation.tuong_hinh,
            PunishmentKind::None
        );
    }

    /// Canonical Tương hình (子卯 Vô lễ) — directed pair Tý → Mão.
    /// Both input orders must report the same direction.
    #[test]
    fn tuong_hinh_vo_le_directed_ty_mao() {
        use crate::almanac::types::{BranchRef, PunishmentKind};

        // Day=Tý(0), pillar=Mão(3)
        let r = compute_branch_relation(0, 3);
        assert_eq!(
            r.tuong_hinh,
            PunishmentKind::DirectedPair {
                aggressor: BranchRef::new(0),
                victim: BranchRef::new(3),
            }
        );

        // Reverse input order still reports Tý → Mão.
        let r = compute_branch_relation(3, 0);
        assert_eq!(
            r.tuong_hinh,
            PunishmentKind::DirectedPair {
                aggressor: BranchRef::new(0),
                victim: BranchRef::new(3),
            }
        );
    }

    /// Self-punishment: Tỵ(5) is NOT in self-punishment (only Thìn, Ngọ,
    /// Dậu, Hợi are). Day=Thân(8) × pillar=Thân(8) is `None`.
    #[test]
    fn tuong_hinh_self_ty_than_is_none() {
        use crate::almanac::types::PunishmentKind;
        let day = CanChi::new(0, 8); // Giáp Thân
        let chart = make_chart((0, 8), (2, 4), (4, 6), (6, 0)); // Year=Thân(8)
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(
            matrix.pillars[0].branch_relation.tuong_hinh,
            PunishmentKind::None
        );
    }

    /// Self-punishment: Thân(8) × Thân(8) is NOT self-punishment
    /// (Thân is in 寅巳申 but not in {Thìn, Ngọ, Dậu, Hợi}).
    /// Day=Tý(0) × pillar=Tý(0) is also NOT self-punishment
    /// (Tý is in 子卯 directed pair, not self).
    #[test]
    fn tuong_hinh_self_ngo_is_self_punishment() {
        use crate::almanac::types::{BranchRef, PunishmentKind};
        let day = CanChi::new(6, 6); // Bính Ngọ
        let chart = make_chart((0, 6), (2, 4), (4, 6), (6, 0)); // Year=Ngọ(6)
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(
            matrix.pillars[0].branch_relation.tuong_hinh,
            PunishmentKind::SelfPunishment {
                branch: BranchRef::new(6),
            }
        );
        // Self-punishment registers as conflict.
        assert!(matrix.pillars[0].branch_relation.has_conflict());
    }

    #[test]
    fn element_interaction_maps_from_thap_than() {
        // Giáp(Mộc) → Bính(Hỏa): Mộc sinh Hỏa → DayGeneratesPillar
        let day = CanChi::new(0, 0); // Giáp
        let chart = make_chart((2, 0), (4, 2), (6, 4), (8, 6)); // Year=Bính
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(
            matrix.pillars[0].element_interaction,
            ElementInteraction::DayGeneratesPillar
        );
    }

    #[test]
    fn neutral_branch_relation() {
        // Day chi = Tý(0), pillar chi = Dần(2) — no direct xung/hợp/hại/hình
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 2), (2, 4), (4, 6), (6, 8));
        let matrix = compute_day_person_matrix(&day, &chart);
        let br = &matrix.pillars[0].branch_relation;
        assert!(!br.luc_xung);
        assert!(!br.luc_hop);
        assert!(!br.tuong_hai);
        // Tý(0) and Dần(2) are in different triads (Tý=Thủy, Dần=Hỏa)
        // and in different punishment relations (Tý is in 子卯 directed;
        // Dần is in 寅巳申 mutual). The relation is therefore `None`.
        assert_eq!(br.tuong_hinh, crate::almanac::types::PunishmentKind::None);
        assert!(br.is_neutral());
    }

    #[test]
    fn evidence_metadata_is_set() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(matrix.evidence.source_id, "khcbppt");
        assert_eq!(matrix.evidence.method, "day-person-interaction-matrix");
    }

    #[test]
    fn matrix_serializes_to_json() {
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        let json = serde_json::to_string(&matrix).expect("should serialize");
        assert!(json.contains("\"day_canchi\""));
        assert!(json.contains("\"pillars\""));
        assert!(json.contains("\"branch_relation\""));
        assert!(json.contains("\"thap_than\""));
    }

    #[test]
    fn day_to_day_master_thap_than_is_computed() {
        // Day = Giáp(0), chart day pillar = Mậu(4) → day_master = Mậu
        // Giáp(Mộc) → Mậu(Thổ): Mộc khắc Thổ, same polarity → Thiên Tài
        let day = CanChi::new(0, 0);
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 6));
        let matrix = compute_day_person_matrix(&day, &chart);
        assert_eq!(matrix.day_to_day_master.label, ThapThanLabel::ThienTai);
    }
}
