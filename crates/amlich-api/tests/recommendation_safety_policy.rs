use amlich_api::{
    DateQuery, RecommendationBucketDto, RecommendationEvidenceSourceDto,
};

fn days_in_month(month: i32, year: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        2 => 28,
        _ => unreachable!("month must be valid"),
    }
}

fn query(day: i32, month: i32, year: i32) -> DateQuery {
    DateQuery {
        day,
        month,
        year,
        timezone: Some(7.0),
    }
}

#[test]
fn api_burial_recommendations_remain_conservative() {
    let mut checked = 0usize;

    for month in 1..=12 {
        for day in 1..=days_in_month(month, 2024) {
            let info = amlich_api::get_day_info(&query(day, month, 2024)).expect("api day info");
            let Some(burial) = info
                .daily_recommendations
                .activities
                .iter()
                .find(|activity| activity.activity_id == "burial_memorial")
            else {
                continue;
            };

            checked += 1;
            assert_ne!(
                burial.bucket,
                RecommendationBucketDto::Nen,
                "api burial recommendation became aggressively positive on 2024-{month:02}-{day:02}"
            );
            assert!(
                burial
                    .reasons
                    .iter()
                    .all(|reason| !reason.summary_en.starts_with("Suitable for")),
                "api burial wording became overconfident on 2024-{month:02}-{day:02}"
            );
        }
    }

    assert!(checked > 0, "expected burial recommendations during the 2024 api scan");
}

#[test]
fn api_ky_manh_recommendations_keep_taboo_authority() {
    let mut checked = 0usize;

    for month in 1..=12 {
        for day in 1..=days_in_month(month, 2024) {
            let info = amlich_api::get_day_info(&query(day, month, 2024)).expect("api day info");
            for activity in info
                .daily_recommendations
                .activities
                .iter()
                .filter(|activity| activity.bucket == RecommendationBucketDto::KyManh)
            {
                checked += 1;
                assert!(
                    activity
                        .reasons
                        .iter()
                        .any(|reason| {
                            reason.evidence.source == RecommendationEvidenceSourceDto::Taboo
                        }),
                    "api ky_manh activity {} on 2024-{month:02}-{day:02} lost taboo authority",
                    activity.activity_id
                );
            }
        }
    }

    assert!(checked > 0, "expected at least one api ky_manh recommendation in the 2024 scan");
}
