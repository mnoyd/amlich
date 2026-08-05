//! Standalone-vs-aggregate parity fixtures for the canonical
//! PersonalDayAssessment (amlich-mwbp.6).
//!
//! The acceptance criterion is: "standalone and aggregate parity fixtures
//! share normalized inputs and conclusions." Both standalone
//! (advisory-only) and aggregate (full report) calls must produce identical
//! axes, identical contributions, identical decisions, and identical
//! unavailable sections on the same normalized inputs.

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

fn sample_birth_full() -> (
    Option<i32>,
    Option<i32>,
    Option<i32>,
    Option<amlich_core::almanac::tu_menh::Gender>,
) {
    (Some(1990), Some(1), Some(1), Some(sample_gender()))
}

#[test]
fn standalone_advisory_assessment_matches_aggregate_report_assessment() {
    let query = sample_query();
    let (by, bm, bd, gender) = sample_birth_full();

    let advisory = get_personal_day_advisory(&query, by, bm, bd, gender).expect("advisory builds");
    let report = get_personal_day_report(&query, by, bm, bd, gender).expect("report builds");

    let advisory_assessment = advisory
        .canonical_assessment
        .as_ref()
        .expect("advisory canonical_assessment must be populated");
    let report_assessment = report
        .canonical_assessment
        .as_ref()
        .expect("report canonical_assessment must be populated");

    // The whole point of the canonical assessment is parity: identical
    // normalized inputs must produce byte-identical assessment payloads.
    assert_eq!(
        advisory_assessment.ruleset_id, report_assessment.ruleset_id,
        "ruleset_id must match between standalone and aggregate"
    );
    assert_eq!(
        advisory_assessment.policy_id, report_assessment.policy_id,
        "policy_id must match between standalone and aggregate"
    );
    assert_eq!(
        advisory_assessment.decision.bucket, report_assessment.decision.bucket,
        "decision.bucket must be the single source of truth"
    );
    assert_eq!(
        advisory_assessment.decision.confidence, report_assessment.decision.confidence,
        "decision.confidence must be the single source of truth"
    );
    assert_eq!(
        advisory_assessment.decision.primary_conclusion,
        report_assessment.decision.primary_conclusion,
        "primary_conclusion must be shared between standalone and aggregate"
    );
    assert_eq!(
        advisory_assessment.axes.generic_day_quality, report_assessment.axes.generic_day_quality,
        "generic_day_quality axis must match"
    );
    assert_eq!(
        advisory_assessment.axes.intent_fit, report_assessment.axes.intent_fit,
        "intent_fit axis must match"
    );
    assert_eq!(
        advisory_assessment.axes.personal_alignment, report_assessment.axes.personal_alignment,
        "personal_alignment axis must match"
    );
    assert_eq!(
        advisory_assessment.axes.annual_pressure, report_assessment.axes.annual_pressure,
        "annual_pressure axis must match"
    );
    assert_eq!(
        advisory_assessment.axes.evidence_coverage, report_assessment.axes.evidence_coverage,
        "evidence_coverage axis must match"
    );
    assert_eq!(
        advisory_assessment.contributions, report_assessment.contributions,
        "every contribution must be byte-identical (stable contribution_id contract)"
    );
    assert_eq!(
        advisory_assessment.unavailable_sections, report_assessment.unavailable_sections,
        "unavailable_sections must match"
    );
}

#[test]
fn standalone_summary_and_severity_match_aggregate_summary_and_severity() {
    let query = sample_query();
    let (by, bm, bd, gender) = sample_birth_full();

    let advisory = get_personal_day_advisory(&query, by, bm, bd, gender).expect("advisory");
    let report = get_personal_day_report(&query, by, bm, bd, gender).expect("report");

    // Summary, severity, and top_signals flow through the report from the
    // canonical advisory; parity must hold.
    assert_eq!(
        report.summary, advisory.summary,
        "report summary must equal canonical advisory summary"
    );
    assert_eq!(
        report.severity, advisory.severity,
        "report severity must equal canonical advisory severity (single verdict)"
    );
    assert_eq!(
        report.top_signals, advisory.top_signals,
        "top_signals must be byte-identical between standalone and aggregate"
    );
}

#[test]
fn anonymous_profile_assessment_marks_personal_alignment_unavailable() {
    let query = sample_query();
    let advisory = get_personal_day_advisory(&query, None, None, None, None).expect("advisory");
    let report = get_personal_day_report(&query, None, None, None, None).expect("report");

    let assessment = advisory
        .canonical_assessment
        .as_ref()
        .expect("assessment on advisory");
    let report_assessment = report
        .canonical_assessment
        .as_ref()
        .expect("assessment on report");

    assert!(
        assessment
            .unavailable_sections
            .iter()
            .any(|s| s.section == "personal_alignment"),
        "personal_alignment must be marked unavailable when gender is missing"
    );
    assert_eq!(
        assessment.axes.personal_alignment.verdict, "unavailable",
        "personal_alignment axis verdict must be 'unavailable' when unsupported"
    );
    assert!(
        assessment.axes.personal_alignment.score.is_none(),
        "personal_alignment score must be None when unsupported"
    );
    // Parity between standalone and aggregate surfaces on the missing-axis surface.
    assert_eq!(
        assessment.axes.personal_alignment,
        report_assessment.axes.personal_alignment
    );
}

