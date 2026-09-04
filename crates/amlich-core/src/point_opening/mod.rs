//! v1.11 Tier-1 Point-Opening Context (Tý Ngọ Lưu Chú, Xu-style 納甲法).
//!
//! Sibling of [`crate::traditional_wellness`] and [`crate::reasoning`];
//! does not contribute to Day Assessment, Hour Ranking, Direction
//! Assessment, or the v1.10 Traditional Wellness Context (ADR-0003 /
//! ADR-0004). Submodules:
//!
//! - [`policy`] — the `TY_NGO_LUU_CHU_POLICY_V1` contract that governs
//!   the first emission of the reserved source id.
//! - [`disclaimer`] — disclaimer v2 (bilingual historical procedural
//!   citation), byte-locked to the REVIEWER-PACK §A.4.
//! - [`divergence`] — the `TNLC-DIV-*` known-divergence registry.
//! - [`state`] — the stable domain types: the point identity triple,
//!   the exactly-one open-or-explicit-closed slot state, and the
//!   `PointOpeningContext` carrier.
//!
//! This module defines the contract only. The corpus loader, resolver,
//! civil-time boundary integration, DaySnapshot projection, semantic
//! graph, and surface work belong to the later `amlich-xlag.2.2.*`
//! beads. Until the four human review gates sign
//! (`amlich-xlag.2.5`–`.2.8`), every corpus record stays
//! `ExternalReviewPending` and every surfaced context carries
//! disclaimer v2 with its review state visible.

pub mod disclaimer;
pub mod divergence;
pub mod policy;
pub mod state;
pub use crate::traditional_wellness::disclaimer::{DisclaimerId, LocalizedDisclaimer};
pub use disclaimer::{
    disclaimer_id_historical_procedural_citation, historical_procedural_citation_disclaimer,
    DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN, DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN,
    DISCLAIMER_ID_HISTORICAL_PROCEDURAL_CITATION_STR,
};
pub use divergence::{all_tnlc_divergences, tnlc_divergence_by_id, TnlcDivergence};
pub use policy::{
    policy_contract, PolicyContract, SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION,
    TY_NGO_LUU_CHU_POLICY_ID,
};
pub use state::{PointOpeningContext, PointOpeningIdentity, PointOpeningSlotState};
