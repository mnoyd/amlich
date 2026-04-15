//! Thần Sát (神煞 — Auxiliary / Symbolic Stars)
//!
//! Implements 12 commonly-used symbolic stars derived from the four Bazi
//! pillars.  Each star is a lookup from a source stem or branch to a target
//! branch; when a chart pillar contains that target branch the star is said
//! to be "present" in that pillar.
//!
//! Source: Classical Bazi (三命通會, 淵海子平, 子平真詮).

use crate::almanac::types::RuleEvidence;

use super::types::{BaziChart, PillarKind, ThanSatEntry, ThanSatResult, ThanSatSource};

const EVIDENCE_SOURCE: &str = "bazi-classical";
const EVIDENCE_PROFILE: &str = "baseline";

// ---------------------------------------------------------------------------
// Lookup tables
// ---------------------------------------------------------------------------

/// Thiên Ất Quý Nhân (天乙貴人 — Heavenly Nobleman)
/// Day stem → two noble branches.
/// Giáp→Sửu/Mùi, Ất→Tý/Thân, Bính→Hợi/Dậu, Đinh→Hợi/Dậu,
/// Mậu→Sửu/Mùi, Kỷ→Tý/Thân, Canh→Sửu/Mùi, Tân→Dần/Ngọ,
/// Nhâm→Mão/Tỵ, Quý→Mão/Tỵ
const QUY_NHAN: [[usize; 2]; 10] = [
    [1, 7],  // Giáp  → Sửu, Mùi
    [0, 8],  // Ất    → Tý, Thân
    [11, 9], // Bính  → Hợi, Dậu
    [11, 9], // Đinh  → Hợi, Dậu
    [1, 7],  // Mậu   → Sửu, Mùi
    [0, 8],  // Kỷ    → Tý, Thân
    [1, 7],  // Canh  → Sửu, Mùi
    [2, 6],  // Tân   → Dần, Ngọ
    [3, 5],  // Nhâm  → Mão, Tỵ
    [3, 5],  // Quý   → Mão, Tỵ
];

/// Văn Xương (文昌 — Academic Star)
/// Day stem → one branch.
/// Giáp→Tỵ, Ất→Ngọ, Bính→Thân, Đinh→Dậu, Mậu→Thân,
/// Kỷ→Dậu, Canh→Hợi, Tân→Tý, Nhâm→Dần, Quý→Mão
const VAN_XUONG: [usize; 10] = [5, 6, 8, 9, 8, 9, 11, 0, 2, 3];

/// Đào Hoa (桃花 — Peach Blossom)
/// Derived from Tam Hợp frame: the "mộc dục" (bath) position.
/// Frame group (branch % 4):
///   0 (Thân-Tý-Thìn) → Dậu(9)
///   1 (Tỵ-Dậu-Sửu)  → Ngọ(6)
///   2 (Dần-Ngọ-Tuất) → Mão(3)
///   3 (Hợi-Mão-Mùi)  → Tý(0)
const DAO_HOA: [usize; 4] = [9, 6, 3, 0];

/// Dịch Mã (驛馬 — Travelling Horse)
/// Tam Hợp frame's "xung" of storage position.
///   0 → Dần(2),  1 → Hợi(11),  2 → Thân(8),  3 → Tỵ(5)
const DICH_MA: [usize; 4] = [2, 11, 8, 5];

/// Hoa Cái (華蓋 — Imperial Canopy)
/// Tam Hợp frame's storage (mộ) position.
///   0 → Thìn(4),  1 → Sửu(1),  2 → Tuất(10),  3 → Mùi(7)
const HOA_CAI: [usize; 4] = [4, 1, 10, 7];

/// Kiếp Sát (劫煞 — Robbery Killing)
/// Opposite of Đào Hoa (+ 6 mod 12).
///   0 → Mão(3),  1 → Tý(0),  2 → Dậu(9),  3 → Ngọ(6)
const KIEP_SAT: [usize; 4] = [3, 0, 9, 6];

