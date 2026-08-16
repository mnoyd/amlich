//! CI guard: ensures bare source_id string literals do not appear in
//! crates/amlich-core/src/ outside of `sources.rs`. All production assignments
//! MUST use the SOURCE_* constants from `crate::sources`.
//!
//! Allow-listed:
//!   - `sources.rs` itself (the canonical definitions)
//!   - Lines inside `#[cfg(test)]` modules (test assertions verify the constant resolves correctly)
//!   - Lines starting with `//` or `///` (doc-comments / inline comments)

use std::fs;
use std::path::{Path, PathBuf};

const FORBIDDEN_LITERALS: &[&str] = &[
    "\"khcbppt\"",
    "\"vn-folk\"",
    "\"ngoc-hap-ky\"",
    "\"cuu-dieu\"",
    "\"tam-menh-thong-hoi\"",
    "\"vn-folk-ritual\"",
    "\"huyen-khong\"",
    "\"kinh-dich\"",             // NEW v1.7 (Phase 20-01, FND-09)
    "\"mai-hoa-dich-so\"",       // NEW v1.7 (Phase 20-01, FND-09)
    "\"shi-er-jing-na-di-zhi\"", // NEW v1.10 (Phase 01-01, ASSOC-01 / SOURCE-01)
    "\"huangdi-neijing-suwen\"", // NEW v1.10 (Phase 02-01, SEASON-01 / SOURCE-01)
    "\"ty-ngo-luu-chu\"", // NEW v1.10 (Phase 01-01, ADR-0003 — reserved, must never be emitted)
];

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read_dir failed") {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

#[test]
fn no_bare_source_id_literals_in_production_src() {
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);

    let mut violations: Vec<String> = Vec::new();
    for path in &files {
        // Skip sources.rs itself — it IS the definitions
        if path.file_name().and_then(|s| s.to_str()) == Some("sources.rs") {
            continue;
        }
        let contents = fs::read_to_string(path).expect("read file");
        let mut in_cfg_test_block = false;
        let mut brace_depth_at_cfg_test: i32 = -1;
        let mut current_brace_depth: i32 = 0;

        for (lineno, line) in contents.lines().enumerate() {
            let trimmed = line.trim_start();
            // Track #[cfg(test)] mod blocks (simple heuristic — sufficient for amlich-core layout)
            if trimmed.starts_with("#[cfg(test)]") {
                in_cfg_test_block = true;
                brace_depth_at_cfg_test = current_brace_depth;
            }
            // Update brace depth (simple — not string-aware, but adequate for production code in this crate)
            for ch in line.chars() {
                match ch {
                    '{' => current_brace_depth += 1,
                    '}' => {
                        current_brace_depth -= 1;
                        if in_cfg_test_block && current_brace_depth <= brace_depth_at_cfg_test {
                            in_cfg_test_block = false;
                            brace_depth_at_cfg_test = -1;
                        }
                    }
                    _ => {}
                }
            }
            if in_cfg_test_block {
                continue;
            }
            // Skip doc-comments and line-comments
            if trimmed.starts_with("//") {
                continue;
            }
            for lit in FORBIDDEN_LITERALS {
                if line.contains(lit) {
                    violations.push(format!(
                        "{}:{}  bare literal {} — use crate::sources::SOURCE_* instead",
                        path.display(),
                        lineno + 1,
                        lit
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Bare source_id literals found in src/ — replace with crate::sources::SOURCE_* constants:\n{}",
        violations.join("\n")
    );
}

/// v1.10 (Phase 01-01, ADR-0003): the reserved `ty-ngo-luu-chu` source ID must
/// not appear ANYWHERE under `crates/amlich-core/src/` — not in production
/// code, not in `#[cfg(test)]` modules, not in doc-comments, not in inline
/// `//` comments. The first guard above already catches bare `"ty-ngo-luu-chu"`
/// string literals in production code; this second guard catches the
/// substring anywhere in the production source tree (including comments)
/// so that future contributors do not accidentally reference the reserved
/// identifier even when discussing it.
///
/// Scope is `crates/amlich-core/src/` only. The ADR document itself
/// (`docs/adr/0003-...`) and the planning research
/// (`.planning/research/LUNAR_HEALTH_RESEARCH.md`) intentionally mention
/// the term for context and are not in scope.
#[test]
fn ty_ngo_luu_chu_substring_never_appears_in_production_source() {
    let needle = "ty-ngo-luu-chu";
    let src_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect_rs_files(&src_dir, &mut files);

    let mut violations: Vec<String> = Vec::new();
    for path in &files {
        let contents = fs::read_to_string(path).expect("read file");
        for (lineno, line) in contents.lines().enumerate() {
            if line.contains(needle) {
                violations.push(format!(
                    "{}:{}  substring {} found — ADR-0003 reserves this id and forbids any reference in production source:\n    {}",
                    path.display(),
                    lineno + 1,
                    needle,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Forbidden `ty-ngo-luu-chu` substring found under crates/amlich-core/src/. Per ADR-0003 the full Tý Ngọ Lưu Chú source id is reserved for a future, separately reviewed milestone and must never be referenced in production source:\n{}",
        violations.join("\n")
    );
}
