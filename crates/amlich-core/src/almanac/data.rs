use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

use serde::Deserialize;

use super::types::SourceMeta;
use crate::tietkhi::TIET_KHI;
use crate::types::{CAN, CHI};

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

#[derive(Debug, Clone, Deserialize)]
pub struct StarRuleBucketRaw {
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
    #[serde(default)]
    pub binh_tinh: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct StarRuleBucket {
    pub cat_tinh: Vec<String>,
    pub sat_tinh: Vec<String>,
    pub binh_tinh: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StarRuleMetaSet {
    pub fixed_by_chi: SourceMeta,
    pub fixed_by_canchi: SourceMeta,
    pub by_year: SourceMeta,
    pub by_month: SourceMeta,
    pub by_tiet_khi: SourceMeta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StarRuleSetsRaw {
    pub fixed_by_canchi: HashMap<String, StarRuleBucketRaw>,
    pub by_year_can: HashMap<String, StarRuleBucketRaw>,
    pub by_lunar_month: HashMap<String, StarRuleBucketRaw>,
    pub by_tiet_khi: HashMap<String, StarRuleBucketRaw>,
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
    pub star_rule_meta: StarRuleMetaSet,
    pub star_rules_fixed_by_canchi: HashMap<String, StarRuleBucket>,
    pub star_rules_by_year_can: HashMap<String, StarRuleBucket>,
    pub star_rules_by_lunar_month: HashMap<u8, StarRuleBucket>,
    pub star_rules_by_tiet_khi: HashMap<String, StarRuleBucket>,
}

#[derive(Debug, Clone, Deserialize)]
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
    star_rule_meta: StarRuleMetaSet,
    star_rule_sets: StarRuleSetsRaw,
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
            star_rule_meta: raw.star_rule_meta,
            star_rules_fixed_by_canchi: raw
                .star_rule_sets
                .fixed_by_canchi
                .into_iter()
                .map(|(k, v)| (k, normalize_star_rule_bucket(v)))
                .collect(),
            star_rules_by_year_can: raw
                .star_rule_sets
                .by_year_can
                .into_iter()
                .map(|(k, v)| (k, normalize_star_rule_bucket(v)))
                .collect(),
            star_rules_by_lunar_month: parse_lunar_month_rule_map(
                raw.star_rule_sets.by_lunar_month,
            ),
            star_rules_by_tiet_khi: raw
                .star_rule_sets
                .by_tiet_khi
                .into_iter()
                .map(|(k, v)| (k, normalize_star_rule_bucket(v)))
                .collect(),
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
    validate_conflict_stars(&raw.conflict_by_chi);
    validate_na_am_pairs(&raw.na_am_pairs);
    validate_nhi_thap_bat_tu(&raw.nhi_thap_bat_tu);
    validate_star_rule_meta(&raw.star_rule_meta);
    validate_star_rule_sets(&raw.star_rule_sets);
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

fn validate_star_rule_meta(meta: &StarRuleMetaSet) {
    validate_source_meta(&meta.fixed_by_chi, "star_rule_meta.fixed_by_chi");
    validate_source_meta(&meta.fixed_by_canchi, "star_rule_meta.fixed_by_canchi");
    validate_source_meta(&meta.by_year, "star_rule_meta.by_year");
    validate_source_meta(&meta.by_month, "star_rule_meta.by_month");
    validate_source_meta(&meta.by_tiet_khi, "star_rule_meta.by_tiet_khi");
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

fn validate_conflict_stars(map: &HashMap<String, ConflictRule>) {
    for (chi, rule) in map {
        assert!(
            !rule.cat_tinh.is_empty(),
            "conflict_by_chi[{chi}].cat_tinh must not be empty"
        );
        assert!(
            !rule.sat_tinh.is_empty(),
            "conflict_by_chi[{chi}].sat_tinh must not be empty"
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

fn normalize_star_rule_bucket(raw: StarRuleBucketRaw) -> StarRuleBucket {
    StarRuleBucket {
        cat_tinh: raw.cat_tinh,
        sat_tinh: raw.sat_tinh,
        binh_tinh: raw.binh_tinh,
    }
}

fn parse_lunar_month_rule_map(
    raw: HashMap<String, StarRuleBucketRaw>,
) -> HashMap<u8, StarRuleBucket> {
    raw.into_iter()
        .map(|(month, bucket)| {
            let value = month
                .parse::<u8>()
                .expect("star_rule_sets.by_lunar_month key must be a numeric month string");
            (value, normalize_star_rule_bucket(bucket))
        })
        .collect()
}

fn validate_star_rule_sets(sets: &StarRuleSetsRaw) {
    validate_fixed_by_canchi_map(&sets.fixed_by_canchi);
    validate_by_year_can_map(&sets.by_year_can);
    validate_by_lunar_month_map(&sets.by_lunar_month);
    validate_by_tiet_khi_map(&sets.by_tiet_khi);
}

fn validate_fixed_by_canchi_map(map: &HashMap<String, StarRuleBucketRaw>) {
    for (key, bucket) in map {
        assert!(
            is_valid_sexagenary_key(key),
            "star_rule_sets.fixed_by_canchi contains invalid canchi key: {key}"
        );
        validate_star_rule_bucket(bucket, &format!("star_rule_sets.fixed_by_canchi[{key}]"));
    }
}

fn validate_by_year_can_map(map: &HashMap<String, StarRuleBucketRaw>) {
    for (key, bucket) in map {
        assert!(
            CAN.contains(&key.as_str()),
            "star_rule_sets.by_year_can contains invalid can key: {key}"
        );
        validate_star_rule_bucket(bucket, &format!("star_rule_sets.by_year_can[{key}]"));
    }
}

fn validate_by_lunar_month_map(map: &HashMap<String, StarRuleBucketRaw>) {
    for (key, bucket) in map {
        let month = key
            .parse::<u8>()
            .expect("star_rule_sets.by_lunar_month key must be numeric");
        assert!(
            (1..=12).contains(&month),
            "star_rule_sets.by_lunar_month key out of range 1..12: {key}"
        );
        validate_star_rule_bucket(bucket, &format!("star_rule_sets.by_lunar_month[{key}]"));
    }
}

fn validate_by_tiet_khi_map(map: &HashMap<String, StarRuleBucketRaw>) {
    for (key, bucket) in map {
        assert!(
            is_valid_tiet_khi_name(key),
            "star_rule_sets.by_tiet_khi contains unknown tiet khi key: {key}"
        );
        validate_star_rule_bucket(bucket, &format!("star_rule_sets.by_tiet_khi[{key}]"));
    }
}

fn validate_star_rule_bucket(bucket: &StarRuleBucketRaw, path: &str) {
    validate_nonempty_star_names(&bucket.cat_tinh, &format!("{path}.cat_tinh"));
    validate_nonempty_star_names(&bucket.sat_tinh, &format!("{path}.sat_tinh"));
    validate_nonempty_star_names(&bucket.binh_tinh, &format!("{path}.binh_tinh"));

    let mut seen = HashSet::new();
    for star in &bucket.cat_tinh {
        assert!(
            seen.insert(star),
            "{path}: duplicate star across categories: {star}"
        );
    }
    for star in &bucket.sat_tinh {
        assert!(
            seen.insert(star),
            "{path}: duplicate star across categories: {star}"
        );
    }
    for star in &bucket.binh_tinh {
        assert!(
            seen.insert(star),
            "{path}: duplicate star across categories: {star}"
        );
    }
}

fn validate_nonempty_star_names(stars: &[String], path: &str) {
    for star in stars {
        assert!(
            !star.trim().is_empty(),
            "{path} must not contain empty star names"
        );
    }
}

fn is_valid_tiet_khi_name(name: &str) -> bool {
    TIET_KHI.iter().any(|term| term.name == name)
}

fn is_valid_sexagenary_key(key: &str) -> bool {
    (0..60).any(|i| key == format!("{} {}", CAN[i % 10], CHI[i % 12]))
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
    fn rejects_invalid_method_tokens() {
        assert!(
            !is_valid_method("BAD_METHOD"),
            "unknown token must be rejected"
        );
        assert!(
            !is_valid_method("NORTH"),
            "English direction must be rejected"
        );
        assert!(!is_valid_method(""), "empty string must be rejected");
        assert!(is_valid_method("table-lookup"));
        assert!(is_valid_method("bai-quyet"));
        assert!(is_valid_method("jd-cycle"));
    }

    #[test]
    fn rejects_invalid_direction_tokens() {
        assert!(
            !is_valid_direction("NORTH"),
            "English direction must be rejected"
        );
        assert!(!is_valid_direction("North"), "mixed-case must be rejected");
        assert!(!is_valid_direction(""), "empty string must be rejected");
        assert!(is_valid_direction("Bắc"));
        assert!(is_valid_direction("Đông Bắc"));
        assert!(is_valid_direction("Tây Nam"));
    }

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

    #[test]
    fn validates_star_rule_schema_loads() {
        let data = baseline_data();
        assert!(data.star_rule_meta.fixed_by_chi.source_id == "khcbppt");
        assert!(data.star_rules_fixed_by_canchi.contains_key("Giáp Thìn"));
        assert!(data.star_rules_by_year_can.contains_key("Giáp"));
        assert!(data.star_rules_by_lunar_month.contains_key(&1));
        assert!(data.star_rules_by_tiet_khi.contains_key("Lập Xuân"));
    }

    #[test]
    fn accepts_known_tiet_khi_names() {
        assert!(is_valid_tiet_khi_name("Lập Xuân"));
        assert!(is_valid_tiet_khi_name("Thanh Minh"));
        assert!(is_valid_tiet_khi_name("Đông Chí"));
    }

    #[test]
    fn rejects_unknown_tiet_khi_names() {
        assert!(!is_valid_tiet_khi_name("Lap Xuan"));
        assert!(!is_valid_tiet_khi_name("Unknown Term"));
    }

    #[test]
    fn validates_sexagenary_key_tokens() {
        assert!(is_valid_sexagenary_key("Giáp Tý"));
        assert!(is_valid_sexagenary_key("Quý Hợi"));
        assert!(!is_valid_sexagenary_key("Giáp Unknown"));
        assert!(!is_valid_sexagenary_key("Unknown Tý"));
    }
}
