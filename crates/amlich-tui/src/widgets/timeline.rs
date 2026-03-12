use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct TimelineWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> TimelineWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for TimelineWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let mut lines = vec![];
        let header_style = Style::default().fg(Color::DarkGray);

        let block = Block::default()
            .title(" Khung Giờ Và Hành Động ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
            
        let inner = block.inner(area);
        block.render(area, buf);

        let Some(hours_data) = &bundle.gio_hoang_dao else {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled("Chưa có dữ liệu giờ tốt.", Style::default().fg(Color::Gray)),
            ]));
            Paragraph::new(lines).render(inner, buf);
            return;
        };

        let top_windows = top_good_windows(hours_data);
        if !top_windows.is_empty() {
            lines.push(Line::from(vec![
                Span::raw("   "),
                Span::styled(
                    format!("Khung tốt: {}", top_windows.join(", ")),
                    Style::default().fg(Color::Green),
                ),
            ]));
        }
        lines.push(Line::from(vec![
            Span::raw("   "),
            Span::styled(action_summary(hours_data), Style::default().fg(Color::Yellow)),
        ]));

        match self.mode {
            LayoutMode::Small => {
                lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(
                        format!("Theo canh giờ: {}", compact_good_hours(hours_data)),
                        Style::default().fg(Color::Gray),
                    ),
                ]));
            }
            _ => {
                let mut top_border = vec![Span::styled("   ┌", header_style)];
                let mut bar_row = vec![Span::styled("   │", header_style)];
                let mut bot_border = vec![Span::styled("   └", header_style)];
                let mut label_row = vec![Span::raw("    ")];

                for (i, hd) in hours_data.all_hours.iter().enumerate() {
                    top_border.push(Span::styled("─────", header_style));

                    if hd.is_good {
                        bar_row.push(Span::styled(" ██  ", Style::default().fg(Color::Green)));
                    } else {
                        bar_row.push(Span::styled("     ", header_style));
                    }

                    bot_border.push(Span::styled("─────", header_style));

                    if i < 11 {
                        top_border.push(Span::styled("┬", header_style));
                        bar_row.push(Span::styled("│", header_style));
                        bot_border.push(Span::styled("┴", header_style));
                    } else {
                        top_border.push(Span::styled("┐", header_style));
                        bar_row.push(Span::styled("│", header_style));
                        bot_border.push(Span::styled("┘", header_style));
                    }

                    let range_short = hd.time_range.replace(":00", "").replace(" - ", "-");
                    label_row.push(Span::styled(
                        format!("{:^5}", range_short),
                        Style::default().fg(Color::DarkGray),
                    ));
                    if i < 11 {
                        label_row.push(Span::raw(" "));
                    }
                }

                lines.push(Line::from(top_border));
                lines.push(Line::from(bar_row));
                lines.push(Line::from(bot_border));
                lines.push(Line::from(label_row));
            }
        }

        Paragraph::new(lines).render(inner, buf);
    }
}

fn top_good_windows(hours_data: &amlich_api::GioHoangDaoDto) -> Vec<String> {
    hours_data
        .good_hours
        .iter()
        .take(3)
        .map(|hour| hour.time_range.clone())
        .collect()
}

fn compact_good_hours(hours_data: &amlich_api::GioHoangDaoDto) -> String {
    let labels: Vec<String> = hours_data
        .good_hours
        .iter()
        .take(4)
        .map(|hour| format!("{} ({})", hour.time_range, hour.hour_chi))
        .collect();
    if labels.is_empty() {
        "Không có khung nổi bật.".to_string()
    } else {
        labels.join(" · ")
    }
}

