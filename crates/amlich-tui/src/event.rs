use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use crate::app::App;

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
            let index = c.to_digit(10).unwrap() as usize - 1;
            app.go_to_bookmark(index);
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
        KeyCode::Tab => {
            app.search_next_result();
            true
        }
        KeyCode::BackTab => {
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

        // Toggle insight panel and language
        KeyCode::Char('i') => app.toggle_insight(),
        KeyCode::Char('L') => app.toggle_insight_lang(),

        // Bookmarks
        KeyCode::Char('b') => app.toggle_bookmark(),
        KeyCode::Char('B') => app.toggle_bookmarks(),

        // Date jump
        KeyCode::Char('g') => app.toggle_date_jump(),

        // Search
        KeyCode::Char('/') => app.toggle_search(),

        // History navigation
        KeyCode::Char('[') => {
            app.history_back();
        }
        KeyCode::Char(']') => {
            app.history_forward();
        }

        _ => {}
    }
}
