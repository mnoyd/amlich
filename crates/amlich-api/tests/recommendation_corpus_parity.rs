use std::{fs, path::PathBuf};

use amlich_api::{
    v2::{get_day_bundle, Include},
    DateQuery,
};
use amlich_core::almanac::recommendation::{ActivityId, RecommendationBucket};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Corpus {
    cases: Vec<CorpusCase>,
}

#[derive(Debug, Deserialize)]
struct CorpusCase {
    id: String,
    date: String,
    #[serde(default)]
    expect: ExpectedExpectations,
}

#[derive(Debug, Deserialize, Default)]
struct ExpectedExpectations {
    #[serde(default)]
    must_match_activity_buckets: Vec<ExpectedActivityBucket>,
}

#[derive(Debug, Deserialize)]
struct ExpectedActivityBucket {
    activity_id: String,
    bucket: amlich_core::almanac::recommendation::RecommendationBucket,
}

fn load_corpus() -> Corpus {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../amlich-core/data/almanac/recommendation-corpus-v1.json");
    let raw = fs::read_to_string(&path).expect("read corpus fixture");
    serde_json::from_str(&raw).expect("parse corpus fixture")
}

fn parse_ymd(ymd: &str) -> (i32, i32, i32) {
    let mut parts = ymd.split('-');
    let year = parts
        .next()
        .expect("year")
        .parse::<i32>()
        .expect("valid year");
    let month = parts
        .next()
        .expect("month")
        .parse::<i32>()
        .expect("valid month");
    let day = parts
        .next()
        .expect("day")
        .parse::<i32>()
        .expect("valid day");
    (day, month, year)
}

fn activity_id_to_snake_case(activity_id: ActivityId) -> String {
    match activity_id {
        ActivityId::Travel => "travel",
        ActivityId::MeetingSocial => "meeting_social",
        ActivityId::OpeningStart => "opening_start",
        ActivityId::ContractAgreement => "contract_agreement",
        ActivityId::BusinessTrade => "business_trade",
        ActivityId::FinanceInvestment => "finance_investment",
        ActivityId::ConstructionGroundbreaking => "construction_groundbreaking",
        ActivityId::RepairRenovation => "repair_renovation",
        ActivityId::MoveRelocation => "move_relocation",
        ActivityId::WeddingEngagement => "wedding_engagement",
        ActivityId::LawsuitDispute => "lawsuit_dispute",
        ActivityId::PrayerOffering => "prayer_offering",
        ActivityId::MedicalTreatment => "medical_treatment",
        ActivityId::BurialMemorial => "burial_memorial",
        ActivityId::CleaningPurging => "cleaning_purging",
    }
    .to_string()
}

#[test]
fn corpus_recommendations_match_between_core_and_api() {
    let corpus = load_corpus();

    for case in corpus.cases {
        let (day, month, year) = parse_ymd(&case.date);
        let query = DateQuery {
            day,
            month,
            year,
            timezone: Some(7.0),
        };

        let core_info = amlich_core::get_day_info(day, month, year);
        let api_info = amlich_api::get_day_info(&query).expect("api day info");

        assert_eq!(
            api_info.daily_recommendations.version, core_info.daily_recommendations.version,
            "{} version mismatch",
            case.id
        );
        assert_eq!(
            api_info.daily_recommendations.summary_vi, core_info.daily_recommendations.summary_vi,
            "{} summary mismatch",
            case.id
        );
        assert_eq!(
            api_info.daily_recommendations.activities.len(),
            core_info.daily_recommendations.activities.len(),
            "{} activity count mismatch",
            case.id
        );

        let core_ids: Vec<String> = core_info
            .daily_recommendations
            .activities
            .iter()
            .map(|a| activity_id_to_snake_case(a.activity_id))
            .collect();
        let api_ids: Vec<String> = api_info
            .daily_recommendations
            .activities
            .iter()
            .map(|a| a.activity_id.clone())
            .collect();
        assert_eq!(api_ids, core_ids, "{} activity ids mismatch", case.id);

        for (api_activity, core_activity) in api_info
            .daily_recommendations
            .activities
            .iter()
            .zip(core_info.daily_recommendations.activities.iter())
        {
            assert_eq!(
                api_activity.reasons.len(),
                core_activity.reasons.len(),
                "{} reasons len mismatch for {}",
                case.id,
                api_activity.activity_id
            );
        }

        for expected in &case.expect.must_match_activity_buckets {
            let core_activity = core_info
                .daily_recommendations
                .activities
                .iter()
                .find(|activity| activity_id_to_snake_case(activity.activity_id) == expected.activity_id)
                .unwrap_or_else(|| panic!("{} missing core activity {}", case.id, expected.activity_id));
            assert_eq!(
                core_activity.bucket, expected.bucket,
                "{} core expected bucket mismatch for {}",
                case.id, expected.activity_id
            );

            let api_activity = api_info
                .daily_recommendations
                .activities
                .iter()
                .find(|activity| activity.activity_id == expected.activity_id)
                .unwrap_or_else(|| panic!("{} missing api activity {}", case.id, expected.activity_id));
            let expected_api_bucket = match expected.bucket {
                RecommendationBucket::Nen => amlich_api::RecommendationBucketDto::Nen,
                RecommendationBucket::CoThe => amlich_api::RecommendationBucketDto::CoThe,
                RecommendationBucket::Tranh => amlich_api::RecommendationBucketDto::Tranh,
                RecommendationBucket::KyManh => amlich_api::RecommendationBucketDto::KyManh,
            };
            assert_eq!(
                api_activity.bucket, expected_api_bucket,
                "{} api expected bucket mismatch for {}",
                case.id, expected.activity_id
            );
        }

        let bundle = get_day_bundle(
            &query,
            &[
                Include::Base,
                Include::CanChi,
                Include::TietKhi,
                Include::Hours,
                Include::Fortune,
            ],
        )
        .expect("bundle");
        let bundle_rec = bundle
            .daily_recommendations
            .as_ref()
            .expect("bundle recommendations present");

        assert_eq!(
            bundle_rec.summary_vi, api_info.daily_recommendations.summary_vi,
            "{} bundle summary mismatch",
            case.id
        );

        // Bucket profile parity check protects enum conversion + serialization shape.
        let core_buckets = core_info.daily_recommendations.activities.iter().fold(
            (0usize, 0usize, 0usize, 0usize),
            |acc, activity| match activity.bucket {
                RecommendationBucket::Nen => (acc.0 + 1, acc.1, acc.2, acc.3),
                RecommendationBucket::CoThe => (acc.0, acc.1 + 1, acc.2, acc.3),
                RecommendationBucket::Tranh => (acc.0, acc.1, acc.2 + 1, acc.3),
                RecommendationBucket::KyManh => (acc.0, acc.1, acc.2, acc.3 + 1),
            },
        );
        let api_buckets = api_info.daily_recommendations.activities.iter().fold(
            (0usize, 0usize, 0usize, 0usize),
            |acc, activity| match activity.bucket {
                amlich_api::RecommendationBucketDto::Nen => (acc.0 + 1, acc.1, acc.2, acc.3),
                amlich_api::RecommendationBucketDto::CoThe => (acc.0, acc.1 + 1, acc.2, acc.3),
                amlich_api::RecommendationBucketDto::Tranh => (acc.0, acc.1, acc.2 + 1, acc.3),
                amlich_api::RecommendationBucketDto::KyManh => (acc.0, acc.1, acc.2, acc.3 + 1),
            },
        );
        assert_eq!(
            api_buckets, core_buckets,
            "{} bucket parity mismatch",
            case.id
        );
    }
}
