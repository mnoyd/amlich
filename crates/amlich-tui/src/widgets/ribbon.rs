use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
#[cfg(test)]
use amlich_api::{RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto};
use chrono::Datelike;

const WEEKDAY_NAMES: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

pub struct RibbonWidget<'a> {
    app: &'a AppState,
}

impl<'a> RibbonWidget<'a> {
    pub fn new(app: &'a AppState, _mode: LayoutMode) -> Self {
        Self { app }
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

        let screen_name = self.app.active_view_label();

        let dow0 = self.app.date.weekday().num_days_from_monday() as usize;

        let hotkey_line = Line::from(vec![
            Span::styled(
                format!(" [{}] ", screen_name),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Tab: màn  ←/→: ngày  t: hôm nay  m: tháng  ?: trợ giúp",
                Style::default().fg(Color::DarkGray),
            ),
        ]);

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
    use crate::state::{
        ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, PageSection,
    };
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
            focused_section: PageSection::Recommendations,
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

    fn render_lines(app: &AppState) -> Vec<String> {
        let area = Rect::new(0, 0, 120, 2);
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

        assert!(hotkey_line.contains("[Dashboard]"));
        assert!(hotkey_line.contains("Tab: màn"));
        assert!(hotkey_line.contains("?: trợ giúp"));
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
