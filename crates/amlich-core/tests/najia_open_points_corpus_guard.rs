//! CI guard for the v1.11 Track-1 frozen Najia open-point corpus
//! (bead `amlich-xlag.2.1`): `crates/amlich-core/data/ty-ngo-luu-chu/
//! najia-open-points.json`.
//!
//! This test intentionally lives in `tests/` and never touches
//! `crates/amlich-core/src/`: per ADR-0003 the reserved `ty-ngo-luu-chu`
//! source id must not appear in production source until the v1.11 engine
//! bead (`amlich-xlag.2.2`) performs its first emission under its own policy
//! contract. The guard here freezes the *data* side only:
//!
//!   1. structural counts (10 day-tables × 6 rows; 120 grid cells; 60 open /
//!      60 closed; nomenclature registry present);
//!   2. an independent re-derivation of the 120-cell grid from the frozen
//!      day-tables via the pinned resolution conventions (五鼠遁 hour pillars,
//!      子-block attributed to the upcoming civil date, running-window rule) —
//!      the grid must match the derived state exactly, with no interpolation;
//!   3. every open row resolves to exactly one grid cell (bijection), each
//!      carrying cross-day spillover truthfulness;
//!   4. reviewer state stays `ExternalReviewPending`, safety class stays
//!      `historical_procedural_citation`, divergence ids are present, and the
//!      facsimile URI stays pending until Gate 1 signs;
//!   5. closed cells serialize explicit unavailable-by-tradition evidence,
//!      never a fallback point;
//!   6. no clinical / technique / efficacy field names appear anywhere in the
//!      serialized corpus (BOUND-02 lexical boundary).

use serde_json::Value;

const CORPUS_JSON: &str = include_str!("../data/ty-ngo-luu-chu/najia-open-points.json");

const STEM_ZH: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const BRANCH_ZH: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

/// 五鼠遁: 子時 seed stem per day stem (甲己→甲, 乙庚→丙, 丙辛→戊, 丁壬→庚, 戊癸→壬).
fn hour_pillar(day_stem: usize, hour_branch: usize) -> String {
    let seed = match day_stem {
        0 | 5 => 0usize,
        1 | 6 => 2,
        2 | 7 => 4,
        3 | 8 => 6,
        4 | 9 => 8,
        _ => unreachable!(),
    };
    format!(
        "{}{}",
        STEM_ZH[(seed + hour_branch) % 10],
        BRANCH_ZH[hour_branch]
    )
}

/// Opening hour branch per day-table, as printed in the 流注圖
/// (甲→戌, 乙→酉, 丙→申, 丁→未, 戊→午, 己→巳, 庚→辰, 辛→卯, 壬→寅, 癸→亥).
fn opening_branch(day_stem: usize) -> usize {
    const B0: [usize; 10] = [10, 9, 8, 7, 6, 5, 4, 3, 2, 11];
    B0[day_stem]
}

/// Running-window membership: table `k` claims cell (d, h) when the branch has
/// the table's opening parity and the cell lies at or after the opening branch
/// on the opening day itself, or before it on the next civil day.
fn in_window(k: usize, d: usize, h: usize) -> bool {
    let b0 = opening_branch(k);
    if h % 2 != b0 % 2 {
        return false;
    }
    (d == k && h >= b0) || (d == (k + 1) % 10 && h < b0)
}

fn stem_index(zh: &str) -> usize {
    STEM_ZH.iter().position(|s| *s == zh).expect("known stem")
}

fn branch_index(zh: &str) -> usize {
    BRANCH_ZH
        .iter()
        .position(|s| *s == zh)
        .expect("known branch")
}

