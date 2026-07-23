//! OnceLock-backed corpus loader for the 64-hexagram Ngô Tất Tố IChing corpus.
//!
//! The corpus is embedded at compile time via `include_str!` and parsed once on
//! first access through a `OnceLock` cache. Every Vietnamese text field is
//! NFC-normalized at load (RIT-08 precedent from `rituals/corpus.rs`); the
//! `hao_tu` length invariant is enforced (ADR-0005 §2: 7 entries for King Wen
//! #1 Kiền and #2 Khôn — the *dụng cửu* / *dụng lục* seventh line; 6 entries for
//! #3..=64); the `$schema_version` is asserted at load (`"iching-v1"`).
//!
//! Schema is frozen by ADR-0005. Any change to the loaded shape requires a
//! superseding ADR and a bump from `"iching-v1"` to `"iching-vN"` in
//! `$schema_version` on the corpus file.
//!
//! WASM-safe by construction: `include_str!` is compile-time, `OnceLock` is std,
//! and there is no filesystem I/O (`std::fs`) or wall-clock access
//! (`chrono::Utc`) anywhere in this module (verified by the
//! `wasm_safety_no_fs_no_utc` integration test, mirroring the v1.6
//! `tests/fengshui_crit3_isolation.rs` grep-guard discipline).

use serde::Deserialize;
use std::sync::OnceLock;
use unicode_normalization::{is_nfc, UnicodeNormalization};

use crate::iching::schema::{HexagramEntry, KingWenHexagram};

/// The 64-hexagram corpus, embedded at compile time. The path is two levels up
/// from `src/iching/` (matching the `rituals/corpus.rs` relative-depth
/// convention: `../../data/rituals/<file>.json`).
const HEXAGRAMS_JSON: &str = include_str!("../../data/iching/hexagrams.json");

/// `$schema_version` asserted at load. Mirrors the rituals precedent
/// (`EXPECTED_SCHEMA_VERSION = "rituals-v1"` in `rituals/corpus.rs:76`).
const EXPECTED_SCHEMA_VERSION: &str = "iching-v1";

/// Envelope shape of the corpus JSON file. Carries the schema-version marker
/// plus the entry list. Mirrors `RitualFile` in `rituals/corpus.rs:78-83`.
#[derive(Debug, Deserialize)]
struct HexagramFile {
    #[serde(rename = "$schema_version")]
    schema_version: String,
    entries: Vec<HexagramEntry>,
}

static HEXAGRAMS: OnceLock<Vec<HexagramEntry>> = OnceLock::new();

/// Return the full 64-entry IChing corpus as a static slice.
///
/// First call parses the embedded `HEXAGRAMS_JSON`, asserts the
/// `$schema_version == "iching-v1"`, and NFC-normalizes every text field on
/// every entry (enforcing the `hao_tu` length invariant per ADR-0005 §2).
/// Subsequent calls return the same cached slice (OnceLock idempotency).
///
/// # Panics
///
/// Panics if the embedded corpus fails to parse, if the `$schema_version` does
/// not equal `"iching-v1"`, or if any entry violates the `hao_tu` length rule.
/// The corpus is compile-embedded so any such failure is a build-time bug, not
/// a runtime condition.
pub fn all_hexagrams() -> &'static [HexagramEntry] {
    HEXAGRAMS
        .get_or_init(|| {
            let file: HexagramFile = serde_json::from_str(HEXAGRAMS_JSON)
                .unwrap_or_else(|e| panic!("Failed to parse iching corpus: {e}"));
            assert_eq!(
                file.schema_version, EXPECTED_SCHEMA_VERSION,
                "iching corpus schema_version must equal {:?} (ADR-0005); found {:?}",
                EXPECTED_SCHEMA_VERSION, file.schema_version
            );
            file.entries
                .into_iter()
                .map(normalize_and_validate)
                .collect()
        })
        .as_slice()
}

/// Look up a single hexagram by its King Wen sequence index.
///
/// Linear scan over the 64-entry cached slice — mirrors `compose()`'s
/// 64-iteration scan decision in `schema.rs:261-269` (premature to pre-compute
/// a reverse lookup map for 64 entries accessed rarely).
pub fn get_hexagram(index: KingWenHexagram) -> Option<&'static HexagramEntry> {
    all_hexagrams().iter().find(|e| e.king_wen_index == index)
}

