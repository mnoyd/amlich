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

impl Widget for WeekStripWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 28 || area.height < 4 {
            return;
        }

        let cells = build_week_strip_cells(self.app);
        let chunks = Layout::horizontal([Constraint::Ratio(1, 7); 7]).split(area);

        for (cell, chunk) in cells.into_iter().zip(chunks.iter().copied()) {
            let weekday = WEEKDAY_LABELS[cell.date.weekday().num_days_from_monday() as usize];
            let mut block_style = Style::default().fg(Color::DarkGray);
            if cell.is_today {
                block_style = block_style.add_modifier(Modifier::UNDERLINED);
            }
            if cell.is_selected {
                block_style = block_style
                    .bg(Color::Cyan)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }

            let mut lunar_line = cell.lunar_label;
            if let Some(badge) = cell.quality_badge {
                lunar_line.push(' ');
                lunar_line.push_str(&badge);
            }

            let block = Block::default().borders(Borders::ALL).style(block_style);
            let inner = block.inner(chunk);
            block.render(chunk, buf);
            Paragraph::new(vec![
                Line::from(Span::raw(weekday.to_string())),
                Line::from(Span::raw(format!("{} {}", cell.solar_label, lunar_line))),
            ])
            .alignment(Alignment::Center)
            .style(block_style)
            .render(inner, buf);
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
        ActiveView, AppState, ExplorerAction, ExplorerField, ExplorerSelection, FocusLens,
        PageSection,
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
