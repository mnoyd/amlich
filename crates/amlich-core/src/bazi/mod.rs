pub mod advisory;
pub mod analysis;
pub mod chart;
pub mod contracts;
pub mod timing;
pub mod types;

pub use advisory::{BaziAdvisoryDomains, BaziAdvisoryReport, UsefulGodAnalysis, build_bazi_advisory, infer_useful_gods};
pub use analysis::{
    BaziAnalysisReport, ChartInteraction, ChartInteractionKind, DayMasterStrength,
    DayMasterStrengthLabel, ElementDistribution, TenGodDistribution, analyze_bazi_chart,
    compute_element_distribution, compute_ten_god_distribution, detect_chart_interactions,
    evaluate_day_master_strength,
};
pub use chart::build_bazi_chart;
pub use contracts::{
    AnnualPillarResponse, BaziAdvisoryResponse, BaziAnalysisResponse, BaziCanChiResponse,
    BaziChartMetadataResponse, BaziChartResponse, BaziLuckPillarResponse, BaziLunarDateResponse,
    BaziPillarResponse, BaziTimingResponse, ChartInteractionResponse, DayMasterStrengthResponse,
    MonthlyPillarResponse, UsefulGodResponse, to_bazi_advisory_response,
    to_bazi_analysis_response, to_bazi_chart_response, to_bazi_timing_response,
};
pub use timing::{
    AnnualPillar, BaziLuckPillar, BaziTimingReport, MonthlyPillar, build_annual_pillar,
    build_bazi_timing_report, build_monthly_pillar,
};
pub use types::{
    BaziChart, BaziChartMetadata, BaziInput, BaziPillar, HiddenStemEntry, PillarKind,
    StemRelationSet,
};