fn action_summary(hours_data: &amlich_api::GioHoangDaoDto) -> String {
    if hours_data.good_hours.is_empty() {
        return "Chưa xác định được khung hành động nổi bật.".to_string();
    }

    let first = &hours_data.good_hours[0].time_range;
    let last = hours_data
        .good_hours
        .last()
        .map(|hour| hour.time_range.as_str())
        .unwrap_or(first.as_str());

    format!(
        "Nên ưu tiên các việc chính từ {first} đến {last}; tổng cộng {} khung giờ tốt.",
        hours_data.good_hour_count
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::{GioHoangDaoDto, HourInfoDto, LunarDto, SolarDto};
    use amlich_api::v2::{ApiMetaDto, DayBundleDto};
    use chrono::NaiveDate;
    use crate::state::{FocusLens, PageSection, ViewMode};

    fn sample_hours() -> GioHoangDaoDto {
        GioHoangDaoDto {
            day_chi: "Ngọ".to_string(),
            good_hour_count: 3,
            good_hours: vec![
                HourInfoDto {
                    hour_index: 0,
                    hour_chi: "Tý".to_string(),
                    time_range: "23:00 - 01:00".to_string(),
                    star: "Thanh Long".to_string(),
                    is_good: true,
                },
                HourInfoDto {
                    hour_index: 1,
                    hour_chi: "Sửu".to_string(),
                    time_range: "01:00 - 03:00".to_string(),
                    star: "Minh Đường".to_string(),
                    is_good: true,
                },
                HourInfoDto {
                    hour_index: 2,
                    hour_chi: "Dần".to_string(),
                    time_range: "03:00 - 05:00".to_string(),
                    star: "Kim Quỹ".to_string(),
                    is_good: true,
                },
            ],
            all_hours: vec![
                HourInfoDto {
                    hour_index: 0,
                    hour_chi: "Tý".to_string(),
                    time_range: "23:00 - 01:00".to_string(),
                    star: "Thanh Long".to_string(),
                    is_good: true,
                },
                HourInfoDto {
                    hour_index: 1,
                    hour_chi: "Sửu".to_string(),
                    time_range: "01:00 - 03:00".to_string(),
                    star: "Minh Đường".to_string(),
                    is_good: true,
                },
                HourInfoDto {
                    hour_index: 2,
                    hour_chi: "Dần".to_string(),
                    time_range: "03:00 - 05:00".to_string(),
                    star: "Kim Quỹ".to_string(),
                    is_good: true,
                },
                HourInfoDto {
                    hour_index: 3,
                    hour_chi: "Mão".to_string(),
                    time_range: "05:00 - 07:00".to_string(),
                    star: "Thiên Hình".to_string(),
                    is_good: false,
                },
            ],
            summary: "Giờ đẹp đầu ngày".to_string(),
        }
    }

    fn sample_app_state(hours: Option<GioHoangDaoDto>) -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: Some(DayBundleDto {
                meta: ApiMetaDto {
                    schema_version: "amlich.api/v2".to_string(),
                    ruleset_id: "test".to_string(),
                    ruleset_version: "v1".to_string(),
                    profile: "baseline".to_string(),
                    generated_at: "2026-03-12T00:00:00Z".to_string(),
                },
                solar: SolarDto {
                    day: 12,
                    month: 3,
                    year: 2026,
                    day_of_week: 4,
                    day_of_week_name: "Thứ Năm".to_string(),
                    date_string: "2026-03-12".to_string(),
                },
                lunar: LunarDto {
                    day: 4,
                    month: 2,
                    year: 2026,
                    is_leap_month: false,
                    date_string: "Mùng 4 tháng Hai".to_string(),
                },
                jd: 0,
                canchi: None,
                tiet_khi: None,
                gio_hoang_dao: hours,
                day_fortune: None,
                daily_recommendations: None,
                insight: None,
            }),
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            focused_section: PageSection::Timing,
            zoomed_section: None,
            expanded_sections: Default::default(),
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
        }
    }

    fn render_text(app: &AppState, mode: LayoutMode) -> String {
        let area = Rect::new(0, 0, 100, 10);
        let mut buf = Buffer::empty(area);
        TimelineWidget::new(app, mode).render(area, &mut buf);

        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn timeline_surfaces_top_good_windows() {
        let app = sample_app_state(Some(sample_hours()));
        let text = render_text(&app, LayoutMode::Large);

        assert!(text.contains("23:00 - 01:00, 01:00 - 03:00, 03:00 - 05:00"));
        assert!(text.contains("Nên ưu tiên"));
    }

    #[test]
    fn timeline_renders_visual_distribution_for_medium_and_large_modes() {
        let app = sample_app_state(Some(sample_hours()));
        let medium = render_text(&app, LayoutMode::Medium);
        let large = render_text(&app, LayoutMode::Large);

        assert!(medium.contains("┌─────┬─────"));
        assert!(medium.contains("23-01"));
        assert!(large.contains("┌─────┬─────"));
        assert!(large.contains("03-05"));
    }

    #[test]
    fn timeline_falls_back_to_compact_text_on_small_mode() {
        let app = sample_app_state(Some(sample_hours()));
        let text = render_text(&app, LayoutMode::Small);

        assert!(text.contains("Khung tốt"));
        assert!(text.contains("23:00 - 01:00"));
        assert!(!text.contains("┌─────┬─────"));
    }

    #[test]
    fn timeline_handles_absent_hour_data_gracefully() {
        let app = sample_app_state(None);
        let text = render_text(&app, LayoutMode::Large);

        assert!(text.contains("Chưa có dữ liệu giờ tốt."));
    }
}
