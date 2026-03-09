use std::{fs, path::PathBuf};

use amlich_core::almanac::recommendation::{ActivityId, RecommendationBucket};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Corpus {
    version: String,
    cases: Vec<CorpusCase>,
}

#[derive(Debug, Deserialize)]
struct CorpusCase {
    id: String,
    date: String,
    rationale: String,
    expect: ExpectedCounts,
}

#[derive(Debug, Deserialize)]
struct ExpectedCounts {
    nen: usize,
    co_the: usize,
    tranh: usize,
    ky_manh: usize,
    summary_contains: String,
    must_include_activity_ids: Vec<String>,
    #[serde(default)]
    must_match_activity_buckets: Vec<ExpectedActivityBucket>,
}

#[derive(Debug, Deserialize)]
struct ExpectedActivityBucket {
    activity_id: String,
    bucket: RecommendationBucket,
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

fn load_corpus() -> Corpus {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data/almanac/recommendation-corpus-v1.json");
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

#[test]
fn recommendation_corpus_matches_expected_profiles() {
    let corpus = load_corpus();
    assert_eq!(corpus.version, "v1-layered");

    for case in corpus.cases {
        let (day, month, year) = parse_ymd(&case.date);
        let info = amlich_core::get_day_info(day, month, year);
        let rec = info.daily_recommendations;

        let nen = rec
            .activities
            .iter()
            .filter(|a| a.bucket == RecommendationBucket::Nen)
            .count();
        let co_the = rec
            .activities
            .iter()
            .filter(|a| a.bucket == RecommendationBucket::CoThe)
            .count();
        let tranh = rec
            .activities
            .iter()
            .filter(|a| a.bucket == RecommendationBucket::Tranh)
            .count();
        let ky_manh = rec
            .activities
            .iter()
            .filter(|a| a.bucket == RecommendationBucket::KyManh)
            .count();
        assert_eq!(nen, case.expect.nen, "{} nen mismatch", case.id);
        assert_eq!(co_the, case.expect.co_the, "{} co_the mismatch", case.id);
        assert_eq!(tranh, case.expect.tranh, "{} tranh mismatch", case.id);
        assert_eq!(ky_manh, case.expect.ky_manh, "{} ky_manh mismatch", case.id);
        assert!(
            rec.summary_vi.contains(&case.expect.summary_contains),
            "{} summary mismatch: {}",
            case.id,
            rec.summary_vi
        );

        let emitted_ids: Vec<String> = rec
            .activities
            .iter()
            .map(|activity| activity_id_to_snake_case(activity.activity_id))
            .collect();
        for required in &case.expect.must_include_activity_ids {
            assert!(
                emitted_ids.iter().any(|id| id == required),
                "{} missing required activity {} | rationale: {}",
                case.id,
                required,
                case.rationale
            );
        }

        for expected in &case.expect.must_match_activity_buckets {
            let actual = rec
                .activities
                .iter()
                .find(|activity| activity_id_to_snake_case(activity.activity_id) == expected.activity_id)
                .unwrap_or_else(|| panic!("{} missing bucket-checked activity {}", case.id, expected.activity_id));
            assert_eq!(
                actual.bucket, expected.bucket,
                "{} bucket mismatch for {} | rationale: {}",
                case.id, expected.activity_id, case.rationale
            );
        }
    }
}
