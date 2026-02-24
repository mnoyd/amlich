use ratatui::{
    layout::{Alignment, Constraint, Layout},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{App, LayoutMode};
use crate::theme;
use crate::widgets::{
    bookmarks::BookmarksOverlay, calendar::CalendarWidget, date_jump::DateJumpPopup,
    help::HelpOverlay, holidays::HolidayOverlay, info_panel::InfoPanel,
    insight_overlay::InsightOverlay, search::SearchPopup,
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

const MIN_TERM_W: u16 = 40;
const MIN_TERM_H: u16 = 15;

const BP_MEDIUM: u16 = 80;
const BP_LARGE: u16 = 120;

fn layout_mode(width: u16) -> LayoutMode {
    if width < BP_MEDIUM {
        LayoutMode::Small
    } else if width < BP_LARGE {
        LayoutMode::Medium
    } else {
        LayoutMode::Large
    }
}

fn large_split_percentages() -> (u16, u16) {
    (35, 65)
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    app.set_layout_mode(layout_mode(size.width));

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

    let vertical = Layout::vertical([
        Constraint::Length(1), // header — 1 line, no border
        Constraint::Min(10),   // body
        Constraint::Length(1), // footer
    ])
    .split(size);

    draw_header(frame, app, vertical[0]);
    draw_body(frame, app, vertical[1]);
    draw_footer(frame, app, vertical[2]);

    // Overlays (render on top of body area)
    if app.show_holidays {
        frame.render_widget(HolidayOverlay::new(app), vertical[1]);
    }
    if app.show_insight {
        frame.render_widget(InsightOverlay::new(app), vertical[1]);
    }
    if app.show_bookmarks {
        frame.render_widget(BookmarksOverlay::new(app), vertical[1]);
    }
    if app.show_date_jump {
        frame.render_widget(DateJumpPopup::new(app), vertical[1]);
    }
    if app.show_search {
        frame.render_widget(SearchPopup::new(app), vertical[1]);
    }
    if app.show_help {
        frame.render_widget(HelpOverlay::new(), vertical[1]);
    }
}

fn draw_header(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let month_name = MONTH_NAMES
        .get(app.view_month.saturating_sub(1) as usize)
        .copied()
        .unwrap_or("Tháng ?");

    let mut spans = vec![Span::styled(
        " Âm Lịch",
        Style::default()
            .fg(theme::ACCENT_FG)
            .add_modifier(Modifier::BOLD),
    )];

    // Bookmark indicator
    if !app.bookmarks.is_empty() {
        spans.push(Span::styled(
            format!("  ◆{}", app.bookmarks.len()),
            Style::default().fg(theme::SECONDARY_FG),
        ));
    }

    // Search result indicator
    if !app.search_results.is_empty() {
        spans.push(Span::styled(
            format!("  /{}", app.search_results.len()),
            Style::default().fg(theme::SECONDARY_FG),
        ));
    }

    // Right-align: navigation
    let nav_text = format!("◂ {} {} ▸ ", month_name, app.view_year);
    let left_len: usize = spans.iter().map(|s| s.content.chars().count()).sum();
    let right_len = nav_text.chars().count();
    let padding = (area.width as usize).saturating_sub(left_len + right_len);

    spans.push(Span::raw(" ".repeat(padding)));
    spans.push(Span::styled(
        nav_text,
        Style::default().fg(theme::SECONDARY_FG),
    ));

    let header = Paragraph::new(Line::from(spans));
    frame.render_widget(header, area);
}

fn draw_body(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mode = app.layout_mode;

    match mode {
        LayoutMode::Small => {
            if app.show_calendar {
                // Full-width calendar overlay (toggled by 'c')
                frame.render_widget(CalendarWidget::new(app), area);
            } else {
                // Info panel only — the hero view
                let inner = padded_area(area, 1, 0);
                frame.render_widget(InfoPanel::new(app), inner);
            }
        }
        LayoutMode::Medium => {
            // Mini calendar sidebar (~35%) + info panel (~65%)
            let cols = Layout::horizontal([Constraint::Percentage(35), Constraint::Percentage(65)])
                .split(area);

            frame.render_widget(CalendarWidget::new(app), cols[0]);

            let inner = padded_area(cols[1], 1, 0);
            frame.render_widget(InfoPanel::new(app), inner);
        }
        LayoutMode::Large => {
            // Full calendar (~35%) + spacious info panel (~65%)
            let (left, right) = large_split_percentages();
            let cols = Layout::horizontal([Constraint::Percentage(left), Constraint::Percentage(right)])
                .split(area);

            frame.render_widget(CalendarWidget::new(app), cols[0]);

            let inner = padded_area(cols[1], 1, 0);
            frame.render_widget(InfoPanel::new(app), inner);
        }
    }
}

/// Add horizontal/vertical padding to an area
fn padded_area(area: ratatui::layout::Rect, h_pad: u16, v_pad: u16) -> ratatui::layout::Rect {
    ratatui::layout::Rect {
        x: area.x + h_pad,
        y: area.y + v_pad,
        width: area.width.saturating_sub(h_pad * 2),
        height: area.height.saturating_sub(v_pad * 2),
    }
}

fn draw_footer(frame: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let mode = app.layout_mode;

    let spans = match mode {
        LayoutMode::Small => vec![
            Span::styled("c ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled(
                if app.show_calendar { "info" } else { "lịch" },
                Style::default().fg(theme::SECONDARY_FG),
            ),
            Span::raw("  "),
            Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nay", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("g ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nhảy", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("? ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("help", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("thoát", Style::default().fg(theme::SECONDARY_FG)),
        ],
        LayoutMode::Medium => vec![
            Span::styled(" hjkl ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nav", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("hôm nay", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("/ ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("tìm", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("g ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nhảy", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("b ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("bm", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("? ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("help", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("thoát", Style::default().fg(theme::SECONDARY_FG)),
        ],
        LayoutMode::Large => vec![
            Span::styled(" hjkl ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("nav", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("Tab ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("focus", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("Enter ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("mở", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("1-5 ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("thẻ", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("t ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("hôm nay", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("? ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("help", Style::default().fg(theme::SECONDARY_FG)),
            Span::raw("  "),
            Span::styled("q ", Style::default().fg(theme::ACCENT_FG)),
            Span::styled("thoát", Style::default().fg(theme::SECONDARY_FG)),
        ],
    };

    let footer = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(footer, area);
}

#[cfg(test)]
mod tests {
    use super::{large_split_percentages, layout_mode};
    use crate::app::LayoutMode;

    #[test]
    fn large_mode_split_is_35_65() {
        let (left, right) = large_split_percentages();
        assert_eq!((left, right), (35, 65));
    }

    #[test]
    fn layout_mode_thresholds_match_spec() {
        assert_eq!(layout_mode(79), LayoutMode::Small);
        assert_eq!(layout_mode(80), LayoutMode::Medium);
        assert_eq!(layout_mode(119), LayoutMode::Medium);
        assert_eq!(layout_mode(120), LayoutMode::Large);
    }
}
