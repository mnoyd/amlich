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
    bookmarks::BookmarksOverlay, calendar::CalendarWidget, date_jump::DateJumpPopup,
    detail::DetailWidget, help::HelpOverlay, holidays::HolidayOverlay, hours::HoursWidget,
    insight::InsightWidget, search::SearchPopup,
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

// Minimum terminal size
const MIN_TERM_W: u16 = 40;
const MIN_TERM_H: u16 = 15;

// Layout breakpoints (body width)
const BP_MEDIUM: u16 = 80;
const BP_LARGE: u16 = 120;

const MIN_CAL_BODY_H: u16 = 15;
const MIN_INSIGHT_H: u16 = 6;

#[derive(Clone, Copy)]
enum LayoutMode {
    Small,
    Medium,
    Large,
}

fn layout_mode(width: u16) -> LayoutMode {
    if width < BP_MEDIUM {
        LayoutMode::Small
    } else if width < BP_LARGE {
        LayoutMode::Medium
    } else {
        LayoutMode::Large
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    if size.width < MIN_TERM_W || size.height < MIN_TERM_H {
        let msg = Paragraph::new("Terminal quá nhỏ.\nCần tối thiểu 40×15.")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme::border_style()),
            );
        frame.render_widget(msg, size);
        return;
    }

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

    // Bookmarks overlay
    if app.show_bookmarks {
        frame.render_widget(BookmarksOverlay::new(app), vertical[1]);
    }

    // Date jump popup
    if app.show_date_jump {
        frame.render_widget(DateJumpPopup::new(app), vertical[1]);
    }

    // Search popup
    if app.show_search {
        frame.render_widget(SearchPopup::new(app), vertical[1]);
    }

    // Help overlay
    if app.show_help {
        frame.render_widget(HelpOverlay::new(), vertical[1]);
    }
}

fn draw_header(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let month_name = MONTH_NAMES[(app.view_month - 1) as usize];

    // Year Can Chi from lunar data
    let year_canchi = app
        .selected_info()
        .map(|i| format!(" ({})", i.canchi.year.full))
        .unwrap_or_default();

    // Build status indicators
    let mut indicators = Vec::new();

    // History indicators
    if app.history_position > 0 {
        indicators.push(Span::styled("←", Style::default().fg(theme::LUNAR_FG)));
    }
    if app.history_position + 1 < app.history.len() {
        indicators.push(Span::styled("→", Style::default().fg(theme::LUNAR_FG)));
    }

    // Bookmark indicator
    if !app.bookmarks.is_empty() {
        indicators.push(Span::styled(
            format!("◆{}", app.bookmarks.len()),
            Style::default().fg(theme::ACCENT_FG),
        ));
    }

    // Search result indicator
    if !app.search_results.is_empty() {
        indicators.push(Span::styled(
            format!("🔍{}", app.search_results.len()),
            Style::default().fg(theme::HOLIDAY_FG),
        ));
    }

    let indicator_spans = if indicators.is_empty() {
        vec![]
    } else {
        vec![Span::raw("  "), Span::raw("["), Span::raw("")]
            .into_iter()
            .chain(indicators.into_iter())
            .chain(vec![Span::raw("]")])
            .collect()
    };

    let mut header_spans = vec![
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
        Span::styled(&year_canchi, Style::default().fg(theme::ACCENT_FG)),
        Span::styled(" ►", Style::default().fg(theme::LABEL_FG)),
    ];

    header_spans.extend(indicator_spans);

    let header = Paragraph::new(Line::from(header_spans))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style()),
    );

    frame.render_widget(header, area);
}

fn draw_body(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mode = layout_mode(area.width);

    // Insight split: only show if enough vertical space
    let can_show_insight = app.show_insight && area.height >= (MIN_CAL_BODY_H + MIN_INSIGHT_H);

    let body_sections = if can_show_insight {
        Layout::vertical([
            Constraint::Min(MIN_CAL_BODY_H),
            Constraint::Length(area.height.saturating_sub(MIN_CAL_BODY_H).min(area.height / 3)),
        ])
        .split(area)
    } else {
        Layout::vertical([Constraint::Percentage(100)]).split(area)
    };

    let main_area = body_sections[0];

    match mode {
        LayoutMode::Small => {
            // Calendar only — full width
            frame.render_widget(CalendarWidget::new(app), main_area);
        }
        LayoutMode::Medium => {
            // Calendar + Detail (with compact hours embedded below detail)
            let cols = Layout::horizontal([
                Constraint::Percentage(60),
                Constraint::Percentage(40),
            ])
            .split(main_area);

            frame.render_widget(CalendarWidget::new(app), cols[0]);

            // Split right column: detail on top, compact hours below
            let right = Layout::vertical([Constraint::Min(8), Constraint::Length(5)])
                .split(cols[1]);

            frame.render_widget(DetailWidget::new(app), right[0]);
            frame.render_widget(HoursWidget::new(app).compact(true), right[1]);
        }
        LayoutMode::Large => {
            // Full 3-column layout
            let cols = Layout::horizontal([
                Constraint::Ratio(1, 2),
                Constraint::Ratio(1, 4),
                Constraint::Ratio(1, 4),
            ])
            .split(main_area);

            frame.render_widget(CalendarWidget::new(app), cols[0]);
            frame.render_widget(DetailWidget::new(app), cols[1]);
            frame.render_widget(HoursWidget::new(app), cols[2]);
        }
    }

    if can_show_insight {
        frame.render_widget(InsightWidget::new(app), body_sections[1]);
    }
}

fn draw_footer(frame: &mut Frame, area: ratatui::layout::Rect) {
    let mode = layout_mode(area.width);

    let spans = match mode {
        LayoutMode::Small => vec![
            Span::styled("hjkl ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nav", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nay", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("? ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("help", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("exit", Style::default().fg(theme::LABEL_FG)),
        ],
        _ => vec![
            Span::styled(" ←↑↓→/hjkl ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("di chuyển", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("hôm nay", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("/ ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("tìm", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("g ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("đến ngày", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("b ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("bm", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("? ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("trợ giúp", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("thoát", Style::default().fg(theme::LABEL_FG)),
        ],
    };

    let footer = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(footer, area);
}
