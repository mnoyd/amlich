use crate::types::{CanChi, CHI, CON_GIAP};

use super::data::baseline_data;
use super::star::resolve_rules;
use super::star::{StarCategory, StarRule};
use super::than_huong::get_than_huong;
use super::than_sat::get_day_star_rules;
use super::truc::get_truc;
use super::types::{
    DayConflict, DayElement, DayFortune, DayStar, DayStars, RuleEvidence, StarQuality,
    StarRuleEvidence, StarSystem,
};
use super::xung_hop::get_xung_hop;

pub fn calculate_day_fortune(
    jd: i32,
    day_canchi: &CanChi,
    lunar_month: i32,
    year_can: &str,
    tiet_khi_name: &str,
) -> DayFortune {
    let data = baseline_data();
    let conflict_rule = data
        .conflict_by_chi
        .get(&day_canchi.chi)
        .expect("conflict rule by chi should exist");
    let na_am = data
        .sexagenary_na_am
        .get(&day_canchi.full)
        .expect("na am entry should exist");

    let opposing_idx = CHI
        .iter()
        .position(|chi| *chi == conflict_rule.opposing_chi)
        .expect("opposing chi should exist");

    let day_chi_idx = CHI
        .iter()
        .position(|chi| *chi == day_canchi.chi)
        .expect("day chi should exist");

    let day_star_index = jd.rem_euclid(28) as usize;
    let day_star_rule = &data.nhi_thap_bat_tu[day_star_index];

    let star_rules = get_day_star_rules(
        &day_canchi.chi,
        &day_canchi.full,
        year_can,
        lunar_month,
        tiet_khi_name,
    );
    let (cat_tinh, sat_tinh) = resolve_rules(&star_rules);
    let profile = data.profile.clone();
    let matched_rules = build_star_rule_evidence(&star_rules, &profile, data);

    let mut travel = get_than_huong(&day_canchi.can);
    travel.evidence = Some(rule_evidence(&data.travel_meta, &profile));

    DayFortune {
        profile: profile.clone(),
        day_element: DayElement {
            na_am: na_am.na_am.clone(),
            element: na_am.element.clone(),
            can_element: day_canchi.ngu_hanh.can.clone(),
            chi_element: day_canchi.ngu_hanh.chi.clone(),
            evidence: Some(rule_evidence(&data.na_am_meta, &profile)),
        },
        conflict: DayConflict {
            opposing_chi: conflict_rule.opposing_chi.clone(),
            opposing_con_giap: CON_GIAP[opposing_idx].to_string(),
            tuoi_xung: vec![
                format!("{} {}", day_canchi.can, conflict_rule.opposing_chi),
                CON_GIAP[opposing_idx].to_string(),
            ],
            sat_huong: conflict_rule.sat_huong.clone(),
            evidence: Some(rule_evidence(&data.conflict_meta, &profile)),
        },
        travel,
        stars: DayStars {
            cat_tinh,
            sat_tinh,
            day_star: Some(DayStar {
                system: StarSystem::NhiThapBatTu,
                index: day_star_index,
                name: day_star_rule.name.clone(),
                quality: parse_star_quality(&day_star_rule.quality),
                evidence: Some(rule_evidence(&data.star_meta, &profile)),
            }),
            star_system: Some(StarSystem::NhiThapBatTu),
            evidence: Some(rule_evidence(&data.star_rule_meta.fixed_by_chi, &profile)),
            matched_rules,
        },
        xung_hop: get_xung_hop(day_chi_idx),
        truc: {
            let mut truc = get_truc(day_chi_idx, lunar_month);
            truc.evidence = Some(RuleEvidence {
                source_id: "formula".to_string(),
                method: "table-lookup".to_string(),
                profile: profile.clone(),
            });
            truc
        },
    }
}

fn rule_evidence(meta: &super::types::SourceMeta, profile: &str) -> RuleEvidence {
    RuleEvidence {
        source_id: meta.source_id.clone(),
        method: meta.method.clone(),
        profile: profile.to_string(),
    }
}

fn star_category_token(category: &StarCategory) -> &'static str {
    match category {
        StarCategory::ByTietKhi => "by_tiet_khi",
        StarCategory::ByMonth => "by_month",
        StarCategory::ByYear => "by_year",
        StarCategory::FixedByCanChi => "fixed_by_canchi",
        StarCategory::FixedByChi => "fixed_by_chi",
        StarCategory::JdCycle => "jd_cycle",
    }
}

fn star_category_meta<'a>(
    category: &StarCategory,
    data: &'a super::data::AlmanacData,
) -> &'a super::types::SourceMeta {
    match category {
        StarCategory::ByTietKhi => &data.star_rule_meta.by_tiet_khi,
        StarCategory::ByMonth => &data.star_rule_meta.by_month,
        StarCategory::ByYear => &data.star_rule_meta.by_year,
        StarCategory::FixedByCanChi => &data.star_rule_meta.fixed_by_canchi,
        StarCategory::FixedByChi => &data.star_rule_meta.fixed_by_chi,
        StarCategory::JdCycle => &data.star_meta,
    }
}

fn build_star_rule_evidence(
    rules: &[StarRule],
    profile: &str,
    data: &super::data::AlmanacData,
) -> Vec<StarRuleEvidence> {
    rules
        .iter()
        .map(|rule| {
            let meta = star_category_meta(&rule.category, data);
            StarRuleEvidence {
                name: rule.name.clone(),
                quality: match rule.quality {
                    super::star::StarQualityTag::Cat => StarQuality::Cat,
                    super::star::StarQualityTag::Hung => StarQuality::Hung,
                    super::star::StarQualityTag::Binh => StarQuality::Binh,
                },
                category: star_category_token(&rule.category).to_string(),
                source_id: rule.source_id.clone(),
                method: meta.method.clone(),
                profile: profile.to_string(),
            }
        })
        .collect()
}

fn parse_star_quality(input: &str) -> StarQuality {
    match input {
        "cat" => StarQuality::Cat,
        "hung" => StarQuality::Hung,
        _ => StarQuality::Binh,
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_day_fortune;
    use crate::get_day_info;

    #[test]
    fn computes_fortune_for_tet_2024() {
        let info = get_day_info(10, 2, 2024);
        let fortune = calculate_day_fortune(
            info.jd,
            &info.canchi.day,
            info.lunar.month,
            &info.canchi.year.can,
            &info.tiet_khi.name,
        );

        assert_eq!(fortune.profile, "baseline");
        assert_eq!(fortune.conflict.opposing_chi, "Tuất");
        assert!(!fortune.travel.xuat_hanh_huong.is_empty());
        assert!(!fortune.stars.cat_tinh.is_empty());
        assert!(!fortune.stars.sat_tinh.is_empty());
    }

    #[test]
    fn computes_28_star_for_day() {
        let info = get_day_info(29, 1, 2025);
        let fortune = calculate_day_fortune(
            info.jd,
            &info.canchi.day,
            info.lunar.month,
            &info.canchi.year.can,
            &info.tiet_khi.name,
        );
        let day_star = fortune.stars.day_star.expect("day star");
        assert!(day_star.index < 28);
    }
}
