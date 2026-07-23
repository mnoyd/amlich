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
///
/// Each row's comment carries the hexagram's Vietnamese / pinyin name for
/// audit; the `(upper, lower)` pair is the source of truth. The King Wen
/// sequence is NOT alphabetically ordered — every tuple was checked against
/// the classical 8×8 upper×lower trigram grid.
pub const COMPOSITION_TABLE: [(TienThienTrigram, TienThienTrigram); 64] = [
    (TienThienTrigram::Kien, TienThienTrigram::Kien), // #1  Thuần Kiền (Qián)
    (TienThienTrigram::Khon, TienThienTrigram::Khon), // #2  Thuần Khôn (Kūn)
    (TienThienTrigram::Kham, TienThienTrigram::Chan), // #3  Truân (Zhūn) — water above thunder
    (TienThienTrigram::Can, TienThienTrigram::Kham),  // #4  Mông (Méng) — mountain above water
    (TienThienTrigram::Kien, TienThienTrigram::Kham), // #5  Nhu (Xū) — heaven above water
    (TienThienTrigram::Kham, TienThienTrigram::Kien), // #6  Tụng (Sòng) — water above heaven
    (TienThienTrigram::Khon, TienThienTrigram::Kham), // #7  Sư (Shī) — earth above water
    (TienThienTrigram::Kham, TienThienTrigram::Khon), // #8  Tỷ (Bǐ) — water above earth
    (TienThienTrigram::Ton, TienThienTrigram::Kien),  // #9  Tiểu Súc (Xiǎo Chù) — wind above heaven
    (TienThienTrigram::Kien, TienThienTrigram::Doai), // #10 Lữ (Lǚ) — heaven above lake
    (TienThienTrigram::Khon, TienThienTrigram::Kien), // #11 Thái (Tài) — earth above heaven
    (TienThienTrigram::Kien, TienThienTrigram::Khon), // #12 Bỉ (Pǐ) — heaven above earth
    (TienThienTrigram::Kien, TienThienTrigram::Ly), // #13 Đồng Nhân (Tóng Rén) — heaven above fire
    (TienThienTrigram::Ly, TienThienTrigram::Kien), // #14 Đại Hữu (Dà Yǒu) — fire above heaven
    (TienThienTrigram::Khon, TienThienTrigram::Can), // #15 Khiêm (Qiān) — earth above mountain
    (TienThienTrigram::Chan, TienThienTrigram::Khon), // #16 Dự (Yù) — thunder above earth
    (TienThienTrigram::Doai, TienThienTrigram::Chan), // #17 Tùy (Suí) — lake above thunder
    (TienThienTrigram::Can, TienThienTrigram::Ton), // #18 Cổ (Gǔ) — mountain above wind
    (TienThienTrigram::Khon, TienThienTrigram::Doai), // #19 Lâm (Lín) — earth above lake
    (TienThienTrigram::Ton, TienThienTrigram::Khon), // #20 Quan (Guān) — wind above earth
    (TienThienTrigram::Ly, TienThienTrigram::Chan), // #21 Phệ Hạp (Shì Hé) — fire above thunder
    (TienThienTrigram::Can, TienThienTrigram::Ly),  // #22 Bí (Bì) — mountain above fire
    (TienThienTrigram::Can, TienThienTrigram::Khon), // #23 Bác (Bō) — mountain above earth
    (TienThienTrigram::Khon, TienThienTrigram::Chan), // #24 Phục (Fù) — earth above thunder
    (TienThienTrigram::Kien, TienThienTrigram::Chan), // #25 Vô Vọng (Wú Wàng) — heaven above thunder
    (TienThienTrigram::Can, TienThienTrigram::Kien), // #26 Đại Súc (Dà Chù) — mountain above heaven
    (TienThienTrigram::Can, TienThienTrigram::Chan), // #27 Di (Yí) — mountain above thunder
    (TienThienTrigram::Doai, TienThienTrigram::Ton), // #28 Đại Quá (Dà Guò) — lake above wind
    (TienThienTrigram::Kham, TienThienTrigram::Kham), // #29 Thuần Khảm (Kǎn) — water above water
    (TienThienTrigram::Ly, TienThienTrigram::Ly),    // #30 Thuần Ly (Lí) — fire above fire
    (TienThienTrigram::Doai, TienThienTrigram::Can), // #31 Hàm (Xián) — lake above mountain
    (TienThienTrigram::Chan, TienThienTrigram::Ton), // #32 Hằng (Héng) — thunder above wind
    (TienThienTrigram::Kien, TienThienTrigram::Can), // #33 Độn (Dùn) — heaven above mountain
    (TienThienTrigram::Chan, TienThienTrigram::Kien), // #34 Đại Tráng (Dà Zhuàng) — thunder above heaven
    (TienThienTrigram::Ly, TienThienTrigram::Khon),   // #35 Tấn (Jìn) — fire above earth
    (TienThienTrigram::Khon, TienThienTrigram::Ly),   // #36 Minh Di (Míng Yí) — earth above fire
    (TienThienTrigram::Ton, TienThienTrigram::Ly),    // #37 Gia Nhân (Jiā Rén) — wind above fire
    (TienThienTrigram::Ly, TienThienTrigram::Doai),   // #38 Khuê (Kuí) — fire above lake
    (TienThienTrigram::Kham, TienThienTrigram::Can),  // #39 Kiển (Jiǎn) — water above mountain
    (TienThienTrigram::Chan, TienThienTrigram::Kham), // #40 Giải (Xiè) — thunder above water
    (TienThienTrigram::Can, TienThienTrigram::Doai),  // #41 Tổn (Sǔn) — mountain above lake
    (TienThienTrigram::Ton, TienThienTrigram::Chan),  // #42 Ích (Yì) — wind above thunder
    (TienThienTrigram::Doai, TienThienTrigram::Kien), // #43 Quải (Guài) — lake above heaven
    (TienThienTrigram::Kien, TienThienTrigram::Ton),  // #44 Cấu (Gòu) — heaven above wind
    (TienThienTrigram::Doai, TienThienTrigram::Khon), // #45 Tụy (Cuì) — lake above earth
    (TienThienTrigram::Khon, TienThienTrigram::Ton),  // #46 Thăng (Shēng) — earth above wind
    (TienThienTrigram::Doai, TienThienTrigram::Kham), // #47 Khốn (Kùn) — lake above water
    (TienThienTrigram::Kham, TienThienTrigram::Ton),  // #48 Tỉnh (Jǐng) — water above wind
    (TienThienTrigram::Doai, TienThienTrigram::Ly),   // #49 Cách (Gé) — lake above fire
    (TienThienTrigram::Ly, TienThienTrigram::Ton),    // #50 Đỉnh (Dǐng) — fire above wind
    (TienThienTrigram::Chan, TienThienTrigram::Chan), // #51 Thuần Chấn (Zhèn) — thunder above thunder
    (TienThienTrigram::Can, TienThienTrigram::Can), // #52 Thuần Cấn (Gèn) — mountain above mountain
    (TienThienTrigram::Ton, TienThienTrigram::Can), // #53 Tiệm (Jiàn) — wind above mountain
    (TienThienTrigram::Chan, TienThienTrigram::Doai), // #54 Quy Muội (Guī Mèi) — thunder above lake
    (TienThienTrigram::Chan, TienThienTrigram::Ly), // #55 Phong (Fēng) — thunder above fire
    (TienThienTrigram::Ly, TienThienTrigram::Can),  // #56 Lữ (Lǚ) — fire above mountain
    (TienThienTrigram::Ton, TienThienTrigram::Ton), // #57 Thuần Tốn (Xùn) — wind above wind
    (TienThienTrigram::Doai, TienThienTrigram::Doai), // #58 Thuần Đoài (Duì) — lake above lake
    (TienThienTrigram::Ton, TienThienTrigram::Kham), // #59 Hoán (Huàn) — wind above water
    (TienThienTrigram::Kham, TienThienTrigram::Doai), // #60 Tiết (Jié) — water above lake
    (TienThienTrigram::Ton, TienThienTrigram::Doai), // #61 Trung Phu (Zhōng Fú) — wind above lake
    (TienThienTrigram::Chan, TienThienTrigram::Can), // #62 Tiểu Quá (Xiǎo Guò) — thunder above mountain
    (TienThienTrigram::Kham, TienThienTrigram::Ly),  // #63 Ký Tế (Jì Jì) — water above fire
    (TienThienTrigram::Ly, TienThienTrigram::Kham),  // #64 Vị Tế (Wèi Jì) — fire above water
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
            let roundtripped: TienThienTrigram = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(
                variant, roundtripped,
                "round-trip failed for {variant:?} ({json})"
            );
        }
        // Spot-check the canonical Mai Hoa boundary value (Tiên Thiên 1 = Kiền).
        assert_eq!(
            serde_json::to_string(&TienThienTrigram::Kien).unwrap(),
            "\"kien\""
        );
    }

    /// Serde stability for `HauThienTrigram`.
    #[test]
    fn hau_thien_trigram_serde_stability() {
        for variant in HauThienTrigram::ALL {
            let json = serde_json::to_string(&variant).expect("serialize");
            let roundtripped: HauThienTrigram = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(
                variant, roundtripped,
                "round-trip failed for {variant:?} ({json})"
            );
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
            let roundtripped: KingWenHexagram = serde_json::from_str(&json).expect("deserialize");
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
