use amlich_api::{get_day_insight_for_date, DayInfoDto, DayInsightDto, HolidayDto};
use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::{
    bookmark_store, date_jump,
    history::HistoryEntry,
    search::{self, SearchResult},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum InsightLang {
    Vi,
    En,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum InsightTab {
    Festival,
    #[default]
    Guidance,
    TietKhi,
}

#[allow(dead_code)]
impl InsightTab {
    pub fn next(self) -> Self {
        match self {
            InsightTab::Festival => InsightTab::Guidance,
            InsightTab::Guidance => InsightTab::TietKhi,
            InsightTab::TietKhi => InsightTab::Festival,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            InsightTab::Festival => InsightTab::TietKhi,
            InsightTab::Guidance => InsightTab::Festival,
            InsightTab::TietKhi => InsightTab::Guidance,
        }
    }

    pub fn name(self, lang: InsightLang) -> &'static str {
        match (self, lang) {
            (InsightTab::Festival, InsightLang::Vi) => "Lễ hội",
            (InsightTab::Festival, InsightLang::En) => "Festival",
            (InsightTab::Guidance, InsightLang::Vi) => "Hướng dẫn",
            (InsightTab::Guidance, InsightLang::En) => "Guidance",
            (InsightTab::TietKhi, InsightLang::Vi) => "Tiết khí",
            (InsightTab::TietKhi, InsightLang::En) => "Season",
        }
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
    pub insight_tab: InsightTab,
    pub insight_scroll: u16,

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
    pub search_results: Vec<SearchResult>,
    pub search_index: usize,

    // Help overlay
    pub show_help: bool,

    // Calendar toggle (small screens)
    pub show_calendar: bool,
}

impl App {
    pub fn new_with_date(initial_date: Option<NaiveDate>) -> Self {
        let today = Local::now().date_naive();
        let selected = initial_date.unwrap_or(today);
        let mut app = App {
            running: true,
            view_year: selected.year(),
            view_month: selected.month(),
            selected_day: selected.day(),
            today,
            month_days: Vec::new(),
            month_holidays: Vec::new(),
            days_in_month: 0,
            first_weekday: 0,
            show_holidays: false,
            holiday_scroll: 0,
            show_insight: false,
            insight_lang: InsightLang::Vi,
            insight_tab: InsightTab::default(),
            insight_scroll: 0,
            bookmarks: bookmark_store::load_bookmarks(),
            show_bookmarks: false,
            bookmark_scroll: 0,
            show_date_jump: false,
            date_jump_input: String::new(),
            show_search: false,
            search_query: String::new(),
            search_results: Vec::new(),
            search_index: 0,
            show_help: false,
            show_calendar: false,
        };
        app.load_month();
        app
    }

    pub fn load_month(&mut self) {
        if !(1..=12).contains(&self.view_month) {
            self.view_month = self.today.month();
        }

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
        self.selected_day = self.selected_day.clamp(1, self.days_in_month.max(1));

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
        let idx = self.selected_day.checked_sub(1)? as usize;
        self.month_days.get(idx)
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
        }
    }

    pub fn prev_day(&mut self) {
        if self.selected_day > 1 {
            self.selected_day -= 1;
        }
    }

    pub fn next_week(&mut self) {
        let new = self.selected_day + 7;
        if new <= self.days_in_month {
            self.selected_day = new;
        }
    }

    pub fn prev_week(&mut self) {
        if self.selected_day > 7 {
            self.selected_day -= 7;
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
    }

    pub fn prev_month(&mut self) {
        if self.view_month == 1 {
            self.view_month = 12;
            self.view_year -= 1;
        } else {
            self.view_month -= 1;
        }
        self.load_month();
    }

    pub fn next_year(&mut self) {
        self.view_year += 1;
        self.load_month();
    }

    pub fn prev_year(&mut self) {
        self.view_year -= 1;
        self.load_month();
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
        self.insight_scroll = 0;
    }

    pub fn toggle_insight_lang(&mut self) {
        self.insight_lang = match self.insight_lang {
            InsightLang::Vi => InsightLang::En,
            InsightLang::En => InsightLang::Vi,
        };
    }

    pub fn set_insight_tab(&mut self, tab: InsightTab) {
        if self.insight_tab != tab {
            self.insight_tab = tab;
            self.insight_scroll = 0;
        }
    }

    pub fn next_insight_tab(&mut self) {
        self.insight_tab = self.insight_tab.next();
        self.insight_scroll = 0;
    }

    #[allow(dead_code)]
    pub fn prev_insight_tab(&mut self) {
        self.insight_tab = self.insight_tab.prev();
        self.insight_scroll = 0;
    }

    pub fn selected_insight(&self) -> Option<DayInsightDto> {
        get_day_insight_for_date(
            self.selected_day as i32,
            self.view_month as i32,
            self.view_year,
        )
        .ok()
    }

    // Bookmarks
    pub fn current_entry(&self) -> HistoryEntry {
        HistoryEntry {
            year: self.view_year,
            month: self.view_month,
            day: self.selected_day,
        }
    }

    fn navigate_to_entry(&mut self, entry: HistoryEntry) -> bool {
        if !entry.is_valid() {
            return false;
        }
        self.view_year = entry.year;
        self.view_month = entry.month;
        self.selected_day = entry.day;
        self.load_month();
        true
    }

    #[allow(dead_code)]
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
            self.bookmarks.sort();
            self.bookmarks.dedup();
        }
        if let Err(err) = bookmark_store::save_bookmarks(&self.bookmarks) {
            eprintln!("failed to save bookmarks: {err}");
        }
    }

    pub fn toggle_bookmarks(&mut self) {
        self.show_bookmarks = !self.show_bookmarks;
        self.bookmark_scroll = 0;
    }

    pub fn go_to_bookmark(&mut self, index: usize) -> bool {
        if let Some(entry) = self.bookmarks.get(index) {
            if self.navigate_to_entry(*entry) {
                self.show_bookmarks = false;
                return true;
            }
            false
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
        self.date_jump_input = date_jump::append_digit(&self.date_jump_input, c);
    }

    pub fn date_jump_backspace(&mut self) {
        self.date_jump_input = date_jump::backspace(&self.date_jump_input);
    }

    pub fn date_jump_is_valid(&self) -> bool {
        date_jump::is_valid(&self.date_jump_input)
    }

    pub fn date_jump_submit(&mut self) -> bool {
        let Some(entry) = date_jump::parse_date_input(&self.date_jump_input) else {
            return false;
        };
        if !self.navigate_to_entry(entry) {
            return false;
        }

        self.show_date_jump = false;
        self.date_jump_input.clear();
        true
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
        self.search_results = search::search_entries(self.view_year, &self.search_query);
        self.search_index = 0;
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

        if let Some(result) = self.search_results.get(self.search_index) {
            self.navigate_to_entry(result.entry)
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

        if let Some(result) = self.search_results.get(self.search_index) {
            self.navigate_to_entry(result.entry)
        } else {
            false
        }
    }

    pub fn is_search_result(&self, day: u32) -> bool {
        self.search_results.iter().any(|r| {
            r.entry.year == self.view_year && r.entry.month == self.view_month && r.entry.day == day
        })
    }

    // Help
    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    // Calendar toggle (small screens)
    pub fn toggle_calendar(&mut self) {
        self.show_calendar = !self.show_calendar;
    }
}
