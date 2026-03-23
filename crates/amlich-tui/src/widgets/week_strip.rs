use chrono::{Datelike, Local, NaiveDate};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::state::AppState;

const WEEKDAY_LABELS: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

pub struct WeekStripCell {
    pub date: NaiveDate,
    pub solar_label: String,
    pub lunar_label: String,
    pub quality_badge: Option<String>,
    pub is_selected: bool,
    pub is_today: bool,
}

pub struct WeekStripWidget<'a> {
    app: &'a AppState,
}

impl<'a> WeekStripWidget<'a> {
    pub fn new(app: &'a AppState) -> Self {
        Self { app }
    }
}

const WEEKDAY_NAMES: [&str; 7] = [
    "Thứ Hai", "Thứ Ba", "Thứ Tư", "Thứ Năm", "Thứ Sáu", "Thứ Bảy", "Chủ Nhật",
];

impl Widget for WeekStripWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 32 || area.height < 2 {
            return;
        }

        let cells = build_week_strip_cells(self.app);
        
        let selected_idx = cells.iter().position(|c| c.is_selected).unwrap_or(0);
        let mut constraints = vec![Constraint::Length(6); 7];
        // The selected day absorbs all remaining horizontal space
        constraints[selected_idx] = Constraint::Min(20);
        
        let chunks = Layout::horizontal(constraints).split(area);

        for (cell, chunk) in cells.into_iter().zip(chunks.iter().copied()) {
            let weekday_abbrev = WEEKDAY_LABELS[cell.date.weekday().num_days_from_monday() as usize];
            
            if cell.is_selected {
                let full_weekday = WEEKDAY_NAMES[cell.date.weekday().num_days_from_monday() as usize];
                
                let block_style = Style::default().bg(Color::Cyan).fg(Color::Black);
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(block_style)
                    .style(block_style);
                    
                let inner = block.inner(chunk);
                block.render(chunk, buf);

                let date_str = format!("{}/{}", cell.date.day(), cell.date.month());
                
                let mut title_spans = vec![
                    Span::styled(format!("📅 {}, {} ", full_weekday, date_str), Style::default().add_modifier(Modifier::BOLD)),
                ];
                
                if cell.is_today {
                    title_spans.push(Span::styled("[Hôm nay]", Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD)));
                }

                let line1 = Line::from(title_spans);
                
                let mut lunar_details = format!("🌙 ÂL: {}", cell.lunar_label);
                if let Some(badge) = cell.quality_badge {
                    lunar_details.push_str(&format!(" ({})", badge));
                }
                
                let line2 = Line::from(Span::raw(lunar_details));

                let y_offset = if inner.height > 2 { (inner.height - 2) / 2 } else { 0 };
                let text_area = Rect {
                    x: inner.x,
                    y: inner.y + y_offset,
                    width: inner.width,
                    height: inner.height.saturating_sub(y_offset),
                };

                Paragraph::new(vec![line1, line2])
                    .alignment(Alignment::Center)
                    .render(text_area, buf);
            } else {
                let mut weekday_style = Style::default().fg(Color::DarkGray);
                let mut solar_style = Style::default().fg(Color::Gray);

                if cell.is_today {
                    weekday_style = weekday_style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                    solar_style = solar_style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                } else {
                    solar_style = solar_style.add_modifier(Modifier::BOLD);
                }

                let y_offset = if chunk.height > 2 { (chunk.height - 2) / 2 } else { 0 };
                let text_area = Rect {
                    x: chunk.x,
                    y: chunk.y + y_offset,
                    width: chunk.width,
                    height: chunk.height.saturating_sub(y_offset),
                };

                let lines = vec![
                    Line::from(Span::styled(weekday_abbrev.to_string(), weekday_style)),
                    Line::from(Span::styled(cell.solar_label, solar_style)),
                ];

                Paragraph::new(lines)
                    .alignment(Alignment::Center)
                    .render(text_area, buf);
            }
        }
    }
}

fn build_week_strip_cells(app: &AppState) -> Vec<WeekStripCell> {
    let today = Local::now().naive_local().date();
    let start_of_week = app
        .date
        .checked_sub_signed(chrono::Duration::days(
            app.date.weekday().num_days_from_monday() as i64,
        ))
        .unwrap_or(app.date);

    (0..7)
        .filter_map(|offset| {
            let date = start_of_week.checked_add_signed(chrono::Duration::days(offset))?;
            let solar_label = date.day().to_string();
            let (lunar_label, quality_badge) = day_labels(app, date);

            Some(WeekStripCell {
                date,
                solar_label,
                lunar_label,
                quality_badge,
                is_selected: date == app.date,
                is_today: date == today,
            })
        })
        .collect()
}

