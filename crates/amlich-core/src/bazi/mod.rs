pub mod advisory;
pub mod analysis;
pub mod chart;
pub mod contracts;
pub mod derived;
pub mod khong_vong;
pub mod report;
pub mod scoring;
pub mod than_sat;
pub mod timing;
pub mod types;

pub use advisory::{
    build_bazi_advisory, infer_useful_gods, BaziAdvisoryDomains, BaziAdvisoryReport,
    UsefulGodAnalysis,
};
pub use analysis::{
    analyze_bazi_chart, compute_element_distribution, compute_ten_god_distribution,
    detect_chart_interactions, evaluate_day_master_strength, BaziAnalysisReport, ChartInteraction,
    ChartInteractionKind, DayMasterStrength, DayMasterStrengthLabel, ElementDistribution,
    TenGodDistribution,
};
pub use chart::build_bazi_chart;
pub use contracts::{
    to_bazi_advisory_response, to_bazi_analysis_response, to_bazi_chart_response,
    to_bazi_timing_response, AnnualPillarResponse, BaziAdvisoryResponse, BaziAnalysisResponse,
    BaziCanChiResponse, BaziChartMetadataResponse, BaziChartResponse, BaziLuckPillarResponse,
    BaziLunarDateResponse, BaziPillarResponse, BaziTimingResponse, ChartInteractionResponse,
    DayMasterStrengthResponse, MonthlyPillarResponse, UsefulGodResponse,
};
pub use report::{
    build_bazi_report, build_bazi_report_with_options, BaziAnalysisEnvelope, BaziReport,
    BaziReportFacts, BaziReportOptions, BaziTimingInput,
};
pub use scoring::{
    build_metrics_from_analysis, compute_bazi_metrics, compute_bazi_metrics_with_matrix,
    default_bazi_scoring_matrix_set, BaziComputedMetrics, BaziCoreMetrics, BaziDomainScore,
    BaziDomainScores, BaziInteractionMetric, BaziScoreContributor, BaziScoringMatrixSet,
    BaziStructureMetrics, BaziTimingMetrics, BaziTimingWindowScore, BranchStrengthProfile,
    DomainMappingMatrix, DomainWeightProfile, ElementRelationMatrix, ElementRelationVector,
    InteractionImpactMatrix, SeasonStrengthMatrix, TenGodContextMatrix, TenGodWeightProfile,
};
pub use timing::{
    build_annual_pillar, build_bazi_timing_report, build_monthly_pillar, AnnualPillar,
    BaziLuckPillar, BaziTimingReport, MonthlyPillar,
};
pub use derived::{compute_menh_than_cung, compute_thai_nguyen};
pub use khong_vong::compute_khong_vong;
pub use than_sat::compute_than_sat;
pub use types::{
    BaziChart, BaziChartMetadata, BaziDerivedReport, BaziInput, BaziPillar, HiddenStemEntry,
    KhongVongAnalysis, KhongVongPair, KhongVongPillarEntry, MenhCungResult, PillarKind,
    StemRelationSet, ThaiNguyenResult, ThanSatEntry, ThanSatResult, ThanSatSource,
};
