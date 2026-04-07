use crate::{
    almanac::{
        dai_van::{calculate_dai_van_with_timezone, get_pillar_at_age},
        thap_than::get_thap_than,
        tu_menh::Gender,
        types::{HeavenlyStem, ThapThanResult},
    },
    bazi::{analysis::detect_chart_interactions, types::BaziChart},
    canchi::{get_month_canchi, get_year_canchi},
};

#[derive(Debug, Clone, PartialEq)]
pub struct BaziLuckPillar {
    pub index: usize,
    pub can_chi: String,
    pub start_age: f64,
    pub end_age: f64,
    pub ten_god_to_day_master: Option<ThapThanResult>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnnualPillar {
    pub year: i32,
    pub can_chi: String,
    pub branch: String,
    pub ten_god_to_day_master: Option<ThapThanResult>,
    pub interactions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonthlyPillar {
    pub year: i32,
    pub month: i32,
    pub can_chi: String,
    pub branch: String,
    pub ten_god_to_day_master: Option<ThapThanResult>,
    pub interactions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaziTimingReport {
    pub dai_van: Vec<BaziLuckPillar>,
    pub active_dai_van: Option<BaziLuckPillar>,
    pub annual: AnnualPillar,
    pub monthly: Vec<MonthlyPillar>,
}

pub fn build_bazi_timing_report(
    chart: &BaziChart,
    gender: Gender,
    current_age: f64,
    target_year: i32,
    months: &[i32],
) -> Result<BaziTimingReport, String> {
    let day_master =
        HeavenlyStem::try_from(chart.day_master.can.as_str()).map_err(|err| err.to_string())?;
    let dai_van_result = calculate_dai_van_with_timezone(
        chart.input.day,
        chart.input.month,
        chart.input.year,
        gender,
        chart.input.timezone,
    );

    let dai_van = dai_van_result
        .pillars
        .iter()
        .map(|pillar| BaziLuckPillar {
            index: pillar.index,
            can_chi: pillar.can_chi.full.clone(),
            start_age: pillar.start_age,
            end_age: pillar.end_age,
            ten_god_to_day_master: HeavenlyStem::try_from(pillar.can_chi.can.as_str())
                .ok()
                .map(|stem| get_thap_than(day_master, stem)),
        })
        .collect::<Vec<_>>();

    let active_dai_van =
        get_pillar_at_age(&dai_van_result, current_age).map(|pillar| BaziLuckPillar {
            index: pillar.index,
            can_chi: pillar.can_chi.full.clone(),
            start_age: pillar.start_age,
            end_age: pillar.end_age,
            ten_god_to_day_master: HeavenlyStem::try_from(pillar.can_chi.can.as_str())
                .ok()
                .map(|stem| get_thap_than(day_master, stem)),
        });

    let annual = build_annual_pillar(chart, target_year, day_master)?;
    let monthly = months
        .iter()
        .copied()
        .map(|month| build_monthly_pillar(chart, target_year, month, day_master))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BaziTimingReport {
        dai_van,
        active_dai_van,
        annual,
        monthly,
    })
}

pub fn build_annual_pillar(
    chart: &BaziChart,
    target_year: i32,
    day_master: HeavenlyStem,
) -> Result<AnnualPillar, String> {
    let can_chi = get_year_canchi(target_year);
    let ten_god_to_day_master = HeavenlyStem::try_from(can_chi.can.as_str())
        .ok()
        .map(|stem| get_thap_than(day_master, stem));

    Ok(AnnualPillar {
        year: target_year,
        can_chi: can_chi.full.clone(),
        branch: can_chi.chi.clone(),
        ten_god_to_day_master,
        interactions: branch_interactions_with_chart(chart, &can_chi.chi),
    })
}

pub fn build_monthly_pillar(
    chart: &BaziChart,
    target_year: i32,
    target_month: i32,
    day_master: HeavenlyStem,
) -> Result<MonthlyPillar, String> {
    if !(1..=12).contains(&target_month) {
        return Err("target month must be in 1..=12".to_string());
    }

    let can_chi = get_month_canchi(target_month, target_year, false);
    let ten_god_to_day_master = HeavenlyStem::try_from(can_chi.can.as_str())
        .ok()
        .map(|stem| get_thap_than(day_master, stem));

    Ok(MonthlyPillar {
        year: target_year,
        month: target_month,
        can_chi: can_chi.full.clone(),
        branch: can_chi.chi.clone(),
        ten_god_to_day_master,
        interactions: branch_interactions_with_chart(chart, &can_chi.chi),
    })
}

fn branch_interactions_with_chart(chart: &BaziChart, transient_branch: &str) -> Vec<String> {
    let mut simulated = chart.clone();
    simulated.pillars.push(crate::bazi::types::BaziPillar {
        kind: crate::bazi::types::PillarKind::Year,
        can_chi: crate::types::CanChi::new(
            0,
            crate::types::CHI
                .iter()
                .position(|chi| chi == &transient_branch)
                .unwrap_or(0),
        ),
        hidden_stems: vec![],
        na_am: None,
        stem_relation_to_day_master: None,
    });

    detect_chart_interactions(&simulated)
        .into_iter()
        .filter(|interaction| {
            interaction
                .participants
                .iter()
                .any(|chi| chi == transient_branch)
        })
        .map(|interaction| interaction.summary_vi)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bazi::build_bazi_chart, types::VIETNAM_TIMEZONE, BaziInput};

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
    fn builds_bazi_timing_report_with_dai_van_and_transits() {
        let chart = sample_chart();
        let report =
            build_bazi_timing_report(&chart, Gender::Male, 15.0, 2027, &[1, 2, 3]).expect("timing");

        assert!(!report.dai_van.is_empty());
        assert!(report.active_dai_van.is_some());
        assert_eq!(report.annual.year, 2027);
        assert_eq!(report.monthly.len(), 3);
    }

    #[test]
    fn annual_pillar_carries_ten_god_relation() {
        let chart = sample_chart();
        let day_master = HeavenlyStem::try_from(chart.day_master.can.as_str()).expect("day master");
        let annual = build_annual_pillar(&chart, 2027, day_master).expect("annual");

        assert!(!annual.can_chi.is_empty());
        assert!(annual.ten_god_to_day_master.is_some());
    }

    #[test]
    fn monthly_pillar_rejects_invalid_month() {
        let chart = sample_chart();
        let day_master = HeavenlyStem::try_from(chart.day_master.can.as_str()).expect("day master");

        let err = build_monthly_pillar(&chart, 2027, 13, day_master).expect_err("invalid month");
        assert_eq!(err, "target month must be in 1..=12");
    }
}
