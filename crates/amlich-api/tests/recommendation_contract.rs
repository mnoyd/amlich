use amlich_api::{
    v2::{get_day_bundle, get_day_bundle_projected},
    DateQuery,
};

fn query(day: i32, month: i32, year: i32) -> DateQuery {
    DateQuery {
        day,
        month,
        year,
        timezone: Some(7.0),
        ruleset_id: None,
        event_kind: None,
        enabled_pack_ids: vec![],
    }
}

fn bucket_rank(bucket: &str) -> u8 {
    match bucket {
        "nen" => 0,
        "co_the" => 1,
        "tranh" => 2,
        "ky_manh" => 3,
        _ => 9,
    }
}

#[test]
fn day_info_exposes_daily_recommendations_contract() {
    let info = amlich_api::get_day_info(&query(10, 2, 2024)).expect("day info");

    assert_eq!(info.ruleset_id, "vn_baseline_v1");
    assert_eq!(info.ruleset_version, "v1");
    assert_eq!(info.profile, "baseline");
    assert_eq!(info.daily_recommendations.ruleset_id, info.ruleset_id);
    assert_eq!(
        info.daily_recommendations.ruleset_version,
        info.ruleset_version
    );
    assert_eq!(info.daily_recommendations.profile, info.profile);
    assert!(info.contextual_recommendations.is_none());
    assert!(info.daily_recommendations.version.starts_with("v1-"));
    assert!(!info.daily_recommendations.summary_vi.is_empty());
    assert!(!info.daily_recommendations.activities.is_empty());

    for activity in &info.daily_recommendations.activities {
        assert!(!activity.activity_id.is_empty());
        assert!(!activity.label.vi.is_empty());
        assert!(!activity.reasons.is_empty());

        for reason in &activity.reasons {
            assert!(!reason.rule_id.is_empty());
            assert!(!reason.summary_vi.is_empty());
            assert!(!reason.evidence.code.is_empty());
            assert!(!reason.evidence.note.is_empty());
        }
    }

    let mut prev_rank = 0u8;
    for (idx, activity) in info.daily_recommendations.activities.iter().enumerate() {
        let rank = bucket_rank(match activity.bucket {
            amlich_api::RecommendationBucketDto::Nen => "nen",
            amlich_api::RecommendationBucketDto::CoThe => "co_the",
            amlich_api::RecommendationBucketDto::Tranh => "tranh",
            amlich_api::RecommendationBucketDto::KyManh => "ky_manh",
        });
        if idx > 0 {
            assert!(rank >= prev_rank, "bucket ordering should be deterministic");
        }
        prev_rank = rank;
    }
}

#[test]
fn day_info_can_expose_contextual_recommendations() {
    let baseline = amlich_api::get_day_info(&query(10, 2, 2024)).expect("baseline day info");
    let mut request = query(10, 2, 2024);
    request.event_kind = Some("contract_signing".to_string());
    request.enabled_pack_ids = vec!["pack.nhi_thap_bat_tu.v1".to_string()];

    let info = amlich_api::get_day_info(&request).expect("day info");
    let contextual = info
        .contextual_recommendations
        .as_ref()
        .expect("contextual recommendations");

    assert_eq!(
        info.daily_recommendations.activities.len(),
        baseline.daily_recommendations.activities.len()
    );
    let baseline_contract = baseline
        .daily_recommendations
        .activities
        .iter()
        .find(|activity| activity.activity_id == "contract_agreement")
        .expect("baseline contract activity");
    let controlled_contract = info
        .daily_recommendations
        .activities
        .iter()
        .find(|activity| activity.activity_id == "contract_agreement")
        .expect("controlled baseline contract activity");
    assert_eq!(controlled_contract.bucket, baseline_contract.bucket);
    assert_eq!(
        controlled_contract.reasons.len(),
        baseline_contract.reasons.len()
    );
    assert_eq!(
        info.daily_recommendations.summary_vi,
        baseline.daily_recommendations.summary_vi
    );
    assert_eq!(
        info.daily_recommendations.summary_en,
        baseline.daily_recommendations.summary_en
    );
    assert_eq!(contextual.active_packs.len(), 1);
    assert!(baseline.daily_recommendations.active_packs.is_empty());
    assert!(contextual
        .activities
        .iter()
        .find(|activity| activity.activity_id == "contract_agreement")
        .expect("contract activity")
        .reasons
        .iter()
        .any(|reason| reason.rule_id == "layer.product_rule.event_kind.contract_signing"));
}

