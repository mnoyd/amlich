//! amlich-mwbp.7 projection parity + serialization contract tests.
//!
//! Acceptance criterion for this bead: "No transport computes an
//! independent verdict or severity; legacy fields are projected from
//! canonical facts or explicitly deprecated; confidence follows
//! capability/evidence coverage; standalone and aggregate outputs agree;
//! old/new shadow comparisons and serialization contract tests pass;
//! rollback can switch projection without reverting safety fixes."
//!
//! These fixtures lock the projection contract for the surfaces that
//! mwbp.6 did not yet wire through the canonical assessment:
//!
//! - the personal-day **matrix** report (raw interaction signals only,
//!   single verdict attached as `canonical_assessment`),
//! - the **hour-selection** analysis/advisory surfaces (compatibility
//!   ranking + canonical day verdict),
//! - the legacy advisory confidence/severity shadow vs. canonical decision
//!   confidence mapping (the `AdvisoryScoring`/`score_day_selection`
//!   compatibility surface was retired in `amlich-0q2f`).

use amlich_api::{
    get_hour_selection_advisory, get_hour_selection_analysis, get_personal_day_advisory,
    get_personal_day_matrix_report, get_personal_day_report, BaziQuery, DateQuery,
};

fn sample_date() -> DateQuery {
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

fn sample_birth_datetime() -> BaziQuery {
    BaziQuery {
        day: 1,
        month: 1,
        year: 1990,
        hour: 9,
        minute: 30,
        time_known: Some(true),
        timezone: Some(7.0),
        longitude: None,
        use_solar_time: false,
        gender: Some("male".to_string()),
    }
}

fn sample_birth_date_only() -> BaziQuery {
    BaziQuery {
        hour: 0,
        minute: 0,
        time_known: Some(false),
        ..sample_birth_datetime()
    }
}

fn sample_birth_no_gender() -> BaziQuery {
    let mut birth = sample_birth_datetime();
    birth.gender = None;
    birth
}

// ---------------------------------------------------------------------------
// Matrix report — single canonical verdict, raw signals only.
// ---------------------------------------------------------------------------

#[test]
fn matrix_report_attaches_canonical_assessment_with_locked_policy() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let assessment = report
        .canonical_assessment
        .as_ref()
        .expect("matrix report MUST attach canonical_assessment (amlich-mwbp.7)");

    assert_eq!(
        assessment.policy_id, "personal-day-assessment",
        "policy_id must be the locked canonical identifier"
    );
    assert!(
        assessment.policy_version.starts_with('v'),
        "policy_version must follow the 'v<digit>' format; got {:?}",
        assessment.policy_version
    );
    assert!(
        !assessment.decision.bucket.is_empty(),
        "matrix surface MUST carry a canonical verdict; the raw signals alone are not a verdict"
    );
    assert!(
        !assessment.decision.primary_conclusion.is_empty(),
        "primary_conclusion must always be populated"
    );
}

#[test]
fn matrix_report_canonical_assessment_confidence_follows_capability() {
    let full = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let sparse = get_personal_day_matrix_report(&sample_birth_date_only(), &sample_date())
        .expect("matrix report");

    let full_assessment = full
        .canonical_assessment
        .as_ref()
        .expect("full matrix assessment");
    let sparse_assessment = sparse
        .canonical_assessment
        .as_ref()
        .expect("sparse matrix assessment");

    // Confidence derives from BirthCapability coverage, not from chart
    // presence or birth-input existence. A full datetime+gender profile
    // must not report lower confidence than a date-only profile.
    let full_rank = confidence_rank(full_assessment.decision.confidence.as_str());
    let sparse_rank = confidence_rank(sparse_assessment.decision.confidence.as_str());
    assert!(
        full_rank >= sparse_rank,
        "full-profile confidence ({:?}) must not be lower than date-only confidence ({:?})",
        full_assessment.decision.confidence,
        sparse_assessment.decision.confidence
    );
}

fn confidence_rank(value: &str) -> u8 {
    match value {
        "low" => 0,
        "medium" => 1,
        "high" => 2,
        _ => 0,
    }
}

