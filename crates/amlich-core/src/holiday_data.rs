use serde::Deserialize;
use std::sync::OnceLock;

const SOLAR_HOLIDAYS_JSON: &str = include_str!("../../../data/holidays/solar-holidays.json");
const LUNAR_FESTIVALS_JSON: &str = include_str!("../../../data/holidays/lunar-festivals.json");

#[derive(Debug, Deserialize)]
struct SolarHolidaysFile {
    holidays: Vec<SolarHolidayData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolarHolidayData {
    pub solar_day: i32,
    pub solar_month: i32,
    pub category: String,
    pub is_major: bool,
    pub names: Names,
}

#[derive(Debug, Deserialize)]
struct LunarFestivalsFile {
    festivals: Vec<LunarFestivalData>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LunarFestivalData {
    pub lunar_day: i32,
    pub lunar_month: i32,
    pub year_offset: i32,
    pub category: String,
    pub is_major: bool,
    pub names: Names,
    #[serde(default)]
    pub is_solar: bool,
}

#[derive(Debug, Deserialize)]
pub struct Names {
    pub vi: Vec<String>,
    pub en: Vec<String>,
}

static SOLAR_HOLIDAYS: OnceLock<Vec<SolarHolidayData>> = OnceLock::new();
static LUNAR_FESTIVALS: OnceLock<Vec<LunarFestivalData>> = OnceLock::new();

pub fn solar_holidays() -> &'static [SolarHolidayData] {
    SOLAR_HOLIDAYS
        .get_or_init(|| {
            let parsed: SolarHolidaysFile = serde_json::from_str(SOLAR_HOLIDAYS_JSON)
                .expect("Failed to parse data/holidays/solar-holidays.json");
            parsed.holidays
        })
        .as_slice()
}

pub fn lunar_festivals() -> &'static [LunarFestivalData] {
    LUNAR_FESTIVALS
        .get_or_init(|| {
            let parsed: LunarFestivalsFile = serde_json::from_str(LUNAR_FESTIVALS_JSON)
                .expect("Failed to parse data/holidays/lunar-festivals.json");
            parsed.festivals
        })
        .as_slice()
}