#[test]
fn invalid_selectors_fail_explicitly() {
    let mut unknown_ruleset = query(10, 2, 2024);
    unknown_ruleset.ruleset_id = Some("not-a-ruleset".to_string());
    assert_eq!(
        amlich_api::get_day_info(&unknown_ruleset).expect_err("unknown ruleset must fail"),
        "unknown almanac ruleset id: not-a-ruleset"
    );

    let mut duplicate_pack = query(10, 2, 2024);
    duplicate_pack.enabled_pack_ids = vec![
        "pack.nhi_thap_bat_tu.v1".to_string(),
        "pack.nhi_thap_bat_tu.v1".to_string(),
    ];
    assert_eq!(
        amlich_api::get_day_info(&duplicate_pack).expect_err("duplicate pack must fail"),
        "duplicate recommendation pack id: pack.nhi_thap_bat_tu.v1"
    );

    let mut unknown_pack = query(10, 2, 2024);
    unknown_pack.enabled_pack_ids = vec!["pack.unknown.v1".to_string()];
    assert_eq!(
        amlich_api::get_day_info(&unknown_pack).expect_err("unknown pack must fail"),
        "unknown recommendation pack id: pack.unknown.v1"
    );

    let mut unsupported_event = query(10, 2, 2024);
    unsupported_event.event_kind = Some("wedding".to_string());
    assert_eq!(
        amlich_api::get_day_info(&unsupported_event).expect_err("unsupported event kind must fail"),
        "unsupported recommendation event_kind: wedding. supported values: contract_signing, medical_checkup, travel"
    );
}

#[test]
fn day_bundle_includes_daily_recommendations_with_fortune() {
    let bundle = get_day_bundle(&query(10, 2, 2024), &[]).expect("bundle");
    let from_info = amlich_api::get_day_info(&query(10, 2, 2024)).expect("day info");

    let bundle_rec = bundle
        .daily_recommendations
        .as_ref()
        .expect("daily recommendations should be present");
    assert_eq!(bundle.schema_version, "amlich.engine/v1");
    assert_eq!(bundle.ruleset_id, from_info.ruleset_id);
    assert_eq!(bundle.ruleset_version, from_info.ruleset_version);
    assert_eq!(bundle.profile, from_info.profile);
    assert!(!bundle.generated_at.is_empty());
    assert_eq!(
        bundle_rec.ruleset_id,
        from_info.daily_recommendations.ruleset_id
    );
    assert_eq!(
        bundle_rec.ruleset_version,
        from_info.daily_recommendations.ruleset_version
    );
    assert_eq!(bundle_rec.profile, from_info.daily_recommendations.profile);
    assert_eq!(bundle_rec.version, from_info.daily_recommendations.version);
    assert_eq!(
        bundle_rec.summary_vi,
        from_info.daily_recommendations.summary_vi
    );
    assert_eq!(
        bundle_rec.activities.len(),
        from_info.daily_recommendations.activities.len()
    );
}

#[test]
fn day_bundle_projection_supports_recommendation_fields() {
    let projected = get_day_bundle_projected(
        &query(10, 2, 2024),
        &[],
        &[
            "schema_version".to_string(),
            "profile".to_string(),
            "daily_recommendations.profile".to_string(),
            "daily_recommendations.summary_vi".to_string(),
            "daily_recommendations.activities".to_string(),
        ],
    )
    .expect("projected");

    assert_eq!(projected["schema_version"], "amlich.engine/v1");
    assert_eq!(projected["profile"], "baseline");
    assert_eq!(projected["daily_recommendations"]["profile"], "baseline");
    assert!(projected["daily_recommendations"]["summary_vi"]
        .as_str()
        .map(|s| !s.is_empty())
        .unwrap_or(false));
    assert!(projected["daily_recommendations"]["activities"]
        .as_array()
        .map(|a| !a.is_empty())
        .unwrap_or(false));
}

#[test]
fn projection_cannot_bypass_omitted_sections() {
    let err = get_day_bundle_projected(
        &query(10, 2, 2024),
        &[
            amlich_api::v2::Include::Base,
            amlich_api::v2::Include::CanChi,
        ],
        &["day_fortune.ruleset_id".to_string()],
    )
    .expect_err("projection should fail when section is omitted");

    assert!(err.contains("unknown field path"));
}
