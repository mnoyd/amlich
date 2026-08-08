//! Stability gate — the machine-readable report that authorises
//! promoting the experimental v2.x `AssessmentPolicy` to the
//! `PersonalDayAssessment::assess` default entry point.
//!
//! Source spec: `docs/architecture/personal-day-audit/SCORING-POLICY-V2-SPEC.md`
//! Bead: `amlich-31oa`.
//!
//! ## Why this exists
//!
//! The v2 policy (`amlich-7bm4`) was designed to be a drop-in
//! replacement for the v1 builder: it preserves the existing
//! `PersonalDayAssessment` envelope, contribution IDs, axis scores,
//! and decision buckets. The v2.1 intent-aware variant
//! (`amlich-lxu3`) and v2.2 interaction-aware variant (`amlich-47wn`)
//! then layered intentional divergences on top of v1, all of which
//! are reviewed in their respective test suites. The stability gate
//! collects those reviews plus four additional stability checks
//! (sensitivity, metamorphic, missing-data/veto invariants, and
//! API/TUI/desktop compatibility) into a single
//! [`StabilityReport`] that a maintainer can read to decide whether
//! to flip the default.
//!
//! ## Design
//!
//! - Every gate is a self-contained, additive test surface. The
//!   gate runner is a thin orchestrator: each test in
//!   `tests/assessment_v2_stability_gate.rs` reports into the report
//!   and the test pass/fail is the gate pass/fail.
//! - The report is `Serialize`/`Deserialize` so it can be dumped to
//!   JSON for CI consumption and archived as a build artifact.
//! - [`current_default_policy_version`] stays pinned to `"v1"`
//!   until a maintainer explicitly flips the default; the gate
//!   authorises the flip but never triggers it. This is the
//!   "promotion status is reported without silently changing the
//!   default" half of the bead's acceptance criteria.
//!
//! ## The six gates
//!
//! - [`StabilityGate::Parity`] — v1/v2 outputs match by design and
//!   every intentional divergence is reviewed in the divergence
//!   index (`assessment_v2_divergence_index`).
//! - [`StabilityGate::Sensitivity`] — every policy weight, perturbed
//!   ±10% and ±20%, does not flip a decision bucket on a
//!   representative fixture grid.
//! - [`StabilityGate::Metamorphic`] — duplicate evidence and
//!   unrelated features cannot perturb a score.
//! - [`StabilityGate::MissingData`] — unavailable features stay
//!   `None`, never become a neutral zero, and never leak into
//!   contributions or axis subtotals.
//! - [`StabilityGate::Veto`] — a named hard veto always wins over
//!   favorable weights and is not flipped by a perturbation.
//! - [`StabilityGate::Compatibility`] — the v1 DTO wire shape, v1
//!   TUI surface, and v1 desktop surface remain unchanged; the v2
//!   path is strictly additive.

use serde::{Deserialize, Serialize};

use crate::assessment::AssessmentPolicy;

/// Identifier for a stability gate. Stable string form for the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilityGate {
    Parity,
    Sensitivity,
    Metamorphic,
    MissingData,
    Veto,
    Compatibility,
}

impl StabilityGate {
    /// All six gates in the canonical order the report emits.
    pub const ALL: [Self; 6] = [
        Self::Parity,
        Self::Sensitivity,
        Self::Metamorphic,
        Self::MissingData,
        Self::Veto,
        Self::Compatibility,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::Sensitivity => "sensitivity",
            Self::Metamorphic => "metamorphic",
            Self::MissingData => "missing_data",
            Self::Veto => "veto",
            Self::Compatibility => "compatibility",
        }
    }
}

/// Per-gate verdict. `Pass` is the only state that authorises
/// promotion; `Warn` is a soft signal (e.g. a sensitivity edge case
/// that flipped a single fixture's bucket on a single axis), and
/// `Fail` is a hard blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateStatus {
    Pass,
    Warn,
    Fail,
}

impl GateStatus {
    /// True when the gate verdict does not block promotion. `Warn`
    /// still counts as pass-with-caveat: it produces a detail entry
    /// in the report so a reviewer can see what happened, but does
    /// not block the candidate from being flipped to default.
    pub fn allows_promotion(self) -> bool {
        matches!(self, Self::Pass | Self::Warn)
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Warn => "warn",
            Self::Fail => "fail",
        }
    }
}

