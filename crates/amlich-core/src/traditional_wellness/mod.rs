//! v1.10 Tier-0 Traditional Wellness Context (十二經納地支 + 四氣調神大論).
//!
//! Sibling of [`crate::reasoning`]; does not contribute to Day Assessment,
//! Hour Ranking, or Direction Assessment per ADR-0003. Submodules:
//!
//! - [`disclaimer`] — the bilingual cultural-information disclaimer text
//!   and the stable [`DisclaimerId`](disclaimer::DisclaimerId) newtype.
//! - [`divergence`] — the [`TimeBasis`](divergence::TimeBasis),
//!   [`ExternalReviewState`](divergence::ExternalReviewState), and the
//!   [`TraditionalWellnessKnownDivergence`](divergence::TraditionalWellnessKnownDivergence)
//!   registries for both v1.10 tracks.
//! - [`branch_channel`] — the twelve-row historical branch-to-channel
//!   association corpus, the [`BranchChannelAssociation`] struct, the
//!   [`TraditionalWellnessContext`] wrapper, and the lookup helpers.
//! - [`seasonal`] — the four-profile Suwen seasonal cultivation corpus,
//!   the frozen 24-term → 4-season composition, the
//!   [`seasonal::SeasonalCultivationContext`] wrapper, and the lookup
//!   helpers (Phase 02-01, SEASON-01).

pub mod branch_channel;
pub mod disclaimer;
pub mod divergence;
pub mod seasonal;

pub use branch_channel::{
    load_corpus, resolve_hour_branch_association, resolve_traditional_wellness_context,
    resolve_traditional_wellness_context_unified, BranchChannelAssociation, SourceCitation,
    TraditionalWellnessContext,
};
pub use disclaimer::{
    cultural_information_disclaimer, disclaimer_id_cultural_information, DisclaimerId,
    LocalizedDisclaimer, DISCLAIMER_CULTURAL_INFORMATION_EN, DISCLAIMER_CULTURAL_INFORMATION_VN,
    DISCLAIMER_ID_CULTURAL_INFORMATION_STR,
};
pub use divergence::{
    all_divergences_for_branch_channel, all_divergences_for_seasonal_cultivation,
    channel_not_organ, civil_time_disclosure, divergence_by_id, fixed_cycle_contestation,
    four_profiles_not_term_regimens, organ_injury_clauses_omitted, phenology_is_not_local_weather,
    ExternalReviewState, TimeBasis, TraditionalWellnessKnownDivergence,
    TraditionalWellnessSourceValue,
};
pub use seasonal::{
    load_seasonal_corpus, resolve_seasonal_cultivation, season_for_term_index, SeasonKey,
    SeasonalCultivationContext, SeasonalCultivationProfile, COMPOSITE_SEASONAL_WELLNESS,
    COMPOSITION_NOTE_EN, COMPOSITION_NOTE_VN, SEASONAL_BOUNDARY_TERM_NAMES,
    SOLAR_TERM_ENGINE_SOURCE_ID, TERMS_PER_SEASON,
};
