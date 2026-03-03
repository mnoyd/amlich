use serde::{Deserialize, Serialize};

use crate::almanac::thap_than::get_thap_than;
use crate::almanac::tu_menh::{compute_kua, Direction, Gender, KuaResult};
use crate::almanac::types::{FiveElement, HeavenlyStem, Polarity, ThapThanResult};
use crate::canchi::{get_month_canchi, get_year_canchi};
use crate::julian::jd_from_date;
use crate::lunar::convert_solar_to_lunar;
use crate::tietkhi::get_days_to_nearest_tiet_khi;
use crate::types::CanChi;

const PILLAR_COUNT: usize = 8;
const PILLAR_SPAN_YEARS: f64 = 10.0;
const VIETNAM_TIME_ZONE: f64 = 7.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChieuThu {
    Thuan,
    Nghich,
}

impl ChieuThu {
    pub fn display_label(self) -> &'static str {
        match self {
            ChieuThu::Thuan => "Thuan",
            ChieuThu::Nghich => "Nghich",
        }
    }

    fn step(self) -> i32 {
        match self {
            ChieuThu::Thuan => 1,
            ChieuThu::Nghich => -1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaiVanConvention {
    pub year_basis: String,
    pub start_age_method: String,
    pub gender_encoding: String,
    pub direction_method: String,
}

impl DaiVanConvention {
    pub fn project_default() -> Self {
        Self {
            year_basis: "lunar_year_stem_from_solar_birth_date".to_string(),
            start_age_method: "nearest_tiet_khi_distance_days_div_3".to_string(),
            gender_encoding: "enum(Male,Female)".to_string(),
            direction_method: "year_polarity_x_gender".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaiVanEvidence {
    pub source_id: String,
    pub method: String,
    pub source_note: String,
}

impl DaiVanEvidence {
    pub fn project_default() -> Self {
        Self {
            source_id: "khcbppt".to_string(),
            method: "bai-quyet".to_string(),
            source_note: "TODO: verify exact KHCBPPT chapter for Dai Van formula mapping"
                .to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaiVanCanChi {
    pub can_index: usize,
    pub chi_index: usize,
    pub can: String,
    pub chi: String,
    pub full: String,
    pub con_giap: String,
}

impl From<&CanChi> for DaiVanCanChi {
    fn from(value: &CanChi) -> Self {
        Self {
            can_index: value.can_index,
            chi_index: value.chi_index,
            can: value.can.clone(),
            chi: value.chi.clone(),
            full: value.full.clone(),
            con_giap: value.con_giap.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaiVanPillar {
    pub index: usize,
    pub can_chi: DaiVanCanChi,
    pub start_age: f64,
    pub end_age: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaiVanResult {
    pub chieu_thu: ChieuThu,
    pub chieu_thu_label: String,
    pub start_age_years: f64,
    pub start_age_display: String,
    pub pillars: Vec<DaiVanPillar>,
    pub convention: DaiVanConvention,
    pub evidence: DaiVanEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaiVanKuaPillarAnalysis {
    pub index: usize,
    pub can_chi: DaiVanCanChi,
    pub pillar_element: FiveElement,
    pub favorable_directions: Vec<Direction>,
    pub unfavorable_directions: Vec<Direction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DaiVanKuaAnalysis {
    pub birth_kua: KuaResult,
    pub pillars: Vec<DaiVanKuaPillarAnalysis>,
}

pub fn determine_chieu_thu(year_stem: HeavenlyStem, gender: Gender) -> ChieuThu {
    match (year_stem.polarity(), gender) {
        (Polarity::Duong, Gender::Male) | (Polarity::Am, Gender::Female) => ChieuThu::Thuan,
        (Polarity::Duong, Gender::Female) | (Polarity::Am, Gender::Male) => ChieuThu::Nghich,
    }
}

pub fn calculate_start_age_years(days_to_nearest_tiet_khi: f64) -> f64 {
    days_to_nearest_tiet_khi.abs() / 3.0
}

pub fn generate_pillars(
    base_month_canchi: &CanChi,
    chieu_thu: ChieuThu,
    start_age: f64,
) -> Vec<DaiVanPillar> {
    let step = chieu_thu.step();
    let mut pillars = Vec::with_capacity(PILLAR_COUNT);

    for index in 0..PILLAR_COUNT {
        let offset = step * index as i32;
        let can = (base_month_canchi.can_index as i32 + offset).rem_euclid(10) as usize;
        let chi = (base_month_canchi.chi_index as i32 + offset).rem_euclid(12) as usize;
        let can_chi = CanChi::new(can, chi);
        let pillar_start = start_age + index as f64 * PILLAR_SPAN_YEARS;
        let pillar_end = pillar_start + PILLAR_SPAN_YEARS;

        pillars.push(DaiVanPillar {
            index,
            can_chi: DaiVanCanChi::from(&can_chi),
            start_age: pillar_start,
            end_age: pillar_end,
        });
    }

    pillars
}

pub fn calculate_dai_van(day: i32, month: i32, year: i32, gender: Gender) -> DaiVanResult {
    calculate_dai_van_with_timezone(day, month, year, gender, VIETNAM_TIME_ZONE)
}

pub fn calculate_dai_van_with_timezone(
    day: i32,
    month: i32,
    year: i32,
    gender: Gender,
    time_zone: f64,
) -> DaiVanResult {
    let lunar = convert_solar_to_lunar(day, month, year, time_zone);
    let year_canchi = get_year_canchi(lunar.year);
    let month_canchi = get_month_canchi(lunar.month, lunar.year, lunar.is_leap);
    let year_stem = HeavenlyStem::try_from(year_canchi.can.as_str())
        .expect("year Can string should map to HeavenlyStem");

    let chieu_thu = determine_chieu_thu(year_stem, gender);
    let signed_days = get_days_to_nearest_tiet_khi(jd_from_date(day, month, year));
    let start_age_years = calculate_start_age_years(signed_days as f64);
    let pillars = generate_pillars(&month_canchi, chieu_thu, start_age_years);

    DaiVanResult {
        chieu_thu,
        chieu_thu_label: chieu_thu.display_label().to_string(),
        start_age_years,
        start_age_display: format!("{start_age_years:.2} years"),
        pillars,
        convention: DaiVanConvention::project_default(),
        evidence: DaiVanEvidence::project_default(),
    }
}

pub fn get_pillar_at_age(result: &DaiVanResult, age: f64) -> Option<&DaiVanPillar> {
    result
        .pillars
        .iter()
        .find(|pillar| age >= pillar.start_age && age < pillar.end_age)
}

pub fn get_current_pillar(result: &DaiVanResult, age: f64) -> Option<&DaiVanPillar> {
    get_pillar_at_age(result, age)
}

pub fn years_to_next_transition(result: &DaiVanResult, age: f64) -> Option<f64> {
    let pillar = get_pillar_at_age(result, age)?;
    Some(pillar.end_age - age)
}

pub fn get_ten_god_for_pillar(
    result: &DaiVanResult,
    pillar_index: usize,
    birth_day_stem: Option<HeavenlyStem>,
) -> Option<ThapThanResult> {
    let day_stem = birth_day_stem?;
    let pillar = result.pillars.get(pillar_index)?;
    resolve_ten_god_for_pillar(day_stem, pillar)
}

pub fn get_ten_god_for_age(
    result: &DaiVanResult,
    age: f64,
    birth_day_stem: Option<HeavenlyStem>,
) -> Option<ThapThanResult> {
    let day_stem = birth_day_stem?;
    let pillar = get_pillar_at_age(result, age)?;
    resolve_ten_god_for_pillar(day_stem, pillar)
}

fn resolve_ten_god_for_pillar(
    day_stem: HeavenlyStem,
    pillar: &DaiVanPillar,
) -> Option<ThapThanResult> {
    let pillar_stem = HeavenlyStem::try_from(pillar.can_chi.can.as_str()).ok()?;
    Some(get_thap_than(day_stem, pillar_stem))
}

pub fn analyze_dai_van_with_kua(
    result: &DaiVanResult,
    birth_year: i32,
    gender: Gender,
) -> DaiVanKuaAnalysis {
    let birth_kua = compute_kua(birth_year, gender);
    analyze_dai_van_with_precomputed_kua(result, birth_kua)
}

pub fn analyze_dai_van_with_precomputed_kua(
    result: &DaiVanResult,
    birth_kua: KuaResult,
) -> DaiVanKuaAnalysis {
    let pillars = result
        .pillars
        .iter()
        .filter_map(|pillar| analyze_single_pillar_with_kua(pillar, &birth_kua))
        .collect();

    DaiVanKuaAnalysis { birth_kua, pillars }
}

pub fn get_kua_analysis_for_pillar(
    analysis: &DaiVanKuaAnalysis,
    pillar_index: usize,
) -> Option<&DaiVanKuaPillarAnalysis> {
    analysis
        .pillars
        .iter()
        .find(|pillar| pillar.index == pillar_index)
}

pub fn get_kua_analysis_for_age<'a>(
    result: &DaiVanResult,
    analysis: &'a DaiVanKuaAnalysis,
    age: f64,
) -> Option<&'a DaiVanKuaPillarAnalysis> {
    let pillar = get_pillar_at_age(result, age)?;
    get_kua_analysis_for_pillar(analysis, pillar.index)
}

fn analyze_single_pillar_with_kua(
    pillar: &DaiVanPillar,
    birth_kua: &KuaResult,
) -> Option<DaiVanKuaPillarAnalysis> {
    let pillar_stem = HeavenlyStem::try_from(pillar.can_chi.can.as_str()).ok()?;
    let pillar_element = pillar_stem.element();
    let element_directions = directions_for_element(pillar_element);

    let favorable_directions = birth_kua
        .favorable_directions
        .iter()
        .copied()
        .filter(|direction| element_directions.contains(direction))
        .collect();

    let unfavorable_directions = birth_kua
        .unfavorable_directions
        .iter()
        .copied()
        .filter(|direction| element_directions.contains(direction))
        .collect();

    Some(DaiVanKuaPillarAnalysis {
        index: pillar.index,
        can_chi: pillar.can_chi.clone(),
        pillar_element,
        favorable_directions,
        unfavorable_directions,
    })
}

fn directions_for_element(element: FiveElement) -> &'static [Direction] {
    match element {
        FiveElement::Moc => &[Direction::East, Direction::Southeast],
        FiveElement::Hoa => &[Direction::South],
        FiveElement::Tho => &[Direction::Northeast, Direction::Southwest],
        FiveElement::Kim => &[Direction::West, Direction::Northwest],
        FiveElement::Thuy => &[Direction::North],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod types_and_metadata {
        use super::*;

        #[test]
        fn result_type_contains_convention_and_evidence_metadata() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Male);

            assert_eq!(
                result.convention.year_basis,
                "lunar_year_stem_from_solar_birth_date"
            );
            assert_eq!(
                result.convention.start_age_method,
                "nearest_tiet_khi_distance_days_div_3"
            );
            assert_eq!(result.convention.gender_encoding, "enum(Male,Female)");
            assert_eq!(result.evidence.source_id, "khcbppt");
            assert_eq!(result.evidence.method, "bai-quyet");
            assert!(result.evidence.source_note.contains("TODO"));
        }

        #[test]
        fn direction_type_has_canonical_and_display_semantics() {
            assert_eq!(ChieuThu::Thuan.display_label(), "Thuan");
            assert_eq!(ChieuThu::Nghich.display_label(), "Nghich");
            assert_eq!(ChieuThu::Thuan.step(), 1);
            assert_eq!(ChieuThu::Nghich.step(), -1);
        }

        #[test]
        fn pillar_range_structure_uses_half_open_ranges_with_decimal_age() {
            let pillars = generate_pillars(&CanChi::new(2, 2), ChieuThu::Thuan, 3.5);
            assert_eq!(pillars[0].start_age, 3.5);
            assert_eq!(pillars[0].end_age, 13.5);
            assert_eq!(pillars[1].start_age, 13.5);
        }

        #[test]
        fn dai_van_result_serializes_cleanly() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Female);
            let encoded = serde_json::to_string(&result).expect("serialize");
            assert!(encoded.contains("\"convention\""));
            assert!(encoded.contains("\"evidence\""));
            assert!(encoded.contains("\"pillars\""));
        }
    }

    mod direction_and_start_age {
        use super::*;

        #[test]
        fn direction_matrix_covers_all_polarity_gender_cases() {
            assert_eq!(
                determine_chieu_thu(HeavenlyStem::Giap, Gender::Male),
                ChieuThu::Thuan
            );
            assert_eq!(
                determine_chieu_thu(HeavenlyStem::Giap, Gender::Female),
                ChieuThu::Nghich
            );
            assert_eq!(
                determine_chieu_thu(HeavenlyStem::At, Gender::Male),
                ChieuThu::Nghich
            );
            assert_eq!(
                determine_chieu_thu(HeavenlyStem::At, Gender::Female),
                ChieuThu::Thuan
            );
        }

        #[test]
        fn start_age_zero_distance_is_zero() {
            assert_eq!(calculate_start_age_years(0.0), 0.0);
        }

        #[test]
        fn start_age_keeps_decimal_precision() {
            assert!((calculate_start_age_years(4.5) - 1.5).abs() < 1e-9);
            assert!((calculate_start_age_years(-4.5) - 1.5).abs() < 1e-9);
        }
    }

    mod pillar_generation {
        use super::*;

        #[test]
        fn calculate_dai_van_returns_exactly_eight_pillars() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Male);
            assert_eq!(result.pillars.len(), 8);
        }

        #[test]
        fn generated_pillars_are_contiguous_with_half_open_boundaries() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Male);
            for i in 1..result.pillars.len() {
                assert!((result.pillars[i - 1].end_age - result.pillars[i].start_age).abs() < 1e-9);
                assert!(
                    (result.pillars[i].end_age - result.pillars[i].start_age - PILLAR_SPAN_YEARS)
                        .abs()
                        < 1e-9
                );
            }
        }

        #[test]
        fn month_canchi_is_base_of_progression() {
            let base = get_month_canchi(1, 2024, false);
            let pillars = generate_pillars(&base, ChieuThu::Thuan, 0.0);
            assert_eq!(pillars[0].can_chi.full, base.full);
            assert_eq!(pillars[1].can_chi.can_index, (base.can_index + 1) % 10);
            assert_eq!(pillars[1].can_chi.chi_index, (base.chi_index + 1) % 12);
        }
    }

    mod helper_contracts {
        use super::*;

        fn fixture_result() -> DaiVanResult {
            DaiVanResult {
                chieu_thu: ChieuThu::Thuan,
                chieu_thu_label: ChieuThu::Thuan.display_label().to_string(),
                start_age_years: 2.0,
                start_age_display: "2.00 years".to_string(),
                pillars: vec![
                    DaiVanPillar {
                        index: 0,
                        can_chi: DaiVanCanChi::from(&CanChi::new(0, 0)),
                        start_age: 2.0,
                        end_age: 12.0,
                    },
                    DaiVanPillar {
                        index: 1,
                        can_chi: DaiVanCanChi::from(&CanChi::new(1, 1)),
                        start_age: 12.0,
                        end_age: 22.0,
                    },
                    DaiVanPillar {
                        index: 2,
                        can_chi: DaiVanCanChi::from(&CanChi::new(2, 2)),
                        start_age: 22.0,
                        end_age: 32.0,
                    },
                ],
                convention: DaiVanConvention::project_default(),
                evidence: DaiVanEvidence::project_default(),
            }
        }

        #[test]
        fn get_pillar_at_age_uses_half_open_boundaries() {
            let result = fixture_result();

            let at_first_start = get_pillar_at_age(&result, 2.0).expect("pillar at first start");
            let at_first_end = get_pillar_at_age(&result, 12.0).expect("pillar at transition age");

            assert_eq!(at_first_start.index, 0);
            assert_eq!(at_first_end.index, 1);
        }

        #[test]
        fn get_pillar_at_age_returns_none_outside_supported_age_range() {
            let result = fixture_result();

            assert!(get_pillar_at_age(&result, 1.999_999).is_none());
            assert!(get_pillar_at_age(&result, 32.0).is_none());
            assert!(get_pillar_at_age(&result, 100.0).is_none());
        }

        #[test]
        fn get_current_pillar_mirrors_get_pillar_at_age_for_same_inputs() {
            let result = fixture_result();
            let sample_ages = [1.5, 2.0, 7.5, 12.0, 31.999_99, 32.0];

            for age in sample_ages {
                let from_current = get_current_pillar(&result, age).map(|pillar| pillar.index);
                let from_lookup = get_pillar_at_age(&result, age).map(|pillar| pillar.index);
                assert_eq!(from_current, from_lookup, "mismatch at age {age}");
            }
        }

        #[test]
        fn years_to_next_transition_returns_exact_end_minus_age_for_in_range_age() {
            let result = fixture_result();
            let age = 4.25;
            let remaining = years_to_next_transition(&result, age).expect("remaining years");

            assert!((remaining - (12.0 - age)).abs() < 1e-9);
        }

        #[test]
        fn years_to_next_transition_uses_incoming_pillar_at_exact_transition_age() {
            let result = fixture_result();
            let remaining = years_to_next_transition(&result, 12.0).expect("remaining years");

            assert!((remaining - 10.0).abs() < 1e-9);
        }

        #[test]
        fn years_to_next_transition_returns_none_for_out_of_range_ages() {
            let result = fixture_result();

            assert!(years_to_next_transition(&result, 1.999_999).is_none());
            assert!(years_to_next_transition(&result, 32.0).is_none());
        }
    }

    mod helpers_and_edge_cases {
        use super::*;

        #[test]
        fn helpers_return_none_outside_covered_range() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Male);
            let first = result.pillars.first().expect("first");
            let last = result.pillars.last().expect("last");

            assert!(get_current_pillar(&result, first.start_age - 0.01).is_none());
            assert!(get_pillar_at_age(&result, last.end_age).is_none());
        }

        #[test]
        fn years_to_next_transition_is_exact_in_range() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Female);
            let first = result.pillars.first().expect("first");
            let age = first.start_age + 2.25;
            let remaining = years_to_next_transition(&result, age).expect("remaining");
            assert!((remaining - 7.75).abs() < 1e-9);
        }

        #[test]
        fn zero_distance_start_age_path_is_supported() {
            let start_age = calculate_start_age_years(0.0);
            let pillars = generate_pillars(&CanChi::new(0, 0), ChieuThu::Thuan, start_age);
            assert_eq!(pillars[0].start_age, 0.0);
        }

        #[test]
        fn leap_month_input_uses_month_canchi_integration_path() {
            let mut leap_date = None;
            for month in 1..=12 {
                for day in 1..=31 {
                    let lunar = convert_solar_to_lunar(day, month, 2023, 7.0);
                    if lunar.is_leap {
                        leap_date = Some((day, month, 2023));
                        break;
                    }
                }
                if leap_date.is_some() {
                    break;
                }
            }

            let (day, month, year) = leap_date.expect("should find leap-month date in 2023");
            let lunar = convert_solar_to_lunar(day, month, year, 7.0);
            let base = get_month_canchi(lunar.month, lunar.year, lunar.is_leap);
            let result = calculate_dai_van(day, month, year, Gender::Male);
            assert!(lunar.is_leap);
            assert!(base.full.contains("(nhuận)"));
            assert_eq!(result.pillars[0].can_chi.can_index, base.can_index);
            assert_eq!(result.pillars[0].can_chi.chi_index, base.chi_index);
        }

        #[test]
        fn year_polarity_transition_examples_remain_stable() {
            let yang_year = get_year_canchi(2024);
            let yin_year = get_year_canchi(2025);
            let yang_stem = HeavenlyStem::try_from(yang_year.can.as_str()).expect("yang stem");
            let yin_stem = HeavenlyStem::try_from(yin_year.can.as_str()).expect("yin stem");

            assert_eq!(
                determine_chieu_thu(yang_stem, Gender::Male),
                ChieuThu::Thuan
            );
            assert_eq!(
                determine_chieu_thu(yang_stem, Gender::Female),
                ChieuThu::Nghich
            );
            assert_eq!(
                determine_chieu_thu(yin_stem, Gender::Male),
                ChieuThu::Nghich
            );
            assert_eq!(
                determine_chieu_thu(yin_stem, Gender::Female),
                ChieuThu::Thuan
            );
        }

        #[test]
        fn deterministic_same_input_same_output() {
            let r1 = calculate_dai_van(10, 2, 2024, Gender::Male);
            let r2 = calculate_dai_van(10, 2, 2024, Gender::Male);
            assert_eq!(r1, r2);
        }
    }

    mod ten_gods_helpers {
        use super::*;

        fn fixture_with_single_canh_pillar() -> DaiVanResult {
            DaiVanResult {
                chieu_thu: ChieuThu::Thuan,
                chieu_thu_label: "Thuan".to_string(),
                start_age_years: 0.0,
                start_age_display: "0.00 years".to_string(),
                pillars: vec![DaiVanPillar {
                    index: 0,
                    can_chi: DaiVanCanChi {
                        can_index: 6,
                        chi_index: 0,
                        can: "Canh".to_string(),
                        chi: "Ty".to_string(),
                        full: "Canh Ty".to_string(),
                        con_giap: "Ty (Rat)".to_string(),
                    },
                    start_age: 0.0,
                    end_age: 10.0,
                }],
                convention: DaiVanConvention::project_default(),
                evidence: DaiVanEvidence::project_default(),
            }
        }

        #[test]
        fn returns_ten_god_for_valid_day_stem_and_age() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Male);
            let first = result.pillars.first().expect("first pillar");
            let age = first.start_age + 0.5;
            let pillar_stem = HeavenlyStem::try_from(first.can_chi.can.as_str())
                .expect("pillar stem should parse");

            let ten_god = get_ten_god_for_age(&result, age, Some(HeavenlyStem::Giap));

            assert_eq!(
                ten_god,
                Some(crate::almanac::thap_than::get_thap_than(
                    HeavenlyStem::Giap,
                    pillar_stem
                ))
            );
        }

        #[test]
        fn returns_none_when_birth_day_stem_missing() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Female);

            assert_eq!(get_ten_god_for_pillar(&result, 0, None), None);
            assert_eq!(
                get_ten_god_for_age(&result, result.pillars[0].start_age + 0.5, None),
                None
            );
        }

        #[test]
        fn returns_none_for_out_of_range_age_or_invalid_pillar_index() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Male);
            let last = result.pillars.last().expect("last pillar");

            assert_eq!(
                get_ten_god_for_pillar(&result, result.pillars.len(), Some(HeavenlyStem::Giap)),
                None
            );
            assert_eq!(
                get_ten_god_for_age(&result, last.end_age, Some(HeavenlyStem::Giap)),
                None
            );
        }

        #[test]
        fn preserves_day_to_pillar_orientation_when_computing_ten_gods() {
            let result = fixture_with_single_canh_pillar();

            let from_helper = get_ten_god_for_pillar(&result, 0, Some(HeavenlyStem::Giap))
                .expect("expected ten gods result");
            let expected =
                crate::almanac::thap_than::get_thap_than(HeavenlyStem::Giap, HeavenlyStem::Canh);
            let reversed =
                crate::almanac::thap_than::get_thap_than(HeavenlyStem::Canh, HeavenlyStem::Giap);

            assert_eq!(from_helper, expected);
            assert_ne!(from_helper.label, reversed.label);
        }

        #[test]
        fn calculate_dai_van_result_shape_remains_unchanged_after_helper_queries() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Male);
            let snapshot = result.clone();

            let _ = get_ten_god_for_pillar(&result, 0, Some(HeavenlyStem::Giap));
            let _ = get_ten_god_for_age(
                &result,
                result.pillars[0].start_age + 0.5,
                Some(HeavenlyStem::Giap),
            );

            assert_eq!(result, snapshot);

            let encoded = serde_json::to_string(&result).expect("serialize dai van result");
            assert!(!encoded.contains("ten_god"));
            assert!(!encoded.contains("thap_than"));
        }

        #[test]
        fn repeated_helper_calls_return_stable_equal_outputs() {
            let result = calculate_dai_van(10, 2, 2024, Gender::Female);
            let age = result.pillars[0].start_age + 1.0;

            let first = get_ten_god_for_age(&result, age, Some(HeavenlyStem::At));
            let second = get_ten_god_for_age(&result, age, Some(HeavenlyStem::At));

            assert_eq!(first, second);
        }
    }

    mod kua_helpers {
        use super::*;

        fn fixture_result_with_distinct_elements() -> DaiVanResult {
            DaiVanResult {
                chieu_thu: ChieuThu::Thuan,
                chieu_thu_label: "Thuan".to_string(),
                start_age_years: 0.0,
                start_age_display: "0.00 years".to_string(),
                pillars: vec![
                    DaiVanPillar {
                        index: 0,
                        can_chi: DaiVanCanChi {
                            can_index: 0,
                            chi_index: 0,
                            can: "Giap".to_string(),
                            chi: "Ty".to_string(),
                            full: "Giap Ty".to_string(),
                            con_giap: "Ty (Rat)".to_string(),
                        },
                        start_age: 0.0,
                        end_age: 10.0,
                    },
                    DaiVanPillar {
                        index: 1,
                        can_chi: DaiVanCanChi {
                            can_index: 6,
                            chi_index: 1,
                            can: "Canh".to_string(),
                            chi: "Suu".to_string(),
                            full: "Canh Suu".to_string(),
                            con_giap: "Suu (Ox)".to_string(),
                        },
                        start_age: 10.0,
                        end_age: 20.0,
                    },
                    DaiVanPillar {
                        index: 2,
                        can_chi: DaiVanCanChi {
                            can_index: 4,
                            chi_index: 2,
                            can: "Mau".to_string(),
                            chi: "Dan".to_string(),
                            full: "Mau Dan".to_string(),
                            con_giap: "Dan (Tiger)".to_string(),
                        },
                        start_age: 20.0,
                        end_age: 30.0,
                    },
                ],
                convention: DaiVanConvention::project_default(),
                evidence: DaiVanEvidence::project_default(),
            }
        }

        #[test]
        fn computes_birth_kua_once_and_reuses_for_all_pillars_in_analysis() {
            let result = fixture_result_with_distinct_elements();
            let analysis = analyze_dai_van_with_kua(&result, 2002, Gender::Male);

            assert_eq!(analysis.birth_kua.kua, 8);
            assert_eq!(
                analysis.birth_kua.convention.kua5_resolution,
                "male->8,female->2"
            );
            assert_eq!(analysis.pillars.len(), result.pillars.len());

            for pillar in &analysis.pillars {
                let from_index = get_kua_analysis_for_pillar(&analysis, pillar.index)
                    .expect("pillar analysis by index");
                assert_eq!(from_index, pillar);
            }
        }

        #[test]
        fn supports_kua_5_resolution_for_female_path() {
            let result = fixture_result_with_distinct_elements();
            let analysis = analyze_dai_van_with_kua(&result, 1998, Gender::Female);

            assert_eq!(analysis.birth_kua.kua, 2);
            assert_eq!(
                analysis.birth_kua.convention.kua5_resolution,
                "male->8,female->2"
            );
        }

        #[test]
        fn analyzes_pillar_element_against_kua_directions() {
            let result = fixture_result_with_distinct_elements();
            let analysis = analyze_dai_van_with_kua(&result, 1990, Gender::Female);

            let moc_pillar = get_kua_analysis_for_pillar(&analysis, 0).expect("moc pillar");
            assert_eq!(moc_pillar.pillar_element, FiveElement::Moc);
            assert!(moc_pillar.favorable_directions.is_empty());
            assert_eq!(
                moc_pillar.unfavorable_directions,
                vec![Direction::East, Direction::Southeast]
            );

            let kim_pillar = get_kua_analysis_for_pillar(&analysis, 1).expect("kim pillar");
            assert_eq!(kim_pillar.pillar_element, FiveElement::Kim);
            assert_eq!(
                kim_pillar.favorable_directions,
                vec![Direction::Northwest, Direction::West]
            );
            assert!(kim_pillar.unfavorable_directions.is_empty());

            let tho_pillar = get_kua_analysis_for_pillar(&analysis, 2).expect("tho pillar");
            assert_eq!(tho_pillar.pillar_element, FiveElement::Tho);
            assert_eq!(
                tho_pillar.favorable_directions,
                vec![Direction::Southwest, Direction::Northeast]
            );
            assert!(tho_pillar.unfavorable_directions.is_empty());
        }

        #[test]
        fn can_lookup_kua_analysis_by_age_using_existing_pillar_boundaries() {
            let result = fixture_result_with_distinct_elements();
            let analysis = analyze_dai_van_with_kua(&result, 1990, Gender::Female);

            let first = get_kua_analysis_for_age(&result, &analysis, 0.0).expect("first age");
            let second =
                get_kua_analysis_for_age(&result, &analysis, 10.0).expect("transition age");

            assert_eq!(first.index, 0);
            assert_eq!(second.index, 1);
            assert!(get_kua_analysis_for_age(&result, &analysis, 30.0).is_none());
        }
    }
}
