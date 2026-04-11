// amlich-core - Vietnamese Lunar Calendar Core Library
//
// This library provides comprehensive Vietnamese lunar calendar calculations including:
// - Solar ↔ Lunar date conversion
// - Can Chi (Heavenly Stems & Earthly Branches) calculations
// - Tiết Khí (24 Solar Terms)
// - Giờ Hoàng Đạo (Auspicious Hours)
// - Vietnamese holidays and festivals

pub mod advisory;
pub mod almanac;
pub mod analysis_envelope;
pub mod bazi;
pub mod canchi;
pub mod gio_hoang_dao;
pub mod holiday_data;
pub mod holidays;
pub mod insight_data;
pub mod julian;
pub mod lunar;
pub mod sun;
pub mod tietkhi;
pub mod types;

// Re-export main types
pub use crate::advisory::{
    build_personalized_day_selection, build_recommendation_context, compute_day_context_from_birth,
    rank_dates_for_intent, rank_hours_for_intent, score_day_selection,
    synthesize_advisory_recommendations, AdvisoryScoring, BirthInput, ConsultationIntent,
    DateRangeInput, EvidenceEnvelope, PersonalizedDaySelection, RankedDateCandidate,
    RankedHourCandidate, ScoredAdvice,
};
pub use crate::almanac::cuu_dieu::{compute_cuu_dieu, CuuDieuQuality, CuuDieuResult};
pub use crate::almanac::hoang_oc::{compute_hoang_oc, HoangOcResult};
pub use crate::almanac::kim_lau::{compute_kim_lau, KimLauCategory, KimLauResult};
pub use crate::almanac::phuc_than::{get_phuc_than, PhucThanResult};
pub use crate::almanac::sat_phuong::{get_sat_phuong, SatPhuongResult};
pub use crate::almanac::tam_tai::{compute_tam_tai, TamTaiResult, TamTaiSeverity};
pub use crate::almanac::thap_than::get_thap_than;
pub use crate::almanac::thai_tue::{compute_thai_tue, ThaiTueConflictKind, ThaiTueResult};
pub use crate::almanac::tu_menh::{compute_kua, Gender, KuaGroup, KuaResult};
pub use crate::almanac::types::{HeavenlyStem, ThapThanLabel, ThapThanResult};
pub use crate::almanac::yearly_han::{
    compute_yearly_han, HanSeverity, YearlyHanAssessment, YearlyHanInput,
};
pub use crate::analysis_envelope::AnalysisEnvelope;
pub use crate::bazi::{
    analyze_bazi_chart, build_annual_pillar, build_bazi_advisory, build_bazi_chart,
    build_bazi_report, build_bazi_report_with_options, build_bazi_timing_report,
    build_metrics_from_analysis, build_monthly_pillar, compute_bazi_metrics,
    compute_bazi_metrics_with_matrix, compute_element_distribution, compute_ten_god_distribution,
    default_bazi_scoring_matrix_set, detect_chart_interactions, evaluate_day_master_strength,
    infer_useful_gods, to_bazi_advisory_response, to_bazi_analysis_response,
    to_bazi_chart_response, to_bazi_timing_response, AnnualPillar, AnnualPillarResponse,
    BaziAdvisoryDomains, BaziAdvisoryReport, BaziAdvisoryResponse, BaziAnalysisEnvelope,
    BaziAnalysisReport, BaziAnalysisResponse, BaziCanChiResponse, BaziChart, BaziChartMetadata,
    BaziChartMetadataResponse, BaziChartResponse, BaziComputedMetrics, BaziCoreMetrics,
    BaziDomainScore, BaziDomainScores, BaziInput, BaziInteractionMetric, BaziLuckPillar,
    BaziLuckPillarResponse, BaziLunarDateResponse, BaziPillar, BaziPillarResponse, BaziReport,
    BaziReportFacts, BaziReportOptions, BaziScoreContributor, BaziScoringMatrixSet,
    BaziStructureMetrics, BaziTimingInput, BaziTimingMetrics, BaziTimingReport, BaziTimingResponse,
    BaziTimingWindowScore, BranchStrengthProfile, ChartInteraction, ChartInteractionKind,
    ChartInteractionResponse, DayMasterStrength, DayMasterStrengthLabel, DayMasterStrengthResponse,
    DomainMappingMatrix, DomainWeightProfile, ElementDistribution, ElementRelationMatrix,
    ElementRelationVector, HiddenStemEntry, InteractionImpactMatrix, MonthlyPillar,
    MonthlyPillarResponse, PillarKind, SeasonStrengthMatrix, StemRelationSet, TenGodContextMatrix,
    TenGodDistribution, TenGodWeightProfile, UsefulGodAnalysis, UsefulGodResponse,
};
pub use types::*;

