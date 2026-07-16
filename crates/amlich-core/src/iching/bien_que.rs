//! Biến quẻ (變卦, "transforming hexagram") derivation — flip the động hào
//! (moving line) on a [`MaiHoaCast`] and re-compose into the new King Wen
//! hexagram.
//!
//! # Algorithm
//!
//! A hexagram is 6 lines numbered bottom=1..top=6. Lower trigram = lines
//! 1-3; upper trigram = lines 4-6. Flipping the động hào bit at the correct
//! position is the entire algorithm:
//!
//! 1. Build the 6-line array `lines[0..6] = [lower[0], lower[1], lower[2],
//!    upper[0], upper[1], upper[2]]` (index `i` ↔ động hào position `i+1`).
//! 2. Flip `lines[dong_hao - 1] = 1 - lines[dong_hao - 1]`.
//! 3. Re-split: `new_lower = [lines[0..3]]`, `new_upper = [lines[3..6]]`.
//! 4. Map back to trigrams, then compose via [`compose`].
//!
//! The 384-case contract test (`crit4_bien_que_384_case_exhaustive_contract`)
//! is the correctness proof for CRIT-4: every `(upper, lower) × động hào`
//! tuple MUST produce a valid biến quẻ that DIFFERS from its chủ quẻ
//! (flipping a line always changes the hexagram).
//!
//! # CRIT-3 isolation
//!
//! This module does NOT define any `impl From<...>` between the three
//! iching newtypes. The composition table + [`compose`] are the ONLY bridges.

use serde::{Deserialize, Serialize};

use crate::iching::mai_hoa::MaiHoaCast;
use crate::iching::schema::{compose, KingWenHexagram, TienThienTrigram};

/// The result of a biến quẻ derivation: the new upper/lower trigram pair
/// (after flipping the động hào on the input cast's hexagram), the re-composed
/// King Wen hexagram index, and an echo of which động hào was flipped.
///
/// CRIT-3 isolation: `upper_trigram`/`lower_trigram` are `TienThienTrigram`
/// (same newtype as the input cast), NOT `HauThienTrigram`. The biến quẻ is
/// re-derived in the Tiên Thiên arrangement (the casting arrangement) and
/// the corpus's Hậu Thiên display is the consumer's concern.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BienQue {
    pub upper_trigram: TienThienTrigram,
    pub lower_trigram: TienThienTrigram,
    pub king_wen: KingWenHexagram,
    pub flipped_dong_hao: u8,
}

/// Trigram → 3-line bit pattern (bottom-to-top, yang=1, yin=0).
///
/// Classical Bā Guà patterns (cite: vi.wikipedia Bát Quái / Mai Hoa Dịch Số):
///
/// | Trigram | Symbol | Lines (b→t) |
/// |---------|--------|-------------|
/// | Kiền    | ☰      | `[1,1,1]`   |
/// | Đoài    | ☱      | `[1,1,0]`   |
/// | Ly      | ☲      | `[1,0,1]`   |
/// | Chấn    | ☳      | `[1,0,0]`   |
/// | Tốn     | ☴      | `[0,1,1]`   |
/// | Khảm    | ☵      | `[0,1,0]`   |
/// | Cấn     | ☶      | `[0,0,1]`   |
/// | Khôn    | ☷      | `[0,0,0]`   |
pub(crate) fn trigram_lines(_t: TienThienTrigram) -> [u8; 3] {
    unimplemented!("RED phase: implement classical 8 trigrams 3-line pattern (Bā Guà)")
}

/// 3-line bit pattern (bottom-to-top) → trigram.
///
/// Reverse of [`trigram_lines`]. Linear scan over `TienThienTrigram::ALL` is
/// adequate (8 entries, accessed once per biến quẻ derivation — a pre-computed
/// reverse map is premature).
pub(crate) fn lines_to_trigram(_lines: [u8; 3]) -> TienThienTrigram {
    unimplemented!("RED phase: implement reverse trigram lookup")
}

