use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

const GOLDEN_JSON: &str = include_str!("../../data/almanac/khcbppt-golden.json");
static GOLDEN_DATA: OnceLock<GoldenDataset> = OnceLock::new();

/// Load and validate the golden dataset, returning a `&'static` reference.
///
/// The dataset is parsed from the embedded JSON on first call and cached
/// via `OnceLock` for subsequent calls.  Validation runs once at parse time
/// and panics on any invariant violation (this is a test oracle, not
/// user-facing data).
pub fn load_golden_dataset() -> &'static GoldenDataset {
    GOLDEN_DATA.get_or_init(|| {
        let dataset: GoldenDataset =
            serde_json::from_str(GOLDEN_JSON).expect("Failed to parse khcbppt-golden.json");
        validate_golden_dataset(&dataset);
        dataset
    })
}

/// Top-level container for the golden reference dataset.
///
/// The golden dataset is a machine-readable collection of KHCBPPT-verified
/// almanac values used as a test oracle by Phase 3 validators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenDataset {
    pub metadata: GoldenMetadata,
    pub entries: Vec<GoldenEntry>,
}

/// Dataset provenance and generation metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenMetadata {
    /// Primary KHCBPPT edition (e.g. "ctext.org 四庫全書 (Qianlong 1741)")
    pub edition: String,
    /// Vietnamese translation edition
    pub secondary_edition: String,
    /// Citation format template
    pub citation_format: String,
    /// Date range covered (e.g. "2020-2030")
    pub date_range: String,
    /// Number of entries in the dataset
    pub entry_count: usize,
    /// ISO date when the dataset was generated
    pub generated: String,
    /// Documents the SRC-03 decision on leap month handling
    pub leap_month_policy: String,
}

/// One date with all subsystem expected values and KHCBPPT citations.
///
/// Each entry represents a single solar date with its corresponding lunar date,
/// Can Chi identification, and expected outputs from every almanac subsystem.
/// The `khcbppt_ref` field provides per-subsystem citation trails back to
/// Phase 1 reference files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenEntry {
    // -- Date identification --
    /// Solar date string (e.g. "2024-02-10")
    pub solar_date: String,
    pub solar_day: i32,
    pub solar_month: i32,
    pub solar_year: i32,
    pub lunar_day: i32,
    pub lunar_month: i32,
    pub lunar_year: i32,
    pub is_leap_month: bool,
    /// Julian Day Number
    pub jd: i32,

    // -- Can Chi --
    /// Full day Can Chi (e.g. "Giap Thin")
    pub day_canchi: String,
    /// Day heavenly stem (e.g. "Giap")
    pub day_can: String,
    /// Day earthly branch (e.g. "Thin")
    pub day_chi: String,
    /// Day earthly branch index (0-11)
    pub day_chi_index: usize,
    /// Year heavenly stem
    pub year_can: String,

    // -- Tiet khi --
    /// Current solar term name
    pub tiet_khi: String,

    // -- Truc (12 duty-stars) --
    pub expected_truc_name: String,
    pub expected_truc_index: usize,
    pub expected_truc_quality: String,

    // -- Day deity (12-deity cycle) --
    pub expected_day_deity_name: String,
    pub expected_day_deity_classification: String,

    // -- Xung hop (clash/harmony relationships) --
    pub expected_luc_xung: String,
    pub expected_tam_hop: Vec<String>,
    pub expected_tu_hanh_xung: Vec<String>,

    // -- Na am (sexagenary sound) --
    pub expected_na_am: String,
    pub expected_element: String,

    // -- Than huong (travel directions) --
    pub expected_xuat_hanh: String,
    pub expected_tai_than: String,
    pub expected_hy_than: String,

    // -- Stars (28-star JD cycle) --
    pub expected_star_index: usize,
    pub expected_star_name: String,
    pub expected_star_quality: String,

    // -- Taboos --
    /// Rule IDs that should fire for this date
    pub expected_taboos: Vec<String>,

    // -- Citation --
    pub khcbppt_ref: GoldenCitation,
}

/// Per-subsystem KHCBPPT citations for a golden entry.
///
/// Each field traces the expected value back to a specific section
/// of KHCBPPT and the corresponding Phase 1 reference file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoldenCitation {
    /// General entry-level citation note
    pub entry_note: String,
    /// Citation for truc (duty-star) value
    pub truc: String,
    /// Citation for day deity value
    pub day_deity: String,
    /// Citation for taboo rules
    pub taboos: String,
    /// Citation for 28-star assignment
    pub stars: String,
    /// Citation for xung hop relationships
    pub xung_hop: String,
    /// Citation for than huong (travel direction) values
    pub than_huong: String,
    /// Citation for na am (sexagenary sound) value
    pub na_am: String,
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

