//! CI guard: ensures ritual JSON corpus contains zero Hán (CJK Unified Ideographs)
//! characters. Văn khấn cổ truyền is Quốc-ngữ — any Hán code point indicates either
//! (a) a copy-paste from a Hán-Việt source without transliteration, or (b) encoding
//! drift. Either is a corpus-authoring bug.
//!
//! Threshold: 0 Hán code points anywhere in any ritual JSON file. ADR-0001 ships
//! NO `hannom_text` field; future Hán quotation requires a superseding ADR with a
//! schema addition before this threshold can be relaxed (per 11-RESEARCH.md Q1).
//!
//! Allow-listed scenarios:
//!   - `data/rituals/` does not exist yet (pre-fixture state; test no-ops).
//!   - Future Phase 12 `manifest.json` is scanned the same way — it must also be
//!     Hán-free.

use std::fs;
use std::path::Path;

fn is_han_char(c: char) -> bool {
    matches!(
        c,
        '\u{4E00}'..='\u{9FFF}'    // CJK Unified Ideographs
        | '\u{3400}'..='\u{4DBF}'  // CJK Ext-A
        | '\u{20000}'..='\u{2A6DF}' // CJK Ext-B
        | '\u{F900}'..='\u{FAFF}'  // CJK Compatibility Ideographs
    )
}

#[test]
fn ritual_corpus_rejects_han_characters() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/rituals");
    if !data_dir.exists() {
        // Phase 11 wave-1: data/rituals/ may not exist on first run. Guard is a no-op.
        return;
    }

    let mut violations: Vec<String> = Vec::new();
    for entry in fs::read_dir(&data_dir).expect("read_dir data/rituals") {
        let path = entry.expect("dir entry").path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let body = fs::read_to_string(&path).expect("read ritual json");
        let han_count = body.chars().filter(|&c| is_han_char(c)).count();
        if han_count > 0 {
            violations.push(format!(
                "{}: {} Hán code points found",
                path.display(),
                han_count
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "Hán characters detected in ritual corpus (threshold = 0). \
         Văn khấn cổ truyền must be Quốc-ngữ; any Hán quotation requires a \
         superseding ADR + schema change. Violations:\n{}",
        violations.join("\n")
    );
}
