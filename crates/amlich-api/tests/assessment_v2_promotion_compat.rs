//! API-side stability gate for the v2.x assessment promotion
//! (`amlich-31oa`).
//!
//! The Rust core's stability gate
//! (`crates/amlich-core/tests/assessment_v2_stability_gate.rs`)
//! runs the four (six) gates and reports a machine-readable
//! [`StabilityReport`]. The amlich-api surface adds three additional
//! compatibility concerns that the core gate cannot cover:
//!
//! - The v1 DTO wire shape (the
//!   `PersonalDayAssessmentDto` returned from
//!   `get_personal_day_report` and `get_personal_day_advisory`) must
//!   stay byte-compatible with the v1 default — every existing
//!   consumer (TUI, desktop, downstream scripts) reads it.
//! - The v2 additive fields (`explanation_graph`, `trace`) must
//!   remain `None` / opt-in for the v1 default path so a v1 caller
//!   does not suddenly start receiving new fields.
//! - The promotion status surface (current default version +
//!   candidate + `can_promote`) must remain stable so a CI job
//!   (or a desktop update banner) can ask "is v2 ready?" without
//!   parsing the Rust `StabilityReport` directly.
//!
//! These tests are the amlich-api half of the compatibility gate.
//! A failure here means a v1 consumer (TUI, desktop, or downstream
//! script) is about to see a behavioural change.

use amlich_api::{get_personal_day_advisory, get_personal_day_report, DateQuery};

fn sample_query() -> DateQuery {
    DateQuery {
        day: 10,
        month: 2,
        year: 2024,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

fn sample_gender() -> amlich_core::almanac::tu_menh::Gender {
    amlich_core::almanac::tu_menh::Gender::Male
}

fn assessment_from_advisory() -> amlich_api::PersonalDayAssessmentDto {
    let advisory = get_personal_day_advisory(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("advisory builds");
    advisory
        .canonical_assessment
        .expect("advisory canonical_assessment must be populated")
}

fn assessment_from_report() -> amlich_api::PersonalDayAssessmentDto {
    let report = get_personal_day_report(
        &sample_query(),
        Some(1990),
        Some(1),
        Some(1),
        Some(sample_gender()),
    )
    .expect("report builds");
    report
        .canonical_assessment
        .expect("report canonical_assessment must be populated")
}

// ---------------------------------------------------------------------------
// DTO wire shape — the v1 default must keep the existing field set.
// Any new field must be additive and either Option/None or
// skip_serializing_if. Adding a required field here would break every
// v1 consumer.
// ---------------------------------------------------------------------------

#[test]
fn v1_default_advisory_dto_carries_v1_policy_version() {
    let assessment = assessment_from_advisory();
    assert_eq!(
        assessment.policy_version, "v1",
        "v1 default path must stay pinned at policy_version=v1 until the stability gate authorises a flip"
    );
    assert_eq!(assessment.policy_id, "personal-day-assessment");
    // The v2 additive field is opt-in; the v1 default must not
    // silently start emitting it. A future change that wires the v2
    // policy into the default entry point without a stability-gate
    // sign-off will trip this assertion.
    assert!(
        assessment.explanation_graph.is_none(),
        "v1 default path must not emit explanation_graph (v2-only additive field)"
    );
}

#[test]
fn v1_default_advisory_and_report_dtos_are_byte_compatible() {
    // The v1 default path is byte-equivalent across standalone
    // (advisory) and aggregate (report) calls. The full byte-level
    // equality is locked by
    // `personal_day_assessment_parity::standalone_*`; the gate
    // here re-asserts a representative subset on the new code.
    let advisory = assessment_from_advisory();
    let report = assessment_from_report();
    assert_eq!(advisory.policy_id, report.policy_id);
    assert_eq!(advisory.policy_version, report.policy_version);
    assert_eq!(advisory.axes, report.axes);
    assert_eq!(advisory.decision.bucket, report.decision.bucket);
    assert_eq!(
        advisory.decision.decision_score,
        report.decision.decision_score
    );
    assert_eq!(advisory.unavailable_sections, report.unavailable_sections);
    assert_eq!(advisory.evidence, report.evidence);
}

#[test]
fn v1_default_dto_field_set_is_stable() {
    // A v1 DTO field set snapshot. Adding a new required field is
    // a breaking change; adding a new optional field with
    // `skip_serializing_if` is allowed (it serialises to `None` for
    // v1 callers, leaving the wire shape byte-equal). If you bump
    // this list, you must also update the desktop TS type and the
    // TUI consumer, and re-run the full parity suite.
    let assessment = assessment_from_advisory();
    let json = serde_json::to_value(&assessment).expect("serialize DTO");
    let obj = json.as_object().expect("DTO is a JSON object");
    let field_names: Vec<&str> = obj.keys().map(String::as_str).collect();
    let expected: Vec<&str> = vec![
        "ruleset_id",
        "ruleset_version",
        "policy_id",
        "policy_version",
        "profile",
        "intent",
        "capability_tier",
        "normalized_birth",
        "axes",
        "decision",
        "factors",
        "contributions",
        "unavailable_sections",
        "evidence",
    ];
    let mut sorted_expected = expected.clone();
    sorted_expected.sort_unstable();
    let mut sorted_actual = field_names.clone();
    sorted_actual.sort_unstable();
    assert_eq!(
        sorted_actual, sorted_expected,
        "v1 DTO field set diverged; an additive field is fine but a removed or renamed field is a breaking change"
    );
    // `explanation_graph` is the v2-only additive field. It
    // serialises to `None` for the v1 default and must be absent
    // from the wire JSON (skip_serializing_if = Option::is_none).
    assert!(
        !obj.contains_key("explanation_graph"),
        "explanation_graph must not appear in the v1 wire JSON"
    );
    assert!(
        obj.get("factors")
            .and_then(|value| value.as_array())
            .is_some_and(|factors| !factors.is_empty()),
        "v1 now exposes the additive canonical factor classification"
    );
}

// ---------------------------------------------------------------------------
// Promotion status surface — CI / desktop / TUI need a stable way to
// ask "is v2 ready?" without parsing the Rust `StabilityReport`.
// ---------------------------------------------------------------------------

#[test]
fn current_default_policy_version_is_pinned_at_v1() {
    // The v1 default stays pinned until a maintainer explicitly
    // edits `current_default_policy_version`. The stability gate
    // authorises the flip but never triggers it.
    assert_eq!(
        amlich_core::assessment::current_default_policy_version(),
        "v1",
        "the production default policy version must stay pinned at v1 until the stability gate authorises a flip"
    );
}

#[test]
fn promotion_status_round_trips_through_serde() {
    use amlich_core::assessment::{PromotionStatus, PromotionStatusReport};
    let report = PromotionStatusReport::build("v2.2", true, None);
    let json = serde_json::to_string(&report).expect("serialize promotion status");
    let back: PromotionStatusReport =
        serde_json::from_str(&json).expect("deserialize promotion status");
    assert_eq!(report, back);
    assert_eq!(back.status, PromotionStatus::V1DefaultExperimental);
    assert!(!back.status.is_v2_default());
    assert!(back.can_promote);
}
