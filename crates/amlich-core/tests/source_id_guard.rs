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
    "\"kinh-dich\"",       // NEW v1.7 (Phase 20-01, FND-09)
    "\"mai-hoa-dich-so\"", // NEW v1.7 (Phase 20-01, FND-09)
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
