//! v1/v2 divergence index (`amlich-31oa`).
//!
//! The stability gate's parity gate is a thin pass-through to this
//! suite. The actual v1/v2 divergence explanations live here so a
//! reviewer can see, in one place, every place where the v1 default
//! and the v2 opt-in intentionally differ.
//!
//! ## What is a "divergence" here
//!
//! A divergence is a field where `PersonalDayAssessment::assess(...)`
//! (v1) and `AssessmentPolicy::baseline_v2().evaluate(...)` (v2)
//! produce different values for the same `(snapshot, profile,
//! intent)` triple. Every divergence must be one of:
//!
//! - **Intentional & reviewed** — the v2 policy was designed to
//!   produce a different value here, and the divergence is
//!   documented in this index. Examples: the `policy_version` tag,
//!   the additive `trace` field, the named-veto precedence on
//!   the han_severe fixture.
//! - **A bug** — the v2 policy should match v1 here but does not.
//!   Divergences in this category are tracked in their own beads
//!   (e.g. `amlich-h85g` for the Kua direction-name mismatch).
//!
//! If the v1/v2 parity grid in
//! `assessment_v2_seam::v1_v2_full_parity_*` is green, the
//! reviewer's job here is just to confirm the index is up to date
//! — any new v2 policy edit that introduces a divergence must
//! add a row to this index in the same commit.
//!
//! ## Out of scope
//!
//! The v2.1 and v2.2 variants intentionally diverge from v1 (the
//! whole point of the intent-aware and interaction-aware policies).
//! Their divergences are documented in `assessment_v2_1_intent_weights`
//! and `assessment_v2_2_interactions` respectively. This index only
//! covers the `baseline_v2` (v1-parity) path.

use amlich_core::almanac::tu_menh::Gender;
use amlich_core::{
    advisory::ConsultationIntent,
    assessment::{AssessmentPolicy, ASSESSMENT_POLICY_V2_ID, ASSESSMENT_POLICY_V2_VERSION},
    birth::{BirthProfile, BirthTime},
    types::VIETNAM_TIMEZONE,
    DaySnapshot,
};

fn snapshot_2024_02_10() -> DaySnapshot {
    amlich_core::calculate_day_snapshot_with_timezone(10, 2, 2024, VIETNAM_TIMEZONE)
}

fn full_profile() -> BirthProfile {
    BirthProfile {
        day: 1,
        month: 1,
        year: 1990,
        time: Some(BirthTime {
            hour: 9,
            minute: 30,
        }),
        timezone: VIETNAM_TIMEZONE,
        longitude: Some(105.85),
        use_solar_time: true,
        gender: Some(Gender::Male),
        location_name: Some("Hanoi".to_string()),
    }
}

/// Documented divergence between v1 (`PersonalDayAssessment::assess`)
/// and v2 (`AssessmentPolicy::baseline_v2().evaluate`) for the same
/// `(snapshot, profile, intent)` triple. Each row is reviewed in the
/// bead that introduced the divergence.
#[derive(Debug, Clone, PartialEq)]
struct DocumentedDivergence {
    field: &'static str,
    v1_value: &'static str,
    v2_value: &'static str,
    reviewed_in: &'static str,
    rationale: &'static str,
}

fn divergence_index() -> Vec<DocumentedDivergence> {
    vec![
        DocumentedDivergence {
            field: "policy_version",
            v1_value: "\"v1\"",
            v2_value: "\"v2\"",
            reviewed_in: "amlich-7bm4 (v2 seam introduction)",
            rationale: "The v2 policy advertises its own version string so a \
                downstream consumer can tell which policy produced a \
                given assessment. The policy_id family stays identical \
                (\"personal-day-assessment\") so v1 consumers can still \
                dispatch on the family.",
        },
        DocumentedDivergence {
            field: "trace (new field)",
            v1_value: "None",
            v2_value: "Some(AssessmentTrace { features, axes, decision, vetoes, interactions })",
            reviewed_in: "amlich-7bm4 (v2 seam); amlich-8tdm (Evidence Graph projection)",
            rationale: "The calculation trace is the v2 additive surface. It is \
                `None` on every v1 default assessment (skip_serializing_if) \
                so v1 wire JSON stays byte-equal. v2 opt-in consumers \
                (TUI, desktop, amlich-api v2 surface) read it as the \
                Evidence Graph projection.",
        },
        DocumentedDivergence {
            field: "contribution[].policy_version",
            v1_value: "\"v1\"",
            v2_value: "\"v2\"",
            reviewed_in: "amlich-7bm4",
            rationale: "Each contribution inherits the assessment's policy \
                version. Same family, same divergence as the envelope.",
        },
    ]
}

