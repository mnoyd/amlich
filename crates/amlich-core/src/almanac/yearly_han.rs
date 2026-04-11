/// Yearly Hạn — Composite annual affliction assessment
///
/// "Hạn" is a Vietnamese umbrella term for annual afflictions. This module
/// aggregates 5 independent checks: Cửu Diệu (sao hạn), Tam Tai, Kim Lâu,
/// Hoàng Ốc, and Thái Tuế.
///
/// **Source:** Composite — each component retains its own source_id
/// **Decision:** DEC-0021

use serde::{Deserialize, Serialize};

use super::cuu_dieu::compute_cuu_dieu;
use super::hoang_oc::{compute_hoang_oc, HoangOcResult};
use super::kim_lau::{compute_kim_lau, KimLauResult};
use super::tam_tai::{compute_tam_tai, TamTaiResult};
use super::thai_tue::{compute_thai_tue, ThaiTueResult};
use super::tu_menh::Gender;
use super::types::RuleEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HanSeverity {
    /// No active affliction
    Low,
    /// 1 active affliction
    Medium,
    /// 2 active afflictions (hạn chồng hạn)
    High,
    /// 3+ active afflictions
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct YearlyHanInput {
    pub birth_lunar_year: i32,
    pub current_lunar_year: i32,
    pub gender: Gender,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct YearlyHanAssessment {
    pub sao_han: CuuDieuResult,
    pub tam_tai: TamTaiResult,
    pub kim_lau: KimLauResult,
    pub hoang_oc: HoangOcResult,
    pub thai_tue: ThaiTueResult,
    /// Number of currently active hạn checks
    pub han_count: u8,
    /// True when han_count >= 2 (hạn chồng hạn)
    pub is_chong_han: bool,
    pub severity: HanSeverity,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<RuleEvidence>,
}

/// Compute the full yearly hạn assessment.
///
/// # Arguments
/// * `input` — Birth and current year data with gender
/// * `birth_chi_index` — 0-based Earthly Branch index of the birth year
/// * `current_year_chi_index` — 0-based Earthly Branch index of the current year
pub fn compute_yearly_han(
    input: &YearlyHanInput,
    birth_chi_index: usize,
    current_year_chi_index: usize,
) -> YearlyHanAssessment {
    let sao_han = compute_cuu_dieu(
        input.birth_lunar_year,
        input.current_lunar_year,
        input.gender,
    );
    let tam_tai = compute_tam_tai(birth_chi_index, current_year_chi_index);
    let kim_lau = compute_kim_lau(input.birth_lunar_year, input.current_lunar_year);
    let hoang_oc = compute_hoang_oc(input.birth_lunar_year, input.current_lunar_year);
    let thai_tue = compute_thai_tue(birth_chi_index, current_year_chi_index);

    let mut han_count: u8 = 0;
    if sao_han.is_han {
        han_count += 1;
    }
    if tam_tai.in_tam_tai {
        han_count += 1;
    }
    if kim_lau.in_kim_lau {
        han_count += 1;
    }
    if !hoang_oc.is_good {
        han_count += 1;
    }
    if thai_tue.has_conflict {
        han_count += 1;
    }

    let severity = match han_count {
        0 => HanSeverity::Low,
        1 => HanSeverity::Medium,
        2 => HanSeverity::High,
        _ => HanSeverity::Critical,
    };

    YearlyHanAssessment {
        sao_han,
        tam_tai,
        kim_lau,
        hoang_oc,
        thai_tue,
        han_count,
        is_chong_han: han_count >= 2,
        severity,
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_input(birth: i32, current: i32, gender: Gender) -> YearlyHanInput {
        YearlyHanInput {
            birth_lunar_year: birth,
            current_lunar_year: current,
            gender,
        }
    }

    #[test]
    fn assessment_populates_all_components() {
        let input = make_input(1990, 2026, Gender::Male);
        // 1990 = Ngọ (6), 2026 = Ngọ (6)
        let birth_chi = (1990 - 4) as usize % 12; // 1986%12=6 → Ngọ
        let year_chi = (2026 - 4) as usize % 12;  // 2022%12=6 → Ngọ
        let r = compute_yearly_han(&input, birth_chi, year_chi);

        // All components should be populated
        assert!(!r.sao_han.star_name.is_empty());
        assert!(!r.tam_tai.tam_hop_group.is_empty());
        assert!(r.kim_lau.tuoi_mu > 0);
        assert!(r.hoang_oc.tuoi_mu > 0);
        // Thai tue: same chi → has Truc conflict
        assert!(r.thai_tue.has_conflict);
    }

    #[test]
    fn severity_scales_with_han_count() {
        // Use a birth year where we can predict multiple hạn
        let input = make_input(1990, 2026, Gender::Male);
        let birth_chi = (1990 - 4) as usize % 12;
        let year_chi = (2026 - 4) as usize % 12;
        let r = compute_yearly_han(&input, birth_chi, year_chi);

        // Severity should match han_count
        match r.han_count {
            0 => assert_eq!(r.severity, HanSeverity::Low),
            1 => assert_eq!(r.severity, HanSeverity::Medium),
            2 => assert_eq!(r.severity, HanSeverity::High),
            _ => assert_eq!(r.severity, HanSeverity::Critical),
        }
    }

    #[test]
    fn chong_han_at_two_or_more() {
        let input = make_input(1990, 2026, Gender::Male);
        let birth_chi = (1990 - 4) as usize % 12;
        let year_chi = (2026 - 4) as usize % 12;
        let r = compute_yearly_han(&input, birth_chi, year_chi);
        assert_eq!(r.is_chong_han, r.han_count >= 2);
    }

    #[test]
    fn evidence_defaults_to_none() {
        let input = make_input(2000, 2026, Gender::Female);
        let r = compute_yearly_han(&input, 4, 10);
        assert!(r.evidence.is_none());
    }

    #[test]
    fn gender_affects_cuu_dieu() {
        let input_m = make_input(2000, 2026, Gender::Male);
        let input_f = make_input(2000, 2026, Gender::Female);
        let r_m = compute_yearly_han(&input_m, 4, 10);
        let r_f = compute_yearly_han(&input_f, 4, 10);
        // Male and female should get different stars for same age
        assert_ne!(r_m.sao_han.star_name, r_f.sao_han.star_name);
    }
}
