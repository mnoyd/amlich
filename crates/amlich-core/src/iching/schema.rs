//! Locked v1.7 IChing (Kinh Dịch) schema — newtypes, composition table, and the
//! `HexagramEntry` corpus record.
//!
//! See ADR-0005 for the schema lock and ADR-0006 for the Mai Hoa casting
//! convention. CRIT-1 (schema-slip after corpus authored) is prevented by
//! locking this shape + proving it with a 1-entry serde round-trip probe
//! (`tests/iching_schema_probe.rs`) BEFORE any of the 64 corpus entries are
//! authored in Phase 21.
//!
//! CRIT-3 (Tiên Thiên trigram numbers vs King Wen hexagram numbers — different
//! mappings sharing a 1..N form) is prevented by declaring three distinct
//! newtypes with NO `impl From<...>` between them. The composition table is the
//! ONLY bridge: `(TienThienTrigram, TienThienTrigram) -> KingWenHexagram`.

use serde::{Deserialize, Serialize};

use crate::almanac::fengshui::golden::DeferralMarker;

// ===========================================================================
// Newtype 1 of 3: TienThienTrigram (Mai Hoa casting arrangement)
// ===========================================================================

/// Tiên Thiên (先天 — Earlier Heaven / Fuxi) trigram arrangement, used by Mai
/// Hoa Dịch Số casting.
///
/// Encoding (per ADR-0005 + vi.wikipedia Mai Hoa Dịch Số): the eight trigrams
/// carry their classical Tiên Thiên numbers, which are also the Mai Hoa
/// modulo-8 result range (after the `((n-1) % 8) + 1` boundary-safe form per
/// ADR-0006):
///
/// | variant | Tiên Thiên # | trigram |
/// |---------|-------------|---------|
/// | `Kien`  | 1           | ☰乾 heaven |
/// | `Doai`  | 2           | ☱兑 lake   |
/// | `Ly`    | 3           | ☲离 fire   |
/// | `Chan`  | 4           | ☳震 thunder|
/// | `Ton`   | 5           | ☴巽 wind   |
/// | `Kham`  | 6           | ☵坎 water  |
/// | `Can`   | 7           | ☶艮 mountain |
/// | `Khon`  | 8           | ☷坤 earth  |
///
/// CRIT-3 isolation: there is intentionally NO `impl From<TienThienTrigram>
/// for HauThienTrigram` (or for `KingWenHexagram`). The composition table is
/// the only bridge. See module-level docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum TienThienTrigram {
    Kien = 1,
    Doai = 2,
    Ly = 3,
    Chan = 4,
    Ton = 5,
    Kham = 6,
    Can = 7,
    Khon = 8,
}

impl TienThienTrigram {
    /// All eight trigrams in Tiên Thiên number order (1..=8).
    /// Mirrors the `Palace::ALL` static-array precedent at
    /// `crates/amlich-core/src/almanac/fengshui/types.rs:32-42`.
    pub const ALL: [TienThienTrigram; 8] = [
        TienThienTrigram::Kien,
        TienThienTrigram::Doai,
        TienThienTrigram::Ly,
        TienThienTrigram::Chan,
        TienThienTrigram::Ton,
        TienThienTrigram::Kham,
        TienThienTrigram::Can,
        TienThienTrigram::Khon,
    ];
}

// ===========================================================================
// Newtype 2 of 3: HauThienTrigram (King Wen / Lo Shu display arrangement)
// ===========================================================================