use crate::almanac::calc::calculate_day_fortune;
use crate::almanac::data::get_ruleset;
use crate::almanac::recommendation::{
    synthesize_daily_recommendations, synthesize_daily_recommendations_with_layers,
    DailyRecommendations, RecommendationSynthesisContext,
};
use crate::almanac::types::DayFortune;
use canchi::{get_day_canchi, get_month_canchi, get_year_canchi};
use gio_hoang_dao::{get_gio_hoang_dao, GioHoangDao};
use julian::jd_from_date;
use lunar::{convert_solar_to_lunar, LunarDate};
use tietkhi::{get_tiet_khi, SolarTerm};

#[derive(Debug, Clone)]
pub struct SolarDate {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub day_of_week: usize,
}

#[derive(Debug, Clone)]
pub struct CanChiSet {
    pub day: CanChi,
    pub month: CanChi,
    pub year: CanChi,
}

#[derive(Debug, Clone)]
pub struct DayContext {
    pub solar: SolarDate,
    pub lunar: LunarDate,
    pub jd: i32,
    pub weekday_index: usize,
    pub canchi: CanChiSet,
    pub tiet_khi: SolarTerm,
    pub gio_hoang_dao: GioHoangDao,
}

#[derive(Debug, Clone)]
pub struct DaySnapshot {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub context: DayContext,
    pub day_fortune: DayFortune,
    pub daily_recommendations: DailyRecommendations,
    pub contextual_recommendations: Option<DailyRecommendations>,
}

#[derive(Debug, Clone, Default)]
struct SnapshotRequest<'a> {
    ruleset_id: Option<&'a str>,
    event_kind: Option<&'a str>,
    enabled_pack_ids: &'a [&'a str],
}

