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
    assert_eq!(info.daily_recommendations.ruleset_version, info.ruleset_version);
    assert_eq!(info.daily_recommendations.profile, info.profile);
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
fn day_bundle_includes_daily_recommendations_with_fortune() {
    let bundle = get_day_bundle(&query(10, 2, 2024), &[]).expect("bundle");
    let from_info = amlich_api::get_day_info(&query(10, 2, 2024)).expect("day info");

    let bundle_rec = bundle
        .daily_recommendations
        .as_ref()
        .expect("daily recommendations should be present");
    assert_eq!(bundle.meta.profile, from_info.profile);
    assert_eq!(bundle_rec.ruleset_id, from_info.daily_recommendations.ruleset_id);
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
            "daily_recommendations.profile".to_string(),
            "daily_recommendations.summary_vi".to_string(),
            "daily_recommendations.activities".to_string(),
            "meta.profile".to_string(),
            "meta.schema_version".to_string(),
        ],
    )
    .expect("projected");

    assert_eq!(projected["meta"]["schema_version"], "amlich.api/v2");
    assert_eq!(projected["meta"]["profile"], "baseline");
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
