use chrono::{Local, NaiveDate, Timelike};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};

use crate::{
    app::{App, DashboardCard, LayoutMode},
    dashboard::{build_preview_rows, derive_verdict, select_upcoming_holidays, PreviewRow},
    theme,
};

pub struct InfoPanel<'a> {
    app: &'a App,
}

impl<'a> InfoPanel<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    fn section_line(name: &str, width: u16, focused: bool) -> Line<'static> {
        let prefix = if focused {
            format!("▶ {} ", name)
        } else {
            format!("── {} ", name)
        };
        let remaining = (width as usize).saturating_sub(prefix.chars().count());
        let line_str = format!("{}{}", prefix, "─".repeat(remaining));

        let style = if focused {
            theme::section_style().add_modifier(Modifier::UNDERLINED)
        } else {
            theme::section_style()
        };

        Line::from(Span::styled(line_str, style))
    }

    fn is_card_focused(&self, card: DashboardCard) -> bool {
        self.app.layout_mode == LayoutMode::Large && self.app.focused_card == card
    }
}

fn current_hour_chi_index() -> usize {
    let hour = Local::now().hour() as usize;
    if hour == 23 {
        0
    } else {
        hour.div_ceil(2)
    }
}

pub(crate) fn build_hero_lines(
    info: &amlich_api::DayInfoDto,
    insight: Option<&amlich_api::DayInsightDto>,
) -> Vec<Line<'static>> {
    let guidance = insight
        .and_then(|day| day.day_guidance.as_ref())
        .map(|g| (g.good_for.vi.as_slice(), g.avoid_for.vi.as_slice()));
    let verdict = derive_verdict(guidance);

    vec![
        Line::from(vec![
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
        ]),
        Line::from(vec![
            Span::raw("🌙 "),
            Span::styled(
                info.lunar.date_string.clone(),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                info.canchi.day.full.clone(),
                Style::default().fg(theme::PRIMARY_FG),
            ),
            Span::styled(
                format!(
                    "  {} · {}",
                    info.canchi.day.ngu_hanh.can, info.canchi.day.ngu_hanh.chi
                ),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Đánh giá: ",
                Style::default()
                    .fg(theme::SECONDARY_FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{} {}", verdict.icon, verdict.label_vi),
                Style::default()
                    .fg(theme::PRIMARY_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ]
}

fn build_guidance_lines(insight: Option<&amlich_api::DayInsightDto>) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    let Some(day_insight) = insight else {
        lines.push(Line::from(Span::styled(
            "Không có thông tin",
            Style::default().fg(theme::SECONDARY_FG),
        )));
        return lines;
    };

    let Some(guidance) = day_insight.day_guidance.as_ref() else {
        lines.push(Line::from(Span::styled(
            "Không có thông tin",
            Style::default().fg(theme::SECONDARY_FG),
        )));
        return lines;
    };

    for item in guidance.good_for.vi.iter().take(4) {
        lines.push(Line::from(vec![
            Span::styled("✓ ", Style::default().fg(theme::GOOD_FG)),
            Span::styled(item.clone(), Style::default().fg(theme::PRIMARY_FG)),
        ]));
    }

    for item in guidance.avoid_for.vi.iter().take(4) {
        lines.push(Line::from(vec![
            Span::styled("✗ ", Style::default().fg(theme::BAD_FG)),
            Span::styled(item.clone(), Style::default().fg(theme::PRIMARY_FG)),
        ]));
    }

    let total = guidance.good_for.vi.len() + guidance.avoid_for.vi.len();
    if total > 8 {
        lines.push(Line::from(Span::styled(
            "[Enter] xem đầy đủ",
            Style::default().fg(theme::SECONDARY_FG),
        )));
    }

    lines
}

pub(crate) fn build_hours_lines(
    info: &amlich_api::DayInfoDto,
    max_items: usize,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let current_idx = current_hour_chi_index();

    let mut ordered = Vec::new();
    for i in current_idx..12 {
        let hour = &info.gio_hoang_dao.all_hours[i];
        if hour.is_good {
            ordered.push(hour);
        }
    }
    for i in 0..current_idx {
        let hour = &info.gio_hoang_dao.all_hours[i];
        if hour.is_good {
            ordered.push(hour);
        }
    }

    let summary = ordered
        .iter()
        .take(3)
        .map(|h| format!("{} ({})", h.hour_chi, h.time_range))
        .collect::<Vec<_>>()
        .join(" · ");

    lines.push(Line::from(vec![
        Span::styled("Tiếp theo: ", Style::default().fg(theme::SECONDARY_FG)),
        Span::styled(summary, Style::default().fg(theme::GOOD_HOUR_FG)),
    ]));

    for hour in ordered.into_iter().take(max_items) {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ({})", hour.hour_chi, hour.time_range),
                Style::default().fg(theme::GOOD_HOUR_FG),
            ),
            Span::styled(
                format!(" · {}", hour.star),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));
    }

    lines
}

fn build_holiday_context_lines(
    info: &amlich_api::DayInfoDto,
    today_holiday: Option<&amlich_api::HolidayDto>,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    if let Some(holiday) = today_holiday {
        lines.push(Line::from(vec![
            Span::styled("Hôm nay: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                holiday.name.clone(),
                Style::default()
                    .fg(theme::HOLIDAY_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    let Some(from) = NaiveDate::from_ymd_opt(
        info.solar.year,
        info.solar.month as u32,
        info.solar.day as u32,
    ) else {
        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "Không có ngày lễ gần",
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }
        return lines;
    };

    let mut holidays = amlich_api::get_holidays(info.solar.year, false);
    holidays.extend(amlich_api::get_holidays(info.solar.year + 1, false));
    let upcoming = select_upcoming_holidays(from, &holidays, 2);

    if upcoming.is_empty() {
        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                "Không có ngày lễ gần",
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }
        return lines;
    }

    for holiday in upcoming {
        lines.push(Line::from(format!(
            "• {} (còn {} ngày)",
            holiday.name, holiday.days_left
        )));
    }

    lines
}

pub(crate) fn build_preview_lines(rows: &[PreviewRow]) -> Vec<Line<'static>> {
    rows.iter()
        .map(|row| {
            Line::from(format!(
                "{}  {}",
                row.date.format("%d/%m"),
                row.date.format("%a")
            ))
        })
        .collect()
}

impl Widget for InfoPanel<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(info) = self.app.selected_info() else {
            Paragraph::new("Không có dữ liệu").render(area, buf);
            return;
        };

        let insight = self.app.selected_insight();
        let mut lines: Vec<Line<'static>> = Vec::new();
        let width = area.width;

        // Hero card
        lines.push(Self::section_line(
            "Tổng quan",
            width,
            self.is_card_focused(DashboardCard::Hero),
        ));
        lines.extend(build_hero_lines(info, insight));
        lines.push(Line::from(""));

        // Guidance card
        lines.push(Self::section_line(
            "Nên / Tránh",
            width,
            self.is_card_focused(DashboardCard::Guidance),
        ));
        lines.extend(build_guidance_lines(insight));
        lines.push(Line::from(""));

        // Hours card
        lines.push(Self::section_line(
            "Giờ tốt",
            width,
            self.is_card_focused(DashboardCard::Hours),
        ));
        lines.extend(build_hours_lines(info, 6));
        lines.push(Line::from(""));

        // Holiday context card
        lines.push(Self::section_line(
            "Ngày lễ gần",
            width,
            self.is_card_focused(DashboardCard::Holiday),
        ));
        lines.extend(build_holiday_context_lines(
            info,
            self.app.holiday_for_day(self.app.selected_day),
        ));
        lines.push(Line::from(""));

        // 3-day preview card
        lines.push(Self::section_line(
            "3 ngày tới",
            width,
            self.is_card_focused(DashboardCard::Preview),
        ));
        if let Some(from) = NaiveDate::from_ymd_opt(
            info.solar.year,
            info.solar.month as u32,
            info.solar.day as u32,
        ) {
            let rows = build_preview_rows(from, 3);
            lines.extend(build_preview_lines(&rows));
        } else {
            lines.push(Line::from(Span::styled(
                "Không thể tạo dự báo",
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::dashboard::build_preview_rows;

    use super::{build_hero_lines, build_hours_lines, build_preview_lines};

    fn sample_day_info() -> amlich_api::DayInfoDto {
        amlich_api::get_day_info_for_date(24, 2, 2026).expect("day info")
    }

    fn sample_insight() -> amlich_api::DayInsightDto {
        amlich_api::get_day_insight_for_date(24, 2, 2026).expect("day insight")
    }

    #[test]
    fn hero_card_includes_verdict_label() {
        let lines = build_hero_lines(&sample_day_info(), Some(&sample_insight()));
        let text = lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(text.contains("Đánh giá:"));
        assert!(
            text.contains("Tốt")
                || text.contains("Trung bình")
                || text.contains("Cần cân nhắc")
                || text.contains("Chưa đủ dữ liệu")
        );
    }

    #[test]
    fn hours_card_shows_time_ranges_and_stars() {
        let lines = build_hours_lines(&sample_day_info(), 6);
        let text = lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert!(text.contains(":00-"));
        assert!(text.contains("Long") || text.contains("Đường") || text.contains("Quỹ"));
    }

    #[test]
    fn preview_card_renders_three_rows() {
        let date = NaiveDate::from_ymd_opt(2026, 2, 24).expect("valid date");
        let rows = build_preview_rows(date, 3);
        let lines = build_preview_lines(&rows);

        assert!(lines.len() >= 3);
    }
}
