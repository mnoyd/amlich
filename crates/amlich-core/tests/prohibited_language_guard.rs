//! v1.10 Phase 01-01 (VERIFY-01 lexical facet) — prohibited-language
//! guard for the Traditional Wellness Context surfaces.
//!
//! Scans three scopes:
//!   1. `crates/amlich-core/data/traditional-wellness/branch-channel.json`
//!      — forbids the §3.2 lexemes from LUNAR_HEALTH_RESEARCH.md and
//!      asserts every row carries the canonical `safety_class`.
//!   2. `crates/amlich-core/src/traditional_wellness/**/*.rs` — forbids
//!      the same lexemes plus the clinical field names. Clinical fields
//!      are checked against Rust struct field declarations only (so
//!      generic words like `treatment` in non-clinical comments don't
//!      trip the guard).
//!   3. `crates/amlich-core/src/traditional_wellness/disclaimer.rs` —
//!      asserts the bilingual strings are byte-identical to the strings
//!      in REVIEWER-PACK.md §A.1 and §A.2. The reviewer pack is the
//!      contract surface; the implementation must mirror it.
//!
//! Precedent: `crates/amlich-core/tests/ritual_han_guard.rs` (Hán-character
//! guard for the v1.5 ritual corpus, per `.planning/milestones/v1.5-phases/
//! 11-van-khan-module-and-lookup-apis/11-01-PLAN.md:11, 28-31`).

use std::fs;
use std::path::{Path, PathBuf};

/// §3.2 Vietnamese phrasing that the guard forbids anywhere in scope 1
/// or scope 2.
const FORBIDDEN_VI: &[&str] = &["hoạt động mạnh nhất", "thải độc", "đạt đỉnh"];

/// §3.2 English phrasing that the guard forbids anywhere in scope 1 or
/// scope 2.
const FORBIDDEN_EN: &[&str] = &[
    "best time to treat",
    "active organ",
    // `peak` checked as a standalone word (whole-word boundary) below
    // to avoid false positives on words like "speakeasy".
    "peak",
    "detox",
    "prevents",
    "treats",
    "cures",
    "diagnoses",
    "reduces risk",
    "balances hormones",
];

/// Clinical field names that must not appear as Rust struct field
/// declarations in the `traditional_wellness` module (per
/// `LUNAR_HEALTH_RESEARCH.md:146` and the public-schema contract in
/// `v1.10-REQUIREMENTS.md:40-43`).
const FORBIDDEN_CLINICAL_FIELDS: &[&str] = &[
    "indication",
    "contraindication",
    "diagnosis",
    "treatment",
    "dose",
    "needle_depth",
    "point_to_press",
    "efficacy",
];

fn collect_files(dir: &Path, suffix: &str, out: &mut Vec<PathBuf>) {
    if !dir.exists() {
        return;
    }
    for entry in fs::read_dir(dir).expect("read_dir failed") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_files(&path, suffix, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some(suffix) {
            out.push(path);
        }
    }
}

fn contains_any<'a>(line: &str, needles: &[&'a str]) -> Option<&'a str> {
    needles.iter().find(|&n| line.contains(n)).copied()
}

/// Whole-word match for `peak` — avoids false positives on substrings
/// like `speakeasy`. We tokenise on whitespace and basic punctuation.
fn contains_whole_word(line: &str, needle: &str) -> bool {
    line.split(|c: char| !c.is_alphanumeric() && c != '\'')
        .any(|tok| tok == needle)
}

fn report(violations: &mut Vec<String>, path: &Path, lineno: usize, line: &str, needle: &str) {
    violations.push(format!(
        "{}:{}  prohibited lexeme/field {:?} found:\n    {}",
        path.display(),
        lineno,
        needle,
        line.trim()
    ));
}