/// Derive the biến quẻ from a [`MaiHoaCast`] by flipping the động hào
/// (moving line) and re-composing the flipped trigram pair into the new
/// King Wen hexagram.
///
/// # Algorithm
///
/// 1. Build the 6-line array: `lines[0..6] = [lower[0], lower[1], lower[2],
///    upper[0], upper[1], upper[2]]` (index `i` ↔ động hào position `i+1`).
/// 2. Flip `lines[dong_hao - 1] = 1 - lines[dong_hao - 1]`.
/// 3. Re-split: `new_lower = [lines[0..3]]`, `new_upper = [lines[3..6]]`.
/// 4. Map back to trigrams, then compose via [`compose`].
///
/// Returns a [`BienQue`] with the new trigram pair + re-composed King Wen
/// index + echo of which động hào was flipped.
pub fn derive_bien_que(_cast: &MaiHoaCast) -> BienQue {
    unimplemented!("RED phase: implement biến quẻ derivation (flip động hào + re-compose)")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iching::mai_hoa::cast_mai_hoa;

    /// Bijectivity contract for [`trigram_lines`] ↔ [`lines_to_trigram`]:
    /// every Tiên Thiên trigram maps to a distinct 3-line pattern, and
    /// `lines_to_trigram` round-trips correctly for all 8.
    #[test]
    fn trigram_lines_bijective() {
        let mut seen: std::collections::HashSet<[u8; 3]> = std::collections::HashSet::new();
        for t in TienThienTrigram::ALL {
            let lines = trigram_lines(t);
            assert!(
                seen.insert(lines),
                "duplicate 3-line pattern for {t:?}: {lines:?}"
            );
            // Round-trip.
            let back = lines_to_trigram(lines);
            assert_eq!(
                back, t,
                "round-trip failed: {t:?} -> {lines:?} -> {back:?}"
            );
        }
        assert_eq!(seen.len(), 8, "expected 8 distinct 3-line patterns");
    }

    /// Worked biến quẻ for the CRIT-2 boundary cast: `(8, 8, 8, 8)`.
    ///
    /// chủ quẻ = King Wen #2 (Thuần Khôn) with upper=lower=Khôn, dong_hao=2.
    /// Flipping động hào 2 (a lower-trigram line, the 2nd line from bottom):
    ///   lower Khôn (0,0,0) → flip line 2 → (0,1,0) = Khảm
    ///   upper Khôn unchanged
    /// biến quẻ = compose(upper=Khôn, lower=Khảm) = King Wen #7 (Sư,
    /// earth above water) per COMPOSITION_TABLE line 189.
    ///
    /// Note: compose(Khảm, Khôn) = King Wen #8 (Tỷ) is the OPPOSITE
    /// trigram order — NOT this case. The trigram order (upper, lower)
    /// is the CRIT-4 trap; assert #7, NOT #8.
    #[test]
    fn crit4_all_eights_bien_que_is_kw7_su() {
        let cast = cast_mai_hoa(8, 8, 8, 8);
        assert_eq!(
            cast.chu_que,
            KingWenHexagram(2),
            "precondition: chủ quẻ must be #2 (Thuần Khôn) for (8,8,8,8)"
        );
        assert_eq!(
            cast.dong_hao, 2,
            "precondition: động hào must be 2 for (8,8,8,8)"
        );

        let bien = derive_bien_que(&cast);

        // upper Khôn unchanged (động hào 2 is a lower-trigram line).
        assert_eq!(
            bien.upper_trigram,
            TienThienTrigram::Khon,
            "upper trigram must be Khôn (unchanged — động hào 2 is a lower-trigram line)"
        );
        // lower Khôn (0,0,0) → flip line 2 → (0,1,0) = Khảm.
        assert_eq!(
            bien.lower_trigram,
            TienThienTrigram::Kham,
            "lower trigram must be Khảm (Khôn line 2 flipped from yin 0 to yang 1)"
        );
        // compose(Khôn, Khảm) = #7 (Sư, earth above water).
        assert_eq!(
            bien.king_wen,
            KingWenHexagram(7),
            "biến quẻ must be King Wen #7 (Sư) — compose(Khôn, Khảm). \
             King Wen #8 (Tỷ) would indicate the upper/lower trigram order was inverted."
        );
        // Explicit rejection of the trigram-order inversion trap.
        assert_ne!(
            bien.king_wen,
            KingWenHexagram(8),
            "biến quẻ must NOT be King Wen #8 (Tỷ) — that is compose(Khảm, Khôn), \
             the wrong trigram order for this case."
        );
        // Echo of the flipped line.
        assert_eq!(
            bien.flipped_dong_hao, 2,
            "flipped_dong_hao must echo the input cast's dong_hao"
        );
    }

    /// CRIT-4 HEADLINE TEST — 384-case exhaustive contract.
    ///
    /// For EVERY (upper, lower) pair in `TienThienTrigram::ALL × ALL`
    /// (8 × 8 = 64 chủ quẻ) and EVERY `dong_hao in 1..=6` (6 moving lines):
    ///
    /// 1. `bien.king_wen.0` is a valid King Wen index `1..=64`.
    /// 2. `bien.king_wen != cast.chu_que` — flipping a line ALWAYS changes
    ///    the hexagram (the CRIT-4 invariant).
    /// 3. `bien.king_wen != KingWenHexagram(0)` — sanity guard.
    ///
    /// Total: 64 × 6 = 384 cases.
    ///
    /// Note: this is the INLINE companion to the integration test
    /// `crit4_bien_que_384_case_exhaustive_contract` (which also asserts the
    /// "exactly one trigram flipped" property).
    #[test]
    fn crit4_bien_que_384_case_exhaustive_contract_inline() {
        for upper in TienThienTrigram::ALL {
            for lower in TienThienTrigram::ALL {
                let chu_que = compose(upper, lower);
                for dong_hao in 1u8..=6 {
                    // Construct a synthetic MaiHoaCast for the (upper, lower) pair
                    // + dong_hao. Fields are pub — construction is direct.
                    let cast = MaiHoaCast {
                        lunar_year_branch: 0,
                        lunar_month: 1,
                        lunar_day: 1,
                        chi_hour_index: 0,
                        upper_trigram: upper,
                        lower_trigram: lower,
                        dong_hao,
                        chu_que,
                    };

                    let bien = derive_bien_que(&cast);

                    assert!(
                        (1..=64).contains(&bien.king_wen.0),
                        "biến quẻ king_wen out of 1..=64 for (upper={upper:?}, lower={lower:?}, \
                         dong_hao={dong_hao}): got {}",
                        bien.king_wen.0
                    );
                    assert_ne!(
                        bien.king_wen, chu_que,
                        "CRIT-4 violation: flipping dong_hao={dong_hao} on chủ quẻ \
                         (upper={upper:?}, lower={lower:?}, chu_que=#{}) did NOT change the \
                         hexagram (biến quẻ == chủ quẻ == #{})",
                        chu_que.0, bien.king_wen.0
                    );
                }
            }
        }
    }

    /// CRIT-3 isolation guard — this module must NOT add any cross-newtype
    /// `From` impl between the three iching newtypes. See the mai_hoa.rs
    /// companion test for the runtime-built needle pattern (avoids the
    /// self-tripping source-grep trap).
    #[test]
    fn crit3_isolation_no_cross_newtype_from_impls_inline() {
        const SRC: &str = include_str!("bien_que.rs");
        let needles: Vec<String> = [
            ("Tien", "ThienTrigram"),
            ("Hau", "ThienTrigram"),
            ("King", "WenHexagram"),
        ]
        .iter()
        .flat_map(|(a, b)| {
            [
                format!("impl From<{a}{b}"),
                format!("impl<{a}{b}> From"),
            ]
        })
        .collect();
        for needle in &needles {
            assert!(
                !SRC.contains(needle.as_str()),
                "CRIT-3 violation: `{needle}` found in bien_que.rs. \
                 The three iching newtypes must NOT have cross-type From impls."
            );
        }
    }
}