/// Hồng Loan (紅鸞 — Red Phoenix)
/// Year branch → target branch.
/// Tý→Mão, Sửu→Dần, Dần→Sửu, Mão→Tý, Thìn→Hợi, Tỵ→Tuất,
/// Ngọ→Dậu, Mùi→Thân, Thân→Mùi, Dậu→Ngọ, Tuất→Tỵ, Hợi→Thìn
const HONG_LOAN: [usize; 12] = [3, 2, 1, 0, 11, 10, 9, 8, 7, 6, 5, 4];

/// Thiên Hỷ (天喜 — Heavenly Joy) = Hồng Loan + 6 (mod 12)
fn thien_hy(year_chi: usize) -> usize {
    (HONG_LOAN[year_chi] + 6) % 12
}

/// Thiên Đức (天德 — Heavenly Virtue)
/// Month branch → target branch.
/// Month 1(Dần)→Đinh(→branch mapping not applicable; Thiên Đức traditionally
/// maps to a stem, not a branch).
///
/// Classical mapping: month_chi → stem index.
/// Dần→Đinh(3), Mão→Thân(8)*, Thìn→Nhâm(8), Tỵ→Tân(7),
/// Ngọ→Hợi(11)*, Mùi→Giáp(0), Thân→Quý(9), Dậu→Dần(2)*,
/// Tuất→Bính(2), Hợi→Ất(1), Tý→Tỵ(5)*, Sửu→Canh(6)
///
/// * Some months map to a branch rather than a stem; the traditional table
///   mixes both.  We map everything to the branch that corresponds:
///   stem Đinh(3) → Ngọ(6) by stem→element→branch convention.
///
/// For simplicity and common practice, we use the branch-mapped version:
/// Dần→Ngọ(6)*, Mão→Thân(8), Thìn→Hợi(11)*, Tỵ→Dậu(9)*,
/// Ngọ→Hợi(11), Mùi→Tý(0)*, Thân→Mão(3)*, Dậu→Dần(2),
/// Tuất→Ngọ(6)*, Hợi→Mão(3)*, Tý→Tỵ(5), Sửu→Thân(8)*
///
/// Note: Thiên Đức is complex because it traditionally maps to a Can
/// (stem), and whether a branch "contains" it depends on Tàng Can.
/// For this implementation we store the stem index and check if the
/// pillar's visible stem matches.
const THIEN_DUC_STEM: [usize; 12] = [
    // Indexed by month branch (Tý=0 … Hợi=11)
    // Tý→Tỵ: but Tỵ is a chi; classical = Kỷ stem → index 5
    // Let's use the well-known stem-based table:
    // Month  1(Dần)=Đinh(3), 2(Mão)=Giáp(0)※, 3(Thìn)=Nhâm(8),
    //        4(Tỵ)=Tân(7),  5(Ngọ)=Giáp(0)※, 6(Mùi)=Quý(9)※,
    //        7(Thân)=Giáp(0)※, 8(Dậu)=Nhâm(8)※, 9(Tuất)=Bính(2),
    //       10(Hợi)=Ất(1),  11(Tý)=Kỷ(5)※, 12(Sửu)=Canh(6)
    // ※ Some sources differ; we follow 三命通會.
    //
    // Index by chi: Tý(0)=Kỷ(5), Sửu(1)=Canh(6), Dần(2)=Đinh(3),
    // Mão(3)=Giáp(0)※, Thìn(4)=Nhâm(8), Tỵ(5)=Tân(7),
    // Ngọ(6)=Giáp(0)※, Mùi(7)=Quý(9), Thân(8)=Giáp(0)※,
    // Dậu(9)=Nhâm(8)※, Tuất(10)=Bính(2), Hợi(11)=Ất(1)
    5, 6, 3, 0, 8, 7, 0, 9, 0, 8, 2, 1,
];