#[test]
fn corpus_has_expected_shape_and_counts() {
    let corpus: Value = serde_json::from_str(CORPUS_JSON).expect("corpus JSON parses");
    let tables = corpus["day_tables"].as_array().expect("day_tables");
    assert_eq!(tables.len(), 10, "exactly ten day-tables");
    for t in tables {
        assert_eq!(t["rows"].as_array().unwrap().len(), 6, "six rows per table");
    }
    let grid = corpus["grid"].as_array().expect("grid");
    assert_eq!(grid.len(), 120, "10 day-stems x 12 hour-branches");
    let open = grid.iter().filter(|c| c["state"] == "open").count();
    let closed = grid.iter().filter(|c| c["state"] == "closed").count();
    assert_eq!(open, 60, "60 open slots (Xu tables: 6 rows x 10 days)");
    assert_eq!(closed, 60, "60 explicitly closed (閉穴) slots");
    assert!(
        corpus["point_nomenclature_registry"]
            .as_array()
            .unwrap()
            .len()
            >= 60,
        "nomenclature registry covers the used points"
    );
    assert_eq!(
        corpus["metadata"]["time_basis"], "local_civil_hour_branch",
        "time basis disclosed"
    );
    assert_eq!(
        corpus["metadata"]["safety_class"], "historical_procedural_citation",
        "safety class pinned"
    );
}

#[test]
fn grid_independently_rederives_from_frozen_tables() {
    let corpus: Value = serde_json::from_str(CORPUS_JSON).expect("corpus JSON parses");

    // (pillar, owning day-table stem index) pairs straight from the frozen rows.
    let mut row_cells: Vec<(String, usize)> = Vec::new();
    for t in corpus["day_tables"].as_array().unwrap() {
        let k = stem_index(t["day_stem_zh"].as_str().unwrap());
        for r in t["rows"].as_array().unwrap() {
            let pillar = r["hour_pillar_zh"].as_str().unwrap().to_string();
            // The resolved cell recorded on the row must satisfy the window rule.
            let cell = &r["resolved_cell"];
            let d = stem_index(cell["day_stem_zh"].as_str().unwrap());
            let h = branch_index(cell["hour_branch_zh"].as_str().unwrap());
            assert!(
                in_window(k, d, h),
                "row {pillar} cell not in window of its table"
            );
            assert_eq!(
                hour_pillar(d, h),
                pillar,
                "row {pillar} resolved cell pillar mismatch"
            );
            row_cells.push((pillar, k));
        }
    }
    assert_eq!(row_cells.len(), 60);

    let mut claimed: Vec<(usize, usize)> = Vec::new();
    for (pillar, k) in &row_cells {
        for d in 0..10 {
            for h in 0..12 {
                if in_window(*k, d, h) && hour_pillar(d, h) == *pillar {
                    claimed.push((d, h));
                }
            }
        }
    }
    assert_eq!(claimed.len(), 60, "each frozen row claims exactly one cell");
    claimed.sort_unstable();
    claimed.dedup();
    assert_eq!(claimed.len(), 60, "row-to-cell mapping is a bijection");

    for cell in corpus["grid"].as_array().unwrap() {
        let d = stem_index(cell["day_stem_zh"].as_str().unwrap());
        let h = branch_index(cell["hour_branch_zh"].as_str().unwrap());
        assert_eq!(
            cell["hour_pillar_zh"].as_str().unwrap(),
            hour_pillar(d, h),
            "grid pillar must equal the 五鼠遁 pillar for its cell"
        );
        let open_cell = claimed.contains(&(d, h));
        match cell["state"].as_str().unwrap() {
            "open" => assert!(
                open_cell,
                "cell ({},{}) marked open but not claimed",
                STEM_ZH[d], BRANCH_ZH[h]
            ),
            "closed" => assert!(
                !open_cell,
                "cell ({},{}) marked closed but a frozen row claims it",
                STEM_ZH[d], BRANCH_ZH[h]
            ),
            other => panic!("unknown state {other}"),
        }
    }
}

