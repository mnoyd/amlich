use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::{
    layout::LayoutMode,
    state::{AppState, ExplorerAction, ExplorerField},
};

pub struct ExplorerWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> ExplorerWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for ExplorerWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Điều Khiển Nhanh ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        block.render(area, buf);

        let staged_marker = if self.app.explorer_has_staged_changes() {
            "CHƯA ÁP DỤNG"
        } else {
            "ĐÃ ĐỒNG BỘ"
        };

        let mut lines = vec![
            Line::from(vec![
                label(self.app, ExplorerField::Date, "Ngày"),
                Span::raw(format!(
                    ": {}",
                    self.app.staged_selection.date.format("%Y-%m-%d")
                )),
            ]),
            Line::from(vec![
                label(self.app, ExplorerField::EventKind, "Sự kiện"),
                Span::raw(format!(
                    ": {}",
                    self.app
                        .event_kind_label(self.app.staged_selection.event_kind.as_deref())
                )),
            ]),
            Line::from(vec![
                label(self.app, ExplorerField::RecommendationPacks, "Gói đề xuất"),
                Span::raw(format!(": {}", staged_marker)),
            ]),
        ];

        for row in self.app.pack_status_rows() {
            lines.push(Line::from(Span::styled(
                format!("  {row}"),
                Style::default().fg(Color::White),
            )));
        }

        lines.push(Line::from(vec![Span::styled(
            format!(
                "Nguồn dữ liệu: {}",
                self.app
                    .ruleset_brief_label(self.app.applied_selection.ruleset_id.as_deref())
            ),
            Style::default().fg(Color::Gray),
        )]));
        lines.push(Line::from(vec![
            label(self.app, ExplorerField::Actions, "Áp dụng"),
            Span::raw(": "),
            action_chip(self.app, ExplorerAction::Apply, "Apply"),
            Span::raw("  "),
            action_chip(self.app, ExplorerAction::Reset, "Reset mặc định"),
        ]));
        lines.push(Line::from(Span::styled(
            format!(
                "Đang áp dụng: {} | {}",
                self.app
                    .event_kind_label(self.app.applied_selection.event_kind.as_deref()),
                self.app.active_pack_summary(&self.app.applied_selection)
            ),
            Style::default().fg(Color::Gray),
        )));
        if self.app.explorer_has_staged_changes() {
            lines.push(Line::from(Span::styled(
                format!(
                    "Chờ áp dụng: {} | {} | {}",
                    self.app.staged_selection.date.format("%Y-%m-%d"),
                    self.app
                        .event_kind_label(self.app.staged_selection.event_kind.as_deref()),
                    self.app.active_pack_summary(&self.app.staged_selection)
                ),
                Style::default().fg(Color::Yellow),
            )));
        }

        Paragraph::new(lines)
            .wrap(Wrap { trim: true })
            .render(inner, buf);
    }
}

fn label(app: &AppState, field: ExplorerField, text: &str) -> Span<'static> {
    let style = if app.explorer_focus == field {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    Span::styled(text.to_string(), style)
}

fn action_chip(app: &AppState, action: ExplorerAction, text: &str) -> Span<'static> {
    let selected = app.explorer_focus == ExplorerField::Actions && app.explorer_action == action;
    let style = if selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    Span::styled(text.to_string(), style)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ExplorerSelection, PageSection};
    use amlich_api::{
        RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto, RulesetDefaultsDto,
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
            aliases: vec!["default".to_string()],
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
            explorer_focus: ExplorerField::RecommendationPacks,
            explorer_action: ExplorerAction::Apply,
            pack_cursor: 0,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            show_week_strip: true,
            show_graph_recommendations: false,
            verbosity: crate::state::ui_prefs::VerbosityMode::Compact,
            focused_section: PageSection::Explorer,
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
            graph_inspector_focus: crate::state::GraphInspectorFocus::Summary,
            graph_inspector_cursor: 0,
            graph_inspector_search_query: String::new(),
            graph_inspector_search_cursor: 0,
            graph_inspector_focus_before_search: None,
            graph_inspector_lens: crate::state::GraphInspectorLens::General,
        }
    }

    #[test]
    fn pack_rows_show_visible_selected_state() {
        let mut app = sample_app_state();
        app.staged_selection.enabled_pack_ids = vec!["pack.nhi_thap_bat_tu.v1".to_string()];

        let rows = app.pack_status_rows();

        assert_eq!(
            rows[0],
            "> [+] pack.nhi_thap_bat_tu.v1 · traditional · advisory"
        );
    }

    #[test]
    fn summaries_include_event_kind_between_ruleset_and_pack_state() {
        let mut app = sample_app_state();
        app.applied_selection.event_kind = Some("travel".to_string());
        app.staged_selection.event_kind = Some("travel".to_string());

        let applied = format!(
            "Applied: {} | {} | {} | {}",
            app.applied_selection.date.format("%Y-%m-%d"),
            app.ruleset_label(app.applied_selection.ruleset_id.as_deref()),
            app.event_kind_label(app.applied_selection.event_kind.as_deref()),
            app.active_pack_summary(&app.applied_selection)
        );

        assert!(applied.contains("travel · Xuất hành"));
    }
}
