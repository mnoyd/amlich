//! Phi Tinh (Flying Stars) — Huyền Không thời gian.
//!
//! Phase 10 landed frozen type stubs (`types`).
//! Phase 13 adds `scanner`, `stars`, `period` (this plan — 13-01),
//! then `annual`, `monthly`, `combined` in subsequent plans.
//! Phase 14 will add `aspects.rs` (81-cell star-pair aspects) and `safety.rs`.
//!
//! Schemas locked by ADR-0002 (monthly anchor) and ADR-0003 (Niên polarity matrix).
//!
//! NOTE: per PITFALLS CRIT-3 and CONTEXT.md, `FlyingStar` is a palace-layout
//! descriptor and is NEVER wired into `interaction/direction_merge.rs` in v1.5.

pub mod annual;
pub mod aspects;
pub mod combined;
pub mod golden;
pub mod monthly;
pub mod period;
pub mod safety;
pub mod scanner;
pub mod stars;
pub mod types;

// Re-exports — public API surface for fengshui module consumers.
pub use annual::{compute_yearly_flying_stars, YearPolarity};
pub use aspects::{compute_palace_aspects, lookup_star_pair_aspect, FsCitation, FsConfidenceTier, StarPairAspect};
pub use combined::{compute_combined_overlay, CombinedFlyingStarLayout};
pub use golden::{
    load_flying_stars_golden, DeferralMarker, GoldenConfidence, KnownDivergence,
    PhiTinhGoldenCase, PhiTinhGoldenDataset,
};
pub use monthly::compute_monthly_flying_stars;
pub use period::{
    base_palaces_for_van, compute_period, compute_period_for_year, load_flying_stars_base, Period,
};
pub use safety::{element_hint_for_palace, is_danger_palace, RemedyHint};
pub use scanner::TietKhiScanner;
pub use stars::{flying_star_from_u8, star_metadata};
// Phase 18-01 (FS-17 schema lock): re-export types for the daily Phi Tinh layer
// so external-crate test consumers can import the additive `DailyFlyingStarLayout`
// sibling struct and the extended `FlyingStarPeriod` enum.
pub use types::{
    minimal_evidence, DailyFlyingStarLayout, FlyingStar, FlyingStarLayout, FlyingStarPeriod,
    Palace,
};
