use chrono::{Datelike, Local, NaiveDate};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

const WEEKDAY_LABELS: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

pub struct CalendarViewWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> CalendarViewWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for CalendarViewWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let month_date = self.app.calendar_cursor;
        let year = month_date.year();
        let month = month_date.month();
        let today = Local::now().naive_local().date();

        let block = Block::default()
            .title(" Chọn Ngày ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 42 || inner.height < 18 {
            Paragraph::new("Phóng to cửa sổ để xem bộ chọn lịch.")
                .style(Style::default().fg(Color::DarkGray))
                .render(inner, buf);
            return;
        }

        let cell_width = (inner.width as usize / 7).max(6);
        let mut lines = vec![
            Line::from(Span::styled(
                format!("Tháng {} năm {}", month, year),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        lines.push(Line::from(
            WEEKDAY_LABELS
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    let style = if i == 6 {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    };
                    Span::styled(format!("{:^width$}", label, width = cell_width), style)
                })
                .collect::<Vec<_>>(),
        ));

        let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("valid first day");
        let padding = first_day.weekday().num_days_from_monday() as usize;
        let days_in_month = days_in_month(year, month);

        let mut day = 1u32;
        for row in 0..6 {
            let mut solar_row = Vec::with_capacity(7);
            let mut lunar_row = Vec::with_capacity(7);
            for col in 0..7 {
                if row == 0 && col < padding {
                    solar_row.push(Span::raw(" ".repeat(cell_width)));
                    lunar_row.push(Span::raw(" ".repeat(cell_width)));
                    continue;
                }

                if day > days_in_month {
                    solar_row.push(Span::raw(" ".repeat(cell_width)));
                    lunar_row.push(Span::raw(" ".repeat(cell_width)));
                    continue;
                }

                let current =
                    NaiveDate::from_ymd_opt(year, month, day).expect("valid calendar day");
                let is_cursor = current == self.app.calendar_cursor;
                let is_active = current == self.app.date;
                let is_today = current == today;
                let is_sunday = col == 6;

                let mut style = Style::default().fg(Color::White);
                if is_sunday {
                    style = style.fg(Color::Red);
                }
                if is_today {
                    style = style.add_modifier(Modifier::BOLD);
                }
                if is_active && !is_cursor {
                    style = style.add_modifier(Modifier::UNDERLINED);
                }
                if is_cursor {
                    style = style
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD);
                }

                let lunar_label = lunar_label_for_date(self.app, current);
                let lunar_style = if is_cursor {
                    style
                } else {
                    style.fg(Color::DarkGray)
                };

                solar_row.push(Span::styled(
                    format!("{:^width$}", day, width = cell_width),
                    style,
                ));
                lunar_row.push(Span::styled(
                    format!("{:^width$}", lunar_label, width = cell_width),
                    lunar_style,
                ));
                day += 1;
            }
            lines.push(Line::from(solar_row));
            lines.push(Line::from(lunar_row));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "h/l, j/k: di chuyển",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            "[ ]: đổi tháng | Enter: chọn | m/Esc: đóng | t: hôm nay",
            Style::default().fg(Color::DarkGray),
        )));

        Paragraph::new(lines).render(inner, buf);
    }
}

fn lunar_label_for_date(app: &AppState, date: NaiveDate) -> String {
    if date == app.date {
        if let Some(bundle) = app.bundle.as_ref() {
            return format!("{}/{}", bundle.lunar.day, bundle.lunar.month);
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

    amlich_api::v2::convert_solar_to_lunar(&query)
        .map(|lunar| format!("{}/{}", lunar.day, lunar.month))
        .unwrap_or_else(|_| "--/--".to_string())
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let next_start =
        NaiveDate::from_ymd_opt(next_year, next_month, 1).expect("valid start of next month");
    next_start.pred_opt().expect("previous day exists").day()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, PageSection,
    };
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
    };

    fn sample_calendar_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
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
            bundle: None,
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

    fn render_calendar_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 120, 30);
        let mut buf = Buffer::empty(area);
        CalendarViewWidget::new(app, LayoutMode::Large).render(area, &mut buf);
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
    fn calendar_popup_hints_match_picker_contract() {
        let app = sample_calendar_state();
        let text = render_calendar_text(&app);

        assert!(text.contains("Enter: chọn"));
        assert!(text.contains("Esc: đóng"));
        assert!(text.contains("[ ]: đổi tháng"));
    }
}
