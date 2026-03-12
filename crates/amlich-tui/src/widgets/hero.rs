use crate::layout::LayoutMode;
use crate::state::AppState;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
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

impl Widget for HeroWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let Some(bundle) = &self.app.bundle else {
            return;
        };

        let solar_str = format!(
            "{} · {} THÁNG {} · {}",
            bundle.solar.day_of_week_name.to_uppercase(),
            bundle.solar.day,
            bundle.solar.month,
            bundle.solar.year
        );
        let lunar_str = format!("Âm lịch: {}", bundle.lunar.date_string);
        let verdict = self.app.hero_verdict();
        let summary = verdict
            .as_ref()
            .map(|item| item.summary.as_str())
            .unwrap_or("Chưa có tóm tắt khuyến nghị.");

        let spotlight = verdict
            .as_ref()
            .map(|item| match (&item.strongest_positive, &item.strongest_negative) {
                (Some(positive), Some(negative)) => {
                    format!("Nên mạnh: {positive} | Cần tránh: {negative}")
                }
                (Some(positive), None) => format!("Nên mạnh: {positive}"),
                (None, Some(negative)) => format!("Cần tránh: {negative}"),
                (None, None) => "Chưa có điểm nhấn khuyến nghị.".to_string(),
            })
            .unwrap_or_else(|| "Chưa có điểm nhấn khuyến nghị.".to_string());
        let identity_str = build_identity_row(bundle);

        let block = Block::default()
            .title(" Tóm Tắt ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));

        let lines = vec![
            Line::from(Span::styled(
                solar_str,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(Span::styled(lunar_str, Style::default().fg(Color::Cyan))),
            Line::from(Span::styled(
                summary,
                Style::default().fg(Color::Yellow),
            )),
            Line::from(Span::styled(
                spotlight,
                Style::default().fg(Color::Green),
            )),
            Line::from(Span::styled(
                identity_str,
                Style::default().fg(Color::Gray),
            )),
        ];

        Paragraph::new(lines)
            .block(block)
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}

fn build_identity_row(bundle: &amlich_api::v2::DayBundleDto) -> String {
    let mut facts = Vec::new();

    if let Some(canchi) = bundle.canchi.as_ref() {
        facts.push(canchi.day.full.clone());
    }
    if let Some(fortune) = bundle.day_fortune.as_ref() {
        facts.push(format!("Trực {}", fortune.truc.name));
    }
    if let Some(tiet_khi) = bundle.tiet_khi.as_ref() {
        facts.push(tiet_khi.name.clone());
    }
    if let Some(holiday_badge) = holiday_badge(bundle) {
        facts.push(holiday_badge);
    }

    if facts.is_empty() {
        "Chưa có dữ liệu định danh ngày.".to_string()
    } else {
        facts.join(" · ")
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

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::{
        ActivityLabelDto, CanChiDto, CanChiInfoDto, DailyRecommendationsDto, DayConflictDto,
        DayElementDto, DayFortuneDto, DayStarsDto, GioHoangDaoDto, HourInfoDto, LunarDto,
        NguHanhDto, RecommendationBucketDto, RecommendationEvidenceDto,
        RecommendationEvidenceSourceDto, RecommendationReasonDto, RecommendationScopeDto,
        RecommendationSeverityDto, SolarDto, SynthesizedRecommendationDto, TietKhiDto,
        TravelDirectionDto, TrucDto, XungHopDto,
    };
    use amlich_api::v2::{ApiMetaDto, DayBundleDto};
    use chrono::NaiveDate;
    use crate::state::{FocusLens, PageSection, ViewMode};

    fn sample_bundle() -> DayBundleDto {
        DayBundleDto {
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
            canchi: Some(CanChiInfoDto {
                day: CanChiDto {
                    can_index: 2,
                    chi_index: 6,
                    can: "Bính".to_string(),
                    chi: "Ngọ".to_string(),
                    full: "Bính Ngọ".to_string(),
                    con_giap: "Ngựa".to_string(),
                    ngu_hanh: NguHanhDto {
                        can: "Hỏa".to_string(),
                        chi: "Hỏa".to_string(),
                    },
                },
                month: CanChiDto {
                    can_index: 0,
                    chi_index: 2,
                    can: "Giáp".to_string(),
                    chi: "Dần".to_string(),
                    full: "Giáp Dần".to_string(),
                    con_giap: "Hổ".to_string(),
                    ngu_hanh: NguHanhDto {
                        can: "Mộc".to_string(),
                        chi: "Mộc".to_string(),
                    },
                },
                year: CanChiDto {
                    can_index: 2,
                    chi_index: 6,
                    can: "Bính".to_string(),
                    chi: "Ngọ".to_string(),
                    full: "Bính Ngọ".to_string(),
                    con_giap: "Ngựa".to_string(),
                    ngu_hanh: NguHanhDto {
                        can: "Hỏa".to_string(),
                        chi: "Hỏa".to_string(),
                    },
                },
                full: "Bính Ngọ".to_string(),
            }),
            tiet_khi: Some(TietKhiDto {
                index: 3,
                name: "Kinh Trập".to_string(),
                description: "Tiết khí thử nghiệm".to_string(),
                longitude: 345,
                current_longitude: 345.0,
                season: "Xuân".to_string(),
            }),
            gio_hoang_dao: Some(GioHoangDaoDto {
                day_chi: "Ngọ".to_string(),
                good_hour_count: 4,
                good_hours: vec![HourInfoDto {
                    hour_index: 0,
                    hour_chi: "Tý".to_string(),
                    time_range: "23:00 - 01:00".to_string(),
                    star: "Thanh Long".to_string(),
                    is_good: true,
                }],
                all_hours: vec![],
                summary: "Giờ đẹp đầu ngày".to_string(),
            }),
            day_fortune: Some(DayFortuneDto {
                ruleset_id: "test".to_string(),
                ruleset_version: "v1".to_string(),
                profile: "baseline".to_string(),
                day_element: DayElementDto {
                    na_am: "Thiên Hà Thủy".to_string(),
                    element: "Thủy".to_string(),
                    can_element: "Hỏa".to_string(),
                    chi_element: "Hỏa".to_string(),
                    evidence: None,
                },
                conflict: DayConflictDto {
                    opposing_chi: "Tý".to_string(),
                    opposing_con_giap: "Chuột".to_string(),
                    tuoi_xung: vec![],
                    sat_huong: "Bắc".to_string(),
                    evidence: None,
                },
                travel: TravelDirectionDto {
                    xuat_hanh_huong: "Đông Nam".to_string(),
                    tai_than: "Chính Nam".to_string(),
                    hy_than: "Đông Bắc".to_string(),
                    evidence: None,
                },
                stars: DayStarsDto {
                    cat_tinh: vec![],
                    sat_tinh: vec![],
                    day_star: None,
                    star_system: None,
                    evidence: None,
                    matched_rules: vec![],
                },
                day_deity: None,
                taboos: vec![],
                xung_hop: XungHopDto {
                    luc_xung: "Tý".to_string(),
                    tam_hop: vec![],
                    tu_hanh_xung: vec![],
                    liu_he: None,
                    xiang_hai: None,
                    xiang_xing: None,
                },
                truc: TrucDto {
                    index: 4,
                    name: "Khai".to_string(),
                    quality: "cat".to_string(),
                    evidence: None,
                },
                tang_can: None,
                ten_gods: None,
                tu_menh: None,
            }),
            daily_recommendations: Some(DailyRecommendationsDto {
                ruleset_id: "test".to_string(),
                ruleset_version: "v1".to_string(),
                profile: "baseline".to_string(),
                scope: RecommendationScopeDto::GeneralDay,
                version: "v1".to_string(),
                summary_vi: "Ngày thuận việc mở đầu, tránh việc lớn.".to_string(),
                summary_en: "Good for starting, avoid major matters.".to_string(),
                activities: vec![
                    SynthesizedRecommendationDto {
                        activity_id: "opening_start".to_string(),
                        label: ActivityLabelDto {
                            vi: "Khai mở".to_string(),
                            en: "Opening".to_string(),
                        },
                        bucket: RecommendationBucketDto::Nen,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "truc.khai.good".to_string(),
                            severity: RecommendationSeverityDto::Primary,
                            summary_vi: "Hợp trực Khai".to_string(),
                            summary_en: "Good under Khai".to_string(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::Truc,
                                code: "truc.khai".to_string(),
                                note: "test".to_string(),
                            },
                        }],
                    },
                    SynthesizedRecommendationDto {
                        activity_id: "groundbreaking".to_string(),
                        label: ActivityLabelDto {
                            vi: "Động thổ".to_string(),
                            en: "Groundbreaking".to_string(),
                        },
                        bucket: RecommendationBucketDto::KyManh,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "taboo.tam_nuong".to_string(),
                            severity: RecommendationSeverityDto::Override,
                            summary_vi: "Tam Nương kỵ việc động thổ".to_string(),
                            summary_en: "Tam Nuong strongly forbids groundbreaking".to_string(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::Taboo,
                                code: "taboo.tam_nuong".to_string(),
                                note: "test".to_string(),
                            },
                        }],
                    },
                ],
            }),
            insight: None,
        }
    }

    fn sample_app_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: Some(sample_bundle()),
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
        }
    }

    fn render_lines(app: &AppState) -> Vec<String> {
        let area = Rect::new(0, 0, 80, 8);
        let mut buf = Buffer::empty(area);
        HeroWidget::new(app, LayoutMode::Large).render(area, &mut buf);

        (0..area.height)
            .map(|y| {
                (0..area.width)
                    .map(|x| buf[(x, y)].symbol())
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect()
    }

    #[test]
    fn hero_shows_solar_lunar_and_summary_verdict() {
        let app = sample_app_state();
        let lines = render_lines(&app).join("\n");

        assert!(lines.contains("THỨ NĂM"));
        assert!(lines.contains("Mùng 4 tháng Hai"));
        assert!(lines.contains("Ngày thuận việc mở đầu, tránh việc lớn."));
    }

    #[test]
    fn hero_includes_key_identity_facts() {
        let app = sample_app_state();
        let lines = render_lines(&app).join("\n");

        assert!(lines.contains("Bính Ngọ"));
        assert!(lines.contains("Trực Khai"));
        assert!(lines.contains("Kinh Trập"));
    }

    #[test]
    fn hero_handles_missing_optional_badges() {
        let mut app = sample_app_state();
        app.bundle.as_mut().expect("bundle").tiet_khi = None;

        let lines = render_lines(&app).join("\n");

        assert!(lines.contains("Ngày thuận việc mở đầu, tránh việc lớn."));
        assert!(lines.contains("Khai mở"));
        assert!(!lines.contains("Lễ"));
    }
}
