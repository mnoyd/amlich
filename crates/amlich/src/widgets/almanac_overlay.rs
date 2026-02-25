use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::app::{AlmanacTab, App, InsightLang};
use crate::theme;

pub struct AlmanacOverlay<'a> {
    app: &'a App,
}

impl<'a> AlmanacOverlay<'a> {
    pub fn new(app: &'a App) -> Self {
        Self { app }
    }

    fn tab_style(&self, tab: AlmanacTab) -> Style {
        if self.app.almanac_tab == tab {
            Style::default()
                .fg(theme::ACCENT_FG)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(theme::SECONDARY_FG)
        }
    }

    fn tab_indicator(&self) -> Line<'_> {
        let lang = self.app.insight_lang;
        let t1 = self.tab_style(AlmanacTab::Overview);
        let t2 = self.tab_style(AlmanacTab::Taboos);
        let t3 = self.tab_style(AlmanacTab::Stars);
        let t4 = self.tab_style(AlmanacTab::Evidence);

        let n1 = AlmanacTab::Overview.name(lang);
        let n2 = AlmanacTab::Taboos.name(lang);
        let n3 = AlmanacTab::Stars.name(lang);
        let n4 = AlmanacTab::Evidence.name(lang);

        Line::from(vec![
            Span::styled("[1] ", t1),
            Span::styled(n1, t1),
            Span::raw("  "),
            Span::styled("[2] ", t2),
            Span::styled(n2, t2),
            Span::raw("  "),
            Span::styled("[3] ", t3),
            Span::styled(n3, t3),
            Span::raw("  "),
            Span::styled("[4] ", t4),
            Span::styled(n4, t4),
        ])
    }

    fn section_line(label: &str) -> Line<'static> {
        Line::from(Span::styled(format!("-- {label} "), theme::section_style()))
    }

    fn tr<'b>(&self, vi: &'b str, en: &'b str) -> &'b str {
        match self.app.insight_lang {
            InsightLang::Vi => vi,
            InsightLang::En => en,
        }
    }

    fn map_source_token(&self, token: &str) -> String {
        match token {
            "khcbppt" => self
                .tr("Khâm định hiệp kỷ biện phương thư", "KhCBBPT")
                .to_string(),
            "tam-menh-thong-hoi" => self
                .tr("Tam mệnh thông hội", "Tam Menh Thong Hoi")
                .to_string(),
            "nhi-thap-bat-tu" => self.tr("Nhị thập bát tú", "28 Mansions").to_string(),
            other => other.to_string(),
        }
    }

    fn map_method_token(&self, token: &str) -> String {
        match token {
            "table-lookup" => self.tr("Tra bảng", "Table lookup").to_string(),
            "bai-quyet" => self.tr("Bài quyết", "Verse rule").to_string(),
            "formula" => self.tr("Công thức", "Formula").to_string(),
            "jd-cycle" => self.tr("Chu kỳ JD", "JD cycle").to_string(),
            other => other.to_string(),
        }
    }

    fn map_profile_token(&self, token: &str) -> String {
        match token {
            "baseline" => self.tr("Mặc định", "Baseline").to_string(),
            other => other.to_string(),
        }
    }

    fn push_kv_line<'b>(lines: &mut Vec<Line<'b>>, label: &str, value: String) {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{label}: "),
                Style::default().fg(theme::SECONDARY_FG),
            ),
            Span::styled(value, Style::default().fg(theme::PRIMARY_FG)),
        ]));
    }

    fn push_wrapped_text<'b>(lines: &mut Vec<Line<'b>>, prefix: &str, value: String) {
        lines.push(Line::from(vec![
            Span::styled(prefix.to_string(), Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(value, Style::default().fg(theme::PRIMARY_FG)),
        ]));
    }

    fn no_fortune_message(&self) -> &'static str {
        match self.app.insight_lang {
            InsightLang::Vi => "Khong co du lieu almanac cho ngay nay",
            InsightLang::En => "No almanac data for this day",
        }
    }

    fn render_overview_tab(&self) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let Some(info) = self.app.selected_info() else {
            return vec![Line::from("Khong co du lieu")];
        };
        let Some(fortune) = &info.day_fortune else {
            return vec![Line::from(Span::styled(
                self.no_fortune_message(),
                Style::default().fg(theme::SECONDARY_FG),
            ))];
        };

        lines.push(Self::section_line("Overview"));
        lines.push(Line::from(vec![
            Span::styled("Ruleset: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                format!(
                    "{}@{} ({})",
                    fortune.ruleset_id, fortune.ruleset_version, fortune.profile
                ),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Truc: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                format!("{} [{}]", fortune.truc.name, fortune.truc.quality),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        if let Some(deity) = &fortune.day_deity {
            lines.push(Line::from(vec![
                Span::styled("Day deity: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    format!("{} ({})", deity.name, deity.classification),
                    Style::default().fg(theme::PRIMARY_FG),
                ),
            ]));
        }
        lines.push(Line::from(vec![
            Span::styled("Day element: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                format!(
                    "{} ({}) | can={} chi={}",
                    fortune.day_element.na_am,
                    fortune.day_element.element,
                    fortune.day_element.can_element,
                    fortune.day_element.chi_element
                ),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));

        lines.push(Line::from(""));
        lines.push(Self::section_line("Conflict / Harmony"));
        lines.push(Line::from(vec![
            Span::styled("Tuoi xung: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                fortune.conflict.tuoi_xung.join(", "),
                Style::default().fg(theme::BAD_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Luc xung: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                fortune.xung_hop.luc_xung.clone(),
                Style::default().fg(theme::BAD_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Tam hop: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                fortune.xung_hop.tam_hop.join(" · "),
                Style::default().fg(theme::GOOD_FG),
            ),
        ]));
        if !fortune.xung_hop.tu_hanh_xung.is_empty() {
            lines.push(Line::from(vec![
                Span::styled("Tu hanh xung: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    fortune.xung_hop.tu_hanh_xung.join(", "),
                    Style::default().fg(theme::BAD_FG),
                ),
            ]));
        }

        lines.push(Line::from(""));
        lines.push(Self::section_line("Travel"));
        lines.push(Line::from(vec![
            Span::styled("Xuat hanh: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                fortune.travel.xuat_hanh_huong.clone(),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Tai than: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                fortune.travel.tai_than.clone(),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Hy than: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                fortune.travel.hy_than.clone(),
                Style::default().fg(theme::PRIMARY_FG),
            ),
        ]));
        lines
    }

    fn render_taboos_tab(&self) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let Some(info) = self.app.selected_info() else {
            return vec![Line::from("Khong co du lieu")];
        };
        let Some(fortune) = &info.day_fortune else {
            return vec![Line::from(Span::styled(
                self.no_fortune_message(),
                Style::default().fg(theme::SECONDARY_FG),
            ))];
        };

        lines.push(Self::section_line("Taboos"));
        if fortune.taboos.is_empty() {
            lines.push(Line::from(Span::styled(
                "No taboos listed",
                Style::default().fg(theme::SECONDARY_FG),
            )));
            return lines;
        }

        for taboo in &fortune.taboos {
            let sev_style = match taboo.severity.as_str() {
                "high" => Style::default()
                    .fg(theme::BAD_FG)
                    .add_modifier(Modifier::BOLD),
                "medium" => Style::default().fg(theme::ACCENT_FG),
                _ => Style::default().fg(theme::SECONDARY_FG),
            };
            lines.push(Line::from(vec![
                Span::styled("• ", Style::default().fg(theme::BAD_FG)),
                Span::styled(taboo.name.clone(), Style::default().fg(theme::PRIMARY_FG)),
                Span::raw("  "),
                Span::styled(format!("[{}]", taboo.severity), sev_style),
            ]));
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    taboo.reason.clone(),
                    Style::default().fg(theme::SECONDARY_FG),
                ),
            ]));
        }

        lines
    }

    fn render_stars_tab(&self) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let Some(info) = self.app.selected_info() else {
            return vec![Line::from("Khong co du lieu")];
        };
        let Some(fortune) = &info.day_fortune else {
            return vec![Line::from(Span::styled(
                self.no_fortune_message(),
                Style::default().fg(theme::SECONDARY_FG),
            ))];
        };

        let stars = &fortune.stars;
        lines.push(Self::section_line("Stars"));
        if let Some(system) = &stars.star_system {
            lines.push(Line::from(vec![
                Span::styled("System: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(system.clone(), Style::default().fg(theme::PRIMARY_FG)),
            ]));
        }
        if let Some(day_star) = &stars.day_star {
            lines.push(Line::from(vec![
                Span::styled("Day star: ", Style::default().fg(theme::SECONDARY_FG)),
                Span::styled(
                    format!(
                        "{} ({}/{})",
                        day_star.name, day_star.system, day_star.quality
                    ),
                    Style::default().fg(theme::PRIMARY_FG),
                ),
            ]));
        }
        lines.push(Line::from(vec![
            Span::styled("Cat tinh: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                stars.cat_tinh.join(", "),
                Style::default().fg(theme::GOOD_FG),
            ),
        ]));
        lines.push(Line::from(vec![
            Span::styled("Sat tinh: ", Style::default().fg(theme::SECONDARY_FG)),
            Span::styled(
                stars.sat_tinh.join(", "),
                Style::default().fg(theme::BAD_FG),
            ),
        ]));

        if !stars.matched_rules.is_empty() {
            lines.push(Line::from(""));
            lines.push(Self::section_line("Matched rules"));
            for rule in stars.matched_rules.iter().take(12) {
                lines.push(Line::from(vec![
                    Span::styled("• ", Style::default().fg(theme::ACCENT_FG)),
                    Span::styled(
                        format!("{} [{} / {}]", rule.name, rule.category, rule.quality),
                        Style::default().fg(theme::PRIMARY_FG),
                    ),
                ]));
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        format!("{} · {} · {}", rule.source_id, rule.method, rule.profile),
                        Style::default().fg(theme::SECONDARY_FG),
                    ),
                ]));
            }
        }

        lines
    }

    fn push_evidence_item<'b>(
        lines: &mut Vec<Line<'b>>,
        label: String,
        ev: Option<&amlich_api::RuleEvidenceDto>,
        raw_mode: bool,
        overlay: &AlmanacOverlay<'_>,
    ) {
        if let Some(ev) = ev {
            lines.push(Line::from(vec![
                Span::styled("• ", Style::default().fg(theme::ACCENT_FG)),
                Span::styled(
                    label,
                    Style::default()
                        .fg(theme::PRIMARY_FG)
                        .add_modifier(Modifier::BOLD),
                ),
            ]));
            if raw_mode {
                Self::push_wrapped_text(
                    lines,
                    "  raw: ",
                    format!("{} · {} · {}", ev.source_id, ev.method, ev.profile),
                );
            } else {
                Self::push_wrapped_text(
                    lines,
                    "  nguồn: ",
                    overlay.map_source_token(&ev.source_id),
                );
                Self::push_wrapped_text(
                    lines,
                    "  cách tính: ",
                    overlay.map_method_token(&ev.method),
                );
                Self::push_wrapped_text(lines, "  hồ sơ: ", overlay.map_profile_token(&ev.profile));
            }
        }
    }

    fn render_evidence_tab(&self) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        let Some(info) = self.app.selected_info() else {
            return vec![Line::from("Khong co du lieu")];
        };
        let Some(fortune) = &info.day_fortune else {
            return vec![Line::from(Span::styled(
                self.no_fortune_message(),
                Style::default().fg(theme::SECONDARY_FG),
            ))];
        };

        let raw_mode = self.app.almanac_evidence_raw;
        let ruleset_title = self.tr("Nguồn dữ liệu", "Ruleset provenance");
        lines.push(Self::section_line(ruleset_title));
        Self::push_kv_line(
            &mut lines,
            self.tr("DayInfo", "DayInfo"),
            format!("{}@{}", info.ruleset_id, info.ruleset_version),
        );
        Self::push_kv_line(
            &mut lines,
            self.tr("Day fortune", "Day fortune"),
            format!(
                "{}@{} ({})",
                fortune.ruleset_id,
                fortune.ruleset_version,
                if raw_mode {
                    fortune.profile.clone()
                } else {
                    self.map_profile_token(&fortune.profile)
                }
            ),
        );
        lines.push(Line::from(Span::styled(
            if raw_mode {
                self.tr("[r] Chế độ raw: bật", "[r] Raw mode: on")
            } else {
                self.tr("[r] Chế độ raw: tắt", "[r] Raw mode: off")
            },
            Style::default().fg(theme::SECONDARY_FG),
        )));

        lines.push(Line::from(""));
        lines.push(Self::section_line(
            self.tr("Cách tính theo mục", "Per-field evidence"),
        ));
        Self::push_evidence_item(
            &mut lines,
            self.tr("Mệnh ngày", "Day element").to_string(),
            fortune.day_element.evidence.as_ref(),
            raw_mode,
            self,
        );
        Self::push_evidence_item(
            &mut lines,
            self.tr("Xung khắc", "Conflict").to_string(),
            fortune.conflict.evidence.as_ref(),
            raw_mode,
            self,
        );
        Self::push_evidence_item(
            &mut lines,
            self.tr("Xuất hành / thần hướng", "Travel directions")
                .to_string(),
            fortune.travel.evidence.as_ref(),
            raw_mode,
            self,
        );
        Self::push_evidence_item(
            &mut lines,
            self.tr("Trực", "Truc").to_string(),
            fortune.truc.evidence.as_ref(),
            raw_mode,
            self,
        );
        Self::push_evidence_item(
            &mut lines,
            self.tr("Sao", "Stars").to_string(),
            fortune.stars.evidence.as_ref(),
            raw_mode,
            self,
        );
        if let Some(deity) = &fortune.day_deity {
            Self::push_evidence_item(
                &mut lines,
                self.tr("Thần ngày", "Day deity").to_string(),
                deity.evidence.as_ref(),
                raw_mode,
                self,
            );
        }
        if let Some(day_star) = &fortune.stars.day_star {
            Self::push_evidence_item(
                &mut lines,
                self.tr("Sao chủ", "Day star").to_string(),
                day_star.evidence.as_ref(),
                raw_mode,
                self,
            );
        }

        if !fortune.taboos.is_empty() {
            lines.push(Line::from(""));
            lines.push(Self::section_line(
                self.tr("Quy tắc kiêng kỵ đang áp dụng", "Applied taboo rules"),
            ));
            for taboo in fortune.taboos.iter().take(20) {
                lines.push(Line::from(vec![
                    Span::styled("• ", Style::default().fg(theme::ACCENT_FG)),
                    Span::styled(taboo.name.clone(), Style::default().fg(theme::PRIMARY_FG)),
                ]));
                if raw_mode {
                    Self::push_wrapped_text(&mut lines, "  rule_id: ", taboo.rule_id.clone());
                } else {
                    Self::push_wrapped_text(&mut lines, "  mã quy tắc: ", taboo.rule_id.clone());
                }
            }
        }

        if lines.len() <= 3 {
            lines.push(Line::from(Span::styled(
                self.tr(
                    "Không có dữ liệu nguồn/cách tính",
                    "No evidence metadata available",
                ),
                Style::default().fg(theme::SECONDARY_FG),
            )));
        }

        lines
    }

    fn tab_content(&self) -> Vec<Line<'_>> {
        match self.app.almanac_tab {
            AlmanacTab::Overview => self.render_overview_tab(),
            AlmanacTab::Taboos => self.render_taboos_tab(),
            AlmanacTab::Stars => self.render_stars_tab(),
            AlmanacTab::Evidence => self.render_evidence_tab(),
        }
    }
}