fn validate_golden_dataset(dataset: &GoldenDataset) {
    assert!(
        !dataset.entries.is_empty(),
        "golden dataset must not be empty"
    );
    assert!(
        dataset.entries.len() >= 150,
        "golden dataset must have at least 150 entries, got {}",
        dataset.entries.len()
    );
    assert_eq!(
        dataset.metadata.entry_count,
        dataset.entries.len(),
        "metadata.entry_count ({}) must match entries.len() ({})",
        dataset.metadata.entry_count,
        dataset.entries.len()
    );

    validate_coverage(dataset);

    for (i, entry) in dataset.entries.iter().enumerate() {
        assert!(
            (2020..=2030).contains(&entry.solar_year),
            "entry {i} ({}) has solar_year {} outside 2020-2030",
            entry.solar_date,
            entry.solar_year
        );
        assert!(
            !entry.khcbppt_ref.entry_note.trim().is_empty(),
            "entry {i} ({}) has empty khcbppt_ref.entry_note",
            entry.solar_date
        );
    }
}

fn validate_coverage(dataset: &GoldenDataset) {
    use std::collections::HashSet;

    let chi_set: HashSet<&str> = dataset.entries.iter().map(|e| e.day_chi.as_str()).collect();
    assert_eq!(
        chi_set.len(),
        12,
        "golden dataset must cover all 12 chi, got {}: {:?}",
        chi_set.len(),
        chi_set
    );

    let can_set: HashSet<&str> = dataset.entries.iter().map(|e| e.day_can.as_str()).collect();
    assert_eq!(
        can_set.len(),
        10,
        "golden dataset must cover all 10 can, got {}: {:?}",
        can_set.len(),
        can_set
    );

    let month_set: HashSet<i32> = dataset.entries.iter().map(|e| e.lunar_month).collect();
    assert_eq!(
        month_set.len(),
        12,
        "golden dataset must cover all 12 lunar months, got {}: {:?}",
        month_set.len(),
        month_set
    );

    let star_set: HashSet<usize> = dataset
        .entries
        .iter()
        .map(|e| e.expected_star_index)
        .collect();
    assert_eq!(
        star_set.len(),
        28,
        "golden dataset must cover all 28 star positions, got {}: {:?}",
        star_set.len(),
        star_set
    );

    let leap_count = dataset.entries.iter().filter(|e| e.is_leap_month).count();
    assert!(
        leap_count >= 2,
        "golden dataset must have at least 2 leap month entries, got {}",
        leap_count
    );
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn golden_dataset_loads_and_validates() {
        let dataset = load_golden_dataset();
        assert!(
            dataset.entries.len() >= 150,
            "expected >= 150 entries, got {}",
            dataset.entries.len()
        );
    }

    #[test]
    fn golden_dataset_covers_all_chi() {
        let dataset = load_golden_dataset();
        let chi_set: HashSet<&str> =
            dataset.entries.iter().map(|e| e.day_chi.as_str()).collect();
        assert_eq!(chi_set.len(), 12, "must cover all 12 chi: {:?}", chi_set);
    }

    #[test]
    fn golden_dataset_covers_all_can() {
        let dataset = load_golden_dataset();
        let can_set: HashSet<&str> =
            dataset.entries.iter().map(|e| e.day_can.as_str()).collect();
        assert_eq!(can_set.len(), 10, "must cover all 10 can: {:?}", can_set);
    }

    #[test]
    fn golden_dataset_covers_all_lunar_months() {
        let dataset = load_golden_dataset();
        let month_set: HashSet<i32> = dataset.entries.iter().map(|e| e.lunar_month).collect();
        assert_eq!(
            month_set.len(),
            12,
            "must cover all 12 lunar months: {:?}",
            month_set
        );
    }

    #[test]
    fn golden_dataset_covers_all_star_positions() {
        let dataset = load_golden_dataset();
        let star_set: HashSet<usize> = dataset
            .entries
            .iter()
            .map(|e| e.expected_star_index)
            .collect();
        assert_eq!(
            star_set.len(),
            28,
            "must cover all 28 star positions: {:?}",
            star_set
        );
    }

    #[test]
    fn golden_dataset_has_leap_month_entries() {
        let dataset = load_golden_dataset();
        let leap_count = dataset.entries.iter().filter(|e| e.is_leap_month).count();
        assert!(
            leap_count >= 2,
            "need at least 2 leap month entries, got {}",
            leap_count
        );
    }

    #[test]
    fn golden_dataset_all_citations_populated() {
        let dataset = load_golden_dataset();
        for (i, entry) in dataset.entries.iter().enumerate() {
            let fields = [
                ("entry_note", &entry.khcbppt_ref.entry_note),
                ("truc", &entry.khcbppt_ref.truc),
                ("day_deity", &entry.khcbppt_ref.day_deity),
                ("taboos", &entry.khcbppt_ref.taboos),
                ("stars", &entry.khcbppt_ref.stars),
                ("xung_hop", &entry.khcbppt_ref.xung_hop),
                ("than_huong", &entry.khcbppt_ref.than_huong),
            ];
            for (name, value) in &fields {
                assert!(
                    !value.trim().is_empty(),
                    "entry {i} ({}) has empty khcbppt_ref.{name}",
                    entry.solar_date
                );
            }
        }
    }

    #[test]
    fn golden_dataset_metadata_consistent() {
        let dataset = load_golden_dataset();
        assert_eq!(
            dataset.metadata.entry_count,
            dataset.entries.len(),
            "metadata.entry_count ({}) must match entries.len() ({})",
            dataset.metadata.entry_count,
            dataset.entries.len()
        );
    }
}
