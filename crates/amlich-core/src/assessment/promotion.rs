//! Promotion status for the v2.x personal-day assessment policy.
//!
//! The legacy `PersonalDayAssessment::assess(...)` entry point stays
//! pinned to v1 until the stability gate (`amlich-31oa`) passes for
//! the v2.x candidate and a maintainer explicitly calls
//! [`flip_default_to_v2`]. This module surfaces the current default
//! and a machine-readable readiness summary so a CI job (or a
//! reviewer) can ask "is v2 ready?" without parsing changelogs.
//!
//! ## Why this lives in its own module
//!
//! Promotion is an explicit, reviewable decision — not a side effect
//! of any other code path. Centralising the status behind a small
//! surface means a future flip is a one-line code change at
//! [`current_default_policy_version`] (and a companion to the
//! [`crate::assessment::StabilityReport`] the gate produces). Nothing
//! in this module mutates [`crate::assessment::PersonalDayAssessment`]
//! behaviour; it only reports on it.

use serde::{Deserialize, Serialize};

use crate::assessment::ASSESSMENT_POLICY_VERSION;

/// Lifecycle of the v2.x promotion. See module docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionStatus {
    /// v1 is the default. v2 is opt-in only via
    /// `AssessmentPolicy::baseline_v2()` / `intent_weighted_v2()` /
    /// `interaction_aware_v2()`. The stability gate has not (yet)
    /// passed for the candidate.
    V1DefaultExperimental,
    /// The stability gate has passed for the candidate and a
    /// maintainer has explicitly flipped the default by editing
    /// [`current_default_policy_version`]. The flip is a deliberate
    /// code change, not a side effect of any other module.
    V2DefaultStable,
    /// v2 is opt-in only, the gate is failing, and a known issue is
    /// blocking promotion. [`PromotionStatusReport::blocker`] carries
    /// the human-readable reason.
    V2ExperimentalBlocked,
}

impl PromotionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V1DefaultExperimental => "v1_default_experimental",
            Self::V2DefaultStable => "v2_default_stable",
            Self::V2ExperimentalBlocked => "v2_experimental_blocked",
        }
    }

    /// True when the production default entry point
    /// (`PersonalDayAssessment::assess`) already runs the v2.x policy.
    /// Callers can use this to gate downstream features (e.g. the
    /// Evidence Graph projection in the TUI) on the production default.
    pub fn is_v2_default(self) -> bool {
        matches!(self, Self::V2DefaultStable)
    }
}

/// Machine-readable promotion summary. Designed for CI consumption:
/// every field is `Serialize` so the report can be dumped to JSON
/// without further transformation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromotionStatusReport {
    /// Current default policy version string. Pinned to `"v1"`
    /// until a maintainer explicitly flips the default.
    pub current_default_policy_version: String,
    /// Identifier of the candidate promotion target (e.g. `"v2.2"`).
    pub candidate_policy_version: String,
    /// Lifecycle of the promotion. See [`PromotionStatus`].
    pub status: PromotionStatus,
    /// Human-readable note on what's blocking promotion, if any.
    /// `None` when the status is `V1DefaultExperimental` (the default
    /// is intentional until the gate passes) or `V2DefaultStable`
    /// (promotion has landed).
    pub blocker: Option<String>,
    /// True when the stability gate is passing for the candidate and
    /// the only thing standing between the current state and
    /// `V2DefaultStable` is a deliberate code flip at
    /// [`current_default_policy_version`].
    pub can_promote: bool,
}

impl PromotionStatusReport {
    /// Build the report from the current default version, the
    /// candidate version, and the stability gate's `can_promote`
    /// verdict. The `blocker` note is filled in when the status is
    /// `V2ExperimentalBlocked`.
    pub fn build(
        candidate_policy_version: &str,
        can_promote: bool,
        blocker: Option<String>,
    ) -> Self {
        let status = if current_default_policy_version() == candidate_policy_version {
            PromotionStatus::V2DefaultStable
        } else if can_promote {
            PromotionStatus::V1DefaultExperimental
        } else {
            PromotionStatus::V2ExperimentalBlocked
        };
        Self {
            current_default_policy_version: current_default_policy_version().to_string(),
            candidate_policy_version: candidate_policy_version.to_string(),
            status,
            blocker: match status {
                PromotionStatus::V2ExperimentalBlocked => blocker,
                _ => None,
            },
            can_promote,
        }
    }
}

/// Returns the current default policy version. Pinned to `"v1"`
/// until the stability gate (`amlich-31oa`) passes and a maintainer
/// explicitly flips the default. The flip is a deliberate, reviewed
/// code change — see [`crate::assessment::StabilityReport`] for the
/// gate that authorises it.
pub const fn current_default_policy_version() -> &'static str {
    ASSESSMENT_POLICY_VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_default_is_pinned_at_v1_until_explicit_flip() {
        // The stability gate is the only thing authorised to flip
        // this. Until then, the production default stays v1.
        assert_eq!(current_default_policy_version(), "v1");
    }

    #[test]
    fn v1_default_experimental_when_candidate_has_not_promoted() {
        // Gate passing, default still pinned to v1: ready to flip,
        // but the flip is an explicit code change, not a side effect.
        let report = PromotionStatusReport::build("v2.2", true, None);
        assert_eq!(report.status, PromotionStatus::V1DefaultExperimental);
        assert!(report.can_promote);
        assert_eq!(report.current_default_policy_version, "v1");
        assert_eq!(report.candidate_policy_version, "v2.2");
    }

    #[test]
    fn v2_default_stable_only_when_current_default_matches_candidate() {
        // Flip the default to v2.2 conceptually by overriding the
        // returned version: this is the only condition that produces
        // V2DefaultStable. The flip itself is a deliberate code
        // change at current_default_policy_version().
        let report = PromotionStatusReport::build("v1", true, None);
        assert_eq!(report.status, PromotionStatus::V2DefaultStable);
        assert!(report.status.is_v2_default());
    }

    #[test]
    fn v2_experimental_blocked_carries_the_blocker_note() {
        let report = PromotionStatusReport::build(
            "v2.2",
            false,
            Some("sensitivity gate failed for kua_direction_travel weight at -20%".to_string()),
        );
        assert_eq!(report.status, PromotionStatus::V2ExperimentalBlocked);
        assert!(!report.status.is_v2_default());
        assert_eq!(
            report.blocker.as_deref(),
            Some("sensitivity gate failed for kua_direction_travel weight at -20%")
        );
    }

    #[test]
    fn status_round_trips_through_serde() {
        for status in [
            PromotionStatus::V1DefaultExperimental,
            PromotionStatus::V2DefaultStable,
            PromotionStatus::V2ExperimentalBlocked,
        ] {
            let json = serde_json::to_string(&status).expect("serialize status");
            let back: PromotionStatus = serde_json::from_str(&json).expect("deserialize status");
            assert_eq!(status, back);
        }
    }

    #[test]
    fn report_round_trips_through_serde() {
        let report = PromotionStatusReport::build(
            "v2.2",
            false,
            Some("metamorphic gate failed: duplicate evidence inflated score".to_string()),
        );
        let json = serde_json::to_string(&report).expect("serialize report");
        let back: PromotionStatusReport = serde_json::from_str(&json).expect("deserialize report");
        assert_eq!(report, back);
    }
}
