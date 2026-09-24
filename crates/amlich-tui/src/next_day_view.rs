//! Replacement TUI surface (`amlich-b14l.7` S1, `amlich-b14l.8` S2):
//! the anonymous Today anchor rendered from the Day View Model, plus the
//! Navigate & Hours loop — `[`/`]`/`t`/`/` date navigation with search,
//! and the `h` Hours drill-down where a Selected Hour updates
//! time-dependent context without moving the date anchor. Legacy surfaces
//! are untouched; this entry is opt-in via `amlich tui --next`.
//!
//! Keyboard map follows the `amlich-b14l.4` resolution: global keys are
//! `[`/`]` previous/next date, `t` today, `/` date search, `?` help,
//! Esc back, `q` quit; within a view `j`/`k` move selection and Enter
//! selects. The footer shows only context-valid keys.

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
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};

use amlich_api::{get_day_view_for_date, DayViewDto, DayViewHourDetailDto, DayViewPatternItemDto};

use crate::theme::Theme;

struct TerminalCleanupGuard;

impl Drop for TerminalCleanupGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, Show);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mode {
    Today,
    Hours,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Overlay {
    None,
    Search,
    Help,
}

struct App {
    selected_date: NaiveDate,
    /// The Selected Hour (`CONTEXT.md`): updates time-dependent context,
    /// never moves the date anchor, survives navigation and view switches.
    selected_hour: Option<usize>,
    mode: Mode,
    overlay: Overlay,
    search_input: String,
    search_error: Option<String>,
    /// Browsing cursor inside the Hours drill-down.
    hour_cursor: usize,
}

impl App {
    fn new(initial_date: NaiveDate) -> Self {
        App {
            selected_date: initial_date,
            selected_hour: None,
            mode: Mode::Today,
            overlay: Overlay::None,
            search_input: String::new(),
            search_error: None,
            hour_cursor: 0,
        }
    }
}

pub fn run_next_day_view(initial_date: Option<NaiveDate>) -> Result<(), String> {
    enable_raw_mode().map_err(|e| e.to_string())?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|e| e.to_string())?;
    let _cleanup_guard = TerminalCleanupGuard;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|e| e.to_string())?;

    let mut app = App::new(initial_date.unwrap_or_else(|| Local::now().date_naive()));
    loop {
        // In the Hours drill-down the browsing cursor drives the hour
        // detail directly; the committed selection follows it.
        if app.mode == Mode::Hours {
            app.selected_hour = Some(app.hour_cursor);
        }
        let view = load_view(app.selected_date, app.selected_hour)?;
        terminal
            .draw(|frame| draw(frame, &app, &view))
            .map_err(|e| e.to_string())?;
        if let Event::Key(key) = read().map_err(|e| e.to_string())? {
            if handle_key(&mut app, key.code, key.modifiers) {
                break;
            }
        }
    }
    Ok(())
}