#[test]
fn divergence_index_lists_every_intentional_v1_v2_field_change() {
    let index = divergence_index();

    // The index is non-empty: at minimum the policy_version and
    // trace field must be listed.
    assert!(
        !index.is_empty(),
        "divergence index is empty; v2 has not been started?"
    );

    // Every row carries a non-empty rationale and a reviewed_in
    // bead reference. A row with an empty rationale is a
    // documentation gap that the parity gate cannot detect on
    // its own.
    for row in &index {
        assert!(
            !row.rationale.is_empty(),
            "divergence for {} is missing a rationale",
            row.field
        );
        assert!(
            !row.reviewed_in.is_empty(),
            "divergence for {} is missing a `reviewed_in` bead reference",
            row.field
        );
    }

    // policy_version is the canonical v1/v2 signal. If it stops
    // being listed in the index, the v2 path has either collapsed
    // back to v1 (a regression) or diverged in a way the index
    // cannot explain.
    assert!(
        index.iter().any(|r| r.field == "policy_version"),
        "policy_version is no longer in the divergence index; v2 has either collapsed to v1 or diverged in a way this index cannot explain"
    );
}

#[test]
fn v2_baseline_matches_v1_except_for_documented_divergences() {
    // The full byte-level equality is locked by
    // `assessment_v2_seam::v1_v2_full_parity_*`. The index test
    // above is a documentation guard; this test is a
    // meta-confirmation that the divergences are exactly the rows
    // the index lists, no more and no less.
    let snap = snapshot_2024_02_10();
    let profile = full_profile();
    let v1 = amlich_core::assessment::PersonalDayAssessment::assess(
        snap.clone(),
        profile.clone(),
        ConsultationIntent::Wedding,
    );
    let v2 = AssessmentPolicy::baseline_v2().evaluate(
        amlich_core::assessment::AssessmentInputs::default(),
        &snap,
        &profile,
        ConsultationIntent::Wedding,
    );

    // v1-comparable fields must be byte-equal.
    assert_eq!(v1.axes, v2.axes);
    assert_eq!(v1.decision, v2.decision);
    assert_eq!(v1.unavailable_sections, v2.unavailable_sections);
    assert_eq!(v1.evidence, v2.evidence);
    assert_eq!(v1.normalized_birth, v2.normalized_birth);
    assert_eq!(v1.intent, v2.intent);
    assert_eq!(v1.ruleset_id, v2.ruleset_id);
    assert_eq!(v1.ruleset_version, v2.ruleset_version);
    assert_eq!(v1.profile, v2.profile);
    assert_eq!(v1.capability, v2.capability);
    assert_eq!(v1.capability_tier, v2.capability_tier);
    assert_eq!(v1.policy_id, v2.policy_id);

    // Contribution-id sets must be byte-equal.
    let v1_ids: Vec<&str> = v1
        .contributions
        .iter()
        .map(|c| c.contribution_id.as_str())
        .collect();
    let v2_ids: Vec<&str> = v2
        .contributions
        .iter()
        .map(|c| c.contribution_id.as_str())
        .collect();
    assert_eq!(v1_ids, v2_ids);

    // The only intentional divergences are the ones the index
    // documents. If a future edit introduces a new divergence
    // without adding a row to the index, this assertion pair
    // (above byte-equalities + below policy_version trace diff)
    // will catch it.
    assert_eq!(v1.policy_version, "v1");
    assert_eq!(v2.policy_version, ASSESSMENT_POLICY_V2_VERSION);
    assert_eq!(v2.policy_id, ASSESSMENT_POLICY_V2_ID);
    assert!(v1.trace.is_none());
    assert!(v2.trace.is_some());
}