/// Per-gate detail entry. Carries a short label, the observed
/// measurement, and the expected one so a reviewer can decide
/// whether a `Warn` deserves a closer look.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateDetail {
    pub label: String,
    pub observed: String,
    pub expected: String,
}

/// Per-gate result. Each gate contributes one of these to the
/// top-level [`StabilityReport`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GateResult {
    pub gate: StabilityGate,
    pub status: GateStatus,
    /// Short human-readable summary suitable for a CI log line.
    pub summary: String,
    /// Detail entries. Empty for a clean `Pass`.
    pub details: Vec<GateDetail>,
}

/// Top-level machine-readable report.
///
/// Fields:
///
/// - `policy_id` / `policy_version` — the v2.x candidate the gate
///   evaluated.
/// - `baseline_policy_version` — the v1 default the candidate would
///   replace, recorded so the report is self-describing.
/// - `gates` — one [`GateResult`] per [`StabilityGate`], in the
///   canonical `ALL` order.
/// - `overall_status` — derived from `gates`; the strictest status
///   wins (`Fail` > `Warn` > `Pass`).
/// - `can_promote` — true when `overall_status.allows_promotion()`.
/// - `promotion_blocker` — human-readable note on the first `Fail`
///   (or any `Warn`, if the gate is configured to escalate), or
///   `None` when the gate is clean.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StabilityReport {
    pub policy_id: String,
    pub candidate_policy_version: String,
    pub baseline_policy_version: String,
    pub overall_status: GateStatus,
    pub gates: Vec<GateResult>,
    pub can_promote: bool,
    pub promotion_blocker: Option<String>,
}

impl StabilityReport {
    /// Build a new empty report for the given candidate. Gates are
    /// appended via [`push_gate`](Self::push_gate) as the test suite
    /// runs; the report is finalised via [`finalise`](Self::finalise).
    pub fn new(candidate_policy: &AssessmentPolicy) -> Self {
        Self {
            policy_id: candidate_policy.policy_id().to_string(),
            candidate_policy_version: candidate_policy.policy_version().to_string(),
            baseline_policy_version: crate::assessment::ASSESSMENT_POLICY_VERSION.to_string(),
            overall_status: GateStatus::Pass,
            gates: Vec::with_capacity(StabilityGate::ALL.len()),
            can_promote: false,
            promotion_blocker: None,
        }
    }

    /// Append a gate result. Order is the caller's responsibility;
    /// the final report should follow [`StabilityGate::ALL`].
    pub fn push_gate(&mut self, result: GateResult) {
        self.gates.push(result);
    }

    /// Aggregate `overall_status`, set `can_promote`, and populate
    /// `promotion_blocker`. Idempotent: calling twice is safe.
    pub fn finalise(&mut self) {
        let mut overall = GateStatus::Pass;
        let mut blocker: Option<String> = None;
        for result in &self.gates {
            if result.status == GateStatus::Fail {
                overall = GateStatus::Fail;
                if blocker.is_none() {
                    blocker = Some(format!(
                        "{} gate failed: {}",
                        result.gate.as_str(),
                        result.summary
                    ));
                }
            } else if result.status == GateStatus::Warn && overall == GateStatus::Pass {
                overall = GateStatus::Warn;
            }
        }
        self.overall_status = overall;
        self.can_promote = self.overall_status.allows_promotion();
        // `Warn` does not block promotion but is recorded as a soft
        // note so a reviewer can see what happened.
        if self.can_promote && blocker.is_none() {
            if let Some(warn) = self.gates.iter().find(|g| g.status == GateStatus::Warn) {
                blocker = Some(format!(
                    "{} gate has a warning: {}",
                    warn.gate.as_str(),
                    warn.summary
                ));
            }
        }
        self.promotion_blocker = blocker;
    }
}

