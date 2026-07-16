//! Black-box integration tests for ICH-01 (Kinh Dịch corpus loader).
//!
//! These tests exercise the loader from the EXTERNAL crate path
//! (`use amlich_core::iching::{...}`) to verify the four ICH-01 success
//! criteria from the caller's perspective:
//!
//! - **SC1** — every King Wen index 1..=64 looks up to a populated entry, and
//!   the ADR-0005 §2 `hao_tu` length rule is honored.
//! - **SC2** — every entry carries the reviewer signature (free-text
//!   `ExternalReviewPending(...)` marker + typed `DeferralMarker`).
//! - **SC3** — every Vietnamese text field is NFC-normalized, and the
//!   `provenance_audit.md` ledger carries 64 rows all dispositioned
//!   `ExternalReviewPending`.
//! - **SC4** — the corpus load is lazy and idempotent (OnceLock), and the
//!   loader is structurally WASM-safe (no `std::fs`, no `Utc::now`).
//!
//! Test #3 (`corpus_trigram_identity_matches_composition_table`) is an
//! authoring-error catcher: it verifies every corpus entry's
//! `upper_trigram`/`lower_trigram` identity (by serialized serde NAME — e.g.
//! `"kien"` — NOT discriminant) matches the locked `COMPOSITION_TABLE`. This
//! preserves CRIT-3 isolation because we never convert between
//! `TienThienTrigram` and `HauThienTrigram`; we compare the serialized names
//! (which are identical because both enums carry `#[serde(rename_all =
//! "snake_case")]`).

use amlich_core::iching::{
    all_hexagrams, get_hexagram, KingWenHexagram, COMPOSITION_TABLE,
};
use unicode_normalization::is_nfc;

// ---------------------------------------------------------------------------
// SC1: every King Wen index 1..=64 looks up to a populated entry
// ---------------------------------------------------------------------------

/// SC1: for every King Wen index 1..=64, `get_hexagram` returns `Some`, the
/// returned entry's `king_wen_index` matches, and the entry carries every
/// required field populated (non-empty vi_name, thoai_tu, cat_hung, hao_tu;
/// valid HauThienTrigram variants for upper/lower trigrams).
#[test]
fn lookup_all_64_indices_succeed() {
    for kw in 1..=64u8 {
        let index = KingWenHexagram::new(kw).unwrap_or_else(|| {
            panic!("KingWenHexagram::new({kw}) must construct for 1..=64")
        });
        let entry = get_hexagram(index).unwrap_or_else(|| {
            panic!("get_hexagram(King Wen #{kw}) returned None")
        });
        assert_eq!(
            entry.king_wen_index, index,
            "returned entry king_wen_index mismatch for #{kw}"
        );
        assert!(
            !entry.vi_name.is_empty(),
            "vi_name empty for King Wen #{kw}"
        );
        assert!(
            !entry.thoai_tu.is_empty(),
            "thoai_tu empty for King Wen #{kw}"
        );
        assert!(
            !entry.cat_hung.is_empty(),
            "cat_hung empty for King Wen #{kw}"
        );
        assert!(
            !entry.hao_tu.is_empty(),
            "hao_tu vec empty for King Wen #{kw}"
        );
        // upper_trigram and lower_trigram are typed HauThienTrigram — by the
        // time they deserialize successfully they ARE valid variants (serde
        // would have rejected an unknown variant at load). We exercise them
        // by serializing back to their snake_case name (catches accidental
        // representation drift if the enum were ever re-encoded).
        let upper_name = serde_json::to_string(&entry.upper_trigram)
            .expect("upper_trigram serializes");
        let lower_name = serde_json::to_string(&entry.lower_trigram)
            .expect("lower_trigram serializes");
        assert!(
            !upper_name.is_empty() && upper_name.starts_with('"'),
            "upper_trigram for #{kw} did not serialize to a JSON string: {upper_name}"
        );
        assert!(
            !lower_name.is_empty() && lower_name.starts_with('"'),
            "lower_trigram for #{kw} did not serialize to a JSON string: {lower_name}"
        );
    }
}

/// SC1 + ADR-0005 §2: the `hao_tu` length rule — 7 entries for #1 Kiền and
/// #2 Khôn (dụng cửu / dụng lục seventh line); 6 entries for #3..=64.
#[test]
fn hao_tu_length_rule_honored() {
    // #1 and #2 carry the seventh "dụng" line.
    for kw in [1u8, 2u8] {
        let entry = get_hexagram(KingWenHexagram::new(kw).unwrap()).unwrap();
        assert_eq!(
            entry.hao_tu.len(),
            7,
            "King Wen #{kw} must have 7 hao_tu entries (dụng cửu / dụng lục)"
        );
    }
    // A spread of mid-and-late hexagrams all carry exactly 6.
    for kw in [3u8, 10, 33, 64] {
        let entry = get_hexagram(KingWenHexagram::new(kw).unwrap()).unwrap();
        assert_eq!(
            entry.hao_tu.len(),
            6,
            "King Wen #{kw} must have 6 hao_tu entries (no seventh line)"
        );
    }
}

