use chrono::{Datelike, Local, NaiveDate};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

const WEEKDAY_LABELS: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventCategory {
    Holiday,
    Festival,
    Lunar,
}

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

        let (calendar_area, events_area) = if inner.width >= 80 {
            let chunks = Layout::horizontal([Constraint::Length(56), Constraint::Min(24)]).split(inner);
            (chunks[0], Some(chunks[1]))
        } else if inner.height >= 26 {
            let chunks = Layout::vertical([Constraint::Length(18), Constraint::Min(8)]).split(inner);
            (chunks[0], Some(chunks[1]))
        } else {
            (inner, None)
        };

        let cell_width = (calendar_area.width as usize / 7).max(6);
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
        let mut month_events: Vec<(u32, Vec<(EventCategory, String)>)> = Vec::new();
        let mut cursor_events: Vec<(EventCategory, String)> = Vec::new();

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
                    if is_cursor {
                        style = style.bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD);
                    } else {
                        style = style.bg(Color::Green).fg(Color::Black).add_modifier(Modifier::BOLD);
                    }
                } else if is_cursor {
                    style = style.bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD);
                } else if is_active {
                    style = style.add_modifier(Modifier::UNDERLINED);
                }

                let (lunar_label, events_today) = lunar_info_and_events(self.app, current);
                if current == self.app.calendar_cursor {
                    cursor_events = events_today.clone();
                }

                let display_lunar_label = if !events_today.is_empty() {
                    month_events.push((day, events_today));
                    format!("{} •", lunar_label)
                } else {
                    lunar_label
                };

                let lunar_style = if is_cursor || is_today {
                    style
                } else {
                    style.fg(Color::DarkGray)
                };

                solar_row.push(Span::styled(
                    format!("{:^width$}", day, width = cell_width),
                    style,
                ));
                lunar_row.push(Span::styled(
                    format!("{:^width$}", display_lunar_label, width = cell_width),
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

        Paragraph::new(lines).render(calendar_area, buf);

        if let Some(area) = events_area {
            let mut event_lines = vec![
                Line::from(vec![
                    Span::styled("📌 ", Style::default()),
                    Span::styled(
                        format!("Chi tiết Ngày {}", self.app.calendar_cursor.day()),
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    ),
                ])
            ];

            if cursor_events.is_empty() {
                event_lines.push(Line::from(Span::styled("  Không có sự kiện.", Style::default().fg(Color::DarkGray))));
            } else {
                for (cat, name) in &cursor_events {
                    let (icon, color) = match cat {
                        EventCategory::Holiday => ("🔴", Color::Red),
                        EventCategory::Festival => ("🎉", Color::Magenta),
                        EventCategory::Lunar => ("🌕", Color::Yellow),
                    };
                    event_lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(icon, Style::default().fg(color)),
                        Span::raw(" "),
                        Span::styled(name.clone(), Style::default().fg(Color::White)),
                    ]));
                }
            }

            event_lines.push(Line::from(""));
            event_lines.push(Line::from(Span::styled(
                "Sự kiện trong tháng",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            )));
            event_lines.push(Line::from(""));

            if month_events.is_empty() {
                event_lines.push(Line::from(Span::styled(" Không có sự kiện", Style::default().fg(Color::DarkGray))));
            } else {
                for (d, evts) in month_events {
                    let mut evt_texts = Vec::new();
                    for (cat, name) in evts {
                        let icon = match cat {
                            EventCategory::Holiday => "🔴",
                            EventCategory::Festival => "🎉",
                            EventCategory::Lunar => "🌕",
                        };
                        evt_texts.push(format!("{} {}", icon, name));
                    }
                    event_lines.push(Line::from(vec![
                        Span::raw(" "),
                        Span::styled(format!("Ngày {:02}: ", d), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
                        Span::styled(evt_texts.join(", "), Style::default().fg(Color::Gray)),
                    ]));
                    event_lines.push(Line::from(""));
                }
            }

            Paragraph::new(event_lines)
                .wrap(ratatui::widgets::Wrap { trim: false })
                .render(
                    ratatui::layout::Rect {
                        x: area.x + 2,
                        y: area.y,
                        width: area.width.saturating_sub(2),
                        height: area.height,
                    },
                    buf,
                );
        }
    }
}

fn lunar_info_and_events(app: &AppState, date: NaiveDate) -> (String, Vec<(EventCategory, String)>) {
    let query = amlich_api::DateQuery {
        day: date.day() as i32,
        month: date.month() as i32,
        year: date.year(),
        timezone: None,
        ruleset_id: app.applied_selection.ruleset_id.clone(),
        event_kind: None,
        enabled_pack_ids: Vec::new(),
    };

    let mut events = Vec::new();
    let label = if let Ok(insight) = amlich_api::v2::get_insight(&query) {
        if insight.lunar.day == 1 || insight.lunar.day == 15 {
            events.push((EventCategory::Lunar, format!("Mùng {} âm lịch", insight.lunar.day)));
        }
        if let Some(fest) = insight.festival {
            for name in fest.names.vi {
                events.push((EventCategory::Festival, name));
            }
        }
        if let Some(hol) = insight.holiday {
            for name in hol.names.vi {
                events.push((EventCategory::Holiday, name));
            }
        }
        format!("{}/{}", insight.lunar.day, insight.lunar.month)
    } else {
        amlich_api::v2::convert_solar_to_lunar(&query)
            .map(|lunar| format!("{}/{}", lunar.day, lunar.month))
            .unwrap_or_else(|_| "--/--".to_string())
    };

    if date == app.date {
        if let Some(bundle) = app.bundle.as_ref() {
            return (format!("{}/{}", bundle.lunar.day, bundle.lunar.month), events);
        }
    }

    (label, events)
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
    use crate::state::{ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, PageSection};
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
