use std::collections::BTreeSet;
use std::path::PathBuf;

use amlich_api::v2::{DayBundleDto, Include};
use amlich_api::{
    RecommendationBucketDto, RecommendationPackCatalogEntryDto, RulesetCatalogEntryDto,
};
use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

use super::ui_prefs::{default_verbosity_for_size, VerbosityMode};

const DEFAULT_EVENT_KIND: &str = "default";
const EVENT_KIND_OPTIONS: [&str; 4] = [
    DEFAULT_EVENT_KIND,
    "contract_signing",
    "medical_checkup",
    "travel",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusLens {
    General,
    Planning,
    DayDetail,
    Personal,
}

impl FocusLens {
    pub fn next(&self) -> Self {
        match self {
            Self::General => Self::Planning,
            Self::Planning => Self::DayDetail,
            Self::DayDetail => Self::Personal,
            Self::Personal => Self::General,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveView {
    Today,
    DayDetail,
    Hours,
    Calendar,
    Personal,
    GraphInspector,
}

impl ActiveView {
    pub fn label(self) -> &'static str {
        match self {
            Self::Today => "Hôm Nay",
            Self::DayDetail => "Chi Tiết Ngày",
            Self::Hours => "Giờ Tốt",
            Self::Calendar => "Lịch",
            Self::Personal => "Cá Nhân",
            Self::GraphInspector => "Đồ Thị Ngữ Nghĩa",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Self::Today => "Today",
            Self::DayDetail => "Ngày",
            Self::Hours => "Giờ",
            Self::Calendar => "Lịch",
            Self::Personal => "Nhân",
            Self::GraphInspector => "Graph",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    SearchModal,
    ContextModal,
    HelpModal,
    PersonalProfileModal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PageSection {
    Explorer,
    Hero,
    Recommendations,
    Timing,
    Travel,
    Risks,
    TraditionalEvidence,
    ExpandedDetails,
}

impl PageSection {
    pub fn next(&self) -> Self {
        match self {
            Self::Explorer => Self::Hero,
            Self::Hero => Self::Recommendations,
            Self::Recommendations => Self::Timing,
            Self::Timing => Self::Travel,
            Self::Travel => Self::Risks,
            Self::Risks => Self::TraditionalEvidence,
            Self::TraditionalEvidence => Self::ExpandedDetails,
            Self::ExpandedDetails => Self::Explorer,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorerField {
    Date,
    Ruleset,
    EventKind,
    RecommendationPacks,
    Actions,
}

impl ExplorerField {
    pub fn next(self) -> Self {
        match self {
            Self::Date => Self::Ruleset,
            Self::Ruleset => Self::EventKind,
            Self::EventKind => Self::RecommendationPacks,
            Self::RecommendationPacks => Self::Actions,
            Self::Actions => Self::Date,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Date => Self::Actions,
            Self::Ruleset => Self::Date,
            Self::EventKind => Self::Ruleset,
            Self::RecommendationPacks => Self::EventKind,
            Self::Actions => Self::RecommendationPacks,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplorerAction {
    Apply,
    Reset,
}

impl ExplorerAction {
    pub fn next(self) -> Self {
        match self {
            Self::Apply => Self::Reset,
            Self::Reset => Self::Apply,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplorerSelection {
    pub date: NaiveDate,
    pub ruleset_id: Option<String>,
    pub event_kind: Option<String>,
    pub enabled_pack_ids: Vec<String>,
}

impl ExplorerSelection {
    fn from_loaded_data(bundle: &DayBundleDto, query: &ExplorerSelection) -> Self {
        let date = NaiveDate::from_ymd_opt(
            bundle.solar.year,
            bundle.solar.month as u32,
            bundle.solar.day as u32,
        )
        .expect("bundle has valid solar date");
        let enabled_pack_ids = bundle
            .contextual_recommendations
            .as_ref()
            .or(bundle.daily_recommendations.as_ref())
            .map(|recommendations| {
                recommendations
                    .active_packs
                    .iter()
                    .map(|pack| pack.pack_id.clone())
                    .collect()
            })
            .filter(|packs: &Vec<String>| !packs.is_empty())
            .unwrap_or_else(|| query.enabled_pack_ids.clone());

        Self {
            date,
            ruleset_id: Some(bundle.ruleset_id.clone()),
            event_kind: query.event_kind.clone(),
            enabled_pack_ids,
        }
    }

    pub(crate) fn normalized(
        mut self,
        ruleset_catalog: &[RulesetCatalogEntryDto],
        recommendation_pack_catalog: &[RecommendationPackCatalogEntryDto],
    ) -> Self {
        self.ruleset_id = normalize_ruleset_selection(ruleset_catalog, self.ruleset_id.as_deref());
        self.enabled_pack_ids =
            normalize_enabled_pack_selection(recommendation_pack_catalog, &self.enabled_pack_ids);
        self
    }

    pub(crate) fn defaults(date: NaiveDate, catalog: &[RulesetCatalogEntryDto]) -> Self {
        let ruleset_id = catalog
            .iter()
            .find(|entry| entry.is_default)
            .or_else(|| catalog.first())
            .map(|entry| entry.canonical_id.clone());

        Self {
            date,
            ruleset_id,
            event_kind: None,
            enabled_pack_ids: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersonalField {
    BirthYear,
    BirthMonth,
    BirthDay,
    BirthHour,
    BirthMinute,
    Gender,
}

impl PersonalField {
    pub fn next(self) -> Self {
        match self {
            Self::BirthYear => Self::BirthMonth,
            Self::BirthMonth => Self::BirthDay,
            Self::BirthDay => Self::BirthHour,
            Self::BirthHour => Self::BirthMinute,
            Self::BirthMinute => Self::Gender,
            Self::Gender => Self::BirthYear,
        }
    }

    pub fn previous(self) -> Self {
        self.next()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonalDraft {
    pub birth_year: String,
    pub birth_month: String,
    pub birth_day: String,
    pub birth_hour: String,
    pub birth_minute: String,
    pub gender: Option<amlich_core::almanac::tu_menh::Gender>,
}

impl PersonalDraft {
    fn empty() -> Self {
        Self {
            birth_year: String::new(),
            birth_month: String::new(),
            birth_day: String::new(),
            birth_hour: String::new(),
            birth_minute: String::new(),
            gender: None,
        }
    }

    fn from_persisted(profile: &PersistedUserProfile) -> Self {
        Self {
            birth_year: profile.birth_year.map(|v| v.to_string()).unwrap_or_default(),
            birth_month: profile.birth_month.map(|v| v.to_string()).unwrap_or_default(),
            birth_day: profile.birth_day.map(|v| v.to_string()).unwrap_or_default(),
            birth_hour: profile.birth_hour.map(|v| v.to_string()).unwrap_or_default(),
            birth_minute: profile.birth_minute.map(|v| v.to_string()).unwrap_or_default(),
            gender: profile.gender.map(|g| match g {
                PersistedProfileGender::Male => amlich_core::almanac::tu_menh::Gender::Male,
                PersistedProfileGender::Female => amlich_core::almanac::tu_menh::Gender::Female,
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationRowVm {
    pub bucket: RecommendationBucketDto,
    pub label: String,
    pub reason_chip: Option<String>,
    pub reason_details: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeroVerdictVm {
    pub summary: String,
    pub strongest_positive: Option<String>,
    pub strongest_negative: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskSummaryVm {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayDetailTimingSummaryVm {
    pub summary: String,
    pub windows: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayDetailRiskBoardVm {
    pub headline: Option<String>,
    pub critical_items: Vec<String>,
    pub caution_items: Vec<String>,
    pub conflict_items: Vec<String>,
    pub notice: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayDetailVerdictSupportVm {
    pub support_line: String,
    pub layer_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectionVerdictVm {
    pub summary: String,
    pub directions: Vec<String>,
    pub deity_context: Option<String>,
    pub note: Option<String>,
    pub matrix_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoursVerdictVm {
    pub summary: String,
    pub top_windows: Vec<String>,
    pub caution: Option<String>,
    pub bad_windows: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayIdentitySummaryVm {
    pub headline: String,
    pub detail_lines: Vec<String>,
    pub application_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraditionalEvidenceSummaryVm {
    pub headline: Option<String>,
    pub positive_signals: Vec<String>,
    pub caution_signals: Vec<String>,
    pub provenance: Vec<String>,
    pub source_notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeasonalVerdictVm {
    pub headline: String,
    pub implication: String,
    pub application_lines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileAvailabilityVm {
    pub has_personal_overlay: bool,
    pub note: String,
    pub missing_requirements: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecommendationLayerKind {
    Baseline,
    Contextual,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecommendationLayerVm {
    pub kind: RecommendationLayerKind,
    pub label: String,
    pub summary: String,
    pub scope_label: String,
    pub ruleset_id: String,
    pub ruleset_version: String,
    pub profile: String,
    pub active_pack_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivePackVm {
    pub pack_id: String,
    pub version: String,
    pub source_family: String,
    pub mode: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PersistedProfileGender {
    Male,
    Female,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct PersistedUserProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    birth_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birth_month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birth_day: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birth_hour: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birth_minute: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gender: Option<PersistedProfileGender>,
}

pub struct AppState {
    pub running: bool,
    pub date: NaiveDate,
    pub scroll_offset: u16,
    pub content_height: u16,
    pub viewport_height: u16,

    // Data cache for the current date
    pub bundle: Option<DayBundleDto>,
    pub personal_matrix: Option<amlich_api::PersonalDayMatrixReportDto>,
    pub is_loading: bool,
    pub error_msg: Option<String>,
    pub ruleset_catalog: Vec<RulesetCatalogEntryDto>,
    pub recommendation_pack_catalog: Vec<RecommendationPackCatalogEntryDto>,
    pub applied_selection: ExplorerSelection,
    pub staged_selection: ExplorerSelection,
    pub explorer_focus: ExplorerField,
    pub explorer_action: ExplorerAction,
    pub pack_cursor: usize,

    // UI toggles
    pub show_guidance_details: bool,
    pub show_tietkhi_details: bool,
    pub show_evidence: bool,
    pub show_week_strip: bool,
    pub show_graph_recommendations: bool,
    pub verbosity: VerbosityMode,
    pub active_view: ActiveView,
    pub view_history: Vec<ActiveView>,
    pub app_mode: AppMode,
    pub focused_section: PageSection,
    pub zoomed_section: Option<PageSection>,
    pub expanded_sections: BTreeSet<PageSection>,
    pub search_input: String,
    pub personal_focus: PersonalField,
    pub personal_draft: PersonalDraft,
    pub calendar_cursor: NaiveDate,
    pub(crate) navigation_history: Vec<NaiveDate>,
}

impl AppState {
    pub fn new(initial_date: Option<NaiveDate>, terminal_size: Option<(u16, u16)>) -> Self {
        let date = initial_date.unwrap_or_else(|| Local::now().naive_local().date());
        let ruleset_catalog = amlich_api::get_ruleset_catalog();
        let recommendation_pack_catalog = amlich_api::get_recommendation_pack_catalog();
        let default_selection = ExplorerSelection::defaults(date, &ruleset_catalog)
            .normalized(&ruleset_catalog, &recommendation_pack_catalog);

        let verbosity = terminal_size
            .map(|(width, height)| default_verbosity_for_size(width, height))
            .unwrap_or(VerbosityMode::Compact);

        let persisted_profile = load_persisted_profile();
        let mut app = Self {
            running: true,
            date,
            scroll_offset: 0,
            content_height: 0,
            viewport_height: 0,
            bundle: None,
            personal_matrix: None,
            is_loading: false,
            error_msg: None,
            ruleset_catalog,
            recommendation_pack_catalog,
            applied_selection: default_selection.clone(),
            staged_selection: default_selection,
            explorer_focus: ExplorerField::Date,
            explorer_action: ExplorerAction::Apply,
            pack_cursor: 0,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            show_week_strip: true,
            show_graph_recommendations: false,
            verbosity,
            active_view: ActiveView::Today,
            view_history: Vec::new(),
            app_mode: AppMode::Normal,
            focused_section: PageSection::Explorer,
            zoomed_section: None,
            expanded_sections: BTreeSet::new(),
            search_input: String::new(),
            personal_focus: PersonalField::BirthYear,
            personal_draft: PersonalDraft::from_persisted(&persisted_profile),
            calendar_cursor: date,
            navigation_history: Vec::new(),
        };

        app.load_data();
        app
    }

    pub fn load_data(&mut self) {
        self.is_loading = true;
        self.error_msg = None;
        let query_selection = self.applied_selection.clone();

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

        let query = amlich_api::DateQuery {
            day: self.date.day() as i32,
            month: self.date.month() as i32,
            year: self.date.year(),
            timezone: None,
            ruleset_id: query_selection.ruleset_id.clone(),
            event_kind: query_selection.event_kind.clone(),
            enabled_pack_ids: query_selection.enabled_pack_ids.clone(),
        };

        match amlich_api::v2::get_day_bundle(&query, &includes) {
            Ok(bundle) => {
                let selection = ExplorerSelection::from_loaded_data(&bundle, &query_selection);
                self.date = selection.date;
                self.bundle = Some(bundle);
                self.personal_matrix = self.load_personal_matrix();
                self.applied_selection = selection.clone();
                self.staged_selection = selection;
                self.pack_cursor = self.clamp_pack_cursor();
                self.is_loading = false;
            }
            Err(e) => {
                self.error_msg = Some(e);
                self.personal_matrix = None;
                self.is_loading = false;
            }
        }
    }

    pub fn next_day(&mut self) {
        if let Some(next) = self.date.succ_opt() {
            self.staged_selection.date = next;
            self.scroll_offset = 0;
        }
    }

    pub fn prev_day(&mut self) {
        if let Some(prev) = self.date.pred_opt() {
            self.staged_selection.date = prev;
            self.scroll_offset = 0;
        }
    }

    pub fn go_today(&mut self) {
        self.staged_selection.date = Local::now().naive_local().date();
        self.scroll_offset = 0;
    }

    pub fn navigate_days(&mut self, delta: i64) {
        if delta == 0 {
            return;
        }

        let Some(target) = self.date.checked_add_signed(chrono::Duration::days(delta)) else {
            return;
        };

        self.jump_to_date(target);
    }

    pub fn navigate_weeks(&mut self, delta: i64) {
        self.navigate_days(delta.saturating_mul(7));
    }

    pub fn navigate_months(&mut self, delta: i32) {
        if delta == 0 {
            return;
        }

        let current = self.date;
        let total_months = current.year() * 12 + current.month0() as i32 + delta;
        let target_year = total_months.div_euclid(12);
        let target_month0 = total_months.rem_euclid(12) as u32;
        let target_month = target_month0 + 1;
        let target_day = current.day().min(days_in_month(target_year, target_month));

        if let Some(target) = NaiveDate::from_ymd_opt(target_year, target_month, target_day) {
            self.jump_to_date(target);
        }
    }

    pub fn jump_to_today(&mut self) {
        self.jump_to_date(Local::now().naive_local().date());
    }

    pub fn jump_to_date(&mut self, date: NaiveDate) {
        if date == self.date {
            return;
        }

        self.navigation_history.push(self.date);
        self.apply_navigated_date(date);
    }

    pub fn undo_navigation(&mut self) {
        let Some(previous_date) = self.navigation_history.pop() else {
            return;
        };

        self.apply_navigated_date(previous_date);
    }

    pub fn toggle_week_strip(&mut self) {
        self.show_week_strip = !self.show_week_strip;
    }

    pub fn has_event_today(&self) -> bool {
        let Some(bundle) = &self.bundle else {
            return false;
        };
        if let Some(insight) = &bundle.insight {
            if insight.festival.is_some() || insight.holiday.is_some() {
                return true;
            }
        }
        bundle.lunar.day == 1 || bundle.lunar.day == 15
    }

    pub fn available_views(&self) -> Vec<ActiveView> {
        vec![
            ActiveView::Today,
            ActiveView::DayDetail,
            ActiveView::Hours,
            ActiveView::Calendar,
            ActiveView::Personal,
            ActiveView::GraphInspector,
        ]
    }

    pub fn next_view(&mut self) {
        self.view_history.push(self.active_view);
        let views = self.available_views();
        let idx = views
            .iter()
            .position(|&v| v == self.active_view)
            .unwrap_or(0);
        self.active_view = views[(idx + 1) % views.len()];
        self.scroll_offset = 0;
    }

    pub fn prev_view(&mut self) {
        self.view_history.push(self.active_view);
        let views = self.available_views();
        let idx = views
            .iter()
            .position(|&v| v == self.active_view)
            .unwrap_or(0);
        self.active_view = views[(idx + views.len() - 1) % views.len()];
        self.scroll_offset = 0;
    }

    pub fn go_to_view(&mut self, view: ActiveView) {
        if self.active_view == view {
            return;
        }
        self.view_history.push(self.active_view);
        self.active_view = view;
        self.scroll_offset = 0;
    }

    pub fn active_view_label(&self) -> &'static str {
        self.active_view.label()
    }

    pub fn focus_next_section(&mut self) {
        self.focused_section = self.focused_section.next();
        self.scroll_offset = 0;
    }

    pub fn focus_previous_section(&mut self) {
        self.focused_section = match self.focused_section {
            PageSection::Explorer => PageSection::ExpandedDetails,
            PageSection::Hero => PageSection::Explorer,
            PageSection::Recommendations => PageSection::Hero,
            PageSection::Timing => PageSection::Recommendations,
            PageSection::Travel => PageSection::Timing,
            PageSection::Risks => PageSection::Travel,
            PageSection::TraditionalEvidence => PageSection::Risks,
            PageSection::ExpandedDetails => PageSection::TraditionalEvidence,
        };
        self.scroll_offset = 0;
    }

    pub fn focus_section(&mut self, section: PageSection) {
        self.focused_section = section;
        self.scroll_offset = 0;
    }

    pub fn is_calendar_view(&self) -> bool {
        self.active_view == ActiveView::Calendar
    }

    pub fn toggle_calendar_view(&mut self) {
        if self.is_calendar_view() {
            self.close_calendar_view();
        } else {
            self.open_calendar_view();
        }
    }

    pub fn open_calendar_view(&mut self) {
        self.go_to_view(ActiveView::Calendar);
        self.calendar_cursor = self.date;
    }

    pub fn close_calendar_view(&mut self) {
        if self.active_view == ActiveView::Calendar {
            if let Some(prev) = self.view_history.pop() {
                self.active_view = prev;
            } else {
                self.active_view = ActiveView::Today;
            }
        }
    }

    pub fn apply_calendar_selection(&mut self) {
        let selected_date = self.calendar_cursor;
        self.close_calendar_view();
        self.jump_to_date(selected_date);
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

    pub fn cycle_ruleset(&mut self, step: i32) {
        if self.ruleset_catalog.is_empty() {
            return;
        }

        let current = self
            .staged_selection
            .ruleset_id
            .as_deref()
            .and_then(|id| {
                self.ruleset_catalog
                    .iter()
                    .position(|entry| entry.canonical_id == id)
            })
            .unwrap_or_else(|| self.default_ruleset_index());
        let next = wrap_index(current, self.ruleset_catalog.len(), step);
        self.staged_selection.ruleset_id = Some(self.ruleset_catalog[next].canonical_id.clone());
    }

    pub fn cycle_event_kind(&mut self, step: i32) {
        let current = EVENT_KIND_OPTIONS
            .iter()
            .position(
                |kind| match (self.staged_selection.event_kind.as_deref(), *kind) {
                    (None, DEFAULT_EVENT_KIND) => true,
                    (Some(active), candidate) => active == candidate,
                    _ => false,
                },
            )
            .unwrap_or(0);
        let next = wrap_index(current, EVENT_KIND_OPTIONS.len(), step);
        self.staged_selection.event_kind = match EVENT_KIND_OPTIONS[next] {
            DEFAULT_EVENT_KIND => None,
            other => Some(other.to_string()),
        };
    }

    pub fn move_pack_cursor(&mut self, step: i32) {
        let len = self.recommendation_pack_catalog.len();
        if len == 0 {
            self.pack_cursor = 0;
            return;
        }
        self.pack_cursor = wrap_index(self.pack_cursor, len, step);
    }

    pub fn toggle_focused_pack(&mut self) {
        let Some(pack) = self.recommendation_pack_catalog.get(self.pack_cursor) else {
            return;
        };

        if let Some(index) = self
            .staged_selection
            .enabled_pack_ids
            .iter()
            .position(|id| id == &pack.pack_id)
        {
            self.staged_selection.enabled_pack_ids.remove(index);
        } else {
            self.staged_selection
                .enabled_pack_ids
                .push(pack.pack_id.clone());
        }
    }

    pub fn focus_next_explorer_field(&mut self) {
        self.explorer_focus = self.explorer_focus.next();
    }

    pub fn focus_previous_explorer_field(&mut self) {
        self.explorer_focus = self.explorer_focus.previous();
    }

    pub fn cycle_explorer_action(&mut self) {
        self.explorer_action = self.explorer_action.next();
    }

    pub fn activate_explorer_focus(&mut self) {
        match self.explorer_focus {
            ExplorerField::RecommendationPacks => self.toggle_focused_pack(),
            ExplorerField::Actions => match self.explorer_action {
                ExplorerAction::Apply => self.apply_staged_selection(),
                ExplorerAction::Reset => self.reset_staged_selection(),
            },
            _ => {}
        }
    }

    pub fn apply_staged_selection(&mut self) {
        self.staged_selection = self
            .staged_selection
            .clone()
            .normalized(&self.ruleset_catalog, &self.recommendation_pack_catalog);
        self.date = self.staged_selection.date;
        self.applied_selection = self.staged_selection.clone();
        self.scroll_offset = 0;
        self.load_data();
    }

    pub fn reset_staged_selection(&mut self) {
        self.staged_selection =
            ExplorerSelection::defaults(self.applied_selection.date, &self.ruleset_catalog)
                .normalized(&self.ruleset_catalog, &self.recommendation_pack_catalog);
        self.pack_cursor = self.clamp_pack_cursor();
        self.scroll_offset = 0;
    }

    pub fn explorer_has_staged_changes(&self) -> bool {
        self.staged_selection != self.applied_selection
    }

    pub fn ruleset_label(&self, ruleset_id: Option<&str>) -> String {
        match ruleset_id.and_then(|id| {
            self.ruleset_catalog
                .iter()
                .find(|entry| entry.canonical_id == id)
        }) {
            Some(entry) => format!(
                "{} · v{} · {} · {}",
                entry.canonical_id, entry.version, entry.region, entry.profile
            ),
            None => "Mặc định hệ thống".to_string(),
        }
    }

    pub fn ruleset_brief_label(&self, ruleset_id: Option<&str>) -> String {
        match ruleset_id.and_then(|id| {
            self.ruleset_catalog
                .iter()
                .find(|entry| entry.canonical_id == id)
        }) {
            Some(entry) => format!("{}@{}", entry.canonical_id, entry.version),
            None => "default".to_string(),
        }
    }

    pub fn event_kind_label(&self, event_kind: Option<&str>) -> String {
        match event_kind {
            Some("contract_signing") => "contract_signing · Ký kết".to_string(),
            Some("medical_checkup") => "medical_checkup · Khám chữa".to_string(),
            Some("travel") => "travel · Xuất hành".to_string(),
            Some(other) => other.to_string(),
            None => "default · Không áp ngữ cảnh".to_string(),
        }
    }

    pub fn pack_status_rows(&self) -> Vec<String> {
        self.recommendation_pack_catalog
            .iter()
            .enumerate()
            .map(|(index, pack)| {
                let selected = self
                    .staged_selection
                    .enabled_pack_ids
                    .iter()
                    .any(|id| id == &pack.pack_id);
                let cursor = if index == self.pack_cursor { '>' } else { ' ' };
                let marker = if selected { "+" } else { "-" };
                format!(
                    "{cursor} [{marker}] {} · {} · {}",
                    pack.pack_id, pack.source_family, pack.mode
                )
            })
            .collect()
    }

    pub fn active_pack_summary(&self, selection: &ExplorerSelection) -> String {
        if selection.enabled_pack_ids.is_empty() {
            "Không có gói tăng cường".to_string()
        } else {
            selection.enabled_pack_ids.join(", ")
        }
    }

    pub fn active_bundle_packs_summary(&self) -> String {
        let packs = self.active_bundle_packs();
        if packs.is_empty() {
            self.active_pack_summary(&self.applied_selection)
        } else {
            packs
                .into_iter()
                .map(|pack| pack.pack_id)
                .collect::<Vec<_>>()
                .join(", ")
        }
    }

    pub fn active_bundle_packs(&self) -> Vec<ActivePackVm> {
        let Some(bundle) = &self.bundle else {
            return Vec::new();
        };

        bundle
            .contextual_recommendations
            .as_ref()
            .or(bundle.daily_recommendations.as_ref())
            .map(|recommendations| {
                recommendations
                    .active_packs
                    .iter()
                    .map(|pack| ActivePackVm {
                        pack_id: pack.pack_id.clone(),
                        version: pack.version.clone(),
                        source_family: pack.source_family.clone(),
                        mode: pack.mode.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn inspection_context_summary(&self) -> String {
        match self.applied_selection.event_kind.as_deref() {
            Some(kind) => format!("Kích hoạt ngữ cảnh: {}", self.event_kind_label(Some(kind))),
            None => "Kích hoạt ngữ cảnh: default · Không áp ngữ cảnh".to_string(),
        }
    }

    pub fn retry_load(&mut self) {
        self.load_data();
    }

    fn default_ruleset_index(&self) -> usize {
        self.ruleset_catalog
            .iter()
            .position(|entry| entry.is_default)
            .unwrap_or(0)
    }

    fn clamp_pack_cursor(&self) -> usize {
        self.recommendation_pack_catalog
            .len()
            .saturating_sub(1)
            .min(self.pack_cursor)
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
        self.set_section_expanded(PageSection::TraditionalEvidence, self.show_tietkhi_details);
    }

    pub fn toggle_guidance_details(&mut self) {
        self.show_guidance_details = !self.show_guidance_details;
        self.set_section_expanded(PageSection::Recommendations, self.show_guidance_details);
    }

    pub fn toggle_evidence(&mut self) {
        self.show_evidence = !self.show_evidence;
    }

    pub fn toggle_graph_recommendations(&mut self) {
        self.show_graph_recommendations = !self.show_graph_recommendations;
    }

    pub fn toggle_zoom_for_focused_section(&mut self) {
        if self.zoomed_section == Some(self.focused_section) {
            self.zoomed_section = None;
        } else {
            self.zoomed_section = Some(self.focused_section);
        }
    }

    pub fn toggle_expand_focused_section(&mut self) {
        let is_expanded = self.is_section_expanded(self.focused_section);
        self.set_section_expanded(self.focused_section, !is_expanded);
    }

    pub fn is_section_expanded(&self, section: PageSection) -> bool {
        self.expanded_sections.contains(&section)
    }

    pub fn expand_section(&mut self, section: PageSection) {
        self.focus_section(section);
        self.set_section_expanded(section, true);
    }

    pub fn recommendation_layers(&self) -> Vec<RecommendationLayerVm> {
        crate::view_models::shared::recommendation_layers(self)
    }

    pub fn top_recommendation_rows(&self) -> Vec<RecommendationRowVm> {
        crate::view_models::today::top_rows(self)
    }

    pub fn hero_verdict(&self) -> Option<HeroVerdictVm> {
        crate::view_models::today::hero_verdict(self)
    }

    pub fn day_detail_timing_summary(&self) -> Option<DayDetailTimingSummaryVm> {
        crate::view_models::hours::day_detail_timing_summary(self)
    }

    pub fn hours_verdict(&self) -> Option<HoursVerdictVm> {
        crate::view_models::hours::hours_verdict(self)
    }

    pub fn risk_summary(&self) -> RiskSummaryVm {
        crate::view_models::today::risk_summary(self)
    }

    pub fn day_detail_risk_board(&self) -> DayDetailRiskBoardVm {
        crate::view_models::day_detail::day_detail_risk_board(self)
    }

    pub fn day_detail_verdict_support(&self) -> Option<DayDetailVerdictSupportVm> {
        crate::view_models::day_detail::day_detail_verdict_support(self)
    }

    pub fn direction_verdict(&self) -> Option<DirectionVerdictVm> {
        crate::view_models::day_detail::direction_verdict(self)
    }

    pub fn day_identity_summary(&self) -> Option<DayIdentitySummaryVm> {
        crate::view_models::today::day_identity_summary(self)
    }

    pub fn traditional_evidence_summary(&self) -> Option<TraditionalEvidenceSummaryVm> {
        crate::view_models::today::traditional_evidence_summary(self)
    }

    pub fn seasonal_verdict(&self) -> Option<SeasonalVerdictVm> {
        crate::view_models::seasonal::seasonal_verdict(self)
    }

    pub fn profile_availability_summary(&self) -> Option<ProfileAvailabilityVm> {
        crate::view_models::seasonal::profile_availability_summary(self)
    }

    pub fn sensitive_domain_notice(&self) -> Option<String> {
        crate::view_models::seasonal::sensitive_domain_notice(self)
    }

    pub fn toggle_search(&mut self) {
        if self.app_mode == AppMode::SearchModal {
            self.app_mode = AppMode::Normal;
        } else {
            self.app_mode = AppMode::SearchModal;
            self.search_input.clear();
        }
    }

    pub fn toggle_context_modal(&mut self) {
        if self.app_mode == AppMode::ContextModal {
            self.app_mode = AppMode::Normal;
        } else {
            self.app_mode = AppMode::ContextModal;
            self.explorer_focus = ExplorerField::EventKind;
        }
    }

    pub fn toggle_help_modal(&mut self) {
        if self.app_mode == AppMode::HelpModal {
            self.app_mode = AppMode::Normal;
        } else {
            self.app_mode = AppMode::HelpModal;
        }
    }

    pub fn toggle_personal_profile_modal(&mut self) {
        if self.app_mode == AppMode::PersonalProfileModal {
            self.app_mode = AppMode::Normal;
            return;
        }

        if let Some(insight) = self
            .bundle
            .as_ref()
            .and_then(|bundle| bundle.insight.as_ref())
        {
            self.personal_draft = PersonalDraft {
                birth_year: self.personal_draft.birth_year.clone(),
                birth_month: self.personal_draft.birth_month.clone(),
                birth_day: self.personal_draft.birth_day.clone(),
                birth_hour: self.personal_draft.birth_hour.clone(),
                birth_minute: self.personal_draft.birth_minute.clone(),
                gender: self.personal_draft.gender,
            };
            if (insight.tu_menh.is_some() || insight.dai_van.is_some())
                && self.personal_draft.birth_year.is_empty()
            {
                self.personal_draft.birth_year = self.date.year().to_string();
            }
        } else {
            self.personal_draft = PersonalDraft::empty();
        }
        self.personal_focus = PersonalField::BirthYear;
        self.app_mode = AppMode::PersonalProfileModal;
    }

    pub fn personal_insert_char(&mut self, ch: char) {
        match self.personal_focus {
            PersonalField::BirthYear
                if ch.is_ascii_digit() && self.personal_draft.birth_year.len() < 4 =>
            {
                self.personal_draft.birth_year.push(ch)
            }
            PersonalField::BirthMonth
                if ch.is_ascii_digit() && self.personal_draft.birth_month.len() < 2 =>
            {
                self.personal_draft.birth_month.push(ch)
            }
            PersonalField::BirthDay
                if ch.is_ascii_digit() && self.personal_draft.birth_day.len() < 2 =>
            {
                self.personal_draft.birth_day.push(ch)
            }
            PersonalField::BirthHour
                if ch.is_ascii_digit() && self.personal_draft.birth_hour.len() < 2 =>
            {
                self.personal_draft.birth_hour.push(ch)
            }
            PersonalField::BirthMinute
                if ch.is_ascii_digit() && self.personal_draft.birth_minute.len() < 2 =>
            {
                self.personal_draft.birth_minute.push(ch)
            }
            _ => {}
        }
    }

    pub fn personal_backspace(&mut self) {
        match self.personal_focus {
            PersonalField::BirthYear => {
                self.personal_draft.birth_year.pop();
            }
            PersonalField::BirthMonth => {
                self.personal_draft.birth_month.pop();
            }
            PersonalField::BirthDay => {
                self.personal_draft.birth_day.pop();
            }
            PersonalField::BirthHour => {
                self.personal_draft.birth_hour.pop();
            }
            PersonalField::BirthMinute => {
                self.personal_draft.birth_minute.pop();
            }
            PersonalField::Gender => {}
        }
    }

    pub fn personal_next_field(&mut self) {
        self.personal_focus = self.personal_focus.next();
    }

    pub fn personal_previous_field(&mut self) {
        self.personal_focus = self.personal_focus.previous();
    }

    pub fn personal_cycle_gender(&mut self, step: i32) {
        use amlich_core::almanac::tu_menh::Gender;

        let current = match self.personal_draft.gender {
            None => {
                if step >= 0 {
                    0
                } else {
                    1
                }
            }
            Some(Gender::Male) => {
                if step >= 0 {
                    1
                } else {
                    0
                }
            }
            Some(Gender::Female) => {
                if step >= 0 {
                    0
                } else {
                    1
                }
            }
        };

        self.personal_draft.gender = Some(match current {
            0 => Gender::Male,
            _ => Gender::Female,
        });
    }

    pub fn apply_personal_profile(&mut self) {
        let Ok(birth_year) = self.personal_draft.birth_year.parse::<i32>() else {
            self.error_msg = Some("Năm sinh không hợp lệ. Hãy nhập 4 chữ số.".to_string());
            self.app_mode = AppMode::Normal;
            return;
        };
        let birth_month = self.personal_draft.birth_month.parse::<i32>().ok();
        let birth_day = self.personal_draft.birth_day.parse::<i32>().ok();
        let birth_hour = self.personal_draft.birth_hour.parse::<u8>().ok();
        let birth_minute = self.personal_draft.birth_minute.parse::<u8>().ok();

        let Some(gender) = self.personal_draft.gender else {
            self.error_msg = Some("Hãy chọn giới tính để mở lớp cá nhân hóa.".to_string());
            self.app_mode = AppMode::Normal;
            return;
        };

        let query = amlich_api::DateQuery {
            day: self.date.day() as i32,
            month: self.date.month() as i32,
            year: self.date.year(),
            timezone: None,
            ruleset_id: self.applied_selection.ruleset_id.clone(),
            event_kind: self.applied_selection.event_kind.clone(),
            enabled_pack_ids: self.applied_selection.enabled_pack_ids.clone(),
        };

        match amlich_api::v2::get_insight_with_profile(
            &query,
            Some(birth_year),
            birth_month,
            birth_day,
            Some(gender),
        ) {
            Ok(insight) => {
                if let Some(bundle) = self.bundle.as_mut() {
                    bundle.insight = Some(insight);
                }
                save_persisted_profile(&self.personal_draft);
                if birth_hour.is_none() {
                    self.personal_draft.birth_hour.clear();
                }
                if birth_minute.is_none() {
                    self.personal_draft.birth_minute.clear();
                }
                self.personal_matrix = self.load_personal_matrix();
                self.error_msg = None;
            }
            Err(err) => {
                self.error_msg = Some(err);
            }
        }

        self.app_mode = AppMode::Normal;
    }

    fn load_personal_matrix(&self) -> Option<amlich_api::PersonalDayMatrixReportDto> {
        let birth_year = self.personal_draft.birth_year.parse::<i32>().ok()?;
        let birth_month = self.personal_draft.birth_month.parse::<i32>().ok()?;
        let birth_day = self.personal_draft.birth_day.parse::<i32>().ok()?;
        let birth_hour = self.personal_draft.birth_hour.parse::<u8>().ok().unwrap_or(0);
        let birth_minute = self
            .personal_draft
            .birth_minute
            .parse::<u8>()
            .ok()
            .unwrap_or(0);
        let gender = self.personal_draft.gender?;

        let query = amlich_api::DateQuery {
            day: self.date.day() as i32,
            month: self.date.month() as i32,
            year: self.date.year(),
            timezone: None,
            ruleset_id: self.applied_selection.ruleset_id.clone(),
            event_kind: self.applied_selection.event_kind.clone(),
            enabled_pack_ids: self.applied_selection.enabled_pack_ids.clone(),
        };
        let birth = amlich_api::BaziQuery {
            day: birth_day,
            month: birth_month,
            year: birth_year,
            hour: birth_hour,
            minute: birth_minute,
            timezone: None,
            longitude: None,
            use_solar_time: false,
            gender: Some(match gender {
                amlich_core::almanac::tu_menh::Gender::Male => "male".to_string(),
                amlich_core::almanac::tu_menh::Gender::Female => "female".to_string(),
            }),
        };

        amlich_api::get_personal_day_matrix_report(&birth, &query).ok()
    }

    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
        self.clamp_scroll();
    }

    pub fn scroll_down_by(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_add(lines);
        self.clamp_scroll();
    }

    pub fn scroll_up_by(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    pub fn clamp_scroll(&mut self) {
        let max = self.content_height.saturating_sub(self.viewport_height);
        self.scroll_offset = self.scroll_offset.min(max);
    }

    fn set_section_expanded(&mut self, section: PageSection, expanded: bool) {
        if expanded {
            self.expanded_sections.insert(section);
        } else {
            self.expanded_sections.remove(&section);
        }

        match section {
            PageSection::Recommendations => self.show_guidance_details = expanded,
            PageSection::TraditionalEvidence => self.show_tietkhi_details = expanded,
            _ => {}
        }
    }

    fn apply_navigated_date(&mut self, date: NaiveDate) {
        self.date = date;
        self.applied_selection.date = date;
        self.staged_selection.date = date;
        self.calendar_cursor = date;
        self.load_data();
    }
}

fn profile_path() -> Option<PathBuf> {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(dirs::config_dir)
        .map(|dir| dir.join("amlich").join("profile.json"))
}

fn load_persisted_profile() -> PersistedUserProfile {
    let Some(path) = profile_path() else {
        return PersistedUserProfile::default();
    };
    match std::fs::read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => PersistedUserProfile::default(),
    }
}

fn save_persisted_profile(draft: &PersonalDraft) {
    let Some(path) = profile_path() else {
        return;
    };
    let Some(parent) = path.parent() else {
        return;
    };
    if std::fs::create_dir_all(parent).is_err() {
        return;
    }

    let profile = PersistedUserProfile {
        birth_year: draft.birth_year.parse::<i32>().ok(),
        birth_month: draft.birth_month.parse::<i32>().ok(),
        birth_day: draft.birth_day.parse::<i32>().ok(),
        birth_hour: draft.birth_hour.parse::<u8>().ok(),
        birth_minute: draft.birth_minute.parse::<u8>().ok(),
        gender: draft.gender.map(|g| match g {
            amlich_core::almanac::tu_menh::Gender::Male => PersistedProfileGender::Male,
            amlich_core::almanac::tu_menh::Gender::Female => PersistedProfileGender::Female,
        }),
    };

    if let Ok(json) = serde_json::to_string_pretty(&profile) {
        let _ = std::fs::write(path, json);
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

fn wrap_index(current: usize, len: usize, step: i32) -> usize {
    if len == 0 {
        return 0;
    }

    (((current as i32) + step).rem_euclid(len as i32)) as usize
}

fn normalize_ruleset_selection(
    ruleset_catalog: &[RulesetCatalogEntryDto],
    ruleset_id: Option<&str>,
) -> Option<String> {
    let requested = ruleset_id.map(str::trim).filter(|id| !id.is_empty())?;
    ruleset_catalog
        .iter()
        .find(|entry| {
            entry.canonical_id == requested
                || entry.id == requested
                || entry.aliases.iter().any(|alias| alias == requested)
        })
        .map(|entry| entry.canonical_id.clone())
}

fn normalize_enabled_pack_selection(
    recommendation_pack_catalog: &[RecommendationPackCatalogEntryDto],
    enabled_pack_ids: &[String],
) -> Vec<String> {
    let mut normalized = Vec::new();
    for pack_id in enabled_pack_ids {
        let trimmed = pack_id.trim();
        if trimmed.is_empty() {
            continue;
        }

        if recommendation_pack_catalog
            .iter()
            .any(|entry| entry.pack_id == trimmed)
            && !normalized.iter().any(|existing| existing == trimmed)
        {
            normalized.push(trimmed.to_string());
        }
    }
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use amlich_api::v2::DayBundleDto;
    use amlich_api::{
        ActivityLabelDto, CanChiDto, CanChiInfoDto, DailyRecommendationsDto, DayConflictDto,
        DayElementDto, DayFortuneDto, DayGuidanceDto, DayInsightDto, DayStarsDto, DayTabooDto,
        GioHoangDaoDto, HourInfoDto, HourInsightEntryDto, HoursInsightDto, LocalizedListDto,
        LocalizedTextDto, LunarDto, NaAmInsightDto, NguHanhDto, RecommendationBucketDto,
        RecommendationEvidenceDto, RecommendationEvidenceSourceDto, RecommendationReasonDto,
        RecommendationScopeDto, RecommendationSeverityDto, RuleEvidenceDto, SolarDto,
        SynthesizedRecommendationDto, TietKhiDto, TietKhiInsightDto, TravelDirectionDto,
        TravelInsightDto, TrucDto, TuMenhInsightDto, XungHopDto,
    };

    fn sample_app_state() -> AppState {
        let date = NaiveDate::from_ymd_opt(2026, 3, 12).expect("valid date");
        let ruleset_catalog = vec![RulesetCatalogEntryDto {
            id: "vn_baseline_v1".to_string(),
            canonical_id: "vn_baseline_v1".to_string(),
            version: "v1".to_string(),
            region: "vn".to_string(),
            profile: "baseline".to_string(),
            schema_version: "amlich.engine/v1".to_string(),
            is_default: true,
            aliases: vec!["default".to_string()],
            defaults: amlich_api::RulesetDefaultsDto {
                tz_offset: 7.0,
                meridian: None,
            },
            source_notes: vec![],
        }];
        let recommendation_pack_catalog = vec![RecommendationPackCatalogEntryDto {
            pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
            request_field: "enabled_pack_ids".to_string(),
            version: "v1".to_string(),
            source_family: "traditional".to_string(),
            mode: "advisory".to_string(),
        }];
        let selection = ExplorerSelection::defaults(date, &ruleset_catalog);
        AppState {
            running: true,
            date,
            scroll_offset: 0,
            content_height: 0,
            viewport_height: 0,
            bundle: None,
            personal_matrix: None,
            is_loading: false,
            error_msg: None,
            ruleset_catalog,
            recommendation_pack_catalog,
            applied_selection: selection.clone(),
            staged_selection: selection,
            explorer_focus: ExplorerField::Date,
            explorer_action: ExplorerAction::Apply,
            pack_cursor: 0,
            show_guidance_details: false,
            show_tietkhi_details: false,
            show_evidence: false,
            show_week_strip: true,
            show_graph_recommendations: false,
            verbosity: crate::state::ui_prefs::VerbosityMode::Compact,
            active_view: ActiveView::Today,
            view_history: Vec::new(),
            app_mode: AppMode::Normal,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            search_input: String::new(),
            personal_focus: PersonalField::BirthYear,
            personal_draft: PersonalDraft::empty(),
            calendar_cursor: date,
            navigation_history: Vec::new(),
        }
    }

    fn sample_bundle() -> DayBundleDto {
        DayBundleDto {
            schema_version: "amlich.engine/v1".to_string(),
            ruleset_id: "test".to_string(),
            ruleset_version: "v1".to_string(),
            profile: "baseline".to_string(),
            generated_at: "2026-03-12T00:00:00Z".to_string(),

            solar: SolarDto {
                day: 12,
                month: 3,
                year: 2026,
                day_of_week: 4,
                day_of_week_name: "Thứ Năm".to_string(),
                date_string: "2026-03-12".to_string(),
            },
            lunar: LunarDto {
                day: 4,
                month: 2,
                year: 2026,
                is_leap_month: false,
                date_string: "Mùng 4 tháng Hai".to_string(),
            },
            jd: 0,
            canchi: Some(CanChiInfoDto {
                day: CanChiDto {
                    can_index: 2,
                    chi_index: 6,
                    can: "Bính".to_string(),
                    chi: "Ngọ".to_string(),
                    full: "Bính Ngọ".to_string(),
                    con_giap: "Ngựa".to_string(),
                    ngu_hanh: NguHanhDto {
                        can: "Hỏa".to_string(),
                        chi: "Hỏa".to_string(),
                    },
                },
                month: CanChiDto {
                    can_index: 0,
                    chi_index: 2,
                    can: "Giáp".to_string(),
                    chi: "Dần".to_string(),
                    full: "Giáp Dần".to_string(),
                    con_giap: "Hổ".to_string(),
                    ngu_hanh: NguHanhDto {
                        can: "Mộc".to_string(),
                        chi: "Mộc".to_string(),
                    },
                },
                year: CanChiDto {
                    can_index: 2,
                    chi_index: 6,
                    can: "Bính".to_string(),
                    chi: "Ngọ".to_string(),
                    full: "Bính Ngọ".to_string(),
                    con_giap: "Ngựa".to_string(),
                    ngu_hanh: NguHanhDto {
                        can: "Hỏa".to_string(),
                        chi: "Hỏa".to_string(),
                    },
                },
                full: "Bính Ngọ".to_string(),
            }),
            tiet_khi: Some(TietKhiDto {
                index: 3,
                name: "Kinh Trập".to_string(),
                description: "Tiết khí thử nghiệm".to_string(),
                longitude: 345,
                current_longitude: 345.0,
                season: "Xuân".to_string(),
            }),
            gio_hoang_dao: Some(GioHoangDaoDto {
                day_chi: "Ngọ".to_string(),
                good_hour_count: 4,
                good_hours: vec![HourInfoDto {
                    hour_index: 0,
                    hour_chi: "Tý".to_string(),
                    time_range: "23:00 - 01:00".to_string(),
                    star: "Thanh Long".to_string(),
                    is_good: true,
                }],
                all_hours: vec![],
                summary: "Giờ đẹp đầu ngày".to_string(),
            }),
            day_fortune: Some(DayFortuneDto {
                ruleset_id: "test".to_string(),
                ruleset_version: "v1".to_string(),
                profile: "baseline".to_string(),
                day_element: DayElementDto {
                    na_am: "Thiên Hà Thủy".to_string(),
                    element: "Thủy".to_string(),
                    can_element: "Hỏa".to_string(),
                    chi_element: "Hỏa".to_string(),
                    evidence: None,
                },
                conflict: DayConflictDto {
                    opposing_chi: "Tý".to_string(),
                    opposing_con_giap: "Chuột".to_string(),
                    tuoi_xung: vec!["Canh Tý".to_string()],
                    sat_huong: "Bắc".to_string(),
                    evidence: None,
                },
                travel: TravelDirectionDto {
                    xuat_hanh_huong: "Đông Nam".to_string(),
                    tai_than: "Chính Nam".to_string(),
                    hy_than: "Đông Bắc".to_string(),
                    evidence: None,
                },
                stars: DayStarsDto {
                    cat_tinh: vec!["Thiên Đức".to_string()],
                    sat_tinh: vec!["Thiên Cương".to_string()],
                    day_star: None,
                    star_system: None,
                    evidence: None,
                    matched_rules: vec![],
                },
                day_deity: None,
                taboos: vec![DayTabooDto {
                    rule_id: "taboo.tam_nuong".to_string(),
                    name: "Tam Nương".to_string(),
                    severity: "high".to_string(),
                    reason: "Không hợp việc lớn".to_string(),
                    evidence: Some(RuleEvidenceDto {
                        source_id: "taboo_table".to_string(),
                        method: "table-lookup".to_string(),
                        profile: "baseline".to_string(),
                    }),
                }],
                xung_hop: XungHopDto {
                    luc_xung: "Tý".to_string(),
                    tam_hop: vec!["Dần".to_string(), "Tuất".to_string()],
                    tu_hanh_xung: vec!["Ngọ".to_string()],
                    liu_he: None,
                    xiang_hai: None,
                    xiang_xing: None,
                },
                truc: TrucDto {
                    index: 4,
                    name: "Khai".to_string(),
                    quality: "cat".to_string(),
                    evidence: None,
                },
                tang_can: None,
                ten_gods: None,
                tu_menh: None,
            }),
            daily_recommendations: Some(DailyRecommendationsDto {
                ruleset_id: "test".to_string(),
                ruleset_version: "v1".to_string(),
                profile: "baseline".to_string(),
                scope: RecommendationScopeDto::GeneralDay,
                version: "v1".to_string(),
                summary_vi: "Ngày thuận việc mở đầu, tránh việc lớn.".to_string(),
                summary_en: "Good for starting, avoid major matters.".to_string(),
                active_packs: vec![],
                activities: vec![
                    SynthesizedRecommendationDto {
                        activity_id: "opening_start".to_string(),
                        label: ActivityLabelDto {
                            vi: "Khai mở".to_string(),
                            en: "Opening".to_string(),
                        },
                        bucket: RecommendationBucketDto::Nen,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "truc.khai.good".to_string(),
                            severity: RecommendationSeverityDto::Primary,
                            summary_vi: "Hợp trực Khai".to_string(),
                            summary_en: "Good under Khai".to_string(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::Truc,
                                code: "truc.khai".to_string(),
                                note: "test".to_string(),
                            },
                        }],
                    },
                    SynthesizedRecommendationDto {
                        activity_id: "meet_visit".to_string(),
                        label: ActivityLabelDto {
                            vi: "Gặp gỡ".to_string(),
                            en: "Meet".to_string(),
                        },
                        bucket: RecommendationBucketDto::CoThe,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "travel.support".to_string(),
                            severity: RecommendationSeverityDto::Supporting,
                            summary_vi: "Có hướng xuất hành phù hợp".to_string(),
                            summary_en: "Travel is acceptable".to_string(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::Travel,
                                code: "travel.good".to_string(),
                                note: "test".to_string(),
                            },
                        }],
                    },
                    SynthesizedRecommendationDto {
                        activity_id: "contract_agreement".to_string(),
                        label: ActivityLabelDto {
                            vi: "Ký kết".to_string(),
                            en: "Contract".to_string(),
                        },
                        bucket: RecommendationBucketDto::Tranh,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "xung_hop.avoid".to_string(),
                            severity: RecommendationSeverityDto::Primary,
                            summary_vi: "Ngày xung".to_string(),
                            summary_en: "Clashing day".to_string(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::XungHop,
                                code: "xung_hop.bad".to_string(),
                                note: "test".to_string(),
                            },
                        }],
                    },
                    SynthesizedRecommendationDto {
                        activity_id: "groundbreaking".to_string(),
                        label: ActivityLabelDto {
                            vi: "Động thổ".to_string(),
                            en: "Groundbreaking".to_string(),
                        },
                        bucket: RecommendationBucketDto::KyManh,
                        reasons: vec![RecommendationReasonDto {
                            rule_id: "taboo.tam_nuong".to_string(),
                            severity: RecommendationSeverityDto::Override,
                            summary_vi: "Tam Nương kỵ việc động thổ".to_string(),
                            summary_en: "Tam Nuong strongly forbids groundbreaking".to_string(),
                            evidence: RecommendationEvidenceDto {
                                source: RecommendationEvidenceSourceDto::Taboo,
                                code: "taboo.tam_nuong".to_string(),
                                note: "test".to_string(),
                            },
                        }],
                    },
                ],
            }),
            contextual_recommendations: None,
            insight: Some(DayInsightDto {
                solar: SolarDto {
                    day: 12,
                    month: 3,
                    year: 2026,
                    day_of_week: 4,
                    day_of_week_name: "Thứ Năm".to_string(),
                    date_string: "2026-03-12".to_string(),
                },
                lunar: LunarDto {
                    day: 4,
                    month: 2,
                    year: 2026,
                    is_leap_month: false,
                    date_string: "Mùng 4 tháng Hai".to_string(),
                },
                festival: None,
                holiday: None,
                canchi: None,
                day_guidance: None,
                tiet_khi: None,
                na_am: None,
                truc: None,
                day_deity: None,
                stars: None,
                taboos: None,
                travel: None,
                xung_hop: None,
                tang_can: None,
                ten_gods: None,
                hours: Some(HoursInsightDto {
                    good_hour_count: 2,
                    good_hours: vec![
                        HourInsightEntryDto {
                            chi: "Mão".to_string(),
                            time_range: "05:00 - 07:00".to_string(),
                            star: "Minh Đường".to_string(),
                        },
                        HourInsightEntryDto {
                            chi: "Tỵ".to_string(),
                            time_range: "09:00 - 11:00".to_string(),
                            star: "Kim Quỹ".to_string(),
                        },
                    ],
                }),
                tu_menh: None,
                dai_van: None,
                yearly_han: None,
            }),
            upcoming_events: vec![],
        }
    }

    fn sample_app_state_with_bundle() -> AppState {
        let mut app = sample_app_state();
        app.bundle = Some(sample_bundle());
        app
    }

    #[test]
    fn section_focus_cycles_in_order() {
        let mut app = sample_app_state();
        app.focused_section = PageSection::Explorer;
        let expected = [
            PageSection::Hero,
            PageSection::Recommendations,
            PageSection::Timing,
            PageSection::Travel,
            PageSection::Risks,
            PageSection::TraditionalEvidence,
            PageSection::ExpandedDetails,
            PageSection::Explorer,
        ];

        for section in expected {
            app.focus_next_section();
            assert_eq!(app.focused_section, section);
        }
    }

    #[test]
    fn evidence_toggle_changes_visibility_flag() {
        let mut app = sample_app_state();

        assert!(!app.show_evidence);
        app.toggle_evidence();
        assert!(app.show_evidence);
        app.toggle_evidence();
        assert!(!app.show_evidence);
    }

    #[test]
    fn zoom_mode_tracks_focused_section() {
        let mut app = sample_app_state();

        app.focused_section = PageSection::Travel;
        app.toggle_zoom_for_focused_section();
        assert_eq!(app.zoomed_section, Some(PageSection::Travel));

        app.focused_section = PageSection::Risks;
        app.toggle_zoom_for_focused_section();
        assert_eq!(app.zoomed_section, Some(PageSection::Risks));

        app.toggle_zoom_for_focused_section();
        assert_eq!(app.zoomed_section, None);
    }

    #[test]
    fn expand_toggle_is_scoped_to_focused_section() {
        let mut app = sample_app_state();

        app.focused_section = PageSection::Recommendations;
        app.toggle_expand_focused_section();
        assert!(app.is_section_expanded(PageSection::Recommendations));
        assert!(!app.is_section_expanded(PageSection::TraditionalEvidence));

        app.focused_section = PageSection::TraditionalEvidence;
        app.toggle_expand_focused_section();
        assert!(app.is_section_expanded(PageSection::Recommendations));
        assert!(app.is_section_expanded(PageSection::TraditionalEvidence));

        app.toggle_expand_focused_section();
        assert!(app.is_section_expanded(PageSection::Recommendations));
        assert!(!app.is_section_expanded(PageSection::TraditionalEvidence));
    }

    #[test]
    fn top_recommendation_rows_follow_bucket_order() {
        let app = sample_app_state_with_bundle();

        let rows = app.top_recommendation_rows();

        assert_eq!(
            rows.iter()
                .map(|row| row.label.as_str())
                .collect::<Vec<_>>(),
            vec!["Khai mở", "Gặp gỡ", "Ký kết", "Động thổ"]
        );
        assert_eq!(
            rows.iter().map(|row| row.bucket).collect::<Vec<_>>(),
            vec![
                RecommendationBucketDto::Nen,
                RecommendationBucketDto::CoThe,
                RecommendationBucketDto::Tranh,
                RecommendationBucketDto::KyManh,
            ]
        );
        assert!(rows[0]
            .reason_details
            .iter()
            .any(|detail| detail.contains("Hợp trực Khai")));
    }

    #[test]
    fn hero_verdict_prefers_summary_and_strongest_rows() {
        let app = sample_app_state_with_bundle();

        let verdict = app.hero_verdict().expect("hero verdict");

        assert_eq!(verdict.summary, "Ngày thuận việc mở đầu, tránh việc lớn.");
        assert_eq!(verdict.strongest_positive.as_deref(), Some("Khai mở"));
        assert_eq!(verdict.strongest_negative.as_deref(), Some("Động thổ"));
    }

    #[test]
    fn risk_summary_surfaces_ky_manh_and_taboos_first() {
        let app = sample_app_state_with_bundle();

        let risk_summary = app.risk_summary();

        assert_eq!(risk_summary.items[0], "Kỵ mạnh: Động thổ");
        assert_eq!(risk_summary.items[1], "Kiêng kỵ: Tam Nương");
        assert!(
            risk_summary
                .items
                .iter()
                .any(|item| item.contains("Lục xung: Tý")),
            "expected luc xung entry in risk summary"
        );
    }

    #[test]
    fn day_detail_timing_summary_prefers_curated_windows_and_existing_gio_summary() {
        let app = sample_app_state_with_bundle();

        let timing = app.day_detail_timing_summary().expect("timing summary");

        assert_eq!(timing.summary, "Giờ đẹp đầu ngày");
        assert_eq!(
            timing.windows,
            vec![
                "Mão 05:00 - 07:00 · Minh Đường",
                "Tỵ 09:00 - 11:00 · Kim Quỹ",
                "Tý 23:00 - 01:00 · Thanh Long",
            ]
        );
    }

    #[test]
    fn day_detail_risk_board_groups_critical_caution_and_conflict_rows() {
        let app = sample_app_state_with_bundle();

        let risk_board = app.day_detail_risk_board();

        assert_eq!(risk_board.headline.as_deref(), Some("Kỵ mạnh: Động thổ"));
        assert!(risk_board
            .critical_items
            .iter()
            .any(|item| item == "Kỵ mạnh: Động thổ"));
        assert!(risk_board
            .critical_items
            .iter()
            .any(|item| item == "Kiêng kỵ: Tam Nương · Không hợp việc lớn"));
        assert!(risk_board
            .caution_items
            .iter()
            .any(|item| item == "Tránh: Ký kết · primary • xung-hợp"));
        assert!(risk_board
            .conflict_items
            .iter()
            .any(|item| item == "Tuổi xung: Canh Tý"));
        assert!(risk_board
            .conflict_items
            .iter()
            .any(|item| item == "Sát hướng: Bắc"));
    }

    #[test]
    fn day_detail_verdict_support_combines_traditional_evidence_and_active_layer_note() {
        let mut app = sample_app_state_with_bundle();
        if let Some(bundle) = app.bundle.as_mut() {
            let mut contextual = bundle
                .daily_recommendations
                .clone()
                .expect("baseline recommendations");
            contextual.profile = "contract_signing".to_string();
            contextual.summary_vi = "Ưu tiên ngữ cảnh ký kết".to_string();
            bundle.contextual_recommendations = Some(contextual);
        }

        let support = app
            .day_detail_verdict_support()
            .expect("day detail verdict support");

        assert_eq!(
            support.support_line,
            "Bính Ngọ · Trực Khai · Cát tinh Thiên Đức · Kỵ mạnh: Động thổ"
        );
        assert_eq!(
            support.layer_note.as_deref(),
            Some("Ngữ cảnh ưu tiên: Ưu tiên ngữ cảnh ký kết")
        );
    }

    #[test]
    fn direction_verdict_uses_day_level_travel_and_deity_context() {
        let mut app = sample_app_state_with_bundle();
        let insight = app
            .bundle
            .as_mut()
            .and_then(|bundle| bundle.insight.as_mut())
            .expect("insight");
        insight.travel = Some(TravelInsightDto {
            xuat_hanh_huong: "Nam".to_string(),
            tai_than: "Tây Nam".to_string(),
            hy_than: "Đông".to_string(),
        });
        insight.day_deity = Some(amlich_api::DayDeityInsightDto {
            name: "Kim Quỹ".to_string(),
            classification: "hoang_dao".to_string(),
            classification_meaning: LocalizedTextDto {
                vi: "Cát thần".to_string(),
                en: "Good deity".to_string(),
            },
            deity_meaning: Some(LocalizedTextDto {
                vi: "Hợp cho việc cần sự bảo chứng.".to_string(),
                en: "Good for protected actions.".to_string(),
            }),
        });

        let verdict = app.direction_verdict().expect("direction verdict");
        assert!(verdict.summary.contains("Nam"));
        assert!(verdict
            .directions
            .iter()
            .any(|item| item.contains("Hỷ Thần")));
        assert!(verdict
            .deity_context
            .as_deref()
            .unwrap_or_default()
            .contains("Kim Quỹ"));
        assert!(verdict.matrix_note.is_none());
    }

    #[test]
    fn direction_verdict_surfaces_personalized_matrix_note_when_profile_overlay_exists() {
        let mut app = sample_app_state_with_bundle();
        let insight = app
            .bundle
            .as_mut()
            .and_then(|bundle| bundle.insight.as_mut())
            .expect("insight");
        insight.tu_menh = Some(TuMenhInsightDto {
            kua: 3,
            group: "Đông tứ mệnh".to_string(),
            trigram: LocalizedTextDto {
                vi: "Chấn".to_string(),
                en: "Zhen".to_string(),
            },
            direction: LocalizedTextDto {
                vi: "Đông".to_string(),
                en: "East".to_string(),
            },
            meaning: LocalizedTextDto {
                vi: "Hợp hướng mở lối và khởi động.".to_string(),
                en: "Favors opening movement.".to_string(),
            },
            group_meaning: LocalizedTextDto {
                vi: "Nhóm hướng tăng trưởng".to_string(),
                en: "Growth group".to_string(),
            },
            favorable_directions: vec!["Đông".to_string()],
            unfavorable_directions: vec!["Tây".to_string()],
        });

        let verdict = app.direction_verdict().expect("direction verdict");
        assert!(verdict.matrix_note.is_some());
    }

    #[test]
    fn apply_personal_profile_populates_personal_matrix_cache() {
        let mut app = sample_app_state_with_bundle();
        app.personal_draft.birth_year = "1990".to_string();
        app.personal_draft.birth_month = "1".to_string();
        app.personal_draft.birth_day = "1".to_string();
        app.personal_draft.birth_hour = "9".to_string();
        app.personal_draft.birth_minute = "30".to_string();
        app.personal_draft.gender = Some(amlich_core::almanac::tu_menh::Gender::Male);

        app.apply_personal_profile();

        assert!(app.personal_matrix.is_some());
        let matrix = app.personal_matrix.as_ref().expect("matrix");
        assert!(matrix.direction_merge.is_some());
        assert!(matrix.personal_hours.is_some());
    }

    #[test]
    fn personal_draft_from_persisted_profile_keeps_birth_time() {
        let profile = PersistedUserProfile {
            birth_year: Some(1990),
            birth_month: Some(1),
            birth_day: Some(1),
            birth_hour: Some(9),
            birth_minute: Some(30),
            gender: Some(PersistedProfileGender::Male),
        };

        let draft = PersonalDraft::from_persisted(&profile);
        assert_eq!(draft.birth_hour, "9");
        assert_eq!(draft.birth_minute, "30");
        assert_eq!(draft.gender, Some(amlich_core::almanac::tu_menh::Gender::Male));
    }

    #[test]
    fn day_identity_summary_combines_canchi_element_and_guidance() {
        let mut app = sample_app_state_with_bundle();
        let insight = app
            .bundle
            .as_mut()
            .and_then(|bundle| bundle.insight.as_mut())
            .expect("insight");
        insight.na_am = Some(NaAmInsightDto {
            na_am: "Thiên Hà Thủy".to_string(),
            element: "Thủy".to_string(),
            meaning: LocalizedTextDto {
                vi: "Dòng nước trời cao, thiên về thanh lọc.".to_string(),
                en: "High celestial water.".to_string(),
            },
        });
        insight.day_guidance = Some(DayGuidanceDto {
            good_for: LocalizedListDto {
                vi: vec!["khởi sự nhẹ".to_string()],
                en: vec!["light beginnings".to_string()],
            },
            avoid_for: LocalizedListDto {
                vi: vec![],
                en: vec![],
            },
        });

        let summary = app.day_identity_summary().expect("day identity");
        assert!(summary.headline.contains("Bính Ngọ"));
        assert!(summary.headline.contains("Thiên Hà Thủy"));
        assert!(summary
            .application_note
            .as_deref()
            .unwrap_or_default()
            .contains("khởi sự nhẹ"));
    }

    #[test]
    fn seasonal_verdict_uses_insight_meaning_weather_and_health() {
        let mut app = sample_app_state_with_bundle();
        let insight = app
            .bundle
            .as_mut()
            .and_then(|bundle| bundle.insight.as_mut())
            .expect("insight");
        insight.tiet_khi = Some(TietKhiInsightDto {
            id: "kinh_trap".to_string(),
            name: LocalizedTextDto {
                vi: "Kinh Trập".to_string(),
                en: "Awakening of Insects".to_string(),
            },
            longitude: 345,
            meaning: LocalizedTextDto {
                vi: "Thời khí chuyển động, hợp sắp việc theo nhịp mới.".to_string(),
                en: "A moving seasonal phase.".to_string(),
            },
            astronomy: LocalizedTextDto {
                vi: "Kinh độ xuân tăng dần.".to_string(),
                en: "Spring longitude increases.".to_string(),
            },
            agriculture: LocalizedListDto {
                vi: vec!["Sắp việc ngoài trời theo nhịp ấm lên".to_string()],
                en: vec![],
            },
            health: LocalizedListDto {
                vi: vec!["Giữ nhịp ngủ nghỉ ổn định".to_string()],
                en: vec![],
            },
            weather: LocalizedTextDto {
                vi: "Không khí ấm và linh hoạt hơn.".to_string(),
                en: "Weather becomes milder.".to_string(),
            },
        });

        let verdict = app.seasonal_verdict().expect("seasonal verdict");
        assert!(verdict.headline.contains("Kinh Trập"));
        assert!(verdict.implication.contains("Thời khí chuyển động"));
        assert!(verdict
            .application_lines
            .iter()
            .any(|line| line.contains("Thời khí")));
        assert!(verdict
            .application_lines
            .iter()
            .any(|line| line.contains("Giữ nhịp")));
    }

    #[test]
    fn profile_availability_summary_distinguishes_general_and_personal_modes() {
        let mut app = sample_app_state_with_bundle();
        let without_profile = app
            .profile_availability_summary()
            .expect("profile availability");
        assert!(!without_profile.has_personal_overlay);

        let insight = app
            .bundle
            .as_mut()
            .and_then(|bundle| bundle.insight.as_mut())
            .expect("insight");
        insight.tu_menh = Some(TuMenhInsightDto {
            kua: 3,
            group: "Đông tứ mệnh".to_string(),
            trigram: LocalizedTextDto {
                vi: "Chấn".to_string(),
                en: "Zhen".to_string(),
            },
            direction: LocalizedTextDto {
                vi: "Đông".to_string(),
                en: "East".to_string(),
            },
            meaning: LocalizedTextDto {
                vi: "Hợp hướng mở lối và khởi động.".to_string(),
                en: "Favors opening movement.".to_string(),
            },
            group_meaning: LocalizedTextDto {
                vi: "Nhóm hướng tăng trưởng".to_string(),
                en: "Growth group".to_string(),
            },
            favorable_directions: vec!["Đông".to_string()],
            unfavorable_directions: vec!["Tây".to_string()],
        });

        let with_profile = app
            .profile_availability_summary()
            .expect("profile availability");
        assert!(with_profile.has_personal_overlay);
        assert!(with_profile.note.contains("cá nhân hóa"));
    }

    #[test]
    fn staged_date_changes_do_not_mutate_applied_selection_until_apply() {
        let mut app = sample_app_state();
        let original = app.applied_selection.date;

        app.next_day();

        assert_ne!(app.staged_selection.date, original);
        assert_eq!(app.applied_selection.date, original);
        assert!(app.explorer_has_staged_changes());
    }

    #[test]
    fn date_navigation_applies_immediately_and_keeps_focus_context() {
        let mut app = sample_app_state();
        app.focus_section(PageSection::Risks);
        app.scroll_offset = 7;

        app.navigate_days(1);

        assert_eq!(app.focused_section, PageSection::Risks);
        assert_eq!(app.scroll_offset, 7);
        assert!(app.bundle.is_some());
    }

    #[test]
    fn undo_navigation_returns_to_previous_date() {
        let mut app = sample_app_state();
        let start = app.date;
        app.navigate_days(1);
        app.navigate_days(1);

        app.undo_navigation();
        assert_ne!(app.date, start);

        app.undo_navigation();
        assert_eq!(app.date, start);
    }

    #[test]
    fn apply_calendar_selection_keeps_scroll_and_focus() {
        let mut app = sample_app_state();
        app.focus_section(PageSection::TraditionalEvidence);
        app.scroll_offset = 0; // go_to_view resets it, so we don't expect it to be 9
        app.open_calendar_view();
        app.calendar_move_days(14);

        app.apply_calendar_selection();

        assert_eq!(app.focused_section, PageSection::TraditionalEvidence);
        assert!(!app.is_calendar_view());
    }

    #[test]
    fn loaded_selection_preserves_selected_event_kind_and_active_packs() {
        let mut app = sample_app_state();
        app.applied_selection.event_kind = Some("travel".to_string());
        app.applied_selection.enabled_pack_ids = vec!["pack.nhi_thap_bat_tu.v1".to_string()];

        let selection =
            ExplorerSelection::from_loaded_data(&sample_bundle(), &app.applied_selection);

        assert_eq!(selection.event_kind.as_deref(), Some("travel"));
        assert_eq!(selection.enabled_pack_ids, vec!["pack.nhi_thap_bat_tu.v1"]);
    }

    #[test]
    fn selection_normalization_maps_aliases_to_canonical_identity() {
        let app = sample_app_state();
        let selection = ExplorerSelection {
            date: app.date,
            ruleset_id: Some("default".to_string()),
            event_kind: Some("travel".to_string()),
            enabled_pack_ids: vec![
                "pack.nhi_thap_bat_tu.v1".to_string(),
                "pack.nhi_thap_bat_tu.v1".to_string(),
                "unknown.pack".to_string(),
            ],
        }
        .normalized(&app.ruleset_catalog, &app.recommendation_pack_catalog);

        assert_eq!(selection.ruleset_id.as_deref(), Some("vn_baseline_v1"));
        assert_eq!(selection.enabled_pack_ids, vec!["pack.nhi_thap_bat_tu.v1"]);
        assert_eq!(selection.event_kind.as_deref(), Some("travel"));
    }

    #[test]
    fn apply_staged_selection_normalizes_alias_before_loading() {
        let mut app = sample_app_state();
        app.staged_selection.ruleset_id = Some("default".to_string());
        app.staged_selection.enabled_pack_ids = vec![
            "pack.nhi_thap_bat_tu.v1".to_string(),
            "pack.nhi_thap_bat_tu.v1".to_string(),
        ];

        app.apply_staged_selection();

        assert_eq!(
            app.applied_selection.ruleset_id.as_deref(),
            Some("vn_baseline_v1")
        );
        assert_eq!(
            app.staged_selection.ruleset_id.as_deref(),
            Some("vn_baseline_v1")
        );
        assert_eq!(
            app.applied_selection.enabled_pack_ids,
            vec!["pack.nhi_thap_bat_tu.v1"]
        );
    }

    #[test]
    fn active_bundle_packs_expose_runtime_provenance_fields() {
        let mut app = sample_app_state_with_bundle();
        if let Some(bundle) = app.bundle.as_mut() {
            if let Some(recommendations) = bundle.daily_recommendations.as_mut() {
                recommendations.active_packs = vec![amlich_api::ActiveRecommendationPackDto {
                    pack_id: "pack.nhi_thap_bat_tu.v1".to_string(),
                    version: "v1".to_string(),
                    source_family: "traditional".to_string(),
                    mode: "advisory".to_string(),
                }];
            }
        }

        let packs = app.active_bundle_packs();

        assert_eq!(packs.len(), 1);
        assert_eq!(packs[0].pack_id, "pack.nhi_thap_bat_tu.v1");
        assert_eq!(packs[0].source_family, "traditional");
        assert_eq!(packs[0].mode, "advisory");
    }

    #[test]
    fn evidence_toggle_does_not_mutate_recommendation_membership_or_order() {
        let mut app = sample_app_state_with_bundle();

        let before = app
            .top_recommendation_rows()
            .into_iter()
            .map(|row| row.label)
            .collect::<Vec<_>>();

        app.toggle_evidence();

        let after = app
            .top_recommendation_rows()
            .into_iter()
            .map(|row| row.label)
            .collect::<Vec<_>>();

        assert_eq!(before, after);
    }

    #[test]
    fn inspection_context_summary_reflects_active_event_kind() {
        let mut app = sample_app_state();
        app.applied_selection.event_kind = Some("contract_signing".to_string());

        assert!(app
            .inspection_context_summary()
            .contains("contract_signing · Ký kết"));
    }

    #[test]
    fn recommendation_layers_prioritize_contextual_and_keep_baseline_visible() {
        let mut app = sample_app_state_with_bundle();
        if let Some(bundle) = app.bundle.as_mut() {
            let mut contextual = bundle
                .daily_recommendations
                .clone()
                .expect("baseline recommendations");
            contextual.profile = "contract_signing".to_string();
            contextual.summary_vi = "Ưu tiên ngữ cảnh ký kết".to_string();
            contextual.active_packs = vec![amlich_api::ActiveRecommendationPackDto {
                pack_id: "pack.contract.v1".to_string(),
                version: "v1".to_string(),
                source_family: "contract".to_string(),
                mode: "advisory".to_string(),
            }];
            bundle.contextual_recommendations = Some(contextual);
        }

        let layers = app.recommendation_layers();

        assert_eq!(layers.len(), 2);
        assert_eq!(layers[0].kind, RecommendationLayerKind::Contextual);
        assert_eq!(layers[0].label, "Đang áp dụng");
        assert_eq!(layers[0].profile, "contract_signing");
        assert_eq!(layers[0].active_pack_ids, vec!["pack.contract.v1"]);
        assert_eq!(layers[1].kind, RecommendationLayerKind::Baseline);
        assert_eq!(layers[1].label, "Nền tham chiếu");
        assert_eq!(layers[1].profile, "baseline");
    }

    #[test]
    fn pack_toggle_is_stateful_and_reset_clears_stale_configuration() {
        let mut app = sample_app_state();

        app.toggle_focused_pack();
        assert_eq!(
            app.staged_selection.enabled_pack_ids,
            vec!["pack.nhi_thap_bat_tu.v1"]
        );
        assert!(app.explorer_has_staged_changes());

        app.reset_staged_selection();
        assert!(app.staged_selection.enabled_pack_ids.is_empty());
        assert_eq!(
            app.staged_selection.ruleset_id,
            Some("vn_baseline_v1".to_string())
        );
        assert_eq!(app.staged_selection.event_kind, None);
    }
}