/// Authoring-error catcher (cross-checks corpus against `COMPOSITION_TABLE`).
///
/// For every King Wen index 1..=64, the corpus entry's `upper_trigram` /
/// `lower_trigram` identity (as serialized serde NAME — e.g. `"kien"`) MUST
/// match the `COMPOSITION_TABLE[i]` Tiên Thiên pair. Both enums carry
/// `#[serde(rename_all = "snake_case")]` so the same logical trigram
/// serializes to the same JSON string in either arrangement.
///
/// CRIT-3 is preserved: we NEVER convert between `TienThienTrigram` and
/// `HauThienTrigram`. We compare their serialized NAMES (identity), not
/// discriminants.
#[test]
fn corpus_trigram_identity_matches_composition_table() {
    for (i, &(tt_upper, tt_lower)) in COMPOSITION_TABLE.iter().enumerate() {
        let king_wen = (i + 1) as u8;
        let entry = get_hexagram(KingWenHexagram::new(king_wen).unwrap())
            .expect("every COMPOSITION_TABLE index maps to a corpus entry");

        // Compare by serialized NAME (identity), NOT discriminant. Both enums
        // share #[serde(rename_all = "snake_case")], so TienThienTrigram::Kien
        // and HauThienTrigram::Kien both serialize to "\"kien\"".
        let tt_upper_name = serde_json::to_string(&tt_upper).unwrap();
        let tt_lower_name = serde_json::to_string(&tt_lower).unwrap();
        let ht_upper_name = serde_json::to_string(&entry.upper_trigram).unwrap();
        let ht_lower_name = serde_json::to_string(&entry.lower_trigram).unwrap();

        assert_eq!(
            tt_upper_name, ht_upper_name,
            "trigram IDENTITY mismatch at King Wen #{king_wen} (upper): \
             composition table serialized as {tt_upper_name}, corpus as {ht_upper_name}"
        );
        assert_eq!(
            tt_lower_name, ht_lower_name,
            "trigram IDENTITY mismatch at King Wen #{king_wen} (lower): \
             composition table serialized as {tt_lower_name}, corpus as {ht_lower_name}"
        );
    }
}

// ---------------------------------------------------------------------------
// SC2: every entry carries the reviewer signature
// ---------------------------------------------------------------------------

/// SC2: every entry carries the `ExternalReviewPending(...)` free-text reviewer
/// marker on `reviewer` AND the typed `DeferralMarker` on `pending_review`, and
/// the two surfaces are consistent (same `assigned_to` and
/// `expected_review_date`).
#[test]
fn every_entry_carries_reviewer_signature() {
    for entry in all_hexagrams() {
        let kw = entry.king_wen_index.0;
        // Free-text reviewer marker must mention ExternalReviewPending.
        assert!(
            entry.reviewer.contains("ExternalReviewPending"),
            "reviewer for King Wen #{kw} must contain 'ExternalReviewPending' (got: {:?})",
            entry.reviewer
        );
        // And both consistency anchors.
        assert!(
            entry.reviewer.contains("external-kinh-dich-reviewer"),
            "reviewer for King Wen #{kw} must reference 'external-kinh-dich-reviewer'"
        );
        assert!(
            entry.reviewer.contains("2026-12-31"),
            "reviewer for King Wen #{kw} must reference expected_review_date '2026-12-31'"
        );

        // Typed DeferralMarker must be present.
        let pending = entry
            .pending_review
            .as_ref()
            .unwrap_or_else(|| panic!("pending_review is None for King Wen #{kw}"));
        assert_eq!(
            pending.expected_review_date, "2026-12-31",
            "pending_review.expected_review_date mismatch for King Wen #{kw}"
        );
        assert_eq!(
            pending.assigned_to.as_deref(),
            Some("external-kinh-dich-reviewer"),
            "pending_review.assigned_to mismatch for King Wen #{kw}"
        );
        assert!(
            !pending.reason.is_empty(),
            "pending_review.reason is empty for King Wen #{kw}"
        );
    }
}

// ---------------------------------------------------------------------------
// SC3: NFC normalization + provenance ledger
// ---------------------------------------------------------------------------

