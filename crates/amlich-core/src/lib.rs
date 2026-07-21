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
pub mod birth;
pub mod canchi;
pub mod gio_hoang_dao;
pub mod holiday_data;
pub mod holidays;
pub mod iching;
pub mod insight_data;
pub mod interaction;
pub mod julian;
pub mod lunar;
pub mod reasoning;
pub mod rituals;
pub mod semantic_graph;
pub mod sources;
pub mod sun;
pub mod tietkhi;
pub mod types;

// Re-export main types
pub use crate::advisory::{
    build_hour_selection_reasoning, build_personalized_day_selection, build_recommendation_context,
    compute_day_context_from_birth, rank_dates_for_intent, rank_hours_for_intent,
    score_day_selection, synthesize_advisory_recommendations, AdvisoryScoring, BirthInput,
    ConsultationIntent, DateRangeInput, EvidenceEnvelope, HourSelectionEvidence,
    HourSelectionReasoning, HourSelectionReasoningExport, PersonalizedDaySelection,
    RankedDateCandidate, RankedHourCandidate, ScoredAdvice,
};
pub use crate::almanac::cuu_dieu::{compute_cuu_dieu, CuuDieuQuality, CuuDieuResult};
pub use crate::almanac::hoang_oc::{compute_hoang_oc, HoangOcResult};
pub use crate::almanac::kim_lau::{compute_kim_lau, KimLauCategory, KimLauResult};
pub use crate::almanac::phuc_than::{get_phuc_than, PhucThanResult};
pub use crate::almanac::sat_phuong::{get_sat_phuong, SatPhuongResult};
pub use crate::almanac::tam_tai::{compute_tam_tai, TamTaiResult, TamTaiSeverity};
pub use crate::almanac::thai_tue::{compute_thai_tue, ThaiTueConflictKind, ThaiTueResult};
pub use crate::almanac::thap_than::get_thap_than;
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
    export_bazi_advisory, infer_useful_gods, to_bazi_advisory_response, to_bazi_analysis_response,
    to_bazi_chart_response, to_bazi_timing_response, AnnualPillar, AnnualPillarResponse,
    BaziAdvisoryDomains, BaziAdvisoryExport, BaziAdvisoryReport, BaziAdvisoryResponse,
    BaziAnalysisEnvelope, BaziAnalysisReport, BaziAnalysisResponse, BaziCanChiResponse, BaziChart,
    BaziChartMetadata, BaziChartMetadataResponse, BaziChartResponse, BaziComputedMetrics,
    BaziCoreMetrics, BaziDomainScore, BaziDomainScores, BaziInput, BaziInteractionMetric,
    BaziLuckPillar, BaziLuckPillarResponse, BaziLunarDateResponse, BaziPillar, BaziPillarResponse,
    BaziReport, BaziReportFacts, BaziReportOptions, BaziScoreContributor, BaziScoringMatrixSet,
    BaziStructureMetrics, BaziTimingInput, BaziTimingMetrics, BaziTimingReport, BaziTimingResponse,
    BaziTimingWindowScore, BranchStrengthProfile, ChartInteraction, ChartInteractionKind,
    ChartInteractionResponse, DayMasterStrength, DayMasterStrengthLabel, DayMasterStrengthResponse,
    DomainMappingMatrix, DomainWeightProfile, ElementDistribution, ElementRelationMatrix,
    ElementRelationVector, HiddenStemEntry, InteractionImpactMatrix, MonthlyPillar,
    MonthlyPillarResponse, PillarKind, SeasonStrengthMatrix, StemRelationSet, TenGodContextMatrix,
    TenGodDistribution, TenGodWeightProfile, UsefulGodAnalysis, UsefulGodResponse,
};
pub use crate::birth::{BirthCapability, BirthDataTier, BirthProfile, BirthTime};
pub use crate::reasoning::{
    ActionId, DecisionConfidence, EdgeEffect, InitiationOpeningDecision,
    InitiationOpeningDecisionExport, InitiationOpeningReasoningBundle, InitiationOpeningVector,
    InterpretedAxis, NodeKind, PersonalFactNode, ReasoningAxisScore, ReasoningConclusionSemantic,
    ReasoningEdgeExport, ReasoningEdgeJustification, ReasoningEvidenceEnvelope,
    ReasoningEvidenceSourceFamily, ReasoningGraphExport, ReasoningNodeExport,
    ReasoningNodeSeverity, ReasoningNote, RecommendationBucket,
};
pub use semantic_graph::{
    build_reasoning_input_graph, debug_inspect_semantic_graph, ConceptLabel, DebugInspectionDate,
    DebugInspectionSummary, DebugSemanticGraphInspection, EdgeConcept, GraphOntology,
    GraphValidationError, NodeConcept, NodeOrigin, ProvenanceEntry, ProvenanceSource,
    ProvenanceTracker, SemanticEdge, SemanticEdgeLabel, SemanticGraph, SemanticId, SemanticNode,
    SemanticNodeId,
};
pub use types::*;