pub fn compute_day_context(day: i32, month: i32, year: i32, time_zone: f64) -> DayContext {
    let jd = jd_from_date(day, month, year);
    let lunar = convert_solar_to_lunar(day, month, year, time_zone);
    let weekday_index = ((jd + 1) % 7) as usize;
    let day_canchi = get_day_canchi(jd);
    let month_canchi = get_month_canchi(lunar.month, lunar.year, lunar.is_leap);
    let year_canchi = get_year_canchi(lunar.year);
    let tiet_khi = get_tiet_khi(jd, time_zone);
    let gio_hoang_dao = get_gio_hoang_dao(day_canchi.chi_index);

    DayContext {
        solar: SolarDate {
            day,
            month,
            year,
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

pub fn calculate_day_snapshot(day: i32, month: i32, year: i32) -> DaySnapshot {
    calculate_day_snapshot_with_timezone(day, month, year, VIETNAM_TIMEZONE)
}

pub fn calculate_day_snapshot_with_recommendation_request(
    day: i32,
    month: i32,
    year: i32,
    time_zone: f64,
    ruleset_id: Option<&str>,
    event_kind: Option<&str>,
    enabled_pack_ids: &[&str],
) -> Result<DaySnapshot, String> {
    calculate_day_snapshot_internal(
        day,
        month,
        year,
        time_zone,
        SnapshotRequest {
            ruleset_id,
            event_kind,
            enabled_pack_ids,
        },
    )
}

pub fn calculate_day_snapshot_with_timezone(
    day: i32,
    month: i32,
    year: i32,
    time_zone: f64,
) -> DaySnapshot {
    calculate_day_snapshot_internal(day, month, year, time_zone, SnapshotRequest::default())
        .expect("default recommendation request should be valid")
}

fn calculate_day_snapshot_internal(
    day: i32,
    month: i32,
    year: i32,
    time_zone: f64,
    recommendation_request: SnapshotRequest<'_>,
) -> Result<DaySnapshot, String> {
    let ruleset_entry = match recommendation_request.ruleset_id {
        Some(ruleset_id) => Some(get_ruleset(ruleset_id).map_err(|err| err.to_string())?),
        None => None,
    };

    let context = compute_day_context(day, month, year, time_zone);
    let day_fortune = calculate_day_fortune(
        context.jd,
        &context.canchi.day,
        context.lunar.day,
        context.lunar.month,
        &context.canchi.year.can,
        &context.tiet_khi.name,
    );
    let recommendation_context = RecommendationSynthesisContext {
        day_chi: &context.canchi.day.chi,
        day_fortune: &day_fortune,
        gio_hoang_dao: Some(&context.gio_hoang_dao),
        tiet_khi_name: Some(&context.tiet_khi.name),
        profile_id: ruleset_entry.map(|entry| entry.descriptor.profile),
        event_kind: None,
        enabled_pack_ids: &[],
    };
    let daily_recommendations = synthesize_daily_recommendations(&recommendation_context);
    let contextual_recommendations = if recommendation_request.event_kind.is_some()
        || !recommendation_request.enabled_pack_ids.is_empty()
    {
        let contextual_context = RecommendationSynthesisContext {
            day_chi: &context.canchi.day.chi,
            day_fortune: &day_fortune,
            gio_hoang_dao: Some(&context.gio_hoang_dao),
            tiet_khi_name: Some(&context.tiet_khi.name),
            profile_id: Some("contextual"),
            event_kind: recommendation_request.event_kind,
            enabled_pack_ids: recommendation_request.enabled_pack_ids,
        };
        Some(
            synthesize_daily_recommendations_with_layers(&contextual_context, &[])
                .map_err(|err| err.to_string())?,
        )
    } else {
        None
    };

    Ok(DaySnapshot {
        ruleset_id: day_fortune.ruleset_id.clone(),
        ruleset_version: day_fortune.ruleset_version.clone(),
        profile: day_fortune.profile.clone(),
        context,
        day_fortune,
        daily_recommendations,
        contextual_recommendations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_day_context_exposes_structured_calendar_facts() {
        let context = compute_day_context(10, 2, 2024, VIETNAM_TIMEZONE);

        assert_eq!(context.solar.day, 10);
        assert_eq!(context.solar.month, 2);
        assert_eq!(context.solar.year, 2024);
        assert_eq!(context.weekday_index, context.solar.day_of_week);
        assert_eq!(context.lunar.day, 1);
        assert_eq!(context.lunar.month, 1);
        assert_eq!(context.lunar.year, 2024);
        assert!(!context.lunar.is_leap);
        assert_eq!(context.canchi.day.full, "Giáp Thìn");
        assert_eq!(context.canchi.year.full, "Giáp Thìn");
        assert_eq!(context.tiet_khi.name, "Lập Xuân");
        assert_eq!(context.gio_hoang_dao.good_hour_count, 6);
    }

    #[test]
    fn compute_day_context_supports_custom_timezone() {
        let context = compute_day_context(10, 2, 2024, 8.0);

        assert_eq!(context.solar.day, 10);
        assert_eq!(context.solar.month, 2);
        assert_eq!(context.solar.year, 2024);
    }

    #[test]
    fn calculate_day_snapshot_keeps_recommendations_and_ruleset_metadata() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);

        assert_eq!(snapshot.ruleset_id, "vn_baseline_v1");
        assert_eq!(snapshot.ruleset_version, "v1");
        assert_eq!(snapshot.profile, "baseline");
        assert!(!snapshot.daily_recommendations.activities.is_empty());
        assert!(!snapshot.daily_recommendations.summary_vi.is_empty());
        assert_eq!(
            snapshot.daily_recommendations.ruleset_id,
            snapshot.ruleset_id
        );
        assert_eq!(
            snapshot.daily_recommendations.ruleset_version,
            snapshot.ruleset_version
        );
        assert_eq!(snapshot.daily_recommendations.profile, snapshot.profile);
        assert!(snapshot.contextual_recommendations.is_none());
    }

    #[test]
    fn calculate_day_snapshot_emits_contextual_recommendations_when_requested() {
        let snapshot = calculate_day_snapshot_with_recommendation_request(
            10,
            2,
            2024,
            VIETNAM_TIMEZONE,
            None,
            Some("contract_signing"),
            &[],
        )
        .expect("contextual day snapshot");

        let contextual = snapshot
            .contextual_recommendations
            .as_ref()
            .expect("contextual recommendations");
        assert!(contextual
            .activities
            .iter()
            .find(|activity| activity.activity_id
                == crate::almanac::recommendation::ActivityId::ContractAgreement)
            .expect("contract activity")
            .reasons
            .iter()
            .any(|reason| reason.rule_id == "layer.product_rule.event_kind.contract_signing"));
    }

    #[test]
    fn calculate_day_snapshot_rejects_unknown_pack_ids() {
        let err = calculate_day_snapshot_with_recommendation_request(
            10,
            2,
            2024,
            VIETNAM_TIMEZONE,
            None,
            None,
            &["pack.unknown.v1"],
        )
        .expect_err("unknown pack should fail");

        assert_eq!(err, "unknown recommendation pack id: pack.unknown.v1");
    }
}
