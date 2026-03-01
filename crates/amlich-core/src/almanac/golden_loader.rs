use serde::{Deserialize, Serialize};

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
