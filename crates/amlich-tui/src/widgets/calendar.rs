use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

use crate::app::App;
use crate::theme;

const WEEK_LABELS: [&str; 7] = ["T2", "T3", "T4", "T5", "T6", "T7", "CN"];
const COL_WIDTH: u16 = 9;

pub struct CalendarWidget<'a> {
    app: &'a App,
}

impl<'a> CalendarWidget<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for CalendarWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Line::from(vec![Span::styled(
                " 📅 Lịch ",
                theme::title_style(),
            )]));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 || inner.width < 20 {
            return;
        }

        // Render weekday header
        let header_y = inner.y;
        let mut x = inner.x + 1;
        for (i, label) in WEEK_LABELS.iter().enumerate() {
            let style = if i >= 5 {
                Style::default()
                    .fg(theme::WEEKEND_FG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(theme::LABEL_FG)
                    .add_modifier(Modifier::BOLD)
            };
            let col_w = COL_WIDTH.min(inner.width.saturating_sub(x - inner.x));
            if col_w == 0 {
                break;
            }
            let centered = format!("{:^width$}", label, width = col_w as usize);
            buf.set_string(x, header_y, &centered, style);
            x += col_w;
        }

        // Render day grid
        let grid_start_y = header_y + 1;
        let mut day: u32 = 1;

        for row in 0..6u16 {
            let y = grid_start_y + row * 2; // 2 lines per row: solar + lunar
            if y + 1 >= inner.y + inner.height {
                break;
            }

            for col in 0..7u16 {
                let cell_x = inner.x + 1 + col * COL_WIDTH;
                if cell_x + COL_WIDTH > inner.x + inner.width {
                    break;
                }

                // Skip empty cells before first day
                if row == 0 && col < self.app.first_weekday as u16 {
                    continue;
                }

                if day > self.app.days_in_month {
                    break;
                }

                let is_selected = day == self.app.selected_day;
                let is_today = self.app.is_today(day);
                let is_weekend = col >= 5;
                let has_holiday = self.app.holiday_for_day(day).is_some();

                // Solar date style
                let solar_style = if is_selected {
                    Style::default()
                        .fg(theme::SELECTED_FG)
                        .bg(theme::SELECTED_BG)
                        .add_modifier(Modifier::BOLD)
                } else if is_today {
                    Style::default()
                        .fg(theme::TODAY_FG)
                        .bg(theme::TODAY_BG)
                        .add_modifier(Modifier::BOLD)
                } else if has_holiday {
                    Style::default()
                        .fg(theme::HOLIDAY_FG)
                        .add_modifier(Modifier::BOLD)
                } else if is_weekend {
                    Style::default().fg(theme::WEEKEND_FG)
                } else {
                    Style::default().fg(theme::SOLAR_FG)
                };

                // Solar day number
                let solar_text = format!("{:^width$}", day, width = COL_WIDTH as usize);
                buf.set_string(cell_x, y, &solar_text, solar_style);

                // Lunar day below
                if let Some(info) = self.app.month_days.get((day - 1) as usize) {
                    let lunar_text = if info.lunar.day == 1 {
                        format!("{}/{}", info.lunar.day, info.lunar.month)
                    } else {
                        format!("{}", info.lunar.day)
                    };
                    let lunar_display =
                        format!("{:^width$}", lunar_text, width = COL_WIDTH as usize);

                    let lunar_style = if is_selected {
                        Style::default()
                            .fg(theme::SELECTED_FG)
                            .bg(theme::SELECTED_BG)
                    } else if is_today {
                        Style::default().fg(theme::TODAY_FG).bg(theme::TODAY_BG)
                    } else {
                        Style::default().fg(theme::LUNAR_FG)
                    };
                    buf.set_string(cell_x, y + 1, &lunar_display, lunar_style);
                }

                day += 1;
            }
        }
    }
}
