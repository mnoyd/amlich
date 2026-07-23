//! Vietnamese ritual content (Văn khấn cổ truyền).
//!
//! Public API surface:
//!   - `schema::*` — locked RitualEntry / RitualEventKey / etc. (ADR-0001).
//!   - `all_rituals()` — full corpus slice (RIT-05).
//!   - `find_van_khan_for_snapshot(&DaySnapshot)` — RIT-01.
//!   - `find_van_khan_for_event(&RitualEventKey)` — RIT-02.
//!   - `find_van_khan_for_life_event(LifeEventKind)` — RIT-03.
//!   - `get_ritual_by_id(&str)` — RIT-04.
//!
//! Schema is locked by ADR-0001 (`.planning/adrs/0001-ritual-schema-v1.md`).
//! Any change to a public type in `schema.rs` requires a superseding ADR.
//!
//! Canonical Holiday ids consumed by `RitualEventKey::HolidayId { value }` are
//! defined in `data/holidays/lunar-festivals.json`. Authors must reference ids
//! from that file; typos return zero matches silently. The Phase 11 integration
//! test `rituals_holiday_id_cross_reference` (plan 11-04) enforces this.
//!
//! Hán-character pollution is rejected by the CI guard
//! `tests/ritual_han_guard.rs`. NFC normalization happens at corpus load time.

mod corpus;
mod matcher;
pub mod schema;

pub use corpus::all_rituals;
pub use matcher::{
    find_van_khan_for_event, find_van_khan_for_life_event, find_van_khan_for_snapshot,
    get_ritual_by_id,
};
pub use schema::*;
