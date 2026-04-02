pub use super::legacy::{
    ActivePackVm, ActiveView, AppMode, AppState, DayIdentitySummaryVm, DirectionVerdictVm,
    ExplorerAction, ExplorerField, ExplorerSelection, FocusLens, HeroVerdictVm, HoursVerdictVm,
    PageSection, ProfileAvailabilityVm, RecommendationLayerKind, RecommendationLayerVm,
    RecommendationRowVm, RiskSummaryVm, DayDetailRiskBoardVm, DayDetailTimingSummaryVm,
    DayDetailVerdictSupportVm, SeasonalVerdictVm, TraditionalEvidenceSummaryVm,
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
