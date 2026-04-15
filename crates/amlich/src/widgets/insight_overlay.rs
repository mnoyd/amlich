use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::{App, BaziSubview, InsightLang, InsightTab};
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

fn recommendation_advisories(lang: InsightLang, info: &amlich_api::DayInfoDto) -> Vec<String> {
    let has_medical = info
        .daily_recommendations
        .activities
        .iter()
        .any(|activity| activity.activity_id == "medical_treatment");
    let has_burial = info
        .daily_recommendations
        .activities
        .iter()
        .any(|activity| activity.activity_id == "burial_memorial");

    let mut notes = Vec::new();
    if has_medical {
        notes.push(pick_text(
            lang,
            "Lưu ý: việc điều trị thực tế luôn ưu tiên đánh giá chuyên môn; lịch chỉ mang tính tham khảo.",
            "Note: real medical care should follow professional judgment first; calendar guidance is only advisory.",
        ));
    }
    if has_burial {
        notes.push(pick_text(
            lang,
            "Lưu ý: an táng hoặc tưởng niệm cần thẩm định thêm theo tập tục và chuyên gia địa phương.",
            "Note: burial or memorial planning needs added review against local tradition and expert guidance.",
        ));
    }

    notes
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

    fn render_guidance_tab(&self, _insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let Some(info) = self.app.selected_info() else {
            return vec![Line::from(Span::styled(
                "Không có dữ liệu",
                Style::default().fg(theme::SECONDARY_FG),
            ))];
        };

        lines.push(Line::from(vec![
            Span::styled(
                match self.app.insight_lang {
                    InsightLang::Vi => "Tổng quan: ",
                    InsightLang::En => "Summary: ",
                },
                Style::default().fg(theme::SECONDARY_FG),
            ),
            Span::styled(
                pick_text(
                    self.app.insight_lang,
                    &info.daily_recommendations.summary_vi,
                    &info.daily_recommendations.summary_en,
                ),
                Style::default().fg(theme::ACCENT_FG),
            ),
        ]));
        lines.push(Line::from(""));

        for bucket in [
            amlich_api::RecommendationBucketDto::Nen,
            amlich_api::RecommendationBucketDto::CoThe,
            amlich_api::RecommendationBucketDto::Tranh,
            amlich_api::RecommendationBucketDto::KyManh,
        ] {
            let items: Vec<&amlich_api::SynthesizedRecommendationDto> = info
                .daily_recommendations
                .activities
                .iter()
                .filter(|activity| activity.bucket == bucket)
                .collect();
            if items.is_empty() {
                continue;
            }

            let (title, style) = match (self.app.insight_lang, bucket) {
                (InsightLang::Vi, amlich_api::RecommendationBucketDto::Nen) => {
                    ("✅ Nên", Style::default().fg(theme::GOOD_FG))
                }
                (InsightLang::Vi, amlich_api::RecommendationBucketDto::CoThe) => {
                    ("ℹ Có thể", Style::default().fg(theme::ACCENT_FG))
                }
                (InsightLang::Vi, amlich_api::RecommendationBucketDto::Tranh) => {
                    ("⚠ Tránh", Style::default().fg(theme::BAD_FG))
                }
                (InsightLang::Vi, amlich_api::RecommendationBucketDto::KyManh) => (
                    "⛔ Kỵ mạnh",
                    Style::default()
                        .fg(theme::BAD_FG)
                        .add_modifier(Modifier::BOLD),
                ),
                (InsightLang::En, amlich_api::RecommendationBucketDto::Nen) => {
                    ("✅ Nên (Recommended)", Style::default().fg(theme::GOOD_FG))
                }
                (InsightLang::En, amlich_api::RecommendationBucketDto::CoThe) => {
                    ("ℹ Có thể (Consider)", Style::default().fg(theme::ACCENT_FG))
                }
                (InsightLang::En, amlich_api::RecommendationBucketDto::Tranh) => {
                    ("⚠ Tránh (Avoid)", Style::default().fg(theme::BAD_FG))
                }
                (InsightLang::En, amlich_api::RecommendationBucketDto::KyManh) => (
                    "⛔ Kỵ mạnh (Hard stop)",
                    Style::default()
                        .fg(theme::BAD_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            };

            lines.push(Line::from(Span::styled(
                format!("{title} ({})", items.len()),
                style.add_modifier(Modifier::BOLD),
            )));
            for item in items.iter().take(6) {
                lines.push(Line::from(format!("• {}", item.label.vi)));
            }
            lines.push(Line::from(""));
        }

        for note in recommendation_advisories(self.app.insight_lang, info) {
            lines.push(Line::from(Span::styled(
                note,
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

    fn render_almanac_tab<'b>(&self, insight: &'b amlich_api::DayInsightDto) -> Vec<Line<'b>> {
        let mut lines = Vec::new();
        let lang = self.app.insight_lang;

        // Can Chi insight detail
        if let Some(canchi) = &insight.canchi {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Can Chi ngày:", "Day Stem-Branch:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(vec![
                Span::raw(format!("  {} — ", canchi.can.name)),
                Span::styled(
                    pick_text(lang, &canchi.can.nature.vi, &canchi.can.nature.en),
                    Style::default().fg(theme::SECONDARY_FG),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::raw(format!("  {} — ", canchi.chi.name)),
                Span::styled(
                    pick_text(lang, &canchi.chi.meaning.vi, &canchi.chi.meaning.en),
                    Style::default().fg(theme::SECONDARY_FG),
                ),
                Span::raw(format!(
                    " ({})",
                    pick_text(lang, &canchi.chi.animal.vi, &canchi.chi.animal.en),
                )),
            ]));
            lines.push(Line::from(""));
        }

        if let Some(truc) = &insight.truc {
            lines.push(Line::from(vec![
                Span::styled(
                    pick_text(lang, "Trực: ", "Truc: "),
                    Style::default().fg(theme::SECONDARY_FG),
                ),
                Span::styled(
                    format!("{} ({})", truc.name, truc.quality),
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            lines.push(Line::from(pick_text(
                lang,
                &truc.meaning.vi,
                &truc.meaning.en,
            )));
            let good = pick_items(lang, &truc.good_for.vi, &truc.good_for.en);
            if !good.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Nên làm:", "Good for:"),
                    Style::default().fg(theme::GOOD_HOUR_FG),
                )));
                push_bulleted(&mut lines, &good, "•", 5);
            }
            let avoid = pick_items(lang, &truc.avoid_for.vi, &truc.avoid_for.en);
            if !avoid.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Hạn chế:", "Avoid:"),
                    Style::default().fg(theme::WEEKEND_FG),
                )));
                push_bulleted(&mut lines, &avoid, "•", 5);
            }
            lines.push(Line::from(""));
        }

        if let Some(deity) = &insight.day_deity {
            let class_text = pick_text(
                lang,
                &deity.classification_meaning.vi,
                &deity.classification_meaning.en,
            );
            lines.push(Line::from(vec![
                Span::styled(
                    pick_text(lang, "Thần cai quản: ", "Day Deity: "),
                    Style::default().fg(theme::SECONDARY_FG),
                ),
                Span::styled(
                    format!("{} — {}", deity.name, class_text),
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            if let Some(meaning) = &deity.deity_meaning {
                lines.push(Line::from(pick_text(lang, &meaning.vi, &meaning.en)));
            }
            lines.push(Line::from(""));
        }

        if let Some(stars) = &insight.stars {
            if !stars.cat_tinh.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(
                        pick_text(lang, "Cát tinh: ", "Lucky stars: "),
                        Style::default().fg(theme::GOOD_HOUR_FG),
                    ),
                    Span::raw(stars.cat_tinh.join(", ")),
                ]));
            }
            if !stars.sat_tinh.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(
                        pick_text(lang, "Sát tinh: ", "Unlucky stars: "),
                        Style::default().fg(theme::WEEKEND_FG),
                    ),
                    Span::raw(stars.sat_tinh.join(", ")),
                ]));
            }
            if let Some(day_star) = &stars.day_star {
                let quality = stars.day_star_quality.as_deref().unwrap_or("");
                lines.push(Line::from(vec![
                    Span::styled(
                        pick_text(lang, "Sao chủ ngày: ", "Day star: "),
                        Style::default().fg(theme::SECONDARY_FG),
                    ),
                    Span::styled(
                        day_star.as_str(),
                        Style::default()
                            .fg(theme::ACCENT_FG)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(" ({quality})")),
                ]));
            }
            lines.push(Line::from(""));
        }

        if let Some(na_am) = &insight.na_am {
            lines.push(Line::from(vec![
                Span::styled("Na Am: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    format!("{} ({})", na_am.na_am, na_am.element),
                    Style::default().fg(theme::ACCENT_FG),
                ),
            ]));
            lines.push(Line::from(pick_text(
                lang,
                &na_am.meaning.vi,
                &na_am.meaning.en,
            )));
            lines.push(Line::from(""));
        }

        if let Some(taboos) = &insight.taboos {
            if !taboos.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Kiêng kỵ:", "Taboos:"),
                    Style::default()
                        .fg(theme::WEEKEND_FG)
                        .add_modifier(Modifier::BOLD),
                )));
                for t in taboos.iter().take(5) {
                    lines.push(Line::from(format!(
                        "• [{}] {} — {}",
                        t.severity, t.name, t.reason
                    )));
                }
            }
        }

        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Không có dữ liệu lịch", "No almanac data"),
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines
    }

    fn render_advanced_tab<'b>(&self, insight: &'b amlich_api::DayInsightDto) -> Vec<Line<'b>> {
        let mut lines = Vec::new();
        let lang = self.app.insight_lang;

        // Travel directions (kept here — not moved to other tabs)
        if let Some(travel) = &insight.travel {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Xuất hành:", "Travel:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(vec![
                Span::styled(
                    pick_text(lang, "  Hướng: ", "  Direction: "),
                    Style::default().fg(theme::SECONDARY_FG),
                ),
                Span::styled(
                    &travel.xuat_hanh_huong,
                    Style::default().fg(theme::GOOD_HOUR_FG),
                ),
            ]));
            lines.push(Line::from(vec![
                Span::styled(
                    pick_text(lang, "  Tài Thần: ", "  Wealth God: "),
                    Style::default().fg(theme::SECONDARY_FG),
                ),
                Span::styled(&travel.tai_than, Style::default().fg(theme::ACCENT_FG)),
            ]));
            lines.push(Line::from(vec![
                Span::styled(
                    pick_text(lang, "  Hỷ Thần: ", "  Joy God: "),
                    Style::default().fg(theme::SECONDARY_FG),
                ),
                Span::styled(&travel.hy_than, Style::default().fg(theme::GOOD_HOUR_FG)),
            ]));
            lines.push(Line::from(""));
        }

        // Cross-reference hints
        lines.push(Line::from(Span::styled(
            pick_text(
                lang,
                "Xem thêm: [5] Giờ  [6] Ngũ hành  [7] Phong thủy",
                "See also: [5] Hours  [6] Elements  [7] Feng Shui",
            ),
            Style::default().fg(theme::SECONDARY_FG),
        )));

        lines
    }

    fn render_personal_tab(&self, insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let lang = self.app.insight_lang;
        let (birth_year, birth_month, birth_day, gender) = self.app.personal_profile();
        let personal_report = self.app.selected_info().and_then(|info| {
            amlich_api::get_personal_day_report(
                &amlich_api::DateQuery {
                    day: info.solar.day,
                    month: info.solar.month,
                    year: info.solar.year,
                    timezone: Some(amlich_core::VIETNAM_TIMEZONE),
                    ruleset_id: None,
                    event_kind: None,
                    enabled_pack_ids: vec![],
                },
                birth_year,
                birth_month,
                birth_day,
                gender,
            )
            .ok()
        });

        if personal_report.is_none() && insight.tu_menh.is_none() && insight.dai_van.is_none() {
            lines.push(Line::from(Span::styled(
                pick_text(
                    lang,
                    "Chưa cấu hình hồ sơ cá nhân.",
                    "No personal profile configured.",
                ),
                Style::default().fg(theme::SECONDARY_FG),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                pick_text(
                    lang,
                    "Dùng: amlich config profile set --birth-year XXXX --gender male/female",
                    "Use: amlich config profile set --birth-year XXXX --gender male/female",
                ),
                Style::default().fg(theme::ACCENT_FG),
            )));
            return lines;
        }

        if let Some(report) = &personal_report {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Tổng quan cá nhân hóa:", "Personalized overview:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(format!(
                "  {} / 4 hồ sơ • {}",
                report.computed_metrics.profile_completeness,
                if report.computed_metrics.has_personal_recommendations {
                    pick_text(lang, "đủ dữ liệu cá nhân", "personal context available")
                } else {
                    pick_text(lang, "thiếu ngữ cảnh cá nhân", "personal context missing")
                }
            )));
            if !report.computed_metrics.available_sections.is_empty() {
                lines.push(Line::from(format!(
                    "  {}",
                    report.computed_metrics.available_sections.join(", ")
                )));
            }
            lines.push(Line::from(""));

            if !report.advisory.highlights.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Điểm nổi bật:", "Highlights:"),
                    Style::default().fg(theme::GOOD_HOUR_FG),
                )));
                push_bulleted(&mut lines, &report.advisory.highlights, "•", 4);
                lines.push(Line::from(""));
            }

            if !report.advisory.cautions.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Lưu ý:", "Cautions:"),
                    Style::default().fg(theme::WEEKEND_FG),
                )));
                push_bulleted(&mut lines, &report.advisory.cautions, "•", 4);
                lines.push(Line::from(""));
            }
        }

        if let Some(tu_menh) = &insight.tu_menh {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Tứ Mệnh (Kua):", "Tu Menh (Kua):"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(format!(
                "  Kua {} — {} ({})",
                tu_menh.kua,
                tu_menh.group,
                pick_text(lang, &tu_menh.trigram.vi, &tu_menh.trigram.en),
            )));
            lines.push(Line::from(pick_text(
                lang,
                &tu_menh.meaning.vi,
                &tu_menh.meaning.en,
            )));
            lines.push(Line::from(""));

            if !tu_menh.favorable_directions.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Hướng tốt:", "Favorable:"),
                    Style::default().fg(theme::GOOD_HOUR_FG),
                )));
                let items: Vec<String> = tu_menh.favorable_directions.clone();
                push_bulleted(&mut lines, &items, "•", 4);
            }

            if !tu_menh.unfavorable_directions.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Hướng xấu:", "Unfavorable:"),
                    Style::default().fg(theme::WEEKEND_FG),
                )));
                let items: Vec<String> = tu_menh.unfavorable_directions.clone();
                push_bulleted(&mut lines, &items, "•", 4);
            }
            lines.push(Line::from(""));
        }

        if let Some(dai_van) = &insight.dai_van {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Đại Vận:", "Dai Van:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(format!(
                "  {} — {}",
                dai_van.direction,
                pick_text(
                    lang,
                    &dai_van.direction_meaning.vi,
                    &dai_van.direction_meaning.en
                ),
            )));
            if let Some(pillar) = &dai_van.current_pillar {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "  Đại vận hiện tại:", "  Current pillar:"),
                    Style::default().fg(theme::GOOD_HOUR_FG),
                )));
                lines.push(Line::from(format!(
                    "  {} ({}-{}) — {}",
                    pillar.can_chi,
                    pillar.start_age,
                    pillar.end_age,
                    pick_text(lang, &pillar.element_meaning.vi, &pillar.element_meaning.en),
                )));
            }
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            pick_text(
                lang,
                "Chi tiết: [7] Phong thủy (la bàn, đại vận, hướng)",
                "Details: [7] Feng Shui (compass, dai van, directions)",
            ),
            Style::default().fg(theme::SECONDARY_FG),
        )));

        lines
    }

    fn render_hours_tab(&self, _insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let lang = self.app.insight_lang;

        // Section 1: All 12 hours from DayInfoDto
        if let Some(info) = self.app.selected_info() {
            let gio = &info.gio_hoang_dao;

            lines.push(Line::from(Span::styled(
                pick_text(lang, "Tổng quan 12 giờ:", "12-Hour Overview:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(format!(
                "  {} {} — {} {}",
                pick_text(lang, "Ngày:", "Day:"),
                gio.day_chi,
                gio.good_hour_count,
                pick_text(lang, "giờ tốt", "good hours"),
            )));
            lines.push(Line::from(""));

            // Good hours section
            lines.push(Line::from(Span::styled(
                pick_text(lang, "★ Giờ Hoàng Đạo:", "★ Auspicious Hours:"),
                Style::default()
                    .fg(theme::GOOD_HOUR_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            for h in &gio.all_hours {
                if h.is_good {
                    lines.push(Line::from(vec![
                        Span::styled("  ★ ", Style::default().fg(theme::GOOD_HOUR_FG)),
                        Span::styled(
                            format!("{:<6}", h.hour_chi),
                            Style::default()
                                .fg(theme::ACCENT_FG)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            format!("({}) ", h.time_range),
                            Style::default().fg(theme::SECONDARY_FG),
                        ),
                        Span::raw(format!("— {}", h.star)),
                    ]));
                }
            }
            lines.push(Line::from(""));

            // Bad hours section
            lines.push(Line::from(Span::styled(
                pick_text(lang, "· Giờ Hắc Đạo:", "· Inauspicious Hours:"),
                Style::default()
                    .fg(theme::WEEKEND_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            for h in &gio.all_hours {
                if !h.is_good {
                    lines.push(Line::from(vec![
                        Span::styled("  · ", Style::default().fg(theme::WEEKEND_FG)),
                        Span::styled(
                            format!("{:<6}", h.hour_chi),
                            Style::default().fg(theme::SECONDARY_FG),
                        ),
                        Span::styled(
                            format!("({}) ", h.time_range),
                            Style::default().fg(theme::SECONDARY_FG),
                        ),
                        Span::raw(format!("— {}", h.star)),
                    ]));
                }
            }
            lines.push(Line::from(""));

            // Timeline summary
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Biểu đồ giờ:", "Hour Chart:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            let mut chart_spans = vec![Span::raw("  ")];
            for h in &gio.all_hours {
                let marker = if h.is_good { "★" } else { "·" };
                let style = if h.is_good {
                    Style::default().fg(theme::GOOD_HOUR_FG)
                } else {
                    Style::default().fg(theme::SECONDARY_FG)
                };
                chart_spans.push(Span::styled(marker, style));
            }
            lines.push(Line::from(chart_spans));

            let mut label_spans = vec![Span::raw("  ")];
            for h in gio.all_hours.iter().take(12) {
                let ch: String = h.hour_chi.chars().take(1).collect();
                let style = if h.is_good {
                    Style::default().fg(theme::GOOD_HOUR_FG)
                } else {
                    Style::default().fg(theme::SECONDARY_FG)
                };
                label_spans.push(Span::styled(ch, style));
            }
            lines.push(Line::from(label_spans));
        } else {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Không có dữ liệu giờ", "No hour data"),
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines
    }

    fn render_elements_tab<'b>(&self, insight: &'b amlich_api::DayInsightDto) -> Vec<Line<'b>> {
        let mut lines = Vec::new();
        let lang = self.app.insight_lang;

        // Section 1: Can Chi & Ngũ Hành overview
        if let Some(canchi) = &insight.canchi {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Can Chi ngày:", "Day Stem-Branch:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(vec![
                Span::raw("  Can: "),
                Span::styled(&canchi.can.name, Style::default().fg(theme::ACCENT_FG)),
                Span::raw(format!(
                    " ({}) — {}",
                    canchi.can.element,
                    pick_text(lang, &canchi.can.meaning.vi, &canchi.can.meaning.en),
                )),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  Chi: "),
                Span::styled(&canchi.chi.name, Style::default().fg(theme::ACCENT_FG)),
                Span::raw(format!(
                    " ({}) — {}",
                    canchi.chi.element,
                    pick_text(lang, &canchi.chi.meaning.vi, &canchi.chi.meaning.en),
                )),
            ]));
            if let Some(element) = &canchi.element {
                lines.push(Line::from(vec![
                    Span::raw(pick_text(lang, "  Nạp âm: ", "  Na Am: ")),
                    Span::styled(
                        pick_text(lang, &element.name.vi, &element.name.en),
                        Style::default().fg(theme::ACCENT_FG),
                    ),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Section 2: Tàng Can (Hidden Stems)
        if let Some(tang_can) = &insight.tang_can {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Tàng Can:", "Hidden Stems:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            let labels = [
                pick_text(lang, "Chính", "Main"),
                pick_text(lang, "Trung", "Central"),
                pick_text(lang, "Dư", "Residual"),
            ];
            let values = [&tang_can.main, &tang_can.central, &tang_can.residual];
            for (i, (label, value)) in labels.iter().zip(values.iter()).enumerate() {
                let s = tang_can.strength[i];
                let bar_len = (s as usize * 10) / 100;
                let bar_full: String = "█".repeat(bar_len);
                let bar_empty: String = "░".repeat(10 - bar_len);
                lines.push(Line::from(vec![
                    Span::raw(format!("  {label}: ")),
                    Span::styled(
                        format!("{value:<4}"),
                        Style::default()
                            .fg(theme::ACCENT_FG)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(bar_full, Style::default().fg(theme::GOOD_HOUR_FG)),
                    Span::styled(bar_empty, Style::default().fg(theme::SECONDARY_FG)),
                    Span::raw(format!(" {s}%")),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Section 3: Thập Thần (Ten Gods)
        if let Some(ten_gods) = &insight.ten_gods {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Thập Thần:", "Ten Gods:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            if let Some(entry) = &ten_gods.to_year_stem {
                let polarity = if entry.same_polarity {
                    pick_text(lang, "đồng cực", "same polarity")
                } else {
                    pick_text(lang, "khác cực", "diff polarity")
                };
                lines.push(Line::from(vec![
                    Span::raw(pick_text(lang, "  Với năm: ", "  To year: ")),
                    Span::styled(&entry.label, Style::default().fg(theme::ACCENT_FG)),
                    Span::raw(format!(
                        " — {} ({}, {})",
                        pick_text(lang, &entry.meaning.vi, &entry.meaning.en),
                        entry.relation,
                        polarity,
                    )),
                ]));
            }
            if let Some(entry) = &ten_gods.to_self {
                let polarity = if entry.same_polarity {
                    pick_text(lang, "đồng cực", "same polarity")
                } else {
                    pick_text(lang, "khác cực", "diff polarity")
                };
                lines.push(Line::from(vec![
                    Span::raw(pick_text(lang, "  Với mình: ", "  To self: ")),
                    Span::styled(&entry.label, Style::default().fg(theme::ACCENT_FG)),
                    Span::raw(format!(
                        " — {} ({}, {})",
                        pick_text(lang, &entry.meaning.vi, &entry.meaning.en),
                        entry.relation,
                        polarity,
                    )),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Section 4: Xung Hợp (Clash/Harmony)
        if let Some(xung_hop) = &insight.xung_hop {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Xung Hợp:", "Clash/Harmony:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(vec![
                Span::styled(
                    pick_text(lang, "  Lục Xung: ", "  Six Clash: "),
                    Style::default().fg(theme::WEEKEND_FG),
                ),
                Span::raw(&xung_hop.luc_xung),
            ]));
            if !xung_hop.tam_hop.is_empty() {
                lines.push(Line::from(vec![
                    Span::styled(
                        pick_text(lang, "  Tam Hợp: ", "  Three Harmony: "),
                        Style::default().fg(theme::GOOD_HOUR_FG),
                    ),
                    Span::raw(xung_hop.tam_hop.join(" — ")),
                ]));
            }
            if let Some(liu_he) = &xung_hop.liu_he {
                lines.push(Line::from(vec![
                    Span::styled(
                        pick_text(lang, "  Lục Hợp: ", "  Six Harmony: "),
                        Style::default().fg(theme::GOOD_HOUR_FG),
                    ),
                    Span::raw(liu_he.as_str()),
                ]));
            }
            if let Some(xiang_hai) = &xung_hop.xiang_hai {
                lines.push(Line::from(vec![
                    Span::styled(
                        pick_text(lang, "  Tương Hại: ", "  Mutual Harm: "),
                        Style::default().fg(theme::WEEKEND_FG),
                    ),
                    Span::raw(xiang_hai.as_str()),
                ]));
            }
            lines.push(Line::from(""));
        }

        // Section 5: Ngũ Hành from Na Am
        if let Some(na_am) = &insight.na_am {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Nạp Âm:", "Na Am:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(vec![Span::raw(format!(
                "  {} ({})",
                na_am.na_am, na_am.element
            ))]));
            lines.push(Line::from(pick_text(
                lang,
                &na_am.meaning.vi,
                &na_am.meaning.en,
            )));
        }

        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Không có dữ liệu ngũ hành", "No element data"),
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines
    }

    fn render_bazi_tab(&self) -> Vec<Line<'_>> {
        let lang = self.app.insight_lang;
        let Some(report) = self.app.selected_bazi() else {
            return vec![
                Line::from(Span::styled(
                    pick_text(
                        lang,
                        "Cần hồ sơ có giới tính để bật Bát Tự trong TUI.",
                        "A profile with gender is required to enable Bazi in the TUI.",
                    ),
                    Style::default().fg(theme::SECONDARY_FG),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    pick_text(
                        lang,
                        "Dùng: amlich config profile set --birth-year XXXX --gender male/female",
                        "Use: amlich config profile set --birth-year XXXX --gender male/female",
                    ),
                    Style::default().fg(theme::ACCENT_FG),
                )),
            ];
        };

        let mut lines = Vec::new();
        let current = self.app.bazi_subview;
        let confidence = report
            .computed_metrics
            .structure_metrics
            .confidence
            .clamp(0.0, 1.0);
        let score_chip = |score: u8| {
            if score >= 70 {
                Style::default()
                    .fg(theme::GOOD_HOUR_FG)
                    .add_modifier(Modifier::BOLD)
            } else if score >= 40 {
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
                    .fg(theme::WEEKEND_FG)
                    .add_modifier(Modifier::BOLD)
            }
        };

        let subviews = [
            (BaziSubview::Overview, "o"),
            (BaziSubview::Timing, "t"),
            (BaziSubview::Advisory, "a"),
            (BaziSubview::Metrics, "m"),
        ];
        let mut subview_spans = vec![Span::styled("  Bazi ", theme::section_style())];
        for (subview, key) in subviews {
            let style = if subview == current {
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::SECONDARY_FG)
            };
            let label = match (subview, lang) {
                (BaziSubview::Overview, InsightLang::Vi) => "Tổng quan",
                (BaziSubview::Overview, InsightLang::En) => "Overview",
                (BaziSubview::Timing, InsightLang::Vi) => "Vận",
                (BaziSubview::Timing, InsightLang::En) => "Timing",
                (BaziSubview::Advisory, InsightLang::Vi) => "Luận",
                (BaziSubview::Advisory, InsightLang::En) => "Advisory",
                (BaziSubview::Metrics, InsightLang::Vi) => "Điểm",
                (BaziSubview::Metrics, InsightLang::En) => "Metrics",
            };
            subview_spans.push(Span::styled(format!("[{key}]{} ", label), style));
        }
        lines.push(Line::from(subview_spans));
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled(
                report.chart.day_master.full.as_str(),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" · "),
            Span::styled(
                report.analysis.day_master_strength.label.as_str(),
                Style::default()
                    .fg(theme::GOOD_HOUR_FG)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" · "),
            Span::styled(
                format!("{:.0}%", confidence * 100.0),
                Style::default().fg(theme::SECONDARY_FG),
            ),
        ]));
        lines.push(Line::from(""));

        match current {
            BaziSubview::Overview => {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Snapshot", "Snapshot"),
                    theme::section_style(),
                )));
                let confidence_fill = (confidence * 12.0).round() as usize;
                lines.push(Line::from(vec![
                    Span::raw("  Confidence "),
                    Span::styled(
                        "█".repeat(confidence_fill),
                        Style::default().fg(theme::GOOD_HOUR_FG),
                    ),
                    Span::styled(
                        "░".repeat(12 - confidence_fill),
                        Style::default().fg(theme::SECONDARY_FG),
                    ),
                ]));
                lines.push(Line::from(""));

                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Tứ trụ", "Four Pillars"),
                    theme::section_style(),
                )));
                for pillar in &report.chart.pillars {
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            format!("{:<5}", pillar.kind),
                            Style::default().fg(theme::SECONDARY_FG),
                        ),
                        Span::raw("│ "),
                        Span::styled(
                            pillar.can_chi.full.as_str(),
                            Style::default().fg(theme::ACCENT_FG),
                        ),
                    ]));
                }
                lines.push(Line::from(""));

                let e = &report.analysis.element_distribution;
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Ngũ hành", "Elements"),
                    theme::section_style(),
                )));
                for (label, value, style) in [
                    ("Mộc", e.moc, Style::default().fg(theme::GOOD_HOUR_FG)),
                    ("Hỏa", e.hoa, Style::default().fg(theme::WEEKEND_FG)),
                    ("Thổ", e.tho, Style::default().fg(theme::ACCENT_FG)),
                    ("Kim", e.kim, Style::default().fg(theme::SECONDARY_FG)),
                    ("Thủy", e.thuy, Style::default().fg(theme::ACCENT_FG)),
                ] {
                    let width = ((value as f32 / 120.0).clamp(0.0, 8.0)).round() as usize;
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            format!("{label:<4}"),
                            Style::default().fg(theme::SECONDARY_FG),
                        ),
                        Span::raw("│"),
                        Span::styled("■".repeat(width), style),
                        Span::styled(
                            "·".repeat(8 - width),
                            Style::default().fg(theme::SECONDARY_FG),
                        ),
                        Span::raw(format!(" {:>3}", value)),
                    ]));
                }
            }
            BaziSubview::Timing => {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Timing board", "Timing board"),
                    theme::section_style(),
                )));
                if let Some(timing) = &report.timing {
                    if let Some(active) = &timing.active_dai_van {
                        lines.push(Line::from(vec![
                            Span::raw("  Đại vận │ "),
                            Span::styled(
                                active.can_chi.as_str(),
                                Style::default()
                                    .fg(theme::ACCENT_FG)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::raw(format!("  {}-{}", active.start_age, active.end_age)),
                        ]));
                    }
                    lines.push(Line::from(vec![
                        Span::raw("  Lưu niên │ "),
                        Span::styled(
                            timing.annual.can_chi.as_str(),
                            Style::default().fg(theme::GOOD_HOUR_FG),
                        ),
                    ]));
                    for month in timing.monthly.iter().take(3) {
                        lines.push(Line::from(vec![
                            Span::raw("  Lưu nguyệt│ "),
                            Span::styled(
                                format!("T{:02}", month.month),
                                Style::default().fg(theme::SECONDARY_FG),
                            ),
                            Span::raw(" "),
                            Span::raw(month.can_chi.as_str()),
                        ]));
                    }
                    if !report
                        .computed_metrics
                        .timing_metrics
                        .activation_summary
                        .is_empty()
                    {
                        lines.push(Line::from(""));
                        for summary in report
                            .computed_metrics
                            .timing_metrics
                            .activation_summary
                            .iter()
                            .take(3)
                        {
                            lines.push(Line::from(vec![
                                Span::styled("  • ", Style::default().fg(theme::GOOD_HOUR_FG)),
                                Span::raw(summary.as_str()),
                            ]));
                        }
                    }
                } else {
                    lines.push(Line::from(Span::styled(
                        pick_text(lang, "Không có dữ liệu vận", "No timing data"),
                        Style::default().fg(theme::SECONDARY_FG),
                    )));
                }
            }
            BaziSubview::Advisory => {
                let useful = &report.advisory.useful_god_analysis;
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Advisory board", "Advisory board"),
                    theme::section_style(),
                )));
                if !useful.favorable_elements.is_empty() {
                    lines.push(Line::from(vec![
                        Span::styled("  + ", Style::default().fg(theme::GOOD_HOUR_FG)),
                        Span::raw(format!(
                            "{} {}",
                            pick_text(lang, "Hành lợi", "Favorable"),
                            useful.favorable_elements.join(", ")
                        )),
                    ]));
                }
                if !useful.unfavorable_elements.is_empty() {
                    lines.push(Line::from(vec![
                        Span::styled("  - ", Style::default().fg(theme::WEEKEND_FG)),
                        Span::raw(format!(
                            "{} {}",
                            pick_text(lang, "Hành kỵ", "Avoid"),
                            useful.unfavorable_elements.join(", ")
                        )),
                    ]));
                }
                for warning in report.advisory.warnings.iter().take(3) {
                    lines.push(Line::from(vec![
                        Span::styled("  ! ", Style::default().fg(theme::WEEKEND_FG)),
                        Span::raw(warning.as_str()),
                    ]));
                }
                lines.push(Line::from(""));
                lines.push(Line::from(report.advisory.summary_vi.clone()));
            }
            BaziSubview::Metrics => {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Metrics board", "Metrics board"),
                    theme::section_style(),
                )));
                for (label_vi, label_en, score) in [
                    (
                        "Công việc",
                        "Career",
                        &report.computed_metrics.domain_scores.career,
                    ),
                    (
                        "Tài lộc",
                        "Wealth",
                        &report.computed_metrics.domain_scores.wealth,
                    ),
                    (
                        "Quan hệ",
                        "Relation",
                        &report.computed_metrics.domain_scores.relationship,
                    ),
                    (
                        "Sức khỏe",
                        "Health",
                        &report.computed_metrics.domain_scores.health,
                    ),
                    (
                        "Vận thời",
                        "Timing",
                        &report.computed_metrics.domain_scores.timing,
                    ),
                ] {
                    let width = (score.score as usize) / 10;
                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            format!("{:<9}", pick_text(lang, label_vi, label_en)),
                            Style::default().fg(theme::SECONDARY_FG),
                        ),
                        Span::styled("■".repeat(width), score_chip(score.score)),
                        Span::styled(
                            "·".repeat(10 - width),
                            Style::default().fg(theme::SECONDARY_FG),
                        ),
                        Span::raw(format!(" {:>3}", score.score)),
                    ]));
                }
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::raw("  Core │ "),
                    Span::raw(format!(
                        "DM {} · season {:.1} · balance {:.1}",
                        report
                            .computed_metrics
                            .core_metrics
                            .day_master_strength_score,
                        report.computed_metrics.core_metrics.season_support_score,
                        report.computed_metrics.core_metrics.element_balance_score,
                    )),
                ]));
            }
        }

        lines
    }

    fn render_feng_shui_tab<'b>(&self, insight: &'b amlich_api::DayInsightDto) -> Vec<Line<'b>> {
        let mut lines = Vec::new();
        let lang = self.app.insight_lang;

        if insight.tu_menh.is_none() && insight.dai_van.is_none() {
            lines.push(Line::from(Span::styled(
                pick_text(
                    lang,
                    "Chưa cấu hình hồ sơ cá nhân.",
                    "No personal profile configured.",
                ),
                Style::default().fg(theme::SECONDARY_FG),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                pick_text(
                    lang,
                    "Dùng: amlich config profile set --birth-year XXXX --gender male/female",
                    "Use: amlich config profile set --birth-year XXXX --gender male/female",
                ),
                Style::default().fg(theme::ACCENT_FG),
            )));
            return lines;
        }

        // Section 1: Kua / Tứ Mệnh
        if let Some(tu_menh) = &insight.tu_menh {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Tứ Mệnh (Kua):", "Tu Menh (Kua):"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(vec![
                Span::raw("  Kua "),
                Span::styled(
                    tu_menh.kua.to_string(),
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    " — {} ({})",
                    tu_menh.group,
                    pick_text(lang, &tu_menh.trigram.vi, &tu_menh.trigram.en),
                )),
            ]));
            lines.push(Line::from(pick_text(
                lang,
                &tu_menh.meaning.vi,
                &tu_menh.meaning.en,
            )));
            lines.push(Line::from(""));

            // Favorable directions
            if !tu_menh.favorable_directions.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Hướng tốt:", "Favorable:"),
                    Style::default().fg(theme::GOOD_HOUR_FG),
                )));
                for dir in &tu_menh.favorable_directions {
                    lines.push(Line::from(vec![
                        Span::styled("  ★ ", Style::default().fg(theme::GOOD_HOUR_FG)),
                        Span::raw(dir.as_str()),
                    ]));
                }
            }
            if !tu_menh.unfavorable_directions.is_empty() {
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "Hướng xấu:", "Unfavorable:"),
                    Style::default().fg(theme::WEEKEND_FG),
                )));
                for dir in &tu_menh.unfavorable_directions {
                    lines.push(Line::from(vec![
                        Span::styled("  ✖ ", Style::default().fg(theme::WEEKEND_FG)),
                        Span::raw(dir.as_str()),
                    ]));
                }
            }
            lines.push(Line::from(""));

            // ASCII compass
            lines.push(Line::from(Span::styled(
                pick_text(lang, "La Bàn:", "Compass:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));

            let good: Vec<&str> = tu_menh
                .favorable_directions
                .iter()
                .map(|s| s.as_str())
                .collect();
            let bad: Vec<&str> = tu_menh
                .unfavorable_directions
                .iter()
                .map(|s| s.as_str())
                .collect();

            let dir_style = |name: &str| -> Style {
                if good.iter().any(|d| d.contains(name)) {
                    Style::default()
                        .fg(theme::GOOD_HOUR_FG)
                        .add_modifier(Modifier::BOLD)
                } else if bad.iter().any(|d| d.contains(name)) {
                    Style::default().fg(theme::WEEKEND_FG)
                } else {
                    Style::default().fg(theme::SECONDARY_FG)
                }
            };
            let marker = |name: &str| -> &str {
                if good.iter().any(|d| d.contains(name)) {
                    "★"
                } else if bad.iter().any(|d| d.contains(name)) {
                    "✖"
                } else {
                    "·"
                }
            };

            let bac = pick_text(lang, "Bắc", "N");
            let nam = pick_text(lang, "Nam", "S");
            let dong = pick_text(lang, "Đông", "E");
            let tay = pick_text(lang, "Tây", "W");

            lines.push(Line::from(vec![
                Span::raw("           "),
                Span::styled(format!("{} {bac}", marker("Bắc")), dir_style("Bắc")),
            ]));
            lines.push(Line::from(vec![
                Span::raw("      "),
                Span::styled(format!("{} TB", marker("Tây Bắc")), dir_style("Tây Bắc")),
                Span::raw("   |   "),
                Span::styled(format!("ĐB {}", marker("Đông Bắc")), dir_style("Đông Bắc")),
            ]));
            lines.push(Line::from(vec![
                Span::raw("     "),
                Span::styled(format!("{} {tay}", marker("Tây")), dir_style("Tây")),
                Span::raw(" ——●—— "),
                Span::styled(format!("{dong} {}", marker("Đông")), dir_style("Đông")),
            ]));
            lines.push(Line::from(vec![
                Span::raw("      "),
                Span::styled(format!("{} TN", marker("Tây Nam")), dir_style("Tây Nam")),
                Span::raw("   |   "),
                Span::styled(format!("ĐN {}", marker("Đông Nam")), dir_style("Đông Nam")),
            ]));
            lines.push(Line::from(vec![
                Span::raw("           "),
                Span::styled(format!("{} {nam}", marker("Nam")), dir_style("Nam")),
            ]));
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("  ★ ", Style::default().fg(theme::GOOD_HOUR_FG)),
                Span::raw(pick_text(lang, "Tốt  ", "Good  ")),
                Span::styled("✖ ", Style::default().fg(theme::WEEKEND_FG)),
                Span::raw(pick_text(lang, "Xấu", "Bad")),
            ]));
            lines.push(Line::from(""));
        }

        // Section 2: Đại Vận
        if let Some(dai_van) = &insight.dai_van {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Đại Vận:", "Dai Van:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(format!(
                "  {} — {}",
                dai_van.direction,
                pick_text(
                    lang,
                    &dai_van.direction_meaning.vi,
                    &dai_van.direction_meaning.en
                ),
            )));

            // Current pillar
            if let Some(pillar) = &dai_van.current_pillar {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    pick_text(lang, "  Đại vận hiện tại:", "  Current pillar:"),
                    Style::default().fg(theme::GOOD_HOUR_FG),
                )));
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  ▶ {} ", pillar.can_chi),
                        Style::default()
                            .fg(theme::ACCENT_FG)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(format!(
                        "({}-{}) ",
                        pillar.start_age as u32, pillar.end_age as u32
                    )),
                    Span::styled(&pillar.element, Style::default().fg(theme::ACCENT_FG)),
                ]));
                lines.push(Line::from(format!(
                    "    {}",
                    pick_text(lang, &pillar.element_meaning.vi, &pillar.element_meaning.en)
                )));
            }

            // All pillars timeline
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                pick_text(lang, "  Các đại vận:", "  All pillars:"),
                Style::default().fg(theme::SECONDARY_FG),
            )));
            for pillar in &dai_van.all_pillars {
                let is_current = dai_van
                    .current_pillar
                    .as_ref()
                    .map(|c| c.index == pillar.index)
                    .unwrap_or(false);

                let marker_str = if is_current { "◄" } else { " " };
                let style = if is_current {
                    Style::default()
                        .fg(theme::ACCENT_FG)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme::SECONDARY_FG)
                };

                lines.push(Line::from(Span::styled(
                    format!(
                        "  {}. {:<10} ({:>2}-{:>2}) {:>4} {marker_str}",
                        pillar.index,
                        pillar.can_chi,
                        pillar.start_age as u32,
                        pillar.end_age as u32,
                        pillar.element,
                    ),
                    style,
                )));
            }
        }

        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Không có dữ liệu phong thủy", "No feng shui data"),
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
            InsightTab::Almanac => self.render_almanac_tab(insight),
            InsightTab::Hours => self.render_hours_tab(insight),
            InsightTab::Elements => self.render_elements_tab(insight),
            InsightTab::FengShui => self.render_feng_shui_tab(insight),
            InsightTab::Advanced => self.render_advanced_tab(insight),
            InsightTab::Personal => self.render_personal_tab(insight),
            InsightTab::Bazi => self.render_bazi_tab(),
        }
    }

    fn tab_indicator(&self) -> Line<'_> {
        let current = self.app.insight_tab;
        let lang = self.app.insight_lang;
        let tabs = [
            (InsightTab::Festival, "1"),
            (InsightTab::Guidance, "2"),
            (InsightTab::TietKhi, "3"),
            (InsightTab::Almanac, "4"),
            (InsightTab::Hours, "5"),
            (InsightTab::Elements, "6"),
            (InsightTab::FengShui, "7"),
            (InsightTab::Advanced, "8"),
            (InsightTab::Personal, "9"),
            (InsightTab::Bazi, "0"),
        ];

        let mut spans = Vec::new();
        for (i, (tab, key)) in tabs.iter().enumerate() {
            let style = if *tab == current {
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme::SECONDARY_FG)
            };
            spans.push(Span::styled(format!("[{key}]"), style));
            spans.push(Span::styled(tab.name(lang), style));
            if i < tabs.len() - 1 {
                spans.push(Span::raw(" "));
            }
        }

        Line::from(spans)
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
            InsightLang::Vi => " 0-9 tab · []/o/t/a/m đổi mục Bát Tự ",
            InsightLang::En => " 0-9 tabs · []/o/t/a/m switch Bazi view ",
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
