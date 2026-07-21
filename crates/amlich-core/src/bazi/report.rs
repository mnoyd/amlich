use crate::{
    almanac::tu_menh::Gender,
    analysis_envelope::AnalysisEnvelope,
    bazi::{
        advisory::{build_bazi_advisory_from_metrics, BaziAdvisoryReport},
        analysis::{analyze_bazi_chart, BaziAnalysisReport},
        chart::build_bazi_chart,
        contracts::{
            to_bazi_advisory_response, to_bazi_analysis_response, to_bazi_chart_response,
            to_bazi_timing_response, BaziAdvisoryResponse, BaziAnalysisResponse, BaziChartResponse,
            BaziTimingResponse,
        },
        scoring::{compute_bazi_metrics, BaziComputedMetrics},
        timing::{build_bazi_timing_report, BaziTimingReport},
        types::BaziInput,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct BaziTimingInput {
    pub current_age: f64,
    pub target_year: i32,
    pub months: Vec<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaziReportOptions {
    pub include_chart_response: bool,
    pub include_analysis_response: bool,
    pub include_timing_response: bool,
    pub include_advisory_response: bool,
}

impl Default for BaziReportOptions {
    fn default() -> Self {
        Self {
            include_chart_response: true,
            include_analysis_response: true,
            include_timing_response: true,
            include_advisory_response: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaziReport {
    pub chart: crate::bazi::types::BaziChart,
    pub analysis: BaziAnalysisReport,
    pub timing: Option<BaziTimingReport>,
    pub computed_metrics: BaziComputedMetrics,
    pub advisory: BaziAdvisoryReport,
    pub chart_response: Option<BaziChartResponse>,
    pub analysis_response: Option<BaziAnalysisResponse>,
    pub timing_response: Option<BaziTimingResponse>,
    pub advisory_response: Option<BaziAdvisoryResponse>,
}

pub type BaziAnalysisEnvelope =
    AnalysisEnvelope<BaziReportFacts, BaziComputedMetrics, BaziAdvisoryReport>;

#[derive(Debug, Clone, PartialEq)]
pub struct BaziReportFacts {
    pub chart: crate::bazi::types::BaziChart,
    pub analysis: BaziAnalysisReport,
    pub timing: Option<BaziTimingReport>,
}

impl BaziReport {
    pub fn as_analysis_envelope(&self) -> BaziAnalysisEnvelope {
        AnalysisEnvelope::new(
            BaziReportFacts {
                chart: self.chart.clone(),
                analysis: self.analysis.clone(),
                timing: self.timing.clone(),
            },
            self.computed_metrics.clone(),
            self.advisory.clone(),
            self.computed_metrics.structure_metrics.confidence,
            self.advisory.warnings.clone(),
        )
    }
}

pub fn build_bazi_report(
    input: BaziInput,
    timing: Option<BaziTimingInput>,
) -> Result<BaziReport, String> {
    build_bazi_report_with_options(input, timing, BaziReportOptions::default())
}

pub fn build_bazi_report_with_options(
    input: BaziInput,
    timing: Option<BaziTimingInput>,
    options: BaziReportOptions,
) -> Result<BaziReport, String> {
    let chart = build_bazi_chart(input)?;
    let analysis = analyze_bazi_chart(&chart);
    let timing_report = match timing {
        Some(timing) => Some(build_timing_for_report(&chart, timing)?),
        None => None,
    };
    let computed_metrics = compute_bazi_metrics(&chart, timing_report.as_ref());
    let advisory = build_bazi_advisory_from_metrics(
        &chart,
        &analysis,
        &computed_metrics,
        timing_report.as_ref(),
    );

    let chart_response = options
        .include_chart_response
        .then(|| to_bazi_chart_response(&chart));
    let analysis_response = options
        .include_analysis_response
        .then(|| to_bazi_analysis_response(&analysis));
    let timing_response = options
        .include_timing_response
        .then(|| timing_report.as_ref().map(to_bazi_timing_response))
        .flatten();
    let advisory_response = options
        .include_advisory_response
        .then(|| to_bazi_advisory_response(&advisory));

    Ok(BaziReport {
        chart,
        analysis,
        timing: timing_report,
        computed_metrics,
        advisory,
        chart_response,
        analysis_response,
        timing_response,
        advisory_response,
    })
}

fn build_timing_for_report(
    chart: &crate::bazi::types::BaziChart,
    timing: BaziTimingInput,
) -> Result<BaziTimingReport, String> {
    let gender = chart
        .input
        .gender
        .ok_or_else(|| "gender is required for bazi timing/report".to_string())?;

    build_bazi_timing_report(
        chart,
        gender,
        timing.current_age,
        timing.target_year,
        &timing.months,
    )
}

#[allow(dead_code)]
fn _assert_gender_sendable(_gender: Gender) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{types::VIETNAM_TIMEZONE, BaziInput};

    fn sample_input() -> BaziInput {
        BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 9,
            minute: 30,
            time_known: true,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: Some(Gender::Male),
        }
    }

    #[test]
    fn builds_full_report_with_timing_and_responses() {
        let report = build_bazi_report(
            sample_input(),
            Some(BaziTimingInput {
                current_age: 15.0,
                target_year: 2027,
                months: vec![1, 2],
            }),
        )
        .expect("report");

        assert_eq!(report.chart.pillars.len(), 4);
        assert!(report.timing.is_some());
        assert!(report.chart_response.is_some());
        assert!(report.timing_response.is_some());
    }

    #[test]
    fn options_can_skip_response_contracts() {
        let report = build_bazi_report_with_options(
            sample_input(),
            None,
            BaziReportOptions {
                include_chart_response: false,
                include_analysis_response: false,
                include_timing_response: false,
                include_advisory_response: false,
            },
        )
        .expect("report");

        assert!(report.chart_response.is_none());
        assert!(report.analysis_response.is_none());
        assert!(report.timing_response.is_none());
        assert!(report.advisory_response.is_none());
    }

    #[test]
    fn timing_requires_gender() {
        let mut input = sample_input();
        input.gender = None;

        let err = build_bazi_report(
            input,
            Some(BaziTimingInput {
                current_age: 15.0,
                target_year: 2027,
                months: vec![1],
            }),
        )
        .expect_err("gender required");

        assert_eq!(err, "gender is required for bazi timing/report");
    }

    #[test]
    fn report_can_project_to_analysis_envelope() {
        let report = build_bazi_report(sample_input(), None).expect("report");
        let envelope = report.as_analysis_envelope();

        assert_eq!(envelope.facts.chart.pillars.len(), 4);
        assert!(!envelope.advisory.summary_vi.is_empty());
        assert!(envelope.confidence >= 0.0);
    }
}