#[test]
fn corpus_json_contains_no_prohibited_lexemes() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("data/traditional-wellness/branch-channel.json");
    let contents = fs::read_to_string(&path).expect("read corpus JSON");
    let mut violations: Vec<String> = Vec::new();

    for (lineno, line) in contents.lines().enumerate() {
        if let Some(n) = contains_any(line, FORBIDDEN_VI) {
            report(&mut violations, &path, lineno + 1, line, n);
        }
        if let Some(n) = contains_any(line, FORBIDDEN_EN) {
            // Skip whole-word-only entries handled below
            if n == "peak" {
                if contains_whole_word(line, "peak") {
                    report(&mut violations, &path, lineno + 1, line, n);
                }
            } else {
                report(&mut violations, &path, lineno + 1, line, n);
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Prohibited lexemes found in branch-channel.json:\n{}",
        violations.join("\n")
    );
}

#[test]
fn corpus_json_has_no_clinical_field_keys() {
    // BOUND-02 / LUNAR_HEALTH_RESEARCH.md:146 — the public corpus must
    // not expose acupuncture-point, treatment, diagnosis, efficacy,
    // food/herb, dose, or disease fields. The phrase lexeme scan above
    // catches leaked substrings; this test catches any *field name*
    // that would invite a clinical reading even when its value is
    // empty. The walk is exhaustive over nested objects and arrays.
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("data/traditional-wellness/branch-channel.json");
    let contents = fs::read_to_string(&path).expect("read corpus JSON");
    let value: serde_json::Value = serde_json::from_str(&contents).expect("parse corpus JSON");
    let mut violations: Vec<String> = Vec::new();

    fn walk(
        value: &serde_json::Value,
        path: &mut String,
        forbidden: &[&str],
        violations: &mut Vec<String>,
    ) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let lower = k.to_lowercase();
                    if forbidden.iter().any(|f| lower == *f) {
                        violations.push(format!("forbidden JSON key {k:?} at {path}{k}"));
                    }
                    let prev_len = path.len();
                    if !path.is_empty() {
                        path.push('.');
                    }
                    path.push_str(k);
                    walk(v, path, forbidden, violations);
                    path.truncate(prev_len);
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let prev_len = path.len();
                    if !path.is_empty() {
                        path.push('.');
                    }
                    let seg = format!("[{i}]");
                    path.push_str(&seg);
                    walk(v, path, forbidden, violations);
                    path.truncate(prev_len);
                }
            }
            _ => {}
        }
    }
    let mut path = String::new();
    walk(
        &value,
        &mut path,
        FORBIDDEN_CLINICAL_FIELDS,
        &mut violations,
    );

    assert!(
        violations.is_empty(),
        "Prohibited clinical JSON keys found in branch-channel.json:\n{}",
        violations.join("\n")
    );
}

#[test]
fn corpus_json_rows_carry_canonical_safety_class() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("data/traditional-wellness/branch-channel.json");
    let contents = fs::read_to_string(&path).expect("read corpus JSON");
    let value: serde_json::Value = serde_json::from_str(&contents).expect("parse corpus JSON");
    let rows = value
        .get("rows")
        .and_then(|r| r.as_array())
        .expect("rows array");
    assert_eq!(rows.len(), 12, "corpus must contain exactly 12 rows");
    for (i, row) in rows.iter().enumerate() {
        let safety = row
            .get("safety_class")
            .and_then(|s| s.as_str())
            .unwrap_or_default();
        assert_eq!(
            safety, "historical_cultural_non_clinical",
            "row {i} must carry the canonical safety_class"
        );
    }
}