/// Global quit signal: true means exit.
fn handle_key(app: &mut App, code: KeyCode, modifiers: KeyModifiers) -> bool {
    if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
        return true;
    }
    if app.overlay != Overlay::None {
        match (app.overlay, code) {
            (_, KeyCode::Esc) => app.overlay = Overlay::None,
            (Overlay::Search, KeyCode::Enter) => {
                if let Some(date) = parse_search(&app.search_input) {
                    app.selected_date = date;
                    app.overlay = Overlay::None;
                    app.search_error = None;
                } else {
                    app.search_error = Some(format!("không hiểu '{}'", app.search_input));
                }
            }
            (Overlay::Search, KeyCode::Backspace) => {
                app.search_input.pop();
            }
            (Overlay::Search, KeyCode::Char(c))
                if !modifiers.contains(KeyModifiers::CONTROL)
                    && !modifiers.contains(KeyModifiers::ALT) =>
            {
                app.search_input.push(c);
                app.search_error = None;
            }
            _ => {}
        }
        return false;
    }

    match code {
        KeyCode::Char('q') | KeyCode::Esc => {
            if app.mode == Mode::Hours {
                app.mode = Mode::Today;
            } else {
                return true;
            }
        }
        KeyCode::Left | KeyCode::Char('[') => {
            step_date(app, -1);
        }
        KeyCode::Right | KeyCode::Char(']') => {
            step_date(app, 1);
        }
        KeyCode::Char('t') | KeyCode::Home => {
            app.selected_date = Local::now().date_naive();
        }
        KeyCode::Char('/') => {
            app.overlay = Overlay::Search;
            app.search_input.clear();
            app.search_error = None;
        }
        KeyCode::Char('?') => {
            app.overlay = Overlay::Help;
        }
        KeyCode::Char('h') => {
            app.mode = Mode::Hours;
            if let Some(hour) = app.selected_hour {
                app.hour_cursor = hour;
            }
        }
        KeyCode::Up | KeyCode::Char('k') if app.mode == Mode::Hours => {
            app.hour_cursor = app.hour_cursor.saturating_sub(1);
        }
        KeyCode::Down | KeyCode::Char('j') if app.mode == Mode::Hours => {
            app.hour_cursor = (app.hour_cursor + 1).min(11);
        }
        // Enter selects: commits the cursor as the Selected Hour.
        KeyCode::Enter if app.mode == Mode::Hours => {
            app.selected_hour = Some(app.hour_cursor);
        }
        _ => {}
    }
    false
}

fn step_date(app: &mut App, delta: i64) {
    app.selected_date = if delta < 0 {
        app.selected_date.pred_opt().unwrap_or(app.selected_date)
    } else {
        app.selected_date.succ_opt().unwrap_or(app.selected_date)
    };
}

/// Search grammar (the legacy TUI convention, `amlich-b14l.4`): a solar
/// date as `YYYY-MM-DD` or `DD/MM/YYYY`. Two-digit years are rejected
/// rather than guessed (chrono's `%Y` would read `25` as year 25 AD).
fn parse_search(input: &str) -> Option<NaiveDate> {
    let trimmed = input.trim();
    if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        if date.year() >= 1000 {
            return Some(date);
        }
        return None;
    }
    if let Ok(date) = NaiveDate::parse_from_str(trimmed, "%d/%m/%Y") {
        if date.year() >= 1000 {
            return Some(date);
        }
        return None;
    }
    None
}

fn load_view(date: NaiveDate, selected_hour: Option<usize>) -> Result<DayViewDto, String> {
    let now = Local::now();
    let current_chi = ((now.hour() + 1) % 24) / 2;
    get_day_view_for_date(
        date.day() as i32,
        date.month() as i32,
        date.year(),
        Some(current_chi as usize),
        selected_hour,
    )
}

fn draw(frame: &mut Frame, app: &App, view: &DayViewDto) {
    match app.mode {
        Mode::Today => draw_today(frame, app, view),
        Mode::Hours => draw_hours_screen(frame, app, view),
    }
    match app.overlay {
        Overlay::Search => draw_search_overlay(frame, app),
        Overlay::Help => draw_help_overlay(frame, app),
        Overlay::None => {}
    }
}

// ---------------------------------------------------------------------
// Today view
// ---------------------------------------------------------------------