#[test]
fn matrix_report_unavailable_sections_are_superset_of_assessment_sections() {
    // Gender missing → the canonical assessment flags personal_alignment
    // and annual_han as unavailable. The matrix surface must NOT silently
    // hide those sections; its unavailable_sections must be a superset.
    let report = get_personal_day_matrix_report(&sample_birth_no_gender(), &sample_date())
        .expect("matrix report");
    let assessment = report
        .canonical_assessment
        .as_ref()
        .expect("canonical_assessment attached");

    for section in &assessment.unavailable_sections {
        let present = report
            .unavailable_sections
            .iter()
            .any(|s| s.section == section.section);
        assert!(
            present,
            "matrix unavailable_sections must include assessment-derived section {:?}; the surface cannot silently hide a missing capability",
            section.section
        );
    }
}

#[test]
fn matrix_report_serialization_round_trips_and_exposes_canonical_assessment() {
    let report = get_personal_day_matrix_report(&sample_birth_datetime(), &sample_date())
        .expect("matrix report");
    let serialized = serde_json::to_string(&report).expect("serialize matrix report");
    assert!(
        serialized.contains("\"canonical_assessment\""),
        "serialized matrix report must expose canonical_assessment key"
    );
    assert!(
        serialized.contains("\"policy_id\":\"personal-day-assessment\""),
        "serialized matrix report must embed the locked canonical policy_id"
    );

    // Round-trip: the DTO must survive serde round-trip with the
    // canonical_assessment intact. This is the rollback contract — the
    // projection can be switched without breaking the payload shape.
    let value: serde_json::Value =
        serde_json::from_str(&serialized).expect("deserialization succeeds");
    let assessment = &value["canonical_assessment"];
    assert!(
        !assessment.is_null(),
        "canonical_assessment must round-trip as non-null"
    );
    assert_eq!(
        assessment["policy_id"].as_str().unwrap(),
        "personal-day-assessment"
    );
    assert!(
        !assessment["contributions"].as_array().unwrap().is_empty(),
        "contributions array must survive round-trip"
    );
}

#[test]
fn matrix_report_canonical_assessment_matches_aggregate_report_assessment() {
    // The single-verdict contract: matrix and aggregate report project
    // from one canonical assessment on identical normalized inputs.
    //
    // The matrix surface accepts full birth datetime (BaziQuery) while
    // the aggregate report accepts only date + gender. To compare
    // apples-to-apples we drive both with a date-only profile so the
    // normalized_birth contract is observable on identical inputs.
    let matrix = get_personal_day_matrix_report(&sample_birth_date_only(), &sample_date())
        .expect("matrix report");
    let report = get_personal_day_report(
        &sample_date(),
        Some(1990),
        Some(1),
        Some(1),
        Some(amlich_core::almanac::tu_menh::Gender::Male),
    )
    .expect("aggregate report");

    let matrix_assessment = matrix
        .canonical_assessment
        .as_ref()
        .expect("matrix assessment");
    let report_assessment = report
        .canonical_assessment
        .as_ref()
        .expect("report assessment");

    // Both surfaces use the OpeningBusiness intent and identical date, so
    // the canonical policy/ruleset metadata must agree.
    assert_eq!(
        matrix_assessment.policy_id, report_assessment.policy_id,
        "policy_id must be identical across matrix and report surfaces"
    );
    assert_eq!(
        matrix_assessment.policy_version, report_assessment.policy_version,
        "policy_version must be identical across matrix and report surfaces"
    );
    assert_eq!(
        matrix_assessment.normalized_birth,
        report_assessment.normalized_birth,
        "normalized_birth must agree on identical date-only inputs — single normalized profile contract"
    );
    assert_eq!(
        matrix_assessment.intent, report_assessment.intent,
        "both surfaces project the OpeningBusiness intent"
    );
    assert_eq!(
        matrix_assessment.decision.bucket, report_assessment.decision.bucket,
        "decision.bucket must agree on identical inputs — single verdict contract"
    );
    assert_eq!(
        matrix_assessment.contributions, report_assessment.contributions,
        "contributions must be byte-identical on identical inputs"
    );
}

