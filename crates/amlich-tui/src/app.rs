use amlich_api::{get_day_insight_for_date, DayInfoDto, DayInsightDto, HolidayDto};
use chrono::{Datelike, Local, NaiveDate};

#[derive(Clone, Copy)]
pub enum InsightLang {
    Vi,
    En,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DensityMode {
    Compact,
    Normal,
    Detail,
}

impl DensityMode {
    pub fn next(self) -> Self {
        match self {
            DensityMode::Compact => DensityMode::Normal,
            DensityMode::Normal => DensityMode::Detail,
            DensityMode::Detail => DensityMode::Compact,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            DensityMode::Compact => "Compact",
            DensityMode::Normal => "Normal",
            DensityMode::Detail => "Detail",
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
    pub show_extra_cultural: bool,

    // Density mode
    pub density_mode: DensityMode,
}

impl App {
    pub fn new() -> Self {
        let today = Local::now().date_naive();
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
            show_extra_cultural: false,
            density_mode: DensityMode::Normal,
        };
        app.load_month();
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
        self.view_year = self.today.year();
        self.view_month = self.today.month();
        self.selected_day = self.today.day();
        self.load_month();
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

    pub fn toggle_density_mode(&mut self) {
        self.density_mode = self.density_mode.next();
    }

    pub fn toggle_extra_cultural(&mut self) {
        self.show_extra_cultural = !self.show_extra_cultural;
        // If extra content is enabled, ensure the insight panel is visible.
        if self.show_extra_cultural && !self.show_insight {
            self.show_insight = true;
        }
    }

    pub fn selected_insight(&self) -> Option<DayInsightDto> {
        get_day_insight_for_date(
            self.selected_day as i32,
            self.view_month as i32,
            self.view_year,
        )
        .ok()
    }
}
