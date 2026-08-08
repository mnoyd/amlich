//! Intent-aware axis weights for the v2.1 personal-day assessment policy.
//!
//! Source spec: `docs/architecture/personal-day-audit/SCORING-POLICY-V2-SPEC.md`
//! Bead: `amlich-lxu3`.
//!
//! `baseline_v2` (the v1-parity seam from `amlich-7bm4`) averages the four
//! scored axes with equal weights. The v2.1 policy introduces a sparse,
//! policy-versioned intent×axis weight table so the final decision projection
//! reflects what each consultation intent actually emphasizes:
//!
//! ```text
//! decision_score = Σ(intent_axis_weight[intent, axis] × axis_score[axis])
//! ```
//!
//! over the *available* scored axes — weights of unavailable axes are
//! excluded and the remaining weights renormalize to sum to 1.0 so a
//! capability gap cannot silently inflate the score (the same
//! "unavailable is not zero" contract locked in `amlich-7bm4`).
//!
//! ## Experimental status
//!
//! These weights are an explicitly experimental product policy. They are
//! NOT a claim of canonical traditional truth. The numbers below are
//! reviewable; the stability gate (`amlich-31oa`) sensitivity-tests every
//! entry at ±10% and ±20% before v2.x is promoted to default. Each entry
//! here carries a short rationale so reviewers can challenge it without
//! grepping for magic numbers.
//!
//! ## Design rules
//!
//! - Every weight is a multiple of 0.05 for reviewability.
//! - Each intent's four scored-axis weights sum to 1.0.
//! - `EvidenceCoverage` is intentionally absent: it is reported as an axis
//!   for explanation but never participates in the decision aggregation.
//! - The table is sparse in the sense that adding a new intent or
//!   re-tuning an entry is a single, versioned edit — no other policy
//!   code changes.

use crate::{
    advisory::ConsultationIntent, assessment::policy::ASSESSMENT_POLICY_V2_1_VERSION,
    assessment::AssessmentAxis,
};

/// Per-intent weights for the four scored axes, used by the v2.1 policy
/// to project axis subtotals into a single decision score. All four
/// weights sum to 1.0; the v2.1 constructor enforces that invariant at
/// build time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntentAxisWeights {
    pub intent: ConsultationIntent,
    pub generic_day_quality: f32,
    pub intent_fit: f32,
    pub personal_alignment: f32,
    pub annual_pressure: f32,
}

impl IntentAxisWeights {
    /// Weight the v2.1 policy applies to a given scored axis under this
    /// intent. Returns `None` for `EvidenceCoverage` (which is reported
    /// but never participates in the decision aggregation) and for any
    /// non-scored axis.
    pub fn weight_for(self, axis: AssessmentAxis) -> Option<f32> {
        match axis {
            AssessmentAxis::GenericDayQuality => Some(self.generic_day_quality),
            AssessmentAxis::IntentFit => Some(self.intent_fit),
            AssessmentAxis::PersonalAlignment => Some(self.personal_alignment),
            AssessmentAxis::AnnualPressure => Some(self.annual_pressure),
            AssessmentAxis::EvidenceCoverage => None,
        }
    }

    /// Sum of the four scored-axis weights. Must be 1.0 within f32
    /// tolerance for every entry; the table constructor asserts it.
    pub fn total(self) -> f32 {
        self.generic_day_quality + self.intent_fit + self.personal_alignment + self.annual_pressure
    }

    /// Return a sensitivity-perturbed copy of this entry: each of the
    /// four scored-axis weights is multiplied by `factor` and rounded
    /// to the nearest 0.05 step, then renormalized so the entry still
    /// sums to 1.0 (the policy invariant). Test-only API used by the
    /// stability gate (`amlich-31oa`).
    pub(crate) fn perturbed(self, factor: f32) -> Self {
        let step = 0.05_f32;
        let round_step = |w: f32| {
            let scaled = w * factor;
            let units = (scaled / step).round();
            (units * step).max(0.0)
        };
        let mut entry = IntentAxisWeights {
            intent: self.intent,
            generic_day_quality: round_step(self.generic_day_quality),
            intent_fit: round_step(self.intent_fit),
            personal_alignment: round_step(self.personal_alignment),
            annual_pressure: round_step(self.annual_pressure),
        };
        let total = entry.total();
        if total > 0.0 {
            entry.generic_day_quality /= total;
            entry.intent_fit /= total;
            entry.personal_alignment /= total;
            entry.annual_pressure /= total;
        }
        entry
    }
}

/// Sparse, policy-versioned table of per-intent axis weights. Construct
/// via [`INTENT_AXIS_WEIGHTS_V2_1`] for the v2.1 policy.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntentAxisWeightTable {
    /// Policy version this weight table is bound to. Must match the
    /// [`crate::assessment::AssessmentPolicy::policy_version`] of the
    /// policy that consumes it.
    pub policy_version: &'static str,
    pub entries: &'static [IntentAxisWeights],
}

