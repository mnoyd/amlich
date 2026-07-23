//! Mai Hoa Dịch Số (梅花易數) casting — pure deterministic derivation of the
//! upper/lower Tiên Thiên trigram pair + động hào (moving line) from lunar
//! calendar inputs.
//!
//! # Algorithm (locked by ADR-0006)
//!
//! ```text
//! sum_base = lunar_year_branch + lunar_month + lunar_day          // upper trigram
//! sum_full = sum_base + chi_hour_index                             // lower trigram + moving line
//!
//! upper_idx = ((sum_base - 1) % 8) + 1   // 1..=8  -> TienThienTrigram::ALL[upper_idx - 1]
//! lower_idx = ((sum_full - 1) % 8) + 1   // 1..=8  -> TienThienTrigram::ALL[lower_idx - 1]
//! dong_hao  = ((sum_full - 1) % 6) + 1   // 1..=6  (1 = bottom line)
//!
//! upper = TienThienTrigram::ALL[(upper_idx - 1) as usize]
//! lower = TienThienTrigram::ALL[(lower_idx - 1) as usize]
//! chu_que = compose(upper, lower)      // KingWenHexagram
//! ```
//!
//! Inputs are LUNAR (per ADR-0006 §2). The caller is responsible for
//! solar→lunar conversion via `almanac::lunar`; this module does NOT do it.
//!
//! # Determinism (CRIT-2 prevention)
//!
//! Every reduction uses the SINGLE named helper [`mai_hoa_remainder`] which
//! implements `((sum - 1) % k) + 1`. Replacing that helper with `sum % k` or
//! `(sum % k) + 1` SILENTLY CORRUPTS ~1/8 of castings (see ADR-0006 §4 for
//! the worked boundary proof). The boundary test (`crit2_all_eights_yields_khon`)
//! is the structural gate.
//!
//! # CRIT-3 isolation
//!
//! This module does NOT define any `impl From<...>` between the three
//! iching newtypes (`TienThienTrigram` / `HauThienTrigram` / `KingWenHexagram`).
//! The composition table + [`compose`] are the ONLY bridges.

use serde::{Deserialize, Serialize};

use crate::iching::schema::{compose, KingWenHexagram, TienThienTrigram};

/// The result of a single Mai Hoa casting: the four lunar inputs (preserved
/// for traceability / recasting) + the derived Tiên Thiên pair + động hào +
/// the chủ quẻ King Wen hexagram.
///
/// Field ranges (documented for callers; not enforced at construction —
/// out-of-range is a caller contract violation consistent with the rest of
/// the crate):
///
/// | field                  | range  |
/// |------------------------|--------|
/// | `lunar_year_branch`    | 0..=11 (Tý=0, Sửu=1, ..., Hợi=11) |
/// | `lunar_month`          | 1..=12 (canonical month only; leap-month deferred per ADR-0006 §2) |
/// | `lunar_day`            | 1..=30 |
/// | `chi_hour_index`       | 0..=11 (Tý=0, ..., Hợi=11) |
/// | `dong_hao`             | 1..=6  (1 = bottom hào) |
/// | `chu_que.0`            | 1..=64 (King Wen sequence index) |
///
/// See ADR-0006 for the locked casting convention.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaiHoaCast {
    pub lunar_year_branch: u8,
    pub lunar_month: u8,
    pub lunar_day: u8,
    pub chi_hour_index: u8,
    pub upper_trigram: TienThienTrigram,
    pub lower_trigram: TienThienTrigram,
    pub dong_hao: u8,
    pub chu_que: KingWenHexagram,
}

/// The CRIT-2 boundary-safe reduction.
///
/// Implements `((sum - 1) % k) + 1` per ADR-0006 §3. The `((n-1) % k) + 1`
/// form maps `sum % k == 0` to position `k` (the last position), not to 0
/// (which would coerce to 1 = Kiền under the naïve form — see ADR-0006 §4
/// for the worked boundary proof).
///
/// **Replacing this helper with `sum % k` or `(sum % k) + 1` regresses CRIT-2.**
/// The boundary test in `mod tests` is the structural gate.
fn mai_hoa_remainder(sum: i32, k: i32) -> i32 {
    ((sum - 1) % k) + 1
}

