use amlich_core::{get_day_info, get_day_info_with_timezone};

fn taboo_ids(info: &amlich_core::DayInfo) -> Vec<&str> {
    info.day_fortune
        .taboos
        .iter()
        .map(|t| t.rule_id.as_str())
        .collect()
}

fn find_day<F>(mut predicate: F) -> Option<amlich_core::DayInfo>
where
    F: FnMut(&amlich_core::DayInfo) -> bool,
{
    for year in 2024..=2026 {
        for month in 1..=12 {
            for day in 1..=31 {
                let info = get_day_info(day, month, year);
                if info.solar.day != day || info.solar.month != month || info.solar.year != year {
                    continue;
                }
                if predicate(&info) {
                    return Some(info);
                }
            }
        }
    }
    None
}

#[test]
fn boundary_tam_nuong_hits_on_lunar_day_three() {
    let info = get_day_info(12, 2, 2024);
    assert_eq!(info.lunar.day, 3);
    let ids = taboo_ids(&info);
    assert!(ids.contains(&"tam_nuong"));
}

#[test]
fn boundary_nguyet_ky_hits_on_lunar_day_five() {
    let info = get_day_info(14, 2, 2024);
    assert_eq!(info.lunar.day, 5);
    let ids = taboo_ids(&info);
    assert!(ids.contains(&"nguyet_ky"));
}

#[test]
fn boundary_sat_chu_rep_month_one_branch_ty() {
    let info = find_day(|info| info.lunar.month == 11 && info.canchi.day.chi == "Dậu")
        .expect("must find representative date for month-11 Sat Chu branch");
    let ids = taboo_ids(&info);
    assert!(ids.contains(&"sat_chu"));
}

#[test]
fn boundary_tho_tu_rep_month_twelve_branch_mui() {
    let info = find_day(|info| info.lunar.month == 12 && info.canchi.day.chi == "Mùi")
        .expect("must find representative date for month-12 Tho Tu branch");
    let ids = taboo_ids(&info);
    assert!(ids.contains(&"tho_tu"));
}

#[test]
fn boundary_timezone_can_shift_taboo_outcome() {
    let mut found = false;

    for year in 2024..=2026 {
        for month in 1..=12 {
            for day in 1..=31 {
                let vn = get_day_info_with_timezone(day, month, year, 7.0);
                if vn.solar.day != day || vn.solar.month != month || vn.solar.year != year {
                    continue;
                }
                let utc = get_day_info_with_timezone(day, month, year, 0.0);
                if taboo_ids(&vn) != taboo_ids(&utc) {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            break;
        }
    }

    assert!(
        found,
        "expected at least one solar day with timezone-sensitive taboo outcome"
    );
}
