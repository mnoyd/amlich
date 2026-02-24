use amlich_api::HolidayDto;
use chrono::{Duration, NaiveDate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerdictLevel {
    Good,
    Neutral,
    Caution,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verdict {
    pub level: VerdictLevel,
    pub label_vi: &'static str,
    pub icon: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpcomingHoliday {
    pub name: String,
    pub date: NaiveDate,
    pub days_left: i64,
    pub is_major: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewRow {
    pub date: NaiveDate,
}

pub fn derive_verdict(guidance: Option<(&[String], &[String])>) -> Verdict {
    let Some((good_for, avoid_for)) = guidance else {
        return Verdict {
            level: VerdictLevel::Unknown,
            label_vi: "Chưa đủ dữ liệu",
            icon: "?",
        };
    };

    let good = good_for.len();
    let avoid = avoid_for.len();

    if good >= avoid.saturating_add(2) {
        Verdict {
            level: VerdictLevel::Good,
            label_vi: "Tốt",
            icon: "✓",
        }
    } else if avoid >= good.saturating_add(2) {
        Verdict {
            level: VerdictLevel::Caution,
            label_vi: "Cần cân nhắc",
            icon: "!",
        }
    } else {
        Verdict {
            level: VerdictLevel::Neutral,
            label_vi: "Trung bình",
            icon: "•",
        }
    }
}

pub fn select_upcoming_holidays(
    from: NaiveDate,
    holidays: &[HolidayDto],
    limit: usize,
) -> Vec<UpcomingHoliday> {
    let mut upcoming: Vec<UpcomingHoliday> = holidays
        .iter()
        .filter_map(|holiday| {
            let date = NaiveDate::from_ymd_opt(
                holiday.solar_year,
                holiday.solar_month as u32,
                holiday.solar_day as u32,
            )?;
            let days_left = date.signed_duration_since(from).num_days();
            (days_left > 0).then_some(UpcomingHoliday {
                name: holiday.name.clone(),
                date,
                days_left,
                is_major: holiday.is_major,
            })
        })
        .collect();

    upcoming.sort_by_key(|item| (item.days_left, !item.is_major));
    upcoming.truncate(limit);
    upcoming
}

pub fn build_preview_rows(from: NaiveDate, days: usize) -> Vec<PreviewRow> {
    (0..days)
        .map(|offset| PreviewRow {
            date: from + Duration::days(offset as i64),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use amlich_api::HolidayDto;
    use chrono::{Datelike, NaiveDate};

    use super::{build_preview_rows, derive_verdict, select_upcoming_holidays};

    fn sample_holidays() -> Vec<HolidayDto> {
        vec![
            HolidayDto {
                name: "Cuối năm".to_string(),
                description: "Holiday at end of year".to_string(),
                solar_day: 31,
                solar_month: 12,
                solar_year: 2026,
                lunar_day: None,
                lunar_month: None,
                lunar_year: None,
                is_solar: true,
                category: "sample".to_string(),
                is_major: false,
            },
            HolidayDto {
                name: "Đầu năm".to_string(),
                description: "Holiday at start of year".to_string(),
                solar_day: 1,
                solar_month: 1,
                solar_year: 2027,
                lunar_day: None,
                lunar_month: None,
                lunar_year: None,
                is_solar: true,
                category: "sample".to_string(),
                is_major: true,
            },
        ]
    }

    #[test]
    fn verdict_is_good_when_good_outnumbers_avoid_by_two() {
        let good = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
        ];
        let avoid = vec!["E".to_string()];

        let verdict = derive_verdict(Some((&good, &avoid)));
        assert_eq!(verdict.label_vi, "Tốt");
    }

    #[test]
    fn verdict_is_caution_when_avoid_outnumbers_good_by_two() {
        let good = vec!["A".to_string()];
        let avoid = vec![
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ];

        let verdict = derive_verdict(Some((&good, &avoid)));
        assert_eq!(verdict.label_vi, "Cần cân nhắc");
    }

    #[test]
    fn upcoming_holidays_cross_year_boundary() {
        let from = NaiveDate::from_ymd_opt(2026, 12, 31).expect("valid date");
        let items = select_upcoming_holidays(from, &sample_holidays(), 2);

        assert!(!items.is_empty());
        assert!(items[0].days_left >= 1);
    }

    #[test]
    fn preview_rows_roll_over_month_end() {
        let from = NaiveDate::from_ymd_opt(2026, 1, 31).expect("valid date");
        let rows = build_preview_rows(from, 3);

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1].date.day(), 1);
        assert_eq!(rows[1].date.month(), 2);
    }
}
