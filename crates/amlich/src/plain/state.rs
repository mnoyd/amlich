use super::screens::Screen;
use amlich_api::{
    v2::{get_day_bundle, DayBundleDto, Include},
    DateQuery,
};
use chrono::{Duration, Local, NaiveDate};

/// The renderer's small, API-owned state.  It intentionally has no widget or
/// terminal types, so the data source remains independently testable.
pub struct AppState {
    pub date: NaiveDate,
    pub bundle: DayBundleDto,
    pub screen: Screen,
    pub scroll: usize,
}

impl AppState {
    pub fn new(date: Option<NaiveDate>) -> Result<Self, String> {
        let date = date.unwrap_or_else(|| Local::now().date_naive());
        Ok(Self {
            date,
            bundle: load(date)?,
            screen: Screen::Today,
            scroll: 0,
        })
    }
    pub fn shift_day(&mut self, days: i64) -> Result<(), String> {
        let date = self
            .date
            .checked_add_signed(Duration::days(days))
            .ok_or("date outside supported range")?;
        self.bundle = load(date)?;
        self.date = date;
        self.scroll = 0;
        Ok(())
    }
}

fn load(date: NaiveDate) -> Result<DayBundleDto, String> {
    get_day_bundle(
        &DateQuery {
            day: date.day() as i32,
            month: date.month() as i32,
            year: date.year(),
            timezone: None,
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
        },
        &[
            Include::Base,
            Include::CanChi,
            Include::TietKhi,
            Include::Hours,
            Include::Fortune,
            Include::Insight,
        ],
    )
}

use chrono::Datelike;
