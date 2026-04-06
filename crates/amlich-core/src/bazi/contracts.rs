use serde::{Deserialize, Serialize};

use crate::{
    almanac::types::{FiveElement, RuleEvidence, ThapThanResult},
    bazi::{
        advisory::{BaziAdvisoryDomains, BaziAdvisoryReport, UsefulGodAnalysis},
        analysis::{
            BaziAnalysisReport, ChartInteraction, ChartInteractionKind, DayMasterStrength,
            DayMasterStrengthLabel, ElementDistribution, TenGodDistribution,
        },
        timing::{AnnualPillar, BaziLuckPillar, BaziTimingReport, MonthlyPillar},
        types::{BaziChart, BaziChartMetadata, BaziInput, HiddenStemEntry, PillarKind},
    },
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziChartResponse {
    pub input: BaziInput,
    pub lunar_date: BaziLunarDateResponse,
    pub day_master: BaziCanChiResponse,
    pub pillars: Vec<BaziPillarResponse>,
    pub metadata: BaziChartMetadataResponse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziPillarResponse {
    pub kind: PillarKind,
    pub can_chi: BaziCanChiResponse,
    #[serde(default)]
    pub hidden_stems: Vec<HiddenStemEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub na_am: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stem_relation_to_day_master: Option<ThapThanResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziCanChiResponse {
    pub can: String,
    pub chi: String,
    pub full: String,
    pub can_index: usize,
    pub chi_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziLunarDateResponse {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub is_leap: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziChartMetadataResponse {
    pub timezone: f64,
    pub use_solar_time: bool,
    pub year_basis: String,
    pub month_basis: String,
    pub day_basis: String,
    pub hour_basis: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hour_evidence: Option<RuleEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziAnalysisResponse {
    pub element_distribution: ElementDistributionResponse,
    pub day_master_strength: DayMasterStrengthResponse,
    #[serde(default)]
    pub interactions: Vec<ChartInteractionResponse>,
    pub ten_god_distribution: TenGodDistributionResponse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DayMasterStrengthResponse {
    pub score: i32,
    pub label: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChartInteractionResponse {
    pub kind: String,
    pub participants: Vec<String>,
    pub summary_vi: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementDistributionResponse {
    pub moc: u16,
    pub hoa: u16,
    pub tho: u16,
    pub kim: u16,
    pub thuy: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TenGodDistributionResponse {
    pub ty_kien: u8,
    pub kiep_tai: u8,
    pub thuc_than: u8,
    pub thuong_quan: u8,
    pub chinh_tai: u8,
    pub thien_tai: u8,
    pub chinh_quan: u8,
    pub that_sat: u8,
    pub chinh_an: u8,
    pub thien_an: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziTimingResponse {
    #[serde(default)]
    pub dai_van: Vec<BaziLuckPillarResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_dai_van: Option<BaziLuckPillarResponse>,
    pub annual: AnnualPillarResponse,
    #[serde(default)]
    pub monthly: Vec<MonthlyPillarResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaziLuckPillarResponse {
    pub index: usize,
    pub can_chi: String,
    pub start_age: f64,
    pub end_age: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ten_god_to_day_master: Option<ThapThanResult>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnualPillarResponse {
    pub year: i32,
    pub can_chi: String,
    pub branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ten_god_to_day_master: Option<ThapThanResult>,
    #[serde(default)]
    pub interactions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MonthlyPillarResponse {
    pub year: i32,
    pub month: i32,
    pub can_chi: String,
    pub branch: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ten_god_to_day_master: Option<ThapThanResult>,
    #[serde(default)]
    pub interactions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziAdvisoryResponse {
    pub useful_god_analysis: UsefulGodResponse,
    pub summary_vi: String,
    #[serde(default)]
    pub warnings: Vec<String>,
    pub domains: BaziAdvisoryDomainsResponse,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsefulGodResponse {
    #[serde(default)]
    pub favorable_elements: Vec<FiveElement>,
    #[serde(default)]
    pub unfavorable_elements: Vec<FiveElement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tentative_yong_shen: Option<FiveElement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tentative_xi_shen: Option<FiveElement>,
    pub confidence: String,
    #[serde(default)]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaziAdvisoryDomainsResponse {
    pub career: Vec<String>,
    pub wealth: Vec<String>,
    pub relationship: Vec<String>,
    pub health: Vec<String>,
    pub timing: Vec<String>,
}

pub fn to_bazi_chart_response(chart: &BaziChart) -> BaziChartResponse {
    BaziChartResponse {
        input: chart.input.clone(),
        lunar_date: BaziLunarDateResponse {
            day: chart.lunar_date.day,
            month: chart.lunar_date.month,
            year: chart.lunar_date.year,
            is_leap: chart.lunar_date.is_leap,
        },
        day_master: to_canchi_response(&chart.day_master),
        pillars: chart
            .pillars
            .iter()
            .map(|pillar| BaziPillarResponse {
                kind: pillar.kind,
                can_chi: to_canchi_response(&pillar.can_chi),
                hidden_stems: pillar.hidden_stems.clone(),
                na_am: pillar.na_am.clone(),
                stem_relation_to_day_master: pillar.stem_relation_to_day_master.clone(),
            })
            .collect(),
        metadata: to_chart_metadata_response(&chart.metadata),
    }
}

pub fn to_bazi_analysis_response(report: &BaziAnalysisReport) -> BaziAnalysisResponse {
    BaziAnalysisResponse {
        element_distribution: to_element_distribution_response(&report.element_distribution),
        day_master_strength: to_day_master_strength_response(&report.day_master_strength),
        interactions: report
            .interactions
            .iter()
            .map(to_chart_interaction_response)
            .collect(),
        ten_god_distribution: to_ten_god_distribution_response(&report.ten_god_distribution),
    }
}

pub fn to_bazi_timing_response(report: &BaziTimingReport) -> BaziTimingResponse {
    BaziTimingResponse {
        dai_van: report.dai_van.iter().map(to_luck_pillar_response).collect(),
        active_dai_van: report.active_dai_van.as_ref().map(to_luck_pillar_response),
        annual: to_annual_pillar_response(&report.annual),
        monthly: report.monthly.iter().map(to_monthly_pillar_response).collect(),
    }
}

pub fn to_bazi_advisory_response(report: &BaziAdvisoryReport) -> BaziAdvisoryResponse {
    BaziAdvisoryResponse {
        useful_god_analysis: to_useful_god_response(&report.useful_god_analysis),
        summary_vi: report.summary_vi.clone(),
        warnings: report.warnings.clone(),
        domains: to_advisory_domains_response(&report.domains),
    }
}

fn to_canchi_response(value: &crate::types::CanChi) -> BaziCanChiResponse {
    BaziCanChiResponse {
        can: value.can.clone(),
        chi: value.chi.clone(),
        full: value.full.clone(),
        can_index: value.can_index,
        chi_index: value.chi_index,
    }
}

fn to_chart_metadata_response(metadata: &BaziChartMetadata) -> BaziChartMetadataResponse {
    BaziChartMetadataResponse {
        timezone: metadata.timezone,
        use_solar_time: metadata.use_solar_time,
        year_basis: metadata.year_basis.clone(),
        month_basis: metadata.month_basis.clone(),
        day_basis: metadata.day_basis.clone(),
        hour_basis: metadata.hour_basis.clone(),
        hour_evidence: metadata.hour_evidence.clone(),
    }
}

fn to_day_master_strength_response(value: &DayMasterStrength) -> DayMasterStrengthResponse {
    DayMasterStrengthResponse {
        score: value.score,
        label: match value.label {
            DayMasterStrengthLabel::Strong => "strong",
            DayMasterStrengthLabel::Balanced => "balanced",
            DayMasterStrengthLabel::Weak => "weak",
        }
        .to_string(),
        reasons: value.reasons.clone(),
    }
}

fn to_chart_interaction_response(value: &ChartInteraction) -> ChartInteractionResponse {
    ChartInteractionResponse {
        kind: match value.kind {
            ChartInteractionKind::BranchClash => "branch_clash",
            ChartInteractionKind::BranchHarmony => "branch_harmony",
            ChartInteractionKind::BranchHarm => "branch_harm",
        }
        .to_string(),
        participants: value.participants.clone(),
        summary_vi: value.summary_vi.clone(),
    }
}

fn to_element_distribution_response(value: &ElementDistribution) -> ElementDistributionResponse {
    ElementDistributionResponse {
        moc: value.moc,
        hoa: value.hoa,
        tho: value.tho,
        kim: value.kim,
        thuy: value.thuy,
    }
}

fn to_ten_god_distribution_response(value: &TenGodDistribution) -> TenGodDistributionResponse {
    TenGodDistributionResponse {
        ty_kien: value.ty_kien,
        kiep_tai: value.kiep_tai,
        thuc_than: value.thuc_than,
        thuong_quan: value.thuong_quan,
        chinh_tai: value.chinh_tai,
        thien_tai: value.thien_tai,
        chinh_quan: value.chinh_quan,
        that_sat: value.that_sat,
        chinh_an: value.chinh_an,
        thien_an: value.thien_an,
    }
}

fn to_luck_pillar_response(value: &BaziLuckPillar) -> BaziLuckPillarResponse {
    BaziLuckPillarResponse {
        index: value.index,
        can_chi: value.can_chi.clone(),
        start_age: value.start_age,
        end_age: value.end_age,
        ten_god_to_day_master: value.ten_god_to_day_master.clone(),
    }
}

fn to_annual_pillar_response(value: &AnnualPillar) -> AnnualPillarResponse {
    AnnualPillarResponse {
        year: value.year,
        can_chi: value.can_chi.clone(),
        branch: value.branch.clone(),
        ten_god_to_day_master: value.ten_god_to_day_master.clone(),
        interactions: value.interactions.clone(),
    }
}

fn to_monthly_pillar_response(value: &MonthlyPillar) -> MonthlyPillarResponse {
    MonthlyPillarResponse {
        year: value.year,
        month: value.month,
        can_chi: value.can_chi.clone(),
        branch: value.branch.clone(),
        ten_god_to_day_master: value.ten_god_to_day_master.clone(),
        interactions: value.interactions.clone(),
    }
}

fn to_useful_god_response(value: &UsefulGodAnalysis) -> UsefulGodResponse {
    UsefulGodResponse {
        favorable_elements: value.favorable_elements.clone(),
        unfavorable_elements: value.unfavorable_elements.clone(),
        tentative_yong_shen: value.tentative_yong_shen,
        tentative_xi_shen: value.tentative_xi_shen,
        confidence: value.confidence.clone(),
        reasons: value.reasons.clone(),
    }
}

fn to_advisory_domains_response(value: &BaziAdvisoryDomains) -> BaziAdvisoryDomainsResponse {
    BaziAdvisoryDomainsResponse {
        career: value.career.clone(),
        wealth: value.wealth.clone(),
        relationship: value.relationship.clone(),
        health: value.health.clone(),
        timing: value.timing.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        almanac::tu_menh::Gender,
        bazi::{
            analyze_bazi_chart, build_bazi_advisory, build_bazi_chart, build_bazi_timing_report,
        },
        types::VIETNAM_TIMEZONE,
        BaziInput,
    };

    fn sample_chart() -> BaziChart {
        build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 9,
            minute: 30,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(Gender::Male),
        })
        .expect("chart")
    }

    #[test]
    fn chart_response_serializes_with_stable_shape() {
        let response = to_bazi_chart_response(&sample_chart());
        let json = serde_json::to_string(&response).expect("serialize");

        assert!(json.contains("\"pillars\""));
        assert!(json.contains("\"day_master\""));
        assert!(json.contains("\"metadata\""));
    }

    #[test]
    fn analysis_timing_and_advisory_responses_serialize() {
        let chart = sample_chart();
        let analysis = analyze_bazi_chart(&chart);
        let timing =
            build_bazi_timing_report(&chart, Gender::Male, 15.0, 2027, &[1, 2]).expect("timing");
        let advisory = build_bazi_advisory(&chart, Some(&timing));

        let analysis_json =
            serde_json::to_string(&to_bazi_analysis_response(&analysis)).expect("analysis json");
        let timing_json =
            serde_json::to_string(&to_bazi_timing_response(&timing)).expect("timing json");
        let advisory_json =
            serde_json::to_string(&to_bazi_advisory_response(&advisory)).expect("advisory json");

        assert!(analysis_json.contains("\"day_master_strength\""));
        assert!(timing_json.contains("\"active_dai_van\""));
        assert!(advisory_json.contains("\"useful_god_analysis\""));
    }
}
