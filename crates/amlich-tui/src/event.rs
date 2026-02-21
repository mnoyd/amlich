use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use crate::app::{App, InsightTab};

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
            KeyCode::Char('n') | KeyCode::Tab => app.next_insight_tab(),
            KeyCode::Char('L') => app.toggle_insight_lang(),
            _ => {}
        }
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

        _ => {}
    }
}
