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

    fn render_almanac_tab(&self, insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let lang = self.app.insight_lang;

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

    fn render_advanced_tab(&self, insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let lang = self.app.insight_lang;

        if let Some(ten_gods) = &insight.ten_gods {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Thập Thần:", "Ten Gods:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            if let Some(entry) = &ten_gods.to_year_stem {
                lines.push(Line::from(format!(
                    "  {} — {}",
                    entry.label,
                    pick_text(lang, &entry.meaning.vi, &entry.meaning.en),
                )));
            }
            if let Some(entry) = &ten_gods.to_self {
                lines.push(Line::from(format!(
                    "  {} — {}",
                    entry.label,
                    pick_text(lang, &entry.meaning.vi, &entry.meaning.en),
                )));
            }
            lines.push(Line::from(""));
        }

        if let Some(travel) = &insight.travel {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Xuất hành:", "Travel:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(format!(
                "  {} {} | {} {} | {} {}",
                pick_text(lang, "Hướng:", "Direction:"),
                travel.xuat_hanh_huong,
                pick_text(lang, "Tài Thần:", "Wealth:"),
                travel.tai_than,
                pick_text(lang, "Hỷ Thần:", "Joy:"),
                travel.hy_than,
            )));
            lines.push(Line::from(""));
        }

        if let Some(xung_hop) = &insight.xung_hop {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Xung Hợp:", "Clash/Harmony:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(format!(
                "  {} {}",
                pick_text(lang, "Lục Xung:", "Six Clash:"),
                xung_hop.luc_xung,
            )));
            if !xung_hop.tam_hop.is_empty() {
                lines.push(Line::from(format!(
                    "  {} {}",
                    pick_text(lang, "Tam Hợp:", "Three Harmony:"),
                    xung_hop.tam_hop.join(", "),
                )));
            }
            if let Some(liu_he) = &xung_hop.liu_he {
                lines.push(Line::from(format!(
                    "  {} {liu_he}",
                    pick_text(lang, "Lục Hợp:", "Six Harmony:"),
                )));
            }
            lines.push(Line::from(""));
        }

        if let Some(tang_can) = &insight.tang_can {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Tàng Can:", "Hidden Stems:"),
                Style::default()
                    .fg(theme::ACCENT_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(format!(
                "  {} {} ({}) | {} ({}) | {} ({})",
                pick_text(lang, "Chính:", "Main:"),
                tang_can.main,
                tang_can.strength[0],
                tang_can.central,
                tang_can.strength[1],
                tang_can.residual,
                tang_can.strength[2],
            )));
            lines.push(Line::from(""));
        }

        if let Some(hours) = &insight.hours {
            lines.push(Line::from(Span::styled(
                format!(
                    "{} ({})",
                    pick_text(lang, "Giờ tốt:", "Good hours:"),
                    hours.good_hour_count,
                ),
                Style::default()
                    .fg(theme::GOOD_HOUR_FG)
                    .add_modifier(Modifier::BOLD),
            )));
            for h in &hours.good_hours {
                lines.push(Line::from(format!(
                    "  {} ({}) — {}",
                    h.chi, h.time_range, h.star
                )));
            }
        }

        if lines.is_empty() {
            lines.push(Line::from(Span::styled(
                pick_text(lang, "Không có dữ liệu nâng cao", "No advanced data"),
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines
    }

    fn render_personal_tab(&self, insight: &amlich_api::DayInsightDto) -> Vec<Line<'_>> {
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
            InsightTab::Advanced => self.render_advanced_tab(insight),
            InsightTab::Personal => self.render_personal_tab(insight),
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
            (InsightTab::Advanced, "5"),
            (InsightTab::Personal, "6"),
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
            spans.push(Span::styled(format!("[{key}] "), style));
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
            InsightLang::Vi => " 1-6 tab ",
            InsightLang::En => " 1-6 tabs ",
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
