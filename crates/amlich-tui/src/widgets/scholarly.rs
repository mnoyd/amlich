use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::layout::LayoutMode;
use crate::state::AppState;
use amlich_api::v2::DayBundleDto;

pub struct ScholarlyWidget<'a> {
    app: &'a AppState,
    _mode: LayoutMode,
}

impl<'a> ScholarlyWidget<'a> {
    pub fn new(app: &'a AppState, mode: LayoutMode) -> Self {
        Self { app, _mode: mode }
    }
}

impl Widget for ScholarlyWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };
        self.render_evidence(area, buf, bundle);
    }
}

impl<'a> ScholarlyWidget<'a> {
    fn render_evidence(&self, area: Rect, buf: &mut Buffer, bundle: &DayBundleDto) {
        let mut lines: Vec<Line<'_>> = vec![];


        let block = Block::default()
            .title(" Chứng Cứ Truyền Thống ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
            
        let inner = block.inner(area);
        block.render(area, buf);

        if let Some(insight) = &bundle.insight {
            if let Some(truc) = &insight.truc {
                lines.push(Line::from(vec![
                    Span::raw("   Trực: "),
                    Span::styled(&truc.name, Style::default().fg(Color::Cyan)),
                    Span::raw(" ("),
                    Span::raw(&truc.quality),
                    Span::raw(")"),
                ]));
            }

            if let Some(stars) = &insight.stars {
                let cat_tinh = stars.cat_tinh.join(", ");
                let cat_str = if stars.cat_tinh.is_empty() {
                    "Không"
                } else {
                    &cat_tinh
                };
                lines.push(Line::from(vec![
                    Span::raw("   Cát tinh: "),
                    Span::styled(cat_str.to_string(), Style::default().fg(Color::Green)),
                ]));

                let sat_tinh = stars.sat_tinh.join(", ");
                let sat_str = if stars.sat_tinh.is_empty() {
                    "Không"
                } else {
                    &sat_tinh
                };
                lines.push(Line::from(vec![
                    Span::raw("   Sát tinh: "),
                    Span::styled(sat_str.to_string(), Style::default().fg(Color::Red)),
                ]));
            }

            if let Some(deity) = &insight.day_deity {
                lines.push(Line::from(vec![
                    Span::raw("   Thần sát: "),
                    Span::styled(&deity.name, Style::default().fg(Color::Yellow)),
                    Span::raw(" ("),
                    Span::raw(&deity.classification),
                    Span::raw(")"),
                ]));
            }
        } else {
            lines.push(Line::from("   Chưa có dữ liệu chứng cứ truyền thống."));
        }

        Paragraph::new(lines).render(inner, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::{
        DayDeityInsightDto, DayInsightDto, LunarDto, LocalizedListDto, LocalizedTextDto, SolarDto,
        StarsInsightDto, TrucInsightDto,
    };
    use amlich_api::v2::{ApiMetaDto, DayBundleDto};
    use chrono::NaiveDate;
    use crate::state::{FocusLens, PageSection, ViewMode};

    fn sample_app_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: Some(DayBundleDto {
                meta: ApiMetaDto {
                    schema_version: "amlich.api/v2".to_string(),
                    ruleset_id: "test".to_string(),
                    ruleset_version: "v1".to_string(),
                    profile: "baseline".to_string(),
                    generated_at: "2026-03-12T00:00:00Z".to_string(),
                },
                solar: SolarDto {
                    day: 12,
                    month: 3,
                    year: 2026,
                    day_of_week: 4,
                    day_of_week_name: "Thứ Năm".to_string(),
                    date_string: "2026-03-12".to_string(),
                },
                lunar: LunarDto {
                    day: 4,
                    month: 2,
                    year: 2026,
                    is_leap_month: false,
                    date_string: "Mùng 4 tháng Hai".to_string(),
                },
                jd: 0,
                canchi: None,
                tiet_khi: None,
                gio_hoang_dao: None,
                day_fortune: None,
                daily_recommendations: None,
                insight: Some(DayInsightDto {
                    solar: SolarDto {
                        day: 12,
                        month: 3,
                        year: 2026,
                        day_of_week: 4,
                        day_of_week_name: "Thứ Năm".to_string(),
                        date_string: "2026-03-12".to_string(),
                    },
                    lunar: LunarDto {
                        day: 4,
                        month: 2,
                        year: 2026,
                        is_leap_month: false,
                        date_string: "Mùng 4 tháng Hai".to_string(),
                    },
                    festival: None,
                    holiday: None,
                    canchi: None,
                    day_guidance: None,
                    tiet_khi: None,
                    na_am: None,
                    truc: Some(TrucInsightDto {
                        name: "Khai".to_string(),
                        quality: "cat".to_string(),
                        meaning: LocalizedTextDto {
                            vi: "Tốt cho mở đầu".to_string(),
                            en: "Good for opening".to_string(),
                        },
                        good_for: LocalizedListDto {
                            vi: vec![],
                            en: vec![],
                        },
                        avoid_for: LocalizedListDto {
                            vi: vec![],
                            en: vec![],
                        },
                    }),
                    day_deity: Some(DayDeityInsightDto {
                        name: "Kim Quỹ".to_string(),
                        classification: "hoang_dao".to_string(),
                        classification_meaning: LocalizedTextDto {
                            vi: "Cát thần".to_string(),
                            en: "Good deity".to_string(),
                        },
                        deity_meaning: None,
                    }),
                    stars: Some(StarsInsightDto {
                        cat_tinh: vec!["Thiên Đức".to_string()],
                        sat_tinh: vec!["Thiên Cương".to_string()],
                        day_star: Some("Kim Quỹ".to_string()),
                        day_star_quality: Some("cat".to_string()),
                    }),
                    taboos: None,
                    travel: None,
                    xung_hop: None,
                    tang_can: None,
                    ten_gods: None,
                    hours: None,
                    tu_menh: None,
                    dai_van: None,
                }),
            }),
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            focused_section: PageSection::TraditionalEvidence,
            zoomed_section: None,
            expanded_sections: Default::default(),
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
        }
    }

    fn render_text(app: &AppState) -> String {
        let area = Rect::new(0, 0, 90, 8);
        let mut buf = Buffer::empty(area);
        ScholarlyWidget::new(app, LayoutMode::Large).render(area, &mut buf);
        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn scholarly_widget_groups_truc_stars_and_deity_as_evidence() {
        let app = sample_app_state();
        let text = render_text(&app);

        assert!(text.contains("Chứng Cứ Truyền Thống"));
        assert!(text.contains("Trực:"));
        assert!(text.contains("Cát tinh:"));
        assert!(text.contains("Thần sát:"));
    }
}
