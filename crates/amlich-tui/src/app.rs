use amlich_api::{get_day_insight_for_date, DayInfoDto, DayInsightDto, HolidayDto};
use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsightLang {
    Vi,
    En,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl HistoryEntry {
    pub fn from_date(date: NaiveDate) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }

    pub fn to_date(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(self.year, self.month, self.day)
    }
}

pub struct App {
    pub running: bool,
    pub view_year: i32,
    pub view_month: u32,
    pub selected_day: u32,
    pub today: NaiveDate,

    // Cached data for current view
    pub month_days: Vec<DayInfoDto>,
    pub month_holidays: Vec<HolidayDto>,
    pub days_in_month: u32,
    pub first_weekday: u32, // 0=Mon, 6=Sun

    // Overlay
    pub show_holidays: bool,
    pub holiday_scroll: u16,

    // Insight panel
    pub show_insight: bool,
    pub insight_lang: InsightLang,

    // Navigation history
    pub history: Vec<HistoryEntry>,
    pub history_position: usize,
    pub history_navigation_in_progress: bool,

    // Bookmarks
    pub bookmarks: Vec<HistoryEntry>,
    pub show_bookmarks: bool,
    pub bookmark_scroll: u16,

    // Date jump input
    pub show_date_jump: bool,
    pub date_jump_input: String,

    // Search
    pub show_search: bool,
    pub search_query: String,
    pub search_results: Vec<HistoryEntry>,
    pub search_index: usize,

    // Help overlay
    pub show_help: bool,
}

impl App {
    pub fn new() -> Self {
        let today = Local::now().date_naive();
        let initial_entry = HistoryEntry::from_date(today);
        let mut app = App {
            running: true,
            view_year: today.year(),
            view_month: today.month(),
            selected_day: today.day(),
            today,
            month_days: Vec::new(),
            month_holidays: Vec::new(),
            days_in_month: 0,
            first_weekday: 0,
            show_holidays: false,
            holiday_scroll: 0,
            show_insight: false,
            insight_lang: InsightLang::Vi,
            history: vec![initial_entry],
            history_position: 0,
            history_navigation_in_progress: false,
            bookmarks: Vec::new(),
            show_bookmarks: false,
            bookmark_scroll: 0,
            show_date_jump: false,
            date_jump_input: String::new(),
            show_search: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: 0,
            show_help: false,
        };
        app.load_month();
        app.load_bookmarks();
        app
    }

    pub fn load_month(&mut self) {
        let year = self.view_year;
        let month = self.view_month as i32;

        // Calculate days in month
        let next = if self.view_month == 12 {
            NaiveDate::from_ymd_opt(year + 1, 1, 1)
        } else {
            NaiveDate::from_ymd_opt(year, self.view_month + 1, 1)
        };
        let current = NaiveDate::from_ymd_opt(year, self.view_month, 1);
        self.days_in_month = match (current, next) {
            (Some(c), Some(n)) => (n - c).num_days() as u32,
            _ => 30,
        };

        // First weekday (0=Mon, 6=Sun)
        if let Some(first) = current {
            self.first_weekday = first.weekday().num_days_from_monday();
        }

        // Clamp selected day
        if self.selected_day > self.days_in_month {
            self.selected_day = self.days_in_month;
        }

        // Load day info for each day
        self.month_days.clear();
        for day in 1..=self.days_in_month {
            if let Ok(info) = amlich_api::get_day_info_for_date(day as i32, month, year) {
                self.month_days.push(info);
            }
        }

        // Load holidays
        self.month_holidays = amlich_api::get_holidays(year, false)
            .into_iter()
            .filter(|h| h.solar_month == month)
            .collect();
    }

    pub fn selected_info(&self) -> Option<&DayInfoDto> {
        self.month_days.get((self.selected_day - 1) as usize)
    }

    pub fn holiday_for_day(&self, day: u32) -> Option<&HolidayDto> {
        self.month_holidays
            .iter()
            .find(|h| h.solar_day == day as i32)
    }

    pub fn is_today(&self, day: u32) -> bool {
        self.view_year == self.today.year()
            && self.view_month == self.today.month()
            && day == self.today.day()
    }

    // Navigation
    pub fn next_day(&mut self) {
        if self.selected_day < self.days_in_month {
            self.selected_day += 1;
            self.push_to_history();
        }
    }

    pub fn prev_day(&mut self) {
        if self.selected_day > 1 {
            self.selected_day -= 1;
            self.push_to_history();
        }
    }

    pub fn next_week(&mut self) {
        let new = self.selected_day + 7;
        if new <= self.days_in_month {
            self.selected_day = new;
            self.push_to_history();
        }
    }

    pub fn prev_week(&mut self) {
        if self.selected_day > 7 {
            self.selected_day -= 7;
            self.push_to_history();
        }
    }

