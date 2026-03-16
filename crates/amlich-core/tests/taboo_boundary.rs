mod support;

use amlich_core::DaySnapshot;
use support::{day_snapshot, day_snapshot_with_timezone};

fn taboo_ids(snapshot: &DaySnapshot) -> Vec<&str> {
    snapshot
        .day_fortune
        .taboos
        .iter()
        .map(|t| t.rule_id.as_str())
        .collect()
}

fn find_day<F>(mut predicate: F) -> Option<DaySnapshot>
where
    F: FnMut(&DaySnapshot) -> bool,
{
    for year in 2024..=2026 {
        for month in 1..=12 {
            for day in 1..=31 {
                let snapshot = day_snapshot(day, month, year);
                if snapshot.context.solar.day != day
                    || snapshot.context.solar.month != month
                    || snapshot.context.solar.year != year
                {
                    continue;
                }
                if predicate(&snapshot) {
                    return Some(snapshot);
                }
            }
        }
    }
    None
}

#[test]
fn boundary_tam_nuong_hits_on_lunar_day_three() {
    let snapshot = day_snapshot(12, 2, 2024);
    assert_eq!(snapshot.context.lunar.day, 3);
    let ids = taboo_ids(&snapshot);
    assert!(ids.contains(&"tam_nuong"));
}

#[test]
fn boundary_nguyet_ky_hits_on_lunar_day_five() {
    let snapshot = day_snapshot(14, 2, 2024);
    assert_eq!(snapshot.context.lunar.day, 5);
    let ids = taboo_ids(&snapshot);
    assert!(ids.contains(&"nguyet_ky"));
}

#[test]
fn boundary_sat_chu_rep_month_one_branch_ty() {
    let snapshot = find_day(|snapshot| {
        snapshot.context.lunar.month == 11 && snapshot.context.canchi.day.chi == "Dậu"
    })
    .expect("must find representative date for month-11 Sat Chu branch");
    let ids = taboo_ids(&snapshot);
    assert!(ids.contains(&"sat_chu"));
}

#[test]
fn boundary_tho_tu_rep_month_twelve_branch_mui() {
    let snapshot = find_day(|snapshot| {
        snapshot.context.lunar.month == 12 && snapshot.context.canchi.day.chi == "Mùi"
    })
    .expect("must find representative date for month-12 Tho Tu branch");
    let ids = taboo_ids(&snapshot);
    assert!(ids.contains(&"tho_tu"));
}

#[test]
fn boundary_timezone_can_shift_taboo_outcome() {
    let mut found = false;

    for year in 2024..=2026 {
        for month in 1..=12 {
            for day in 1..=31 {
                let vn = day_snapshot_with_timezone(day, month, year, 7.0);
                if vn.context.solar.day != day
                    || vn.context.solar.month != month
                    || vn.context.solar.year != year
                {
                    continue;
                }
                let utc = day_snapshot_with_timezone(day, month, year, 0.0);
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
