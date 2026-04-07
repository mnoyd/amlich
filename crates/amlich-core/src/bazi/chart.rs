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
    let lunar_date = convert_solar_to_lunar(input.day, input.month, input.year, input.timezone);
    let year_pillar = get_year_canchi(lunar_date.year);
    let month_pillar = get_month_canchi(lunar_date.month, lunar_date.year, lunar_date.is_leap);
    let day_pillar = get_day_canchi(jd_from_date(input.day, input.month, input.year));
    let day_master =
        HeavenlyStem::try_from(day_pillar.can.as_str()).map_err(|err| err.to_string())?;
    let hour_pillar = compute_hour_pillar(day_master, input.hour, input.minute)
        .ok_or_else(|| "invalid birth hour/minute for hour pillar".to_string())?;

    let metadata = BaziChartMetadata::new(&input, &hour_pillar);

    let year = build_pillar(PillarKind::Year, &year_pillar, day_master);
    let month = build_pillar(PillarKind::Month, &month_pillar, day_master);
    let day = build_pillar(PillarKind::Day, &day_pillar, day_master);
    let hour = build_pillar(PillarKind::Hour, &hour_pillar.can_chi, day_master);
    let pillars = vec![year.clone(), month.clone(), day.clone(), hour.clone()];

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
    fn bazi_chart_populates_hidden_stems_and_ten_gods() {
        let chart = build_bazi_chart(BaziInput {
            day: 10,
            month: 2,
            year: 2024,
            hour: 9,
            minute: 30,
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
            timezone: VIETNAM_TIMEZONE,
            longitude: None,
            use_solar_time: false,
            gender: None,
        })
        .expect("chart");

        assert_eq!(chart.hour_pillar.kind, PillarKind::Hour);
        assert_eq!(chart.metadata.hour_basis, "day_stem_seed_table");
        assert!(chart.metadata.hour_evidence.is_some());
    }
}
