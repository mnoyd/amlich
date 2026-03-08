use chrono::{Datelike, Local, NaiveDate};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

const WEEKDAY_LABELS: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];

pub struct CalendarViewWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> CalendarViewWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for CalendarViewWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let month_date = self.app.calendar_cursor;
        let year = month_date.year();
        let month = month_date.month();
        let today = Local::now().naive_local().date();

        let block = Block::default()
            .title(" Lịch Tháng ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow));
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 24 || inner.height < 10 {
            Paragraph::new("Phóng to cửa sổ để xem lịch.")
                .style(Style::default().fg(Color::DarkGray))
                .render(inner, buf);
            return;
        }

        let mut lines = vec![
            Line::from(Span::styled(
                format!("Tháng {} năm {}", month, year),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        lines.push(Line::from(
            WEEKDAY_LABELS
                .iter()
                .enumerate()
                .map(|(i, label)| {
                    let style = if i == 6 {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::BOLD)
                    };
                    Span::styled(format!("{:^4}", label), style)
                })
                .collect::<Vec<_>>(),
        ));

        let first_day = NaiveDate::from_ymd_opt(year, month, 1).expect("valid first day");
        let padding = first_day.weekday().num_days_from_monday() as usize;
        let days_in_month = days_in_month(year, month);

        let mut day = 1u32;
        for row in 0..6 {
            let mut row_spans = Vec::with_capacity(7);
            for col in 0..7 {
                if row == 0 && col < padding {
                    row_spans.push(Span::raw("    "));
                    continue;
                }

                if day > days_in_month {
                    row_spans.push(Span::raw("    "));
                    continue;
                }

                let current =
                    NaiveDate::from_ymd_opt(year, month, day).expect("valid calendar day");
                let is_cursor = current == self.app.calendar_cursor;
                let is_active = current == self.app.date;
                let is_today = current == today;
                let is_sunday = col == 6;

                let mut style = Style::default().fg(Color::White);
                if is_sunday {
                    style = style.fg(Color::Red);
                }
                if is_today {
                    style = style.add_modifier(Modifier::BOLD);
                }
                if is_active && !is_cursor {
                    style = style.add_modifier(Modifier::UNDERLINED);
                }
                if is_cursor {
                    style = style
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD);
                }

                row_spans.push(Span::styled(format!("{:>3} ", day), style));
                day += 1;
            }
            lines.push(Line::from(row_spans));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "h/l or ←/→: ±1 ngày | j/k or ↑/↓: ±1 tuần",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            "PgUp/PgDn or p/n: ±1 tháng | Enter: chọn ngày | Esc/Space: đóng",
            Style::default().fg(Color::DarkGray),
        )));

        Paragraph::new(lines).render(inner, buf);
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let next_start =
        NaiveDate::from_ymd_opt(next_year, next_month, 1).expect("valid start of next month");
    next_start.pred_opt().expect("previous day exists").day()
}
