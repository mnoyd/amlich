use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::{ui_prefs::VerbosityMode, AppState};
#[cfg(test)]
use amlich_api::{RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto};
use chrono::Datelike;

const WEEKDAY_NAMES: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

pub struct RibbonWidget<'a> {
    app: &'a AppState,
    mode: LayoutMode,
}

impl<'a> RibbonWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, mode }
    }
}

impl Widget for RibbonWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.app.is_calendar_view() {
            let line = Line::from(vec![
                Span::styled(
                    " [Lịch] ",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "h/l, j/k: di chuyển  ",
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled("[ ]: đổi tháng  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    "Enter: chọn  m/Esc: đóng",
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            let p = Paragraph::new(line).alignment(Alignment::Center);
            let bottom_line = Rect {
                x: area.x,
                y: area.y + 1,
                width: area.width,
                height: 1,
            };
            p.render(bottom_line, buf);
            return;
        }

        let available = self.app.available_views();
        let mut view_spans = vec![];
        for v in available.iter() {
            let label = match self.mode {
                LayoutMode::Small => {
                    if v != &self.app.active_view {
                        continue;
                    }
                    format!("< [{}] >", v.short_label())
                }
                LayoutMode::Medium => {
                    if v == &self.app.active_view {
                        format!(" [{}] ", v.short_label())
                    } else {
                        format!(" {} ", v.short_label())
                    }
                }
                LayoutMode::Large => {
                    if v == &self.app.active_view {
                        format!(" [{}] ", v.label())
                    } else {
                        format!(" {} ", v.label())
                    }
                }
            };

            let style = if v == &self.app.active_view {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            view_spans.push(Span::styled(label, style));
        }

        let mut all_spans = view_spans;
        let verbosity_label = match self.app.active_verbosity() {
            VerbosityMode::Compact => "Compact",
            VerbosityMode::Verbose => "Verbose",
        };
        all_spans.push(Span::styled(
            format!(
                "| v: {verbosity_label}  Tab: màn  1-5: chọn  ←/→: ngày  t: hôm nay  ?: trợ giúp"
            ),
            Style::default().fg(Color::DarkGray),
        ));

        let dow0 = self.app.date.weekday().num_days_from_monday() as usize;

        let hotkey_line = Line::from(all_spans);

        let top_line = Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: 1,
        };
        Paragraph::new(hotkey_line)
            .alignment(Alignment::Center)
            .render(top_line, buf);

        let mut spans = vec![Span::raw(" < ")];

        for (i, name) in WEEKDAY_NAMES.iter().enumerate() {
            if i == dow0 {
                // Today/Selected
                spans.push(Span::styled(
                    format!("[{}] ", name),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ));
            } else {
                spans.push(Span::styled(
                    format!("{} ", name),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }
        spans.push(Span::raw(">"));

        let p = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
        let bottom_line = Rect {
            x: area.x,
            y: area.y + 1,
            width: area.width,
            height: 1,
        };
        p.render(bottom_line, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ExplorerAction, ExplorerField, ExplorerSelection, PageSection};
    use chrono::NaiveDate;

    fn sample_app_state() -> AppState {
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

            scroll_offset: 0,
            content_height: 0,
            viewport_height: 0,
            bundle: None,
            personal_matrix: None,
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
            verbosity: crate::state::ui_prefs::VerbosityMode::Compact,
            focused_section: PageSection::Recommendations,
            zoomed_section: None,
            expanded_sections: Default::default(),
            app_mode: crate::state::AppMode::Normal,
            search_input: String::new(),
            personal_focus: crate::state::PersonalField::BirthYear,
            personal_draft: crate::state::PersonalDraft {
                birth_year: String::new(),
                birth_month: String::new(),
                birth_day: String::new(),
                birth_hour: String::new(),
                birth_minute: String::new(),
                gender: None,
            },
            calendar_cursor: date,
            navigation_history: Vec::new(),
            active_view: crate::state::ActiveView::Today,
            view_history: Vec::new(),
        }
    }

    fn render_lines(app: &AppState) -> Vec<String> {
        let area = Rect::new(0, 0, 200, 2);
        let mut buf = Buffer::empty(area);
        RibbonWidget::new(app, LayoutMode::Large).render(area, &mut buf);
        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn ribbon_places_hotkeys_on_the_top_line_only() {
        let app = sample_app_state();
        let lines = render_lines(&app);
        let hotkey_line = &lines[0];
        let weekday_line = &lines[1];

        assert!(hotkey_line.contains("[Hôm Nay]"));
        assert!(hotkey_line.contains("Tab: màn"));
        assert!(hotkey_line.contains("1-5: chọn"));
        assert!(!weekday_line.contains("Tab:"));
        assert!(!weekday_line.contains("màn"));
    }

    #[test]
    fn ribbon_keeps_weekday_strip_clean_and_highlighted() {
        let app = sample_app_state();
        let lines = render_lines(&app);
        let weekday_line = &lines[1];

        assert!(weekday_line.contains("[T5]"));
        assert!(weekday_line.contains("T2"));
        assert!(weekday_line.contains("CN"));
    }
}
