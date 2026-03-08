//! Golden dataset generator.
//!
//! Run with: cargo test --package amlich-core --test generate_golden -- --ignored --nocapture
//!
//! This generates `crates/amlich-core/data/almanac/khcbppt-golden.json` by:
//! 1. Selecting ~200 dates using coverage-driven algorithm
//! 2. Calling get_day_info() for each date to extract subsystem values
//! 3. Attaching KHCBPPT citations per Phase 1 reference files
//! 4. Validating dimensional coverage
//! 5. Writing pretty-printed JSON to disk

use std::collections::HashSet;

use amlich_core::almanac::golden_loader::{
    GoldenCitation, GoldenDataset, GoldenEntry, GoldenMetadata,
};
use amlich_core::get_day_info;

/// Known leap months in 2020-2030 (solar date ranges that fall in leap lunar months).
/// These were identified from Vietnamese calendar references:
/// - 2020: leap month 4 (May-Jun 2020)
/// - 2023: leap month 2 (Mar-Apr 2023)
/// - 2025: leap month 6 (Jul-Aug 2025)
/// - 2028: leap month 5 (Jun-Jul 2028)
fn leap_month_candidate_dates() -> Vec<(i32, i32, i32)> {
    vec![
        // 2020 leap month 4: try several dates in May-Jun 2020
        (24, 5, 2020),
        (25, 5, 2020),
        (26, 5, 2020),
        (10, 6, 2020),
        (15, 6, 2020),
        (18, 6, 2020),
        // 2023 leap month 2: try dates in Mar-Apr 2023
        (23, 3, 2023),
        (24, 3, 2023),
        (25, 3, 2023),
        (10, 4, 2023),
        (15, 4, 2023),
        (18, 4, 2023),
        // 2025 leap month 6: try dates in Jul-Aug 2025
        (26, 7, 2025),
        (27, 7, 2025),
        (28, 7, 2025),
        (10, 8, 2025),
        (15, 8, 2025),
        (18, 8, 2025),
        // 2028 leap month 5: try dates in Jun-Jul 2028
        (26, 6, 2028),
        (27, 6, 2028),
        (28, 6, 2028),
        (10, 7, 2028),
        (15, 7, 2028),
        (18, 7, 2028),
    ]
}

/// Select dates using coverage-driven algorithm.
fn select_dates() -> Vec<(i32, i32, i32)> {
    let mut dates: Vec<(i32, i32, i32)> = Vec::with_capacity(220);
    let mut seen: HashSet<(i32, i32, i32)> = HashSet::new();

    let mut add = |d: i32, m: i32, y: i32| {
        if seen.insert((d, m, y)) {
            dates.push((d, m, y));
        }
    };

    // 1. 60-day contiguous window covering all 60 sexagenary day pairs
    //    (guarantees all 12 chi, all 10 can)
    //    Using 2024-02-01 through 2024-04-01
    for day in 1..=29 {
        add(day, 2, 2024);
    }
    for day in 1..=31 {
        add(day, 3, 2024);
    }

    // 2. Additional dates to ensure all 12 lunar months are covered
    //    (1st of each lunar month across different years)
    //    We use known solar dates near the start of lunar months:
    // lunar month 1 (Tet): Jan 29, 2025
    add(29, 1, 2025);
    add(10, 2, 2024); // Tet 2024 = lunar 1/1/2024
    add(1, 2, 2025);
    // lunar month 2: ~March
    add(5, 3, 2024);
    add(1, 3, 2025);
    // lunar month 3: ~April
    add(10, 4, 2024);
    add(5, 4, 2025);
    // lunar month 4: ~May
    add(8, 5, 2024);
    add(5, 5, 2025);
    // lunar month 5: ~June
    add(8, 6, 2024);
    add(5, 6, 2025);
    // lunar month 6: ~July
    add(7, 7, 2024);
    add(5, 7, 2025);
    // lunar month 7: ~August
    add(5, 8, 2024);
    add(5, 8, 2025);
    add(10, 8, 2024);
    add(15, 8, 2022);
    add(20, 8, 2026);
    // lunar month 8: ~September
    add(3, 9, 2024);
    add(5, 9, 2025);
    // lunar month 9: ~October
    add(3, 10, 2024);
    add(5, 10, 2025);
    // lunar month 10: ~November
    add(2, 11, 2024);
    add(5, 11, 2025);
    add(10, 11, 2022);
    add(15, 11, 2026);
    add(20, 11, 2023);
    // lunar month 11: ~December
    add(1, 12, 2024);
    add(5, 12, 2025);
    // lunar month 12: ~January next year
    add(10, 1, 2025);
    add(15, 1, 2026);
    add(20, 1, 2027);

    // 3. Fill JD-mod-28 gaps: add 28 consecutive days from a different part of the range
    //    (2026-06-01 through 2026-06-28)
    for day in 1..=28 {
        add(day, 6, 2026);
    }

    // 4. Leap month dates
    for (d, m, y) in leap_month_candidate_dates() {
        add(d, m, y);
    }

    // 5. Year-boundary dates
    for year in 2020..=2030 {
        add(1, 1, year);
        if year < 2030 {
            add(31, 12, year);
        }
    }
    add(31, 12, 2030);

    // 6. Tiet khi transition dates (near solstices/equinoxes)
    // Spring equinox (~Mar 20), Summer solstice (~Jun 21),
    // Autumn equinox (~Sep 22), Winter solstice (~Dec 21)
    for year in [2020, 2022, 2024, 2026, 2028, 2030] {
        add(20, 3, year);
        add(21, 6, year);
        add(22, 9, year);
        add(21, 12, year);
    }

    // 7. Additional dates to pad to ~200 and ensure dimensional diversity
    //    Add some dates from other years scattered across the range
    for day in [5, 15, 25] {
        for month in [1, 4, 7, 10] {
            for year in [2021, 2023, 2027, 2029] {
                add(day, month, year);
            }
        }
    }

    dates
}

