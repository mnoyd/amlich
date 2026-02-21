use crate::history::HistoryEntry;

pub fn search_entries(view_year: i32, raw_query: &str) -> Vec<HistoryEntry> {
    let query = raw_query.trim().to_lowercase();
    if query.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();

    // Search through holidays for the current year and adjacent years.
    for year in (view_year - 1)..=(view_year + 1) {
        let holidays = amlich_api::get_holidays(year, false);
        for holiday in holidays {
            let name_matches = holiday.name.to_lowercase().contains(&query)
                || holiday.description.to_lowercase().contains(&query);

            if name_matches {
                if let Some(entry) =
                    HistoryEntry::new(year, holiday.solar_month as u32, holiday.solar_day as u32)
                {
                    results.push(entry);
                }
            }
        }
    }

    // Keyword-based fallback for culturally common terms.
    let search_terms = [
        "tết",
        "tet",
        "người đời",
        "nguoi doi",
        "giỗ tổ",
        "gio to",
        "trung thu",
        "trungthu",
        "quốc khánh",
        "quoc khanh",
        "quooc khanh",
        "lễ tình nhân",
        "le tinh nhan",
        "valentine",
    ];

    for term in search_terms {
        if (query.contains(term) || term.contains(&query))
            && (query.contains("tết") || query.contains("tet"))
        {
            // Simplified Tet window.
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
    }

    results.sort();
    results.dedup();
    results
}

#[cfg(test)]
mod tests {
    use super::search_entries;

    #[test]
    fn tet_search_deduplicates_and_sorts() {
        let results = search_entries(2025, "tet");
        assert!(!results.is_empty());
        assert!(results.windows(2).all(|w| w[0] <= w[1]));
    }

    #[test]
    fn empty_query_returns_no_results() {
        assert!(search_entries(2025, "   ").is_empty());
    }
}