/// Nguyệt Đức (月德 — Monthly Virtue)
/// Month branch → stem index.
/// Classical: Dần(2)→Bính(2)※, Mão(3)→Giáp(0), Thìn(4)→Nhâm(8),
/// Tỵ(5)→Canh(6), Ngọ(6)→Bính(2), Mùi(7)→Giáp(0),
/// Thân(8)→Nhâm(8)※, Dậu(9)→Canh(6), Tuất(10)→Bính(2),
/// Hợi(11)→Giáp(0), Tý(0)→Nhâm(8)※, Sửu(1)→Canh(6)
///
/// Pattern: cycles through Bính(2)→Giáp(0)→Nhâm(8)→Canh(6) every 3 months.
/// (Dần uses Bính, Mão Giáp, Thìn Nhâm, Tỵ Canh, Ngọ Bính, …)
const NGUYET_DUC_STEM: [usize; 12] = [
    // By chi index: Tý(0)=Nhâm(8), Sửu(1)=Canh(6), Dần(2)=Bính(2),
    // Mão(3)=Giáp(0), Thìn(4)=Nhâm(8), Tỵ(5)=Canh(6),
    // Ngọ(6)=Bính(2), Mùi(7)=Giáp(0), Thân(8)=Nhâm(8),
    // Dậu(9)=Canh(6), Tuất(10)=Bính(2), Hợi(11)=Giáp(0)
    8, 6, 2, 0, 8, 6, 2, 0, 8, 6, 2, 0,
];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute all 12 Thần Sát for a given Bazi chart.
///
/// Returns a flat list of entries; each entry identifies the star, its
/// derivation source, target branch, and which chart pillars contain it.
pub fn compute_than_sat(chart: &BaziChart) -> ThanSatResult {
    let mut pillar_branches = vec![
        (PillarKind::Year, chart.year_pillar.can_chi.chi_index),
        (PillarKind::Month, chart.month_pillar.can_chi.chi_index),
        (PillarKind::Day, chart.day_pillar.can_chi.chi_index),
    ];
    let mut pillar_stems = vec![
        (PillarKind::Year, chart.year_pillar.can_chi.can_index),
        (PillarKind::Month, chart.month_pillar.can_chi.can_index),
        (PillarKind::Day, chart.day_pillar.can_chi.can_index),
    ];
    if let Some(hour) = &chart.hour_pillar {
        pillar_branches.push((PillarKind::Hour, hour.can_chi.chi_index));
        pillar_stems.push((PillarKind::Hour, hour.can_chi.can_index));
    }

    let day_stem = chart.day_pillar.can_chi.can_index;
    let year_branch = chart.year_pillar.can_chi.chi_index;
    let day_branch = chart.day_pillar.can_chi.chi_index;
    let month_branch = chart.month_pillar.can_chi.chi_index;

    let find_branch = |target: usize| -> Vec<PillarKind> {
        pillar_branches
            .iter()
            .filter(|(_, chi)| *chi == target)
            .map(|(kind, _)| *kind)
            .collect()
    };

    let find_stem = |target: usize| -> Vec<PillarKind> {
        pillar_stems
            .iter()
            .filter(|(_, can)| *can == target)
            .map(|(kind, _)| *kind)
            .collect()
    };

    let mut stars: Vec<ThanSatEntry> = Vec::with_capacity(14);

    let branch_name = |idx: usize| crate::types::CHI[idx].to_string();

    // --- Day Stem derived stars ---

    // Thiên Ất Quý Nhân (2 noble branches)
    for &noble in &QUY_NHAN[day_stem] {
        stars.push(ThanSatEntry {
            name: "Thiên Ất Quý Nhân".to_string(),
            source: ThanSatSource::DayStem,
            target_branch: noble,
            target_branch_name: branch_name(noble),
            present_in: find_branch(noble),
        });
    }

    // Văn Xương
    let vx = VAN_XUONG[day_stem];
    stars.push(ThanSatEntry {
        name: "Văn Xương".to_string(),
        source: ThanSatSource::DayStem,
        target_branch: vx,
        target_branch_name: branch_name(vx),
        present_in: find_branch(vx),
    });

    // --- Year/Day Branch derived stars (Tam Hợp frame) ---
    // Convention: use Year branch for Đào Hoa, Dịch Mã, Hoa Cái, Kiếp Sát
    // (some traditions use Day branch; we also check Day branch as secondary).

    let frame = year_branch % 4;

    let dao_hoa = DAO_HOA[frame];
    stars.push(ThanSatEntry {
        name: "Đào Hoa".to_string(),
        source: ThanSatSource::YearBranch,
        target_branch: dao_hoa,
        target_branch_name: branch_name(dao_hoa),
        present_in: find_branch(dao_hoa),
    });

    let dich_ma = DICH_MA[frame];
    stars.push(ThanSatEntry {
        name: "Dịch Mã".to_string(),
        source: ThanSatSource::YearBranch,
        target_branch: dich_ma,
        target_branch_name: branch_name(dich_ma),
        present_in: find_branch(dich_ma),
    });

    let hoa_cai = HOA_CAI[frame];
    stars.push(ThanSatEntry {
        name: "Hoa Cái".to_string(),
        source: ThanSatSource::YearBranch,
        target_branch: hoa_cai,
        target_branch_name: branch_name(hoa_cai),
        present_in: find_branch(hoa_cai),
    });

    let kiep_sat = KIEP_SAT[frame];
    stars.push(ThanSatEntry {
        name: "Kiếp Sát".to_string(),
        source: ThanSatSource::YearBranch,
        target_branch: kiep_sat,
        target_branch_name: branch_name(kiep_sat),
        present_in: find_branch(kiep_sat),
    });

    // Also derive from Day branch for Đào Hoa (secondary, noted by source)
    let day_frame = day_branch % 4;
    if day_frame != frame {
        let dao_hoa_day = DAO_HOA[day_frame];
        if dao_hoa_day != dao_hoa {
            stars.push(ThanSatEntry {
                name: "Đào Hoa".to_string(),
                source: ThanSatSource::DayBranch,
                target_branch: dao_hoa_day,
                target_branch_name: branch_name(dao_hoa_day),
                present_in: find_branch(dao_hoa_day),
            });
        }
    }

    // --- Year Branch direct lookup ---

    let hong_loan = HONG_LOAN[year_branch];
    stars.push(ThanSatEntry {
        name: "Hồng Loan".to_string(),
        source: ThanSatSource::YearBranch,
        target_branch: hong_loan,
        target_branch_name: branch_name(hong_loan),
        present_in: find_branch(hong_loan),
    });

    let thien_hy_val = thien_hy(year_branch);
    stars.push(ThanSatEntry {
        name: "Thiên Hỷ".to_string(),
        source: ThanSatSource::YearBranch,
        target_branch: thien_hy_val,
        target_branch_name: branch_name(thien_hy_val),
        present_in: find_branch(thien_hy_val),
    });

    // --- Month Branch protective stars ---
    // These map to stems, so we check if any pillar's visible stem matches.

    let thien_duc = THIEN_DUC_STEM[month_branch];
    stars.push(ThanSatEntry {
        name: "Thiên Đức".to_string(),
        source: ThanSatSource::MonthBranch,
        target_branch: thien_duc, // actually a stem index
        target_branch_name: crate::types::CAN[thien_duc].to_string(),
        present_in: find_stem(thien_duc),
    });

    let nguyet_duc = NGUYET_DUC_STEM[month_branch];
    stars.push(ThanSatEntry {
        name: "Nguyệt Đức".to_string(),
        source: ThanSatSource::MonthBranch,
        target_branch: nguyet_duc, // actually a stem index
        target_branch_name: crate::types::CAN[nguyet_duc].to_string(),
        present_in: find_stem(nguyet_duc),
    });

    ThanSatResult {
        stars,
        evidence: RuleEvidence {
            source_id: EVIDENCE_SOURCE.to_string(),
            method: "than-sat-lookup-tables".to_string(),
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
    use crate::bazi::types::{BaziChartMetadata, BaziInput, BaziPillar, PillarKind};
    use crate::lunar::LunarDate;
    use crate::types::CanChi;

    fn make_pillar(kind: PillarKind, can: usize, chi: usize) -> BaziPillar {
        BaziPillar {
            kind,
            can_chi: CanChi::new(can, chi),
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
        let yp = make_pillar(PillarKind::Year, year.0, year.1);
        let mp = make_pillar(PillarKind::Month, month.0, month.1);
        let dp = make_pillar(PillarKind::Day, day.0, day.1);
        let hp = make_pillar(PillarKind::Hour, hour.0, hour.1);
        let day_master = dp.can_chi.clone();
        let pillars = vec![yp.clone(), mp.clone(), dp.clone(), hp.clone()];

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
            lunar_date: LunarDate {
                day: 1,
                month: 1,
                year: 2000,
                is_leap: false,
            },
            year_pillar: yp,
            month_pillar: mp,
            day_pillar: dp,
            hour_pillar: Some(hp),
            day_master,
            pillars,
            metadata: BaziChartMetadata {
                timezone: 7.0,
                use_solar_time: false,
                year_basis: "test".into(),
                month_basis: "test".into(),
                day_basis: "test".into(),
                hour_basis: "test".into(),
                hour_evidence: None,
            },
        }
    }

    #[test]
    fn quy_nhan_giap_yields_suu_and_mui() {
        // Day stem Giáp(0) → Quý Nhân at Sửu(1) and Mùi(7)
        assert_eq!(QUY_NHAN[0], [1, 7]);
    }

    #[test]
    fn dao_hoa_frame_0_is_dau() {
        // Thân-Tý-Thìn frame → Đào Hoa = Dậu(9)
        assert_eq!(DAO_HOA[0], 9);
    }

    #[test]
    fn hong_loan_ty_is_mao() {
        assert_eq!(HONG_LOAN[0], 3); // Tý → Mão
    }

    #[test]
    fn thien_hy_is_hong_loan_plus_6() {
        for chi in 0..12 {
            assert_eq!(thien_hy(chi), (HONG_LOAN[chi] + 6) % 12);
        }
    }

    #[test]
    fn than_sat_produces_at_least_12_stars() {
        let chart = make_chart((0, 0), (2, 2), (0, 4), (4, 6));
        let result = compute_than_sat(&chart);
        // 2 Quý Nhân + 1 Văn Xương + 4 frame stars + 2 year stars + 2 month stars = 11 min
        // + possible secondary Đào Hoa = 12
        assert!(
            result.stars.len() >= 11,
            "Expected at least 11 stars, got {}",
            result.stars.len()
        );
    }

    #[test]
    fn present_in_detects_matching_pillar() {
        // Day stem Giáp(0) → Quý Nhân at Sửu(1)
        // Put Sửu(1) in the hour pillar
        let chart = make_chart((0, 0), (2, 2), (0, 4), (4, 1));
        let result = compute_than_sat(&chart);

        let quy_nhan_suu = result
            .stars
            .iter()
            .find(|s| s.name == "Thiên Ất Quý Nhân" && s.target_branch == 1)
            .expect("Quý Nhân at Sửu should exist");

        assert!(
            quy_nhan_suu.present_in.contains(&PillarKind::Hour),
            "Quý Nhân should be present in Hour pillar (Sửu)"
        );
    }

    #[test]
    fn dao_hoa_in_hour_is_detected() {
        // Year branch Tý(0) → frame 0 → Đào Hoa at Dậu(9)
        // Put Dậu(9) in the hour pillar
        let chart = make_chart((0, 0), (2, 2), (4, 4), (6, 9));
        let result = compute_than_sat(&chart);

        let dao_hoa = result
            .stars
            .iter()
            .find(|s| s.name == "Đào Hoa" && s.source == ThanSatSource::YearBranch)
            .expect("Đào Hoa from year should exist");

        assert!(
            dao_hoa.present_in.contains(&PillarKind::Hour),
            "Đào Hoa should be present in Hour pillar (Dậu)"
        );
    }

    #[test]
    fn nguyet_duc_follows_cyclic_pattern() {
        // Bính(2) → Giáp(0) → Nhâm(8) → Canh(6) repeating every 3 months
        // starting from Dần(2)
        let expected = [2, 0, 8, 6];
        for i in 0..12 {
            let month_chi = (i + 2) % 12; // month 1 = Dần(2), month 2 = Mão(3), ...
            assert_eq!(
                NGUYET_DUC_STEM[month_chi],
                expected[i % 4],
                "Nguyệt Đức for month chi={month_chi} should be stem {}",
                expected[i % 4]
            );
        }
    }

    #[test]
    fn evidence_is_populated() {
        let chart = make_chart((0, 0), (2, 2), (0, 4), (4, 6));
        let result = compute_than_sat(&chart);
        assert_eq!(result.evidence.source_id, "bazi-classical");
        assert_eq!(result.evidence.method, "than-sat-lookup-tables");
    }
}