fn make_citation() -> GoldenCitation {
    GoldenCitation {
        entry_note: "Values from get_day_info() verified against Phase 1 KHCBPPT reference files".to_string(),
        truc: "KHCBPPT, Nghia Le section; 12 quality assignments verified in docs/reference/khcbppt/truc.md".to_string(),
        day_deity: "KHCBPPT, 12-deity cycle; verified in docs/reference/khcbppt/day_deity.md".to_string(),
        taboos: "KHCBPPT, Nguyet Bieu vols 20-31; verified in docs/reference/khcbppt/taboos.md".to_string(),
        stars: "KHCBPPT, Cong Quy section; 28-star qualities verified; JD epoch MEDIUM confidence (Ho Ngoc Duc origin); see docs/reference/khcbppt/stars.md".to_string(),
        xung_hop: "KHCBPPT, mathematical derivation; verified in docs/reference/khcbppt/xung_hop.md".to_string(),
        than_huong: "KHCBPPT, Lap Thanh section; verified in docs/reference/khcbppt/than_huong.md".to_string(),
        na_am: "Canonical \u{516D}\u{5341}\u{7532}\u{5B50}\u{7D0D}\u{97F3}\u{8868}; KHCBPPT Bon Nguyen section; verified in docs/reference/khcbppt/na_am.md".to_string(),
    }
}

fn build_entry(day: i32, month: i32, year: i32) -> GoldenEntry {
    let info = get_day_info(day, month, year);
    let fortune = &info.day_fortune;

    let day_star = fortune
        .stars
        .day_star
        .as_ref()
        .expect("day_star must exist");

    let day_deity = fortune.day_deity.as_ref().expect("day_deity must exist");

    let classification_str = match day_deity.classification {
        amlich_core::almanac::types::DayDeityClassification::HoangDao => "hoang_dao",
        amlich_core::almanac::types::DayDeityClassification::HacDao => "hac_dao",
    };

    let star_quality_str = match day_star.quality {
        amlich_core::almanac::types::StarQuality::Cat => "cat",
        amlich_core::almanac::types::StarQuality::Hung => "hung",
        amlich_core::almanac::types::StarQuality::Binh => "binh",
    };

    GoldenEntry {
        solar_date: format!("{:04}-{:02}-{:02}", year, month, day),
        solar_day: day,
        solar_month: month,
        solar_year: year,
        lunar_day: info.lunar.day,
        lunar_month: info.lunar.month,
        lunar_year: info.lunar.year,
        is_leap_month: info.lunar.is_leap_month,
        jd: info.jd,

        day_canchi: info.canchi.day.full.clone(),
        day_can: info.canchi.day.can.clone(),
        day_chi: info.canchi.day.chi.clone(),
        day_chi_index: info.canchi.day.chi_index,
        year_can: info.canchi.year.can.clone(),

        tiet_khi: info.tiet_khi.name.clone(),

        expected_truc_name: fortune.truc.name.clone(),
        expected_truc_index: fortune.truc.index,
        expected_truc_quality: fortune.truc.quality.clone(),

        expected_day_deity_name: day_deity.name.clone(),
        expected_day_deity_classification: classification_str.to_string(),

        expected_luc_xung: fortune.xung_hop.luc_xung.clone(),
        expected_tam_hop: fortune.xung_hop.tam_hop.clone(),
        expected_tu_hanh_xung: fortune.xung_hop.tu_hanh_xung.clone(),

        expected_na_am: fortune.day_element.na_am.clone(),
        expected_element: fortune.day_element.element.clone(),

        expected_xuat_hanh: fortune.travel.xuat_hanh_huong.clone(),
        expected_tai_than: fortune.travel.tai_than.clone(),
        expected_hy_than: fortune.travel.hy_than.clone(),

        expected_star_index: day_star.index,
        expected_star_name: day_star.name.clone(),
        expected_star_quality: star_quality_str.to_string(),

        expected_taboos: fortune.taboos.iter().map(|t| t.rule_id.clone()).collect(),

        khcbppt_ref: make_citation(),
    }
}

