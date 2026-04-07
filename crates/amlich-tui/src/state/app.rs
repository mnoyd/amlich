pub use super::app_state::{
    ActivePackVm, ActiveView, AppMode, AppState, DayDetailRiskBoardVm, DayDetailTimingSummaryVm,
    DayDetailVerdictSupportVm, DayIdentitySummaryVm, DirectionVerdictVm, ExplorerAction,
    ExplorerField, ExplorerSelection, HeroVerdictVm, HoursVerdictVm, PageSection,
    ProfileAvailabilityVm, RecommendationLayerKind, RecommendationLayerVm, RecommendationRowVm,
    RiskSummaryVm, SeasonalVerdictVm, TraditionalEvidenceSummaryVm,
};

use super::ui_prefs::VerbosityMode;

impl AppState {
    pub fn active_verbosity(&self) -> VerbosityMode {
        self.verbosity
    }

    pub fn toggle_verbosity(&mut self) {
        self.verbosity = self.verbosity.toggle();
    }
}
