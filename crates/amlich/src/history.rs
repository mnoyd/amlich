use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub struct HistoryEntry {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl HistoryEntry {
    pub fn new(year: i32, month: u32, day: u32) -> Option<Self> {
        NaiveDate::from_ymd_opt(year, month, day).map(Self::from_date)
    }

    pub fn from_date(date: NaiveDate) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }

    pub fn to_date(self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, self.month, self.day)
    }

    pub fn is_valid(self) -> bool {
        self.to_date().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::HistoryEntry;

    #[test]
    fn validates_calendar_dates() {
        assert!(HistoryEntry::new(2024, 2, 29).is_some());
        assert!(HistoryEntry::new(2023, 2, 29).is_none());
        assert!(HistoryEntry::new(2025, 0, 1).is_none());
    }
}
