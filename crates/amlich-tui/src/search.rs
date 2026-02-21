use crate::history::HistoryEntry;
use chrono::{Local, NaiveDate};
use deunicode::deunicode;
use std::collections::HashMap;

/// Enriched search result with holiday name and lunar date
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SearchResult {
    pub entry: HistoryEntry,
    pub holiday_name: String,
    pub lunar: String,
}

impl SearchResult {
    fn new(entry: HistoryEntry, holiday_name: String, lunar: String) -> Self {
        Self {
            entry,
            holiday_name,
            lunar,
        }
    }
}

/// Normalize Vietnamese text to ASCII (không dấu) for comparison
fn normalize(text: &str) -> String {
    deunicode(text).to_lowercase()
}

/// Format lunar date as "d/m Âm"
fn format_lunar(day: i32, month: i32) -> String {
    format!("{}/{} Âm", day, month)
}

/// Calculate sort key for proximity to today
/// Returns (is_past, days_from_today) for sorting
/// Future dates sort first (is_past=false), then by closest first
fn sort_key(entry: &HistoryEntry, today: NaiveDate) -> (bool, i64) {
    let entry_date = entry.to_date();
    match entry_date {
        Some(date) => {
            let days_diff = (date - today).num_days();
            // Future: days_diff > 0, sort by days_diff ascending
            // Past: days_diff < 0, sort by days_diff descending (most recent first)
            if days_diff >= 0 {
                (false, days_diff)
            } else {
                (true, -days_diff)
            }
        }
        None => (true, i64::MAX), // Invalid dates go last
    }
}

pub fn search_entries(view_year: i32, raw_query: &str) -> Vec<SearchResult> {
    let query = normalize(raw_query);
    let query = query.trim();
    if query.is_empty() {
        return Vec::new();
    }

    let today = Local::now().date_naive();
    let mut results_map: HashMap<(i32, u32, u32), SearchResult> = HashMap::new();

    // Search through holidays for the current year and adjacent years.
    for year in (view_year - 1)..=(view_year + 1) {
        let holidays = amlich_api::get_holidays(year, false);
        for holiday in holidays {
            let name_normalized = normalize(&holiday.name);
            let desc_normalized = normalize(&holiday.description);

            let name_matches = name_normalized.contains(&query) || desc_normalized.contains(&query);

            if name_matches {
                if let Some(entry) =
                    HistoryEntry::new(year, holiday.solar_month as u32, holiday.solar_day as u32)
                {
                    // Calculate lunar date
                    let lunar = amlich_core::lunar::convert_solar_to_lunar(
                        holiday.solar_day,
                        holiday.solar_month,
                        holiday.solar_year,
                        amlich_core::VIETNAM_TIMEZONE,
                    );
                    let lunar_str = format_lunar(lunar.day, lunar.month);

                    let key = (entry.year, entry.month, entry.day);
                    results_map.entry(key).or_insert_with(|| {
                        SearchResult::new(entry, holiday.name.clone(), lunar_str)
                    });
                }
            }
        }
    }

    // Special handling for Tết search - show the entire Tết period
    let tet_query = normalize("tết");
    if query.contains(&tet_query) {
        let tet_name = "Tết Nguyên Đán".to_string();
        for year in (view_year - 1)..=(view_year + 1) {
            for day in 20..=31 {
                if let Some(entry) = HistoryEntry::new(year, 1, day) {
                    let lunar = amlich_core::lunar::convert_solar_to_lunar(
                        day as i32,
                        1,
                        year,
                        amlich_core::VIETNAM_TIMEZONE,
                    );
                    let key = (entry.year, entry.month, entry.day);
                    results_map.entry(key).or_insert_with(|| {
                        SearchResult::new(
                            entry,
                            tet_name.clone(),
                            format_lunar(lunar.day, lunar.month),
                        )
                    });
                }
            }
            for day in 1..=19 {
                if let Some(entry) = HistoryEntry::new(year, 2, day) {
                    let lunar = amlich_core::lunar::convert_solar_to_lunar(
                        day as i32,
                        2,
                        year,
                        amlich_core::VIETNAM_TIMEZONE,
                    );
                    let key = (entry.year, entry.month, entry.day);
                    results_map.entry(key).or_insert_with(|| {
                        SearchResult::new(
                            entry,
                            tet_name.clone(),
                            format_lunar(lunar.day, lunar.month),
                        )
                    });
                }
            }
        }
    }

    // Convert to vec and sort by proximity to today
    let mut results: Vec<SearchResult> = results_map.into_values().collect();
    results.sort_by_key(|r| sort_key(&r.entry, today));

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tet_search_returns_results() {
        let results = search_entries(2025, "tet");
        assert!(!results.is_empty());
        // Verify results have holiday names and lunar dates
        for r in results {
            assert!(!r.holiday_name.is_empty());
            assert!(r.lunar.contains("Âm"));
        }
    }

    #[test]
    fn tet_search_with_diacritics() {
        let results = search_entries(2025, "tết");
        assert!(!results.is_empty());
    }

    #[test]
    fn empty_query_returns_no_results() {
        assert!(search_entries(2025, "   ").is_empty());
    }

    #[test]
    fn normalize_removes_diacritics() {
        assert_eq!(normalize("Tết Nguyên Đán"), "tet nguyen dan");
        assert_eq!(normalize("Lễ Vu Lan"), "le vu lan");
        assert_eq!(normalize("Giỗ Tổ Hùng Vương"), "gio to hung vuong");
        assert_eq!(normalize("Tết Trung Thu"), "tet trung thu");
    }

    #[test]
    fn format_lunar_works() {
        assert_eq!(format_lunar(1, 1), "1/1 Âm");
        assert_eq!(format_lunar(15, 8), "15/8 Âm");
    }
}