/// Hậu Thiên (後天 — Later Heaven / King Wen / Lo Shu) trigram arrangement,
/// used as display metadata on the corpus's `HexagramEntry`.
///
/// Encoding per ADR-0005 (Pitfall 1 pin): the Lo Shu palace numbers — the
/// EXACT assignment the project's `Palace` enum already uses at
/// `crates/amlich-core/src/almanac/fengshui/types.rs:15-43`. The 5/center is
/// skipped (Palace::Center is the palace, not a trigram):
///
/// | variant | Lo Shu # | trigram |
/// |---------|----------|---------|
/// | `Kham`  | 1        | ☵坎 water  |
/// | `Khon`  | 2        | ☷坤 earth  |
/// | `Chan`  | 3        | ☳震 thunder|
/// | `Ton`   | 4        | ☴巽 wind   |
/// | `Kien`  | 6        | ☰乾 heaven |
/// | `Doai`  | 7        | ☱兑 lake   |
/// | `Can`   | 8        | ☶艮 mountain |
/// | `Ly`    | 9        | ☲离 fire   |
///
/// The corpus (Ngô Tất Tố *Kinh Dịch Trọn Bộ*) follows the King Wen text
/// tradition, so displaying trigram numbers in the Hậu Thiên (Lo Shu)
/// arrangement is consistent with that tradition (CONTEXT.md line 54-55).
///
/// CRIT-3 isolation: there is intentionally NO `impl From<HauThienTrigram>
/// for TienThienTrigram` (or for `KingWenHexagram`). The composition table is
/// the only bridge. See module-level docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum HauThienTrigram {
    Kham = 1,
    Khon = 2,
    Chan = 3,
    Ton = 4,
    Kien = 6,
    Doai = 7,
    Can = 8,
    Ly = 9,
}

impl HauThienTrigram {
    /// All eight Hậu Thiên trigrams in Lo Shu number order (skipping 5/center).
    pub const ALL: [HauThienTrigram; 8] = [
        HauThienTrigram::Kham,
        HauThienTrigram::Khon,
        HauThienTrigram::Chan,
        HauThienTrigram::Ton,
        HauThienTrigram::Kien,
        HauThienTrigram::Doai,
        HauThienTrigram::Can,
        HauThienTrigram::Ly,
    ];
}

// ===========================================================================
// Newtype 3 of 3: KingWenHexagram (King Wen sequence index)
// ===========================================================================

/// King Wen (文王) hexagram sequence index — the canonical I Ching numbering
/// 1..=64 used by the Ngô Tất Tố corpus.
///
/// This is a `pub struct(u8)` newtype (NOT a 64-variant enum) because 64 named
/// variants is too verbose to maintain ergonomically; the composition table
/// carries the readable Tiên Thiên-pair → King Wen mapping. Construction is
/// gated by `new()` so out-of-range values cannot appear.
///
/// CRIT-3 isolation: there is intentionally NO `impl From<KingWenHexagram>
/// for TienThienTrigram` or `HauThienTrigram`. King Wen hexagram numbers
/// share the 1..N form with both trigram newtypes but mean something
/// completely different — that shared form is exactly the CRIT-3 trap. See
/// module-level docs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct KingWenHexagram(pub u8);

impl KingWenHexagram {
    /// Construct a `KingWenHexagram` for a valid King Wen sequence index
    /// (1..=64). Returns `None` for out-of-range values.
    pub const fn new(n: u8) -> Option<Self> {
        if n >= 1 && n <= 64 {
            Some(KingWenHexagram(n))
        } else {
            None
        }
    }
}

// ===========================================================================
// Composition table: the ONLY bridge between Tiên Thiên pairs and King Wen
// ===========================================================================

/// The 64 King Wen hexagrams indexed by King Wen number (index 0 = King Wen #1
/// = Thuần Kiền). Each entry is `(upper_trigram, lower_trigram)` in the Tiên
/// Thiên arrangement. Validated bijective at load (see
/// `tests::composition_table_is_bijective`).
///
/// CRITICAL: this array is the ONLY bridge between the Tiên Thiên pair space
/// and the King Wen index space. Hand-authored from the classical King Wen
/// sequence — the bijectivity test is the correctness proof.
// RED-PHASE PLACEHOLDER: all 64 entries are (Kien, Kien) — bijectivity test
// WILL FAIL on this stub. Replace with the real 64-entry table in GREEN.
pub const COMPOSITION_TABLE: [(TienThienTrigram, TienThienTrigram); 64] = [
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #1
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #2 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #3 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #4 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #5 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #6 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #7 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #8 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #9 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #10 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #11 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #12 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #13 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #14 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #15 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #16 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #17 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #18 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #19 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #20 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #21 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #22 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #23 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #24 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #25 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #26 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #27 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #28 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #29 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #30 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #31 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #32 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #33 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #34 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #35 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #36 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #37 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #38 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #39 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #40 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #41 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #42 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #43 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #44 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #45 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #46 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #47 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #48 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #49 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #50 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #51 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #52 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #53 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #54 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #55 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #56 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #57 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #58 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #59 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #60 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #61 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #62 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #63 (placeholder)
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #64 (placeholder)
];

