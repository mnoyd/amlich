use super::data::baseline_data;
use super::types::{DayDeity, DayDeityClassification};
use crate::types::CHI;

pub fn resolve_day_deity(lunar_month: i32, day_chi: &str) -> DayDeity {
    let data = baseline_data();

    let month_branch = lunar_month_branch_name(lunar_month);
    let start = data
        .day_deity_rule_set
        .month_group_start_by_chi
        .get(month_branch)
        .expect("day deity month group must be defined");
    let day_idx = CHI
        .iter()
        .position(|chi| *chi == day_chi)
        .expect("day chi must be valid");

    let cycle_index = (*start + day_idx) % 12;
    let entry = &data.day_deity_rule_set.cycle[cycle_index];

    DayDeity {
        name: entry.name.clone(),
        classification: parse_classification(&entry.classification),
        evidence: None,
    }
}

fn lunar_month_branch_name(lunar_month: i32) -> &'static str {
    let normalized = ((lunar_month - 1).rem_euclid(12) + 1) as usize;
    CHI[(normalized + 1) % 12]
}

fn parse_classification(input: &str) -> DayDeityClassification {
    match input {
        "hoang_dao" => DayDeityClassification::HoangDao,
        "hac_dao" => DayDeityClassification::HacDao,
        _ => panic!("invalid day deity classification token: {input}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_month_one_ty_day_to_thanh_long() {
        let deity = resolve_day_deity(1, "Tý");
        assert_eq!(deity.name, "Thanh Long");
        assert_eq!(deity.classification, DayDeityClassification::HoangDao);
    }

    #[test]
    fn resolves_month_two_ty_day_to_thien_hinh() {
        let deity = resolve_day_deity(2, "Tý");
        assert_eq!(deity.name, "Thiên Hình");
        assert_eq!(deity.classification, DayDeityClassification::HacDao);
    }

    #[test]
    fn resolves_month_nine_tuat_day_to_thien_hinh() {
        let deity = resolve_day_deity(9, "Tuất");
        assert_eq!(deity.name, "Thiên Hình");
        assert_eq!(deity.classification, DayDeityClassification::HacDao);
    }

    #[test]
    fn wraps_lunar_months_outside_1_to_12() {
        let wrapped = resolve_day_deity(13, "Tý");
        let base = resolve_day_deity(1, "Tý");
        assert_eq!(wrapped.name, base.name);
        assert_eq!(wrapped.classification, base.classification);
    }
}