#[test]
fn traditional_wellness_rs_contains_no_prohibited_lexemes_or_clinical_fields() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/traditional_wellness");
    let mut files = Vec::new();
    collect_files(&dir, "rs", &mut files);
    assert!(
        !files.is_empty(),
        "expected at least one .rs file under src/traditional_wellness"
    );

    let mut violations: Vec<String> = Vec::new();
    for path in &files {
        let contents = fs::read_to_string(path).expect("read file");
        for (lineno, line) in contents.lines().enumerate() {
            // Skip line- and doc-comments for phrase lexemes
            let trimmed = line.trim_start();
            let in_comment = trimmed.starts_with("//");

            if !in_comment {
                if let Some(n) = contains_any(line, FORBIDDEN_VI) {
                    report(&mut violations, path, lineno + 1, line, n);
                }
                if let Some(n) = contains_any(line, FORBIDDEN_EN) {
                    if n == "peak" {
                        if contains_whole_word(line, "peak") {
                            report(&mut violations, path, lineno + 1, line, n);
                        }
                    } else {
                        report(&mut violations, path, lineno + 1, line, n);
                    }
                }
            }

            // Clinical field names — checked against struct-field
            // declarations only (`pub <name>:`), allowing the field name
            // to appear in tests / comments / doc-comments. Comment lines
            // are still checked because mentioning a clinical field as
            // a "rejected field" is itself acceptable — but a
            // `pub <forbidden>: ...` declaration is not.
            let lower = line.to_lowercase();
            for field in FORBIDDEN_CLINICAL_FIELDS {
                if let Some(pos) = lower.find(&format!("pub {field}:")) {
                    // Skip exact matches in self-tests that explicitly
                    // assert the field is forbidden (the guard's own
                    // documentation).
                    let after = &line[pos..];
                    let _ = after; // presence itself is the violation
                    report(&mut violations, path, lineno + 1, line, field);
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Prohibited lexemes/clinical fields found in src/traditional_wellness/:\n{}",
        violations.join("\n")
    );
}

#[test]
fn bilingual_disclaimer_is_byte_identical_to_reviewer_pack() {
    use amlich_core::traditional_wellness::{
        DISCLAIMER_CULTURAL_INFORMATION_EN, DISCLAIMER_CULTURAL_INFORMATION_VN,
    };

    // REVIEWER-PACK.md is the contract surface — the §E guarantees
    // promise the reviewer that "All localized outputs carry the
    // disclaimer or a stable disclaimer ID that clients are
    // contractually required to render." To keep that promise
    // auditable, this test reads the pack file at test time and
    // asserts the implementation constants appear verbatim inside §A.1
    // and §A.2 blockquotes. Drift in either direction (implementation
    // drifts from pack, or pack drifts from implementation) fails CI.
    let pack_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(".planning")
        .join("milestones")
        .join("v1.10-phases")
        .join("01-hour-branch-channel-association")
        .join("REVIEWER-PACK.md");
    let pack = fs::read_to_string(&pack_path).unwrap_or_else(|e| {
        panic!(
            "could not read REVIEWER-PACK at {}: {e}",
            pack_path.display()
        )
    });

    // Extract the §A.1 and §A.2 blockquote bodies. The pack uses the
    // standard markdown `> ` prefix on a single line per section.
    let extract_blockquote = |pack: &str, marker: &str| -> Option<String> {
        let after = pack.split_once(marker)?.1;
        let line = after
            .lines()
            .map(str::trim)
            .find(|l| l.starts_with("> ") || l.starts_with(">"))?;
        let body = line.trim_start_matches("> ").trim();
        Some(body.to_string())
    };

    let pack_vn = extract_blockquote(&pack, "**§A.1 Vietnamese:**")
        .expect("REVIEWER-PACK must contain §A.1 Vietnamese blockquote");
    let pack_en = extract_blockquote(&pack, "**§A.2 English:**")
        .expect("REVIEWER-PACK must contain §A.2 English blockquote");

    assert_eq!(
        DISCLAIMER_CULTURAL_INFORMATION_VN, pack_vn,
        "Vietnamese disclaimer must be byte-identical to REVIEWER-PACK.md §A.1"
    );
    assert_eq!(
        DISCLAIMER_CULTURAL_INFORMATION_EN, pack_en,
        "English disclaimer must be byte-identical to REVIEWER-PACK.md §A.2"
    );
}

#[test]
fn corpus_row_wording_appears_in_reviewer_pack() {
    // ASSOC-01 / pack integrity — every row's wording_vi / wording_en
    // appears verbatim in REVIEWER-PACK.md §A.4 so the reviewer
    // contract surface and the implementation surface cannot drift
    // apart silently.
    let pack_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(".planning")
        .join("milestones")
        .join("v1.10-phases")
        .join("01-hour-branch-channel-association")
        .join("REVIEWER-PACK.md");
    let pack = fs::read_to_string(&pack_path).expect("read REVIEWER-PACK");
    let corpus_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("traditional-wellness")
        .join("branch-channel.json");
    let corpus_raw = fs::read_to_string(&corpus_path).expect("read corpus JSON");
    let value: serde_json::Value = serde_json::from_str(&corpus_raw).expect("parse corpus JSON");
    let rows = value
        .get("rows")
        .and_then(|r| r.as_array())
        .expect("rows array");
    let mut missing: Vec<String> = Vec::new();
    for row in rows {
        let branch_vi = row.get("branch_vi").and_then(|v| v.as_str()).unwrap_or("");
        let wording_vi = row.get("wording_vi").and_then(|v| v.as_str()).unwrap_or("");
        let wording_en = row.get("wording_en").and_then(|v| v.as_str()).unwrap_or("");
        if !pack.contains(wording_vi) {
            missing.push(format!(
                "row {branch_vi} wording_vi not found in REVIEWER-PACK §A.4: {wording_vi:?}"
            ));
        }
        if !pack.contains(wording_en) {
            missing.push(format!(
                "row {branch_vi} wording_en not found in REVIEWER-PACK §A.4: {wording_en:?}"
            ));
        }
    }
    assert!(
        missing.is_empty(),
        "Drift between corpus wording and REVIEWER-PACK §A.4:\n{}",
        missing.join("\n")
    );
}
