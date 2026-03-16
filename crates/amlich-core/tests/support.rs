use amlich_core::{
    calculate_day_snapshot, calculate_day_snapshot_with_timezone, DaySnapshot,
};

pub fn day_snapshot(day: i32, month: i32, year: i32) -> DaySnapshot {
    calculate_day_snapshot(day, month, year)
}

#[allow(dead_code)]
pub fn day_snapshot_with_timezone(day: i32, month: i32, year: i32, time_zone: f64) -> DaySnapshot {
    calculate_day_snapshot_with_timezone(day, month, year, time_zone)
}
