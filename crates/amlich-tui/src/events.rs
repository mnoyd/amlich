use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::state::{AppState, PageSection};

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
                    app.date = d;
                    app.load_data();
                } else if let Ok(d) = chrono::NaiveDate::parse_from_str(&q, "%d/%m/%Y") {
                    app.date = d;
                    app.load_data();
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
            KeyCode::Esc | KeyCode::Char(' ') | KeyCode::Char('c') => app.close_calendar_view(),
            KeyCode::Enter => app.apply_calendar_selection(),
            KeyCode::Right | KeyCode::Char('l') => app.calendar_move_days(1),
            KeyCode::Left | KeyCode::Char('h') => app.calendar_move_days(-1),
            KeyCode::Down | KeyCode::Char('j') => app.calendar_move_days(7),
            KeyCode::Up | KeyCode::Char('k') => app.calendar_move_days(-7),
            KeyCode::PageDown | KeyCode::Char('n') => app.calendar_next_month(),
            KeyCode::PageUp | KeyCode::Char('p') => app.calendar_prev_month(),
            KeyCode::Char('t') => app.calendar_go_today(),
            _ => {}
        }
        return false;
    }

    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
        KeyCode::Right | KeyCode::Char('l') => app.next_day(),
        KeyCode::Left | KeyCode::Char('h') => app.prev_day(),
        KeyCode::Char('t') => app.go_today(),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
        KeyCode::PageDown => app.scroll_down_by(10),
        KeyCode::PageUp => app.scroll_up_by(10),
        KeyCode::Tab => app.focus_next_section(),
        KeyCode::Enter => app.toggle_expand_focused_section(),
        KeyCode::Char('e') => app.toggle_evidence(),
        KeyCode::Char('z') => app.toggle_zoom_for_focused_section(),
        KeyCode::Char('a') => app.expand_section(PageSection::Recommendations),
        KeyCode::Char(' ') | KeyCode::Char('c') => app.toggle_calendar_view(),
        KeyCode::Char('/') | KeyCode::Char('s') => app.toggle_search(),
        _ => {}
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use crate::state::{FocusLens, ViewMode};

    fn sample_app_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: None,
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
        }
    }

    #[test]
    fn tab_moves_panel_focus() {
        let mut app = sample_app_state();

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

        dispatch_key(&mut app, KeyCode::Char('e'), KeyModifiers::NONE);
        assert!(app.show_evidence);

        dispatch_key(&mut app, KeyCode::Char('e'), KeyModifiers::NONE);
        assert!(!app.show_evidence);
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
}
