use crate::history::HistoryEntry;
use deunicode::deunicode;

/// Normalize Vietnamese text to ASCII (không dấu) for comparison
fn normalize(text: &str) -> String {
    deunicode(text).to_lowercase()
}

pub fn search_entries(view_year: i32, raw_query: &str) -> Vec<HistoryEntry> {
    let query = normalize(raw_query);
    let query = query.trim();
    if query.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();

    // Search through holidays for the current year and adjacent years.
    for year in (view_year - 1)..=(view_year + 1) {
        let holidays = amlich_api::get_holidays(year, false);
        for holiday in holidays {
            let name_normalized = normalize(&holiday.name);
            let desc_normalized = normalize(&holiday.description);

            let name_matches = name_normalized.contains(query) || desc_normalized.contains(query);

            if name_matches {
                if let Some(entry) =
                    HistoryEntry::new(year, holiday.solar_month as u32, holiday.solar_day as u32)
                {
                    results.push(entry);
                }
            }
        }
    }

    // Special handling for Tết search - show the entire Tết period
    // This matches both "tet" and any query containing it after normalization
    let tet_query = normalize("tết");
    if query.contains(&tet_query) {
        for year in (view_year - 1)..=(view_year + 1) {
            for day in 20..=31 {
                if let Some(entry) = HistoryEntry::new(year, 1, day) {
                    results.push(entry);
                }
            }
            for day in 1..=19 {
                if let Some(entry) = HistoryEntry::new(year, 2, day) {
                    results.push(entry);
                }
            }
        }
    }

    results.sort();
    results.dedup();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tet_search_deduplicates_and_sorts() {
        let results = search_entries(2025, "tet");
        assert!(!results.is_empty());
        assert!(results.windows(2).all(|w| w[0] <= w[1]));
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
}
