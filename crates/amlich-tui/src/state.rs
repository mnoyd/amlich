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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewMode {
    Day,
    Calendar,
}

pub struct AppState {
    pub running: bool,
    pub date: NaiveDate,
    pub lens: FocusLens,
    pub view_mode: ViewMode,
    pub scroll_offset: u16,

    // Data cache for the current date
    pub bundle: Option<DayBundleDto>,
    pub is_loading: bool,
    pub error_msg: Option<String>,

    // UI toggles
    pub show_guidance_details: bool,
    pub show_tietkhi_details: bool,
    pub show_search: bool,
    pub search_input: String,
    pub calendar_cursor: NaiveDate,
}

impl AppState {
    pub fn new(initial_date: Option<NaiveDate>) -> Self {
        let date = initial_date.unwrap_or_else(|| Local::now().naive_local().date());

        let mut app = Self {
            running: true,
            date,
            lens: FocusLens::General,
            view_mode: ViewMode::Day,
            scroll_offset: 0,
            bundle: None,
            is_loading: false,
            error_msg: None,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_search: false,
            search_input: String::new(),
            calendar_cursor: date,
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
        self.toggle_calendar_view();
    }

    pub fn is_calendar_view(&self) -> bool {
        self.view_mode == ViewMode::Calendar
    }

    pub fn toggle_calendar_view(&mut self) {
        if self.is_calendar_view() {
            self.close_calendar_view();
        } else {
            self.open_calendar_view();
        }
    }

    pub fn open_calendar_view(&mut self) {
        self.view_mode = ViewMode::Calendar;
        self.calendar_cursor = self.date;
        self.scroll_offset = 0;
    }

    pub fn close_calendar_view(&mut self) {
        self.view_mode = ViewMode::Day;
    }

    pub fn apply_calendar_selection(&mut self) {
        self.date = self.calendar_cursor;
        self.view_mode = ViewMode::Day;
        self.scroll_offset = 0;
        self.load_data();
    }

    pub fn calendar_move_days(&mut self, delta_days: i64) {
        if let Some(next) = self
            .calendar_cursor
            .checked_add_signed(chrono::Duration::days(delta_days))
        {
            self.calendar_cursor = next;
        }
    }

    pub fn calendar_go_today(&mut self) {
        self.calendar_cursor = Local::now().naive_local().date();
    }

    pub fn calendar_prev_month(&mut self) {
        self.calendar_shift_month(-1);
    }

    pub fn calendar_next_month(&mut self) {
        self.calendar_shift_month(1);
    }

    fn calendar_shift_month(&mut self, delta_months: i32) {
        let current = self.calendar_cursor;
        let total_months = current.year() * 12 + current.month0() as i32 + delta_months;
        let target_year = total_months.div_euclid(12);
        let target_month0 = total_months.rem_euclid(12) as u32;
        let target_month = target_month0 + 1;

        let clamped_day = current.day().min(days_in_month(target_year, target_month));
        if let Some(next) = NaiveDate::from_ymd_opt(target_year, target_month, clamped_day) {
            self.calendar_cursor = next;
        }
    }

    pub fn toggle_tietkhi(&mut self) {
        self.show_tietkhi_details = !self.show_tietkhi_details;
    }

    pub fn toggle_guidance_details(&mut self) {
        self.show_guidance_details = !self.show_guidance_details;
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

fn days_in_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };

    let next_month_start =
        NaiveDate::from_ymd_opt(next_year, next_month, 1).expect("valid next month date");
    next_month_start
        .pred_opt()
        .expect("previous day exists")
        .day()
}
