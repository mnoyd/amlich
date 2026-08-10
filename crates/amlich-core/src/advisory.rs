use serde::{Deserialize, Serialize};

use crate::{
    almanac::recommendation::{
        synthesize_daily_recommendations_with_layers, ActivityId, DailyRecommendations,
        RecommendationPackLookupError, RecommendationSynthesisContext,
    },
    assessment::PersonalDayAssessment,
    canchi::get_year_canchi,
    julian::jd_from_date,
    lunar::{convert_solar_to_lunar, LunarDate},
    sources::SOURCE_KHCBPPT,
    tietkhi::get_tiet_khi,
    types::{CanChi, VIETNAM_TIMEZONE},
    CanChiSet, DayContext, HourRankingPolicy, HourRankingWarning, RankedHourV1, SolarDate,
    HOUR_RANKING_POLICY_V2_4_VERSION,
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
    /// Structured warning context emitted by the v1 hour-ranking policy
    /// when the supplied canonical [`PersonalDayAssessment`] classifies
    /// the day as `Avoid`. `None` when no assessment was threaded
    /// through, or when the day verdict is anything other than `Avoid`.
    /// Additive `Option<T>` (amlich-rv13.5) — keeps v1.6 → v1.7 round-trip
    /// byte-equal for callers that never thread a day assessment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_context: Option<HourRankingWarning>,
    #[serde(default)]
    pub evidence: Vec<HourSelectionEvidence>,
    /// Versioned policy that produced this reasoning. v1 keeps the
    /// legacy birth-year-chi semantics; v2.4 (`amlich-bz0f.4`) layers
    /// three typed, source-attributed full-profile observations on top
    /// so a full birth profile (date + time) produces a richer
    /// `PersonalHourAlignment` axis. `None` for reasoning produced by
    /// the pre-v1.9 wrappers. Additive `Option<T>` — keeps existing
    /// round-trips byte-equal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
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
        if reasoning.warning_context.is_some() {
            evidence.push(HourSelectionEvidence {
                source_family: "amlich_core".to_string(),
                source_id: "hour_ranking_warning".to_string(),
                method: "policy_rank_warning_context".to_string(),
                note: Some("day verdict Avoid — hour ranking carries warning context".to_string()),
            });
        }
        if reasoning.policy_version.as_deref() == Some(HOUR_RANKING_POLICY_V2_4_VERSION) {
            // amlich-bz0f.4: surface the PersonalHourMatrix source
            // family on the export's evidence list so desktop / TUI
            // consumers can label the v2.4 trio's contributions without
            // parsing every ranked hour's note.
            evidence.push(HourSelectionEvidence {
                source_family: "personal_hour_matrix".to_string(),
                source_id: SOURCE_KHCBPPT.to_string(),
                method: "hour_ranking_policy_v2_4".to_string(),
                note: Some(
                    "v2.4 full-profile hour-pillar Thập Thần, hour chi × birth hour chi branch relation, and hour stem element support"
                        .to_string(),
                ),
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
            warning_context: reasoning.warning_context.clone(),
            evidence,
            policy_version: reasoning.policy_version.clone(),
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
    /// Structured warning context attached by the v1 hour-ranking policy
    /// when a threaded [`PersonalDayAssessment`] classifies the day as
    /// `Avoid`. Carries the day bucket and Vietnamese message verbatim
    /// from [`HourRankingWarning`]; consumers should surface it instead
    /// of presenting the ranked hours as a recommendation that overrides
    /// the day verdict. `None` when no assessment was threaded through,
    /// or when the day verdict is anything other than `Avoid`. Additive
    /// field (amlich-rv13.5).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warning_context: Option<HourRankingWarning>,
    /// Versioned policy that produced this reasoning. v1 keeps the
    /// legacy birth-year-chi semantics; v2.4 (`amlich-bz0f.4`) layers
    /// three typed, source-attributed full-profile observations on top
    /// so a full birth profile (date + time) produces a richer
    /// `PersonalHourAlignment` axis. `None` for reasoning produced by
    /// the pre-v1.9 wrappers. Additive `Option<T>` — keeps existing
    /// round-trips byte-equal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
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
/// **Compatibility ranking projection (amlich-rv13.4).** This function is
/// a thin projection of the canonical
/// [`crate::assessment::HourRankingPolicy::baseline_v1`] policy. The new
/// policy owns the ranking calculation and exposes normalized
/// `0.0..=1.0` [`crate::assessment::RankedHourV1`] outputs with typed
/// axes/contributions; this wrapper projects those values back to the
/// legacy [`RankedHourCandidate`] integer `0..=100` score shape so
/// existing consumers keep working through the migration window.
///
/// **Day-verdict warning threading (amlich-rv13.5).** Pass
/// `day_assessment = Some(&assessment)` when a canonical
/// [`PersonalDayAssessment`] is available. When the assessment's day
/// bucket is `Avoid`, the v1 policy attaches a [`HourRankingWarning`] to
/// every ranked hour; this wrapper both embeds the warning into each
/// candidate's Vietnamese `note_vi` (prefixed with `[Cảnh báo]`) and
/// surfaces it as the structured `warning_context` field on the
/// [`HourSelectionReasoning`] built by [`build_hour_selection_reasoning`].
/// When `day_assessment` is `None` or the day verdict is anything other
/// than `Avoid`, the warning is omitted — the ranking stays a pure
/// rank-only projection that never restates the day verdict.
///
/// The numeric `score` on each [`RankedHourCandidate`] is a deterministic
/// projection of the v1 `rank_score` (×100, rounded, clamped to `0..=100`)
/// and is **not** a day-verdict score — it is not comparable to the
/// canonical
/// [`PersonalDayAssessment::decision`](crate::assessment::PersonalDayDecision)
/// score. Consumers must read the canonical verdict off
/// `canonical_assessment` (attached by the amlich-api hour-selection
/// surfaces) and use this ranking only to pick among hour slots that the
/// day verdict already permits.
///
/// Tie-break for exact score equality uses traditional Chi order
/// (`chi_index` ascending) per spec §"Ranking order". This is a
/// deliberate change from the prior alphabetical Vietnamese-name
/// tie-break, matching the v1 policy's contract and the spec's explicit
/// "do not tie-break alphabetically by Vietnamese Chi name" rule.
pub fn rank_hours_for_intent(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
    day_assessment: Option<&PersonalDayAssessment>,
) -> Result<Vec<RankedHourCandidate>, String> {
    let ranked_v1 = rank_hours_v1(day, month, year, intent, birth, day_assessment)?;

    Ok(ranked_v1
        .iter()
        .map(project_ranked_hour_v1_to_legacy_candidate)
        .collect())
}

/// Shared seam that runs the v1 hour-ranking policy once and returns the
/// canonical [`RankedHourV1`] list. Both [`rank_hours_for_intent`] and
/// [`build_hour_selection_reasoning`] consume this so the ranking is not
/// recomputed when both the legacy projection and the structured warning
/// are needed in the same call path (amlich-rv13.5).
fn rank_hours_v1(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
    day_assessment: Option<&PersonalDayAssessment>,
) -> Result<Vec<RankedHourV1>, String> {
    let snapshot = crate::calculate_day_snapshot(day, month, year);
    let policy = HourRankingPolicy::baseline_v1();
    policy.rank(&snapshot, intent, birth, day_assessment)
}

/// Full-profile v2.4 projection (`amlich-bz0f.4`). Same external
/// contract as [`rank_hours_for_intent`], but uses the
/// [`HourRankingPolicy::full_profile_v2_4`] policy so the personal
/// alignment axis folds in the hour-pillar Thập Thần, hour chi × birth
/// hour chi branch relation, and hour stem element support signals.
///
/// The v2.4 policy emits explicit `Unavailable` observations for the
/// three new features when a full birth profile (date + time) is not
/// available, so date-only and anonymous callers still see the same
/// twelve-hour ordering as [`rank_hours_for_intent`] — only the
/// underlying trace gains the new `Unavailable` rows.
pub fn rank_hours_for_intent_full_profile_v2_4(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
    day_assessment: Option<&PersonalDayAssessment>,
) -> Result<Vec<RankedHourCandidate>, String> {
    let snapshot = crate::calculate_day_snapshot(day, month, year);
    let policy = HourRankingPolicy::full_profile_v2_4();
    let ranked_v2_4 = policy.rank(&snapshot, intent, birth, day_assessment)?;

    Ok(ranked_v2_4
        .iter()
        .map(project_ranked_hour_v1_to_legacy_candidate)
        .collect())
}

/// Shared seam that runs the v2.4 full-profile hour-ranking policy once
/// and returns the canonical [`RankedHourV1`] list. Both
/// [`rank_hours_for_intent_full_profile_v2_4`] and any future v2.4-aware
/// consumer consume this so the ranking is computed once per call path.
#[allow(dead_code)]
fn rank_hours_v2_4(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
    day_assessment: Option<&PersonalDayAssessment>,
) -> Result<Vec<RankedHourV1>, String> {
    let snapshot = crate::calculate_day_snapshot(day, month, year);
    let policy = HourRankingPolicy::full_profile_v2_4();
    policy.rank(&snapshot, intent, birth, day_assessment)
}

/// Project a canonical v1 [`crate::RankedHourV1`] output back
/// to the legacy [`RankedHourCandidate`] shape used by
/// [`rank_hours_for_intent`].
///
/// The projection is **monotonic** with the v1 `rank_score` (the integer
/// score is `round(rank_score × 100).clamp(0, 100)`), so order from the
/// v1 policy is preserved end-to-end. The Vietnamese `note_vi` text is
/// re-derived from the v1 axes so consumers see a stable, human-readable
/// description that reflects the policy's actual calculation rather than
/// the legacy hand-rolled heuristic.
fn project_ranked_hour_v1_to_legacy_candidate(v1: &RankedHourV1) -> RankedHourCandidate {
    let note_vi = build_legacy_note_vi(v1);
    let score = (v1.rank_score * 100.0).round().clamp(0.0, 100.0) as i32;
    RankedHourCandidate {
        chi_name: v1.chi_name.clone(),
        time_range: v1.time_range.clone(),
        is_auspicious: v1.is_auspicious,
        score,
        note_vi,
    }
}

/// Compose the legacy Vietnamese `note_vi` text from the v1 axis
/// outcomes. Mirrors the legacy format ("Giờ X là/không thuộc giờ hoàng
/// đạo." plus optional axis clauses) so consumers reading the note see
/// the same shape, but the facts are pulled from the v1 policy's
/// per-axis scores instead of the legacy hand-rolled heuristic.
fn build_legacy_note_vi(v1: &RankedHourV1) -> String {
    let mut parts: Vec<String> = Vec::new();
    if v1.is_auspicious {
        parts.push(format!("Giờ {} là giờ hoàng đạo.", v1.chi_name));
    } else {
        parts.push(format!("Giờ {} không thuộc giờ hoàng đạo.", v1.chi_name));
    }

    if let Some(score) = v1.axes.personal_hour_alignment.score {
        // v1 personal alignment is 1.0 on match, 0.0 on clash,
        // 0.5 on neutral; only the match is broad-compatible with the
        // legacy "Có đồng khí với chi tuổi." clause.
        if (score - 1.0).abs() < 1e-3 {
            parts.push("Có đồng khí với chi tuổi.".to_string());
        }
    }

    if let Some(score) = v1.axes.day_hour_harmony.score {
        // v1 harmony scores: tam-hợp ≈ 0.8, lục-hợp ≈ 0.7,
        // lục-xung ≈ 0.1, default ≈ 0.5. Surface the three named
        // relations; the default is information-free so it stays
        // implicit in the hoàng-đạo sentence.
        if (score - 0.8).abs() < 1e-3 {
            parts.push("Tam hợp với chi ngày.".to_string());
        } else if (score - 0.7).abs() < 1e-3 {
            parts.push("Lục hợp với chi ngày.".to_string());
        } else if (score - 0.1).abs() < 1e-3 {
            parts.push("Lục xung với chi ngày.".to_string());
        }
    }

    if let Some(warning) = &v1.warning_context {
        parts.push(format!("[Cảnh báo] {}", warning.message_vi));
    }

    parts.join(" ")
}

pub fn build_hour_selection_reasoning(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
    day_assessment: Option<&PersonalDayAssessment>,
) -> Result<HourSelectionReasoning, String> {
    build_hour_selection_reasoning_with_policy(
        day,
        month,
        year,
        intent,
        birth,
        day_assessment,
        HourRankingPolicy::baseline_v1(),
    )
}

/// v2.4 (`amlich-bz0f.4`) full-profile projection of the hour selection
/// reasoning. Uses the [`HourRankingPolicy::full_profile_v2_4`] policy
/// so the personal alignment axis folds in the hour-pillar Thập Thần,
/// hour chi × birth hour chi branch relation, and hour stem element
/// support signals when a full birth profile (date + time) is available.
/// Date-only and anonymous callers collapse to the v1 baseline so the
/// reasoning stays byte-identical for callers without a full profile.
pub fn build_hour_selection_reasoning_full_profile_v2_4(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
    day_assessment: Option<&PersonalDayAssessment>,
) -> Result<HourSelectionReasoning, String> {
    build_hour_selection_reasoning_with_policy(
        day,
        month,
        year,
        intent,
        birth,
        day_assessment,
        HourRankingPolicy::full_profile_v2_4(),
    )
}

/// Shared seam that builds the hour selection reasoning from a
/// caller-supplied [`HourRankingPolicy`]. The reasoning pipeline
/// (ranked hours → top recommendation → Vietnamese / English summary)
/// is policy-agnostic; only the underlying ranking inputs change.
fn build_hour_selection_reasoning_with_policy(
    day: i32,
    month: i32,
    year: i32,
    intent: ConsultationIntent,
    birth: Option<&BirthInput>,
    day_assessment: Option<&PersonalDayAssessment>,
    policy: HourRankingPolicy,
) -> Result<HourSelectionReasoning, String> {
    let snapshot = crate::calculate_day_snapshot(day, month, year);
    let ranked = policy.rank(&snapshot, intent, birth, day_assessment)?;
    let warning_context = ranked.iter().find_map(|hour| hour.warning_context.clone());
    let policy_version = Some(policy.policy_version().to_string());
    let ranked_hours: Vec<RankedHourCandidate> = ranked
        .iter()
        .map(project_ranked_hour_v1_to_legacy_candidate)
        .collect();
    let top_recommendation = ranked_hours.first().cloned();
    let auspicious_count = ranked_hours
        .iter()
        .filter(|hour| hour.is_auspicious)
        .count();
    let summary_vi = match top_recommendation.as_ref() {
        Some(top) => {
            let base = format!(
                "Ưu tiên giờ {} ({}) cho {} vì đứng đầu xếp hạng với {} giờ hoàng đạo hỗ trợ.",
                top.chi_name,
                top.time_range,
                intent.event_kind(),
                auspicious_count
            );
            if let Some(warning) = &warning_context {
                format!("{base} {}", warning.message_vi)
            } else {
                base
            }
        }
        None => format!(
            "Không có giờ phù hợp để xếp hạng cho {}.",
            intent.event_kind()
        ),
    };
    let summary_en = match top_recommendation.as_ref() {
        Some(top) => {
            let base = format!(
                "Prefer the {} hour ({}) for {} because it leads the ranking with {} auspicious windows supporting the day.",
                top.chi_name,
                top.time_range,
                intent.event_kind(),
                auspicious_count
            );
            if let Some(warning) = &warning_context {
                format!("{base} {}", warning.message_vi)
            } else {
                base
            }
        }
        None => format!(
            "No ranked hour candidates are available for {}.",
            intent.event_kind()
        ),
    };

    Ok(HourSelectionReasoning {
        intent,
        summary_vi,
        summary_en,
        top_recommendation,
        ranked_hours,
        warning_context,
        policy_version,
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
        let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
            .expect("ranked hours");

        assert!(!ranked.is_empty());
        assert!(ranked[0].score >= ranked[ranked.len() - 1].score);
    }

    // amlich-rv13.4 — legacy hour-ranking compatibility wrapper tests.
    //
    // The wrapper now delegates to HourRankingPolicy::baseline_v1 and
    // projects the canonical normalized rank_score back to the legacy
    // 0..=100 integer shape. These tests pin the broad-compat contract:
    // every legacy caller keeps working without knowing about the v1
    // policy, and the wrapper preserves the v1 policy's order.
    //
    // amlich-rv13.5 added the `day_assessment` parameter for threaded
    // warning context; the `wrapper_for` helper passes `None` by default
    // so the broad-compat tests stay isolated from the new threading
    // behavior. Tests dedicated to the threaded warning live further
    // down in this module.

    fn wrapper_for(
        date: (i32, i32, i32),
        intent: ConsultationIntent,
        birth: Option<&BirthInput>,
    ) -> Vec<RankedHourCandidate> {
        rank_hours_for_intent(date.0, date.1, date.2, intent, birth, None).expect("ranked hours")
    }

    #[test]
    fn wrapper_returns_all_twelve_ranked_slots() {
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        assert_eq!(ranked.len(), 12);
    }

    #[test]
    fn wrapper_score_projects_to_zero_hundred_range() {
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        for hour in &ranked {
            assert!(
                (0..=100).contains(&hour.score),
                "legacy score must be in 0..=100; got {}",
                hour.score
            );
        }
    }

    #[test]
    fn wrapper_is_auspicious_matches_snapshot_hoang_dao_table() {
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let snapshot_chi: Vec<(&str, &str, bool)> = snapshot
            .context
            .gio_hoang_dao
            .all_hours
            .iter()
            .map(|h| (h.hour_chi.as_str(), h.time_range.as_str(), h.is_good))
            .collect();
        for hour in &ranked {
            let match_in_snapshot = snapshot_chi
                .iter()
                .find(|(chi, range, _)| *chi == hour.chi_name && *range == hour.time_range)
                .expect("wrapper hour must come from snapshot");
            assert_eq!(
                hour.is_auspicious, match_in_snapshot.2,
                "is_auspicious for {} must match snapshot hoang_dao table",
                hour.chi_name
            );
        }
    }

    #[test]
    fn wrapper_hoang_dao_hours_strictly_outrank_hac_dao_hours() {
        // Broad-compat gate from the v1 spec: Hoàng Đạo hours generally
        // rank above Hắc Đạo hours. With intent_timing_fit unavailable
        // and no birth alignment, the wrapper must preserve the v1
        // policy's strict separation.
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        let min_hoang_dao = ranked
            .iter()
            .filter(|h| h.is_auspicious)
            .map(|h| h.score)
            .min()
            .expect("at least one Hoàng Đạo hour");
        let max_hac_dao = ranked
            .iter()
            .filter(|h| !h.is_auspicious)
            .map(|h| h.score)
            .max()
            .expect("at least one Hắc Đạo hour");
        assert!(
            min_hoang_dao > max_hac_dao,
            "Hoàng Đạo hours must strictly outrank Hắc Đạo hours; \
             min_hoang_dao={min_hoang_dao}, max_hac_dao={max_hac_dao}"
        );
    }

    #[test]
    fn wrapper_score_is_monotonic_with_v1_rank_score() {
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let policy = HourRankingPolicy::baseline_v1();
        let v1 = policy
            .rank(&snapshot, ConsultationIntent::Travel, None, None)
            .expect("v1 rank");

        // Pair wrapper hours with their v1 source by (chi_name, time_range).
        for hour in &ranked {
            let v1_hour = v1
                .iter()
                .find(|h| h.chi_name == hour.chi_name && h.time_range == hour.time_range)
                .expect("every wrapper hour must map to a v1 hour");
            let expected = (v1_hour.rank_score * 100.0).round().clamp(0.0, 100.0) as i32;
            assert_eq!(
                hour.score, expected,
                "wrapper score for {} must equal round(v1_rank_score * 100) = {expected}, got {}",
                v1_hour.chi_name, hour.score
            );
        }
    }

    #[test]
    fn wrapper_is_deterministic_for_identical_inputs() {
        let a = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        let b = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        assert_eq!(a, b);
    }

    #[test]
    fn wrapper_ranking_is_invariant_to_intent_in_v1() {
        // intent_timing_fit is uniformly unavailable in v1, so the
        // wrapper's ranking (which derives entirely from the v1 policy)
        // must not change across intents. The legacy Travel-intent
        // +xuat_hanh_huong bonus is intentionally dropped because it
        // folded a daily fact into the per-hour ranking — see
        // amlich-rv13.4 design note.
        let travel = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        let contract = wrapper_for((10, 2, 2024), ConsultationIntent::ContractSigning, None);
        let wedding = wrapper_for((10, 2, 2024), ConsultationIntent::Wedding, None);

        let ids = |r: &[RankedHourCandidate]| -> Vec<(String, String)> {
            r.iter()
                .map(|h| (h.chi_name.clone(), h.time_range.clone()))
                .collect()
        };
        assert_eq!(ids(&travel), ids(&contract));
        assert_eq!(ids(&travel), ids(&wedding));
    }

    #[test]
    fn wrapper_tie_breaks_by_traditional_chi_order_not_alphabetical() {
        // Per spec §"Ranking order", exact-tie tie-break uses
        // chi_index ascending (traditional Chi order). The v1 policy
        // already enforces this; the wrapper must inherit it rather
        // than the prior alphabetical-Vietnamese-name fallback. We
        // verify the property by checking that on any window of equal
        // scores, the wrapper output's chi_index is monotonically
        // non-decreasing.
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let chi_index_for = |chi: &str| -> usize {
            snapshot
                .context
                .gio_hoang_dao
                .all_hours
                .iter()
                .find(|h| h.hour_chi == chi)
                .map(|h| h.hour_index)
                .expect("chi must appear in snapshot table")
        };
        for window in ranked.windows(2) {
            let left = &window[0];
            let right = &window[1];
            if left.score == right.score {
                let left_idx = chi_index_for(&left.chi_name);
                let right_idx = chi_index_for(&right.chi_name);
                assert!(
                    left_idx < right_idx,
                    "score tie {0} must break by chi_index ascending (traditional Chi order); \
                     got {left_chi} (idx={left_idx}) before {right_chi} (idx={right_idx})",
                    left.score,
                    left_chi = left.chi_name,
                    right_chi = right.chi_name,
                );
            }
        }
    }

    #[test]
    fn wrapper_note_vi_starts_with_hoang_dao_clause() {
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        for hour in &ranked {
            if hour.is_auspicious {
                assert!(
                    hour.note_vi.contains("là giờ hoàng đạo"),
                    "Hoàng Đạo note must mention 'là giờ hoàng đạo'; got {:?}",
                    hour.note_vi
                );
            } else {
                assert!(
                    hour.note_vi.contains("không thuộc giờ hoàng đạo"),
                    "Hắc Đạo note must mention 'không thuộc giờ hoàng đạo'; got {:?}",
                    hour.note_vi
                );
            }
        }
    }

    #[test]
    fn wrapper_note_vi_mentions_birth_year_chi_match_when_present() {
        let birth = BirthInput {
            day: 1,
            month: 1,
            year: 1990,
            hour: None,
            minute: None,
            timezone: VIETNAM_TIMEZONE,
            gender: None,
            location_name: None,
        };
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Wedding, Some(&birth));

        // Find the wrapper hour whose v1 axes report a personal
        // alignment match (score ≈ 1.0). That hour's note must carry
        // the legacy "Có đồng khí với chi tuổi." clause.
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let policy = HourRankingPolicy::baseline_v1();
        let v1 = policy
            .rank(&snapshot, ConsultationIntent::Wedding, Some(&birth), None)
            .expect("v1 rank");
        let match_chi: Option<&str> = v1
            .iter()
            .find(|h| {
                h.axes
                    .personal_hour_alignment
                    .score
                    .map(|s| (s - 1.0).abs() < 1e-3)
                    .unwrap_or(false)
            })
            .map(|h| h.chi_name.as_str());

        if let Some(chi) = match_chi {
            let matching_hour = ranked
                .iter()
                .find(|h| h.chi_name == chi)
                .expect("match hour must appear in wrapper output");
            assert!(
                matching_hour.note_vi.contains("Có đồng khí với chi tuổi"),
                "match hour {chi} note must carry the 'đồng khí' clause; got {:?}",
                matching_hour.note_vi
            );
        }
    }

    #[test]
    fn wrapper_note_vi_mentions_branch_relation_when_special() {
        // At least one of tam-hợp / lục-hợp / lục-xung should appear
        // somewhere in the wrapper output for 10/2/2024 — the snapshot
        // has all three branches represented across its twelve slots.
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        let has_branch_clause = ranked.iter().any(|h| {
            h.note_vi.contains("Tam hợp với chi ngày")
                || h.note_vi.contains("Lục hợp với chi ngày")
                || h.note_vi.contains("Lục xung với chi ngày")
        });
        assert!(
            has_branch_clause,
            "at least one wrapper hour must surface a branch-relation clause"
        );
    }

    #[test]
    fn wrapper_note_vi_omits_branch_clause_for_neutral_hours() {
        let ranked = wrapper_for((10, 2, 2024), ConsultationIntent::Travel, None);
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let policy = HourRankingPolicy::baseline_v1();
        let v1 = policy
            .rank(&snapshot, ConsultationIntent::Travel, None, None)
            .expect("v1 rank");

        // For each wrapper hour whose v1 harmony score is the neutral
        // 0.5 baseline, the wrapper note must NOT carry any branch
        // clause (the neutral baseline is information-free in the legacy
        // format).
        for hour in &ranked {
            let v1_hour = v1
                .iter()
                .find(|h| h.chi_name == hour.chi_name && h.time_range == hour.time_range)
                .expect("v1 hour");
            let harmony = v1_hour
                .axes
                .day_hour_harmony
                .score
                .expect("harmony is always available");
            if (harmony - 0.5).abs() < 1e-3 {
                assert!(
                    !hour.note_vi.contains("Tam hợp với chi ngày"),
                    "neutral harmony hour {} must not carry 'Tam hợp' clause",
                    hour.chi_name
                );
                assert!(
                    !hour.note_vi.contains("Lục hợp với chi ngày"),
                    "neutral harmony hour {} must not carry 'Lục hợp' clause",
                    hour.chi_name
                );
                assert!(
                    !hour.note_vi.contains("Lục xung với chi ngày"),
                    "neutral harmony hour {} must not carry 'Lục xung' clause",
                    hour.chi_name
                );
            }
        }
    }

    // -------------------------------------------------------------------
    // amlich-rv13.5 — day-verdict warning threading tests.
    //
    // The wrapper must thread an optional canonical PersonalDayAssessment
    // into the v1 hour-ranking policy. When the assessment's day bucket
    // is `Avoid`, every ranked hour must carry a Vietnamese `[Cảnh báo]`
    // clause in `note_vi` and the reasoning must surface the structured
    // warning. When the assessment is absent or the bucket is anything
    // other than `Avoid`, the warning is omitted — the ranking stays a
    // pure rank-only projection.
    // -------------------------------------------------------------------

    fn forced_avoid_assessment(
        snapshot: &crate::DaySnapshot,
        intent: ConsultationIntent,
    ) -> crate::assessment::PersonalDayAssessment {
        use crate::almanac::tu_menh::Gender;
        use crate::assessment::{PersonalDayAssessmentBuilder, PersonalDayDecision};
        use crate::birth::BirthProfile;
        let profile = BirthProfile {
            day: 1,
            month: 1,
            year: 1990,
            time: None,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(Gender::Male),
            location_name: None,
        };
        let mut assessment =
            PersonalDayAssessmentBuilder::new(snapshot.clone(), profile, intent).build();
        assessment.decision = PersonalDayDecision {
            bucket: crate::reasoning::RecommendationBucket::Avoid,
            ..assessment.decision
        };
        assessment
    }

    fn forced_favorable_assessment(
        snapshot: &crate::DaySnapshot,
        intent: ConsultationIntent,
    ) -> crate::assessment::PersonalDayAssessment {
        use crate::almanac::tu_menh::Gender;
        use crate::assessment::{PersonalDayAssessmentBuilder, PersonalDayDecision};
        use crate::birth::BirthProfile;
        let profile = BirthProfile {
            day: 1,
            month: 1,
            year: 1990,
            time: None,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(Gender::Male),
            location_name: None,
        };
        let mut assessment =
            PersonalDayAssessmentBuilder::new(snapshot.clone(), profile, intent).build();
        assessment.decision = PersonalDayDecision {
            bucket: crate::reasoning::RecommendationBucket::Favorable,
            ..assessment.decision
        };
        assessment
    }

    #[test]
    fn wrapper_with_avoid_day_assessment_attaches_warning_to_note_vi() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let assessment = forced_avoid_assessment(&snapshot, ConsultationIntent::Travel);
        let ranked = rank_hours_for_intent(
            10,
            2,
            2024,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("ranked hours");
        assert_eq!(ranked.len(), 12);
        for hour in &ranked {
            assert!(
                hour.note_vi.contains("[Cảnh báo]"),
                "Avoid day ranking must carry [Cảnh báo] in note_vi for hour {}; got {:?}",
                hour.chi_name,
                hour.note_vi
            );
            // The warning must surface the v1 policy's exact message text.
            assert!(
                hour.note_vi.contains("không thay đổi đánh giá ngày"),
                "Avoid day warning must surface the v1 policy's clarification; got {:?}",
                hour.note_vi
            );
        }
    }

    #[test]
    fn wrapper_without_day_assessment_omits_warning_from_note_vi() {
        let ranked = rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
            .expect("ranked hours");
        for hour in &ranked {
            assert!(
                !hour.note_vi.contains("[Cảnh báo]"),
                "no assessment threaded → note_vi must not carry [Cảnh báo]; got {:?}",
                hour.note_vi
            );
        }
    }

    #[test]
    fn wrapper_with_favorable_day_assessment_omits_warning_from_note_vi() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let assessment = forced_favorable_assessment(&snapshot, ConsultationIntent::Travel);
        let ranked = rank_hours_for_intent(
            10,
            2,
            2024,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("ranked hours");
        for hour in &ranked {
            assert!(
                !hour.note_vi.contains("[Cảnh báo]"),
                "Favorable (Nên) day assessment must NOT attach warning; got {:?}",
                hour.note_vi
            );
        }
    }

    #[test]
    fn wrapper_with_avoid_day_assessment_ranking_is_unchanged_from_no_assessment() {
        // Acceptance criterion: Avoid day assessments do not suppress
        // hour ranking. Order and score identity must be byte-equal to
        // the no-assessment ranking.
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let assessment = forced_avoid_assessment(&snapshot, ConsultationIntent::Travel);
        let no_assessment =
            rank_hours_for_intent(10, 2, 2024, ConsultationIntent::Travel, None, None)
                .expect("no assessment");
        let with_assessment = rank_hours_for_intent(
            10,
            2,
            2024,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("with assessment");
        // Score and order must match; only note_vi is allowed to differ
        // (the warning clause).
        assert_eq!(no_assessment.len(), with_assessment.len());
        for (lhs, rhs) in no_assessment.iter().zip(with_assessment.iter()) {
            assert_eq!(lhs.chi_name, rhs.chi_name);
            assert_eq!(lhs.time_range, rhs.time_range);
            assert_eq!(lhs.is_auspicious, rhs.is_auspicious);
            assert_eq!(
                lhs.score, rhs.score,
                "Avoid day must not change rank score for {}",
                lhs.chi_name
            );
            // note_vi must differ — the Avoid path adds the warning clause.
            assert_ne!(lhs.note_vi, rhs.note_vi);
        }
    }

    #[test]
    fn build_hour_selection_reasoning_with_avoid_day_carries_structured_warning() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let assessment = forced_avoid_assessment(&snapshot, ConsultationIntent::Travel);
        let reasoning = build_hour_selection_reasoning(
            10,
            2,
            2024,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("reasoning");
        let warning = reasoning
            .warning_context
            .as_ref()
            .expect("Avoid day reasoning must carry structured warning_context");
        assert_eq!(
            warning.day_bucket,
            crate::reasoning::RecommendationBucket::Avoid
        );
        assert!(!warning.message_vi.is_empty());
        // The warning text must surface in summary_vi too so non-DTO
        // consumers see the clarification without parsing note_vi.
        assert!(
            reasoning
                .summary_vi
                .contains("không thay đổi đánh giá ngày"),
            "summary_vi must surface the v1 Avoid warning text; got {:?}",
            reasoning.summary_vi
        );
        // And every ranked hour's note_vi must still carry the legacy
        // [Cảnh báo] prefix.
        for hour in &reasoning.ranked_hours {
            assert!(hour.note_vi.contains("[Cảnh báo]"));
        }
    }

    #[test]
    fn build_hour_selection_reasoning_without_day_assessment_omits_warning() {
        let reasoning =
            build_hour_selection_reasoning(10, 2, 2024, ConsultationIntent::Travel, None, None)
                .expect("reasoning");
        assert!(
            reasoning.warning_context.is_none(),
            "no assessment → warning_context must be None"
        );
    }

    #[test]
    fn build_hour_selection_reasoning_with_favorable_day_omits_warning() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let assessment = forced_favorable_assessment(&snapshot, ConsultationIntent::Travel);
        let reasoning = build_hour_selection_reasoning(
            10,
            2,
            2024,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("reasoning");
        assert!(
            reasoning.warning_context.is_none(),
            "Favorable day → warning_context must be None"
        );
    }

    #[test]
    fn hour_selection_export_surfaces_warning_context_structurally() {
        let snapshot = crate::calculate_day_snapshot(10, 2, 2024);
        let assessment = forced_avoid_assessment(&snapshot, ConsultationIntent::Travel);
        let reasoning = build_hour_selection_reasoning(
            10,
            2,
            2024,
            ConsultationIntent::Travel,
            None,
            Some(&assessment),
        )
        .expect("reasoning");
        let export = reasoning.export(None);
        let warning = export
            .warning_context
            .as_ref()
            .expect("export must surface warning_context for Avoid days");
        assert_eq!(
            warning.day_bucket,
            crate::reasoning::RecommendationBucket::Avoid
        );
        // The export's evidence list must include the new warning entry.
        assert!(
            export
                .evidence
                .iter()
                .any(|e| e.source_id == "hour_ranking_warning"),
            "export evidence must include the hour_ranking_warning entry; got {:?}",
            export.evidence
        );
        // Serializing the export to JSON must include warning_context.
        let json = serde_json::to_string(&export).expect("serialize");
        assert!(
            json.contains("\"warning_context\""),
            "Avoid export JSON must include warning_context; got {json}"
        );
    }

    #[test]
    fn hour_selection_export_json_omits_warning_context_when_absent() {
        let reasoning =
            build_hour_selection_reasoning(10, 2, 2024, ConsultationIntent::Travel, None, None)
                .expect("reasoning");
        let export = reasoning.export(None);
        assert!(export.warning_context.is_none());
        let json = serde_json::to_string(&export).expect("serialize");
        assert!(
            !json.contains("\"warning_context\""),
            "absent warning_context must NOT appear in JSON; got {json}"
        );
    }
}