fn draw_today(frame: &mut Frame, app: &App, view: &DayViewDto) {
    let detail = view.hours.detail.as_ref();
    let detail_height = if detail.is_some() { 8 } else { 0 };
    let constraints = [
        Constraint::Length(5),
        Constraint::Min(10),
        Constraint::Length(3),
        Constraint::Length(detail_height),
        Constraint::Length(1),
    ];
    let chunks = Layout::vertical(constraints).split(frame.area());

    draw_header(frame, view, chunks[0]);
    draw_body(frame, view, chunks[1]);
    draw_hour_strip(frame, view, chunks[2]);
    if let Some(detail) = detail {
        draw_hour_detail(frame, app, view, detail, chunks[3]);
    }
    draw_footer(
        frame,
        chunks[4],
        " [/] ngày trước/sau · t hôm nay · / tìm ngày · h giờ · ? trợ giúp · q thoát ",
    );
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

fn draw_hour_strip(frame: &mut Frame, view: &DayViewDto, area: Rect) {
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
            let style = if Some(hour.hour_index) == view.hours.selected_hour_index {
                Style::default().add_modifier(Modifier::REVERSED)
            } else if hour.is_current {
                Theme::highlight()
            } else if hour.is_hoang_dao {
                Style::default().fg(Theme::GOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let marker = if Some(hour.hour_index) == view.hours.selected_hour_index {
                "+"
            } else if hour.is_current {
                "*"
            } else {
                " "
            };
            vec![
                Span::styled(format!("{marker}{} {}", hour.chi, hour.time_range), style),
                Span::raw("  "),
            ]
        })
        .collect();
    let paragraph = Paragraph::new(Line::from(spans)).alignment(Alignment::Left);
    frame.render_widget(paragraph, inner);
}

fn hour_detail_lines<'a>(
    detail: &'a DayViewHourDetailDto,
    current_hour_index: Option<usize>,
) -> Vec<Line<'a>> {
    let classification_style = if detail.is_hoang_dao {
        Style::default().fg(Theme::GOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    let mut lines = vec![Line::from(vec![
        Span::styled(
            format!(" Giờ {} · {} ", detail.chi, detail.time_range),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::styled(format!("{}  ", detail.classification), classification_style),
        Span::styled(
            format!("sao {}  ", detail.star),
            Style::default().fg(Theme::CYAN),
        ),
        Span::styled(
            if Some(detail.hour_index) == current_hour_index {
                "(đang diễn ra)"
            } else {
                ""
            },
            Theme::highlight(),
        ),
    ])];
    for reason in &detail.reasons {
        lines.push(Line::from(Span::styled(
            format!("  – {reason}"),
            Style::default().fg(Color::Gray),
        )));
    }
    lines
}

fn draw_hour_detail(
    frame: &mut Frame,
    _app: &App,
    view: &DayViewDto,
    detail: &DayViewHourDetailDto,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::secondary_border())
        .title(" Giờ đang chọn (ngày không đổi) ");
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let paragraph = Paragraph::new(hour_detail_lines(detail, view.hours.current_hour_index))
        .wrap(Wrap { trim: true });
    frame.render_widget(paragraph, inner);
}

fn draw_footer(frame: &mut Frame, area: Rect, text: &str) {
    let line = Line::from(Span::styled(text, Style::default().fg(Color::DarkGray)));
    let paragraph = Paragraph::new(line).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

// ---------------------------------------------------------------------
// Hours drill-down (b14l.4: `h` opens the full-screen twelve-window list)
// ---------------------------------------------------------------------

fn draw_hours_screen(frame: &mut Frame, app: &App, view: &DayViewDto) {
    let area = frame.area();
    let rows = Layout::vertical([Constraint::Min(0), Constraint::Length(1)]).split(area);
    let content = rows[0];
    let footer = rows[1];
    // Responsive density: wide terminals stack list and detail; narrow
    // terminals alternate (list with the detail summary as trailing rows).
    let wide = content.width >= 80;
    let chunks = if wide {
        Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(content)
            .to_vec()
    } else {
        Layout::vertical([Constraint::Min(12), Constraint::Length(8)])
            .split(content)
            .to_vec()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::primary_border())
        .title(" Giờ trong ngày · mười hai cửa sổ, nổi bật ≠ phù hợp nhất ");
    let inner = block.inner(chunks[0]);
    frame.render_widget(block, chunks[0]);

    let mut lines = Vec::new();
    for hour in &view.hours.hours {
        let cursor = hour.hour_index == app.hour_cursor;
        let current = Some(hour.hour_index) == view.hours.current_hour_index;
        let style = if cursor {
            Theme::highlight()
        } else if hour.is_hoang_dao {
            Style::default().fg(Theme::GOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };
        let marker = if cursor {
            if current {
                "▶*"
            } else {
                "▶ "
            }
        } else if current {
            "  *"
        } else {
            "  "
        };
        let notable = if hour.is_notable { " nổi bật" } else { "" };
        lines.push(Line::from(vec![
            Span::styled(format!("{marker}{} {}", hour.chi, hour.time_range), style),
            Span::raw("  "),
            Span::styled(
                if hour.is_hoang_dao {
                    "Hoàng Đạo"
                } else {
                    "Hắc Đạo"
                },
                style,
            ),
            Span::raw("  "),
            Span::styled(format!("sao {}", hour.star), style),
            Span::styled(notable.to_string(), Style::default().fg(Color::DarkGray)),
        ]));
    }
    frame.render_widget(Paragraph::new(lines), inner);

    // Detail pane: the Selected Hour's projection (classification, ruling
    // star, hour-context reasons) — never a ranking.
    let detail_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Theme::secondary_border())
        .title(" Chi tiết giờ đang chọn (ngày không đổi) ");
    let detail_inner = detail_block.inner(chunks[1]);
    frame.render_widget(detail_block, chunks[1]);
    let detail_lines = match &view.hours.detail {
        Some(detail) => hour_detail_lines(detail, view.hours.current_hour_index),
        None => vec![Line::from(Span::styled(
            " j/k chọn giờ · Enter chốt giờ đang chọn ",
            Style::default().fg(Color::DarkGray),
        ))],
    };
    frame.render_widget(
        Paragraph::new(detail_lines).wrap(Wrap { trim: true }),
        detail_inner,
    );

    draw_footer(
        frame,
        footer,
        " j/k di chuyển · Enter chốt chọn · Esc về Today · [/] đổi ngày · q thoát ",
    );
}

// ---------------------------------------------------------------------
// Overlays
// ---------------------------------------------------------------------

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}

fn draw_search_overlay(frame: &mut Frame, app: &App) {
    let area = centered_rect(frame.area(), 44, 6);
    frame.render_widget(Clear, area);
    let input = format!("> {}█", app.search_input);
    let mut lines = vec![
        Line::from(" Đến ngày (YYYY-MM-DD hoặc DD/MM/YYYY):"),
        Line::from(Span::styled(input, Style::default().fg(Color::Yellow))),
    ];
    if let Some(error) = &app.search_error {
        lines.push(Line::from(Span::styled(
            format!(" {error}"),
            Style::default().fg(Theme::RED),
        )));
    }
    let block = Block::default()
        .title(" Tìm ngày ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Theme::CYAN));
    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

fn draw_help_overlay(frame: &mut Frame, _app: &App) {
    let area = centered_rect(frame.area(), 52, 12);
    frame.render_widget(Clear, area);
    let lines = vec![
        Line::from(Span::styled(
            " Phím tắt · Đài quan sát ngày ",
            Theme::title_style(),
        )),
        Line::from(" [/]  ngày trước / ngày sau"),
        Line::from(" t    về hôm nay"),
        Line::from(" /    đến ngày (tìm kiếm)"),
        Line::from(" h    mở danh sách Giờ trong ngày"),
        Line::from(" j/k  di chuyển chọn giờ"),
        Line::from(" Ent  chốt giờ đang chọn (ngày không đổi)"),
        Line::from(" Esc  quay lại / đóng"),
        Line::from(" ?    trợ giúp · q thoát"),
        Line::from(""),
        Line::from(Span::styled(
            " * cửa sổ đang diễn ra · + giờ đang chọn",
            Style::default().fg(Color::DarkGray),
        )),
    ];
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Theme::CYAN));
    let paragraph = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::get_day_view_for_date;
    use chrono::NaiveDate;
    use ratatui::{backend::TestBackend, Terminal as TestTerminal};

    fn rendered(app: &App, view: &DayViewDto, width: u16, height: u16) -> String {
        let backend = TestBackend::new(width, height);
        let mut terminal = TestTerminal::new(backend).expect("test terminal");
        terminal.draw(|frame| draw(frame, app, view)).expect("draw");
        terminal
            .backend()
            .buffer()
            .content
            .iter()
            .map(|cell| cell.symbol().to_string())
            .collect()
    }

    #[test]
    fn renders_the_anonymous_today_anchor_from_the_day_view_model() {
        let view = get_day_view_for_date(15, 6, 2024, Some(5), None).expect("day view");
        let app = App::new(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap());
        let content = rendered(&app, &view, 120, 30);

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
        assert!(content.contains("tìm ngày"));
    }

    #[test]
    fn today_view_renders_the_selected_hour_detail_without_moving_the_anchor() {
        let view = get_day_view_for_date(25, 7, 2025, Some(5), Some(9)).expect("day view");
        let app = App::new(NaiveDate::from_ymd_opt(2025, 7, 25).unwrap());
        let content = rendered(&app, &view, 120, 34);

        assert!(content.contains("2025-07-25"));
        assert!(content.contains("Giờ đang chọn"));
        assert!(content.contains("Dậu"));
        assert!(content.contains("17:00-19:00"));
        assert!(content.contains("Hắc Đạo"));
        assert!(content.contains("Câu Trận"));
        // Hour-context reasons ride the same surface.
        assert!(content.contains("Chọn giờ chỉ cập nhật bối cảnh"));
        // The leap-month label stays while the hour is selected.
        assert!(content.contains("nhuận"));
    }

    #[test]
    fn hours_drill_down_lists_twelve_windows_with_classification_and_star() {
        let view = get_day_view_for_date(25, 7, 2025, Some(5), Some(0)).expect("day view");
        let mut app = App::new(NaiveDate::from_ymd_opt(2025, 7, 25).unwrap());
        app.mode = Mode::Hours;
        app.hour_cursor = 0;
        let content = rendered(&app, &view, 120, 30);

        assert!(content.contains("mười hai cửa sổ"));
        for hour in &view.hours.hours {
            assert!(
                content.contains(&hour.chi),
                "hour {} missing from the list",
                hour.chi
            );
        }
        assert!(content.contains("Hoàng Đạo"));
        assert!(content.contains("Hắc Đạo"));
        assert!(content.contains("sao "));
        assert!(content.contains("Chi tiết giờ đang chọn"));
        assert!(content.contains("Esc về Today"));
    }

    #[test]
    fn hours_drill_down_renders_in_narrow_density() {
        let view = get_day_view_for_date(25, 7, 2025, Some(5), Some(0)).expect("day view");
        let mut app = App::new(NaiveDate::from_ymd_opt(2025, 7, 25).unwrap());
        app.mode = Mode::Hours;
        let content = rendered(&app, &view, 52, 30);

        assert!(content.contains("Giờ trong ngày"));
        assert!(content.contains("Chi tiết giờ đang chọn"));
    }

    #[test]
    fn search_overlay_renders_input_and_errors() {
        let view = get_day_view_for_date(15, 6, 2024, None, None).expect("day view");
        let mut app = App::new(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap());
        app.overlay = Overlay::Search;
        app.search_input = "31/02/2024".to_string();
        app.search_error = Some("không hiểu '31/02/2024'".to_string());
        let content = rendered(&app, &view, 80, 24);

        assert!(content.contains("Tìm ngày"));
        assert!(content.contains("31/02/2024"));
        assert!(content.contains("không hiểu"));
    }

    #[test]
    fn search_parses_both_accepted_formats_and_rejects_the_rest() {
        assert_eq!(
            parse_search("2025-07-25"),
            NaiveDate::from_ymd_opt(2025, 7, 25)
        );
        assert_eq!(
            parse_search("25/07/2025"),
            NaiveDate::from_ymd_opt(2025, 7, 25)
        );
        assert_eq!(
            parse_search(" 2025-07-25 "),
            NaiveDate::from_ymd_opt(2025, 7, 25)
        );
        assert_eq!(parse_search("25/07/25"), None);
        assert_eq!(parse_search("mai"), None);
        assert_eq!(parse_search(""), None);
    }

    #[test]
    fn keymap_follows_the_b14l4_resolution() {
        let mut app = App::new(NaiveDate::from_ymd_opt(2025, 7, 25).unwrap());

        handle_key(&mut app, KeyCode::Char('['), KeyModifiers::NONE);
        assert_eq!(
            app.selected_date,
            NaiveDate::from_ymd_opt(2025, 7, 24).unwrap()
        );
        handle_key(&mut app, KeyCode::Char(']'), KeyModifiers::NONE);
        assert_eq!(
            app.selected_date,
            NaiveDate::from_ymd_opt(2025, 7, 25).unwrap()
        );
        handle_key(&mut app, KeyCode::Left, KeyModifiers::NONE);
        assert_eq!(
            app.selected_date,
            NaiveDate::from_ymd_opt(2025, 7, 24).unwrap()
        );
        handle_key(&mut app, KeyCode::Right, KeyModifiers::NONE);

        handle_key(&mut app, KeyCode::Char('t'), KeyModifiers::NONE);
        assert_eq!(app.selected_date, Local::now().date_naive());

        handle_key(&mut app, KeyCode::Char('/'), KeyModifiers::NONE);
        assert_eq!(app.overlay, Overlay::Search);
        for c in "2024-02-10".chars() {
            handle_key(&mut app, KeyCode::Char(c), KeyModifiers::NONE);
        }
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(app.overlay, Overlay::None);
        assert_eq!(
            app.selected_date,
            NaiveDate::from_ymd_opt(2024, 2, 10).unwrap()
        );

        handle_key(&mut app, KeyCode::Char('h'), KeyModifiers::NONE);
        assert_eq!(app.mode, Mode::Hours);
        handle_key(&mut app, KeyCode::Down, KeyModifiers::NONE);
        handle_key(&mut app, KeyCode::Char('j'), KeyModifiers::NONE);
        assert_eq!(app.hour_cursor, 2);
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(app.selected_hour, Some(2));
        // The selection survives leaving the drill-down.
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE);
        assert_eq!(app.mode, Mode::Today);
        assert_eq!(app.selected_hour, Some(2));

        assert!(handle_key(&mut app, KeyCode::Char('q'), KeyModifiers::NONE));
        assert!(handle_key(
            &mut App::new(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
            KeyCode::Char('c'),
            KeyModifiers::CONTROL
        ));
    }

    #[test]
    fn search_rejects_unparseable_input_without_leaving_the_overlay() {
        let mut app = App::new(NaiveDate::from_ymd_opt(2025, 7, 25).unwrap());
        handle_key(&mut app, KeyCode::Char('/'), KeyModifiers::NONE);
        for c in "không phải ngày".chars() {
            handle_key(&mut app, KeyCode::Char(c), KeyModifiers::NONE);
        }
        handle_key(&mut app, KeyCode::Enter, KeyModifiers::NONE);
        assert_eq!(app.overlay, Overlay::Search);
        assert!(app.search_error.is_some());
        handle_key(&mut app, KeyCode::Esc, KeyModifiers::NONE);
        assert_eq!(app.overlay, Overlay::None);
        assert_eq!(
            app.selected_date,
            NaiveDate::from_ymd_opt(2025, 7, 25).unwrap()
        );
    }
}