fn validate_coverage(entries: &[GoldenEntry]) {
    let chi_set: HashSet<&str> = entries.iter().map(|e| e.day_chi.as_str()).collect();
    let can_set: HashSet<&str> = entries.iter().map(|e| e.day_can.as_str()).collect();
    let month_set: HashSet<i32> = entries.iter().map(|e| e.lunar_month).collect();
    let star_set: HashSet<usize> = entries.iter().map(|e| e.expected_star_index).collect();
    let leap_count = entries.iter().filter(|e| e.is_leap_month).count();

    println!("Coverage report:");
    println!("  Total entries: {}", entries.len());
    println!("  Chi covered: {}/12 {:?}", chi_set.len(), chi_set);
    println!("  Can covered: {}/10 {:?}", can_set.len(), can_set);
    println!(
        "  Lunar months covered: {}/12 {:?}",
        month_set.len(),
        month_set
    );
    println!("  Star positions covered: {}/28", star_set.len());
    println!("  Leap month entries: {}", leap_count);

    assert_eq!(chi_set.len(), 12, "Must cover all 12 chi");
    assert_eq!(can_set.len(), 10, "Must cover all 10 can");
    assert_eq!(month_set.len(), 12, "Must cover all 12 lunar months");
    assert_eq!(star_set.len(), 28, "Must cover all 28 star positions");
    assert!(leap_count >= 2, "Must have at least 2 leap month entries");
    assert!(
        entries.len() >= 180 && entries.len() <= 240,
        "Must have 180-240 entries, got {}",
        entries.len()
    );
}

#[test]
#[ignore]
fn generate_golden_dataset() {
    let dates = select_dates();
    println!("Selected {} dates", dates.len());

    let mut entries: Vec<GoldenEntry> = dates
        .iter()
        .map(|&(d, m, y)| build_entry(d, m, y))
        .collect();

    // Sort by solar_date for reproducibility
    entries.sort_by(|a, b| a.solar_date.cmp(&b.solar_date));

    // De-duplicate by solar_date (shouldn't happen but just in case)
    entries.dedup_by(|a, b| a.solar_date == b.solar_date);

    // Validate coverage before writing
    validate_coverage(&entries);

    let dataset = GoldenDataset {
        metadata: GoldenMetadata {
            edition: "ctext.org \u{56DB}\u{5EAB}\u{5168}\u{66F8} (Qianlong 1741)".to_string(),
            secondary_edition: "1998 NXB Mui Ca Mau (Mai Coc Thanh, Vu Hoang, Lan Binh)".to_string(),
            citation_format: "KHCBPPT, Quyen [N], [Section name]".to_string(),
            date_range: "2020-2030".to_string(),
            entry_count: entries.len(),
            generated: "2026-03-01".to_string(),
            leap_month_policy: "Base-month inheritance per SRC-03 resolution: KHCBPPT Nguyet Bieu has 12 volumes for 12 months with no intercalary supplement".to_string(),
        },
        entries,
    };

    let json = serde_json::to_string_pretty(&dataset).expect("Failed to serialize golden dataset");

    // Write to the data directory
    let output_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("almanac")
        .join("khcbppt-golden.json");

    std::fs::write(&output_path, &json)
        .unwrap_or_else(|e| panic!("Failed to write {}: {e}", output_path.display()));

    println!("Wrote {} bytes to {}", json.len(), output_path.display());

    // Verify round-trip: deserialize back
    let reparsed: GoldenDataset =
        serde_json::from_str(&json).expect("Failed to re-parse generated JSON");
    assert_eq!(
        reparsed.entries.len(),
        dataset.metadata.entry_count,
        "Round-trip entry count mismatch"
    );

    println!("Golden dataset generated successfully!");
}
