use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use crate::app::{AlmanacTab, App, InsightTab};

// Bookmarks overlay mode handling
fn handle_bookmarks_mode(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('b') | KeyCode::Char('B') | KeyCode::Esc | KeyCode::Char('q') => {
            app.toggle_bookmarks();
            true
        }
        KeyCode::Char('j') | KeyCode::Down => {
            app.bookmark_scroll = app.bookmark_scroll.saturating_add(1);
            true
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.bookmark_scroll = app.bookmark_scroll.saturating_sub(1);
            true
        }
        KeyCode::Char(c @ '1'..='9') => {
            if let Some(index) = c.to_digit(10).map(|n| n as usize - 1) {
                app.go_to_bookmark(index);
            }
            true
        }
        _ => false,
    }
}

// Date jump input mode handling
fn handle_date_jump_mode(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.toggle_date_jump();
            true
        }
        KeyCode::Enter => {
            app.date_jump_submit();
            true
        }
        KeyCode::Backspace => {
            app.date_jump_backspace();
            true
        }
        KeyCode::Char(c) => {
            app.date_jump_char(c);
            true
        }
        _ => false,
    }
}

// Search input mode handling
fn handle_search_mode(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => {
            app.toggle_search();
            true
        }
        KeyCode::Tab | KeyCode::Down => {
            app.search_next_result();
            true
        }
        KeyCode::BackTab | KeyCode::Up => {
            app.search_prev_result();
            true
        }
        KeyCode::Backspace => {
            app.search_backspace();
            true
        }
        KeyCode::Char(c) => {
            app.search_char(c);
            true
        }
        _ => false,
    }
}

pub fn handle_events(app: &mut App) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != crossterm::event::KeyEventKind::Press {
                return Ok(());
            }
            handle_key(app, key);
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    // Ctrl+C always quits
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.running = false;
        return;
    }

    // Insight overlay mode
    if app.show_insight {
        match key.code {
            KeyCode::Char('i') | KeyCode::Esc | KeyCode::Char('q') => app.toggle_insight(),
            KeyCode::Char('j') | KeyCode::Down => {
                app.insight_scroll = app.insight_scroll.saturating_add(1)
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.insight_scroll = app.insight_scroll.saturating_sub(1)
            }
            KeyCode::Char('1') => app.set_insight_tab(InsightTab::Festival),
            KeyCode::Char('2') => app.set_insight_tab(InsightTab::Guidance),
            KeyCode::Char('3') => app.set_insight_tab(InsightTab::TietKhi),
            KeyCode::Char('n') => app.next_insight_tab(),
            KeyCode::Char('L') => app.toggle_insight_lang(),
            _ => {}
        }
        return;
    }

    // Almanac overlay mode
    if app.show_almanac {
        match key.code {
            KeyCode::Char('a') | KeyCode::Esc | KeyCode::Char('q') => app.toggle_almanac(),
            KeyCode::Char('j') | KeyCode::Down => {
                app.almanac_scroll = app.almanac_scroll.saturating_add(1)
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.almanac_scroll = app.almanac_scroll.saturating_sub(1)
            }
            KeyCode::Char('1') => app.set_almanac_tab(AlmanacTab::Overview),
            KeyCode::Char('2') => app.set_almanac_tab(AlmanacTab::Taboos),
            KeyCode::Char('3') => app.set_almanac_tab(AlmanacTab::Stars),
            KeyCode::Char('4') => app.set_almanac_tab(AlmanacTab::Evidence),
            KeyCode::Tab => app.next_almanac_tab(),
            KeyCode::BackTab => app.prev_almanac_tab(),
            _ => {}
        }
        return;
    }

    // Help overlay (global - accessible from anywhere)
    if app.show_help {
        match key.code {
            KeyCode::Char('?') | KeyCode::Esc | KeyCode::Char('q') => app.toggle_help(),
            _ => {}
        }
        return;
    }

    // Toggle help from anywhere
    if key.code == KeyCode::Char('?') {
        app.toggle_help();
        return;
    }

    // Date jump input mode
    if app.show_date_jump {
        handle_date_jump_mode(app, key);
        return;
    }

    // Search input mode
    if app.show_search {
        handle_search_mode(app, key);
        return;
    }

    // Bookmarks overlay mode
    if app.show_bookmarks {
        handle_bookmarks_mode(app, key);
        return;
    }

    // Holiday overlay mode
    if app.show_holidays {
        match key.code {
            KeyCode::Char('h') | KeyCode::Esc | KeyCode::Char('q') => app.toggle_holidays(),
            KeyCode::Char('j') | KeyCode::Down => {
                app.holiday_scroll = app.holiday_scroll.saturating_add(1)
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.holiday_scroll = app.holiday_scroll.saturating_sub(1)
            }
            _ => {}
        }
        return;
    }

    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => app.running = false,

        // Day navigation (vim + arrows)
        KeyCode::Char('l') | KeyCode::Right => app.next_day(),
        KeyCode::Char('h') | KeyCode::Left => app.prev_day(),
        KeyCode::Char('j') | KeyCode::Down => app.next_week(),
        KeyCode::Char('k') | KeyCode::Up => app.prev_week(),

        // Month navigation
        KeyCode::Char('n') | KeyCode::Tab => app.next_month(),
        KeyCode::Char('p') | KeyCode::BackTab => app.prev_month(),

        // Year navigation
        KeyCode::Char('N') | KeyCode::PageDown => app.next_year(),
        KeyCode::Char('P') | KeyCode::PageUp => app.prev_year(),

        // Jump to today
        KeyCode::Char('t') => app.go_today(),

        // Toggle holiday list
        KeyCode::Char('H') => app.toggle_holidays(),

        // Toggle insight panel
        KeyCode::Char('i') => app.toggle_insight(),

        // Toggle almanac panel
        KeyCode::Char('a') => app.toggle_almanac(),

        // Bookmarks
        KeyCode::Char('b') => app.toggle_bookmark(),
        KeyCode::Char('B') => app.toggle_bookmarks(),

        // Date jump
        KeyCode::Char('g') => app.toggle_date_jump(),

        // Search
        KeyCode::Char('/') => app.toggle_search(),

        // Toggle calendar view (for small screens)
        KeyCode::Char('c') => app.toggle_calendar(),

        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    use crate::app::{AlmanacTab, App};

    use super::handle_key;

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn a_toggles_almanac_overlay() {
        let mut app = App::new_with_date(None);
        assert!(!app.show_almanac);

        handle_key(&mut app, key(KeyCode::Char('a')));
        assert!(app.show_almanac);

        handle_key(&mut app, key(KeyCode::Char('a')));
        assert!(!app.show_almanac);
    }

    #[test]
    fn almanac_mode_tab_and_scroll_controls_work() {
        let mut app = App::new_with_date(None);
        app.toggle_almanac();

        handle_key(&mut app, key(KeyCode::Char('2')));
        assert_eq!(app.almanac_tab, AlmanacTab::Taboos);

        handle_key(&mut app, key(KeyCode::Down));
        assert_eq!(app.almanac_scroll, 1);

        handle_key(&mut app, key(KeyCode::Tab));
        assert_eq!(app.almanac_tab, AlmanacTab::Stars);

        handle_key(&mut app, key(KeyCode::BackTab));
        assert_eq!(app.almanac_tab, AlmanacTab::Taboos);

        handle_key(&mut app, key(KeyCode::Esc));
        assert!(!app.show_almanac);
    }
}
