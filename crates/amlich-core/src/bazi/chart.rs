use crate::{
    almanac::{
        hour_pillar::compute_hour_pillar, na_am::get_na_am_by_pair, tang_can::get_tang_can,
        thap_than::get_thap_than, types::HeavenlyStem,
    },
    bazi::types::{
        BaziChart, BaziChartMetadata, BaziInput, BaziPillar, HiddenStemEntry, PillarKind,
    },
    canchi::{get_day_canchi, get_month_canchi, get_year_canchi},
    julian::jd_from_date,
    lunar::convert_solar_to_lunar,
};

pub fn build_bazi_chart(input: BaziInput) -> Result<BaziChart, String> {
    crate::build_count::bazi_chart_built();
    let lunar_date = convert_solar_to_lunar(input.day, input.month, input.year, input.timezone);
    let year_pillar = get_year_canchi(lunar_date.year);
    let month_pillar = get_month_canchi(lunar_date.month, lunar_date.year, lunar_date.is_leap);
    let day_pillar = get_day_canchi(jd_from_date(input.day, input.month, input.year));
    let day_master =
        HeavenlyStem::try_from(day_pillar.can.as_str()).map_err(|err| err.to_string())?;
    let hour_pillar = if input.time_known {
        Some(
            compute_hour_pillar(day_master, input.hour, input.minute)
                .ok_or_else(|| "invalid birth hour/minute for hour pillar".to_string())?,
        )
    } else {
        None
    };

    let metadata = BaziChartMetadata::new(
        &input,
        hour_pillar.as_ref().map(|hour| hour.evidence.clone()),
    );

    let year = build_pillar(PillarKind::Year, &year_pillar, day_master);
    let month = build_pillar(PillarKind::Month, &month_pillar, day_master);
    let day = build_pillar(PillarKind::Day, &day_pillar, day_master);
    let hour = hour_pillar
        .as_ref()
        .map(|hour| build_pillar(PillarKind::Hour, &hour.can_chi, day_master));
    let mut pillars = vec![year.clone(), month.clone(), day.clone()];
    if let Some(hour) = &hour {
        pillars.push(hour.clone());
    }

    Ok(BaziChart {
        input,
        lunar_date,
        year_pillar: year,
        month_pillar: month,
        day_pillar: day.clone(),
        hour_pillar: hour,
        day_master: day.can_chi,
        pillars,
        metadata,
    })
}

fn build_pillar(
    kind: PillarKind,
    can_chi: &crate::types::CanChi,
    day_master: HeavenlyStem,
) -> BaziPillar {
    let hidden_stems = hidden_stem_entries(&can_chi.chi, day_master);
    let stem_relation_to_day_master = HeavenlyStem::try_from(can_chi.can.as_str())
        .ok()
        .map(|stem| get_thap_than(day_master, stem));
    let na_am = get_na_am_by_pair(&can_chi.can, &can_chi.chi)
        .ok()
        .map(|entry| entry.na_am);

    BaziPillar {
        kind,
        can_chi: can_chi.clone(),
        hidden_stems,
        na_am,
        stem_relation_to_day_master,
    }
}

fn hidden_stem_entries(chi_name: &str, day_master: HeavenlyStem) -> Vec<HiddenStemEntry> {
    let tang_can = get_tang_can(chi_name);
    let stems = [
        (tang_can.main, tang_can.strength[0]),
        (tang_can.central, tang_can.strength[1]),
        (tang_can.residual, tang_can.strength[2]),
    ];

    stems
        .into_iter()
        .filter(|(symbol, strength)| !symbol.is_empty() && *strength > 0)
        .map(|(symbol, strength)| HiddenStemEntry {
            stem_name: map_hidden_stem_symbol(&symbol).map(str::to_string),
            ten_god_to_day_master: map_hidden_stem_symbol(&symbol)
                .and_then(|name| HeavenlyStem::try_from(name).ok())
                .map(|stem| get_thap_than(day_master, stem)),
            stem_symbol: symbol,
            strength,
        })
        .collect()
}

