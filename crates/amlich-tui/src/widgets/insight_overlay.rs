use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::{App, InsightLang, InsightTab};
use crate::theme;

fn pick_text(lang: InsightLang, vi: &str, en: &str) -> String {
    match lang {
        InsightLang::Vi => vi.to_string(),
        InsightLang::En => en.to_string(),
    }
}

fn pick_items(lang: InsightLang, vi: &[String], en: &[String]) -> Vec<String> {
    match lang {
        InsightLang::Vi => vi.to_vec(),
        InsightLang::En => en.to_vec(),
    }
}

pub struct InsightOverlay<'a> {
    app: &'a App,
}

impl<'a> InsightOverlay<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    fn render_festival_tab(&self, insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
        let mut lines = Vec::new();

        if let Some(festival) = &insight.festival {
            let festival_name = pick_items(
                self.app.insight_lang,
                &festival.names.vi,
                &festival.names.en,
            )
            .first()
            .cloned()
            .unwrap_or_else(|| "Festival".to_string());

            lines.push(Line::from(vec![
                Span::styled("🎉 ", Style::default()),
                Span::styled(
                    festival_name,
                    Style::default()
                        .fg(theme::HOLIDAY_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            if let Some(origin) = &festival.origin {
                lines.push(Line::from(pick_text(
                    self.app.insight_lang,
                    &origin.vi,
                    &origin.en,
                )));
            }

            if let Some(activities) = &festival.activities {
                let activity_list =
                    pick_items(self.app.insight_lang, &activities.vi, &activities.en);
                if !activity_list.is_empty() {
                    lines.push(Line::from(""));
                    let label = match self.app.insight_lang {
                        InsightLang::Vi => "Hoạt động:",
                        InsightLang::En => "Activities:",
                    };
                    lines.push(Line::from(Span::styled(
                        label,
                        Style::default().fg(theme::LABEL_FG),
                    )));
                    for activity in activity_list.iter().take(5) {
                        lines.push(Line::from(format!("  • {activity}")));
                    }
                }
            }
        } else if let Some(holiday) = &insight.holiday {
            let holiday_name =
                pick_items(self.app.insight_lang, &holiday.names.vi, &holiday.names.en)
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "Holiday".to_string());

            lines.push(Line::from(vec![
                Span::styled("🏮 ", Style::default()),
                Span::styled(
                    holiday_name,
                    Style::default()
                        .fg(theme::HOLIDAY_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            if let Some(significance) = &holiday.significance {
                lines.push(Line::from(pick_text(
                    self.app.insight_lang,
                    &significance.vi,
                    &significance.en,
                )));
            }
        } else {
            let no_data = match self.app.insight_lang {
                InsightLang::Vi => "Không có lễ hội hay ngày lễ",
                InsightLang::En => "No festival or holiday today",
            };
            lines.push(Line::from(Span::styled(
                no_data,
                Style::default().fg(theme::LABEL_FG),
            )));
        }

        lines
    }

    fn render_guidance_tab(&self, insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
        let mut lines = Vec::new();

        if let Some(guidance) = &insight.day_guidance {
            let (good_label, avoid_label) = match self.app.insight_lang {
                InsightLang::Vi => ("✅ Nên làm", "⛔ Hạn chế"),
                InsightLang::En => ("✅ Do", "⛔ Avoid"),
            };

            lines.push(Line::from(Span::styled(
                good_label,
                Style::default()
                    .fg(theme::GOOD_HOUR_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            for item in pick_items(
                self.app.insight_lang,
                &guidance.good_for.vi,
                &guidance.good_for.en,
            ) {
                lines.push(Line::from(format!("• {item}")));
            }

            lines.push(Line::from(""));

            lines.push(Line::from(Span::styled(
                avoid_label,
                Style::default()
                    .fg(theme::WEEKEND_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            for item in pick_items(
                self.app.insight_lang,
                &guidance.avoid_for.vi,
                &guidance.avoid_for.en,
            ) {
                lines.push(Line::from(format!("• {item}")));
            }
        } else {
            let no_data = match self.app.insight_lang {
                InsightLang::Vi => "Không có thông tin hướng dẫn",
                InsightLang::En => "No guidance available",
            };
            lines.push(Line::from(Span::styled(
                no_data,
                Style::default().fg(theme::LABEL_FG),
            )));
        }

        lines
    }

    fn render_tiet_khi_tab(&self, insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
        let mut lines = Vec::new();

        if let Some(tiet_khi) = &insight.tiet_khi {
            lines.push(Line::from(vec![
                Span::styled("🌤️ ", Style::default()),
                Span::styled(
                    pick_text(self.app.insight_lang, &tiet_khi.name.vi, &tiet_khi.name.en),
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));

            lines.push(Line::from(""));

            let weather_label = match self.app.insight_lang {
                InsightLang::Vi => "Thời tiết:",
                InsightLang::En => "Weather:",
            };
            lines.push(Line::from(Span::styled(
                weather_label,
                Style::default().fg(theme::LABEL_FG),
            )));
            lines.push(Line::from(pick_text(
                self.app.insight_lang,
                &tiet_khi.weather.vi,
                &tiet_khi.weather.en,
            )));

            lines.push(Line::from(""));

            let agri = pick_items(
                self.app.insight_lang,
                &tiet_khi.agriculture.vi,
                &tiet_khi.agriculture.en,
            );
            let health = pick_items(
                self.app.insight_lang,
                &tiet_khi.health.vi,
                &tiet_khi.health.en,
            );

            if !agri.is_empty() {
                for item in agri.iter().take(3) {
                    lines.push(Line::from(format!("🌾 {item}")));
                }
            }

            if !health.is_empty() {
                for item in health.iter().take(3) {
                    lines.push(Line::from(format!("💚 {item}")));
                }
            }
        } else {
            let no_data = match self.app.insight_lang {
                InsightLang::Vi => "Không có thông tin tiết khí",
                InsightLang::En => "No seasonal information",
            };
            lines.push(Line::from(Span::styled(
                no_data,
                Style::default().fg(theme::LABEL_FG),
            )));
        }

        lines
    }

    fn tab_content(&self) -> Vec<Line<'_>> {
        let Some(insight) = self.app.selected_insight() else {
            let no_data = match self.app.insight_lang {
                InsightLang::Vi => "Không có dữ liệu insight",
                InsightLang::En => "No insight data",
            };
            return vec![Line::from(no_data)];
        };

        match self.app.insight_tab {
            InsightTab::Festival => self.render_festival_tab(&insight),
            InsightTab::Guidance => self.render_guidance_tab(&insight),
            InsightTab::TietKhi => self.render_tiet_khi_tab(&insight),
        }
    }

    fn tab_indicator(&self) -> Line<'_> {
        let tab = self.app.insight_tab;
        let lang = self.app.insight_lang;

        let (festival_style, guidance_style, tiet_khi_style) = match tab {
            InsightTab::Festival => (
                Style::default()
                    .fg(theme::HOLIDAY_FG)
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(theme::LABEL_FG),
                Style::default().fg(theme::LABEL_FG),
            ),
            InsightTab::Guidance => (
                Style::default().fg(theme::LABEL_FG),
                Style::default()
                    .fg(theme::GOOD_HOUR_FG)
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(theme::LABEL_FG),
            ),
            InsightTab::TietKhi => (
                Style::default().fg(theme::LABEL_FG),
                Style::default().fg(theme::LABEL_FG),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
        };

        let (festival_name, guidance_name, tiet_khi_name) = match lang {
            InsightLang::Vi => ("Lễ hội", "Hướng dẫn", "Tiết khí"),
            InsightLang::En => ("Festival", "Guidance", "Season"),
        };

        Line::from(vec![
            Span::styled("[1", festival_style),
            Span::styled("] ", festival_style),
            Span::styled(festival_name, festival_style),
            Span::raw("  "),
            Span::styled("[2", guidance_style),
            Span::styled("] ", guidance_style),
            Span::styled(guidance_name, guidance_style),
            Span::raw("  "),
            Span::styled("[3", tiet_khi_style),
            Span::styled("] ", tiet_khi_style),
            Span::styled(tiet_khi_name, tiet_khi_style),
        ])
    }
}

impl Widget for InsightOverlay<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Center the overlay - same pattern as HolidayOverlay
        let width = area.width.clamp(40, 70);
        let height = area.height.clamp(12, area.height.saturating_sub(4));
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let overlay_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(overlay_area, buf);

        let title_lang = match self.app.insight_lang {
            InsightLang::Vi => "VI",
            InsightLang::En => "EN",
        };

        let close_help = match self.app.insight_lang {
            InsightLang::Vi => " i/Esc đóng ",
            InsightLang::En => " i/Esc close ",
        };

        let scroll_help = match self.app.insight_lang {
            InsightLang::Vi => " ↑↓ cuộn ",
            InsightLang::En => " ↑↓ scroll ",
        };

        let tab_help = match self.app.insight_lang {
            InsightLang::Vi => " 1/2/3 tab ",
            InsightLang::En => " 1/2/3 tabs ",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Line::from(vec![Span::styled(
                format!(" ✨ Insight ({title_lang}) ",),
                theme::title_style(),
            )]))
            .title_bottom(
                Line::from(
                    vec![
                        Span::styled(close_help, Style::default().fg(theme::ACCENT_FG)),
                        Span::raw(" "),
                        Span::styled(scroll_help, Style::default().fg(theme::ACCENT_FG)),
                        Span::raw(" "),
                        Span::styled(tab_help, Style::default().fg(theme::ACCENT_FG)),
                    ]
                    .into_iter()
                    .collect::<Vec<_>>(),
                )
                .alignment(Alignment::Center),
            );

        // Render tab indicator
        let mut content = vec![Line::from(""), self.tab_indicator(), Line::from("")];

        // Add tab content
        content.extend(self.tab_content());

        Paragraph::new(content)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((self.app.insight_scroll, 0))
            .render(overlay_area, buf);
    }
}
