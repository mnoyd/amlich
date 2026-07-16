//! v1.7 IChing (Kinh Dịch) module. See ADR-0005 (schema), ADR-0006 (casting convention).
//!
//! Phase 20 locks the schema + three CRIT-3-isolating newtypes + the bijective
//! 64-entry Tiên Thiên-pair → King Wen composition table only. Phase 21 authors
//! the 64-entry corpus; Phase 22 implements Mai Hoa casting; Phase 24 wires the
//! evaluator.
//!
//! CRITICAL (CRIT-3 prevention): the three newtypes (`TienThienTrigram`,
//! `HauThienTrigram`, `KingWenHexagram`) carry NO `impl From<...>` between
//! them. The composition table is the ONLY bridge. Adding a cross-newtype
//! `From` re-opens CRIT-3 (Tiên Thiên numbers ≠ King Wen numbers, shared
//! 1..N form).
pub mod corpus;
pub mod schema;

pub use corpus::{all_hexagrams, get_hexagram};
pub use schema::{
    compose, HauThienTrigram, HexagramEntry, KingWenHexagram, TienThienTrigram, COMPOSITION_TABLE,
};