impl Widget for AlmanacOverlay<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = area.width.clamp(52, 90);
        let height = area.height.clamp(14, area.height.saturating_sub(2));
        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 2;
        let overlay_area = Rect::new(x, y, width, height);

        Clear.render(overlay_area, buf);

        let title_lang = match self.app.insight_lang {
            InsightLang::Vi => "VI",
            InsightLang::En => "EN",
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(theme::border_style())
            .title(Line::from(Span::styled(
                format!(" Almanac ({title_lang}) "),
                theme::section_style(),
            )))
            .title_bottom(
                Line::from(vec![
                    Span::styled(" a/Esc close ", Style::default().fg(theme::ACCENT_FG)),
                    Span::raw(" "),
                    Span::styled(" j/k scroll ", Style::default().fg(theme::ACCENT_FG)),
                    Span::raw(" "),
                    Span::styled(" 1-4 tabs ", Style::default().fg(theme::ACCENT_FG)),
                    Span::raw(" "),
                    Span::styled(" r raw ", Style::default().fg(theme::ACCENT_FG)),
                ])
                .alignment(Alignment::Center),
            );

        let mut content = vec![Line::from(""), self.tab_indicator(), Line::from("")];
        content.extend(self.tab_content());

        Paragraph::new(content)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((self.app.almanac_scroll, 0))
            .render(overlay_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::app::{AlmanacTab, App};

    use super::AlmanacOverlay;

    fn lines_text(lines: Vec<ratatui::text::Line<'_>>) -> String {
        lines
            .into_iter()
            .map(|line| {
                line.spans
                    .into_iter()
                    .map(|s| s.content.to_string())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn overview_tab_renders_ruleset_and_truc() {
        let mut app = App::new_with_date(Some(
            NaiveDate::from_ymd_opt(2024, 2, 10).expect("valid date"),
        ));
        app.almanac_tab = AlmanacTab::Overview;
        let text = lines_text(AlmanacOverlay::new(&app).tab_content());
        assert!(text.contains("Ruleset:"));
        assert!(text.contains("Truc:"));
    }

    #[test]
    fn evidence_tab_renders_provenance_section() {
        let mut app = App::new_with_date(Some(
            NaiveDate::from_ymd_opt(2024, 2, 10).expect("valid date"),
        ));
        app.almanac_tab = AlmanacTab::Evidence;
        let text = lines_text(AlmanacOverlay::new(&app).tab_content());
        assert!(text.contains("Nguồn dữ liệu"));
        assert!(text.contains("Day fortune:"));
    }

    #[test]
    fn evidence_tab_renders_human_labels_by_default() {
        let mut app = App::new_with_date(Some(
            NaiveDate::from_ymd_opt(2024, 2, 10).expect("valid date"),
        ));
        app.almanac_tab = AlmanacTab::Evidence;
        let text = lines_text(AlmanacOverlay::new(&app).tab_content());
        assert!(text.contains("cách tính"));
        assert!(
            text.contains("Tra bảng") || text.contains("Bài quyết") || text.contains("Công thức")
        );
    }
}
