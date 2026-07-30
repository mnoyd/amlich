use serde::{Deserialize, Serialize};

use crate::{
    almanac::recommendation::{
        synthesize_daily_recommendations_with_layers, ActivityId, DailyRecommendations,
        RecommendationPackLookupError, RecommendationSynthesisContext,
    },
    canchi::get_year_canchi,
    julian::jd_from_date,
    lunar::{convert_solar_to_lunar, LunarDate},
    tietkhi::get_tiet_khi,
    types::{CanChi, VIETNAM_TIMEZONE},
    CanChiSet, DayContext, SolarDate,
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
        let auspicious_count = reasoning
            .ranked_hours
            .iter()
            .filter(|h| h.is_auspicious)
            .count();
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

/// Rank the twelve traditional hour slots for a given `intent`.
///
/// **Compatibility ranking projection (amlich-mwbp.7).** The numeric
/// `score` on each [`RankedHourCandidate`] is a deterministic ordering
/// heuristic over Hoàng Đạo membership + intent/birth-compatibility
/// bonuses — it is NOT a day-verdict score and is not comparable to the
/// canonical
/// [`PersonalDayAssessment::decision`](crate::assessment::PersonalDayDecision)
/// score. Consumers must read the canonical verdict off
/// `canonical_assessment` (attached by the amlich-api hour-selection
/// surfaces) and use this ranking only to pick among hour slots that the
/// day verdict already permits.
///
/// The deeper rework (typed contributions for hour slots, intent-policy
/// ranking) is owned by amlich-mwbp.8; this function only carries an
/// explicit compatibility label during the migration window.
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
    let auspicious_count = ranked_hours
        .iter()
        .filter(|hour| hour.is_auspicious)
        .count();
    let summary_vi = match top_recommendation.as_ref() {
        Some(top) => format!(
            "Ưu tiên giờ {} ({}) cho {} vì đứng đầu xếp hạng với {} giờ hoàng đạo hỗ trợ.",
            top.chi_name,
            top.time_range,
            intent.event_kind(),
            auspicious_count
        ),
        None => format!(
            "Không có giờ phù hợp để xếp hạng cho {}.",
            intent.event_kind()
        ),
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
    fn rank_hours_prioritizes_auspicious_slots() {
        let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None)
            .expect("ranked hours");

        assert!(!ranked.is_empty());
        assert!(ranked[0].score >= ranked[ranked.len() - 1].score);
    }
}
