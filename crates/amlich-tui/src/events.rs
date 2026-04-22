use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::state::{AppState, ExplorerAction, ExplorerField};

pub fn handle_events(app: &mut AppState) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            return Ok(dispatch_key(app, key.code, key.modifiers));
        }
    }

    Ok(false)
}

pub(crate) fn dispatch_key(app: &mut AppState, code: KeyCode, modifiers: KeyModifiers) -> bool {
    if code == KeyCode::Char('c') && modifiers == KeyModifiers::CONTROL {
        return true;
    }

    if app.app_mode == crate::state::AppMode::SearchModal {
        match code {
            KeyCode::Esc => app.toggle_search(),
            KeyCode::Enter => {
                let q = app.search_input.clone();
                if let Ok(d) = chrono::NaiveDate::parse_from_str(&q, "%Y-%m-%d") {
                    app.jump_to_date(d);
                } else if let Ok(d) = chrono::NaiveDate::parse_from_str(&q, "%d/%m/%Y") {
                    app.jump_to_date(d);
                }
                app.toggle_search();
            }
            KeyCode::Char(c) => app.search_input.push(c),
            KeyCode::Backspace => {
                app.search_input.pop();
            }
            _ => {}
        }
        return false;
    }

    // Global view switching
    match code {
        KeyCode::Tab => {
            app.next_view();
            return false;
        }
        KeyCode::BackTab => {
            app.prev_view();
            return false;
        }
        KeyCode::Char('1') => {
            app.go_to_view(crate::state::ActiveView::Today);
            return false;
        }
        KeyCode::Char('2') => {
            app.go_to_view(crate::state::ActiveView::DayDetail);
            return false;
        }
        KeyCode::Char('3') => {
            app.go_to_view(crate::state::ActiveView::Hours);
            return false;
        }
        KeyCode::Char('4') => {
            app.go_to_view(crate::state::ActiveView::Calendar);
            return false;
        }
        KeyCode::Char('5') => {
            app.go_to_view(crate::state::ActiveView::Personal);
            return false;
        }
        KeyCode::Char('6') => {
            app.go_to_view(crate::state::ActiveView::GraphInspector);
            return false;
        }
        _ => {}
    }

    if app.is_calendar_view() {
        match code {
            KeyCode::Char('q') => app.running = false,
            KeyCode::Char('m') | KeyCode::Esc => app.close_calendar_view(),
            KeyCode::Enter => app.apply_calendar_selection(),
            KeyCode::Right | KeyCode::Char('l') => app.calendar_move_days(1),
            KeyCode::Left | KeyCode::Char('h') => app.calendar_move_days(-1),
            KeyCode::Down | KeyCode::Char('j') => app.calendar_move_days(7),
            KeyCode::Up | KeyCode::Char('k') => app.calendar_move_days(-7),
            KeyCode::Char(']') | KeyCode::PageDown | KeyCode::Char('n') | KeyCode::Char('L') => {
                app.calendar_next_month()
            }
            KeyCode::Char('[') | KeyCode::PageUp | KeyCode::Char('p') | KeyCode::Char('H') => {
                app.calendar_prev_month()
            }
            KeyCode::Char('t') => app.calendar_go_today(),
            _ => {}
        }
        return false;
    }

    if app.active_view == crate::state::ActiveView::GraphInspector {
        match code {
            KeyCode::Char('q') => app.running = false,
            KeyCode::Char('d') => app.toggle_dev_inspector_mode(),
            KeyCode::Esc => {
                if !app.dev_inspector_mode {
                    match app.causality_focus {
                        crate::state::CausalityFocus::SummaryList => app.running = false,
                        crate::state::CausalityFocus::DetailFlow(_) => app.causality_go_back(),
                    }
                } else {
                    use crate::state::GraphInspectorFocus;
                    match &app.graph_inspector_focus {
                        GraphInspectorFocus::Summary
                        | GraphInspectorFocus::ReasoningLens
                        | GraphInspectorFocus::RecommendationLens
                        | GraphInspectorFocus::ConvergenceLens => app.running = false,
                        GraphInspectorFocus::Search => app.graph_inspector_exit_search(),
                        _ => app.graph_inspector_go_back(),
                    }
                }
            }
            KeyCode::Char('r') => app.toggle_graph_recommendations(),
            KeyCode::Tab => app.graph_inspector_cycle_lens(),
            KeyCode::Right => app.navigate_days(1),
            KeyCode::Left => app.navigate_days(-1),
            KeyCode::Char('t') => app.jump_to_today(),
            KeyCode::Down | KeyCode::Char('j') => {
                use crate::state::GraphInspectorFocus;
                match &app.graph_inspector_focus {
                    GraphInspectorFocus::Summary => app.scroll_down(),
                    GraphInspectorFocus::Search => app.graph_inspector_search_move_cursor(1),
                    GraphInspectorFocus::ReasoningLens
                    | GraphInspectorFocus::RecommendationLens
                    | GraphInspectorFocus::ConvergenceLens => app.graph_inspector_move_cursor(1),
                    _ => app.graph_inspector_move_cursor(1),
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                use crate::state::GraphInspectorFocus;
                match &app.graph_inspector_focus {
                    GraphInspectorFocus::Summary => app.scroll_up(),
                    GraphInspectorFocus::Search => app.graph_inspector_search_move_cursor(-1),
                    GraphInspectorFocus::ReasoningLens
                    | GraphInspectorFocus::RecommendationLens
                    | GraphInspectorFocus::ConvergenceLens => app.graph_inspector_move_cursor(-1),
                    _ => app.graph_inspector_move_cursor(-1),
                }
            }
            KeyCode::Enter | KeyCode::Char('l') => {
                if !app.dev_inspector_mode {
                    app.causality_drill_down("".to_string()); // Will be updated in UI implementation
                } else {
                    use crate::state::GraphInspectorFocus;
                    match &app.graph_inspector_focus {
                        GraphInspectorFocus::Search => app.graph_inspector_search_select(),
                        _ => app.graph_inspector_drill_down(),
                    }
                }
            }
            KeyCode::Backspace | KeyCode::Char('h') => {
                if !app.dev_inspector_mode {
                    app.causality_go_back();
                } else {
                    use crate::state::GraphInspectorFocus;
                    match &app.graph_inspector_focus {
                        GraphInspectorFocus::Search => app.graph_inspector_search_backspace(),
                        GraphInspectorFocus::ReasoningLens
                        | GraphInspectorFocus::RecommendationLens
                        | GraphInspectorFocus::ConvergenceLens => {
                            app.graph_inspector_lens = crate::state::GraphInspectorLens::General;
                            app.graph_inspector_focus = crate::state::GraphInspectorFocus::Summary;
                        }
                        _ => app.graph_inspector_go_back(),
                    }
                }
            }
            KeyCode::Char('/') | KeyCode::Char('s') => {
                use crate::state::GraphInspectorFocus;
                if app.graph_inspector_focus != GraphInspectorFocus::Search {
                    app.graph_inspector_enter_search();
                }
            }
            KeyCode::Char(c) => {
                use crate::state::GraphInspectorFocus;
                if app.graph_inspector_focus == GraphInspectorFocus::Search {
                    app.graph_inspector_search_insert_char(c);
                }
            }
            _ => {}
        }
        return false;
    }

    if app.app_mode == crate::state::AppMode::PersonalProfileModal {
        match code {
            KeyCode::Esc => app.toggle_personal_profile_modal(),
            KeyCode::Enter => app.apply_personal_profile(),
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => app.personal_next_field(),
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => app.personal_previous_field(),
            KeyCode::Up | KeyCode::Char('k') => {
                if app.personal_focus == crate::state::PersonalField::Gender {
                    app.personal_cycle_gender(-1);
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.personal_focus == crate::state::PersonalField::Gender {
                    app.personal_cycle_gender(1);
                }
            }
            KeyCode::Backspace => app.personal_backspace(),
            KeyCode::Char(c) => {
                if app.personal_focus == crate::state::PersonalField::Gender {
                    match c {
                        'm' | 'M' => app.personal_cycle_gender(-1),
                        'f' | 'F' => app.personal_cycle_gender(1),
                        _ => app.personal_insert_char(c),
                    }
                } else {
                    app.personal_insert_char(c);
                }
            }
            _ => {}
        }
        return false;
    }

    if app.app_mode == crate::state::AppMode::ContextModal {
        match code {
            KeyCode::Esc | KeyCode::Char('o') => app.toggle_context_modal(),
            KeyCode::Char('r') if app.error_msg.is_some() => app.retry_load(),
            KeyCode::Down | KeyCode::Char('j') => match app.explorer_focus {
                ExplorerField::Date => app.next_day(),
                ExplorerField::Ruleset => app.cycle_ruleset(1),
                ExplorerField::EventKind => app.cycle_event_kind(1),
                ExplorerField::RecommendationPacks => app.move_pack_cursor(1),
                ExplorerField::Actions => app.cycle_explorer_action(),
            },
            KeyCode::Up | KeyCode::Char('k') => match app.explorer_focus {
                ExplorerField::Date => app.prev_day(),
                ExplorerField::Ruleset => app.cycle_ruleset(-1),
                ExplorerField::EventKind => app.cycle_event_kind(-1),
                ExplorerField::RecommendationPacks => app.move_pack_cursor(-1),
                ExplorerField::Actions => app.cycle_explorer_action(),
            },
            KeyCode::Right | KeyCode::Char('l') | KeyCode::Tab => {
                let fields = [
                    ExplorerField::EventKind,
                    ExplorerField::RecommendationPacks,
                    ExplorerField::Ruleset,
                    ExplorerField::Date,
                    ExplorerField::Actions,
                ];
                let idx = fields
                    .iter()
                    .position(|f| *f == app.explorer_focus)
                    .unwrap();
                let next_idx = (idx + 1) % fields.len();
                app.explorer_focus = fields[next_idx];
            }
            KeyCode::Left | KeyCode::Char('h') | KeyCode::BackTab => {
                let fields = [
                    ExplorerField::EventKind,
                    ExplorerField::RecommendationPacks,
                    ExplorerField::Ruleset,
                    ExplorerField::Date,
                    ExplorerField::Actions,
                ];
                let idx = fields
                    .iter()
                    .position(|f| *f == app.explorer_focus)
                    .unwrap();
                let prev_idx = if idx == 0 { fields.len() - 1 } else { idx - 1 };
                app.explorer_focus = fields[prev_idx];
            }
            KeyCode::Char(' ') => {
                if app.explorer_focus == ExplorerField::RecommendationPacks {
                    app.toggle_focused_pack();
                } else if app.explorer_focus == ExplorerField::Date {
                    app.open_calendar_view();
                }
            }
            KeyCode::Enter => {
                if app.explorer_focus == ExplorerField::Actions {
                    match app.explorer_action {
                        ExplorerAction::Apply => {
                            app.apply_staged_selection();
                            app.toggle_context_modal();
                        }
                        ExplorerAction::Reset => {
                            app.reset_staged_selection();
                        }
                    }
                } else if app.explorer_focus == ExplorerField::RecommendationPacks {
                    app.toggle_focused_pack();
                    app.apply_staged_selection();
                } else {
                    app.apply_staged_selection();
                    app.toggle_context_modal();
                }
            }
            _ => {}
        }
        return false;
    }

    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
        KeyCode::Char('r') if app.error_msg.is_some() => app.retry_load(),
        KeyCode::Right | KeyCode::Char('l') => app.navigate_days(1),
        KeyCode::Left | KeyCode::Char('h') => app.navigate_days(-1),
        KeyCode::Char('L') => app.navigate_months(1),
        KeyCode::Char('H') => app.navigate_months(-1),
        KeyCode::Char(']') => app.navigate_months(1),
        KeyCode::Char('[') => app.navigate_months(-1),
        KeyCode::Char('t') => app.jump_to_today(),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
        KeyCode::PageDown => app.scroll_down_by(10),
        KeyCode::PageUp => app.scroll_up_by(10),
        KeyCode::Char('y') => {
            if let Some(bundle) = &app.bundle {
                let cc = bundle.canchi.as_ref();
                let cc_day = cc.map(|c| c.day.full.as_str()).unwrap_or("");
                let cc_month = cc.map(|c| c.month.full.as_str()).unwrap_or("");
                let cc_year = cc.map(|c| c.year.full.as_str()).unwrap_or("");

                let ghd = bundle
                    .gio_hoang_dao
                    .as_ref()
                    .map(|g| g.summary.as_str())
                    .unwrap_or("Không có");

                let summary = format!(
                    "Dương Lịch: {} ({})\nÂm Lịch: {}\nCan Chi: Ngày {}, Tháng {}, Năm {}\nGiờ Hoàng Đạo: {}",
                    bundle.solar.date_string,
                    bundle.solar.day_of_week_name,
                    bundle.lunar.date_string,
                    cc_day,
                    cc_month,
                    cc_year,
                    ghd
                );
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(summary);
                }
            }
        }
        KeyCode::Char('e') => app.toggle_evidence(),
        KeyCode::Char('v') => app.toggle_verbosity(),
        KeyCode::Char('m') => app.open_calendar_view(),
        KeyCode::Char('w') => app.toggle_week_strip(),
        KeyCode::Char('o') => app.toggle_context_modal(),
        KeyCode::Char('p') if app.active_view == crate::state::ActiveView::Personal => {
            app.toggle_personal_profile_modal()
        }
        KeyCode::Char('?') => app.toggle_help_modal(),
        KeyCode::Char('g') => app.toggle_search(),
        KeyCode::Char('u') => app.undo_navigation(),
        KeyCode::Char(' ') | KeyCode::Char('c') => app.toggle_calendar_view(),
        KeyCode::Char('/') | KeyCode::Char('s') => app.toggle_search(),
        _ => {}
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ActiveView, AppMode, ExplorerAction, ExplorerField, ExplorerSelection, PageSection,
    };
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
            app_mode: AppMode::Normal,
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
            show_graph_recommendations: false,
            verbosity: crate::state::ui_prefs::VerbosityMode::Compact,
            focused_section: PageSection::Explorer,
            zoomed_section: None,
            expanded_sections: Default::default(),

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
    fn tab_cycles_screen_forward() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Hero;

        dispatch_key(&mut app, KeyCode::Tab, KeyModifiers::NONE);

        assert_eq!(app.active_view, ActiveView::DayDetail);
    }

    #[test]
    fn backtab_cycles_screen_backward() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Hero;

        dispatch_key(&mut app, KeyCode::BackTab, KeyModifiers::SHIFT);

        assert_eq!(app.active_view, ActiveView::GraphInspector);
    }

    #[test]
    fn enter_does_not_toggle_section_in_screen_mode() {
        let mut app = sample_app_state();
        app.focus_section(PageSection::TraditionalEvidence);
        assert!(!app.is_section_expanded(PageSection::TraditionalEvidence));

        dispatch_key(&mut app, KeyCode::Enter, KeyModifiers::NONE);
        assert!(!app.is_section_expanded(PageSection::TraditionalEvidence));
    }

    #[test]
    fn char_e_toggles_evidence_visibility() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Hero;

        dispatch_key(&mut app, KeyCode::Char('e'), KeyModifiers::NONE);
        assert!(app.show_evidence);

        dispatch_key(&mut app, KeyCode::Char('e'), KeyModifiers::NONE);
        assert!(!app.show_evidence);
    }

    #[test]
    fn key_t_jumps_to_today() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Hero;

        dispatch_key(&mut app, KeyCode::Char('t'), KeyModifiers::NONE);

        assert_eq!(app.date, chrono::Local::now().naive_local().date());
    }

    #[test]
    fn key_shift_h_and_shift_l_navigate_month() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Hero;
        let start = app.date;

        dispatch_key(&mut app, KeyCode::Char('L'), KeyModifiers::SHIFT);
        // month navigation adds 1 month. 2026-03-12 -> 2026-04-12
        let mut expected_date = start;
        expected_date = expected_date
            .checked_add_months(chrono::Months::new(1))
            .unwrap();
        assert_eq!(app.date, expected_date);

        dispatch_key(&mut app, KeyCode::Char('H'), KeyModifiers::SHIFT);
        assert_eq!(app.date, start);
    }

    #[test]
    fn key_m_opens_month_popup_and_enter_applies_selection() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Hero;
        let start = app.staged_selection.date;

        dispatch_key(&mut app, KeyCode::Char('m'), KeyModifiers::NONE);
        assert!(app.is_calendar_view());

        dispatch_key(&mut app, KeyCode::Right, KeyModifiers::NONE);
        dispatch_key(&mut app, KeyCode::Enter, KeyModifiers::NONE);

        assert!(!app.is_calendar_view());
        assert_eq!(app.staged_selection.date, start + chrono::Duration::days(1));
    }

    #[test]
    fn char_z_does_not_toggle_zoom_in_screen_mode() {
        let mut app = sample_app_state();
        app.focus_section(PageSection::Risks);

        dispatch_key(&mut app, KeyCode::Char('z'), KeyModifiers::NONE);
        assert_eq!(app.zoomed_section, None);
    }

    #[test]
    fn char_a_does_not_expand_recommendation_section_in_screen_mode() {
        let mut app = sample_app_state();
        app.focus_section(PageSection::Risks);

        dispatch_key(&mut app, KeyCode::Char('a'), KeyModifiers::NONE);

        assert_eq!(app.focused_section, PageSection::Risks);
        assert!(!app.is_section_expanded(PageSection::Recommendations));
    }

    #[test]
    fn explorer_space_toggles_pack_without_applying() {
        let mut app = sample_app_state();
        app.app_mode = crate::state::AppMode::ContextModal;
        app.explorer_focus = ExplorerField::RecommendationPacks;

        dispatch_key(&mut app, KeyCode::Char(' '), KeyModifiers::NONE);

        assert_eq!(
            app.staged_selection.enabled_pack_ids,
            vec!["pack.nhi_thap_bat_tu.v1"]
        );
        assert!(app.applied_selection.enabled_pack_ids.is_empty());
    }

    #[test]
    fn explorer_enter_on_reset_restores_defaults() {
        let mut app = sample_app_state();
        app.app_mode = crate::state::AppMode::ContextModal;
        app.staged_selection.event_kind = Some("travel".to_string());
        app.explorer_focus = ExplorerField::Actions;
        app.explorer_action = ExplorerAction::Reset;

        dispatch_key(&mut app, KeyCode::Enter, KeyModifiers::NONE);

        assert_eq!(app.staged_selection.event_kind, None);
    }
}
