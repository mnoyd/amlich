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
    #[serde(default)]
    pub origin: Option<BilingualText>,
    #[serde(default)]
    pub significance: Option<BilingualText>,
    #[serde(default)]
    pub activities: Option<BilingualList>,
    #[serde(default)]
    pub traditions: Option<BilingualList>,
    #[serde(default)]
    pub food: Vec<FoodItem>,
    #[serde(default)]
    pub taboos: Vec<TabooItem>,
    #[serde(default)]
    pub proverbs: Vec<ProverbItem>,
    #[serde(default)]
    pub regions: Option<Regions>,
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
    pub origin: Option<BilingualText>,
    #[serde(default)]
    pub activities: Option<BilingualList>,
    #[serde(default)]
    pub food: Vec<FoodItem>,
    #[serde(default)]
    pub taboos: Vec<TabooItem>,
    #[serde(default)]
    pub proverbs: Vec<ProverbItem>,
    #[serde(default)]
    pub regions: Option<Regions>,
    #[serde(default)]
    pub is_solar: bool,
}

#[derive(Debug, Deserialize)]
pub struct Names {
    pub vi: Vec<String>,
    pub en: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct BilingualText {
    pub vi: String,
    pub en: String,
}

#[derive(Debug, Deserialize)]
pub struct BilingualList {
    pub vi: Vec<String>,
    pub en: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct FoodItem {
    pub name: BilingualText,
    pub description: BilingualText,
}

#[derive(Debug, Deserialize)]
pub struct TabooItem {
    pub action: BilingualText,
    pub reason: BilingualText,
}

#[derive(Debug, Deserialize)]
pub struct ProverbItem {
    pub text: String,
    pub meaning: BilingualText,
}

#[derive(Debug, Deserialize)]
pub struct Regions {
    pub north: BilingualText,
    pub central: BilingualText,
    pub south: BilingualText,
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

#[cfg(test)]
mod tests {
    use super::{lunar_festivals, solar_holidays};

    #[test]
    fn parses_lunar_festival_cultural_fields() {
        let tet = lunar_festivals()
            .iter()
            .find(|f| f.names.vi.iter().any(|name| name == "Tết Nguyên Đán"))
            .expect("Expected Tết Nguyên Đán in lunar-festivals.json");

        assert!(tet.origin.is_some(), "origin should be parsed");
        assert!(tet.activities.is_some(), "activities should be parsed");
        assert!(!tet.food.is_empty(), "food should be parsed");
        assert!(!tet.taboos.is_empty(), "taboos should be parsed");
        assert!(!tet.proverbs.is_empty(), "proverbs should be parsed");
        assert!(tet.regions.is_some(), "regions should be parsed");
    }

    #[test]
    fn parses_solar_holiday_cultural_fields() {
        let teachers_day = solar_holidays()
            .iter()
            .find(|h| {
                h.names
                    .vi
                    .iter()
                    .any(|name| name == "Ngày Nhà Giáo Việt Nam")
            })
            .expect("Expected Ngày Nhà Giáo Việt Nam in solar-holidays.json");

        assert!(teachers_day.origin.is_some(), "origin should be parsed");
        assert!(
            teachers_day.significance.is_some(),
            "significance should be parsed"
        );
        assert!(
            teachers_day.activities.is_some(),
            "activities should be parsed"
        );
        assert!(
            teachers_day.traditions.is_some(),
            "traditions should be parsed"
        );
        assert!(!teachers_day.food.is_empty(), "food should be parsed");
        assert!(!teachers_day.taboos.is_empty(), "taboos should be parsed");
        assert!(
            !teachers_day.proverbs.is_empty(),
            "proverbs should be parsed"
        );
        assert!(teachers_day.regions.is_some(), "regions should be parsed");
    }
}
