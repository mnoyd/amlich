//! Vietnamese ritual content (Văn khấn cổ truyền).
//!
//! Phase 10 lands the schema lock only (this module's `schema` sub-module).
//! Phase 11 will add `corpus` (OnceLock loader) and `matcher` (lookup APIs).
//! Phase 12 authors ≥60 corpus entries under `data/rituals/`.
//!
//! Schema is locked by ADR-0001 (`.planning/adrs/0001-ritual-schema-v1.md`).

pub mod schema;
mod corpus;