    pub fn next_month(&mut self) {
        if self.view_month == 12 {
            self.view_month = 1;
            self.view_year += 1;
        } else {
            self.view_month += 1;
        }
        self.load_month();
        self.push_to_history();
    }

    pub fn prev_month(&mut self) {
        if self.view_month == 1 {
            self.view_month = 12;
            self.view_year -= 1;
        } else {
            self.view_month -= 1;
        }
        self.load_month();
        self.push_to_history();
    }

    pub fn next_year(&mut self) {
        self.view_year += 1;
        self.load_month();
        self.push_to_history();
    }

    pub fn prev_year(&mut self) {
        self.view_year -= 1;
        self.load_month();
        self.push_to_history();
    }

    pub fn go_today(&mut self) {
        self.today = Local::now().date_naive();
        let today_entry = HistoryEntry::from_date(self.today);
        self.navigate_to_entry(today_entry);
    }

    pub fn toggle_holidays(&mut self) {
        self.show_holidays = !self.show_holidays;
        self.holiday_scroll = 0;
    }

    pub fn toggle_insight(&mut self) {
        self.show_insight = !self.show_insight;
    }

    pub fn toggle_insight_lang(&mut self) {
        self.insight_lang = match self.insight_lang {
            InsightLang::Vi => InsightLang::En,
            InsightLang::En => InsightLang::Vi,
        };
    }

    pub fn selected_insight(&self) -> Option<DayInsightDto> {
        get_day_insight_for_date(
            self.selected_day as i32,
            self.view_month as i32,
            self.view_year,
        )
        .ok()
    }

    // History navigation
    fn push_to_history(&mut self) {
        let current = self.current_entry();

        // Don't push if we're just going back to the previous entry
        if self.history_position > 0 && self.history[self.history_position - 1] == current {
            self.history_position -= 1;
            return;
        }

        // Remove any forward history when navigating from a non-tip position
        if self.history_position < self.history.len() - 1 {
            self.history.truncate(self.history_position + 1);
        }

        // Only push if different from current tip
        if self.history.last() != Some(&current) {
            self.history.push(current);
            self.history_position = self.history.len() - 1;

            // Limit history size
            if self.history.len() > 100 {
                self.history.remove(0);
                self.history_position = self.history_position.saturating_sub(1);
            }
        }
    }

    pub fn history_back(&mut self) -> bool {
        if self.history_position > 0 {
            self.history_navigation_in_progress = true;
            self.history_position -= 1;
            if let Some(entry) = self.history.get(self.history_position) {
                self.apply_history_entry(*entry);
                return true;
            }
        }
        false
    }

    pub fn history_forward(&mut self) -> bool {
        if self.history_position + 1 < self.history.len() {
            self.history_navigation_in_progress = true;
            self.history_position += 1;
            if let Some(entry) = self.history.get(self.history_position) {
                self.apply_history_entry(*entry);
                return true;
            }
        }
        false
    }

    pub fn can_go_back(&self) -> bool {
        self.history_position > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.history_position + 1 < self.history.len()
    }

    // Bookmarks
    pub fn current_entry(&self) -> HistoryEntry {
        HistoryEntry {
            year: self.view_year,
            month: self.view_month,
            day: self.selected_day,
        }
    }

    fn apply_history_entry(&mut self, entry: HistoryEntry) {
        self.view_year = entry.year;
        self.view_month = entry.month;
        self.selected_day = entry.day;
        self.load_month();
    }

    fn navigate_to_entry(&mut self, entry: HistoryEntry) {
        self.apply_history_entry(entry);
        self.push_to_history();
    }

    pub fn is_bookmarked(&self) -> bool {
        let current = self.current_entry();
        self.bookmarks.contains(&current)
    }

    pub fn toggle_bookmark(&mut self) {
        let current = self.current_entry();
        if let Some(pos) = self.bookmarks.iter().position(|&b| b == current) {
            self.bookmarks.remove(pos);
        } else {
            self.bookmarks.push(current);
            self.bookmarks.sort_by_key(|b| (b.year, b.month, b.day));
        }
        self.save_bookmarks();
    }

    pub fn toggle_bookmarks(&mut self) {
        self.show_bookmarks = !self.show_bookmarks;
        self.bookmark_scroll = 0;
    }

    pub fn go_to_bookmark(&mut self, index: usize) -> bool {
        if let Some(entry) = self.bookmarks.get(index) {
            self.navigate_to_entry(*entry);
            self.show_bookmarks = false;
            true
        } else {
            false
        }
    }

    // Date jump input
    pub fn toggle_date_jump(&mut self) {
        self.show_date_jump = !self.show_date_jump;
        if self.show_date_jump {
            // Pre-fill with current date in dd/mm/yyyy format
            self.date_jump_input = format!(
                "{:02}/{:02}/{}",
                self.selected_day, self.view_month, self.view_year
            );
        } else {
            self.date_jump_input.clear();
        }
    }

