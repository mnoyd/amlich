use chrono::{Datelike, NaiveDate};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Cell, Row, Table, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;

pub struct MiniCalendarWidget<'a> {
    app: &'a AppState,
}

impl<'a> MiniCalendarWidget<'a> {
    pub fn new(app: &'a AppState, _mode: LayoutMode) -> Self {
        Self { app }
    }
}

impl Widget for MiniCalendarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = format!(" Tháng {}/{} ", self.app.date.month(), self.app.date.year());
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let header = Row::new(vec!["T2", "T3", "T4", "T5", "T6", "T7", "CN"])
            .style(Style::default().fg(Color::Gray));

        let date = self.app.date;
        let year = date.year();
        let month = date.month();
        
        let first_day = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
        let mut start_date = first_day;
        
        // Backtrack to Monday
        while start_date.weekday() != chrono::Weekday::Mon {
            start_date = start_date.pred_opt().unwrap();
        }

        let mut rows = vec![];
        let mut current_date = start_date;
        let today_style = Style::default().fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
        let current_month_style = Style::default().fg(Color::White);
        let other_month_style = Style::default().fg(Color::DarkGray);

        for _ in 0..6 {
            let mut row_cells = vec![];
            let mut has_current_month = false;
            
            for _ in 0..7 {
                let is_today = current_date == date;
                let is_current_month = current_date.month() == month;
                
                if is_current_month {
                    has_current_month = true;
                }

                let style = if is_today {
                    today_style
                } else if is_current_month {
                    current_month_style
                } else {
                    other_month_style
                };

                row_cells.push(Cell::from(Span::styled(format!("{:>2}", current_date.day()), style)));
                current_date = current_date.succ_opt().unwrap();
            }

            if has_current_month {
                rows.push(Row::new(row_cells));
            }
        }

        let widths = [
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ];

        Table::new(rows, widths)
            .header(header)
            .block(block)
            .column_spacing(1)
            .render(area, buf);
    }
}