#[test]
fn cross_day_spillover_pins_survive_in_the_grid() {
    let corpus: Value = serde_json::from_str(CORPUS_JSON).expect("corpus JSON parses");
    let find = |d: &str, h: &str| {
        corpus["grid"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["day_stem_zh"] == d && c["hour_branch_zh"] == h)
            .unwrap_or_else(|| panic!("cell {d}/{h} missing"))
            .clone()
    };
    // 甲→乙 spillover: 乙-day 子 block opens 前谷 (甲-table row 2), while the
    // 甲-day 子 block is closed (the 癸-table window has no 子 row).
    let yi_zi = find("乙", "子");
    assert_eq!(yi_zi["state"], "open");
    assert_eq!(yi_zi["resolves_to"]["table"], "jia");
    assert_eq!(yi_zi["resolves_to"]["row_index"], 2);
    let jia_zi = find("甲", "子");
    assert_eq!(jia_zi["state"], "closed");
    // 壬→癸 spillover: 癸-day 子 block opens 關沖 (壬-table row 6, 氣納三焦).
    let gui_zi = find("癸", "子");
    assert_eq!(gui_zi["state"], "open");
    assert_eq!(gui_zi["resolves_to"]["table"], "ren");
    assert_eq!(gui_zi["resolves_to"]["row_index"], 6);
    // The famous 癸-day gap: only 子 and 亥 are open on 癸-days.
    let gui_open: Vec<&str> = corpus["grid"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["day_stem_zh"] == "癸" && c["state"] == "open")
        .map(|c| c["hour_branch_zh"].as_str().unwrap())
        .collect();
    assert_eq!(gui_open, vec!["子", "亥"]);
}

#[test]
fn review_provenance_and_safety_fields_are_locked_pending_gates() {
    let corpus: Value = serde_json::from_str(CORPUS_JSON).expect("corpus JSON parses");
    for t in corpus["day_tables"].as_array().unwrap() {
        for r in t["rows"].as_array().unwrap() {
            let reviewer = r["reviewer"].as_str().unwrap();
            assert!(
                reviewer.starts_with(
                    "ExternalReviewPending(reason=\"najia_xu_style_table_row_review_pending\""
                ) && reviewer.contains("assigned_to=\"classical_chinese_reviewer\""),
                "rows stay unsigned until Gate 1"
            );
            assert_eq!(r["safety_class"], "historical_procedural_citation");
            let divs: Vec<&str> = r["known_divergence_ids"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            assert!(
                divs.contains(&"TNLC-DIV-01"),
                "closed-slot divergence present"
            );
            for s in r["sources"].as_array().unwrap() {
                assert_eq!(s["source_id"], "ty-ngo-luu-chu");
                assert_eq!(s["edition_or_facsimile_uri"], "PENDING_CLASSICAL_REVIEW");
                assert!(!s["transcription_uri"].as_str().unwrap().is_empty());
            }
        }
    }
    for p in corpus["point_nomenclature_registry"].as_array().unwrap() {
        assert!(
            p["nomenclature_reviewer"]
                .as_str()
                .unwrap()
                .contains("assigned_to=\"vietnamese_nomenclature_reviewer\""),
            "Gate 2 owns the nomenclature"
        );
    }
}

#[test]
fn closed_slots_serialize_unavailable_state_not_fallback_points() {
    let corpus: Value = serde_json::from_str(CORPUS_JSON).expect("corpus JSON parses");
    for cell in corpus["grid"].as_array().unwrap() {
        if cell["state"] == "closed" {
            let ev = &cell["closed_evidence"];
            assert!(ev["running_tables"].as_array().unwrap().len() == 2);
            assert!(
                ev["doctrine_zh"].as_str().unwrap().contains("失時為之闔"),
                "closed cells cite the classical open/closed doctrine"
            );
            assert!(
                cell.get("points").is_none() && cell.get("resolves_to").is_none(),
                "closed cells never carry a point or fallback row"
            );
        } else {
            assert!(cell["resolves_to"]["table"].is_string());
            assert!(cell["cross_day_spillover"].is_boolean());
        }
    }
}

#[test]
fn corpus_carries_no_clinical_or_technique_field_names() {
    let raw = CORPUS_JSON;
    const FORBIDDEN: [&str; 12] = [
        "needle_depth",
        "indication",
        "contraindication",
        "efficacy",
        "treats",
        "cures",
        "diagnosis",
        "point_to_press",
        "manipulation",
        "best_time",
        "depth_cun",
        "moxa_protocol",
    ];
    for lexeme in FORBIDDEN {
        assert!(
            !raw.contains(lexeme),
            "forbidden clinical/technique lexeme `{lexeme}` found in the frozen corpus"
        );
    }
}