/// Cast a Mai Hoa Dịch Số hexagram from lunar calendar inputs.
///
/// Returns a [`MaiHoaCast`] carrying the four lunar inputs (for traceability)
/// + the derived Tiên Thiên upper/lower trigram pair + động hào (moving line)
/// + the chủ quẻ (principal hexagram) King Wen index.
///
/// Pure integer arithmetic — no RNG, no wall-clock, no filesystem. Two
/// calls with identical inputs return equal `MaiHoaCast` values (determinism
/// invariant; covered by the `casting_is_deterministic` test).
///
/// # Arguments
///
/// * `lunar_year_branch` — the Earthly Branch of the lunar year (0..=11)
/// * `lunar_month`       — the lunar month number (1..=12)
/// * `lunar_day`         — the lunar day-of-month (1..=30)
/// * `chi_hour_index`    — the Earthly Branch of the hour (0..=11)
///
/// Inputs are lunar; the caller is responsible for solar→lunar conversion
/// via `almanac::lunar` (ADR-0006 §2). DEC-0017 early-Tý / late-Tý split is
/// the CALLER's concern, not this function's.
pub fn cast_mai_hoa(
    lunar_year_branch: u8,
    lunar_month: u8,
    lunar_day: u8,
    chi_hour_index: u8,
) -> MaiHoaCast {
    let sum_base: i32 = (lunar_year_branch as i32) + (lunar_month as i32) + (lunar_day as i32);
    let sum_full: i32 = sum_base + (chi_hour_index as i32);

    // CRIT-2 lock: use the named helper, NOT `sum % k` or `(sum % k) + 1`.
    let upper_idx = mai_hoa_remainder(sum_base, 8) as u8; // 1..=8
    let lower_idx = mai_hoa_remainder(sum_full, 8) as u8; // 1..=8
    let dong_hao = mai_hoa_remainder(sum_full, 6) as u8; // 1..=6

    let upper = TienThienTrigram::ALL[(upper_idx - 1) as usize];
    let lower = TienThienTrigram::ALL[(lower_idx - 1) as usize];
    let chu_que = compose(upper, lower);

    MaiHoaCast {
        lunar_year_branch,
        lunar_month,
        lunar_day,
        chi_hour_index,
        upper_trigram: upper,
        lower_trigram: lower,
        dong_hao,
        chu_que,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// CRIT-2 HEADLINE TEST — all-eights boundary. The worked example from
    /// ADR-0006 §4: `cast_mai_hoa(8, 8, 8, 8)` must yield Khôn for BOTH
    /// trigrams, động hào 2, and King Wen hexagram #2 (Thuần Khôn).
    ///
    /// The naïve `24 % 8 = 0 → coerce to 1` would produce Kiền for BOTH
    /// trigrams and King Wen #1 (Thuần Kiền) — a completely different
    /// hexagram. The `((n-1) % k) + 1` form produces Khôn directly.
    ///
    /// Cites ADR-0006 §4 verbatim:
    ///   sum_base = 24 → ((24-1) % 8) + 1 = (23 % 8) + 1 = 7 + 1 = 8 → Khôn
    ///   sum_full = 32 → ((32-1) % 8) + 1 = (31 % 8) + 1 = 7 + 1 = 8 → Khôn
    ///   dong_hao = ((32-1) % 6) + 1 = (31 % 6) + 1 = 1 + 1 = 2
    ///   Result: compose(Khôn, Khôn) = King Wen #2 (Thuần Khôn)
    #[test]
    fn crit2_all_eights_yields_khon() {
        let cast = cast_mai_hoa(8, 8, 8, 8);

        assert_eq!(
            cast.upper_trigram,
            TienThienTrigram::Khon,
            "upper trigram must be Khôn (Tiên Thiên #8) at all-eights boundary (CRIT-2)"
        );
        assert_eq!(
            cast.lower_trigram,
            TienThienTrigram::Khon,
            "lower trigram must be Khôn (Tiên Thiên #8) at all-eights boundary (CRIT-2)"
        );
        assert_eq!(
            cast.dong_hao, 2,
            "động hào must be 2 at all-eights boundary (CRIT-2)"
        );
        assert_eq!(
            cast.chu_que,
            KingWenHexagram(2),
            "chủ quẻ must be King Wen #2 (Thuần Khôn) at all-eights boundary (CRIT-2). \
             King Wen #1 (Thuần Kiền) would indicate the naïve `sum % k` convention regressed."
        );

        // Explicit rejection of the naïve-convention failure mode.
        assert_ne!(
            cast.chu_que,
            KingWenHexagram(1),
            "chu_que must NOT be King Wen #1 (Thuần Kiền) — that is the naïve-convention \
             regression signature (CRIT-2)."
        );
    }

    /// Determinism — identical inputs MUST produce identical output (no RNG,
    /// no wall-clock). Verifies the cast is a pure function.
    #[test]
    fn casting_is_deterministic() {
        let a = cast_mai_hoa(3, 5, 14, 7);
        let b = cast_mai_hoa(3, 5, 14, 7);
        assert_eq!(a, b, "cast_mai_hoa must be deterministic (pure function)");

        // Spot-check one more tuple.
        let c = cast_mai_hoa(0, 1, 1, 0);
        let d = cast_mai_hoa(0, 1, 1, 0);
        assert_eq!(
            c, d,
            "cast_mai_hoa must be deterministic across inputs (pure function)"
        );
    }

    /// Worked non-boundary derivation: `(1, 1, 1, 1)`.
    ///
    /// sum_base = 1+1+1 = 3 → ((3-1) % 8) + 1 = (2 % 8) + 1 = 2 + 1 = 3 → Ly
    /// sum_full = 3+1 = 4 → ((4-1) % 8) + 1 = (3 % 8) + 1 = 3 + 1 = 4 → Chan
    /// dong_hao = ((4-1) % 6) + 1 = (3 % 6) + 1 = 3 + 1 = 4
    /// Result: compose(Ly, Chan) = King Wen #21 (Phệ Hạp, fire above thunder)
    #[test]
    fn non_boundary_one_one_one_one_yields_ly_chan_kw21() {
        let cast = cast_mai_hoa(1, 1, 1, 1);

        assert_eq!(cast.upper_trigram, TienThienTrigram::Ly);
        assert_eq!(cast.lower_trigram, TienThienTrigram::Chan);
        assert_eq!(cast.dong_hao, 4);
        assert_eq!(cast.chu_que, KingWenHexagram(21));
    }

    /// Sanity sweep: every cast's động hào lands in 1..=6 (never 0, never >6)
    /// and the chu_que is always a valid King Wen index 1..=64.
    ///
    /// Exhaustively sweep the valid input space (`12 * 12 * 30 * 12 = 51,840`
    /// casts — trivial integer ops, runs in <1s).
    #[test]
    fn remainder_indices_always_in_range() {
        for year_branch in 0u8..=11 {
            for month in 1u8..=12 {
                for day in 1u8..=30 {
                    for hour in 0u8..=11 {
                        let cast = cast_mai_hoa(year_branch, month, day, hour);
                        assert!(
                            (1..=6).contains(&cast.dong_hao),
                            "dong_hao out of 1..=6 range: {} for inputs ({},{},{},{})",
                            cast.dong_hao,
                            year_branch,
                            month,
                            day,
                            hour
                        );
                        assert!(
                            (1..=64).contains(&cast.chu_que.0),
                            "chu_que out of 1..=64 range: {} for inputs ({},{},{},{})",
                            cast.chu_que.0,
                            year_branch,
                            month,
                            day,
                            hour
                        );
                    }
                }
            }
        }
    }

    /// CRIT-3 isolation guard — this module must NOT add any cross-newtype
    /// `From` impl between `TienThienTrigram` / `HauThienTrigram` /
    /// `KingWenHexagram`. The composition table + `compose()` remain the only
    /// bridges.
    ///
    /// Constructs the cross-newtype From pattern strings at runtime so this
    /// test's own source doesn't trip the grep (mirrors the WASM-safety
    /// grep-guard discipline from
    /// `tests/iching_corpus_integration.rs::wasm_safety_no_fs_no_utc`).
    #[test]
    fn crit3_isolation_no_cross_newtype_from_impls_inline() {
        const SRC: &str = include_str!("mai_hoa.rs");
        // Build the cross-newtype From pattern strings at runtime so this
        // test's own definition doesn't appear in the source-grep.
        let needles: Vec<String> = [
            ("Tien", "ThienTrigram"),
            ("Hau", "ThienTrigram"),
            ("King", "WenHexagram"),
        ]
        .iter()
        .flat_map(|(a, b)| {
            let pat1 = format!("{a}{b}"); // not used directly
            let _ = pat1;
            [format!("impl From<{a}{b}"), format!("impl<{a}{b}> From")]
        })
        .collect();
        for needle in &needles {
            assert!(
                !SRC.contains(needle.as_str()),
                "CRIT-3 violation: `{needle}` found in mai_hoa.rs. \
                 The three iching newtypes must NOT have cross-type From impls."
            );
        }
    }
}
