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
//!   registry.
//! - [`branch_channel`] — the twelve-row historical branch-to-channel
//!   association corpus, the [`BranchChannelAssociation`] struct, the
//!   [`TraditionalWellnessContext`] wrapper, and the lookup helpers.

pub mod branch_channel;
pub mod disclaimer;
pub mod divergence;

pub use branch_channel::{
    load_corpus, resolve_hour_branch_association, resolve_traditional_wellness_context,
    BranchChannelAssociation, SourceCitation, TraditionalWellnessContext,
};
pub use disclaimer::{
    cultural_information_disclaimer, disclaimer_id_cultural_information, DisclaimerId,
    LocalizedDisclaimer, DISCLAIMER_CULTURAL_INFORMATION_EN, DISCLAIMER_CULTURAL_INFORMATION_VN,
    DISCLAIMER_ID_CULTURAL_INFORMATION_STR,
};
pub use divergence::{
    all_divergences_for_branch_channel, channel_not_organ, civil_time_disclosure,
    divergence_by_id, fixed_cycle_contestation, ExternalReviewState, TimeBasis,
    TraditionalWellnessKnownDivergence, TraditionalWellnessSourceValue,
};