#[test]
fn sparse_profile_assessment_recorded_for_audit() {
    let query = sample_query();
    let report = get_personal_day_report(&query, None, None, None, None).expect("report");
    let assessment = report.canonical_assessment.as_ref().expect("assessment");

    let serialized = serde_json::to_string(assessment).expect("serializes for downstream audit");
    assert!(
        !serialized.contains("\"score\":null") || serialized.contains("\"unavailable_reason\""),
        "axis entries must either carry an unavailable_reason or a numeric score; bare null scores are not allowed"
    );

    // Decision must be available even when coverage is thin — that's the
    // contract: every PersonalDayAssessment carries a single verdict
    // coming from the assessment, not from missing-input heuristics.
    assert!(
        !assessment.decision.bucket.is_empty(),
        "decision.bucket must always be populated, never empty"
    );
    assert!(
        !assessment.decision.primary_conclusion.is_empty(),
        "primary_conclusion must always be populated, never empty"
    );
}

#[test]
fn each_contribution_carries_policy_and_ruleset_metadata() {
    let query = sample_query();
    let (by, bm, bd, gender) = sample_birth_full();
    let report = get_personal_day_report(&query, by, bm, bd, gender).expect("report");
    let assessment = report.canonical_assessment.as_ref().expect("assessment");

    assert!(
        !assessment.contributions.is_empty(),
        "expected at least one contribution for a full profile; got {}",
        assessment.contributions.len()
    );

    for contribution in &assessment.contributions {
        assert!(
            !contribution.contribution_id.is_empty(),
            "contribution_id must be stable and non-empty"
        );
        assert_eq!(
            contribution.policy_id, "personal-day-assessment",
            "policy_id must be the locked assessment policy identifier"
        );
        assert!(
            !contribution.policy_version.is_empty(),
            "policy_version must be recorded"
        );
        assert!(
            !contribution.ruleset_id.is_empty(),
            "ruleset_id must be recorded so parity checks can compare"
        );
        assert!(
            !contribution.ruleset_version.is_empty(),
            "ruleset_version must be recorded so parity checks can compare"
        );
        assert!(
            (0.0..=1.0).contains(&contribution.strength),
            "strength must live in 0..=1; got {:?}",
            contribution.strength
        );
    }
}

#[test]
fn policy_version_is_locked_for_this_migration() {
    let query = sample_query();
    let (by, bm, bd, gender) = sample_birth_full();
    let report = get_personal_day_report(&query, by, bm, bd, gender).expect("report");
    let assessment = report.canonical_assessment.as_ref().expect("assessment");

    assert_eq!(
        assessment.policy_id, "personal-day-assessment",
        "policy_id must be the locked assessment policy identifier"
    );
    assert!(
        assessment.policy_version.starts_with('v'),
        "policy_version must follow the conventional 'v<digit>' format; got {:?}",
        assessment.policy_version
    );
}

// ---------------------------------------------------------------------------
// amlich-l0wu: Veto-forced decision projects correctly through the API DTO.
//
// The API still uses the v1 builder until the stability gate (amlich-31oa)
// promotes v2. v1's hard_veto and v2's named vetoes produce byte-identical
// decisions (locked by assessment_v2_seam::v1_v2_full_parity_*). This test
// pins the DTO-side projection: a veto-firing fixture must reach the API
// consumer as `bucket: "avoid"` with the veto-override score, regardless of
// how favorable the weighted axes are.
// ---------------------------------------------------------------------------

#[test]
fn veto_forced_decision_projects_to_avoid_through_dto() {
    // 1985 birth year hits Hạn High severity on 2024-02-10, which fires a
    // hard veto (v1 implicit; v2 named `veto.annual.han_severe`).
    let query = sample_query();
    let report = get_personal_day_report(
        &query,
        Some(1985),
        Some(1),
        Some(1),
        Some(amlich_core::almanac::tu_menh::Gender::Female),
    )
    .expect("report");
    let assessment = report.canonical_assessment.as_ref().expect("assessment");

    assert_eq!(
        assessment.decision.bucket, "avoid",
        "a veto-firing fixture must project to the avoid bucket through the DTO"
    );
    assert_eq!(
        assessment.decision.semantic, "override_avoid",
        "the veto override semantic must propagate to the DTO"
    );
    // The veto forces a fixed low score (~0.15), independent of the
    // weighted axis average.
    let score = assessment
        .decision
        .decision_score
        .expect("veto decision carries a score");
    assert!(
        score <= 0.2,
        "veto-forced decision score must be the override floor, got {score}"
    );
}

#[test]
fn unavailable_sections_project_through_dto_for_capability_gaps() {
    // The no-gender query must surface the unavailable sections through
    // the DTO so API consumers can explain what evidence was missing
    // (amlich-l0wu: missing inputs are reported).
    let query = sample_query();
    let report = get_personal_day_report(&query, None, None, None, None).expect("report");
    let assessment = report.canonical_assessment.as_ref().expect("assessment");

    assert!(
        !assessment.unavailable_sections.is_empty(),
        "no-birth-info query must report unavailable sections through the DTO"
    );
    let section_names: Vec<&str> = assessment
        .unavailable_sections
        .iter()
        .map(|s| s.section.as_str())
        .collect();
    assert!(
        section_names.contains(&"personal_alignment"),
        "personal_alignment section must be reported unavailable without gender, got {:?}",
        section_names
    );
    assert!(
        section_names.contains(&"annual_han"),
        "annual_han section must be reported unavailable without gender, got {:?}",
        section_names
    );
}
