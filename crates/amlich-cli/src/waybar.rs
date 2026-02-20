use amlich_api::{get_holidays, DayInfoDto};
use chrono::{Local, NaiveDate, Timelike};

use crate::DisplayMode;

const HOLIDAY_LOOKAHEAD_DAYS: i64 = 10;

fn format_full(info: &DayInfoDto) -> String {
    format!(
        "📅 {}/{}/{} 🌙 {}/{}/{} ({}) 📜 {}",
        info.solar.day,
        info.solar.month,
        info.solar.year,
        info.lunar.day,
        info.lunar.month,
        info.lunar.year,
        info.canchi.year.full,
        info.canchi.day.full
    )
}

fn format_lunar(info: &DayInfoDto) -> String {
    if info.lunar.is_leap_month {
        format!(
            "🌙 {}/{}/{} (Nhuận)",
            info.lunar.day, info.lunar.month, info.lunar.year
        )
    } else {
        format!(
            "🌙 {}/{}/{}",
            info.lunar.day, info.lunar.month, info.lunar.year
        )
    }
}

fn format_canchi(info: &DayInfoDto) -> String {
    format!("📜 {}", info.canchi.day.full)
}

fn format_minimal(info: &DayInfoDto) -> String {
    format!("{}/{}", info.lunar.day, info.lunar.month)
}

fn current_hour_chi_index() -> usize {
    let hour = Local::now().hour() as usize;
    if hour == 23 {
        0
    } else {
        hour.div_ceil(2)
    }
}

fn next_good_hours(info: &DayInfoDto, current_chi: usize, max_count: usize) -> Vec<String> {
    let mut ordered = Vec::new();

    for i in current_chi..12 {
        let h = &info.gio_hoang_dao.all_hours[i];
        if h.is_good {
            ordered.push(format!("{} ({})", h.hour_chi, h.time_range));
        }
    }
    for i in 0..current_chi {
        let h = &info.gio_hoang_dao.all_hours[i];
        if h.is_good {
            ordered.push(format!("{} ({})", h.hour_chi, h.time_range));
        }
    }

    ordered.into_iter().take(max_count).collect()
}

fn format_holiday_line(info: &DayInfoDto) -> Option<String> {
    let today = NaiveDate::from_ymd_opt(
        info.solar.year,
        info.solar.month as u32,
        info.solar.day as u32,
    )?;

    let mut holidays = get_holidays(info.solar.year, false);
    holidays.extend(get_holidays(info.solar.year + 1, false));

    let today_holidays: Vec<_> = holidays
        .iter()
        .filter(|h| {
            h.solar_year == info.solar.year
                && h.solar_month == info.solar.month
                && h.solar_day == info.solar.day
        })
        .map(|h| h.name.clone())
        .collect();

    if !today_holidays.is_empty() {
        return Some(format!("🎉 Hôm nay: {}", today_holidays.join(" • ")));
    }

    let mut best_name: Option<String> = None;
    let mut best_diff = i64::MAX;

    for h in &holidays {
        if let Some(date) =
            NaiveDate::from_ymd_opt(h.solar_year, h.solar_month as u32, h.solar_day as u32)
        {
            let diff = date.signed_duration_since(today).num_days();
            if diff > 0 && diff < best_diff {
                best_diff = diff;
                best_name = Some(h.name.clone());
            }
        }
    }

    if let Some(name) = best_name {
        if best_diff <= HOLIDAY_LOOKAHEAD_DAYS {
            return Some(format!("🎉 Sắp tới: {} (còn {} ngày)", name, best_diff));
        }
    }

    None
}

fn format_tooltip(info: &DayInfoDto) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "📅 {} ({}) • 🌙 {}",
        info.solar.date_string, info.solar.day_of_week_name, info.lunar.date_string
    ));
    lines.push(format!(
        "📜 Can Chi: {} • tháng {} • năm {}",
        info.canchi.day.full, info.canchi.month.full, info.canchi.year.full
    ));
    lines.push(format!(
        "🌟 Ngũ hành: {} (Can) - {} (Chi)",
        info.canchi.day.ngu_hanh.can, info.canchi.day.ngu_hanh.chi
    ));

    lines.push(String::new());
    lines.push(format!(
        "🌤️ Tiết khí: {} • {}",
        info.tiet_khi.name, info.tiet_khi.season
    ));
    lines.push(format!(
        "   Kinh độ mặt trời: {:.1}°",
        info.tiet_khi.current_longitude
    ));

    let current_idx = current_hour_chi_index();
    let current = &info.gio_hoang_dao.all_hours[current_idx];
    let state = if current.is_good { "✅" } else { "❌" };

    lines.push(String::new());
    lines.push(format!(
        "⏰ Giờ hiện tại: {} {} • {}",
        current.hour_chi, current.time_range, current.star
    ));
    lines.push(format!("   Trạng thái: {}", state));

    let next = next_good_hours(info, current_idx, 3);
    if !next.is_empty() {
        lines.push(format!("   Giờ tốt tiếp theo: {}", next.join(", ")));
    }

    if let Some(holiday_line) = format_holiday_line(info) {
        lines.push(String::new());
        lines.push(holiday_line);
    }

    lines.join("\n")
}

pub fn format_waybar_json(info: &DayInfoDto, mode: &DisplayMode) -> String {
    let text = match mode {
        DisplayMode::Full => format_full(info),
        DisplayMode::Lunar => format_lunar(info),
        DisplayMode::CanChi => format_canchi(info),
        DisplayMode::Minimal => format_minimal(info),
    };

    let tooltip = format_tooltip(info);
    let class = mode.to_string();

    serde_json::json!({
        "text": text,
        "tooltip": tooltip,
        "class": class
    })
    .to_string()
}
