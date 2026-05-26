//! Phi Tinh (Flying Stars) — Huyền Không thời gian.
//!
//! Phase 10 lands frozen type stubs only (this module's `types` sub-module).
//! Phase 13 will add `period.rs`, `annual.rs`, `monthly.rs`, `combined.rs` with the
//! algorithm implementations and Vận-table JSON loaders.
//! Phase 14 will add `aspects.rs` (81-cell star-pair aspects) and `safety.rs`.
//!
//! Schemas locked by ADR-0002 (monthly anchor) and ADR-0003 (Niên polarity matrix).
//!
//! NOTE: per PITFALLS CRIT-3 and CONTEXT.md, `FlyingStar` is a palace-layout descriptor
//! and is NEVER wired into `interaction/direction_merge.rs` in v1.5.

pub mod types;