use crate::reasoning::{build_initiation_opening_decision, PersonalReasoningInput};

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

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarDate {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub day_of_week: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanChiSet {
    pub day: CanChi,
    pub month: CanChi,
    pub year: CanChi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayContext {
    pub solar: SolarDate,
    pub lunar: LunarDate,
    pub jd: i32,
    pub weekday_index: usize,
    pub canchi: CanChiSet,
    pub tiet_khi: SolarTerm,
    pub gio_hoang_dao: GioHoangDao,
}

/// Slim DTO summarising the Phi Tinh (Flying Stars) overlay for a snapshot day.
///
/// Contains only scalar/array fields — no nested evidence envelopes — per research Q1.
/// Derived from `compute_combined_overlay` inside `calculate_day_snapshot_internal`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlyingStarsSummary {
    /// Active Vận period number (1-9).
    pub van: u8,
    /// Solar year the layout was computed for.
    pub year: i32,
    /// Solar month (1-based) the monthly layer was computed for.
    pub month: u8,
    /// Center palace star from the annual (Niên) layer.
    pub center_star: crate::almanac::fengshui::types::FlyingStar,
    /// Per-palace (annual, monthly) star pairs, Palace::ALL order.
    pub palace_overlays: [(
        crate::almanac::fengshui::types::FlyingStar,
        crate::almanac::fengshui::types::FlyingStar,
    ); 9],
    /// Additive optional annual palace safety-hint projection (Phase 23-02,
    /// XLK-03). Each entry mirrors the `palace_overlays` order and carries
    /// the Vietnamese safety-hint text for the ANNUAL star at that palace
    /// (`None` for auspicious stars that have no mitigation hint). Populated
    /// by `calculate_day_snapshot_internal` at the snapshot boundary so the
    /// later reasoning cross-link consumes only this DTO (no lower-level
    /// imports). Absent from JSON when the whole field is `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub palace_safety_hints: Option<[Option<String>; 9]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaySnapshot {
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub context: DayContext,
    pub day_fortune: DayFortune,
    pub daily_recommendations: DailyRecommendations,
    pub contextual_recommendations: Option<DailyRecommendations>,
    /// Additive optional Phi Tinh (Flying Stars) overlay. Absent in JSON when None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flying_stars: Option<FlyingStarsSummary>,
    /// Additive optional list of matching ritual ids from the Văn khấn corpus.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applicable_rituals: Option<Vec<String>>,
    /// Additive optional daily Phi Tinh (Lưu Nhật / 日紫白) overlay.
    /// Absent in JSON when None. Phase 18-04 (FS-19). Auto-populated
    /// alongside `flying_stars` in `calculate_day_snapshot_internal`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub daily_flying_stars: Option<crate::almanac::fengshui::types::DailyFlyingStarLayout>,
    /// Additive optional structured offering handles (Phase 19, INT-08, preferred
    /// path). Each `OfferingRef` is derived from the matching ritual entry's
    /// `offerings: Vec<Offering>` field — see `rituals::OfferingRef` for the
    /// locked identity tuple. Absent in JSON when None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offering_refs: Option<Vec<crate::rituals::OfferingRef>>,
    /// Additive optional flat-string summary of offering names (Phase 19, INT-08,
    /// legacy BC path). Auto-populated from `offering_refs` (flattened `name_vi`
    /// values, deduplicated). Carries no `offering_id` or `source_id` — use
    /// `offering_refs` for structured queries. Absent in JSON when None.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offerings: Option<Vec<String>>,
    /// Additive optional directional cross-link summary (Phase 23-02, XLK-03).
    /// Defaults to `None` — no calculation path in `calculate_day_snapshot_internal`
    /// auto-populates it; the explicit enrichment helper shipped by the
    /// implementation plan clones the snapshot and attaches the summary. Absent
    /// from JSON when None so the v1.6 → v1.7 round-trip stays byte-equal.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction_cross_link: Option<crate::reasoning::DirectionCrossLinkSummary>,
    /// Additive optional I Ching consultation summary (Phase 24-01, ICH-05 +
    /// partial INT-12). Populated only via the explicit
    /// [`enrich_day_snapshot_with_iching`] helper. Ordinary
    /// [`calculate_day_snapshot`] calls leave this as `None` — no auto-cast
    /// is invented. Absent from JSON when None (additive
    /// `Option<T>` + `serde(default, skip_serializing_if = "Option::is_none")`
    /// discipline).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iching_cast: Option<crate::iching::IChingCastSummary>,
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

pub fn build_initiation_opening_reasoning(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<InitiationOpeningDecision, String> {
    build_initiation_opening_decision(snapshot, personal_input)
}

pub fn build_initiation_opening_reasoning_bundle(
    snapshot: &DaySnapshot,
    personal_input: Option<&PersonalReasoningInput>,
) -> Result<reasoning::InitiationOpeningReasoningBundle, String> {
    reasoning::build_initiation_opening_reasoning_bundle(snapshot, personal_input)
}

/// Phase 23 (XLK-03) immutable enrichment: clone the snapshot, build the
/// directional cross-link, project it to the slim DTO form, and attach it
/// to the `direction_cross_link` field on the cloned snapshot.
///
/// Dispatches on the [`DATE_ONLY_BIRTH_CHI_INDEX`][reasoning::DATE_ONLY_BIRTH_CHI_INDEX]
/// sentinel: passing `usize::MAX` selects the date-only builder; any other
/// value selects the validated personal builder (which requires
/// `birth_chi_index < 12`). The input snapshot is never mutated.
///
/// Ordinary [`calculate_day_snapshot`] / [`calculate_day_snapshot_with_timezone`]
/// calls leave `direction_cross_link` as `None`; only this helper populates
/// it. The result is absent from JSON when None so v1.6 → v1.7 round-trip
/// stays byte-equal.
pub fn enrich_day_snapshot_with_direction_cross_link(
    snapshot: &DaySnapshot,
    birth_chi_index: usize,
) -> Result<DaySnapshot, String> {
    use reasoning::direction_composite::{
        build_direction_cross_link_date, build_direction_cross_link_personal, project_to_summary,
        DATE_ONLY_BIRTH_CHI_INDEX,
    };
    let summary = if birth_chi_index == DATE_ONLY_BIRTH_CHI_INDEX {
        project_to_summary(&build_direction_cross_link_date(snapshot)?)
    } else {
        project_to_summary(&build_direction_cross_link_personal(
            snapshot,
            birth_chi_index,
        )?)
    };
    let mut enriched = snapshot.clone();
    enriched.direction_cross_link = Some(summary);
    Ok(enriched)
}

/// Phase 24-01 (ICH-05 + partial INT-12) immutable enrichment: clone the
/// snapshot, run the [`crate::iching::IChingEvaluator`] over an explicit
/// [`crate::iching::IChingQuery`] (Tier-0 path, no birth data), and attach
/// the resulting [`crate::iching::IChingCastSummary`] DTO to the
/// `iching_cast` field on the cloned snapshot.
///
/// The input snapshot is never mutated. Ordinary
/// [`calculate_day_snapshot`] / [`calculate_day_snapshot_with_timezone`]
/// calls leave `iching_cast` as `None`; only this helper populates it. The
/// result is absent from JSON when None (additive `Option<T>` discipline)
/// so the v1.6 → v1.7 round-trip stays byte-equal.
///
/// Mirrors the immutable clone-and-attach pattern of
/// [`enrich_day_snapshot_with_direction_cross_link`] (Phase 23-03).
///
/// Reachable as both `amlich_core::enrich_day_snapshot_with_iching` (this
/// site) and `amlich_core::iching::enrich_day_snapshot_with_iching`
/// (re-exported from `iching/mod.rs` so callers in the iching module
/// namespace can use it directly).
pub fn enrich_day_snapshot_with_iching(
    snapshot: &DaySnapshot,
    query: crate::iching::IChingQuery,
) -> Result<DaySnapshot, String> {
    let evaluator = crate::iching::IChingEvaluator::new(query);
    let summary = evaluator.evaluate(snapshot)?;
    let mut enriched = snapshot.clone();
    enriched.iching_cast = Some(summary);
    Ok(enriched)
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

    // Build snapshot first without additive fields so we can pass it to find_van_khan_for_snapshot.
    let mut snap = DaySnapshot {
        ruleset_id: day_fortune.ruleset_id.clone(),
        ruleset_version: day_fortune.ruleset_version.clone(),
        profile: day_fortune.profile.clone(),
        context,
        day_fortune,
        daily_recommendations,
        contextual_recommendations,
        flying_stars: None,
        applicable_rituals: None,
        daily_flying_stars: None,
        offering_refs: None,
        offerings: None,
        direction_cross_link: None,
        iching_cast: None,
    };

    // Populate flying_stars from the combined Phi Tinh overlay.
    {
        use crate::almanac::fengshui::types::FlyingStarPeriod;
        use crate::almanac::fengshui::{
            compute_combined_overlay, element_hint_for_palace, TietKhiScanner,
        };
        let scanner = TietKhiScanner::new();
        let lunar_month = snap.context.lunar.month as u8;
        let solar_year = snap.context.solar.year;
        let overlay = compute_combined_overlay(solar_year, lunar_month, &scanner);
        let van = if let FlyingStarPeriod::Van { van } = overlay.van_layout.period {
            van
        } else {
            1 // fallback; period is always Van for a solar year
        };
        // Pre-bake the annual palace safety-hint Vietnamese text at the DTO
        // boundary so the later reasoning cross-link consumes only the
        // snapshot field (no lower-level imports). Mirrors the `palace_overlays`
        // order; `None` entries mark auspicious stars with no mitigation hint.
        let palace_safety_hints: [Option<String>; 9] = std::array::from_fn(|i| {
            element_hint_for_palace(overlay.palace_overlays[i].0)
                .map(|hint| hint.hint_text_vi.clone())
        });
        snap.flying_stars = Some(FlyingStarsSummary {
            van,
            year: overlay.year,
            month: overlay.month,
            center_star: overlay.annual_layout.center_star,
            palace_overlays: overlay.palace_overlays,
            palace_safety_hints: Some(palace_safety_hints),
        });
    }

    // Populate daily_flying_stars from the daily Phi Tinh (Lưu Nhật) overlay.
    {
        use crate::almanac::fengshui::{compute_daily_flying_stars, TietKhiScanner};
        let scanner = TietKhiScanner::new();
        let solar_year = snap.context.solar.year;
        let solar_month = snap.context.solar.month as u32;
        let solar_day = snap.context.solar.day as u32;
        snap.daily_flying_stars = Some(compute_daily_flying_stars(
            (solar_year, solar_month, solar_day),
            &scanner,
        ));
    }

    // Populate applicable_rituals from the ritual corpus.
    {
        use crate::rituals::find_van_khan_for_snapshot;
        let ritual_ids: Vec<String> = find_van_khan_for_snapshot(&snap)
            .iter()
            .map(|r| r.ritual_id.clone())
            .collect();
        snap.applicable_rituals = Some(ritual_ids);
    }

    // Populate offering_refs (structured) + offerings (legacy flat-string)
    // from the ritual corpus. Both fields derived from the same source —
    // offering_refs is the preferred path, offerings is the legacy summary.
    // Phase 19-01 (INT-08). The flat-string list is deduped by name_vi and
    // preserves insertion order (matches the literal SC text per Q4 interpretation i).
    {
        use crate::rituals::get_ritual_by_id;
        use crate::sources::SOURCE_VN_FOLK_RITUAL;

        let mut offering_refs: Vec<crate::rituals::OfferingRef> = Vec::new();
        let mut offerings_flat: Vec<String> = Vec::new();

        if let Some(ritual_ids) = &snap.applicable_rituals {
            for ritual_id in ritual_ids {
                let Some(entry) = get_ritual_by_id(ritual_id) else {
                    continue;
                };
                for (idx, offering) in entry.offerings.iter().enumerate() {
                    let offering_ref = crate::rituals::OfferingRef::new(
                        format!("ritual.{ritual_id}.offering.{idx}"),
                        offering.name_vi.clone(),
                        offering.name_en.clone(),
                        SOURCE_VN_FOLK_RITUAL.to_string(),
                    );
                    if !offerings_flat.contains(&offering.name_vi) {
                        offerings_flat.push(offering.name_vi.clone());
                    }
                    offering_refs.push(offering_ref);
                }
            }
        }

        snap.offering_refs = if offering_refs.is_empty() {
            None
        } else {
            Some(offering_refs)
        };
        snap.offerings = if offerings_flat.is_empty() {
            None
        } else {
            Some(offerings_flat)
        };
    }

    Ok(snap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_snapshot_serde_round_trip() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let json = serde_json::to_string(&snapshot).expect("serialization failed");
        let roundtripped: DaySnapshot =
            serde_json::from_str(&json).expect("deserialization failed");
        assert_eq!(roundtripped.ruleset_id, snapshot.ruleset_id);
        assert_eq!(roundtripped.profile, snapshot.profile);
        assert_eq!(roundtripped.context.solar.day, snapshot.context.solar.day);
        assert_eq!(
            roundtripped.context.solar.month,
            snapshot.context.solar.month
        );
        assert_eq!(roundtripped.context.solar.year, snapshot.context.solar.year);
    }

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

    #[test]
    fn day_snapshot_populates_additive_surfaces() {
        let snapshot = calculate_day_snapshot(17, 2, 2026);

        // flying_stars must be populated
        let fs = snapshot
            .flying_stars
            .as_ref()
            .expect("flying_stars must be Some");
        assert_eq!(fs.palace_overlays.len(), 9);

        // applicable_rituals must be populated (may be empty vec but not None)
        assert!(snapshot.applicable_rituals.is_some());

        // When fields are None, they must not appear in serialized JSON
        let mut none_snapshot = snapshot.clone();
        none_snapshot.flying_stars = None;
        none_snapshot.applicable_rituals = None;
        let json = serde_json::to_string(&none_snapshot).expect("serialization failed");
        assert!(
            !json.contains("\"flying_stars\""),
            "flying_stars must not appear in JSON when None"
        );
        assert!(
            !json.contains("\"applicable_rituals\""),
            "applicable_rituals must not appear in JSON when None"
        );
    }

    // Phase 19-01 focused populate test (INT-08, warning 1 fix):
    // asserts the additive offering_refs + offerings fields specifically
    // (the existing `day_snapshot_populates_additive_surfaces` only checks
    // `flying_stars` + `applicable_rituals`).
    #[test]
    fn day_snapshot_offering_refs_populated_and_deduped() {
        use crate::sources::SOURCE_VN_FOLK_RITUAL;

        let snap = calculate_day_snapshot(17, 2, 2026); // Tết 2026 — guarantees applicable_rituals non-empty

        // 1. Both fields populated
        let refs = snap
            .offering_refs
            .as_ref()
            .expect("offering_refs must be Some when applicable_rituals is non-empty");
        assert!(
            !refs.is_empty(),
            "offering_refs must be non-empty for Tết 2026"
        );
        let flat = snap
            .offerings
            .as_ref()
            .expect("offerings (flat-string) must be Some when applicable_rituals is non-empty");
        assert!(
            !flat.is_empty(),
            "offerings (flat-string) must be non-empty for Tết 2026"
        );

        // 2. Identity: offering_id is non-empty, follows "ritual.{ritual_id}.offering.{idx}" pattern
        let first = &refs[0];
        assert!(
            !first.offering_id.is_empty(),
            "OfferingRef.offering_id must be non-empty"
        );
        assert!(
            first.offering_id.starts_with("ritual."),
            "OfferingRef.offering_id must follow ritual.<id>.offering.<idx> pattern; got {:?}",
            first.offering_id
        );

        // 3. Source-id discipline: every OfferingRef.source_id == "vn-folk-ritual"
        for r in refs {
            assert_eq!(
                r.source_id, SOURCE_VN_FOLK_RITUAL,
                "OfferingRef.source_id must equal vn-folk-ritual; got {:?}",
                r.source_id
            );
        }

        // 4. Dedup: the flat-string offerings Vec is a deduped subset of OfferingRef.name_vi values
        for r in refs {
            assert!(
                flat.contains(&r.name_vi),
                "flat-string offerings must contain every OfferingRef.name_vi = {:?}",
                r.name_vi
            );
        }
        // And the flat-string Vec itself contains no duplicates
        let mut seen = std::collections::HashSet::new();
        for name in flat {
            assert!(
                seen.insert(name.clone()),
                "flat-string offerings must be deduplicated; found duplicate: {:?}",
                name
            );
        }

        // 5. None behavior: explicitly clear both fields, verify None → absent in JSON
        let mut snap_none = snap.clone();
        snap_none.offering_refs = None;
        snap_none.offerings = None;
        let json = serde_json::to_string(&snap_none).expect("serialization failed");
        assert!(
            !json.contains("\"offering_refs\""),
            "offering_refs must NOT appear in JSON when None; got: {json}"
        );
        assert!(
            !json.contains("\"offerings\""),
            "offerings must NOT appear in JSON when None; got: {json}"
        );
    }

    // Phase 23-02 Task 2: additive DTO transport contracts.

    #[test]
    fn day_snapshot_direction_cross_link_defaults_to_none() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        assert!(
            snapshot.direction_cross_link.is_none(),
            "direction_cross_link must default to None; no calculation path auto-populates it"
        );
        let json = serde_json::to_string(&snapshot).expect("serialize");
        assert!(
            !json.contains("\"direction_cross_link\""),
            "direction_cross_link must NOT appear in JSON when None; got: {json}"
        );
    }

    #[test]
    fn flying_stars_summary_carries_palace_safety_hints() {
        let snapshot = calculate_day_snapshot(10, 2, 2024);
        let fs = snapshot
            .flying_stars
            .as_ref()
            .expect("flying_stars must be Some");
        let hints = fs
            .palace_safety_hints
            .as_ref()
            .expect("palace_safety_hints must be populated by the snapshot constructor");
        assert_eq!(
            hints.len(),
            9,
            "palace_safety_hints must follow the 9-palace overlay order"
        );
    }

    #[test]
    fn v16_json_without_palace_safety_hints_deserializes() {
        // A v1.6-era JSON that predates palace_safety_hints must still
        // deserialize cleanly into the v1.7 shape with the field defaulting to None.
        let snap = calculate_day_snapshot(10, 2, 2024);
        let mut value: serde_json::Value = serde_json::to_value(&snap).expect("serialize to value");
        if let Some(fs) = value
            .get_mut("flying_stars")
            .and_then(|v| v.as_object_mut())
        {
            fs.remove("palace_safety_hints");
        }
        let stripped = serde_json::to_string(&value).expect("reserialize stripped value");
        assert!(
            !stripped.contains("\"palace_safety_hints\""),
            "test precondition: stripped JSON must not contain palace_safety_hints"
        );
        let recovered: DaySnapshot =
            serde_json::from_str(&stripped).expect("deserialize pre-v1.7 JSON");
        assert!(
            recovered
                .flying_stars
                .as_ref()
                .unwrap()
                .palace_safety_hints
                .is_none(),
            "missing palace_safety_hints must default to None"
        );
    }
}
