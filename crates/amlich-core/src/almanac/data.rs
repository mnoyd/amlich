use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use serde::Deserialize;

use crate::types::{CAN, CHI};
use super::types::SourceMeta;

const BASELINE_JSON: &str = include_str!("../../data/almanac/baseline.json");
const VALID_DIRECTIONS: [&str; 8] = [
    "Bắc",
    "Đông Bắc",
    "Đông",
    "Đông Nam",
    "Nam",
    "Tây Nam",
    "Tây",
    "Tây Bắc",
];

#[derive(Debug, Clone, Deserialize)]
pub struct TravelRule {
    pub xuat_hanh_huong: String,
    pub tai_than: String,
    pub hy_than: String,
    pub ky_than: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConflictRule {
    pub opposing_chi: String,
    pub sat_huong: String,
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NaAmEntry {
    pub can: String,
    pub chi: String,
    pub na_am: String,
    pub element: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DayStarRule {
    pub name: String,
    pub quality: String,
}

#[derive(Debug, Clone)]
pub struct AlmanacData {
    pub profile: String,
    pub travel_meta: SourceMeta,
    pub conflict_meta: SourceMeta,
    pub na_am_meta: SourceMeta,
    pub star_meta: SourceMeta,
    pub travel_by_can: HashMap<String, TravelRule>,
    pub conflict_by_chi: HashMap<String, ConflictRule>,
    pub sexagenary_na_am: HashMap<String, NaAmEntry>,
    pub nhi_thap_bat_tu: Vec<DayStarRule>,
}

#[derive(Debug, Deserialize)]
struct RawAlmanacData {
    profile: String,
    travel_meta: SourceMeta,
    conflict_meta: SourceMeta,
    na_am_meta: SourceMeta,
    star_meta: SourceMeta,
    travel_by_can: HashMap<String, TravelRule>,
    conflict_by_chi: HashMap<String, ConflictRule>,
    na_am_pairs: Vec<String>,
    nhi_thap_bat_tu: Vec<DayStarRule>,
}

static BASELINE_DATA: OnceLock<AlmanacData> = OnceLock::new();

pub fn baseline_data() -> &'static AlmanacData {
    BASELINE_DATA.get_or_init(|| {
        let raw: RawAlmanacData =
            serde_json::from_str(BASELINE_JSON).expect("Failed to parse baseline almanac data");

        validate_raw_data(&raw);

        AlmanacData {
            profile: raw.profile,
            travel_meta: raw.travel_meta,
            conflict_meta: raw.conflict_meta,
            na_am_meta: raw.na_am_meta,
            star_meta: raw.star_meta,
            travel_by_can: raw.travel_by_can,
            conflict_by_chi: raw.conflict_by_chi,
            sexagenary_na_am: expand_sexagenary_na_am(&raw.na_am_pairs),
            nhi_thap_bat_tu: raw.nhi_thap_bat_tu,
        }
    })
}

fn validate_raw_data(raw: &RawAlmanacData) {
    validate_source_meta(&raw.travel_meta, "travel_meta");
    validate_source_meta(&raw.conflict_meta, "conflict_meta");
    validate_source_meta(&raw.na_am_meta, "na_am_meta");
    validate_source_meta(&raw.star_meta, "star_meta");
    validate_can_map(&raw.travel_by_can);
    validate_chi_map(&raw.conflict_by_chi);
    validate_directions(raw);
    validate_na_am_pairs(&raw.na_am_pairs);
    validate_nhi_thap_bat_tu(&raw.nhi_thap_bat_tu);
}

const VALID_METHODS: [&str; 3] = ["table-lookup", "bai-quyet", "jd-cycle"];

pub fn is_valid_method(method: &str) -> bool {
    VALID_METHODS.contains(&method)
}

fn validate_source_meta(meta: &SourceMeta, field: &str) {
    assert!(
        !meta.source_id.is_empty(),
        "{field}.source_id must not be empty"
    );
    assert!(
        is_valid_method(&meta.method),
        "{field}.method '{}' is not a valid method token",
        meta.method
    );
}

fn validate_can_map(map: &HashMap<String, TravelRule>) {
    let expected: HashSet<&str> = CAN.iter().copied().collect();
    let actual: HashSet<&str> = map.keys().map(String::as_str).collect();
    assert_eq!(
        actual, expected,
        "travel_by_can must contain exactly 10 can"
    );
}

fn validate_chi_map(map: &HashMap<String, ConflictRule>) {
    let expected: HashSet<&str> = CHI.iter().copied().collect();
    let actual: HashSet<&str> = map.keys().map(String::as_str).collect();
    assert_eq!(
        actual, expected,
        "conflict_by_chi must contain exactly 12 chi"
    );
}

fn validate_directions(raw: &RawAlmanacData) {
    for rule in raw.travel_by_can.values() {
        assert!(
            is_valid_direction(&rule.xuat_hanh_huong),
            "invalid direction for xuat_hanh_huong: {}",
            rule.xuat_hanh_huong
        );
        assert!(
            is_valid_direction(&rule.tai_than),
            "invalid direction for tai_than: {}",
            rule.tai_than
        );
        assert!(
            is_valid_direction(&rule.hy_than),
            "invalid direction for hy_than: {}",
            rule.hy_than
        );
        if let Some(ky_than) = &rule.ky_than {
            assert!(
                is_valid_direction(ky_than),
                "invalid direction for ky_than: {}",
                ky_than
            );
        }
    }

    for rule in raw.conflict_by_chi.values() {
        assert!(
            is_valid_direction(&rule.sat_huong),
            "invalid direction for sat_huong: {}",
            rule.sat_huong
        );
    }
}

fn validate_na_am_pairs(values: &[String]) {
    assert_eq!(
        values.len(),
        30,
        "na_am_pairs must contain exactly 30 items"
    );
    for value in values {
        assert!(!value.trim().is_empty(), "na_am_pairs cannot contain empty");
    }
}

fn validate_nhi_thap_bat_tu(values: &[DayStarRule]) {
    assert_eq!(
        values.len(),
        28,
        "nhi_thap_bat_tu must contain exactly 28 stars"
    );
    for star in values {
        assert!(!star.name.trim().is_empty(), "star name cannot be empty");
        assert!(
            matches!(star.quality.as_str(), "cat" | "hung" | "binh"),
            "invalid star quality: {}",
            star.quality
        );
    }
}

pub fn is_valid_direction(direction: &str) -> bool {
    VALID_DIRECTIONS.contains(&direction)
}

fn expand_sexagenary_na_am(na_am_pairs: &[String]) -> HashMap<String, NaAmEntry> {
    let mut out = HashMap::with_capacity(60);
    for i in 0..60 {
        let can = CAN[i % 10];
        let chi = CHI[i % 12];
        let na_am = na_am_pairs[i / 2].clone();
        let element = na_am.split_whitespace().last().unwrap_or("").to_string();
        out.insert(
            format!("{can} {chi}"),
            NaAmEntry {
                can: can.to_string(),
                chi: chi.to_string(),
                na_am,
                element,
            },
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_metadata_fields_exist() {
        let data = baseline_data();
        assert!(
            !data.travel_meta.source_id.is_empty(),
            "travel_meta.source_id must not be empty"
        );
        assert!(
            !data.travel_meta.method.is_empty(),
            "travel_meta.method must not be empty"
        );
        assert!(
            !data.conflict_meta.source_id.is_empty(),
            "conflict_meta.source_id must not be empty"
        );
        assert!(
            !data.conflict_meta.method.is_empty(),
            "conflict_meta.method must not be empty"
        );
        assert!(
            !data.na_am_meta.source_id.is_empty(),
            "na_am_meta.source_id must not be empty"
        );
        assert!(
            !data.na_am_meta.method.is_empty(),
            "na_am_meta.method must not be empty"
        );
        assert!(
            !data.star_meta.source_id.is_empty(),
            "star_meta.source_id must not be empty"
        );
        assert!(
            !data.star_meta.method.is_empty(),
            "star_meta.method must not be empty"
        );
    }

    #[test]
    fn validates_expected_collection_sizes() {
        let data = baseline_data();
        assert_eq!(data.travel_by_can.len(), 10);
        assert_eq!(data.conflict_by_chi.len(), 12);
        assert_eq!(data.sexagenary_na_am.len(), 60);
    }

    #[test]
    fn validates_direction_tokens() {
        let data = baseline_data();
        for entry in data.travel_by_can.values() {
            assert!(is_valid_direction(&entry.xuat_hanh_huong));
            assert!(is_valid_direction(&entry.tai_than));
            assert!(is_valid_direction(&entry.hy_than));
        }
    }

    #[test]
    fn validates_na_am_cycle_examples() {
        let data = baseline_data();
        assert_eq!(
            data.sexagenary_na_am
                .get("Giáp Tý")
                .map(|v| v.na_am.as_str()),
            Some("Hải Trung Kim")
        );
        assert_eq!(
            data.sexagenary_na_am
                .get("Ất Sửu")
                .map(|v| v.na_am.as_str()),
            Some("Hải Trung Kim")
        );
        assert_eq!(
            data.sexagenary_na_am
                .get("Bính Dần")
                .map(|v| v.na_am.as_str()),
            Some("Lư Trung Hỏa")
        );
    }
}
