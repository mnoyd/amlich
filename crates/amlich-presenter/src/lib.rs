use amlich_api::DayInfoDto;

pub fn format_day_info(info: &DayInfoDto) -> String {
    let mut lines = Vec::new();

    lines.push(format!(
        "📅 Ngày {} ({})",
        info.solar.date_string, info.solar.day_of_week_name
    ));
    lines.push(format!("🌙 Âm lịch: {}", info.lunar.date_string));
    lines.push("📜 Can Chi:".to_string());
    lines.push(format!(
        "   • Ngày: {} ({})",
        info.canchi.day.full, info.canchi.day.con_giap
    ));
    lines.push(format!("   • Tháng: {}", info.canchi.month.full));
    lines.push(format!(
        "   • Năm: {} ({})",
        info.canchi.year.full, info.canchi.year.con_giap
    ));
    lines.push("🌟 Ngũ hành:".to_string());
    lines.push(format!(
        "   • Ngày: {} (Can) - {} (Chi)",
        info.canchi.day.ngu_hanh.can, info.canchi.day.ngu_hanh.chi
    ));
    lines.push(format!(
        "🌤️  Tiết khí: {} - {}",
        info.tiet_khi.name, info.tiet_khi.season
    ));
    lines.push(format!("   • {}", info.tiet_khi.description));
    lines.push(format!(
        "   • Kinh độ mặt trời: {}°",
        info.tiet_khi.current_longitude
    ));
    lines.push(format!(
        "⏰ Giờ Hoàng Đạo ({} giờ tốt):",
        info.gio_hoang_dao.good_hour_count
    ));
    for h in &info.gio_hoang_dao.good_hours {
        lines.push(format!(
            "   • {} ({}) - {}",
            h.hour_chi, h.time_range, h.star
        ));
    }

    lines.join("\n")
}
