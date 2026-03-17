pub mod events;
pub mod layout;
pub mod state;
pub mod theme;
pub mod widgets;

use chrono::NaiveDate;
use std::io;

use crossterm::{
    cursor::Show,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::events::handle_events;
use crate::layout::draw;
use crate::state::AppState;

struct TerminalCleanupGuard;

impl Drop for TerminalCleanupGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
    }
}

pub fn run_tui(initial_date: Option<NaiveDate>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let _cleanup_guard = TerminalCleanupGuard;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = AppState::new(initial_date);
    let result = run(&mut terminal, app);

    let _ = terminal.show_cursor();

    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: AppState) -> io::Result<()> {
    while app.running {
        terminal.draw(|frame| draw(frame, &app))?;
        match handle_events(&mut app) {
            Ok(true) => {
                app.running = false; // Ctrl+C
            }
            Ok(false) => {}
            Err(e) => {
                return Err(io::Error::other(e.to_string()));
            }
        }
    }

    Ok(())
}
