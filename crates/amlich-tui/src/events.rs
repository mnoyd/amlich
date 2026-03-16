use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::state::{AppState, ExplorerField, PageSection};

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

    if app.show_search {
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

    if app.is_calendar_view() {
        match code {
            KeyCode::Char('q') => app.running = false,
            KeyCode::Char('m') | KeyCode::Esc => app.close_calendar_view(),
            KeyCode::Enter => app.apply_calendar_selection(),
            KeyCode::Right | KeyCode::Char('l') => app.calendar_move_days(1),
            KeyCode::Left | KeyCode::Char('h') => app.calendar_move_days(-1),
            KeyCode::Down | KeyCode::Char('j') => app.calendar_move_days(7),
            KeyCode::Up | KeyCode::Char('k') => app.calendar_move_days(-7),
            KeyCode::Char(']') | KeyCode::PageDown | KeyCode::Char('n') => {
                app.calendar_next_month()
            }
            KeyCode::Char('[') | KeyCode::PageUp | KeyCode::Char('p') => app.calendar_prev_month(),
            KeyCode::Char('t') => app.calendar_go_today(),
            _ => {}
        }
        return false;
    }

    if app.focused_section == PageSection::Explorer {
        match code {
            KeyCode::Char('q') | KeyCode::Esc => app.running = false,
            KeyCode::Char('r') if app.error_msg.is_some() => app.retry_load(),
            KeyCode::Tab => app.focus_next_section(),
            KeyCode::BackTab => app.focus_previous_section(),
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
            KeyCode::Right | KeyCode::Char('l') => app.focus_next_explorer_field(),
            KeyCode::Left | KeyCode::Char('h') => app.focus_previous_explorer_field(),
            KeyCode::Char(' ') => match app.explorer_focus {
                ExplorerField::RecommendationPacks => app.toggle_focused_pack(),
                ExplorerField::Actions => app.cycle_explorer_action(),
                _ => {}
            },
            KeyCode::Char('r') => app.reset_staged_selection(),
            KeyCode::Enter => app.activate_explorer_focus(),
            KeyCode::Char('t') => app.jump_to_today(),
            KeyCode::Char('m') => app.open_calendar_view(),
            KeyCode::Char('w') => app.toggle_week_strip(),
            KeyCode::Char('g') => app.toggle_search(),
            KeyCode::Char('u') => app.undo_navigation(),
            KeyCode::Char('c') => app.toggle_calendar_view(),
            _ => {}
        }
        return false;
    }

    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
        KeyCode::Char('r') if app.error_msg.is_some() => app.retry_load(),
        KeyCode::Right | KeyCode::Char('l') => app.navigate_days(1),
        KeyCode::Left | KeyCode::Char('h') => app.navigate_days(-1),
        KeyCode::Char('L') => app.navigate_weeks(1),
        KeyCode::Char('H') => app.navigate_weeks(-1),
        KeyCode::Char(']') => app.navigate_months(1),
        KeyCode::Char('[') => app.navigate_months(-1),
        KeyCode::Char('t') => app.jump_to_today(),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
        KeyCode::PageDown => app.scroll_down_by(10),
        KeyCode::PageUp => app.scroll_up_by(10),
        KeyCode::Tab => app.focus_next_section(),
        KeyCode::BackTab => app.focus_previous_section(),
        KeyCode::Enter => app.toggle_expand_focused_section(),
        KeyCode::Char('e') => app.toggle_evidence(),
        KeyCode::Char('z') => app.toggle_zoom_for_focused_section(),
        KeyCode::Char('a') => app.expand_section(PageSection::Recommendations),
        KeyCode::Char('m') => app.open_calendar_view(),
        KeyCode::Char('w') => app.toggle_week_strip(),
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
    use crate::state::{ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, ViewMode};
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
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
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
            focused_section: PageSection::Explorer,
            zoomed_section: None,
            expanded_sections: Default::default(),
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
            navigation_history: Vec::new(),
        }
    }

    #[test]
    fn tab_moves_panel_focus() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Hero;

        dispatch_key(&mut app, KeyCode::Tab, KeyModifiers::NONE);

        assert_eq!(app.focused_section, PageSection::Recommendations);
    }

    #[test]
    fn enter_toggles_focused_section() {
        let mut app = sample_app_state();
        app.focus_section(PageSection::TraditionalEvidence);

        dispatch_key(&mut app, KeyCode::Enter, KeyModifiers::NONE);
        assert!(app.is_section_expanded(PageSection::TraditionalEvidence));

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
    fn key_shift_h_and_shift_l_navigate_week() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Hero;
        let start = app.date;

        dispatch_key(&mut app, KeyCode::Char('L'), KeyModifiers::SHIFT);
        assert_eq!(app.date, start + chrono::Duration::days(7));

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
    fn char_z_toggles_zoom_for_current_section() {
        let mut app = sample_app_state();
        app.focus_section(PageSection::Risks);

        dispatch_key(&mut app, KeyCode::Char('z'), KeyModifiers::NONE);
        assert_eq!(app.zoomed_section, Some(PageSection::Risks));

        dispatch_key(&mut app, KeyCode::Char('z'), KeyModifiers::NONE);
        assert_eq!(app.zoomed_section, None);
    }

    #[test]
    fn char_a_expands_recommendation_section() {
        let mut app = sample_app_state();
        app.focus_section(PageSection::Risks);

        dispatch_key(&mut app, KeyCode::Char('a'), KeyModifiers::NONE);

        assert_eq!(app.focused_section, PageSection::Recommendations);
        assert!(app.is_section_expanded(PageSection::Recommendations));
    }

    #[test]
    fn explorer_space_toggles_pack_without_applying() {
        let mut app = sample_app_state();
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
        app.staged_selection.event_kind = Some("travel".to_string());
        app.staged_selection.enabled_pack_ids = vec!["pack.nhi_thap_bat_tu.v1".to_string()];
        app.explorer_focus = ExplorerField::Actions;
        app.explorer_action = ExplorerAction::Reset;

        dispatch_key(&mut app, KeyCode::Enter, KeyModifiers::NONE);

        assert_eq!(app.staged_selection.event_kind, None);
        assert!(app.staged_selection.enabled_pack_ids.is_empty());
    }

    #[test]
    fn retry_shortcut_reinvokes_load_when_error_is_present() {
        let mut app = sample_app_state();
        app.error_msg = Some("boom".to_string());

        dispatch_key(&mut app, KeyCode::Char('r'), KeyModifiers::NONE);

        assert!(app.error_msg.is_none());
    }
}