/// Compose a Tiên Thiên upper+lower trigram pair into the King Wen hexagram
/// index.
///
/// Used by Phase 22's Mai Hoa casting algorithm. Linear scan over
/// `COMPOSITION_TABLE` (64 iterations — negligible; a pre-computed reverse
/// table is premature optimisation per 20-RESEARCH.md "Don't Hand-Roll").
///
/// # Panics
///
/// Panics if the pair is missing from the table. Unreachable by the
/// bijectivity contract (every pair is present); the panic is a contract
/// violation signal during development.
pub fn compose(upper: TienThienTrigram, lower: TienThienTrigram) -> KingWenHexagram {
    for (i, &(u, l)) in COMPOSITION_TABLE.iter().enumerate() {
        if u == upper && l == lower {
            return KingWenHexagram((i + 1) as u8);
        }
    }
    // Unreachable: every pair is present (bijectivity test guarantees this).
    panic!("composition table missing pair ({upper:?}, {lower:?})")
}

// ===========================================================================
// HexagramEntry: locked corpus record shape (CRIT-1 schema lock)
// ===========================================================================

/// A single entry in the 64-hexagram Ngô Tất Tố corpus.
///
/// FIELD SET LOCKED — any field-set change requires a superseding ADR
/// (CRIT-1 prevention; the 64 corpus entries × ~7 text fields = 448 fields
/// must not be re-edited due to a schema slip).
///
/// See ADR-0005 for the lock + 20-CONTEXT.md line 49-66 for the verbatim
/// shape rationale.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HexagramEntry {
    /// King Wen sequence index (1..=64). Newtype-isolated from trigram
    /// numbers per CRIT-3.
    pub king_wen_index: KingWenHexagram,
    /// Vietnamese hexagram name (NFC-normalised). E.g. `"Khôn / Địa"`.
    pub vi_name: String,
    /// Reserved English/Vietnamese-romanised name. Additive `Option<T>`
    /// discipline — absent in JSON when `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vi_name_en: Option<String>,
    /// Upper trigram in the Hậu Thiên (Lo Shu) arrangement — display
    /// metadata per the King Wen text tradition (NOT Tiên Thiên).
    pub upper_trigram: HauThienTrigram,
    /// Lower trigram in the Hậu Thiên (Lo Shu) arrangement.
    pub lower_trigram: HauThienTrigram,
    ///Thoái từ (彖辭) — the hexagram judgment text.
    pub thoai_tu: String,
    /// Reserved English thoái từ translation. Additive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thoai_tu_en: Option<String>,
    /// Hào từ (爻辭) — line texts. Six entries for hexagrams 3-64; seven
    /// entries for hexagrams #1 Kiền and #2 Khôn (the "dụng cửu" / "dụng
    /// lục" seventh line). Loader-enforced.
    pub hao_tu: Vec<String>,
    /// Reserved English hào từ translations. Additive.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hao_tu_en: Option<Vec<String>>,
    /// Cát hung (吉凶) — auspicious/inauspicious summary.
    pub cat_hung: String,
    /// Reviewer free-text marker — mirrors the Văn khấn corpus's
    /// `ExternalReviewPending(reason="..."; expected_review_date="...";
    /// assigned_to="...")` shape. Survives reviewer-name change without
    /// schema migration.
    pub reviewer: String,
    /// Typed `PendingExternalReview` deferral marker (reused verbatim from
    /// `almanac::fengshui::golden::DeferralMarker`, v1.6 RIT-14 pattern).
    /// Absent when the entry has cleared review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_review: Option<DeferralMarker>,
}

