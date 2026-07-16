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
pub mod bien_que;
pub mod corpus;
pub mod evaluator;
pub mod golden;
pub mod mai_hoa;
pub mod schema;
pub mod the_dung;

pub use bien_que::{derive_bien_que, BienQue};
pub use corpus::{all_hexagrams, get_hexagram};
pub use evaluator::{
    HexagramEntryProjection, IChingCastSummary, IChingEvaluation, IChingEvaluator, IChingQuery,
    COMPOSITE_ICHING_CONSULTATION,
};
pub use golden::{load_mai_hoa_golden, MaiHoaGoldenCase, MaiHoaGoldenDataset};
pub use mai_hoa::{cast_mai_hoa, MaiHoaCast};
pub use schema::{
    compose, HauThienTrigram, HexagramEntry, KingWenHexagram, TienThienTrigram, COMPOSITION_TABLE,
};
pub use the_dung::{classify_the_dung, trigram_element, CatHung, TheDungClassification, TheDungRelation};
