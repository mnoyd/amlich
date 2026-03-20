use std::collections::BTreeSet;

use amlich_api::v2::{DayBundleDto, Include};
use amlich_api::{
    DailyRecommendationsDto, RecommendationBucketDto, RecommendationEvidenceSourceDto,
    RecommendationPackCatalogEntryDto, RecommendationSeverityDto, RulesetCatalogEntryDto,
};
use chrono::{Datelike, Local, NaiveDate};

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
pub enum ActiveView {
    Dashboard,
    Event,
    Scholar,
    Hours,
    Elements,
    FengShui,
    SolarTerms,
    Planning,
    Calendar,
}

impl ActiveView {
    pub fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dashboard",
            Self::Event => "Event",
            Self::Scholar => "Scholar",
            Self::Hours => "Giờ Tốt",
            Self::Elements => "Ngũ Hành",
            Self::FengShui => "Phong Thủy",
            Self::SolarTerms => "Tiết Khí",
            Self::Planning => "Planning",
            Self::Calendar => "Calendar",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            Self::Dashboard => "Dash",
            Self::Event => "Evt",
            Self::Scholar => "Sch",
            Self::Hours => "Giờ",
            Self::Elements => "NHành",
            Self::FengShui => "PThủy",
            Self::SolarTerms => "TKhí",
            Self::Planning => "Plan",
            Self::Calendar => "Cal",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    SearchModal,
    ContextModal,
    HelpModal,
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
pub struct ScholarTimingSummaryVm {
    pub summary: String,
    pub windows: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScholarRiskBoardVm {
    pub headline: Option<String>,
    pub critical_items: Vec<String>,
    pub caution_items: Vec<String>,
    pub conflict_items: Vec<String>,
    pub notice: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScholarVerdictSupportVm {
    pub support_line: String,
    pub layer_note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectionVerdictVm {
    pub summary: String,
    pub directions: Vec<String>,
    pub deity_context: Option<String>,
    pub note: Option<String>,
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

pub struct AppState {
    pub running: bool,
    pub date: NaiveDate,
    pub lens: FocusLens,
    pub scroll_offset: u16,

    // Data cache for the current date
    pub bundle: Option<DayBundleDto>,
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
    pub active_view: ActiveView,
    pub view_history: Vec<ActiveView>,
    pub app_mode: AppMode,
    pub focused_section: PageSection,
    pub zoomed_section: Option<PageSection>,
    pub expanded_sections: BTreeSet<PageSection>,
    pub search_input: String,
    pub calendar_cursor: NaiveDate,
    pub(crate) navigation_history: Vec<NaiveDate>,
}

impl AppState {
    pub fn new(initial_date: Option<NaiveDate>) -> Self {
        let date = initial_date.unwrap_or_else(|| Local::now().naive_local().date());
        let ruleset_catalog = amlich_api::get_ruleset_catalog();
        let recommendation_pack_catalog = amlich_api::get_recommendation_pack_catalog();
        let default_selection = ExplorerSelection::defaults(date, &ruleset_catalog)
            .normalized(&ruleset_catalog, &recommendation_pack_catalog);

        let mut app = Self {
            running: true,
            date,
            lens: FocusLens::General,
            scroll_offset: 0,
            bundle: None,
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
            active_view: ActiveView::Dashboard,
            view_history: Vec::new(),
            app_mode: AppMode::Normal,
            focused_section: PageSection::Explorer,
            zoomed_section: None,
            expanded_sections: BTreeSet::new(),
            search_input: String::new(),
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
                self.applied_selection = selection.clone();
                self.staged_selection = selection;
                self.pack_cursor = self.clamp_pack_cursor();
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

    pub fn next_lens(&mut self) {
        self.lens = self.lens.next();
        self.scroll_offset = 0; // Reset scroll on lens change
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
        let mut views = vec![ActiveView::Dashboard];
        if self.has_event_today() {
            views.push(ActiveView::Event);
        }
        views.extend(vec![
            ActiveView::Scholar,
            ActiveView::Hours,
            ActiveView::Elements,
            ActiveView::FengShui,
            ActiveView::SolarTerms,
            ActiveView::Planning,
            ActiveView::Calendar,
        ]);
        views
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

    pub fn toggle_calendar(&mut self) {
        self.toggle_calendar_view();
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
                self.active_view = ActiveView::Dashboard;
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

    fn selected_recommendations(&self) -> Option<&amlich_api::DailyRecommendationsDto> {
        let bundle = self.bundle.as_ref()?;
        bundle
            .contextual_recommendations
            .as_ref()
            .or(bundle.daily_recommendations.as_ref())
    }

    pub fn recommendation_layers(&self) -> Vec<RecommendationLayerVm> {
        let Some(bundle) = self.bundle.as_ref() else {
            return Vec::new();
        };

        let mut layers = Vec::new();

        if let Some(contextual) = bundle.contextual_recommendations.as_ref() {
            layers.push(RecommendationLayerVm {
                kind: RecommendationLayerKind::Contextual,
                label: "Đang áp dụng".to_string(),
                summary: contextual.summary_vi.clone(),
                scope_label: recommendation_scope_label(contextual.scope).to_string(),
                ruleset_id: contextual.ruleset_id.clone(),
                ruleset_version: contextual.ruleset_version.clone(),
                profile: contextual.profile.clone(),
                active_pack_ids: contextual
                    .active_packs
                    .iter()
                    .map(|pack| pack.pack_id.clone())
                    .collect(),
            });
        }

        if let Some(baseline) = bundle.daily_recommendations.as_ref() {
            layers.push(RecommendationLayerVm {
                kind: RecommendationLayerKind::Baseline,
                label: if bundle.contextual_recommendations.is_some() {
                    "Nền tham chiếu".to_string()
                } else {
                    "Đang áp dụng".to_string()
                },
                summary: baseline.summary_vi.clone(),
                scope_label: recommendation_scope_label(baseline.scope).to_string(),
                ruleset_id: baseline.ruleset_id.clone(),
                ruleset_version: baseline.ruleset_version.clone(),
                profile: baseline.profile.clone(),
                active_pack_ids: baseline
                    .active_packs
                    .iter()
                    .map(|pack| pack.pack_id.clone())
                    .collect(),
            });
        }

        layers
    }

    pub fn top_recommendation_rows(&self) -> Vec<RecommendationRowVm> {
        let Some(recommendations) = self.selected_recommendations() else {
            return Vec::new();
        };

        recommendation_bucket_order()
            .into_iter()
            .filter_map(|bucket| top_row_for_bucket(recommendations, bucket))
            .collect()
    }

    pub fn hero_verdict(&self) -> Option<HeroVerdictVm> {
        let recommendations = self.selected_recommendations()?;
        let rows = self.top_recommendation_rows();
        let strongest_positive = rows
            .iter()
            .find(|row| {
                matches!(
                    row.bucket,
                    RecommendationBucketDto::Nen | RecommendationBucketDto::CoThe
                )
            })
            .map(|row| row.label.clone());
        let strongest_negative = rows
            .iter()
            .find(|row| row.bucket == RecommendationBucketDto::KyManh)
            .or_else(|| {
                rows.iter()
                    .find(|row| row.bucket == RecommendationBucketDto::Tranh)
            })
            .map(|row| row.label.clone());
        let summary = if recommendations.summary_vi.trim().is_empty() {
            strongest_positive
                .clone()
                .or_else(|| strongest_negative.clone())
                .unwrap_or_default()
        } else {
            recommendations.summary_vi.clone()
        };

        Some(HeroVerdictVm {
            summary,
            strongest_positive,
            strongest_negative,
        })
    }

    pub fn scholar_timing_summary(&self) -> Option<ScholarTimingSummaryVm> {
        let bundle = self.bundle.as_ref()?;
        let gio = bundle.gio_hoang_dao.as_ref();
        let insight_hours = bundle
            .insight
            .as_ref()
            .and_then(|insight| insight.hours.as_ref());

        if gio.is_none() && insight_hours.is_none() {
            return None;
        }

        let summary = gio
            .and_then(|hours| {
                let summary = hours.summary.trim();
                (!summary.is_empty()).then_some(summary.to_string())
            })
            .or_else(|| {
                insight_hours.map(|hours| format_good_hour_count_summary(hours.good_hour_count))
            })
            .or_else(|| gio.map(|hours| format_good_hour_count_summary(hours.good_hour_count)))?;

        let mut windows = Vec::new();
        let mut seen = BTreeSet::new();

        if let Some(hours) = insight_hours {
            for hour in hours.good_hours.iter().take(3) {
                let dedupe_key = format!("{}|{}", hour.chi, hour.time_range);
                if seen.insert(dedupe_key) {
                    windows.push(format_hour_window(
                        &hour.chi,
                        &hour.time_range,
                        Some(hour.star.as_str()),
                    ));
                }
            }
        }

        if let Some(hours) = gio {
            for hour in &hours.good_hours {
                if windows.len() >= 3 {
                    break;
                }

                let dedupe_key = format!("{}|{}", hour.hour_chi, hour.time_range);
                if seen.insert(dedupe_key) {
                    windows.push(format_hour_window(
                        &hour.hour_chi,
                        &hour.time_range,
                        Some(hour.star.as_str()),
                    ));
                }
            }
        }

        Some(ScholarTimingSummaryVm { summary, windows })
    }

    pub fn hours_verdict(&self) -> Option<HoursVerdictVm> {
        let bundle = self.bundle.as_ref()?;
        let timing = self.scholar_timing_summary()?;
        let mut bad_windows = Vec::new();

        if let Some(gio) = &bundle.gio_hoang_dao {
            for hour in gio.all_hours.iter().filter(|hour| !hour.is_good).take(3) {
                bad_windows.push(format_hour_window(
                    &hour.hour_chi,
                    &hour.time_range,
                    Some(hour.star.as_str()),
                ));
            }
        }

        let caution = self.hero_verdict().and_then(|verdict| {
            if timing.windows.is_empty() {
                return None;
            }

            verdict.strongest_negative.map(|negative| {
                format!("Có giờ đẹp để xoay xở, nhưng tổng thể ngày vẫn cần dè chừng: {negative}.")
            })
        });

        Some(HoursVerdictVm {
            summary: timing.summary,
            top_windows: timing.windows,
            caution,
            bad_windows,
        })
    }

    pub fn risk_summary(&self) -> RiskSummaryVm {
        let mut items = Vec::new();
        for row in self.top_recommendation_rows() {
            if row.bucket == RecommendationBucketDto::KyManh {
                items.push(format!("Kỵ mạnh: {}", row.label));
            }
        }

        if let Some(fortune) = self
            .bundle
            .as_ref()
            .and_then(|bundle| bundle.day_fortune.as_ref())
        {
            for taboo in &fortune.taboos {
                items.push(format!("Kiêng kỵ: {}", taboo.name));
            }
            items.push(format!("Lục xung: {}", fortune.xung_hop.luc_xung));
            items.push(format!("Sát hướng: {}", fortune.conflict.sat_huong));
        }

        RiskSummaryVm { items }
    }

    pub fn scholar_risk_board(&self) -> ScholarRiskBoardVm {
        let flat_summary = self.risk_summary();
        let mut critical_items = Vec::new();
        let mut caution_items = Vec::new();
        let mut conflict_items = Vec::new();

        for row in self.top_recommendation_rows() {
            match row.bucket {
                RecommendationBucketDto::KyManh => {
                    push_unique(&mut critical_items, format!("Kỵ mạnh: {}", row.label));
                }
                RecommendationBucketDto::Tranh => {
                    let label = row
                        .reason_chip
                        .as_ref()
                        .map(|chip| format!("Tránh: {} · {}", row.label, chip))
                        .unwrap_or_else(|| format!("Tránh: {}", row.label));
                    push_unique(&mut caution_items, label);
                }
                _ => {}
            }
        }

        if let Some(fortune) = self
            .bundle
            .as_ref()
            .and_then(|bundle| bundle.day_fortune.as_ref())
        {
            for taboo in &fortune.taboos {
                let item = if taboo.reason.trim().is_empty() {
                    format!("Kiêng kỵ: {}", taboo.name)
                } else {
                    format!("Kiêng kỵ: {} · {}", taboo.name, taboo.reason)
                };

                if taboo.severity.eq_ignore_ascii_case("high") {
                    push_unique(&mut critical_items, item);
                } else {
                    push_unique(&mut caution_items, item);
                }
            }

            if !fortune.conflict.tuoi_xung.is_empty() {
                push_unique(
                    &mut conflict_items,
                    format!("Tuổi xung: {}", fortune.conflict.tuoi_xung.join(", ")),
                );
            }
            if !fortune.xung_hop.luc_xung.trim().is_empty() {
                push_unique(
                    &mut conflict_items,
                    format!("Lục xung: {}", fortune.xung_hop.luc_xung),
                );
            }
            if !fortune.conflict.sat_huong.trim().is_empty() {
                push_unique(
                    &mut conflict_items,
                    format!("Sát hướng: {}", fortune.conflict.sat_huong),
                );
            }
        }

        let headline = critical_items
            .first()
            .cloned()
            .or_else(|| caution_items.first().cloned())
            .or_else(|| flat_summary.items.first().cloned())
            .or_else(|| conflict_items.first().cloned());

        ScholarRiskBoardVm {
            headline,
            critical_items,
            caution_items,
            conflict_items,
            notice: self.sensitive_domain_notice(),
        }
    }

    pub fn scholar_verdict_support(&self) -> Option<ScholarVerdictSupportVm> {
        let bundle = self.bundle.as_ref()?;
        let mut segments = Vec::new();

        if let Some(canchi) = bundle
            .canchi
            .as_ref()
            .map(|canchi| canchi.full.trim())
            .filter(|full| !full.is_empty())
        {
            segments.push(canchi.to_string());
        }

        if let Some(truc) = bundle
            .day_fortune
            .as_ref()
            .map(|fortune| fortune.truc.name.trim())
        {
            if !truc.is_empty() {
                segments.push(format!("Trực {truc}"));
            }
        }

        if let Some(star_summary) = bundle.day_fortune.as_ref().and_then(primary_star_summary) {
            segments.push(star_summary);
        }

        if let Some(primary_risk) = self
            .risk_summary()
            .items
            .into_iter()
            .find(|item| item.starts_with("Kiêng kỵ:") || item.starts_with("Kỵ mạnh:"))
        {
            segments.push(primary_risk);
        }

        if segments.is_empty() {
            let verdict = self.hero_verdict()?;
            if let Some(positive) = verdict.strongest_positive {
                segments.push(format!("Nên: {positive}"));
            }
            if let Some(negative) = verdict.strongest_negative {
                segments.push(format!("Tránh: {negative}"));
            }
        }

        if segments.is_empty() {
            return None;
        }

        let layer_note = self
            .recommendation_layers()
            .first()
            .filter(|layer| layer.kind == RecommendationLayerKind::Contextual)
            .map(|layer| format!("Ngữ cảnh ưu tiên: {}", layer.summary));

        Some(ScholarVerdictSupportVm {
            support_line: segments.join(" · "),
            layer_note,
        })
    }

    pub fn direction_verdict(&self) -> Option<DirectionVerdictVm> {
        let bundle = self.bundle.as_ref()?;

        let xuat_hanh = bundle
            .insight
            .as_ref()
            .and_then(|insight| insight.travel.as_ref())
            .map(|travel| travel.xuat_hanh_huong.as_str())
            .or_else(|| {
                bundle
                    .day_fortune
                    .as_ref()
                    .map(|fortune| fortune.travel.xuat_hanh_huong.as_str())
            });
        let hy_than = bundle
            .insight
            .as_ref()
            .and_then(|insight| insight.travel.as_ref())
            .map(|travel| travel.hy_than.as_str())
            .or_else(|| {
                bundle
                    .day_fortune
                    .as_ref()
                    .map(|fortune| fortune.travel.hy_than.as_str())
            });
        let tai_than = bundle
            .insight
            .as_ref()
            .and_then(|insight| insight.travel.as_ref())
            .map(|travel| travel.tai_than.as_str())
            .or_else(|| {
                bundle
                    .day_fortune
                    .as_ref()
                    .map(|fortune| fortune.travel.tai_than.as_str())
            });

        if xuat_hanh.is_none() && hy_than.is_none() && tai_than.is_none() {
            return None;
        }

        let summary = match xuat_hanh {
            Some(direction) if !direction.trim().is_empty() => {
                format!("Nếu cần hành sự, ưu tiên dịch chuyển về {direction}.")
            }
            _ => "Nên lấy hướng và thần vị làm điểm neo khi xuất hành.".to_string(),
        };

        let mut directions = Vec::new();
        if let Some(direction) = xuat_hanh.filter(|value| !value.trim().is_empty()) {
            directions.push(format!("Xuất hành: {direction}"));
        }
        if let Some(direction) = hy_than.filter(|value| !value.trim().is_empty()) {
            directions.push(format!("Hỷ Thần: {direction}"));
        }
        if let Some(direction) = tai_than.filter(|value| !value.trim().is_empty()) {
            directions.push(format!("Tài Thần: {direction}"));
        }

        let deity_context = bundle
            .insight
            .as_ref()
            .and_then(|insight| insight.day_deity.as_ref())
            .map(|deity| {
                let mut segments = vec![format!(
                    "{} · {}",
                    deity.name, deity.classification_meaning.vi
                )];
                if let Some(meaning) = deity
                    .deity_meaning
                    .as_ref()
                    .map(|meaning| take_first_sentence(&meaning.vi))
                    .filter(|meaning| !meaning.is_empty())
                {
                    segments.push(meaning);
                }
                segments.join(" · ")
            });

        let note = self
            .recommendation_layers()
            .first()
            .filter(|layer| layer.kind == RecommendationLayerKind::Contextual)
            .map(|layer| format!("Ngữ cảnh đang ưu tiên: {}", layer.profile));

        Some(DirectionVerdictVm {
            summary,
            directions,
            deity_context,
            note,
        })
    }

    pub fn day_identity_summary(&self) -> Option<DayIdentitySummaryVm> {
        let bundle = self.bundle.as_ref()?;
        let canchi = bundle.canchi.as_ref();
        let fortune = bundle.day_fortune.as_ref();
        let insight = bundle.insight.as_ref();

        if canchi.is_none() && fortune.is_none() && insight.is_none() {
            return None;
        }

        let mut headline_parts = Vec::new();
        if let Some(canchi) = canchi {
            headline_parts.push(canchi.day.full.clone());
        }
        if let Some(fortune) = fortune {
            headline_parts.push(format!(
                "{} · {}",
                fortune.day_element.element, fortune.day_element.na_am
            ));
        }
        let headline = if headline_parts.is_empty() {
            "Khí ngày chưa đủ dữ liệu để luận".to_string()
        } else {
            headline_parts.join(" · ")
        };

        let mut detail_lines = Vec::new();
        if let Some(canchi) = canchi {
            push_unique(
                &mut detail_lines,
                format!(
                    "Can chi ngày: {} {} · con giáp {}",
                    canchi.day.can, canchi.day.chi, canchi.day.con_giap
                ),
            );
        }
        if let Some(fortune) = fortune {
            push_unique(
                &mut detail_lines,
                format!(
                    "Ngũ hành ngày: {} · can {} / chi {}",
                    fortune.day_element.element,
                    fortune.day_element.can_element,
                    fortune.day_element.chi_element
                ),
            );
        }
        if let Some(can_chi_insight) = insight.and_then(|insight| insight.canchi.as_ref()) {
            let element_tone = can_chi_insight
                .element
                .as_ref()
                .map(|element| take_first_sentence(&element.nature.vi))
                .filter(|value| !value.is_empty());
            let can_tone = take_first_sentence(&can_chi_insight.can.nature.vi);
            let chi_tone = take_first_sentence(&can_chi_insight.chi.meaning.vi);
            let mut parts = vec![
                format!("Can {}: {}", can_chi_insight.can.name, can_tone),
                format!("Chi {}: {}", can_chi_insight.chi.name, chi_tone),
            ];
            if let Some(element_tone) = element_tone {
                parts.push(format!("Khí hành: {element_tone}"));
            }
            push_unique(&mut detail_lines, parts.join(" · "));
        }
        if let Some(na_am) = insight.and_then(|insight| insight.na_am.as_ref()) {
            push_unique(
                &mut detail_lines,
                format!(
                    "Nạp âm {}: {}",
                    na_am.na_am,
                    take_first_sentence(&na_am.meaning.vi)
                ),
            );
        }

        let application_note = insight
            .and_then(|insight| insight.day_guidance.as_ref())
            .and_then(|guidance| guidance.good_for.vi.first())
            .map(|value| format!("Ứng dụng: hợp để {value}."))
            .or_else(|| {
                insight
                    .and_then(|insight| insight.truc.as_ref())
                    .and_then(|truc| truc.good_for.vi.first())
                    .map(|value| format!("Ứng dụng: trực này thuận cho {value}."))
            });

        Some(DayIdentitySummaryVm {
            headline,
            detail_lines,
            application_note,
        })
    }

    pub fn traditional_evidence_summary(&self) -> Option<TraditionalEvidenceSummaryVm> {
        let bundle = self.bundle.as_ref()?;
        let mut headline_parts = Vec::new();
        let mut positive_signals = Vec::new();
        let mut caution_signals = Vec::new();
        let mut provenance = Vec::new();

        if let Some(truc) = bundle
            .insight
            .as_ref()
            .and_then(|insight| insight.truc.as_ref())
        {
            headline_parts.push(format!("Trực {} ({})", truc.name, truc.quality));
            let meaning = take_first_sentence(&truc.meaning.vi);
            if !meaning.is_empty() {
                push_unique(&mut positive_signals, format!("Luận trực: {meaning}"));
            }
        } else if let Some(fortune) = &bundle.day_fortune {
            headline_parts.push(format!(
                "Trực {} ({})",
                fortune.truc.name, fortune.truc.quality
            ));
        }

        if let Some(stars) = bundle
            .insight
            .as_ref()
            .and_then(|insight| insight.stars.as_ref())
        {
            if let Some(day_star) = &stars.day_star {
                let quality = stars.day_star_quality.as_deref().unwrap_or("không rõ");
                headline_parts.push(format!("Sao ngày {day_star} ({quality})"));
            }

            for star in stars.cat_tinh.iter().take(3) {
                push_unique(&mut positive_signals, format!("Cát tinh: {star}"));
            }
            for star in stars.sat_tinh.iter().take(3) {
                push_unique(&mut caution_signals, format!("Hung tinh: {star}"));
            }
        }

        if let Some(fortune) = &bundle.day_fortune {
            for rule in fortune.stars.matched_rules.iter().take(4) {
                push_unique(
                    &mut provenance,
                    format!("{} · {} · {}", rule.name, rule.quality, rule.category),
                );
            }
        }

        if headline_parts.is_empty()
            && positive_signals.is_empty()
            && caution_signals.is_empty()
            && provenance.is_empty()
        {
            return None;
        }

        Some(TraditionalEvidenceSummaryVm {
            headline: (!headline_parts.is_empty()).then_some(headline_parts.join(" · ")),
            positive_signals,
            caution_signals,
            provenance,
        })
    }

    pub fn seasonal_verdict(&self) -> Option<SeasonalVerdictVm> {
        let bundle = self.bundle.as_ref()?;
        let tiet_khi = bundle.tiet_khi.as_ref()?;
        let insight = bundle
            .insight
            .as_ref()
            .and_then(|insight| insight.tiet_khi.as_ref());

        let headline = format!("{} · mùa {}", tiet_khi.name, tiet_khi.season);
        let implication = insight
            .map(|insight| take_first_sentence(&insight.meaning.vi))
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| take_first_sentence(&tiet_khi.description));

        let mut application_lines = Vec::new();
        if let Some(insight) = insight {
            let weather = take_first_sentence(&insight.weather.vi);
            if !weather.is_empty() {
                push_unique(&mut application_lines, format!("Thời khí: {weather}"));
            }
            if let Some(item) = insight.agriculture.vi.first() {
                push_unique(&mut application_lines, format!("Nhịp việc mùa này: {item}"));
            }
            if let Some(item) = insight.health.vi.first() {
                push_unique(&mut application_lines, format!("Chăm sóc cơ thể: {item}"));
            }
        }

        Some(SeasonalVerdictVm {
            headline,
            implication,
            application_lines,
        })
    }

    pub fn profile_availability_summary(&self) -> Option<ProfileAvailabilityVm> {
        let bundle = self.bundle.as_ref()?;
        let has_personal_overlay = bundle
            .insight
            .as_ref()
            .map(|insight| insight.tu_menh.is_some() || insight.dai_van.is_some())
            .unwrap_or(false);

        let note = if has_personal_overlay {
            "Đã có lớp cá nhân hóa; tách riêng phần ngày chung và phần mệnh cá nhân.".to_string()
        } else {
            "Chưa có hồ sơ cá nhân; màn hình này chỉ nên đọc như hướng theo ngày, không phải phong thủy bản mệnh.".to_string()
        };

        Some(ProfileAvailabilityVm {
            has_personal_overlay,
            note,
        })
    }

    pub fn sensitive_domain_notice(&self) -> Option<String> {
        let recommendations = self.selected_recommendations()?;
        let has_medical = recommendations
            .activities
            .iter()
            .any(|activity| activity.activity_id == "medical_treatment");
        let has_burial = recommendations
            .activities
            .iter()
            .any(|activity| activity.activity_id == "burial_memorial");

        let mut notes = Vec::new();
        if has_medical {
            notes.push(
                "Lưu ý: điều trị thực tế luôn ưu tiên đánh giá chuyên môn; lịch chỉ mang tính tham khảo."
                    .to_string(),
            );
        }
        if has_burial {
            notes.push(
                "Lưu ý: an táng hoặc tưởng niệm cần thẩm định thêm theo tập tục và chuyên gia địa phương."
                    .to_string(),
            );
        }

        if notes.is_empty() {
            None
        } else {
            Some(notes.join(" "))
        }
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

fn recommendation_bucket_order() -> [RecommendationBucketDto; 4] {
    [
        RecommendationBucketDto::Nen,
        RecommendationBucketDto::CoThe,
        RecommendationBucketDto::Tranh,
        RecommendationBucketDto::KyManh,
    ]
}

fn top_row_for_bucket(
    recommendations: &DailyRecommendationsDto,
    bucket: RecommendationBucketDto,
) -> Option<RecommendationRowVm> {
    let activity = recommendations
        .activities
        .iter()
        .find(|activity| activity.bucket == bucket)?;
    let reason_chip = activity
        .reasons
        .iter()
        .min_by_key(|reason| severity_rank(reason.severity))
        .map(|reason| {
            format!(
                "{} • {}",
                severity_label(reason.severity),
                source_label(reason.evidence.source)
            )
        });

    Some(RecommendationRowVm {
        bucket,
        label: activity.label.vi.clone(),
        reason_chip,
        reason_details: activity
            .reasons
            .iter()
            .map(|reason| {
                format!(
                    "{} · {} · {} · {} · {}",
                    severity_label(reason.severity),
                    source_label(reason.evidence.source),
                    reason.summary_vi,
                    reason.evidence.code,
                    reason.evidence.note
                )
            })
            .collect(),
    })
}

fn severity_rank(severity: RecommendationSeverityDto) -> u8 {
    match severity {
        RecommendationSeverityDto::Override => 0,
        RecommendationSeverityDto::Primary => 1,
        RecommendationSeverityDto::Supporting => 2,
    }
}

fn severity_label(severity: RecommendationSeverityDto) -> &'static str {
    match severity {
        RecommendationSeverityDto::Override => "override",
        RecommendationSeverityDto::Primary => "primary",
        RecommendationSeverityDto::Supporting => "support",
    }
}

fn source_label(source: RecommendationEvidenceSourceDto) -> &'static str {
    match source {
        RecommendationEvidenceSourceDto::DayGuidance => "guidance",
        RecommendationEvidenceSourceDto::Truc => "trực",
        RecommendationEvidenceSourceDto::Stars => "sao",
        RecommendationEvidenceSourceDto::DayDeity => "thần sát",
        RecommendationEvidenceSourceDto::Taboo => "kiêng kỵ",
        RecommendationEvidenceSourceDto::XungHop => "xung-hợp",
        RecommendationEvidenceSourceDto::TietKhi => "tiết khí",
        RecommendationEvidenceSourceDto::GioHoangDao => "giờ tốt",
        RecommendationEvidenceSourceDto::Travel => "xuất hành",
        RecommendationEvidenceSourceDto::ProductRule => "mở rộng",
    }
}

fn recommendation_scope_label(scope: amlich_api::RecommendationScopeDto) -> &'static str {
    match scope {
        amlich_api::RecommendationScopeDto::GeneralDay => "general_day",
    }
}

fn format_good_hour_count_summary(good_hour_count: usize) -> String {
    match good_hour_count {
        0 => "Không thấy khung giờ đẹp nổi bật.".to_string(),
        1 => "Có 1 khung giờ thuận để hành sự.".to_string(),
        count => format!("Có {count} khung giờ thuận để hành sự."),
    }
}

fn format_hour_window(chi: &str, time_range: &str, star: Option<&str>) -> String {
    match star.map(str::trim).filter(|star| !star.is_empty()) {
        Some(star) => format!("{chi} {time_range} · {star}"),
        None => format!("{chi} {time_range}"),
    }
}

fn take_first_sentence(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    trimmed
        .split_terminator(['.', '!', '?', '\n'])
        .next()
        .unwrap_or(trimmed)
        .trim()
        .to_string()
}

fn push_unique(items: &mut Vec<String>, value: String) {
    if !items.iter().any(|item| item == &value) {
        items.push(value);
    }
}

fn primary_star_summary(fortune: &amlich_api::DayFortuneDto) -> Option<String> {
    fortune
        .stars
        .day_star
        .as_ref()
        .map(|star| format!("Sao ngày {}", star.name))
        .or_else(|| {
            fortune
                .stars
                .cat_tinh
                .first()
                .map(|star| format!("Cát tinh {star}"))
        })
        .or_else(|| {
            fortune
                .stars
                .sat_tinh
                .first()
                .map(|star| format!("Hung tinh {star}"))
        })
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
            lens: FocusLens::General,
            scroll_offset: 0,
            bundle: None,
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
            active_view: ActiveView::Dashboard,
            view_history: Vec::new(),
            app_mode: AppMode::Normal,
            focused_section: PageSection::Hero,
            zoomed_section: None,
            expanded_sections: Default::default(),
            search_input: String::new(),
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
    fn scholar_timing_summary_prefers_curated_windows_and_existing_gio_summary() {
        let app = sample_app_state_with_bundle();

        let timing = app.scholar_timing_summary().expect("timing summary");

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
    fn scholar_risk_board_groups_critical_caution_and_conflict_rows() {
        let app = sample_app_state_with_bundle();

        let risk_board = app.scholar_risk_board();

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
    fn scholar_verdict_support_combines_traditional_evidence_and_active_layer_note() {
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
            .scholar_verdict_support()
            .expect("scholar verdict support");

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