fn day_labels(app: &AppState, date: NaiveDate) -> (String, Option<String>) {
    if date == app.date {
        if let Some(bundle) = app.bundle.as_ref() {
            return (
                format!("{}/{}", bundle.lunar.day, bundle.lunar.month),
                bundle
                    .day_fortune
                    .as_ref()
                    .map(|fortune| fortune.truc.quality.clone()),
            );
        }
    }

    let query = amlich_api::DateQuery {
        day: date.day() as i32,
        month: date.month() as i32,
        year: date.year(),
        timezone: None,
        ruleset_id: app.applied_selection.ruleset_id.clone(),
        event_kind: None,
        enabled_pack_ids: Vec::new(),
    };

    let lunar_label = amlich_api::v2::convert_solar_to_lunar(&query)
        .map(|lunar| format!("{}/{}", lunar.day, lunar.month))
        .unwrap_or_else(|_| "--/--".to_string());

    (lunar_label, None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        AppState, ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, PageSection,
    };
    use amlich_api::{
        LunarDto, RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
        SolarDto,
    };
    use ratatui::widgets::Widget;

    fn sample_app_state_with_bundle() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 9, 20).expect("valid date");
        let ruleset_catalog = vec![RulesetCatalogEntryDto {
            id: "vn_baseline_v1".to_string(),
            canonical_id: "vn_baseline_v1".to_string(),
            version: "v1".to_string(),
            region: "vn".to_string(),
            profile: "baseline".to_string(),
            schema_version: "amlich.engine/v1".to_string(),
            is_default: true,
            aliases: vec![],
            defaults: RulesetDefaultsDto {
                tz_offset: 7.0,
                meridian: None,
            },
            source_notes: vec![],
        }];
        let recommendation_pack_catalog = vec![RecommendationPackCatalogEntryDto {
            pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
            request_field: "enabled_pack_ids".to_string(),
            version: "v1".to_string(),
            source_family: "traditional".to_string(),
            mode: "advisory".to_string(),
        }];
        let selection = ExplorerSelection::defaults(date, &ruleset_catalog);
        AppState {
            running: true,
            date,
            lens: FocusLens::General,

            scroll_offset: 0,
            bundle: Some(amlich_api::v2::DayBundleDto {
                schema_version: "amlich.engine/v1".to_string(),
                ruleset_id: "vn_baseline_v1".to_string(),
                ruleset_version: "v1".to_string(),
                profile: "baseline".to_string(),
                generated_at: "2026-09-20T00:00:00Z".to_string(),
                solar: SolarDto {
                    day: 20,
                    month: 9,
                    year: 2026,
                    day_of_week: 7,
                    day_of_week_name: "Chủ Nhật".to_string(),
                    date_string: "2026-09-20".to_string(),
                },
                lunar: LunarDto {
                    day: 15,
                    month: 8,
                    year: 2026,
                    is_leap_month: false,
                    date_string: "Rằm tháng 8".to_string(),
                },
                jd: 0,
                canchi: None,
                tiet_khi: None,
                gio_hoang_dao: None,
                day_fortune: None,
                daily_recommendations: None,
                contextual_recommendations: None,
                insight: None,
                upcoming_events: vec![],
            }),
            is_loading: false,
            error_msg: None,
            ruleset_catalog,
            recommendation_pack_catalog,
            applied_selection: selection.clone(),
            staged_selection: selection,
            explorer_focus: ExplorerField::Date,
            explorer_action: ExplorerAction::Apply,
            pack_cursor: 0,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            show_week_strip: true,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            app_mode: crate::state::AppMode::Normal,
            search_input: String::new(),
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Dashboard,
            view_history: Vec::new(),
        }
    }

    fn render_week_strip_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 84, 4);
        let mut buf = Buffer::empty(area);
        WeekStripWidget::new(app).render(area, &mut buf);
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
    fn renders_seven_days_with_solar_and_lunar_labels() {
        let app = sample_app_state_with_bundle();
        let rendered = render_week_strip_text(&app);

        assert!(rendered.contains("T2"));
        assert!(rendered.contains("20"));
        assert!(rendered.contains("15/8"));
    }
}
