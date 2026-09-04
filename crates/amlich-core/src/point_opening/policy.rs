//! v1.11 Point-Opening Context — the `TY_NGO_LUU_CHU_POLICY_V1`
//! contract (bead `amlich-xlag.2.2.1`, ADR-0004).
//!
//! ADR-0003 reserved the source id emitted by this module until a
//! separately reviewed milestone shipped its own policy contract,
//! golden dataset, and safety review. v1.11 is that milestone: the
//! reservation is satisfied by the contract below, which the corpus
//! loader, the resolver, and the CI guards all enforce. The
//! first-emission privilege is confined to [`crate::point_opening`]
//! plus the constant definition in [`crate::sources`] — the substring
//! guard in `tests/source_id_guard.rs` keeps it that way.

use serde::{Deserialize, Serialize};

use crate::sources::{SOURCE_SHI_ER_JING_NA_DI_ZHI, SOURCE_TY_NGO_LUU_CHU};

/// Policy id stamped on every Point-Opening Context and asserted by the
/// integration goldens.
pub const TY_NGO_LUU_CHU_POLICY_ID: &str = "TY_NGO_LUU_CHU_POLICY_V1";

/// Canonical safety classification for every point-opening row
/// (BOUND-02). Serializes on the resolver output; the prohibited-field
/// guard asserts nothing else rides along.
pub const SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION: &str = "historical_procedural_citation";

/// The machine-readable policy contract. Field values mirror ADR-0004's
/// consequences one-to-one and are asserted by
/// `tests/point_opening_contract_guard.rs`; serde round trips preserve
/// every field so the contract can travel alongside any surfaced
/// context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContract {
    /// `TY_NGO_LUU_CHU_POLICY_V1`.
    pub policy_id: String,
    /// The reserved source id performing its first, policy-gated
    /// emission.
    pub source_id: String,
    /// `historical_procedural_citation`.
    pub safety_class: String,
    /// The Tier-0 corpus that must never be cross-cited by point rows.
    pub never_cross_cites: String,
    /// Closed (閉穴) slots serialize an explicit unavailable-by-tradition
    /// state and are never filled by later-school rules (TNLC-DIV-01).
    pub closed_slots_stay_closed: bool,
    /// Point output is citation framing only: no technique, depth,
    /// manipulation, indication, contraindication, or efficacy content
    /// exists in any schema or surface.
    pub citation_framing_only: bool,
    /// Every surfaced context carries bilingual disclaimer v2 with its
    /// review state visible until the named gates sign.
    pub disclaimer_required_until_gates_sign: bool,
}

/// The canonical v1.11 policy contract instance.
pub fn policy_contract() -> PolicyContract {
    PolicyContract {
        policy_id: TY_NGO_LUU_CHU_POLICY_ID.to_string(),
        source_id: SOURCE_TY_NGO_LUU_CHU.to_string(),
        safety_class: SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION.to_string(),
        never_cross_cites: SOURCE_SHI_ER_JING_NA_DI_ZHI.to_string(),
        closed_slots_stay_closed: true,
        citation_framing_only: true,
        disclaimer_required_until_gates_sign: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_pins_the_reserved_source_and_its_separation() {
        let contract = policy_contract();
        assert_eq!(contract.policy_id, TY_NGO_LUU_CHU_POLICY_ID);
        assert_eq!(contract.source_id, crate::sources::SOURCE_TY_NGO_LUU_CHU);
        assert_eq!(
            contract.never_cross_cites,
            crate::sources::SOURCE_SHI_ER_JING_NA_DI_ZHI
        );
        assert_ne!(contract.source_id, contract.never_cross_cites);
        assert!(contract.closed_slots_stay_closed);
        assert!(contract.citation_framing_only);
        assert!(contract.disclaimer_required_until_gates_sign);
    }

    #[test]
    fn safety_class_is_the_canonical_literal() {
        assert_eq!(
            SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION,
            "historical_procedural_citation"
        );
    }

    #[test]
    fn policy_id_is_the_canonical_literal() {
        assert_eq!(TY_NGO_LUU_CHU_POLICY_ID, "TY_NGO_LUU_CHU_POLICY_V1");
    }

    #[test]
    fn contract_round_trips_through_serde_preserving_every_field() {
        let contract = policy_contract();
        let json = serde_json::to_string(&contract).expect("contract serializes");
        let recovered: PolicyContract = serde_json::from_str(&json).expect("contract parses");
        assert_eq!(recovered, contract);
        let json2 = serde_json::to_string(&recovered).expect("recovered serializes");
        assert_eq!(json, json2, "wire shape must be stable");
    }

    #[test]
    fn contract_wire_shape_pins_every_field_name() {
        let json = serde_json::to_value(policy_contract()).expect("serializes");
        let expected = serde_json::json!({
            "policy_id": TY_NGO_LUU_CHU_POLICY_ID,
            "source_id": crate::sources::SOURCE_TY_NGO_LUU_CHU,
            "safety_class": SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION,
            "never_cross_cites": crate::sources::SOURCE_SHI_ER_JING_NA_DI_ZHI,
            "closed_slots_stay_closed": true,
            "citation_framing_only": true,
            "disclaimer_required_until_gates_sign": true,
        });
        assert_eq!(json, expected);
    }
}