    pub fn date_jump_char(&mut self, c: char) {
        // Only allow digits and slashes
        if c.is_ascii_digit() || c == '/' {
            // Don't let input get too long (max dd/mm/yyyy = 10 chars)
            if self.date_jump_input.len() < 10 {
                self.date_jump_input.push(c);
            }
        }
    }

    pub fn date_jump_backspace(&mut self) {
        self.date_jump_input.pop();
    }

    pub fn date_jump_submit(&mut self) -> bool {
        // Parse dd/mm/yyyy format
        let parts: Vec<&str> = self.date_jump_input.split('/').collect();
        if parts.len() == 3 {
            if let (Ok(day), Ok(month), Ok(year)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<i32>(),
            ) {
                if NaiveDate::from_ymd_opt(year, month, day).is_some() {
                    self.navigate_to_entry(HistoryEntry { year, month, day });
                    self.show_date_jump = false;
                    self.date_jump_input.clear();
                    return true;
                }
            }
        }
        false
    }

    // Search
    pub fn toggle_search(&mut self) {
        self.show_search = !self.show_search;
        if self.show_search {
            self.search_query.clear();
            self.search_results.clear();
            self.search_index = 0;
        }
    }

    pub fn search_char(&mut self, c: char) {
        self.search_query.push(c);
        self.perform_search();
    }

    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        self.perform_search();
    }

    fn perform_search(&mut self) {
        self.search_results.clear();
        self.search_index = 0;

        if self.search_query.is_empty() {
            return;
        }

        let query = self.search_query.to_lowercase();

        // Search through holidays for the current year and adjacent years
        for year in (self.view_year - 1)..=(self.view_year + 1) {
            let holidays = amlich_api::get_holidays(year, false);
            for holiday in holidays {
                let name_matches = holiday.name.to_lowercase().contains(&query)
                    || holiday.description.to_lowercase().contains(&query);

                if name_matches {
                    self.search_results.push(HistoryEntry {
                        year,
                        month: holiday.solar_month as u32,
                        day: holiday.solar_day as u32,
                    });
                }
            }
        }

        // Also search through day insights for special terms
        let search_terms = [
            "tết", "tet", "người đời", "nguoi doi", "giỗ tổ", "gio to",
            "trung thu", "trungthu", "quốc khánh", "quoc khanh", "quooc khanh",
            "lễ tình nhân", "le tinh nhan", "valentine",
        ];

        for term in search_terms {
            if query.contains(term) || term.contains(&query) {
                // Add dates that might match (simplified approach)
                // Tết Nguyên Đán
                if query.contains("tết") || query.contains("tet") {
                    // Add approximate Tết dates for common years
                    for year in (self.view_year - 1)..=(self.view_year + 1) {
                        // This is a simplified approach - real Tết calculation is complex
                        // For now, just check late Jan/early Feb
                        for day in 20..=31 {
                            self.search_results.push(HistoryEntry { year, month: 1, day });
                        }
                        for day in 1..=19 {
                            self.search_results.push(HistoryEntry { year, month: 2, day });
                        }
                    }
                }
            }
        }

        // Deduplicate and sort
        self.search_results.sort_by_key(|e| (e.year, e.month, e.day));
        self.search_results.dedup();
    }

    pub fn search_next_result(&mut self) -> bool {
        if self.search_results.is_empty() {
            return false;
        }

        if self.search_index + 1 < self.search_results.len() {
            self.search_index += 1;
        } else {
            self.search_index = 0; // Wrap around
        }

        if let Some(entry) = self.search_results.get(self.search_index) {
            self.navigate_to_entry(*entry);
            true
        } else {
            false
        }
    }

    pub fn search_prev_result(&mut self) -> bool {
        if self.search_results.is_empty() {
            return false;
        }

        if self.search_index > 0 {
            self.search_index -= 1;
        } else {
            self.search_index = self.search_results.len().saturating_sub(1); // Wrap around
        }

        if let Some(entry) = self.search_results.get(self.search_index) {
            self.navigate_to_entry(*entry);
            true
        } else {
            false
        }
    }

    pub fn is_search_result(&self, day: u32) -> bool {
        self.search_results.iter().any(|e| {
            e.year == self.view_year && e.month == self.view_month && e.day == day
        })
    }

    // Help
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    // Bookmark persistence
    fn bookmarks_path() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|dir| dir.join("amlich").join("bookmarks.json"))
    }

    pub fn load_bookmarks(&mut self) {
        if let Some(path) = Self::bookmarks_path() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(loaded) = serde_json::from_str::<Vec<HistoryEntry>>(&content) {
                    self.bookmarks = loaded;
                }
            }
        }
    }

    pub fn save_bookmarks(&self) {
        if let Some(path) = Self::bookmarks_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(&self.bookmarks) {
                let _ = std::fs::write(&path, json);
            }
        }
    }
}
