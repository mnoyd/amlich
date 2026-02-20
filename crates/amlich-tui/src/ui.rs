use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::theme;
use crate::widgets::{
    calendar::CalendarWidget, detail::DetailWidget, holidays::HolidayOverlay, hours::HoursWidget,
};

const MONTH_NAMES: [&str; 12] = [
    "Tháng 1",
    "Tháng 2",
    "Tháng 3",
    "Tháng 4",
    "Tháng 5",
    "Tháng 6",
    "Tháng 7",
    "Tháng 8",
    "Tháng 9",
    "Tháng 10",
    "Tháng 11",
    "Tháng 12",
];

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Top-level: header (3) + body + footer (1)
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Min(10),
        Constraint::Length(1),
    ])
    .split(size);

    draw_header(frame, app, vertical[0]);
    draw_body(frame, app, vertical[1]);
    draw_footer(frame, vertical[2]);

    // Holiday overlay
    if app.show_holidays {
        frame.render_widget(HolidayOverlay::new(app), vertical[1]);
    }
}

fn draw_header(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let month_name = MONTH_NAMES[(app.view_month - 1) as usize];

    // Year Can Chi from lunar data
    let year_canchi = app
        .selected_info()
        .map(|i| format!(" ({})", i.canchi.year.full))
        .unwrap_or_default();

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            " 🌙 Âm Lịch ",
            Style::default()
                .fg(theme::HEADER_FG)
                .bg(theme::HEADER_BG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled("◄ ", Style::default().fg(theme::LABEL_FG)),
        Span::styled(
            format!("{} {} ", month_name, app.view_year),
            Style::default()
                .fg(theme::VALUE_FG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            &year_canchi,
            Style::default().fg(theme::ACCENT_FG),
        ),
        Span::styled(" ►", Style::default().fg(theme::LABEL_FG)),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style()),
    );

    frame.render_widget(header, area);
}

fn draw_body(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    // 3 columns: calendar (50%) | detail (25%) | hours (25%)
    let columns = Layout::horizontal([
        Constraint::Percentage(50),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ])
    .split(area);

    frame.render_widget(CalendarWidget::new(app), columns[0]);
    frame.render_widget(DetailWidget::new(app), columns[1]);
    frame.render_widget(HoursWidget::new(app), columns[2]);
}

fn draw_footer(frame: &mut Frame, area: ratatui::layout::Rect) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" ←↑↓→/hjkl ", Style::default().fg(theme::ACCENT_FG)),
        Span::styled("di chuyển", Style::default().fg(theme::LABEL_FG)),
        Span::raw("  "),
        Span::styled("n/p ", Style::default().fg(theme::ACCENT_FG)),
        Span::styled("tháng", Style::default().fg(theme::LABEL_FG)),
        Span::raw("  "),
        Span::styled("N/P ", Style::default().fg(theme::ACCENT_FG)),
        Span::styled("năm", Style::default().fg(theme::LABEL_FG)),
        Span::raw("  "),
        Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
        Span::styled("hôm nay", Style::default().fg(theme::LABEL_FG)),
        Span::raw("  "),
        Span::styled("H ", Style::default().fg(theme::ACCENT_FG)),
        Span::styled("ngày lễ", Style::default().fg(theme::LABEL_FG)),
        Span::raw("  "),
        Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
        Span::styled("thoát", Style::default().fg(theme::LABEL_FG)),
    ]))
    .alignment(Alignment::Center);

    frame.render_widget(footer, area);
}
