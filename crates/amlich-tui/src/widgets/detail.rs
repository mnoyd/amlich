use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::app::App;
use crate::theme;

pub struct DetailWidget<'a> {
    app: &'a App,
}

impl<'a> DetailWidget<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

impl Widget for DetailWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Line::from(vec![Span::styled(
                " 📜 Chi Tiết ",
                theme::title_style(),
            )]));

        let Some(info) = self.app.selected_info() else {
            let p = Paragraph::new("Không có dữ liệu").block(block);
            p.render(area, buf);
            return;
        };

        let mut lines = Vec::new();

        // Solar date
        lines.push(Line::from(vec![
            Span::styled("📅 ", Style::default()),
            Span::styled(
                format!("{} ({})", info.solar.date_string, info.solar.day_of_week_name),
                Style::default()
                    .fg(theme::VALUE_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(""));

        // Lunar date
        lines.push(Line::from(vec![
            Span::styled("🌙 Âm lịch: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled(
                &info.lunar.date_string,
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(""));

        // Can Chi section
        lines.push(Line::from(Span::styled(
            "📜 Can Chi",
            Style::default()
                .fg(theme::TITLE_FG)
                .add_modifier(Modifier::BOLD),
        )));

        lines.push(Line::from(vec![
            Span::styled("   Ngày:  ", Style::default().fg(theme::LABEL_FG)),
            Span::styled(&info.canchi.day.full, Style::default().fg(theme::VALUE_FG)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("   Tháng: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled(&info.canchi.month.full, Style::default().fg(theme::VALUE_FG)),
        ]));
        lines.push(Line::from(vec![
            Span::styled("   Năm:   ", Style::default().fg(theme::LABEL_FG)),
            Span::styled(
                format!("{} ({})", info.canchi.year.full, info.canchi.year.con_giap),
                Style::default().fg(theme::VALUE_FG),
            ),
        ]));

        lines.push(Line::from(""));

        // Ngũ Hành
        lines.push(Line::from(Span::styled(
            "🌟 Ngũ Hành",
            Style::default()
                .fg(theme::TITLE_FG)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(vec![
            Span::styled("   Can: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled(
                &info.canchi.day.ngu_hanh.can,
                Style::default().fg(theme::VALUE_FG),
            ),
            Span::styled("  Chi: ", Style::default().fg(theme::LABEL_FG)),
            Span::styled(
                &info.canchi.day.ngu_hanh.chi,
                Style::default().fg(theme::VALUE_FG),
            ),
        ]));

        lines.push(Line::from(""));

        // Tiết Khí
        lines.push(Line::from(Span::styled(
            "🌤️  Tiết Khí",
            Style::default()
                .fg(theme::TITLE_FG)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(vec![
            Span::styled("   ", Style::default()),
            Span::styled(
                &info.tiet_khi.name,
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!(" ({})", info.tiet_khi.season),
                Style::default().fg(theme::LABEL_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("   ", Style::default()),
            Span::styled(
                &info.tiet_khi.description,
                Style::default().fg(theme::LUNAR_FG),
            ),
        ]));

        // Holidays for selected day
        if let Some(holiday) = self.app.holiday_for_day(self.app.selected_day) {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "🎉 Ngày Lễ",
                Style::default()
                    .fg(theme::TITLE_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(vec![
                Span::styled("   ", Style::default()),
                Span::styled(
                    &holiday.name,
                    Style::default()
                        .fg(theme::HOLIDAY_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled("   ", Style::default()),
                Span::styled(&holiday.description, Style::default().fg(theme::LUNAR_FG)),
            ]));
        }

        let p = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false });
        p.render(area, buf);
    }
}
