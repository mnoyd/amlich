use crate::layout::LayoutMode;
use crate::state::AppState;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct HeroWidget<'a> {
    app: &'a AppState,
}

impl<'a> HeroWidget<'a> {
    pub fn new(app: &'a AppState, _mode: LayoutMode) -> Self {
        Self { app }
    }
}

const ASCII_DIGITS: [&[&str]; 10] = [
    &[
        "  000  ", " 0   0 ", "0     0", "0     0", " 0   0 ", "  000  ",
    ],
    &[
        "   1   ", "  11   ", " 1 1   ", "   1   ", "   1   ", " 11111 ",
    ],
    &[
        "  222  ", " 2   2 ", "    2  ", "   2   ", "  2    ", " 22222 ",
    ],
    &[
        " 3333  ", "     3 ", "   33  ", "     3 ", " 3   3 ", "  333  ",
    ],
    &[
        "    4  ", "   44  ", "  4 4  ", " 44444 ", "    4  ", "    4  ",
    ],
    &[
        " 55555 ", " 5     ", " 5555  ", "     5 ", " 5   5 ", "  555  ",
    ],
    &[
        "  666  ", " 6     ", " 6666  ", " 6   6 ", " 6   6 ", "  666  ",
    ],
    &[
        " 77777 ", "    7  ", "   7   ", "  7    ", " 7     ", " 7     ",
    ],
    &[
        "  888  ", " 8   8 ", "  888  ", " 8   8 ", " 8   8 ", "  888  ",
    ],
    &[
        "  999  ", " 9   9 ", "  9999 ", "     9 ", "    9  ", "  99   ",
    ],
];

impl Widget for HeroWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(200, 40, 40))); // Red border
        let inner = block.inner(area);
        block.render(area, buf);

        let layout = Layout::vertical([
            Constraint::Min(8),    // Day + Text
            Constraint::Length(1), // Holiday
        ])
        .split(inner);

        let top_layout = Layout::horizontal([
            Constraint::Length(25), // Ascii digits
            Constraint::Min(20),
        ])
        .split(layout[0]);

        // Render ascii day
        let day = bundle.solar.day;
        let d1 = (day / 10) as usize;
        let d2 = (day % 10) as usize;

        let mut ascii_lines = vec![Line::from(""); 6];
        for i in 0..6 {
            let s = format!("{}  {}", ASCII_DIGITS[d1][i], ASCII_DIGITS[d2][i]);
            ascii_lines[i] = Line::from(Span::styled(
                s,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ));
        }

        // Vertically center ascii art
        let padding = (top_layout[0].height.saturating_sub(6)) / 2;
        let mut final_ascii = vec![Line::from(""); padding as usize];
        final_ascii.extend(ascii_lines);

        Paragraph::new(final_ascii)
            .alignment(Alignment::Center)
            .render(top_layout[0], buf);

        let dow = bundle.solar.day_of_week_name.to_uppercase();
        let solar_month_year = format!("THÁNG {} NĂM {}", bundle.solar.month, bundle.solar.year);
        let lunar_str = format!("Âm lịch: {}", bundle.lunar.date_string);

        let canchi_str = bundle
            .canchi
            .as_ref()
            .map(|c| c.day.full.clone())
            .unwrap_or_default();
        let nap_am = bundle
            .day_fortune
            .as_ref()
            .map(|f| format!("Nạp âm: {}", f.day_element.na_am))
            .unwrap_or_default();

        let mut right_lines = vec![Line::from(""); padding as usize];
        right_lines.extend(vec![
            Line::from(Span::styled(
                dow,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(
                solar_month_year,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(lunar_str, Style::default().fg(Color::Cyan))),
            Line::from(Span::styled(canchi_str, Style::default().fg(Color::Gray))),
            Line::from(Span::styled(nap_am, Style::default().fg(Color::DarkGray))),
        ]);

        Paragraph::new(right_lines)
            .alignment(Alignment::Left)
            .render(top_layout[1], buf);

        if let Some(badge) = holiday_badge(bundle) {
            let banner = Paragraph::new(Line::from(vec![
                Span::styled(" ★ ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    badge,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" ★ ", Style::default().fg(Color::Yellow)),
            ]))
            .alignment(Alignment::Center)
            .style(Style::default().bg(Color::Rgb(180, 0, 0))); // Red bg

            banner.render(layout[1], buf);
        }
    }
}

fn holiday_badge(bundle: &amlich_api::v2::DayBundleDto) -> Option<String> {
    let insight = bundle.insight.as_ref()?;
    if let Some(holiday) = insight.holiday.as_ref() {
        return holiday.names.vi.first().cloned();
    }
    insight
        .festival
        .as_ref()
        .and_then(|festival| festival.names.vi.first().cloned())
}