/// SC3 (RIT-08 precedent): every Vietnamese text field on every returned entry
/// is NFC-normalized.
#[test]
fn every_text_field_is_nfc_normalized() {
    for entry in all_hexagrams() {
        let kw = entry.king_wen_index.0;
        assert!(is_nfc(&entry.vi_name), "vi_name not NFC for King Wen #{kw}");
        assert!(is_nfc(&entry.thoai_tu), "thoai_tu not NFC for King Wen #{kw}");
        assert!(is_nfc(&entry.cat_hung), "cat_hung not NFC for King Wen #{kw}");
        for (i, line) in entry.hao_tu.iter().enumerate() {
            assert!(
                is_nfc(line),
                "hao_tu[{i}] not NFC for King Wen #{kw}: {line:?}"
            );
        }
        // Reserved *_en fields (None in v1.7) normalized if Some.
        if let Some(s) = &entry.vi_name_en {
            assert!(is_nfc(s), "vi_name_en not NFC for King Wen #{kw}");
        }
        if let Some(s) = &entry.thoai_tu_en {
            assert!(is_nfc(s), "thoai_tu_en not NFC for King Wen #{kw}");
        }
        if let Some(lines) = &entry.hao_tu_en {
            for (i, line) in lines.iter().enumerate() {
                assert!(
                    is_nfc(line),
                    "hao_tu_en[{i}] not NFC for King Wen #{kw}"
                );
            }
        }
    }
}

/// SC3: the provenance ledger (`data/iching/provenance_audit.md`) carries
/// exactly 64 data rows, all dispositioned `ExternalReviewPending`. Mirrors
/// the Phase 17 RIT-14 ledger-driven test pattern — the test reads the
/// ledger so it cannot drift from the audit.
#[test]
fn provenance_ledger_has_64_rows_all_pending() {
    const LEDGER: &str = include_str!("../data/iching/provenance_audit.md");

    // Each data row matches `| <number> |` at the start of the line.
    // Header / separator rows (`|---|...`) and section sub-headings do NOT
    // match this pattern.
    let data_rows: Vec<&str> = LEDGER
        .lines()
        .filter(|line| {
            // Trim leading whitespace then require `| <digits> |`.
            let trimmed = line.trim_start();
            trimmed.starts_with("| ")
                && trimmed
                    .get(2..)
                    .map(|rest| rest.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false))
                    .unwrap_or(false)
        })
        .collect();

    assert_eq!(
        data_rows.len(),
        64,
        "provenance ledger must carry exactly 64 data rows (one per King Wen index); found {}",
        data_rows.len()
    );

    // Every data row must carry the ExternalReviewPending disposition.
    for (i, row) in data_rows.iter().enumerate() {
        assert!(
            row.contains("ExternalReviewPending"),
            "provenance ledger row {i} is not dispositioned ExternalReviewPending: {row}"
        );
    }
}

// ---------------------------------------------------------------------------
// SC4: lazy + idempotent + WASM-safe
// ---------------------------------------------------------------------------

/// SC4: the corpus load is lazy (OnceLock) — two calls return slices backed by
/// the SAME allocation (pointer equality).
#[test]
fn load_is_lazy_and_idempotent() {
    let a = all_hexagrams();
    let b = all_hexagrams();
    assert_eq!(
        a.as_ptr(),
        b.as_ptr(),
        "OnceLock should return the same slice pointer on subsequent calls"
    );
    assert_eq!(a.len(), b.len(), "len should match between calls");
    assert_eq!(a.len(), 64);
}

/// SC4: structural WASM-safety — the loader must NOT use `std::fs` (filesystem
/// access) or `chrono::Utc::now()` (wall-clock). `include_str!` is
/// compile-time; `OnceLock` is std (WASM-safe). Mirrors the v1.6
/// `tests/fengshui_crit3_isolation.rs` grep-guard discipline.
#[test]
fn wasm_safety_no_fs_no_utc() {
    const CORPUS_SRC: &str = include_str!("../src/iching/corpus.rs");
    // Match actual USAGE of the filesystem / wall-clock APIs — i.e. the
    // `::` path qualifier (e.g. `std::fs::read_to_string`, `use std::fs`,
    // `Utc::now::...`). A bare mention of the string in a doc comment
    // (e.g. this very doc paragraph) MUST NOT trigger the guard, so we
    // anchor on the qualifier.
    assert!(
        !CORPUS_SRC.contains("std::fs::") && !CORPUS_SRC.contains("use std::fs;"),
        "WASM violation: std::fs path appears in corpus.rs (filesystem I/O is not WASM-safe)"
    );
    assert!(
        !CORPUS_SRC.contains("Utc::now"),
        "WASM violation: Utc::now appears in corpus.rs (wall-clock is not WASM-safe)"
    );
}
