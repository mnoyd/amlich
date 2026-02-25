use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::theme;

pub struct InfoPanel<'a> {
    app: &'a App,
}

impl<'a> InfoPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    /// Build section divider: "── Section Name ──────..."
    fn section_line(name: &str, width: u16) -> Line<'static> {
        let prefix = format!("── {} ", name);
        let remaining = (width as usize).saturating_sub(prefix.chars().count());
        let line_str = format!("{}{}", prefix, "─".repeat(remaining));
        Line::from(Span::styled(line_str, theme::section_style()))
    }

    fn star_quality_label(input: &str) -> &'static str {
        match input {
            "cat" => "Cát",
            "hung" => "Hung",
            _ => "Bình",
        }
    }

    fn taboo_severity_score(input: &str) -> u8 {
        match input {
            "high" => 3,
            "medium" => 2,
            "low" => 1,
            _ => 0,
        }
    }

    fn taboo_severity_label(input: &str) -> &'static str {
        match input {
            "high" => "cao",
            "medium" => "vừa",
            "low" => "thấp",
            _ => "không rõ",
        }
    }

    fn build_lines(&self, width: u16) -> Vec<Line<'_>> {
        let mut lines: Vec<Line<'_>> = Vec::new();
        let Some(info) = self.app.selected_info() else {
            lines.push(Line::from("Không có dữ liệu"));
            return lines;
        };

        lines.push(Line::from(vec![
            Span::styled(
                info.solar.date_string.clone(),
                Style::default()
                    .fg(theme::PRIMARY_FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {}", info.solar.day_of_week_name),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::raw("🌙 "),
            Span::styled(
                info.lunar.date_string.clone(),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        lines.push(Line::from(""));

        lines.push(Self::section_line("Can Chi / Mệnh", width));
        lines.push(Line::from(vec![
            Span::styled("Ngày: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                format!("{} ({})", info.canchi.day.full, info.canchi.day.con_giap),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Tháng: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                info.canchi.month.full.clone(),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Năm: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                format!("{} ({})", info.canchi.year.full, info.canchi.year.con_giap),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        if let Some(fortune) = &info.day_fortune {
            lines.push(Line::from(vec![
                Span::styled("Mệnh ngày: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    format!(
                        "{} ({})",
                        fortune.day_element.na_am, fortune.day_element.element
                    ),
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Trực: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    format!(
                        "{} ({})",
                        fortune.truc.name,
                        Self::star_quality_label(&fortune.truc.quality)
                    ),
                    Style::default().fg(theme::PRIMARY_FG),
                ),
            ]));

            lines.push(Line::from(""));
            lines.push(Self::section_line("Almanac nhanh", width));
            lines.push(Line::from(vec![
                Span::styled("Ruleset: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    format!(
                        "{}@{} ({})",
                        fortune.ruleset_id, fortune.ruleset_version, fortune.profile
                    ),
                    Style::default().fg(theme::PRIMARY_FG),
                ),
            ]));
            if let Some(day_deity) = &fortune.day_deity {
                lines.push(Line::from(vec![
                    Span::styled("Thần ngày: ", Style::default().fg(theme::SECONDARY_FG)),
                    Span::styled(
                        format!("{} ({})", day_deity.name, day_deity.classification),
                        Style::default().fg(theme::PRIMARY_FG),
                    ),
                ]));
            }
            if let Some(day_star) = &fortune.stars.day_star {
                lines.push(Line::from(vec![
                    Span::styled("Sao chủ: ", Style::default().fg(theme::SECONDARY_FG)),
                    Span::styled(
                        format!(
                            "{} - {}",
                            day_star.name,
                            Self::star_quality_label(&day_star.quality)
                        ),
                        Style::default().fg(theme::PRIMARY_FG),
                    ),
                ]));
            }
            if !fortune.taboos.is_empty() {
                let max_severity = fortune
                    .taboos
                    .iter()
                    .max_by_key(|taboo| Self::taboo_severity_score(&taboo.severity));
                if let Some(taboo) = max_severity {
                    lines.push(Line::from(vec![
                        Span::styled("Mức kiêng: ", Style::default().fg(theme::SECONDARY_FG)),
                        Span::styled(
                            format!(
                                "{} mục (cao nhất: {})",
                                fortune.taboos.len(),
                                Self::taboo_severity_label(&taboo.severity)
                            ),
                            Style::default().fg(theme::BAD_FG),
                        ),
                    ]));
                }
            }
            lines.push(Line::from(Span::styled(
                "[a] xem chi tiết almanac",
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines.push(Line::from(""));
        lines.push(Self::section_line("Giờ hoàng đạo / Tuổi xung", width));
        let hour_summary = info
            .gio_hoang_dao
            .good_hours
            .iter()
            .map(|h| format!("{}({})", h.hour_chi, h.time_range))
            .collect::<Vec<_>>()
            .join(" · ");
        lines.push(Line::from(vec![
            Span::styled("Giờ tốt: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(hour_summary, Style::default().fg(theme::GOOD_HOUR_FG)),
        ]));
        if let Some(fortune) = &info.day_fortune {
            lines.push(Line::from(vec![
                Span::styled("Tuổi xung: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.conflict.tuoi_xung.join(", "),
                    Style::default().fg(theme::BAD_FG),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Lục xung: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.xung_hop.luc_xung.clone(),
                    Style::default().fg(theme::BAD_FG),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Tam hợp: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.xung_hop.tam_hop.join(" · "),
                    Style::default().fg(theme::GOOD_FG),
                ),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Self::section_line("Xuất hành / Thần hướng", width));
        if let Some(fortune) = &info.day_fortune {
            lines.push(Line::from(vec![
                Span::styled("Xuất hành: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.travel.xuat_hanh_huong.clone(),
                    Style::default().fg(theme::PRIMARY_FG),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Tài thần: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.travel.tai_than.clone(),
                    Style::default().fg(theme::PRIMARY_FG),
                ),
                Span::styled("  Hỷ thần: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.travel.hy_than.clone(),
                    Style::default().fg(theme::PRIMARY_FG),
                ),
            ]));
        } else {
            lines.push(Line::from(Span::styled(
                "Không có thông tin",
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines.push(Line::from(""));
        lines.push(Self::section_line("Cát tinh / Sát tinh / Sao", width));
        if let Some(fortune) = &info.day_fortune {
            lines.push(Line::from(vec![
                Span::styled("Cát tinh: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.stars.cat_tinh.join(", "),
                    Style::default().fg(theme::GOOD_FG),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("Sát tinh: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.stars.sat_tinh.join(", "),
                    Style::default().fg(theme::BAD_FG),
                ),
            ]));
            if let Some(star) = &fortune.stars.day_star {
                lines.push(Line::from(vec![
                    Span::styled("Sao: ", Style::default().fg(theme::SECONDARY_FG)),
                    Span::styled(
                        format!(
                            "{} ({} - {})",
                            star.name,
                            star.system,
                            Self::star_quality_label(&star.quality)
                        ),
                        Style::default().fg(theme::PRIMARY_FG),
                    ),
                ]));
            }
        } else {
            lines.push(Line::from(Span::styled(
                "Không có thông tin",
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("── Tiết khí ", theme::section_style()),
            Span::styled(
                format!("{} · {}", info.tiet_khi.name, info.tiet_khi.season),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        if !info.tiet_khi.description.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("   {}", info.tiet_khi.description),
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        if let Some(insight) = self.app.selected_insight() {
            if let Some(guidance) = &insight.day_guidance {
                lines.push(Line::from(""));
                lines.push(Self::section_line("Nên / Tránh", width));
                for item in guidance.good_for.vi.iter().take(3) {
                    lines.push(Line::from(vec![
                        Span::styled("✓ ", Style::default().fg(theme::GOOD_FG)),
                        Span::styled(item.clone(), Style::default().fg(theme::PRIMARY_FG)),
                    ]));
                }
                for item in guidance.avoid_for.vi.iter().take(3) {
                    lines.push(Line::from(vec![
                        Span::styled("✗ ", Style::default().fg(theme::BAD_FG)),
                        Span::styled(item.clone(), Style::default().fg(theme::PRIMARY_FG)),
                    ]));
                }
            }
        }

        if let Some(holiday) = self.app.holiday_for_day(self.app.selected_day) {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("── Ngày lễ ", theme::section_style()),
                Span::styled(
                    holiday.name.clone(),
                    Style::default()
                        .fg(theme::HOLIDAY_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            if !holiday.description.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("   {}", holiday.description),
                    Style::default().fg(theme::SECONDARY_FG),
                )));
            }
        }

        lines
    }
}

impl Widget for InfoPanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let lines = self.build_lines(area.width);
        let p = Paragraph::new(lines).wrap(Wrap { trim: false });
        p.render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::InfoPanel;
    use crate::app::App;

    fn line_to_string(line: &ratatui::text::Line<'_>) -> String {
        line.spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect::<String>()
    }

    #[test]
    fn expanded_day_view_lines_include_almanac_fields() {
        let date = NaiveDate::from_ymd_opt(2024, 2, 10).expect("valid date");
        let app = App::new_with_date(Some(date));
        let panel = InfoPanel::new(&app);
        let lines = panel.build_lines(120);
        let text = lines
            .iter()
            .map(line_to_string)
            .collect::<Vec<_>>()
            .join("\n");

        for needle in [
            "Ngày:",
            "Tháng:",
            "Mệnh ngày:",
            "Trực:",
            "Giờ tốt:",
            "Tuổi xung:",
            "Lục xung:",
            "Tam hợp:",
            "Xuất hành:",
            "Tài thần:",
            "Hỷ thần:",
            "Cát tinh:",
            "Sát tinh:",
            "Sao:",
            "Tiết khí",
            "Almanac nhanh",
            "Ruleset:",
            "[a] xem chi tiết almanac",
        ] {
            assert!(text.contains(needle), "missing `{needle}` in rendered info");
        }
    }
}
