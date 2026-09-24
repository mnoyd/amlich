//! Replacement TUI surface, Slice 1 (`amlich-b14l.7`): the anonymous Today
//! anchor rendered from the Day View Model. Live data, keyboard-first, one
//! interaction state. Legacy surfaces are untouched; this entry is opt-in via
//! `amlich tui --next`.

use std::io::{self};

use chrono::{Datelike, Local, NaiveDate, Timelike};
use crossterm::{
    cursor::Show,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use amlich_api::{get_day_view_for_date, DayViewDto, DayViewPatternItemDto};

use crate::theme::Theme;

struct TerminalCleanupGuard;

impl Drop for TerminalCleanupGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
    }
}

pub fn run_next_day_view(initial_date: Option<NaiveDate>) -> Result<(), String> {
    enable_raw_mode().map_err(|e| e.to_string())?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|e| e.to_string())?;
    let _cleanup_guard = TerminalCleanupGuard;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| e.to_string())?;

    let mut selected = initial_date.unwrap_or_else(|| Local::now().date_naive());
    loop {
        let view = load_view(selected)?;
        terminal
            .draw(|frame| draw(frame, &view))
            .map_err(|e| e.to_string())?;
        if let Event::Key(key) = read().map_err(|e| e.to_string())? {
            match (key.code, key.modifiers) {
                (KeyCode::Char('q') | KeyCode::Esc, _)
                | (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                (KeyCode::Left | KeyCode::Char('h'), _) => {
                    selected = selected.pred_opt().unwrap_or(selected);
                }
                (KeyCode::Right | KeyCode::Char('l'), _) => {
                    selected = selected.succ_opt().unwrap_or(selected);
                }
                (KeyCode::Char('t') | KeyCode::Home, _) => {
                    selected = Local::now().date_naive();
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn load_view(date: NaiveDate) -> Result<DayViewDto, String> {
    let now = Local::now();
    let current_chi = ((now.hour() + 1) % 24) / 2;
    get_day_view_for_date(
        date.day() as i32,
        date.month() as i32,
        date.year(),
        Some(current_chi as usize),
    )
}

fn draw(frame: &mut Frame, view: &DayViewDto) {
    let chunks = Layout::vertical([
        Constraint::Length(5),
        Constraint::Min(10),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .split(frame.area());

    draw_header(frame, view, chunks[0]);
    draw_body(frame, view, chunks[1]);
    draw_hours(frame, view, chunks[2]);
    draw_footer(frame, chunks[3]);
}

fn draw_header(frame: &mut Frame, view: &DayViewDto, area: Rect) {
    let leap = if view.lunar.is_leap_month {
        " (nhuận)"
    } else {
        ""
    };
    let header = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::primary_border());
    frame.render_widget(header, area);

    let inner = Block::default().borders(Borders::ALL).inner(area);
    let lines = vec![
        Line::from(vec![
            Span::styled("ÂM LỊCH · Đài quan sát ngày", Theme::title_style()),
            Span::raw("   "),
            Span::styled("day-view ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                view.schema_version.clone(),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![Span::styled(
            format!(
                "{}, {}",
                view.solar.day_of_week_name, view.solar.date_string
            ),
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw(format!(
                "Âm lịch {}{} · ngày {} {} · tháng {} {} · năm {} {}",
                view.lunar.date_string,
                leap,
                view.canchi.day.can,
                view.canchi.day.chi,
                view.canchi.month.can,
                view.canchi.month.chi,
                view.canchi.year.can,
                view.canchi.year.chi
            )),
            Span::raw("  ·  "),
            Span::raw(format!(
                "Tiết {} ({})",
                view.tiet_khi.name, view.tiet_khi.season
            )),
        ]),
    ];
    let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
    frame.render_widget(paragraph, inner);
}

fn pattern_spans<'a>(items: &'a [DayViewPatternItemDto], color: Color) -> Vec<Line<'a>> {
    let mut lines = Vec::new();
    for item in items {
        lines.push(Line::from(vec![
            Span::styled(format!(" {} ", item.title), Style::default().fg(color)),
            Span::styled(
                format!("— {}", item.reason),
                Style::default().fg(Color::Gray),
            ),
        ]));
    }
    lines
}

fn draw_body(frame: &mut Frame, view: &DayViewDto, area: Rect) {
    let columns = Layout::horizontal([
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
        Constraint::Ratio(1, 3),
    ])
    .split(area);

    let groups: [(&str, &Vec<DayViewPatternItemDto>, Color); 3] = [
        ("Hỗ trợ", &view.pattern.supports, Theme::GREEN),
        ("Giới hạn", &view.pattern.constraints, Theme::RED),
        ("Chưa biết", &view.pattern.unknowns, Theme::CYAN),
    ];
    for (index, (title, items, color)) in groups.iter().enumerate() {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::secondary_border())
            .title(format!(" {title} ({}) ", items.len()));
        let inner = block.inner(columns[index]);
        frame.render_widget(block, columns[index]);
        let paragraph = Paragraph::new(pattern_spans(items, *color));
        frame.render_widget(paragraph, inner);
    }
}

fn draw_hours(frame: &mut Frame, view: &DayViewDto, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::secondary_border())
        .title(" Giờ trong ngày (nổi bật ≠ phù hợp nhất) ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    let spans: Vec<Span> = view
        .hours
        .hours
        .iter()
        .flat_map(|hour| {
            let style = if hour.is_current {
                Theme::highlight()
            } else if hour.is_hoang_dao {
                Style::default().fg(Theme::GOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let marker = if hour.is_current { "*" } else { " " };
            vec![
                Span::styled(format!("{marker}{} {}", hour.chi, hour.time_range), style),
                Span::raw("  "),
            ]
        })
        .collect();
    let paragraph = Paragraph::new(Line::from(spans)).alignment(Alignment::Left);
    frame.render_widget(paragraph, inner);
}

fn draw_footer(frame: &mut Frame, area: Rect) {
    let line = Line::from(Span::styled(
        " ←/h ngày trước · →/l ngày sau · t hôm nay · q thoát ",
        Style::default().fg(Color::DarkGray),
    ));
    let paragraph = Paragraph::new(line).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::get_day_view_for_date;
    use ratatui::{backend::TestBackend, Terminal as TestTerminal};

    #[test]
    fn renders_the_anonymous_today_anchor_from_the_day_view_model() {
        let view = get_day_view_for_date(15, 6, 2024, Some(5)).expect("day view");
        let backend = TestBackend::new(120, 30);
        let mut terminal = TestTerminal::new(backend).expect("test terminal");
        terminal.draw(|frame| draw(frame, &view)).expect("draw");

        let content: String = terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|cell| cell.symbol().to_string())
            .collect();

        assert!(content.contains("Đài quan sát ngày"));
        assert!(content.contains("2024-06-15"));
        assert!(content.contains("Hỗ trợ"));
        assert!(content.contains("Giới hạn"));
        assert!(content.contains("Chưa biết"));
        assert!(content.contains("Chưa có mục đích"));
        assert!(content.contains("Giờ trong ngày"));
        assert!(content.contains("*"));
        let current = view
            .hours
            .hours
            .iter()
            .find(|hour| hour.is_current)
            .expect("one current hour");
        assert!(content.contains(&current.chi));
    }
}
