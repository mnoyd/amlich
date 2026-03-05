use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use std::io;

use crate::state::AppState;

pub fn handle_events(app: &mut AppState) -> Result<bool, Box<dyn std::error::Error>> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            // Global overrides
            if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
                return Ok(true); // quit
            }
            
            if app.show_search {
                match key.code {
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
                    KeyCode::Backspace => { app.search_input.pop(); }
                    _ => {}
                }
                return Ok(false);
            }

            // Normal key handling
            match key.code {
                // Quit
                KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                // Ctrl+C is handled as a global override above

                // Daily Navigation
                KeyCode::Right | KeyCode::Char('l') => app.next_day(),
                KeyCode::Left | KeyCode::Char('h') => app.prev_day(),
                KeyCode::Char('t') => app.go_today(),

                // Scrolling
                KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
                KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
                KeyCode::PageDown => app.scroll_down_by(10), // Arbitrary page size for now
                KeyCode::PageUp => app.scroll_up_by(10),

                // Lenses
                KeyCode::Tab => app.next_lens(),

                // Interactives
                KeyCode::Enter => app.toggle_tietkhi(),

                // Modals / Overlays
                KeyCode::Char(' ') => app.toggle_calendar(),
                KeyCode::Char('/') | KeyCode::Char('s') => app.toggle_search(),
                
                // (Other keys like 'g' for date jump, '/' for search to be added later)
                _ => {}
            }
        }
    }

    Ok(false)
}
