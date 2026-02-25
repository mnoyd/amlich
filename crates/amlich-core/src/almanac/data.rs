use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::sync::OnceLock;

use serde::Deserialize;

use super::types::{
    RuleSetDefaults, RuleSetDescriptor as RulesetDescriptorDoc, RuleSetSourceNote, SourceMeta,
};
use crate::tietkhi::TIET_KHI;
use crate::types::{CAN, CHI};

const BASELINE_JSON: &str = include_str!("../../data/almanac/baseline.json");
pub const DEFAULT_RULESET_ID: &str = "vn_baseline_v1";
const BASELINE_RULESET_ALIAS: &str = "baseline";
const DEFAULT_RULESET_VERSION: &str = "v1";
const DEFAULT_RULESET_REGION: &str = "vn";
const DEFAULT_RULESET_SCHEMA_VERSION: &str = "ruleset-descriptor/v1";
const DEFAULT_RULESET_TZ_OFFSET: f64 = 7.0;
const VALID_REGIONS: [&str; 1] = ["vn"];
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
pub struct DayDeityMeta {
    pub source_id: String,
    pub method: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DayDeityRuleRaw {
    pub name: String,
    pub classification: String,
}

#[derive(Debug, Clone)]
pub struct DayDeityRule {
    pub name: String,
    pub classification: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DayDeityRuleSetRaw {
    pub cycle: Vec<DayDeityRuleRaw>,
    pub month_group_start_by_chi: HashMap<String, usize>,
}

#[derive(Debug, Clone)]
pub struct DayDeityRuleSet {
    pub cycle: Vec<DayDeityRule>,
    pub month_group_start_by_chi: HashMap<String, usize>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TabooRuleMetaSet {
    pub tam_nuong: SourceMeta,
    pub nguyet_ky: SourceMeta,
    pub sat_chu: SourceMeta,
    pub tho_tu: SourceMeta,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TabooDayRuleRaw {
    pub rule_id: String,
    pub name: String,
    pub severity: String,
    pub lunar_days: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct TabooDayRule {
    pub rule_id: String,
    pub name: String,
    pub severity: String,
    pub lunar_days: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TabooMonthChiRuleRaw {
    pub rule_id: String,
    pub name: String,
    pub severity: String,
    pub by_lunar_month: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct TabooMonthChiRule {
    pub rule_id: String,
    pub name: String,
    pub severity: String,
    pub by_lunar_month: HashMap<u8, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TabooRuleSetsRaw {
    pub tam_nuong: TabooDayRuleRaw,
    pub nguyet_ky: TabooDayRuleRaw,
    pub sat_chu: TabooMonthChiRuleRaw,
    pub tho_tu: TabooMonthChiRuleRaw,
}

#[derive(Debug, Clone)]
pub struct TabooRuleSets {
    pub tam_nuong: TabooDayRule,
    pub nguyet_ky: TabooDayRule,
    pub sat_chu: TabooMonthChiRule,
    pub tho_tu: TabooMonthChiRule,
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
    pub day_deity_meta: SourceMeta,
    pub taboo_rule_meta: TabooRuleMetaSet,
    pub travel_by_can: HashMap<String, TravelRule>,
    pub conflict_by_chi: HashMap<String, ConflictRule>,
    pub sexagenary_na_am: HashMap<String, NaAmEntry>,
    pub nhi_thap_bat_tu: Vec<DayStarRule>,
    pub star_rule_meta: StarRuleMetaSet,
    pub star_rules_fixed_by_canchi: HashMap<String, StarRuleBucket>,
    pub star_rules_by_year_can: HashMap<String, StarRuleBucket>,
    pub star_rules_by_lunar_month: HashMap<u8, StarRuleBucket>,
    pub star_rules_by_tiet_khi: HashMap<String, StarRuleBucket>,
    pub day_deity_rule_set: DayDeityRuleSet,
    pub taboo_rules: TabooRuleSets,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RulesetDescriptor {
    pub id: &'static str,
    pub version: &'static str,
    pub region: &'static str,
    pub profile: &'static str,
}

impl RulesetDescriptor {
    pub fn to_document_descriptor(self) -> RulesetDescriptorDoc {
        RulesetDescriptorDoc {
            id: self.id.to_string(),
            version: self.version.to_string(),
            region: self.region.to_string(),
            profile: self.profile.to_string(),
            defaults: RuleSetDefaults {
                tz_offset: DEFAULT_RULESET_TZ_OFFSET,
                meridian: None,
            },
            source_notes: vec![
                RuleSetSourceNote {
                    family: "travel".to_string(),
                    source_id: "khcbppt".to_string(),
                    note: "Direction table and bai quyet mapping".to_string(),
                },
                RuleSetSourceNote {
                    family: "taboo_rules".to_string(),
                    source_id: "khcbppt".to_string(),
                    note: "Tam Nuong/Nguyet Ky/Sat Chu/Tho Tu frozen for v1".to_string(),
                },
            ],
            schema_version: DEFAULT_RULESET_SCHEMA_VERSION.to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RulesetRegistryEntry {
    pub descriptor: RulesetDescriptor,
    pub aliases: &'static [&'static str],
    loader: fn() -> &'static AlmanacData,
}

impl RulesetRegistryEntry {
    pub fn data(&self) -> &'static AlmanacData {
        (self.loader)()
    }

    fn matches_id(&self, ruleset_id: &str) -> bool {
        self.descriptor.id == ruleset_id || self.aliases.contains(&ruleset_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RulesetLookupError {
    UnknownRulesetId(String),
}

impl fmt::Display for RulesetLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRulesetId(id) => write!(f, "unknown almanac ruleset id: {id}"),
        }
    }
}

impl Error for RulesetLookupError {}

#[derive(Debug, Clone, Deserialize)]
struct RawAlmanacData {
    profile: String,
    travel_meta: SourceMeta,
    conflict_meta: SourceMeta,
    na_am_meta: SourceMeta,
    star_meta: SourceMeta,
    day_deity_meta: SourceMeta,
    taboo_rule_meta: TabooRuleMetaSet,
    travel_by_can: HashMap<String, TravelRule>,
    conflict_by_chi: HashMap<String, ConflictRule>,
    na_am_pairs: Vec<String>,
    nhi_thap_bat_tu: Vec<DayStarRule>,
    star_rule_meta: StarRuleMetaSet,
    star_rule_sets: StarRuleSetsRaw,
    day_deity_rule_set: DayDeityRuleSetRaw,
    taboo_rule_sets: TabooRuleSetsRaw,
}

static BASELINE_DATA: OnceLock<AlmanacData> = OnceLock::new();

static RULESET_REGISTRY: [RulesetRegistryEntry; 1] = [RulesetRegistryEntry {
    descriptor: RulesetDescriptor {
        id: DEFAULT_RULESET_ID,
        version: DEFAULT_RULESET_VERSION,
        region: DEFAULT_RULESET_REGION,
        profile: BASELINE_RULESET_ALIAS,
    },
    aliases: &[BASELINE_RULESET_ALIAS],
    loader: baseline_data,
}];

pub fn default_ruleset() -> &'static RulesetRegistryEntry {
    &RULESET_REGISTRY[0]
}

pub fn get_ruleset(ruleset_id: &str) -> Result<&'static RulesetRegistryEntry, RulesetLookupError> {
    RULESET_REGISTRY
        .iter()
        .find(|entry| entry.matches_id(ruleset_id))
        .ok_or_else(|| RulesetLookupError::UnknownRulesetId(ruleset_id.to_string()))
}

pub fn get_ruleset_data(ruleset_id: &str) -> Result<&'static AlmanacData, RulesetLookupError> {
    get_ruleset(ruleset_id).map(RulesetRegistryEntry::data)
}

pub fn get_ruleset_descriptor_doc(
    ruleset_id: &str,
) -> Result<RulesetDescriptorDoc, RulesetLookupError> {
    let descriptor =
        get_ruleset(ruleset_id).map(|entry| entry.descriptor.to_document_descriptor())?;
    validate_ruleset_descriptor_doc(&descriptor);
    Ok(descriptor)
}

fn validate_ruleset_descriptor_doc(descriptor: &RulesetDescriptorDoc) {
    assert!(
        !descriptor.id.trim().is_empty(),
        "ruleset descriptor id must not be empty"
    );
    assert!(
        !descriptor.version.trim().is_empty(),
        "ruleset descriptor version must not be empty"
    );
    assert!(
        VALID_REGIONS.contains(&descriptor.region.as_str()),
        "ruleset descriptor region '{}' is not supported",
        descriptor.region
    );
    assert!(
        !descriptor.profile.trim().is_empty(),
        "ruleset descriptor profile must not be empty"
    );
    assert!(
        matches!(descriptor.defaults.tz_offset, -12.0..=14.0),
        "ruleset descriptor defaults.tz_offset must be in -12..14"
    );
    if let Some(meridian) = &descriptor.defaults.meridian {
        assert!(
            !meridian.trim().is_empty(),
            "ruleset descriptor defaults.meridian must not be empty when provided"
        );
    }
    assert_eq!(
        descriptor.schema_version, DEFAULT_RULESET_SCHEMA_VERSION,
        "ruleset descriptor schema_version must be '{}'",
        DEFAULT_RULESET_SCHEMA_VERSION
    );
    validate_ruleset_source_notes(&descriptor.source_notes);
}

fn validate_ruleset_source_notes(notes: &[RuleSetSourceNote]) {
    let mut seen = HashSet::new();
    for note in notes {
        assert!(
            !note.family.trim().is_empty(),
            "ruleset descriptor source note family must not be empty"
        );
        assert!(
            !note.source_id.trim().is_empty(),
            "ruleset descriptor source note source_id must not be empty"
        );
        assert!(
            !note.note.trim().is_empty(),
            "ruleset descriptor source note note must not be empty"
        );
        assert!(
            seen.insert(note.family.as_str()),
            "ruleset descriptor source notes contain duplicate family: {}",
            note.family
        );
    }
}

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
            day_deity_meta: raw.day_deity_meta,
            taboo_rule_meta: raw.taboo_rule_meta,
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
            day_deity_rule_set: normalize_day_deity_rule_set(raw.day_deity_rule_set),
            taboo_rules: normalize_taboo_rule_sets(raw.taboo_rule_sets),
        }
    })
}

fn validate_raw_data(raw: &RawAlmanacData) {
    validate_source_meta(&raw.travel_meta, "travel_meta");
    validate_source_meta(&raw.conflict_meta, "conflict_meta");
    validate_source_meta(&raw.na_am_meta, "na_am_meta");
    validate_source_meta(&raw.star_meta, "star_meta");
    validate_source_meta(&raw.day_deity_meta, "day_deity_meta");
    validate_taboo_rule_meta(&raw.taboo_rule_meta);
    validate_can_map(&raw.travel_by_can);
    validate_chi_map(&raw.conflict_by_chi);
    validate_directions(raw);
    validate_conflict_stars(&raw.conflict_by_chi);
    validate_na_am_pairs(&raw.na_am_pairs);
    validate_nhi_thap_bat_tu(&raw.nhi_thap_bat_tu);
    validate_star_rule_meta(&raw.star_rule_meta);
    validate_star_rule_sets(&raw.star_rule_sets);
    validate_day_deity_rule_set(&raw.day_deity_rule_set);
    validate_taboo_rule_sets(&raw.taboo_rule_sets);
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

fn validate_taboo_rule_meta(meta: &TabooRuleMetaSet) {
    validate_source_meta(&meta.tam_nuong, "taboo_rule_meta.tam_nuong");
    validate_source_meta(&meta.nguyet_ky, "taboo_rule_meta.nguyet_ky");
    validate_source_meta(&meta.sat_chu, "taboo_rule_meta.sat_chu");
    validate_source_meta(&meta.tho_tu, "taboo_rule_meta.tho_tu");
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

fn normalize_day_deity_rule_set(raw: DayDeityRuleSetRaw) -> DayDeityRuleSet {
    DayDeityRuleSet {
        cycle: raw
            .cycle
            .into_iter()
            .map(|entry| DayDeityRule {
                name: entry.name,
                classification: entry.classification,
            })
            .collect(),
        month_group_start_by_chi: raw.month_group_start_by_chi,
    }
}

fn normalize_taboo_rule_sets(raw: TabooRuleSetsRaw) -> TabooRuleSets {
    TabooRuleSets {
        tam_nuong: TabooDayRule {
            rule_id: raw.tam_nuong.rule_id,
            name: raw.tam_nuong.name,
            severity: raw.tam_nuong.severity,
            lunar_days: raw.tam_nuong.lunar_days,
        },
        nguyet_ky: TabooDayRule {
            rule_id: raw.nguyet_ky.rule_id,
            name: raw.nguyet_ky.name,
            severity: raw.nguyet_ky.severity,
            lunar_days: raw.nguyet_ky.lunar_days,
        },
        sat_chu: TabooMonthChiRule {
            rule_id: raw.sat_chu.rule_id,
            name: raw.sat_chu.name,
            severity: raw.sat_chu.severity,
            by_lunar_month: parse_taboo_month_chi_map(raw.sat_chu.by_lunar_month),
        },
        tho_tu: TabooMonthChiRule {
            rule_id: raw.tho_tu.rule_id,
            name: raw.tho_tu.name,
            severity: raw.tho_tu.severity,
            by_lunar_month: parse_taboo_month_chi_map(raw.tho_tu.by_lunar_month),
        },
    }
}

fn parse_taboo_month_chi_map(raw: HashMap<String, String>) -> HashMap<u8, String> {
    raw.into_iter()
        .map(|(month, chi)| {
            let value = month
                .parse::<u8>()
                .expect("taboo by_lunar_month key must be a numeric month string");
            (value, chi)
        })
        .collect()
}

fn validate_star_rule_sets(sets: &StarRuleSetsRaw) {
    validate_fixed_by_canchi_map(&sets.fixed_by_canchi);
    validate_by_year_can_map(&sets.by_year_can);
    validate_by_lunar_month_map(&sets.by_lunar_month);
    validate_by_tiet_khi_map(&sets.by_tiet_khi);
}

fn validate_day_deity_rule_set(rule_set: &DayDeityRuleSetRaw) {
    assert_eq!(
        rule_set.cycle.len(),
        12,
        "day_deity_rule_set.cycle must contain exactly 12 entries"
    );

    for (idx, entry) in rule_set.cycle.iter().enumerate() {
        assert!(
            !entry.name.trim().is_empty(),
            "day_deity_rule_set.cycle[{idx}].name must not be empty"
        );
        assert!(
            matches!(entry.classification.as_str(), "hoang_dao" | "hac_dao"),
            "day_deity_rule_set.cycle[{idx}].classification must be hoang_dao|hac_dao"
        );
    }

    let expected: HashSet<&str> = CHI.iter().copied().collect();
    let actual: HashSet<&str> = rule_set
        .month_group_start_by_chi
        .keys()
        .map(String::as_str)
        .collect();
    assert_eq!(
        actual, expected,
        "day_deity_rule_set.month_group_start_by_chi must contain all 12 chi keys"
    );

    for (chi, start) in &rule_set.month_group_start_by_chi {
        assert!(
            *start < 12,
            "day_deity_rule_set.month_group_start_by_chi[{chi}] must be in 0..12"
        );
    }
}

fn validate_taboo_rule_sets(sets: &TabooRuleSetsRaw) {
    validate_taboo_day_rule(&sets.tam_nuong, "taboo_rule_sets.tam_nuong", "tam_nuong");
    validate_taboo_day_rule(&sets.nguyet_ky, "taboo_rule_sets.nguyet_ky", "nguyet_ky");
    validate_taboo_month_chi_rule(&sets.sat_chu, "taboo_rule_sets.sat_chu", "sat_chu");
    validate_taboo_month_chi_rule(&sets.tho_tu, "taboo_rule_sets.tho_tu", "tho_tu");
}

fn validate_taboo_day_rule(rule: &TabooDayRuleRaw, path: &str, expected_rule_id: &str) {
    validate_taboo_common_fields(
        &rule.rule_id,
        &rule.name,
        &rule.severity,
        path,
        expected_rule_id,
    );
    assert!(
        !rule.lunar_days.is_empty(),
        "{path}.lunar_days must not be empty"
    );

    let mut seen = HashSet::new();
    for day in &rule.lunar_days {
        assert!(
            (1..=30).contains(day),
            "{path}.lunar_days contains out-of-range lunar day: {day}"
        );
        assert!(
            seen.insert(*day),
            "{path}.lunar_days contains duplicate lunar day: {day}"
        );
    }
}

fn validate_taboo_month_chi_rule(rule: &TabooMonthChiRuleRaw, path: &str, expected_rule_id: &str) {
    validate_taboo_common_fields(
        &rule.rule_id,
        &rule.name,
        &rule.severity,
        path,
        expected_rule_id,
    );
    assert_eq!(
        rule.by_lunar_month.len(),
        12,
        "{path}.by_lunar_month must contain exactly 12 months"
    );

    let expected_months: HashSet<u8> = (1..=12).collect();
    let mut actual_months = HashSet::new();

    for (month, chi) in &rule.by_lunar_month {
        let month_num = month
            .parse::<u8>()
            .expect("taboo by_lunar_month key must be a numeric month string");
        assert!(
            (1..=12).contains(&month_num),
            "{path}.by_lunar_month key out of range 1..12: {month}"
        );
        actual_months.insert(month_num);
        assert!(
            CHI.contains(&chi.as_str()),
            "{path}.by_lunar_month[{month}] contains invalid chi: {chi}"
        );
    }

    assert_eq!(
        actual_months, expected_months,
        "{path}.by_lunar_month must define all lunar months 1..12"
    );
}

fn validate_taboo_common_fields(
    rule_id: &str,
    name: &str,
    severity: &str,
    path: &str,
    expected_rule_id: &str,
) {
    assert_eq!(
        rule_id, expected_rule_id,
        "{path}.rule_id must be '{expected_rule_id}'"
    );
    assert!(!name.trim().is_empty(), "{path}.name must not be empty");
    assert!(
        is_valid_taboo_severity(severity),
        "{path}.severity must be one of 'hard' | 'soft' (got '{severity}')"
    );
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

fn is_valid_taboo_severity(value: &str) -> bool {
    matches!(value, "hard" | "soft")
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
    fn validates_day_deity_rule_schema_loads() {
        let data = baseline_data();
        assert_eq!(data.day_deity_meta.source_id, "khcbppt");
        assert_eq!(data.day_deity_rule_set.cycle.len(), 12);
        assert_eq!(data.day_deity_rule_set.cycle[0].name, "Thanh Long");
        assert_eq!(
            data.day_deity_rule_set
                .month_group_start_by_chi
                .get("Dần")
                .copied(),
            Some(0)
        );
        assert_eq!(
            data.day_deity_rule_set
                .month_group_start_by_chi
                .get("Tý")
                .copied(),
            Some(8)
        );
    }

    #[test]
    fn resolves_default_ruleset_by_canonical_id() {
        let entry = get_ruleset(DEFAULT_RULESET_ID).expect("canonical ruleset lookup");
        assert_eq!(entry.descriptor.id, "vn_baseline_v1");
        assert_eq!(entry.descriptor.version, "v1");
        assert_eq!(entry.descriptor.region, "vn");
        assert_eq!(entry.descriptor.profile, "baseline");
    }

    #[test]
    fn resolves_baseline_alias_to_same_ruleset() {
        let alias = get_ruleset("baseline").expect("alias lookup");
        let canonical = get_ruleset(DEFAULT_RULESET_ID).expect("canonical lookup");
        assert_eq!(alias.descriptor.id, canonical.descriptor.id);
        assert!(std::ptr::eq(alias.data(), canonical.data()));
    }

    #[test]
    fn rejects_unknown_ruleset_id() {
        let err = get_ruleset("does-not-exist").expect_err("unknown ruleset must fail");
        assert_eq!(
            err.to_string(),
            "unknown almanac ruleset id: does-not-exist"
        );
    }

    #[test]
    fn baseline_loader_shim_matches_default_registry_entry() {
        assert!(std::ptr::eq(baseline_data(), default_ruleset().data()));
        assert!(std::ptr::eq(
            baseline_data(),
            get_ruleset_data(DEFAULT_RULESET_ID).expect("ruleset data")
        ));
    }

    #[test]
    fn builds_ruleset_descriptor_doc_with_required_fields() {
        let descriptor = get_ruleset_descriptor_doc(DEFAULT_RULESET_ID).expect("descriptor doc");
        assert_eq!(descriptor.id, "vn_baseline_v1");
        assert_eq!(descriptor.version, "v1");
        assert_eq!(descriptor.region, "vn");
        assert_eq!(descriptor.defaults.tz_offset, 7.0);
        assert_eq!(descriptor.defaults.meridian, None);
        assert_eq!(descriptor.schema_version, "ruleset-descriptor/v1");
        assert!(descriptor
            .source_notes
            .iter()
            .any(|note| note.family == "taboo_rules" && note.source_id == "khcbppt"));
    }

    #[test]
    fn descriptor_doc_loader_accepts_alias_path() {
        let alias = get_ruleset_descriptor_doc("baseline").expect("alias descriptor");
        let canonical =
            get_ruleset_descriptor_doc(DEFAULT_RULESET_ID).expect("canonical descriptor");
        assert_eq!(alias, canonical);
    }

    #[test]
    fn rejects_invalid_ruleset_descriptor_doc_region() {
        let mut descriptor = default_ruleset().descriptor.to_document_descriptor();
        descriptor.region = "cn".to_string();

        let result = std::panic::catch_unwind(|| validate_ruleset_descriptor_doc(&descriptor));
        assert!(result.is_err(), "invalid region must fail validation");
    }

    #[test]
    fn rejects_invalid_ruleset_descriptor_doc_tokens() {
        let mut descriptor = default_ruleset().descriptor.to_document_descriptor();
        descriptor.defaults.tz_offset = 20.0;
        descriptor.source_notes.push(RuleSetSourceNote {
            family: "taboo_rules".to_string(),
            source_id: "".to_string(),
            note: "dup family and empty source".to_string(),
        });

        let result = std::panic::catch_unwind(|| validate_ruleset_descriptor_doc(&descriptor));
        assert!(
            result.is_err(),
            "invalid descriptor tokens must fail validation"
        );
    }

    #[test]
    fn validates_taboo_rule_schema_loads() {
        let data = baseline_data();
        assert_eq!(data.taboo_rule_meta.tam_nuong.method, "table-lookup");
        assert_eq!(data.taboo_rules.tam_nuong.rule_id, "tam_nuong");
        assert_eq!(data.taboo_rules.nguyet_ky.lunar_days, vec![5, 14, 23]);
        assert_eq!(
            data.taboo_rules
                .sat_chu
                .by_lunar_month
                .get(&1)
                .map(String::as_str),
            Some("Tỵ")
        );
        assert_eq!(
            data.taboo_rules
                .tho_tu
                .by_lunar_month
                .get(&12)
                .map(String::as_str),
            Some("Mùi")
        );
    }

    #[test]
    fn rejects_invalid_taboo_day_rule_values() {
        let bad = TabooDayRuleRaw {
            rule_id: "tam_nuong".to_string(),
            name: "Tam Nương".to_string(),
            severity: "critical".to_string(),
            lunar_days: vec![3, 31],
        };

        let result = std::panic::catch_unwind(|| {
            validate_taboo_day_rule(&bad, "taboo_rule_sets.tam_nuong", "tam_nuong")
        });
        assert!(result.is_err(), "invalid taboo day rule must panic");
    }

    #[test]
    fn rejects_invalid_taboo_month_chi_rule_values() {
        let mut by_lunar_month = HashMap::new();
        for month in 1..=11 {
            by_lunar_month.insert(month.to_string(), "Tý".to_string());
        }
        by_lunar_month.insert("12".to_string(), "NotAChi".to_string());

        let bad = TabooMonthChiRuleRaw {
            rule_id: "sat_chu".to_string(),
            name: "Sát Chủ".to_string(),
            severity: "hard".to_string(),
            by_lunar_month,
        };

        let result = std::panic::catch_unwind(|| {
            validate_taboo_month_chi_rule(&bad, "taboo_rule_sets.sat_chu", "sat_chu")
        });
        assert!(result.is_err(), "invalid taboo month-chi rule must panic");
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
