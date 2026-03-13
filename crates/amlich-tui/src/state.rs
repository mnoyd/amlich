use std::collections::BTreeSet;

use amlich_api::{
    DailyRecommendationsDto, RecommendationBucketDto, RecommendationEvidenceSourceDto,
    RecommendationSeverityDto,
};
use amlich_api::v2::{get_day_bundle_for_date, DayBundleDto, Include};
use chrono::{Datelike, Local, NaiveDate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusLens {
    General,
    Planning,
    Scholarly,
    Personal,
}

impl FocusLens {
    pub fn next(&self) -> Self {
        match self {
            Self::General => Self::Planning,
            Self::Planning => Self::Scholarly,
            Self::Scholarly => Self::Personal,
            Self::Personal => Self::General,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Day,
    Calendar,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageSection {
    Hero,
    Recommendations,
    Timing,
    Travel,
    Risks,
    TraditionalEvidence,
    ExpandedDetails,
}

impl PageSection {
    pub fn next(&self) -> Self {
        match self {
            Self::Hero => Self::Recommendations,
            Self::Recommendations => Self::Timing,
            Self::Timing => Self::Travel,
            Self::Travel => Self::Risks,
            Self::Risks => Self::TraditionalEvidence,
            Self::TraditionalEvidence => Self::ExpandedDetails,
            Self::ExpandedDetails => Self::Hero,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationRowVm {
    pub bucket: RecommendationBucketDto,
    pub label: String,
    pub reason_chip: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeroVerdictVm {
    pub summary: String,
    pub strongest_positive: Option<String>,
    pub strongest_negative: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskSummaryVm {
    pub items: Vec<String>,
}

pub struct AppState {
    pub running: bool,
    pub date: NaiveDate,
    pub lens: FocusLens,
    pub view_mode: ViewMode,
    pub scroll_offset: u16,

    // Data cache for the current date
    pub bundle: Option<DayBundleDto>,
    pub is_loading: bool,
    pub error_msg: Option<String>,

    // UI toggles
    pub show_guidance_details: bool,
    pub show_tietkhi_details: bool,
    pub show_evidence: bool,
    pub focused_section: PageSection,
    pub zoomed_section: Option<PageSection>,
    pub expanded_sections: BTreeSet<PageSection>,
    pub show_search: bool,
    pub search_input: String,
    pub calendar_cursor: NaiveDate,
}

impl AppState {
    pub fn new(initial_date: Option<NaiveDate>) -> Self {
        let date = initial_date.unwrap_or_else(|| Local::now().naive_local().date());

        let mut app = Self {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: None,
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: BTreeSet::new(),
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
        };

        app.load_data();
        app
    }

    pub fn load_data(&mut self) {
        self.is_loading = true;
        self.error_msg = None;

        // In the future this might be done asynchronously if we want non-blocking UI
        // But for now we just do it synchronously like the old app
        let includes = vec![
            Include::Base,
            Include::CanChi,
            Include::TietKhi,
            Include::Hours,
            Include::Fortune,
            Include::Insight,
        ];

        match get_day_bundle_for_date(
            self.date.day() as i32,
            self.date.month() as i32,
            self.date.year(),
            &includes,
            None,
        ) {
            Ok(bundle) => {
                self.bundle = Some(bundle);
                self.is_loading = false;
            }
            Err(e) => {
                self.error_msg = Some(e);
                self.is_loading = false;
            }
        }
    }

    pub fn next_day(&mut self) {
        if let Some(next) = self.date.succ_opt() {
            self.date = next;
            self.scroll_offset = 0;
            self.load_data();
        }
    }

    pub fn prev_day(&mut self) {
        if let Some(prev) = self.date.pred_opt() {
            self.date = prev;
            self.scroll_offset = 0;
            self.load_data();
        }
    }

    pub fn go_today(&mut self) {
        self.date = Local::now().naive_local().date();
        self.scroll_offset = 0;
        self.load_data();
    }

    pub fn next_lens(&mut self) {
        self.lens = self.lens.next();
        self.scroll_offset = 0; // Reset scroll on lens change
    }

    pub fn focus_next_section(&mut self) {
        self.focused_section = self.focused_section.next();
        self.scroll_offset = 0;
    }

    pub fn focus_section(&mut self, section: PageSection) {
        self.focused_section = section;
        self.scroll_offset = 0;
    }

    pub fn toggle_calendar(&mut self) {
        self.toggle_calendar_view();
    }

    pub fn is_calendar_view(&self) -> bool {
        self.view_mode == ViewMode::Calendar
    }

    pub fn toggle_calendar_view(&mut self) {
        if self.is_calendar_view() {
            self.close_calendar_view();
        } else {
            self.open_calendar_view();
        }
    }

    pub fn open_calendar_view(&mut self) {
        self.view_mode = ViewMode::Calendar;
        self.calendar_cursor = self.date;
        self.scroll_offset = 0;
    }

    pub fn close_calendar_view(&mut self) {
        self.view_mode = ViewMode::Day;
    }

    pub fn apply_calendar_selection(&mut self) {
        self.date = self.calendar_cursor;
        self.view_mode = ViewMode::Day;
        self.scroll_offset = 0;
        self.load_data();
    }

    pub fn calendar_move_days(&mut self, delta_days: i64) {
        if let Some(next) = self
            .calendar_cursor
            .checked_add_signed(chrono::Duration::days(delta_days))
        {
            self.calendar_cursor = next;
        }
    }

    pub fn calendar_go_today(&mut self) {
        self.calendar_cursor = Local::now().naive_local().date();
    }

    pub fn calendar_prev_month(&mut self) {
        self.calendar_shift_month(-1);
    }

    pub fn calendar_next_month(&mut self) {
        self.calendar_shift_month(1);
    }

    fn calendar_shift_month(&mut self, delta_months: i32) {
        let current = self.calendar_cursor;
        let total_months = current.year() * 12 + current.month0() as i32 + delta_months;
        let target_year = total_months.div_euclid(12);
        let target_month0 = total_months.rem_euclid(12) as u32;
        let target_month = target_month0 + 1;

        let clamped_day = current.day().min(days_in_month(target_year, target_month));
        if let Some(next) = NaiveDate::from_ymd_opt(target_year, target_month, clamped_day) {
            self.calendar_cursor = next;
        }
    }

    pub fn toggle_tietkhi(&mut self) {
        self.show_tietkhi_details = !self.show_tietkhi_details;
        self.set_section_expanded(
            PageSection::TraditionalEvidence,
            self.show_tietkhi_details,
        );
    }

    pub fn toggle_guidance_details(&mut self) {
        self.show_guidance_details = !self.show_guidance_details;
        self.set_section_expanded(PageSection::Recommendations, self.show_guidance_details);
    }

    pub fn toggle_evidence(&mut self) {
        self.show_evidence = !self.show_evidence;
    }

    pub fn toggle_zoom_for_focused_section(&mut self) {
        if self.zoomed_section == Some(self.focused_section) {
            self.zoomed_section = None;
        } else {
            self.zoomed_section = Some(self.focused_section);
        }
    }

    pub fn toggle_expand_focused_section(&mut self) {
        let is_expanded = self.is_section_expanded(self.focused_section);
        self.set_section_expanded(self.focused_section, !is_expanded);
    }

    pub fn is_section_expanded(&self, section: PageSection) -> bool {
        self.expanded_sections.contains(&section)
    }

    pub fn expand_section(&mut self, section: PageSection) {
        self.focus_section(section);
        self.set_section_expanded(section, true);
    }

    fn selected_recommendations(&self) -> Option<&amlich_api::DailyRecommendationsDto> {
        let bundle = self.bundle.as_ref()?;
        bundle
            .contextual_recommendations
            .as_ref()
            .or(bundle.daily_recommendations.as_ref())
    }

    pub fn top_recommendation_rows(&self) -> Vec<RecommendationRowVm> {
        let Some(recommendations) = self.selected_recommendations() else {
            return Vec::new();
        };

        recommendation_bucket_order()
            .into_iter()
            .filter_map(|bucket| top_row_for_bucket(recommendations, bucket))
            .collect()
    }

    pub fn hero_verdict(&self) -> Option<HeroVerdictVm> {
        let recommendations = self.selected_recommendations()?;
        let rows = self.top_recommendation_rows();
        let strongest_positive = rows
            .iter()
            .find(|row| {
                matches!(
                    row.bucket,
                    RecommendationBucketDto::Nen | RecommendationBucketDto::CoThe
                )
            })
            .map(|row| row.label.clone());
        let strongest_negative = rows
            .iter()
            .find(|row| row.bucket == RecommendationBucketDto::KyManh)
            .or_else(|| {
                rows.iter()
                    .find(|row| row.bucket == RecommendationBucketDto::Tranh)
            })
            .map(|row| row.label.clone());
        let summary = if recommendations.summary_vi.trim().is_empty() {
            strongest_positive
                .clone()
                .or_else(|| strongest_negative.clone())
                .unwrap_or_default()
        } else {
            recommendations.summary_vi.clone()
        };

        Some(HeroVerdictVm {
            summary,
            strongest_positive,
            strongest_negative,
        })
    }

    pub fn risk_summary(&self) -> RiskSummaryVm {
        let mut items = Vec::new();
        for row in self.top_recommendation_rows() {
            if row.bucket == RecommendationBucketDto::KyManh {
                items.push(format!("Kỵ mạnh: {}", row.label));
            }
        }

        if let Some(fortune) = self
            .bundle
            .as_ref()
            .and_then(|bundle| bundle.day_fortune.as_ref())
        {
            for taboo in &fortune.taboos {
                items.push(format!("Kiêng kỵ: {}", taboo.name));
            }
            items.push(format!("Lục xung: {}", fortune.xung_hop.luc_xung));
            items.push(format!("Sát hướng: {}", fortune.conflict.sat_huong));
        }

        RiskSummaryVm { items }
    }

    pub fn sensitive_domain_notice(&self) -> Option<String> {
        let recommendations = self.selected_recommendations()?;
        let has_medical = recommendations
            .activities
            .iter()
            .any(|activity| activity.activity_id == "medical_treatment");
        let has_burial = recommendations
            .activities
            .iter()
            .any(|activity| activity.activity_id == "burial_memorial");

        let mut notes = Vec::new();
        if has_medical {
            notes.push(
                "Lưu ý: điều trị thực tế luôn ưu tiên đánh giá chuyên môn; lịch chỉ mang tính tham khảo."
                    .to_string(),
            );
        }
        if has_burial {
            notes.push(
                "Lưu ý: an táng hoặc tưởng niệm cần thẩm định thêm theo tập tục và chuyên gia địa phương."
                    .to_string(),
            );
        }

        if notes.is_empty() {
            None
        } else {
            Some(notes.join(" "))
        }
    }

    pub fn toggle_search(&mut self) {
        self.show_search = !self.show_search;
        if self.show_search {
            self.search_input.clear();
        }
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    // We can add page up/down later by exposing the viewport height to the state,
    // or passing the step amount from the event handler
    pub fn scroll_down_by(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_add(lines);
    }

    pub fn scroll_up_by(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    fn set_section_expanded(&mut self, section: PageSection, expanded: bool) {
        if expanded {
            self.expanded_sections.insert(section);
        } else {
            self.expanded_sections.remove(&section);
        }

        match section {
            PageSection::Recommendations => self.show_guidance_details = expanded,
            PageSection::TraditionalEvidence => self.show_tietkhi_details = expanded,
            _ => {}
        }
    }
}

fn recommendation_bucket_order() -> [RecommendationBucketDto; 4] {
    [
        RecommendationBucketDto::Nen,
        RecommendationBucketDto::CoThe,
        RecommendationBucketDto::Tranh,
        RecommendationBucketDto::KyManh,
    ]
}

fn top_row_for_bucket(
    recommendations: &DailyRecommendationsDto,
    bucket: RecommendationBucketDto,
) -> Option<RecommendationRowVm> {
    let activity = recommendations
        .activities
        .iter()
        .find(|activity| activity.bucket == bucket)?;
    let reason_chip = activity
        .reasons
        .iter()
        .min_by_key(|reason| severity_rank(reason.severity))
        .map(|reason| {
            format!(
                "{} • {}",
                severity_label(reason.severity),
                source_label(reason.evidence.source)
            )
        });

    Some(RecommendationRowVm {
        bucket,
        label: activity.label.vi.clone(),
        reason_chip,
    })
}

fn severity_rank(severity: RecommendationSeverityDto) -> u8 {
    match severity {
        RecommendationSeverityDto::Override => 0,
        RecommendationSeverityDto::Primary => 1,
        RecommendationSeverityDto::Supporting => 2,
    }
}

fn severity_label(severity: RecommendationSeverityDto) -> &'static str {
    match severity {
        RecommendationSeverityDto::Override => "override",
        RecommendationSeverityDto::Primary => "primary",
        RecommendationSeverityDto::Supporting => "support",
    }
}

fn source_label(source: RecommendationEvidenceSourceDto) -> &'static str {
    match source {
        RecommendationEvidenceSourceDto::DayGuidance => "guidance",
        RecommendationEvidenceSourceDto::Truc => "trực",
        RecommendationEvidenceSourceDto::Stars => "sao",
        RecommendationEvidenceSourceDto::DayDeity => "thần sát",
        RecommendationEvidenceSourceDto::Taboo => "kiêng kỵ",
        RecommendationEvidenceSourceDto::XungHop => "xung-hợp",
        RecommendationEvidenceSourceDto::TietKhi => "tiết khí",
        RecommendationEvidenceSourceDto::GioHoangDao => "giờ tốt",
        RecommendationEvidenceSourceDto::Travel => "xuất hành",
        RecommendationEvidenceSourceDto::ProductRule => "mở rộng",
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    let next_month_start =
        NaiveDate::from_ymd_opt(next_year, next_month, 1).expect("valid next month date");
    next_month_start
        .pred_opt()
        .expect("previous day exists")
        .day()
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::{
        ActivityLabelDto, CanChiDto, CanChiInfoDto, DailyRecommendationsDto, DayConflictDto,
        DayElementDto, DayFortuneDto, DayStarsDto, DayTabooDto, GioHoangDaoDto, HourInfoDto,
        LunarDto, NguHanhDto, RecommendationBucketDto, RecommendationEvidenceDto,
        RecommendationEvidenceSourceDto, RecommendationReasonDto, RecommendationScopeDto,
        RecommendationSeverityDto, RuleEvidenceDto, SolarDto, SynthesizedRecommendationDto,
        TietKhiDto, TravelDirectionDto, TrucDto, XungHopDto,
    };
    use amlich_api::v2::DayBundleDto;

    fn sample_app_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        AppState {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: None,
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

    fn sample_bundle() -> DayBundleDto {
        DayBundleDto {
            schema_version: "amlich.engine/v1".to_string(),
                ruleset_id: "test".to_string(),
                ruleset_version: "v1".to_string(),
                profile: "baseline".to_string(),
                generated_at: "2026-03-12T00:00:00Z".to_string(),
                
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
                    tuoi_xung: vec!["Canh Tý".to_string()],
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
                    cat_tinh: vec!["Thiên Đức".to_string()],
                    sat_tinh: vec!["Thiên Cương".to_string()],
                    day_star: None,
                    star_system: None,
                    evidence: None,
                    matched_rules: vec![],
                },
                day_deity: None,
                taboos: vec![DayTabooDto {
                    rule_id: "taboo.tam_nuong".to_string(),
                    name: "Tam Nương".to_string(),
                    severity: "high".to_string(),
                    reason: "Không hợp việc lớn".to_string(),
                    evidence: Some(RuleEvidenceDto {
                        source_id: "taboo_table".to_string(),
                        method: "table-lookup".to_string(),
                        profile: "baseline".to_string(),
                    }),
                }],
                xung_hop: XungHopDto {
                    luc_xung: "Tý".to_string(),
                    tam_hop: vec!["Dần".to_string(), "Tuất".to_string()],
                    tu_hanh_xung: vec!["Ngọ".to_string()],
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
                active_packs: vec![],
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
                        activity_id: "meet_visit".to_string(),
                        label: ActivityLabelDto {
                            vi: "Gặp gỡ".to_string(),
                            en: "Meet".to_string(),
                        },
                        bucket: RecommendationBucketDto::CoThe,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "travel.support".to_string(),
                            severity: RecommendationSeverityDto::Supporting,
                            summary_vi: "Có hướng xuất hành phù hợp".to_string(),
                            summary_en: "Travel is acceptable".to_string(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::Travel,
                                code: "travel.good".to_string(),
                                note: "test".to_string(),
                            },
                        }],
                    },
                    SynthesizedRecommendationDto {
                        activity_id: "contract_agreement".to_string(),
                        label: ActivityLabelDto {
                            vi: "Ký kết".to_string(),
                            en: "Contract".to_string(),
                        },
                        bucket: RecommendationBucketDto::Tranh,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "xung_hop.avoid".to_string(),
                            severity: RecommendationSeverityDto::Primary,
                            summary_vi: "Ngày xung".to_string(),
                            summary_en: "Clashing day".to_string(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::XungHop,
                                code: "xung_hop.bad".to_string(),
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
            contextual_recommendations: None,
            insight: None,
        }
    }

    fn sample_app_state_with_bundle() -> AppState {
        let mut app = sample_app_state();
        app.bundle = Some(sample_bundle());
        app
    }

    #[test]
    fn section_focus_cycles_in_order() {
        let mut app = sample_app_state();
        let expected = [
            PageSection::Recommendations,
            PageSection::Timing,
            PageSection::Travel,
            PageSection::Risks,
            PageSection::TraditionalEvidence,
            PageSection::ExpandedDetails,
            PageSection::Hero,
        ];

        for section in expected {
            app.focus_next_section();
            assert_eq!(app.focused_section, section);
        }
    }

    #[test]
    fn evidence_toggle_changes_visibility_flag() {
        let mut app = sample_app_state();

        assert!(!app.show_evidence);
        app.toggle_evidence();
        assert!(app.show_evidence);
        app.toggle_evidence();
        assert!(!app.show_evidence);
    }

    #[test]
    fn zoom_mode_tracks_focused_section() {
        let mut app = sample_app_state();

        app.focused_section = PageSection::Travel;
        app.toggle_zoom_for_focused_section();
        assert_eq!(app.zoomed_section, Some(PageSection::Travel));

        app.focused_section = PageSection::Risks;
        app.toggle_zoom_for_focused_section();
        assert_eq!(app.zoomed_section, Some(PageSection::Risks));

        app.toggle_zoom_for_focused_section();
        assert_eq!(app.zoomed_section, None);
    }

    #[test]
    fn expand_toggle_is_scoped_to_focused_section() {
        let mut app = sample_app_state();

        app.focused_section = PageSection::Recommendations;
        app.toggle_expand_focused_section();
        assert!(app.is_section_expanded(PageSection::Recommendations));
        assert!(!app.is_section_expanded(PageSection::TraditionalEvidence));

        app.focused_section = PageSection::TraditionalEvidence;
        app.toggle_expand_focused_section();
        assert!(app.is_section_expanded(PageSection::Recommendations));
        assert!(app.is_section_expanded(PageSection::TraditionalEvidence));

        app.toggle_expand_focused_section();
        assert!(app.is_section_expanded(PageSection::Recommendations));
        assert!(!app.is_section_expanded(PageSection::TraditionalEvidence));
    }

    #[test]
    fn top_recommendation_rows_follow_bucket_order() {
        let app = sample_app_state_with_bundle();

        let rows = app.top_recommendation_rows();

        assert_eq!(
            rows.iter().map(|row| row.label.as_str()).collect::<Vec<_>>(),
            vec!["Khai mở", "Gặp gỡ", "Ký kết", "Động thổ"]
        );
        assert_eq!(
            rows.iter().map(|row| row.bucket).collect::<Vec<_>>(),
            vec![
                RecommendationBucketDto::Nen,
                RecommendationBucketDto::CoThe,
                RecommendationBucketDto::Tranh,
                RecommendationBucketDto::KyManh,
            ]
        );
    }

    #[test]
    fn hero_verdict_prefers_summary_and_strongest_rows() {
        let app = sample_app_state_with_bundle();

        let verdict = app.hero_verdict().expect("hero verdict");

        assert_eq!(verdict.summary, "Ngày thuận việc mở đầu, tránh việc lớn.");
        assert_eq!(verdict.strongest_positive.as_deref(), Some("Khai mở"));
        assert_eq!(verdict.strongest_negative.as_deref(), Some("Động thổ"));
    }

    #[test]
    fn risk_summary_surfaces_ky_manh_and_taboos_first() {
        let app = sample_app_state_with_bundle();

        let risk_summary = app.risk_summary();

        assert_eq!(risk_summary.items[0], "Kỵ mạnh: Động thổ");
        assert_eq!(risk_summary.items[1], "Kiêng kỵ: Tam Nương");
        assert!(
            risk_summary.items.iter().any(|item| item.contains("Lục xung: Tý")),
            "expected luc xung entry in risk summary"
        );
    }
}