impl IntentAxisWeightTable {
    /// Look up the weights for a given intent. Panics if the table is
    /// malformed (an intent is missing) — the table is a static, reviewed
    /// constant, so a missing entry is a build-time programmer error
    /// rather than a runtime input.
    pub fn weights_for(self, intent: ConsultationIntent) -> IntentAxisWeights {
        for entry in self.entries {
            if entry.intent == intent {
                return *entry;
            }
        }
        panic!(
            "intent_axis_weight table v{} is missing weights for intent {:?}",
            self.policy_version, intent
        )
    }

    /// Return a sensitivity-perturbed copy of this table as a
    /// `'static` reference. Test-only API used by the stability gate
    /// (`amlich-31oa`) to ±10% and ±20% perturb every entry.
    ///
    /// Each entry's four scored-axis weights are multiplied by `factor`
    /// (rounded to the nearest 0.05 step for reviewability), then
    /// renormalized so the per-entry sum is 1.0. The leaked allocation
    /// is bounded by the number of perturbation combinations the gate
    /// runs (a few dozen, total ≈ a few KB).
    pub(crate) fn perturbed_to_static(self, factor: f32) -> &'static Self {
        let perturbed_entries: Vec<IntentAxisWeights> =
            self.entries.iter().map(|e| e.perturbed(factor)).collect();
        let perturbed_slice: &'static [IntentAxisWeights] =
            Box::leak(perturbed_entries.into_boxed_slice());
        let table = IntentAxisWeightTable {
            policy_version: self.policy_version,
            entries: perturbed_slice,
        };
        Box::leak(Box::new(table))
    }
}

