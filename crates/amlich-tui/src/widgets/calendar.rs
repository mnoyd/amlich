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

        if inner.height < 3 || inner.width < 7 {
            return;
        }

        let Some(col_widths) = column_widths(inner.width) else {
            return;
        };

        // Render weekday header
        let header_y = inner.y;
        let mut x = inner.x;
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
            let col_w = col_widths[i];
            let centered = centered_cell(label, col_w);
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

            let mut cell_x = inner.x;
            for col in 0..7u16 {
                let cell_w = col_widths[col as usize];

                // Skip empty cells before first day
                if row == 0 && col < self.app.first_weekday as u16 {
                    cell_x += cell_w;
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
                let solar_text = centered_cell(&day.to_string(), cell_w);
                buf.set_string(cell_x, y, &solar_text, solar_style);

                // Lunar day below
                if let Some(info) = self.app.month_days.get((day - 1) as usize) {
                    let lunar_text = if info.lunar.day == 1 {
                        format!("{}/{}", info.lunar.day, info.lunar.month)
                    } else {
                        format!("{}", info.lunar.day)
                    };
                    let lunar_display = centered_cell(&lunar_text, cell_w);

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
                cell_x += cell_w;
            }
        }
    }
}

fn column_widths(total_width: u16) -> Option<[u16; 7]> {
    if total_width < 7 {
        return None;
    }

    let base = total_width / 7;
    let extra = total_width % 7;
    let mut widths = [base; 7];

    for i in 0..extra {
        widths[i as usize] += 1;
    }

    Some(widths)
}

fn centered_cell(text: &str, width: u16) -> String {
    if width == 0 {
        return String::new();
    }

    let width = width as usize;
    let trimmed: String = text.chars().take(width).collect();
    let len = trimmed.chars().count();

    if len >= width {
        return trimmed;
    }

    let pad_total = width - len;
    let pad_left = pad_total / 2;
    let pad_right = pad_total - pad_left;

    format!("{}{}{}", " ".repeat(pad_left), trimmed, " ".repeat(pad_right))
}

#[cfg(test)]
mod tests {
    use super::{centered_cell, column_widths};

    #[test]
    fn column_widths_distributes_evenly() {
        let widths = column_widths(64).expect("widths should exist");
        assert_eq!(widths.iter().sum::<u16>(), 64);
        assert_eq!(widths[0], 10);
        assert_eq!(widths[1], 9);
        assert_eq!(widths[6], 9);
    }

    #[test]
    fn column_widths_rejects_too_narrow() {
        assert!(column_widths(6).is_none());
    }

    #[test]
    fn centered_cell_trims_long_text() {
        assert_eq!(centered_cell("12345", 3), "123");
    }

    #[test]
    fn centered_cell_pads_short_text() {
        assert_eq!(centered_cell("2", 3), " 2 ");
    }
}
