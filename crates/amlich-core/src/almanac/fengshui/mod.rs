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

pub mod period;
pub mod scanner;
pub mod stars;
pub mod types;

// Re-exports — public API surface for fengshui module consumers.
pub use period::{
    base_palaces_for_van, compute_period, compute_period_for_year, load_flying_stars_base, Period,
};
pub use scanner::TietKhiScanner;
pub use stars::{flying_star_from_u8, star_metadata};