// ===========================================================================
// Inline tests: bijectivity + serde stability for the three newtypes
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// Bijectivity contract: the composition table is the correctness proof
    /// for the 64 Tiên Thiên pairs ↔ King Wen indices mapping.
    ///
    /// Asserts:
    /// 1. `COMPOSITION_TABLE.len() == 64`.
    /// 2. Every pair is distinct (no duplicate (upper, lower) tuples).
    /// 3. Every King Wen index 1..=64 has a table entry.
    /// 4. Every Tiên Thiên × Tiên Thiên (8×8=64) pair composes via `compose()`
    ///    without panic (surjective coverage).
    #[test]
    fn composition_table_is_bijective() {
        assert_eq!(COMPOSITION_TABLE.len(), 64);

        let mut seen: HashSet<(u8, u8)> = HashSet::new();
        for (i, &(upper, lower)) in COMPOSITION_TABLE.iter().enumerate() {
            let king_wen = i as u8 + 1;
            assert!(
                (1..=64).contains(&king_wen),
                "King Wen index out of range: {king_wen}"
            );
            let pair = (upper as u8, lower as u8);
            assert!(
                seen.insert(pair),
                "duplicate pair at King Wen #{king_wen}: {pair:?}"
            );
        }

        // Exhaustive surjectivity: every Tiên Thiên pair composes without panic.
        for u in TienThienTrigram::ALL {
            for l in TienThienTrigram::ALL {
                let _ = compose(u, l);
            }
        }
    }

    /// Serde stability for `TienThienTrigram`: serialises to snake_case name,
    /// deserialises back. Locks the JSON shape before the corpus exists.
    #[test]
    fn tien_thien_trigram_serde_stability() {
        for variant in TienThienTrigram::ALL {
            let json = serde_json::to_string(&variant).expect("serialize");
            let roundtripped: TienThienTrigram =
                serde_json::from_str(&json).expect("deserialize");
            assert_eq!(variant, roundtripped, "round-trip failed for {variant:?} ({json})");
        }
        // Spot-check the canonical Mai Hoa boundary value (Tiên Thiên 1 = Kiền).
        assert_eq!(serde_json::to_string(&TienThienTrigram::Kien).unwrap(), "\"kien\"");
    }

    /// Serde stability for `HauThienTrigram`.
    #[test]
    fn hau_thien_trigram_serde_stability() {
        for variant in HauThienTrigram::ALL {
            let json = serde_json::to_string(&variant).expect("serialize");
            let roundtripped: HauThienTrigram =
                serde_json::from_str(&json).expect("deserialize");
            assert_eq!(variant, roundtripped, "round-trip failed for {variant:?} ({json})");
        }
        // Spot-check: Hậu Thiên Khảm = Lo Shu 1.
        assert_eq!(HauThienTrigram::Kham as u8, 1);
        // Spot-check: Hậu Thiên Ly = Lo Shu 9 (NOT 5 — Pitfall 1 pin).
        assert_eq!(HauThienTrigram::Ly as u8, 9);
    }

    /// Serde + constructor stability for `KingWenHexagram`.
    #[test]
    fn king_wen_hexagram_serde_stability() {
        // Constructor accepts 1..=64, rejects 0 and 65+.
        assert!(KingWenHexagram::new(0).is_none());
        assert!(KingWenHexagram::new(65).is_none());
        for n in 1..=64u8 {
            let kw = KingWenHexagram::new(n).expect("1..=64 must construct");
            assert_eq!(kw.0, n);
            let json = serde_json::to_string(&kw).expect("serialize");
            let roundtripped: KingWenHexagram =
                serde_json::from_str(&json).expect("deserialize");
            assert_eq!(kw, roundtripped, "round-trip failed for King Wen #{n}");
        }
    }

    /// CRIT-3 isolation: `TienThienTrigram` and `HauThienTrigram` carry
    /// distinct discriminant assignments (the same logical trigram has a
    /// different number in each arrangement). This is a runtime spot-check
    /// of the CRIT-3 prevention that is structurally enforced by NOT
    /// defining any cross-newtype `From` impl.
    #[test]
    fn tien_thien_and_hau_thien_have_distinct_encodings() {
        // Kiền is Tiên Thiên 1 but Hậu Thiên 6.
        assert_eq!(TienThienTrigram::Kien as u8, 1);
        assert_eq!(HauThienTrigram::Kien as u8, 6);
        // Khôn is Tiên Thiên 8 but Hậu Thiên 2.
        assert_eq!(TienThienTrigram::Khon as u8, 8);
        assert_eq!(HauThienTrigram::Khon as u8, 2);
    }
}
