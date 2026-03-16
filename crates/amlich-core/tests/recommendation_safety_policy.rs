mod support;

use amlich_core::almanac::recommendation::{
    ActivityId, RecommendationBucket, RecommendationEvidenceSource,
};
use support::day_snapshot;

fn days_in_month(month: i32, year: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 => 29,
        2 => 28,
        _ => unreachable!("month must be valid"),
    }
}

#[test]
fn burial_recommendations_remain_conservative_in_default_engine() {
    let mut checked = 0usize;

    for month in 1..=12 {
        for day in 1..=days_in_month(month, 2024) {
            let snapshot = day_snapshot(day, month, 2024);
            let Some(burial) = snapshot
                .daily_recommendations
                .activities
                .iter()
                .find(|activity| activity.activity_id == ActivityId::BurialMemorial)
            else {
                continue;
            };

            checked += 1;
            assert_ne!(
                burial.bucket,
                RecommendationBucket::Nen,
                "burial recommendation became aggressively positive on 2024-{month:02}-{day:02}"
            );
            assert!(
                burial
                    .reasons
                    .iter()
                    .all(|reason| !reason.summary_en.starts_with("Suitable for")),
                "burial wording became overconfident on 2024-{month:02}-{day:02}"
            );
        }
    }

    assert!(checked > 0, "expected burial recommendations during the 2024 scan");
}

#[test]
fn ky_manh_recommendations_always_include_taboo_evidence() {
    let mut checked = 0usize;

    for month in 1..=12 {
        for day in 1..=days_in_month(month, 2024) {
            let snapshot = day_snapshot(day, month, 2024);
            for activity in snapshot
                .daily_recommendations
                .activities
                .iter()
                .filter(|activity| activity.bucket == RecommendationBucket::KyManh)
            {
                checked += 1;
                assert!(
                    activity
                        .reasons
                        .iter()
                        .any(|reason| reason.evidence.source == RecommendationEvidenceSource::Taboo),
                    "ky_manh activity {:?} on 2024-{month:02}-{day:02} lost taboo authority",
                    activity.activity_id
                );
            }
        }
    }

    assert!(checked > 0, "expected at least one ky_manh recommendation in the 2024 scan");
}