/// The v2.1 intent×axis weight table.
///
/// Rationale per intent (multiple of 0.05, four axes sum to 1.0):
///
/// - **Wedding** — personal compatibility (bazi, lục xung) and Kim Lâu
///   are the traditional blockers; intent-fit (good giờ) and generic
///   quality round it out.
/// - **MovingHouse** — Hoàng Ốc (annual) is the famous construction /
///   move blocker; intent-fit (good giờ) matters; bazi and generic are
///   secondary.
/// - **OpeningBusiness** — generic day fortune and good giờ dominate a
///   public-facing launch; personal alignment and annual pressure are
///   secondary screens.
/// - **ContractSigning** — same shape as OpeningBusiness: a public
///   commitment benefits from a generally favorable day and good giờ.
/// - **Travel** — intent-fit (Kua direction, good giờ for travel) is
///   the dominant signal; generic quality matters; personal and annual
///   are lighter screens.
/// - **Burial** — most traditional intent: Tang Môn / Hạn (annual) and
///   personal alignment carry the most weight; good giờ is secondary.
/// - **Renovation** — Hoàng Ốc (annual) is the famous construction /
///   renovation blocker, mirroring MovingHouse.
/// - **Medical** — generic day quality and good giờ dominate a
///   non-ceremonial consultation; personal and annual are light screens.
/// - **Prayer** — good giờ (intent-fit) and generic day quality dominate
///   a devotional act; personal and annual are light screens.
pub const INTENT_AXIS_WEIGHTS_V2_1: IntentAxisWeightTable = IntentAxisWeightTable {
    policy_version: ASSESSMENT_POLICY_V2_1_VERSION,
    entries: &[
        IntentAxisWeights {
            intent: ConsultationIntent::Wedding,
            generic_day_quality: 0.20,
            intent_fit: 0.25,
            personal_alignment: 0.30,
            annual_pressure: 0.25,
        },
        IntentAxisWeights {
            intent: ConsultationIntent::MovingHouse,
            generic_day_quality: 0.20,
            intent_fit: 0.25,
            personal_alignment: 0.20,
            annual_pressure: 0.35,
        },
        IntentAxisWeights {
            intent: ConsultationIntent::OpeningBusiness,
            generic_day_quality: 0.30,
            intent_fit: 0.30,
            personal_alignment: 0.20,
            annual_pressure: 0.20,
        },
        IntentAxisWeights {
            intent: ConsultationIntent::ContractSigning,
            generic_day_quality: 0.30,
            intent_fit: 0.30,
            personal_alignment: 0.20,
            annual_pressure: 0.20,
        },
        IntentAxisWeights {
            intent: ConsultationIntent::Travel,
            generic_day_quality: 0.25,
            intent_fit: 0.40,
            personal_alignment: 0.15,
            annual_pressure: 0.20,
        },
        IntentAxisWeights {
            intent: ConsultationIntent::Burial,
            generic_day_quality: 0.15,
            intent_fit: 0.20,
            personal_alignment: 0.30,
            annual_pressure: 0.35,
        },
        IntentAxisWeights {
            intent: ConsultationIntent::Renovation,
            generic_day_quality: 0.20,
            intent_fit: 0.25,
            personal_alignment: 0.20,
            annual_pressure: 0.35,
        },
        IntentAxisWeights {
            intent: ConsultationIntent::Medical,
            generic_day_quality: 0.35,
            intent_fit: 0.35,
            personal_alignment: 0.15,
            annual_pressure: 0.15,
        },
        IntentAxisWeights {
            intent: ConsultationIntent::Prayer,
            generic_day_quality: 0.30,
            intent_fit: 0.40,
            personal_alignment: 0.15,
            annual_pressure: 0.15,
        },
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    /// Every ConsultationIntent variant has an entry in the v2.1 table.
    /// Adding a new intent to the enum without adding weights here is a
    /// programmer error this test catches at build time.
    #[test]
    fn v2_1_table_covers_every_intent() {
        let all_intents = [
            ConsultationIntent::Wedding,
            ConsultationIntent::MovingHouse,
            ConsultationIntent::OpeningBusiness,
            ConsultationIntent::ContractSigning,
            ConsultationIntent::Travel,
            ConsultationIntent::Burial,
            ConsultationIntent::Renovation,
            ConsultationIntent::Medical,
            ConsultationIntent::Prayer,
        ];
        for intent in all_intents {
            let weights = INTENT_AXIS_WEIGHTS_V2_1.weights_for(intent);
            assert_eq!(weights.intent, intent);
        }
        assert_eq!(
            INTENT_AXIS_WEIGHTS_V2_1.entries.len(),
            9,
            "v2.1 table must carry one entry per ConsultationIntent variant"
        );
    }

    /// Every v2.1 entry's four scored-axis weights sum to 1.0 within f32
    /// tolerance. A table entry that doesn't sum to 1.0 would silently
    /// bias the decision score; this guard catches it at test time.
    #[test]
    fn v2_1_weights_sum_to_one_per_intent() {
        for entry in INTENT_AXIS_WEIGHTS_V2_1.entries {
            let total = entry.total();
            assert!(
                (total - 1.0).abs() < 1e-6,
                "v2.1 weights for {:?} sum to {total}, expected 1.0",
                entry.intent
            );
        }
    }

    /// Every weight is a multiple of 0.05 — a deliberate reviewability
    /// constraint so reviewers can sanity-check the rationales without
    /// chasing magic decimals.
    #[test]
    fn v2_1_weights_are_reviewable_multiples_of_0_05() {
        let step = 0.05_f32;
        for entry in INTENT_AXIS_WEIGHTS_V2_1.entries {
            for axis in [
                AssessmentAxis::GenericDayQuality,
                AssessmentAxis::IntentFit,
                AssessmentAxis::PersonalAlignment,
                AssessmentAxis::AnnualPressure,
            ] {
                let w = entry.weight_for(axis).expect("scored axis weight");
                let units = (w / step).round();
                assert!(
                    (w - units * step).abs() < 1e-6,
                    "{:?} weight for {:?} is {w}, not a multiple of 0.05",
                    entry.intent,
                    axis
                );
            }
        }
    }

    /// The EvidenceCoverage axis is deliberately excluded from the
    /// decision aggregation; the table reflects that.
    #[test]
    fn v2_1_weights_exclude_evidence_coverage() {
        for entry in INTENT_AXIS_WEIGHTS_V2_1.entries {
            assert_eq!(
                entry.weight_for(AssessmentAxis::EvidenceCoverage),
                None,
                "EvidenceCoverage must not carry an intent-specific weight"
            );
        }
    }

    /// The v2.1 table is NOT the equal-weight baseline (0.25/0.25/0.25/0.25).
    /// If a future edit collapses the table back to equal weights, the
    /// whole point of amlich-lxu3 is lost — this test guards against that
    /// regression by checking at least one intent differs from the
    /// baseline.
    #[test]
    fn v2_1_table_diverges_from_equal_weight_baseline() {
        let equal = [0.25_f32; 4];
        let mut any_divergence = false;
        for entry in INTENT_AXIS_WEIGHTS_V2_1.entries {
            let w = [
                entry.generic_day_quality,
                entry.intent_fit,
                entry.personal_alignment,
                entry.annual_pressure,
            ];
            if w != equal {
                any_divergence = true;
                break;
            }
        }
        assert!(
            any_divergence,
            "v2.1 table collapsed back to equal weights; intent-awareness is gone"
        );
    }

    /// The table's policy_version is pinned to the v2.1 constant so any
    /// consumer can trace which policy produced a decision projection.
    #[test]
    fn v2_1_table_is_versioned() {
        assert_eq!(INTENT_AXIS_WEIGHTS_V2_1.policy_version, "v2.1");
    }
}
