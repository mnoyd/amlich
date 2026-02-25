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

fn push_bulleted(lines: &mut Vec<Line<'_>>, items: &[String], marker: &str, limit: usize) {
    for item in items.iter().take(limit) {
        lines.push(Line::from(format!("{marker} {item}")));
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
                        Style::default().fg(theme::SECONDARY_FG),
                    )));
                    for activity in activity_list.iter().take(5) {
                        lines.push(Line::from(format!("  • {activity}")));
                    }
                }
            }

            let foods = festival
                .food
                .iter()
                .map(|food| pick_text(self.app.insight_lang, &food.name.vi, &food.name.en))
                .collect::<Vec<_>>();
            if !foods.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Món ăn:",
                        InsightLang::En => "Foods:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                push_bulleted(&mut lines, &foods, "•", 4);
            }

            if !festival.taboos.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Điều kiêng:",
                        InsightLang::En => "Taboos:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                for taboo in festival.taboos.iter().take(4) {
                    let action =
                        pick_text(self.app.insight_lang, &taboo.action.vi, &taboo.action.en);
                    let reason =
                        pick_text(self.app.insight_lang, &taboo.reason.vi, &taboo.reason.en);
                    lines.push(Line::from(format!("• {action}")));
                    lines.push(Line::from(Span::styled(
                        format!("  {reason}"),
                        Style::default().fg(theme::SECONDARY_FG),
                    )));
                }
            }

            if !festival.proverbs.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Tục ngữ:",
                        InsightLang::En => "Proverbs:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                for proverb in festival.proverbs.iter().take(2) {
                    let meaning = pick_text(
                        self.app.insight_lang,
                        &proverb.meaning.vi,
                        &proverb.meaning.en,
                    );
                    lines.push(Line::from(format!("• {}", proverb.text)));
                    lines.push(Line::from(Span::styled(
                        format!("  {meaning}"),
                        Style::default().fg(theme::SECONDARY_FG),
                    )));
                }
            }

            if let Some(regions) = &festival.regions {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Theo vùng:",
                        InsightLang::En => "By region:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                lines.push(Line::from(format!(
                    "• Bắc: {}",
                    pick_text(self.app.insight_lang, &regions.north.vi, &regions.north.en)
                )));
                lines.push(Line::from(format!(
                    "• Trung: {}",
                    pick_text(
                        self.app.insight_lang,
                        &regions.central.vi,
                        &regions.central.en
                    )
                )));
                lines.push(Line::from(format!(
                    "• Nam: {}",
                    pick_text(self.app.insight_lang, &regions.south.vi, &regions.south.en)
                )));
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

            if let Some(activities) = &holiday.activities {
                let items = pick_items(self.app.insight_lang, &activities.vi, &activities.en);
                if !items.is_empty() {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        match self.app.insight_lang {
                            InsightLang::Vi => "Hoạt động:",
                            InsightLang::En => "Activities:",
                        },
                        Style::default().fg(theme::SECONDARY_FG),
                    )));
                    push_bulleted(&mut lines, &items, "•", 4);
                }
            }

            if let Some(traditions) = &holiday.traditions {
                let items = pick_items(self.app.insight_lang, &traditions.vi, &traditions.en);
                if !items.is_empty() {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        match self.app.insight_lang {
                            InsightLang::Vi => "Tập tục:",
                            InsightLang::En => "Traditions:",
                        },
                        Style::default().fg(theme::SECONDARY_FG),
                    )));
                    push_bulleted(&mut lines, &items, "•", 4);
                }
            }

            if !holiday.food.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Món ăn:",
                        InsightLang::En => "Foods:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                for food in holiday.food.iter().take(4) {
                    lines.push(Line::from(format!(
                        "• {}",
                        pick_text(self.app.insight_lang, &food.name.vi, &food.name.en)
                    )));
                }
            }

            if !holiday.taboos.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Điều kiêng:",
                        InsightLang::En => "Taboos:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                for taboo in holiday.taboos.iter().take(3) {
                    lines.push(Line::from(format!(
                        "• {}",
                        pick_text(self.app.insight_lang, &taboo.action.vi, &taboo.action.en)
                    )));
                }
            }

            if !holiday.proverbs.is_empty() {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Tục ngữ:",
                        InsightLang::En => "Proverbs:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                for proverb in holiday.proverbs.iter().take(2) {
                    lines.push(Line::from(format!("• {}", proverb.text)));
                }
            }

            if let Some(regions) = &holiday.regions {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Theo vùng:",
                        InsightLang::En => "By region:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                lines.push(Line::from(format!(
                    "• Bắc: {}",
                    pick_text(self.app.insight_lang, &regions.north.vi, &regions.north.en)
                )));
                lines.push(Line::from(format!(
                    "• Trung: {}",
                    pick_text(
                        self.app.insight_lang,
                        &regions.central.vi,
                        &regions.central.en
                    )
                )));
                lines.push(Line::from(format!(
                    "• Nam: {}",
                    pick_text(self.app.insight_lang, &regions.south.vi, &regions.south.en)
                )));
            }
        } else {
            let no_data = match self.app.insight_lang {
                InsightLang::Vi => "Không có lễ hội hay ngày lễ",
                InsightLang::En => "No festival or holiday today",
            };
            lines.push(Line::from(Span::styled(
                no_data,
                Style::default().fg(theme::SECONDARY_FG),
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
                Style::default().fg(theme::SECONDARY_FG),
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
            lines.push(Line::from(Span::styled(
                match self.app.insight_lang {
                    InsightLang::Vi => "Ý nghĩa:",
                    InsightLang::En => "Meaning:",
                },
                Style::default().fg(theme::SECONDARY_FG),
            )));
            lines.push(Line::from(pick_text(
                self.app.insight_lang,
                &tiet_khi.meaning.vi,
                &tiet_khi.meaning.en,
            )));

            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                match self.app.insight_lang {
                    InsightLang::Vi => "Thiên văn:",
                    InsightLang::En => "Astronomy:",
                },
                Style::default().fg(theme::SECONDARY_FG),
            )));
            lines.push(Line::from(pick_text(
                self.app.insight_lang,
                &tiet_khi.astronomy.vi,
                &tiet_khi.astronomy.en,
            )));

            lines.push(Line::from(""));

            let weather_label = match self.app.insight_lang {
                InsightLang::Vi => "Thời tiết:",
                InsightLang::En => "Weather:",
            };
            lines.push(Line::from(Span::styled(
                weather_label,
                Style::default().fg(theme::SECONDARY_FG),
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
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Nông nghiệp:",
                        InsightLang::En => "Agriculture:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
                for item in agri.iter().take(3) {
                    lines.push(Line::from(format!("🌾 {item}")));
                }
            }

            if !health.is_empty() {
                lines.push(Line::from(Span::styled(
                    match self.app.insight_lang {
                        InsightLang::Vi => "Sức khỏe:",
                        InsightLang::En => "Health:",
                    },
                    Style::default().fg(theme::SECONDARY_FG),
                )));
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
                Style::default().fg(theme::SECONDARY_FG),
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
            InsightTab::Festival => self.render_festival_tab(insight),
            InsightTab::Guidance => self.render_guidance_tab(insight),
            InsightTab::TietKhi => self.render_tiet_khi_tab(insight),
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
                Style::default().fg(theme::SECONDARY_FG),
                Style::default().fg(theme::SECONDARY_FG),
            ),
            InsightTab::Guidance => (
                Style::default().fg(theme::SECONDARY_FG),
                Style::default()
                    .fg(theme::GOOD_HOUR_FG)
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(theme::SECONDARY_FG),
            ),
            InsightTab::TietKhi => (
                Style::default().fg(theme::SECONDARY_FG),
                Style::default().fg(theme::SECONDARY_FG),
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
                format!(" Insight ({title_lang}) "),
                theme::section_style(),
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
