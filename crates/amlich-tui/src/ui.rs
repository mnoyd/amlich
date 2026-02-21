use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, DensityMode};
use crate::theme;
use crate::widgets::{
    calendar::CalendarWidget, detail::DetailWidget, holidays::HolidayOverlay, hours::HoursWidget,
    insight::InsightWidget,
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

    // Top-level: header (3) + summary (1) + body + footer (3)
    let vertical = Layout::vertical([
        Constraint::Length(3),
        Constraint::Length(1),
        Constraint::Min(8),
        Constraint::Length(3),
    ])
    .split(size);

    draw_header(frame, app, vertical[0]);
    draw_summary(frame, app, vertical[1]);
    draw_body(frame, app, vertical[2]);
    draw_footer(frame, app, vertical[3]);

    // Holiday overlay
    if app.show_holidays {
        frame.render_widget(HolidayOverlay::new(app), vertical[2]);
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
        Span::styled(&year_canchi, Style::default().fg(theme::ACCENT_FG)),
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
    let mode = layout_mode(area.width);

    // Insight split: only show if enough vertical space
    let can_show_insight = app.show_insight
        && !matches!(app.density_mode, DensityMode::Compact)
        && area.height >= (MIN_CAL_BODY_H + MIN_INSIGHT_H);

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

    if matches!(app.density_mode, DensityMode::Compact) {
        frame.render_widget(CalendarWidget::new(app), main_area);
    } else {
        match mode {
            LayoutMode::Small => {
                if matches!(app.density_mode, DensityMode::Detail) && main_area.height >= 20 {
                    let rows = Layout::vertical([Constraint::Min(12), Constraint::Length(8)])
                        .split(main_area);
                    let bottom = Layout::horizontal([
                        Constraint::Percentage(65),
                        Constraint::Percentage(35),
                    ])
                    .split(rows[1]);

                    frame.render_widget(CalendarWidget::new(app), rows[0]);
                    frame.render_widget(DetailWidget::new(app), bottom[0]);
                    frame.render_widget(HoursWidget::new(app).compact(true), bottom[1]);
                } else {
                    frame.render_widget(CalendarWidget::new(app), main_area);
                }
            }
            LayoutMode::Medium => {
                let cols = Layout::horizontal([
                    Constraint::Percentage(60),
                    Constraint::Percentage(40),
                ])
                .split(main_area);

                frame.render_widget(CalendarWidget::new(app), cols[0]);

                if matches!(app.density_mode, DensityMode::Detail) {
                    let right = Layout::vertical([Constraint::Min(10), Constraint::Min(8)])
                        .split(cols[1]);
                    frame.render_widget(DetailWidget::new(app), right[0]);
                    frame.render_widget(HoursWidget::new(app), right[1]);
                } else {
                    // Normal: compact hours under details
                    let right = Layout::vertical([Constraint::Min(8), Constraint::Length(5)])
                        .split(cols[1]);
                    frame.render_widget(DetailWidget::new(app), right[0]);
                    frame.render_widget(HoursWidget::new(app).compact(true), right[1]);
                }
            }
            LayoutMode::Large => {
                let cols = if matches!(app.density_mode, DensityMode::Detail) {
                    Layout::horizontal([
                        Constraint::Percentage(45),
                        Constraint::Percentage(30),
                        Constraint::Percentage(25),
                    ])
                    .split(main_area)
                } else {
                    Layout::horizontal([
                        Constraint::Ratio(1, 2),
                        Constraint::Ratio(1, 4),
                        Constraint::Ratio(1, 4),
                    ])
                    .split(main_area)
                };

                frame.render_widget(CalendarWidget::new(app), cols[0]);
                frame.render_widget(DetailWidget::new(app), cols[1]);
                frame.render_widget(HoursWidget::new(app), cols[2]);
            }
        }
    }

    if can_show_insight {
        frame.render_widget(InsightWidget::new(app), body_sections[1]);
    }
}

fn draw_summary(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let Some(info) = app.selected_info() else {
        frame.render_widget(
            Paragraph::new("Không có dữ liệu ngày được chọn")
                .alignment(Alignment::Left),
            area,
        );
        return;
    };

    let leap = if info.lunar.is_leap_month { " nhuận" } else { "" };
    let holiday = app
        .holiday_for_day(app.selected_day)
        .map(|h| format!(" | Lễ {}", h.name))
        .unwrap_or_default();
    let extra_state = if app.show_extra_cultural {
        "Bật"
    } else {
        "Tắt"
    };
    let summary = format!(
        "M:{} | Extra:{} | DL {} ({}) | AL {}{} | Can Chi {} | Tiết khí {} | Giờ tốt {}{}",
        app.density_mode.label(),
        extra_state,
        info.solar.date_string,
        info.solar.day_of_week_name,
        info.lunar.date_string,
        leap,
        info.canchi.day.full,
        info.tiet_khi.name,
        info.gio_hoang_dao.good_hour_count,
        holiday
    );
    let text = truncate_to_width(&summary, area.width as usize);

    frame.render_widget(
        Paragraph::new(text)
            .style(Style::default().fg(theme::ACCENT_FG))
            .alignment(Alignment::Left),
        area,
    );
}

fn draw_footer(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mode = layout_mode(area.width);

    let shortcuts = match mode {
        LayoutMode::Small => vec![
            Span::styled("hjkl ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nav", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("n/p ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("th", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nay", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("m ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("mode", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("x ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("extra", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("exit", Style::default().fg(theme::LABEL_FG)),
        ],
        _ => vec![
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
            Span::styled("m ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("mật độ", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("H ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("ngày lễ", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("i ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("insight", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("L ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("VI/EN", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("x ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("extra", Style::default().fg(theme::LABEL_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("thoát", Style::default().fg(theme::LABEL_FG)),
        ],
    };

    let legend = match mode {
        LayoutMode::Small => vec![
            Span::styled("Màu: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled("DL", Style::default().fg(theme::SOLAR_FG)),
            Span::styled(" AL", Style::default().fg(theme::LUNAR_FG)),
            Span::styled(" T7/CN", Style::default().fg(theme::WEEKEND_FG)),
            Span::styled(" Lễ", Style::default().fg(theme::HOLIDAY_FG)),
        ],
        _ => vec![
            Span::styled("Màu: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled("Dương lịch", Style::default().fg(theme::SOLAR_FG)),
            Span::raw(" / "),
            Span::styled("Âm lịch", Style::default().fg(theme::LUNAR_FG)),
            Span::raw(" / "),
            Span::styled("T7-CN", Style::default().fg(theme::WEEKEND_FG)),
            Span::raw(" / "),
            Span::styled("Ngày lễ", Style::default().fg(theme::HOLIDAY_FG)),
        ],
    };

    let mode_line = Line::from(vec![
        Span::styled("Chế độ: ", Style::default().fg(theme::LABEL_FG)),
        Span::styled(
            app.density_mode.label(),
            Style::default()
                .fg(theme::ACCENT_FG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" | Extra: ", Style::default().fg(theme::LABEL_FG)),
        Span::styled(
            if app.show_extra_cultural { "Bật" } else { "Tắt" },
            Style::default()
                .fg(theme::ACCENT_FG)
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let footer = Paragraph::new(vec![mode_line, Line::from(shortcuts), Line::from(legend)])
        .alignment(Alignment::Center);
    frame.render_widget(footer, area);
}

fn truncate_to_width(text: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }

    let len = text.chars().count();
    if len <= width {
        return text.to_string();
    }

    if width <= 3 {
        return ".".repeat(width);
    }

    let prefix: String = text.chars().take(width - 3).collect();
    format!("{prefix}...")
}