/// `serde_json` is already in the dependency graph (used by the
/// existing v2 tests via `serde_json::to_string` round-trips). The
/// feature is not added here; tests opt in by depending on the
/// workspace-level `serde_json` dep.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::assessment::AssessmentPolicy;

    #[test]
    fn gate_status_allows_promotion() {
        assert!(GateStatus::Pass.allows_promotion());
        assert!(GateStatus::Warn.allows_promotion());
        assert!(!GateStatus::Fail.allows_promotion());
    }

    #[test]
    fn gate_status_round_trips_through_serde() {
        for status in [GateStatus::Pass, GateStatus::Warn, GateStatus::Fail] {
            let json = serde_json::to_string(&status).expect("serialize");
            let back: GateStatus = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(status, back);
        }
    }

    #[test]
    fn stability_gate_string_form_is_stable() {
        for gate in StabilityGate::ALL {
            assert!(!gate.as_str().is_empty());
            assert_eq!(gate.as_str(), gate.as_str().to_ascii_lowercase().as_str());
        }
    }

    #[test]
    fn empty_report_finalises_to_pass_with_no_blocker() {
        let policy = AssessmentPolicy::baseline_v2();
        let mut report = StabilityReport::new(&policy);
        report.finalise();
        assert_eq!(report.overall_status, GateStatus::Pass);
        assert!(report.can_promote);
        assert!(report.promotion_blocker.is_none());
    }

    #[test]
    fn single_failing_gate_blocks_promotion() {
        let policy = AssessmentPolicy::baseline_v2();
        let mut report = StabilityReport::new(&policy);
        report.push_gate(GateResult {
            gate: StabilityGate::Sensitivity,
            status: GateStatus::Fail,
            summary: "kua_direction_travel weight at -20% flipped 2 fixtures".to_string(),
            details: vec![],
        });
        report.finalise();
        assert_eq!(report.overall_status, GateStatus::Fail);
        assert!(!report.can_promote);
        assert!(report
            .promotion_blocker
            .as_deref()
            .unwrap_or("")
            .contains("sensitivity"));
    }

    #[test]
    fn warn_does_not_block_promotion_but_is_recorded() {
        let policy = AssessmentPolicy::baseline_v2();
        let mut report = StabilityReport::new(&policy);
        report.push_gate(GateResult {
            gate: StabilityGate::Metamorphic,
            status: GateStatus::Warn,
            summary: "duplicate taboos inflated the Wedding fixture score by 0.02".to_string(),
            details: vec![],
        });
        report.finalise();
        assert_eq!(report.overall_status, GateStatus::Warn);
        assert!(report.can_promote);
        assert!(report
            .promotion_blocker
            .as_deref()
            .unwrap_or("")
            .contains("metamorphic"));
    }

    #[test]
    fn fail_outranks_warn() {
        let policy = AssessmentPolicy::baseline_v2();
        let mut report = StabilityReport::new(&policy);
        report.push_gate(GateResult {
            gate: StabilityGate::Metamorphic,
            status: GateStatus::Warn,
            summary: "soft warning".to_string(),
            details: vec![],
        });
        report.push_gate(GateResult {
            gate: StabilityGate::Sensitivity,
            status: GateStatus::Fail,
            summary: "hard fail".to_string(),
            details: vec![],
        });
        report.finalise();
        assert_eq!(report.overall_status, GateStatus::Fail);
        assert!(!report.can_promote);
        // Blocker cites the failing gate, not the warning.
        assert!(report
            .promotion_blocker
            .as_deref()
            .unwrap_or("")
            .contains("sensitivity"));
    }

    #[test]
    fn report_round_trips_through_serde() {
        let policy = AssessmentPolicy::baseline_v2();
        let mut report = StabilityReport::new(&policy);
        report.push_gate(GateResult {
            gate: StabilityGate::Parity,
            status: GateStatus::Pass,
            summary: "v1/v2 parity preserved across 45-fixture grid".to_string(),
            details: vec![],
        });
        report.push_gate(GateResult {
            gate: StabilityGate::Sensitivity,
            status: GateStatus::Pass,
            summary: "all weights stable at +/-10% and +/-20%".to_string(),
            details: vec![GateDetail {
                label: "intent_axis_weight/wedding/annual_pressure".to_string(),
                observed: "no flip".to_string(),
                expected: "no flip".to_string(),
            }],
        });
        report.finalise();
        let json = serde_json::to_string(&report).expect("serialize report");
        let back: StabilityReport = serde_json::from_str(&json).expect("deserialize report");
        assert_eq!(report, back);
    }
}
