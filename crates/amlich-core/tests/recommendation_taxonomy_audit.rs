use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use amlich_core::almanac::recommendation::normalize_activity_alias;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TrucInsightFile {
    truc: Vec<TrucInsightRecord>,
}

#[derive(Debug, Deserialize)]
struct TrucInsightRecord {
    good_for: LocalizedList,
    avoid_for: LocalizedList,
}

#[derive(Debug, Deserialize)]
struct LocalizedList {
    vi: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CanChiFile {
    #[serde(rename = "dayGuidance")]
    day_guidance: BTreeMap<String, DayGuidanceRecord>,
}

#[derive(Debug, Deserialize)]
struct DayGuidanceRecord {
    #[serde(rename = "goodFor")]
    good_for: LocalizedList,
    #[serde(rename = "avoidFor")]
    avoid_for: LocalizedList,
}

#[test]
fn truc_alias_gap_set_is_known_and_documented() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/truc-insight.json");
    let raw = std::fs::read_to_string(path).expect("read truc insight file");
    let file: TrucInsightFile = serde_json::from_str(&raw).expect("parse truc insight file");

    let mut unknown = BTreeSet::new();
    for item in file.truc {
        for phrase in item.good_for.vi.iter().chain(item.avoid_for.vi.iter()) {
            if normalize_activity_alias(phrase).is_none() {
                unknown.insert(phrase.clone());
            }
        }
    }

    let expected: BTreeSet<String> = [
        "Bắt thú",
        "Bịt lỗ",
        "Cất giữ",
        "Cắt may",
        "Kết thúc công việc",
        "Phá dỡ",
        "Thu hoạch",
        "Tháo gỡ",
        "Trồng cây",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(unknown, expected);
}

#[test]
fn day_guidance_remains_mixed_actionable_and_informational() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data/canchi.json");
    let raw = std::fs::read_to_string(path).expect("read canchi file");
    let file: CanChiFile = serde_json::from_str(&raw).expect("parse canchi file");

    let mut total = 0usize;
    let mut mapped = 0usize;
    for record in file.day_guidance.into_values() {
        for phrase in record.good_for.vi.iter().chain(record.avoid_for.vi.iter()) {
            total += 1;
            if normalize_activity_alias(phrase).is_some() {
                mapped += 1;
            }
        }
    }

    assert!(mapped > 0, "expected at least some actionable phrases");
    assert!(
        mapped < total,
        "expected informational phrases to remain unmapped for v1"
    );
}