// ---------------------------------------------------------------------------
// Hour-selection surfaces — compatibility ranking + canonical verdict.
// ---------------------------------------------------------------------------

#[test]
fn hour_selection_advisory_attaches_canonical_assessment_with_travel_intent() {
    let advisory =
        get_hour_selection_advisory(&sample_date(), Some(1990), Some(1), Some(1), Some("male"))
            .expect("hour advisory");
    let assessment = advisory
        .canonical_assessment
        .as_ref()
        .expect("hour advisory MUST attach canonical_assessment (amlich-mwbp.7)");

    assert_eq!(
        assessment.intent, "travel",
        "hour-selection surface projects the Travel intent"
    );
    assert!(
        !assessment.decision.bucket.is_empty(),
        "hour surface MUST carry a canonical verdict; the ranked-hours score is not a verdict"
    );
    // The hour surface's `canonical` (HourSelectionReasoningExport) is the
    // compatibility ranking; the canonical_assessment is the verdict.
    // Both must be present so consumers can tell them apart.
    assert!(
        advisory.canonical.is_some(),
        "compatibility canonical ranking must still be present alongside the canonical assessment"
    );
}

#[test]
fn hour_selection_analysis_attaches_canonical_assessment() {
    let analysis =
        get_hour_selection_analysis(&sample_date(), Some(1990), Some(1), Some(1), Some("male"))
            .expect("hour analysis");
    assert!(
        analysis.canonical_assessment.is_some(),
        "hour analysis MUST attach canonical_assessment (amlich-mwbp.7)"
    );
}

#[test]
fn hour_selection_advisory_canonical_assessment_agrees_with_standalone_advisory() {
    // The hour surface and the standalone personal-day advisory must
    // agree on the canonical verdict for the same normalized inputs +
    // Travel intent. The standalone advisory uses
    // `build_personal_day_canonical_assessment` which defaults to
    // OpeningBusiness; we force the comparison through the same Travel
    // intent by building the advisory's assessment via its
    // canonical_assessment field, which carries whichever intent the
    // surface used. Hour uses Travel; the standalone advisory's
    // event_kind can be passed via DateQuery.event_kind.
    let mut travel_query = sample_date();
    travel_query.event_kind = Some("travel".to_string());

    let hour_advisory =
        get_hour_selection_advisory(&travel_query, Some(1990), Some(1), Some(1), Some("male"))
            .expect("hour advisory");
    let standalone = get_personal_day_advisory(
        &travel_query,
        Some(1990),
        Some(1),
        Some(1),
        Some(amlich_core::almanac::tu_menh::Gender::Male),
    )
    .expect("standalone advisory");

    let hour_assessment = hour_advisory
        .canonical_assessment
        .as_ref()
        .expect("hour assessment");
    let standalone_assessment = standalone
        .canonical_assessment
        .as_ref()
        .expect("standalone assessment");

    // Confidence follows capability coverage on both surfaces; with the
    // same birth profile they must agree even if intents differ.
    assert_eq!(
        hour_assessment.decision.confidence, standalone_assessment.decision.confidence,
        "confidence derives from capability coverage, not from intent or surface"
    );
    assert_eq!(
        hour_assessment.normalized_birth, standalone_assessment.normalized_birth,
        "normalized_birth must agree across hour and standalone advisory surfaces"
    );
}

#[test]
fn hour_selection_anonymous_profile_attaches_assessment_with_unavailable_personal_alignment() {
    // Anonymous caller — no birth at all. The hour surface still must
    // not present an independent verdict; the attached assessment marks
    // personal_alignment as unavailable.
    let analysis =
        get_hour_selection_analysis(&sample_date(), None, None, None, None).expect("hour analysis");
    let assessment = analysis
        .canonical_assessment
        .as_ref()
        .expect("anonymous hour analysis still attaches canonical_assessment");

    assert!(
        assessment
            .unavailable_sections
            .iter()
            .any(|s| s.section == "personal_alignment"),
        "personal_alignment must be unavailable for an anonymous profile"
    );
    assert_eq!(
        assessment.axes.personal_alignment.verdict, "unavailable",
        "personal_alignment axis verdict must be 'unavailable' for anonymous profile"
    );
}