/// NFC-normalize every Vietnamese text field on the entry AND enforce the
/// ADR-0005 §2 `hao_tu` length invariant.
///
/// # Panics
///
/// Panics if the `hao_tu` length rule is violated. The corpus is
/// compile-embedded so a violation is a build-time bug (caught by `cargo test`
/// before release), not a runtime condition.
fn normalize_and_validate(mut entry: HexagramEntry) -> HexagramEntry {
    // ADR-0005 §2 hao_tu length invariant: #1 Kiền & #2 Khôn carry 7 entries
    // (dụng cửu / dụng lục seventh line); #3..=64 carry 6 entries. Enforced at
    // load (cannot be a serde constraint — `Vec<String>` has no
    // length-dependent-on-other-field derive).
    let kw = entry.king_wen_index.0;
    let expected = if kw == 1 || kw == 2 { 7 } else { 6 };
    assert_eq!(
        entry.hao_tu.len(),
        expected,
        "hao_tu length rule violation for King Wen #{}: expected {}, got {} (ADR-0005 §2)",
        kw,
        expected,
        entry.hao_tu.len()
    );

    // RIT-08 NFC normalization: every Vietnamese text field gets passed through
    // `nfc()`. `is_nfc()` returns true for already-canonical text -> fast
    // early-out (the corpus is authored NFC, so the early-out is the common
    // path).
    entry.vi_name = nfc(&entry.vi_name);
    entry.thoai_tu = nfc(&entry.thoai_tu);
    entry.cat_hung = nfc(&entry.cat_hung);
    for line in entry.hao_tu.iter_mut() {
        *line = nfc(line);
    }
    // Reserved *_en fields — None in v1.7 but normalized if Some (forward-safety).
    if let Some(s) = entry.vi_name_en.as_deref() {
        entry.vi_name_en = Some(nfc(s));
    }
    if let Some(s) = entry.thoai_tu_en.as_deref() {
        entry.thoai_tu_en = Some(nfc(s));
    }
    if let Some(lines) = entry.hao_tu_en.as_mut() {
        for line in lines.iter_mut() {
            *line = nfc(line);
        }
    }
    entry
}

fn nfc(s: &str) -> String {
    if is_nfc(s) {
        s.to_string()
    } else {
        s.nfc().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// SC1: the corpus carries exactly 64 entries (one per King Wen index
    /// 1..=64).
    #[test]
    fn all_hexagrams_has_64_entries() {
        assert_eq!(all_hexagrams().len(), 64);
    }

    /// SC1: every King Wen index 1..=64 looks up successfully.
    #[test]
    fn every_index_lookup_succeeds() {
        for kw in 1..=64u8 {
            let index = KingWenHexagram::new(kw).expect("1..=64 must construct");
            let entry = get_hexagram(index)
                .unwrap_or_else(|| panic!("get_hexagram(KingWenHexagram({kw})) returned None"));
            assert_eq!(
                entry.king_wen_index, index,
                "returned entry king_wen_index mismatch for #{kw}"
            );
        }
    }

    /// SC1 + name spot-checks: #1 is "Thuần Kiền", #64 is the last hexagram.
    #[test]
    fn known_endpoints_have_expected_vi_names() {
        let first = get_hexagram(KingWenHexagram::new(1).unwrap()).unwrap();
        assert_eq!(first.vi_name, "Thuần Kiền");
        let last = get_hexagram(KingWenHexagram::new(64).unwrap()).unwrap();
        assert_eq!(last.king_wen_index.0, 64);
    }

    /// SC1: corpus is in ascending `king_wen_index` order (1, 2, ..., 64).
    #[test]
    fn entries_are_in_ascending_king_wen_order() {
        let all = all_hexagrams();
        for (i, entry) in all.iter().enumerate() {
            assert_eq!(
                entry.king_wen_index.0 as usize,
                i + 1,
                "entry at position {i} has king_wen_index {} (expected {})",
                entry.king_wen_index.0,
                i + 1
            );
        }
    }

    /// ADR-0005 §2: hao_tu length invariant — #1/#2 carry 7 entries (dụng cửu /
    /// dụng lục seventh line); #3..=64 carry 6 entries.
    #[test]
    fn hao_tu_length_invariant_at_load() {
        let one = get_hexagram(KingWenHexagram::new(1).unwrap()).unwrap();
        assert_eq!(one.hao_tu.len(), 7, "#1 Kiền must have 7 hao_tu (dụng cửu)");
        let two = get_hexagram(KingWenHexagram::new(2).unwrap()).unwrap();
        assert_eq!(two.hao_tu.len(), 7, "#2 Khôn must have 7 hao_tu (dụng lục)");
        let three = get_hexagram(KingWenHexagram::new(3).unwrap()).unwrap();
        assert_eq!(three.hao_tu.len(), 6, "#3 must have 6 hao_tu");
        let sixty_four = get_hexagram(KingWenHexagram::new(64).unwrap()).unwrap();
        assert_eq!(sixty_four.hao_tu.len(), 6, "#64 must have 6 hao_tu");
    }

    /// SC4: OnceLock idempotency — two calls return the same pointer.
    #[test]
    fn load_is_idempotent() {
        let a = all_hexagrams();
        let b = all_hexagrams();
        assert_eq!(
            a.as_ptr(),
            b.as_ptr(),
            "OnceLock should return the same slice on subsequent calls"
        );
        assert_eq!(a.len(), b.len());
    }

    /// SC3 (RIT-08 precedent): every text field on every returned entry is
    /// NFC-normalized.
    #[test]
    fn every_text_field_is_nfc() {
        for entry in all_hexagrams() {
            assert!(
                is_nfc(&entry.vi_name),
                "vi_name not NFC for #{}",
                entry.king_wen_index.0
            );
            assert!(
                is_nfc(&entry.thoai_tu),
                "thoai_tu not NFC for #{}",
                entry.king_wen_index.0
            );
            assert!(
                is_nfc(&entry.cat_hung),
                "cat_hung not NFC for #{}",
                entry.king_wen_index.0
            );
            for (i, line) in entry.hao_tu.iter().enumerate() {
                assert!(
                    is_nfc(line),
                    "hao_tu[{i}] not NFC for #{}",
                    entry.king_wen_index.0
                );
            }
        }
    }
}
