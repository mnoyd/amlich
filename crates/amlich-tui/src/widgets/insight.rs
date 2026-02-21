use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::app::{App, InsightLang};
use crate::theme;

pub struct InsightWidget<'a> {
    app: &'a App,
}

impl<'a> InsightWidget<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }
}

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

impl Widget for InsightWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title_lang = match self.app.insight_lang {
            InsightLang::Vi => "VI",
            InsightLang::En => "EN",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Line::from(vec![Span::styled(
                format!(" ✨ Insight ({title_lang}) "),
                theme::title_style(),
            )]));

        let Some(insight) = self.app.selected_insight() else {
            Paragraph::new("Không có dữ liệu insight")
                .block(block)
                .render(area, buf);
            return;
        };

        let festival = insight.festival.as_ref();
        let holiday = insight.holiday.as_ref();
        let mut lines: Vec<Line> = Vec::new();

        if let Some(festival) = festival {
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
            if let Some(origin) = festival.origin.as_ref() {
                lines.push(Line::from(pick_text(
                    self.app.insight_lang,
                    &origin.vi,
                    &origin.en,
                )));
            }
            lines.push(Line::from(""));
        } else if let Some(holiday) = holiday {
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
            if let Some(significance) = holiday.significance.as_ref() {
                lines.push(Line::from(pick_text(
                    self.app.insight_lang,
                    &significance.vi,
                    &significance.en,
                )));
            }
            lines.push(Line::from(""));
        }

        if let Some(guidance) = insight.day_guidance.as_ref() {
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
            )
            .into_iter()
            .take(3)
            {
                lines.push(Line::from(format!("• {item}")));
            }

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
            )
            .into_iter()
            .take(2)
            {
                lines.push(Line::from(format!("• {item}")));
            }
            lines.push(Line::from(""));
        }

        if self.app.show_extra_cultural {
            let activities: Vec<String>;
            let taboos: Vec<String>;

            if let Some(festival) = festival {
                activities = festival
                    .activities
                    .as_ref()
                    .map(|list| pick_items(self.app.insight_lang, &list.vi, &list.en))
                    .unwrap_or_default()
                    .into_iter()
                    .take(2)
                    .collect();
                taboos = festival
                    .taboos
                    .iter()
                    .map(|item| pick_text(self.app.insight_lang, &item.action.vi, &item.action.en))
                    .take(2)
                    .collect();
            } else if let Some(holiday) = holiday {
                let picked = holiday
                    .activities
                    .as_ref()
                    .map(|list| pick_items(self.app.insight_lang, &list.vi, &list.en))
                    .unwrap_or_default();
                activities = picked.into_iter().take(2).collect();
                taboos = holiday
                    .taboos
                    .iter()
                    .map(|item| pick_text(self.app.insight_lang, &item.action.vi, &item.action.en))
                    .take(2)
                    .collect();
            } else {
                activities = Vec::new();
                taboos = Vec::new();
            }

            if !activities.is_empty() || !taboos.is_empty() {
                let (activities_label, taboos_label) = match self.app.insight_lang {
                    InsightLang::Vi => ("🪭 Hoạt động", "🚫 Kiêng kỵ"),
                    InsightLang::En => ("🪭 Activities", "🚫 Taboos"),
                };

                lines.push(Line::from(Span::styled(
                    activities_label,
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD),
                )));
                for item in activities {
                    lines.push(Line::from(format!("• {item}")));
                }

                lines.push(Line::from(Span::styled(
                    taboos_label,
                    Style::default()
                        .fg(theme::WEEKEND_FG)
                        .add_modifier(Modifier::BOLD),
                )));
                for item in taboos {
                    lines.push(Line::from(format!("• {item}")));
                }
                lines.push(Line::from(""));
            }
        }

        if let Some(tiet_khi) = insight.tiet_khi.as_ref() {
            lines.push(Line::from(vec![
                Span::styled("🌤️ ", Style::default()),
                Span::styled(
                    pick_text(self.app.insight_lang, &tiet_khi.name.vi, &tiet_khi.name.en),
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(pick_text(
                self.app.insight_lang,
                &tiet_khi.weather.vi,
                &tiet_khi.weather.en,
            )));

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

            if let Some(item) = agri.first() {
                lines.push(Line::from(format!("🌾 {item}")));
            }
            if let Some(item) = health.first() {
                lines.push(Line::from(format!("💚 {item}")));
            }
        }

        Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }
}