// ---------------------------------------------------------------------------
// Legacy AdvisoryScoring shadow vs. canonical decision confidence.
// ---------------------------------------------------------------------------

#[test]
fn legacy_advisory_scoring_confidence_shadows_canonical_decision_confidence() {
    // The legacy `AdvisoryScoring.confidence` string is a compatibility
    // projection of the canonical
    // `PersonalDayAssessment.decision.confidence`. The mapping must be
    // 1:1 so rolling the projection back to the legacy adapter does not
    // silently change severity semantics.
    //
    // We exercise three capability tiers (full profile, date+gender,
    // anonymous) without overriding event_kind so the default
    // OpeningBusiness intent drives both surfaces identically.
    type BirthRow<'a> = (Option<i32>, Option<i32>, Option<i32>, Option<&'a str>);
    let cases: &[BirthRow<'_>] = &[
        (Some(1990), Some(1), Some(1), Some("male")),
        (Some(1990), Some(1), Some(1), None),
        (None, None, None, None),
    ];

    for (by, bm, bd, gender) in cases {
        let query = sample_date();
        let gender_enum = gender.and_then(|g| match g {
            "male" => Some(amlich_core::almanac::tu_menh::Gender::Male),
            "female" => Some(amlich_core::almanac::tu_menh::Gender::Female),
            _ => None,
        });
        let advisory =
            get_personal_day_advisory(&query, *by, *bm, *bd, gender_enum).expect("advisory");

        let canonical_confidence = advisory
            .canonical_assessment
            .as_ref()
            .map(|a| a.decision.confidence.as_str())
            .expect("canonical_assessment present");
        let legacy_confidence = advisory
            .reasoning_confidence
            .as_deref()
            .expect("reasoning_confidence present");

        assert_eq!(
            legacy_confidence, canonical_confidence,
            "legacy reasoning_confidence must equal canonical decision.confidence (shadow parity)"
        );

        // The confidence label must be one of the canonical enum variants.
        assert!(
            matches!(canonical_confidence, "low" | "medium" | "high"),
            "canonical confidence must serialize to a known enum variant; got {canonical_confidence:?}"
        );
    }
}

#[test]
fn legacy_advisory_severity_is_projected_from_canonical_decision_bucket() {
    // Severity (high/medium/low) must derive from the canonical
    // decision.bucket (Avoid/Cautious/Mixed/Favorable), not from raw
    // caution-message counts. We cannot assert an exact mapping because
    // the advisory layer combines bucket + cautions for the Favorable
    // and Mixed cases, but we CAN assert that an Avoid bucket never
    // produces a "low" severity (that would be a safety regression).
    //
    // We scan a small set of dates to find at least one Avoid verdict
    // and then check the safety property; if none appear in the sample
    // the test still passes (no Avoid verdict → no safety concern).
    let mut saw_avoid = false;
    for (d, m) in [(10, 2), (15, 4), (7, 7), (3, 11)] {
        let query = DateQuery {
            day: d,
            month: m,
            year: 2024,
            timezone: Some(7.0),
            ruleset_id: None,
            event_kind: None,
            enabled_pack_ids: vec![],
        };
        let advisory = get_personal_day_advisory(
            &query,
            Some(1990),
            Some(1),
            Some(1),
            Some(amlich_core::almanac::tu_menh::Gender::Male),
        )
        .expect("advisory");

        let assessment = advisory
            .canonical_assessment
            .as_ref()
            .expect("canonical_assessment");

        if assessment.decision.bucket == "avoid" {
            saw_avoid = true;
            assert_ne!(
                advisory.severity, "low",
                "an Avoid canonical verdict must never be projected to 'low' severity (safety: amlich-mwbp.7) — date {d}/{m}"
            );
        }
    }
    // Sanity: the sample must contain at least one Avoid verdict so the
    // safety property is actually exercised.
    assert!(
        saw_avoid,
        "sample must contain at least one Avoid verdict to exercise the severity safety property"
    );
}
