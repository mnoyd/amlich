//! A deliberately boring terminal UI: it writes ANSI text and never changes
//! terminal mode.  This keeps `amlich` useful over serial consoles and SSH.
mod render;
mod screens;
mod state;

use std::io::{self, Write};

use chrono::NaiveDate;
use crossterm::event::{read, Event, KeyCode};

use self::{screens::Screen, state::AppState};

pub fn run(initial_date: Option<NaiveDate>) -> Result<(), String> {
    let mut app = AppState::new(initial_date)?;
    loop {
        print!("\x1b[2J\x1b[H{}", screens::render(&app));
        io::stdout().flush().map_err(|error| error.to_string())?;
        let Event::Key(key) = read().map_err(|error| error.to_string())? else {
            continue;
        };
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => break,
            KeyCode::Tab | KeyCode::Right => app.screen = app.screen.next(),
            KeyCode::BackTab | KeyCode::Left => app.screen = app.screen.previous(),
            KeyCode::Char('?') => app.screen = Screen::Help,
            KeyCode::Char('1') => app.screen = Screen::Today,
            KeyCode::Char('2') => app.screen = Screen::Personal,
            KeyCode::Char('3') => app.screen = Screen::Hours,
            KeyCode::Char('4') => app.screen = Screen::EventDetail,
            KeyCode::Char('5') => app.screen = Screen::Elements,
            KeyCode::Char('6') => app.screen = Screen::FengShui,
            KeyCode::Char('7') => app.screen = Screen::Insight,
            KeyCode::Char('8') => app.screen = Screen::GraphInspector,
            KeyCode::Char('j') | KeyCode::Down => app.scroll = app.scroll.saturating_add(1),
            KeyCode::Char('k') | KeyCode::Up => app.scroll = app.scroll.saturating_sub(1),
            KeyCode::Char('h') => app.shift_day(-1)?,
            KeyCode::Char('l') => app.shift_day(1)?,
            _ => {}
        }
    }
    Ok(())
}
