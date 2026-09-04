//! v1.11 Point-Opening Context — the `TNLC-DIV-*` known-divergence
//! registry.
//!
//! Mirrors `.planning/research/TNLC_POINT_OPENING_RESEARCH.md` §2. The
//! registry is the vocabulary the domain carrier validates
//! `known_divergence_ids` against; divergences are disclosures, never
//! errors to resolve. The frozen corpus uses TNLC-DIV-01/02/03/05 on
//! its rows and grid cells and TNLC-DIV-04 on the nomenclature
//! registry; `tests/point_opening_contract_guard.rs` asserts the
//! lockstep.

/// One known-divergence entry (id, title, project decision).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TnlcDivergence {
    /// Stable id, e.g. `TNLC-DIV-01`.
    pub id: &'static str,
    /// Short title of the divergence.
    pub title: &'static str,
    /// The recorded project decision (never presented as a classical
    /// fact).
    pub decision: &'static str,
}

const REGISTRY: &[TnlcDivergence] = &[
    TnlcDivergence {
        id: "TNLC-DIV-01",
        title: "Closed (閉穴) slots and later filling schools",
        decision: "Freeze the Xu-style tables as printed in Zhenjiu Dacheng; closed slots serialize an explicit closed state; filling schools are recorded as divergence, never merged in.",
    },
    TnlcDivergence {
        id: "TNLC-DIV-02",
        title: "氣納三焦 / 血納包絡 row placement across editions",
        decision: "Encode the Zhenjiu Dacheng rows; keep 三焦/心包 as traditional identities (LH-DIV-06 alignment); no biomedical conversion.",
    },
    TnlcDivergence {
        id: "TNLC-DIV-03",
        title: "Classical timekeeping vs modern civil time and the day boundary",
        decision: "Reuse Amlich's existing day-pillar and local_civil_hour_branch conventions; disclose time_basis; pin cross-day spillover goldens to the frozen convention (the 23:00-01:00 block belongs to the upcoming civil date).",
    },
    TnlcDivergence {
        id: "TNLC-DIV-04",
        title: "Vietnamese point nomenclature variance and code glosses",
        decision: "Gate 2 signs one Vietnamese nomenclature set + code set; codes are presented as standard lookup glosses, never as WHO endorsement of efficacy.",
    },
    TnlcDivergence {
        id: "TNLC-DIV-05",
        title: "Classical contestation of the method",
        decision: "Carry the v1.10 historical-contestation marker; a point-opening citation is never a physiological or efficacy claim.",
    },
];

/// Look up a registered divergence by id.
pub fn tnlc_divergence_by_id(id: &str) -> Option<&'static TnlcDivergence> {
    REGISTRY.iter().find(|d| d.id == id)
}

/// All registered divergences, in id order.
pub fn all_tnlc_divergences() -> &'static [TnlcDivergence] {
    REGISTRY
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_is_complete_and_unique() {
        assert_eq!(REGISTRY.len(), 5);
        for (i, d) in REGISTRY.iter().enumerate() {
            assert_eq!(d.id, format!("TNLC-DIV-{:02}", i + 1));
        }
    }

    #[test]
    fn lookup_resolves_every_registered_id() {
        for d in REGISTRY {
            assert_eq!(tnlc_divergence_by_id(d.id).map(|r| r.id), Some(d.id));
        }
        assert!(tnlc_divergence_by_id("TNLC-DIV-99").is_none());
    }

    #[test]
    fn every_entry_carries_title_and_decision() {
        for d in REGISTRY {
            assert!(!d.title.is_empty(), "{} must carry a title", d.id);
            assert!(!d.decision.is_empty(), "{} must carry a decision", d.id);
        }
    }
}
