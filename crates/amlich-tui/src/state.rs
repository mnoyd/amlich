use amlich_api::v2::{get_day_bundle_for_date, DayBundleDto, Include};
use chrono::{Datelike, Local, NaiveDate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusLens {
    General,
    Planning,
    Scholarly,
    Personal,
}

impl FocusLens {
    pub fn next(&self) -> Self {
        match self {
            Self::General => Self::Planning,
            Self::Planning => Self::Scholarly,
            Self::Scholarly => Self::Personal,
            Self::Personal => Self::General,
        }
    }
}

pub struct AppState {
    pub running: bool,
    pub date: NaiveDate,
    pub lens: FocusLens,
    pub scroll_offset: u16,
    
    // Data cache for the current date
    pub bundle: Option<DayBundleDto>,
    pub is_loading: bool,
    pub error_msg: Option<String>,
    
    // UI toggles
    pub show_calendar: bool,
    pub show_tietkhi_details: bool,
    pub show_search: bool,
    pub search_input: String,
}

impl AppState {
    pub fn new(initial_date: Option<NaiveDate>) -> Self {
        let date = initial_date.unwrap_or_else(|| Local::now().naive_local().date());
        
        let mut app = Self {
            running: true,
            date,
            lens: FocusLens::General,
            scroll_offset: 0,
            bundle: None,
            is_loading: false,
            error_msg: None,
            show_calendar: false,
            show_tietkhi_details: false,
            show_search: false,
            search_input: String::new(),
        };
        
        app.load_data();
        app
    }

    pub fn load_data(&mut self) {
        self.is_loading = true;
        self.error_msg = None;
        
        // In the future this might be done asynchronously if we want non-blocking UI
        // But for now we just do it synchronously like the old app
        let includes = vec![
            Include::Base,
            Include::CanChi,
            Include::TietKhi,
            Include::Hours,
            Include::Fortune,
            Include::Insight,
        ];
        
        match get_day_bundle_for_date(
            self.date.day() as i32,
            self.date.month() as i32,
            self.date.year(),
            &includes,
            None,
        ) {
            Ok(bundle) => {
                self.bundle = Some(bundle);
                self.is_loading = false;
            }
            Err(e) => {
                self.error_msg = Some(e);
                self.is_loading = false;
            }
        }
    }

    pub fn next_day(&mut self) {
        if let Some(next) = self.date.succ_opt() {
            self.date = next;
            self.scroll_offset = 0;
            self.load_data();
        }
    }

    pub fn prev_day(&mut self) {
        if let Some(prev) = self.date.pred_opt() {
            self.date = prev;
            self.scroll_offset = 0;
            self.load_data();
        }
    }
    
    pub fn go_today(&mut self) {
        self.date = Local::now().naive_local().date();
        self.scroll_offset = 0;
        self.load_data();
    }
    
    pub fn next_lens(&mut self) {
        self.lens = self.lens.next();
        self.scroll_offset = 0; // Reset scroll on lens change
    }
    
    pub fn toggle_calendar(&mut self) {
        self.show_calendar = !self.show_calendar;
    }
    
    pub fn toggle_tietkhi(&mut self) {
        self.show_tietkhi_details = !self.show_tietkhi_details;
    }
    
    pub fn toggle_search(&mut self) {
        self.show_search = !self.show_search;
        if self.show_search {
            self.search_input.clear();
        }
    }
    
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
    
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }
    
    // We can add page up/down later by exposing the viewport height to the state,
    // or passing the step amount from the event handler
    pub fn scroll_down_by(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_add(lines);
    }
    
    pub fn scroll_up_by(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }
}
