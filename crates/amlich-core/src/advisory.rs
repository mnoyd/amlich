use chrono::Datelike;
use serde::{Deserialize, Serialize};

use crate::{
    almanac::recommendation::{
        synthesize_daily_recommendations_with_layers, ActivityId, DailyRecommendations,
        RecommendationPackLookupError, RecommendationSynthesisContext,
    },
    almanac::tu_menh::compute_kua,
    canchi::get_year_canchi,
    julian::jd_from_date,
    lunar::{convert_solar_to_lunar, LunarDate},
    tietkhi::get_tiet_khi,
    types::{CanChi, VIETNAM_TIMEZONE},
    CanChiSet, DayContext, DaySnapshot, SolarDate,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsultationIntent {
    Wedding,
    MovingHouse,
    OpeningBusiness,
    ContractSigning,
    Travel,
    Burial,
    Renovation,
    Medical,
    Prayer,
}

impl ConsultationIntent {
    pub fn event_kind(self) -> &'static str {
        match self {
            ConsultationIntent::Wedding => "wedding",
            ConsultationIntent::MovingHouse => "moving_house",
            ConsultationIntent::OpeningBusiness => "opening_business",
            ConsultationIntent::ContractSigning => "contract_signing",
            ConsultationIntent::Travel => "travel",
            ConsultationIntent::Burial => "burial",
            ConsultationIntent::Renovation => "renovation",
            ConsultationIntent::Medical => "medical_checkup",
            ConsultationIntent::Prayer => "prayer",
        }
    }

    pub fn primary_activity(self) -> ActivityId {
        match self {
            ConsultationIntent::Wedding => ActivityId::WeddingEngagement,
            ConsultationIntent::MovingHouse => ActivityId::MoveRelocation,
            ConsultationIntent::OpeningBusiness => ActivityId::OpeningStart,
            ConsultationIntent::ContractSigning => ActivityId::ContractAgreement,
            ConsultationIntent::Travel => ActivityId::Travel,
            ConsultationIntent::Burial => ActivityId::BurialMemorial,
            ConsultationIntent::Renovation => ActivityId::RepairRenovation,
            ConsultationIntent::Medical => ActivityId::MedicalTreatment,
            ConsultationIntent::Prayer => ActivityId::PrayerOffering,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BirthInput {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour: Option<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minute: Option<u8>,
    #[serde(default = "default_timezone")]
    pub timezone: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<crate::almanac::tu_menh::Gender>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
}

fn default_timezone() -> f64 {
    VIETNAM_TIMEZONE
}

impl BirthInput {
    pub fn to_lunar_date(&self) -> LunarDate {
        convert_solar_to_lunar(self.day, self.month, self.year, self.timezone)
    }

    pub fn birth_year_canchi(&self) -> CanChi {
        get_year_canchi(self.to_lunar_date().year)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvisoryScoring {
    pub score: i32,
    pub verdict: String,
    pub confidence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceEnvelope {
    pub source_family: String,
    pub source_id: String,
    pub method: String,
    pub profile: String,
    pub note: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScoredAdvice {
    pub summary_vi: String,
    pub summary_en: String,
    pub scoring: AdvisoryScoring,
    pub reasons: Vec<String>,
    pub warnings: Vec<String>,
    pub applied_rulesets: Vec<String>,
    pub evidence: Vec<EvidenceEnvelope>,
    pub recommendations: DailyRecommendations,
}

#[derive(Debug, Clone)]
pub struct PersonalizedDaySelection {
    pub intent: ConsultationIntent,
    pub birth: Option<BirthInput>,
    pub snapshot: DaySnapshot,
    pub advisory: ScoredAdvice,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DateRangeInput {
    pub start_day: i32,
    pub start_month: i32,
    pub start_year: i32,
    pub end_day: i32,
    pub end_month: i32,
    pub end_year: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankedDateCandidate {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub score: i32,
    pub verdict: String,
    pub summary_vi: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankedHourCandidate {
    pub chi_name: String,
    pub time_range: String,
    pub is_auspicious: bool,
    pub score: i32,
    pub note_vi: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HourSelectionEvidence {
    pub source_family: String,
    pub source_id: String,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HourSelectionReasoningExport {
    pub intent: String,
    pub birth_data_tier: String,
    pub summary_vi: String,
    pub summary_en: String,
    pub top_recommendation: Option<RankedHourCandidate>,
    pub ranked_hours: Vec<RankedHourCandidate>,
    pub auspicious_count: usize,
    pub total_hours: usize,
    #[serde(default)]
    pub evidence: Vec<HourSelectionEvidence>,
}

impl HourSelectionReasoningExport {
    pub fn from_reasoning(reasoning: &HourSelectionReasoning, birth: Option<&BirthInput>) -> Self {
        let auspicious_count = reasoning.ranked_hours.iter().filter(|h| h.is_auspicious).count();
        let total_hours = reasoning.ranked_hours.len();
        let birth_data_tier = match birth {
            Some(b) if b.hour.is_some() && b.minute.is_some() && b.gender.is_some() => "datetime",
            Some(_) => "date",
            None => "anonymous",
        };
        let mut evidence = vec![HourSelectionEvidence {
            source_family: "amlich_core".to_string(),
            source_id: "hour_selection".to_string(),
            method: "rank_hours_for_intent".to_string(),
            note: Some(format!("intent={}", reasoning.intent.event_kind())),
        }];
        if let Some(birth) = birth {
            evidence.push(HourSelectionEvidence {
                source_family: "birth_input".to_string(),
                source_id: format!("birth.{}.{}.{}", birth.year, birth.month, birth.day),
                method: "birth_compatibility".to_string(),
                note: None,
            });
        }

        HourSelectionReasoningExport {
            intent: reasoning.intent.event_kind().to_string(),
            birth_data_tier: birth_data_tier.to_string(),
            summary_vi: reasoning.summary_vi.clone(),
            summary_en: reasoning.summary_en.clone(),
            top_recommendation: reasoning.top_recommendation.clone(),
            ranked_hours: reasoning.ranked_hours.clone(),
            auspicious_count,
            total_hours,
            evidence,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HourSelectionReasoning {
    pub intent: ConsultationIntent,
    pub summary_vi: String,
    pub summary_en: String,
    pub top_recommendation: Option<RankedHourCandidate>,
    pub ranked_hours: Vec<RankedHourCandidate>,
}

impl HourSelectionReasoning {
    pub fn export(&self, birth: Option<&BirthInput>) -> HourSelectionReasoningExport {
        HourSelectionReasoningExport::from_reasoning(self, birth)
    }
}

pub fn build_personalized_day_selection(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<BirthInput>,
    enabled_pack_ids: &[&str],
) -> Result<PersonalizedDaySelection, String> {
    let snapshot = crate::calculate_day_snapshot_with_recommendation_request(
        day,
        month,
        year,
        VIETNAM_TIMEZONE,
        None,
        Some(intent.event_kind()),
        enabled_pack_ids,
    )?;

    let recommendations = snapshot
        .contextual_recommendations
        .clone()
        .unwrap_or_else(|| snapshot.daily_recommendations.clone());
    let advisory = score_day_selection(
        &snapshot.context,
        &snapshot,
        recommendations,
        intent,
        birth.as_ref(),
    );

    Ok(PersonalizedDaySelection {
        intent,
        birth,
        snapshot,
        advisory,
    })
}

pub fn score_day_selection(
    context: &DayContext,
    snapshot: &DaySnapshot,
    recommendations: DailyRecommendations,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
) -> ScoredAdvice {
    let mut score = 50;
    let mut reasons = vec![format!(
        "Ngày {} có trực {} và {} hoạt động được đánh giá.",
        context.canchi.day.full,
        snapshot.day_fortune.truc.name,
        recommendations.activities.len()
    )];
    let mut warnings = Vec::new();

    if let Some(primary) = recommendations
        .activities
        .iter()
        .find(|activity| activity.activity_id == intent.primary_activity())
    {
        use crate::almanac::recommendation::RecommendationBucket;

        match primary.bucket {
            RecommendationBucket::Nen => score += 25,
            RecommendationBucket::CoThe => score += 10,
            RecommendationBucket::Tranh => score -= 15,
            RecommendationBucket::KyManh => score -= 30,
        }

        reasons.push(format!(
            "Hoạt động chính '{}' được xếp nhóm {:?}.",
            primary.label.vi, primary.bucket
        ));
    } else {
        warnings.push("Chưa có rule chuyên biệt mạnh cho mục đích này.".to_string());
    }

    if !snapshot.day_fortune.taboos.is_empty() {
        score -= (snapshot.day_fortune.taboos.len() as i32).min(3) * 5;
        reasons.push(format!(
            "Ngày có {} điều kiêng kỵ được ghi nhận.",
            snapshot.day_fortune.taboos.len()
        ));
    }

    if let Some(birth) = birth {
        apply_birth_compatibility(
            &mut score,
            &mut reasons,
            &mut warnings,
            birth,
            context,
            snapshot,
        );
    } else {
        warnings
            .push("Chưa có dữ liệu sinh nên chưa cá nhân hóa đầy đủ theo tuổi/mệnh.".to_string());
    }

    let verdict = if score >= 75 {
        "strong_match"
    } else if score >= 60 {
        "good_match"
    } else if score >= 45 {
        "mixed"
    } else {
        "weak_match"
    }
    .to_string();

    let confidence = if birth.is_some() { "medium" } else { "low" }.to_string();

    ScoredAdvice {
        summary_vi: recommendations.summary_vi.clone(),
        summary_en: recommendations.summary_en.clone(),
        scoring: AdvisoryScoring {
            score: score.clamp(0, 100),
            verdict,
            confidence,
        },
        reasons,
        warnings,
        applied_rulesets: vec![format!(
            "{}@{}",
            snapshot.ruleset_id, snapshot.ruleset_version
        )],
        evidence: vec![EvidenceEnvelope {
            source_family: "amlich_core".to_string(),
            source_id: snapshot.ruleset_id.clone(),
            method: "phase0_phase1_foundation".to_string(),
            profile: snapshot.profile.clone(),
            note: format!("intent={}", intent.event_kind()),
        }],
        recommendations,
    }
}

fn apply_birth_compatibility(
    score: &mut i32,
    reasons: &mut Vec<String>,
    warnings: &mut Vec<String>,
    birth: &BirthInput,
    context: &DayContext,
    snapshot: &DaySnapshot,
) {
    let birth_year = birth.birth_year_canchi();
    let day_chi = snapshot.context.canchi.day.chi.as_str();

    if birth_year.chi == day_chi {
        *score += 6;
        reasons.push(format!(
            "Chi ngày {} trùng chi tuổi {}, thiên về đồng khí.",
            day_chi, birth_year.full
        ));
    } else if snapshot.day_fortune.xung_hop.luc_xung == birth_year.chi {
        *score -= 20;
        warnings.push(format!(
            "Chi ngày {} rơi vào lục xung với tuổi {}.",
            day_chi, birth_year.full
        ));
    } else if snapshot
        .day_fortune
        .xung_hop
        .tam_hop
        .iter()
        .any(|chi| chi == &birth_year.chi)
    {
        *score += 12;
        reasons.push(format!(
            "Ngày nằm trong tam hợp với tuổi {}.",
            birth_year.full
        ));
    } else if snapshot.day_fortune.xung_hop.liu_he.as_deref() == Some(birth_year.chi.as_str()) {
        *score += 8;
        reasons.push(format!("Ngày có lục hợp với tuổi {}.", birth_year.full));
    } else if snapshot.day_fortune.xung_hop.xiang_hai.as_deref() == Some(birth_year.chi.as_str()) {
        *score -= 8;
        warnings.push(format!("Ngày có tương hại với tuổi {}.", birth_year.full));
    } else {
        reasons.push(format!(
            "Đã đối chiếu tuổi {} với chi ngày {} ở mức xung/hợp cơ bản.",
            birth_year.full, context.canchi.day.full
        ));
    }

    if let Some(gender) = birth.gender {
        let kua = compute_kua(birth.year, gender);
        let favorable = kua
            .favorable_directions
            .iter()
            .any(|direction| direction.to_string() == snapshot.day_fortune.travel.xuat_hanh_huong);
        let unfavorable = kua
            .unfavorable_directions
            .iter()
            .any(|direction| direction.to_string() == snapshot.day_fortune.travel.xuat_hanh_huong);

        if favorable {
            *score += 6;
            reasons.push(format!(
                "Hướng xuất hành {} trùng nhóm hướng tốt của cung mệnh {}.",
                snapshot.day_fortune.travel.xuat_hanh_huong, kua.kua
            ));
        } else if unfavorable {
            *score -= 6;
            warnings.push(format!(
                "Hướng xuất hành {} rơi vào nhóm hướng bất lợi của cung mệnh {}.",
                snapshot.day_fortune.travel.xuat_hanh_huong, kua.kua
            ));
        }
    } else {
        warnings.push("Thiếu giới tính nên chưa đối chiếu thêm theo cung mệnh/Kua.".to_string());
    }
}

pub fn compute_day_context_from_birth(birth: &BirthInput) -> DayContext {
    let jd = jd_from_date(birth.day, birth.month, birth.year);
    let lunar = birth.to_lunar_date();
    let weekday_index = ((jd + 1) % 7) as usize;
    let day_canchi = crate::canchi::get_day_canchi(jd);
    let month_canchi = crate::canchi::get_month_canchi(lunar.month, lunar.year, lunar.is_leap);
    let year_canchi = crate::canchi::get_year_canchi(lunar.year);
    let tiet_khi = get_tiet_khi(jd, birth.timezone);
    let gio_hoang_dao = crate::gio_hoang_dao::get_gio_hoang_dao(day_canchi.chi_index);

    DayContext {
        solar: SolarDate {
            day: birth.day,
            month: birth.month,
            year: birth.year,
            day_of_week: weekday_index,
        },
        lunar,
        jd,
        weekday_index,
        canchi: CanChiSet {
            day: day_canchi,
            month: month_canchi,
            year: year_canchi,
        },
        tiet_khi,
        gio_hoang_dao,
    }
}

pub fn build_recommendation_context<'a>(
    context: &'a DayContext,
    day_fortune: &'a crate::almanac::types::DayFortune,
    intent: Option<ConsultationIntent>,
    enabled_pack_ids: &'a [&'a str],
) -> RecommendationSynthesisContext<'a> {
    RecommendationSynthesisContext {
        day_chi: &context.canchi.day.chi,
        day_fortune,
        gio_hoang_dao: Some(&context.gio_hoang_dao),
        tiet_khi_name: Some(&context.tiet_khi.name),
        profile_id: Some("advisory"),
        event_kind: intent.map(ConsultationIntent::event_kind),
        enabled_pack_ids,
    }
}

pub fn synthesize_advisory_recommendations(
    context: &DayContext,
    day_fortune: &crate::almanac::types::DayFortune,
    intent: Option<ConsultationIntent>,
    enabled_pack_ids: &[&str],
) -> Result<DailyRecommendations, RecommendationPackLookupError> {
    let ctx = build_recommendation_context(context, day_fortune, intent, enabled_pack_ids);
    synthesize_daily_recommendations_with_layers(&ctx, &[])
}

pub fn rank_dates_for_intent(
    range: &DateRangeInput,
    intent: ConsultationIntent,
    birth: Option<BirthInput>,
    enabled_pack_ids: &[&str],
    top_n: usize,
) -> Result<Vec<RankedDateCandidate>, String> {
    let start = chrono::NaiveDate::from_ymd_opt(
        range.start_year,
        range.start_month as u32,
        range.start_day as u32,
    )
    .ok_or_else(|| "invalid start date".to_string())?;
    let end = chrono::NaiveDate::from_ymd_opt(
        range.end_year,
        range.end_month as u32,
        range.end_day as u32,
    )
    .ok_or_else(|| "invalid end date".to_string())?;

    if start > end {
        return Err("start date must be before or equal to end date".to_string());
    }

    let mut ranked = Vec::new();
    let mut current = start;

    while current <= end {
        let selection = build_personalized_day_selection(
            current.day() as i32,
            current.month() as i32,
            current.year(),
            intent,
            birth.clone(),
            enabled_pack_ids,
        )?;

        ranked.push(RankedDateCandidate {
            day: current.day() as i32,
            month: current.month() as i32,
            year: current.year(),
            score: selection.advisory.scoring.score,
            verdict: selection.advisory.scoring.verdict.clone(),
            summary_vi: selection.advisory.summary_vi.clone(),
        });

        current = current
            .succ_opt()
            .ok_or_else(|| "date iteration overflow".to_string())?;
    }

    ranked.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.year.cmp(&right.year))
            .then_with(|| left.month.cmp(&right.month))
            .then_with(|| left.day.cmp(&right.day))
    });

    ranked.truncate(top_n);
    Ok(ranked)
}

pub fn rank_hours_for_intent(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
) -> Result<Vec<RankedHourCandidate>, String> {
    let snapshot = crate::calculate_day_snapshot(day, month, year);
    let mut ranked = Vec::new();

    for hour in &snapshot.context.gio_hoang_dao.all_hours {
        let mut score = if hour.is_good { 70 } else { 35 };
        let mut note = if hour.is_good {
            format!("Giờ {} là giờ hoàng đạo.", hour.hour_chi)
        } else {
            format!("Giờ {} không thuộc giờ hoàng đạo.", hour.hour_chi)
        };

        if intent == ConsultationIntent::Travel
            && snapshot.day_fortune.travel.xuat_hanh_huong == "Nam"
            && hour.is_good
        {
            score += 5;
            note.push_str(" Phù hợp thêm cho xuất hành.");
        }

        if let Some(birth) = birth {
            let birth_year = birth.birth_year_canchi();
            if birth_year.chi == hour.hour_chi {
                score += 4;
                note.push_str(" Có đồng khí với chi tuổi.");
            } else if snapshot.day_fortune.xung_hop.luc_xung == birth_year.chi {
                score -= 4;
            }
        }

        ranked.push(RankedHourCandidate {
            chi_name: hour.hour_chi.clone(),
            time_range: hour.time_range.clone(),
            is_auspicious: hour.is_good,
            score: score.clamp(0, 100),
            note_vi: note,
        });
    }

    ranked.sort_by(|left, right| {
        right
            .score
            .cmp(&left.score)
            .then_with(|| left.chi_name.cmp(&right.chi_name))
    });
    Ok(ranked)
}

pub fn build_hour_selection_reasoning(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
) -> Result<HourSelectionReasoning, String> {
    let ranked_hours = rank_hours_for_intent(day, month, year, intent, birth)?;
    let top_recommendation = ranked_hours.first().cloned();
    let auspicious_count = ranked_hours.iter().filter(|hour| hour.is_auspicious).count();
    let summary_vi = match top_recommendation.as_ref() {
        Some(top) => format!(
            "Ưu tiên giờ {} ({}) cho {} vì đứng đầu xếp hạng với {} giờ hoàng đạo hỗ trợ.",
            top.chi_name,
            top.time_range,
            intent.event_kind(),
            auspicious_count
        ),
        None => format!("Không có giờ phù hợp để xếp hạng cho {}.", intent.event_kind()),
    };
    let summary_en = match top_recommendation.as_ref() {
        Some(top) => format!(
            "Prefer the {} hour ({}) for {} because it leads the ranking with {} auspicious windows supporting the day.",
            top.chi_name,
            top.time_range,
            intent.event_kind(),
            auspicious_count
        ),
        None => format!("No ranked hour candidates are available for {}.", intent.event_kind()),
    };

    Ok(HourSelectionReasoning {
        intent,
        summary_vi,
        summary_en,
        top_recommendation,
        ranked_hours,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consultation_intent_maps_to_event_kind() {
        assert_eq!(
            ConsultationIntent::ContractSigning.event_kind(),
            "contract_signing"
        );
    }

    #[test]
    fn birth_input_defaults_to_vietnam_timezone() {
        let birth = BirthInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: None,
            minute: None,
            timezone: default_timezone(),
            gender: None,
            location_name: None,
        };

        assert_eq!(birth.timezone, VIETNAM_TIMEZONE);
    }

    #[test]
    fn personalized_day_selection_builds_scored_advice() {
        let result = build_personalized_day_selection(
            10,
            2,
            2024,
            ConsultationIntent::ContractSigning,
            None,
            &[],
        )
        .expect("selection");

        assert_eq!(result.intent, ConsultationIntent::ContractSigning);
        assert!(!result.advisory.summary_vi.is_empty());
        assert!((0..=100).contains(&result.advisory.scoring.score));
    }

    #[test]
    fn advisory_context_can_synthesize_intent_recommendations() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let recommendations = synthesize_advisory_recommendations(
            &snapshot.context,
            &snapshot.day_fortune,
            Some(ConsultationIntent::ContractSigning),
            &[],
        )
        .expect("recommendations");

        assert!(recommendations
            .activities
            .iter()
            .any(|activity| { activity.activity_id == ActivityId::ContractAgreement }));
    }

    #[test]
    fn rank_dates_returns_best_candidates_in_score_order() {
        let ranked = rank_dates_for_intent(
            &DateRangeInput {
                start_day: 10,
                start_month: 2,
                start_year: 2024,
                end_day: 12,
                end_month: 2,
                end_year: 2024,
            },
            ConsultationIntent::ContractSigning,
            None,
            &[],
            2,
        )
        .expect("ranked dates");

        assert_eq!(ranked.len(), 2);
        assert!(ranked[0].score >= ranked[1].score);
    }

    #[test]
    fn rank_hours_prioritizes_auspicious_slots() {
        let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None)
            .expect("ranked hours");

        assert!(!ranked.is_empty());
        assert!(ranked[0].score >= ranked[ranked.len() - 1].score);
    }

    #[test]
    fn birth_compatibility_affects_selection_score() {
        let without_birth =
            build_personalized_day_selection(10, 2, 2024, ConsultationIntent::Travel, None, &[])
                .expect("baseline");
        let with_birth = build_personalized_day_selection(
            10,
            2,
            2024,
            ConsultationIntent::Travel,
            Some(BirthInput {
                day: 10,
                month: 2,
                year: 2024,
                hour: None,
                minute: None,
                timezone: VIETNAM_TIMEZONE,
                gender: Some(crate::almanac::tu_menh::Gender::Male),
                location_name: None,
            }),
            &[],
        )
        .expect("personalized");

        assert_ne!(
            without_birth.advisory.scoring.score,
            with_birth.advisory.scoring.score
        );
    }
}