fn map_hidden_stem_symbol(symbol: &str) -> Option<&'static str> {
    match symbol {
        "甲" => Some("Giáp"),
        "乙" => Some("Ất"),
        "丙" => Some("Bính"),
        "丁" => Some("Đinh"),
        "戊" => Some("Mậu"),
        "己" => Some("Kỷ"),
        "庚" => Some("Canh"),
        "辛" => Some("Tân"),
        "壬" => Some("Nhâm"),
        "癸" => Some("Quý"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VIETNAM_TIMEZONE;

    #[test]
    fn builds_bazi_chart_with_four_pillars() {
        let chart = build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 9,
            minute: 30,
            time_known: true,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
        })
        .expect("chart");

        assert_eq!(chart.pillars.len(), 4);
        assert_eq!(chart.day_master.full, chart.day_pillar.can_chi.full);
    }

    #[test]
    fn builds_bazi_chart_without_hour_pillar_for_date_only_input() {
        let chart = build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 0,
            minute: 0,
            time_known: false,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
        })
        .expect("chart");

        assert_eq!(chart.pillars.len(), 3);
        assert!(chart.hour_pillar.is_none());
        assert!(chart.metadata.hour_evidence.is_none());
    }

    /// Regression for amlich-mwbp.1: a real midnight birth (00:00) must
    /// produce an hour pillar, distinct from the unknown-time path that
    /// suppresses it. Previously the `hour == 0 && minute == 0` sentinel
    /// silently demoted real midnight births to date-only.
    #[test]
    fn builds_bazi_chart_with_hour_pillar_for_real_midnight_birth() {
        let chart = build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 0,
            minute: 0,
            time_known: true,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
        })
        .expect("chart");

        assert_eq!(chart.pillars.len(), 4);
        assert!(chart.hour_pillar.is_some());
        assert!(chart.metadata.hour_evidence.is_some());
    }

    /// Regression for amlich-mwbp.1: midnight-one (00:01) must survive
    /// end-to-end as a distinct input from real midnight (00:00). Both fall
    /// in the Tý slot (23:00-01:00) and produce the same hour-pillar Can
    /// Chi, but the input minute must survive so downstream BirthProfile
    /// serialization distinguishes them.
    #[test]
    fn builds_bazi_chart_preserves_midnight_minute_distinction() {
        let midnight = build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 0,
            minute: 0,
            time_known: true,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
        })
        .expect("midnight chart");

        let midnight_one = build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 0,
            minute: 1,
            time_known: true,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
        })
        .expect("midnight-one chart");

        // Both produce an hour pillar (Tý slot covers 23:00-01:00).
        assert!(midnight.hour_pillar.is_some());
        assert!(midnight_one.hour_pillar.is_some());

        // The capability projections distinguish the two profiles via
        // BirthProfile.time equality even though the chart hour pillar
        // Can Chi is identical.
        let cap_midnight = crate::birth::BirthProfile::from_bazi_input(&midnight.input);
        let cap_midnight_one = crate::birth::BirthProfile::from_bazi_input(&midnight_one.input);
        assert_ne!(cap_midnight.time, cap_midnight_one.time);
    }

    #[test]
    fn bazi_chart_populates_hidden_stems_and_ten_gods() {
        let chart = build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 9,
            minute: 30,
            time_known: true,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
        })
        .expect("chart");

        assert!(!chart.month_pillar.hidden_stems.is_empty());
        assert!(chart
            .month_pillar
            .hidden_stems
            .iter()
            .all(|entry| entry.ten_god_to_day_master.is_some()));
    }

    #[test]
    fn bazi_chart_emits_hour_pillar_and_metadata() {
        let chart = build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 23,
            minute: 0,
            time_known: true,
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
        })
        .expect("chart");

        assert_eq!(
            chart.hour_pillar.as_ref().map(|pillar| pillar.kind),
            Some(PillarKind::Hour)
        );
        assert_eq!(chart.metadata.hour_basis, "day_stem_seed_table");
        assert!(chart.metadata.hour_evidence.is_some());
    }
}
