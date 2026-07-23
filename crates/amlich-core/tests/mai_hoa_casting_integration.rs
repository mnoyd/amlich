//! Black-box integration tests for ICH-02 (Mai Hoa casting) and ICH-03
//! (biến quẻ derivation).
//!
//! These tests exercise the casting + biến quẻ surface from the EXTERNAL
//! crate path (`use amlich_core::iching::{...}`) — no access to internal
//! helpers — to verify the ICH-02 + ICH-03 success criteria from the
//! caller's perspective.
//!
//! Requirements closed:
//! - **ICH-02** — `cast_mai_hoa(...) -> MaiHoaCast` deterministic, CRIT-2
//!   boundary-safe (`((n-1)%k)+1`).
//! - **ICH-03** — `derive_bien_que(...) -> BienQue` flips động hào line,
//!   CRIT-4 384-case contract holds.
//!
//! Pitfalls gated:
//! - **CRIT-2** — Mai Hoa `% 8 == 0` / `% 6 == 0` remainder-zero convention.
//! - **CRIT-4** — biến quẻ bit-position correctness (flipping a line MUST
//!   change the hexagram).
//!
//! Mirror of the Phase 21-02 `iching_corpus_integration.rs` discipline
//! (black-box tests + WASM-safety grep guard).

use amlich_core::iching::{
    cast_mai_hoa, compose, derive_bien_que, BienQue, MaiHoaCast, TienThienTrigram,
};

// ---------------------------------------------------------------------------
// CRIT-2: all-eights boundary
// ---------------------------------------------------------------------------

/// CRIT-2 HEADLINE TEST — the worked boundary example from ADR-0006 §4.
///
/// `cast_mai_hoa(8, 8, 8, 8)` MUST yield:
///
/// - upper_trigram = Khôn (Tiên Thiên #8)
/// - lower_trigram = Khôn (Tiên Thiên #8)
/// - dong_hao = 2
/// - chu_que = King Wen #2 (Thuần Khôn)
///
/// The naïve `sum % 8 = 0 → coerce to 1` convention would produce Kiền
/// (Tiên Thiên #1) for BOTH trigrams and King Wen #1 (Thuần Kiền) — a
/// COMPLETELY DIFFERENT hexagram. The `((n-1) % k) + 1` form (locked in
/// `mai_hoa::mai_hoa_remainder`) produces Khôn directly. This test
/// EXPLICITLY asserts the Khôn result AND explicitly rejects the
/// naïve-convention failure mode.
#[test]
fn crit2_all_eights_boundary_yields_khon_not_kien() {
    let cast = cast_mai_hoa(8, 8, 8, 8);

    // Worked arithmetic from ADR-0006 §4:
    //   sum_base = 8+8+8 = 24 → ((24-1)%8)+1 = (23%8)+1 = 7+1 = 8 → Khôn
    //   sum_full = 24+8 = 32 → ((32-1)%8)+1 = (31%8)+1 = 7+1 = 8 → Khôn
    //   dong_hao = ((32-1)%6)+1 = (31%6)+1 = 1+1 = 2
    //   compose(Khôn, Khôn) = King Wen #2 (Thuần Khôn)
    assert_eq!(
        cast.upper_trigram,
        TienThienTrigram::Khon,
        "CRIT-2 violation: upper trigram must be Khôn (Tiên Thiên #8) for (8,8,8,8) \
         per ADR-0006 §4"
    );
    assert_eq!(
        cast.lower_trigram,
        TienThienTrigram::Khon,
        "CRIT-2 violation: lower trigram must be Khôn (Tiên Thiên #8) for (8,8,8,8) \
         per ADR-0006 §4"
    );
    assert_eq!(
        cast.dong_hao, 2,
        "động hào must be 2 for (8,8,8,8) per ADR-0006 §4"
    );
    assert_eq!(
        cast.chu_que,
        amlich_core::iching::KingWenHexagram(2),
        "chu_que must be King Wen #2 (Thuần Khôn) per ADR-0006 §4"
    );

    // EXPLICIT REJECTION of the naïve-convention failure mode. A future
    // refactor that regresses CRIT-2 (e.g. by switching to `sum % k` or
    // `(sum % k) + 1`) will produce King Wen #1 (Thuần Kiền) — this
    // assertion is the structural gate.
    assert_ne!(
        cast.chu_que,
        amlich_core::iching::KingWenHexagram(1),
        "CRIT-2 violation: chu_que MUST NOT be King Wen #1 (Thuần Kiền) — that is the \
         naïve-convention regression signature. The cast must use ((n-1)%k)+1."
    );

    // Inputs are preserved on the struct for traceability.
    assert_eq!(cast.lunar_year_branch, 8);
    assert_eq!(cast.lunar_month, 8);
    assert_eq!(cast.lunar_day, 8);
    assert_eq!(cast.chi_hour_index, 8);
}

// ---------------------------------------------------------------------------
// Determinism — no RNG, no wall-clock
// ---------------------------------------------------------------------------

/// Determinism — `cast_mai_hoa` is a pure function (no RNG, no wall-clock).
///
/// (1) Two calls with identical inputs return EQUAL `MaiHoaCast` values.
/// (2) Sweeping 20 distinct input tuples, each tuple produces a stable result
///     across 3 repeated calls.
#[test]
fn casting_is_deterministic_and_rng_free() {
    let a = cast_mai_hoa(3, 5, 14, 7);
    let b = cast_mai_hoa(3, 5, 14, 7);
    assert_eq!(
        a, b,
        "cast_mai_hoa must be deterministic — two calls with identical inputs MUST be equal"
    );

    // 20 distinct tuples, each repeated 3x.
    let tuples: Vec<(u8, u8, u8, u8)> = vec![
        (0, 1, 1, 0),
        (1, 2, 3, 4),
        (2, 3, 5, 6),
        (3, 4, 7, 8),
        (4, 5, 9, 10),
        (5, 6, 11, 0),
        (6, 7, 13, 2),
        (7, 8, 15, 4),
        (8, 9, 17, 6),
        (9, 10, 19, 8),
        (10, 11, 21, 10),
        (11, 12, 23, 0),
        (0, 6, 12, 6),
        (3, 9, 15, 9),
        (6, 12, 18, 0),
        (9, 1, 21, 3),
        (1, 7, 13, 5),
        (4, 10, 16, 8),
        (7, 1, 19, 11),
        (11, 8, 30, 0),
    ];
    assert_eq!(tuples.len(), 20);
    for t in tuples {
        let first = cast_mai_hoa(t.0, t.1, t.2, t.3);
        for _ in 0..3 {
            let again = cast_mai_hoa(t.0, t.1, t.2, t.3);
            assert_eq!(
                first, again,
                "cast_mai_hoa({t:?}) is unstable across repeated calls"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Range invariant — every cast lands in the valid range
// ---------------------------------------------------------------------------

/// Sweep the full valid input space (`12 × 12 × 30 × 12 = 51,840` casts) and
/// assert every `dong_hao` is `1..=6` and every `chu_que.0` is `1..=64`. Also
/// collect all derived trigrams and assert they are members of
/// `TienThienTrigram::ALL`.
#[test]
fn remainder_indices_always_in_range() {
    // Track which Tiên Thiên trigram discriminants (u8 values 1..=8) we've
    // seen. TienThienTrigram doesn't impl Hash, so use a [bool; 8] indexed
    // by discriminant - 1.
    let all_discriminants: Vec<u8> = TienThienTrigram::ALL.iter().map(|t| *t as u8).collect();

    let mut seen_upper = [false; 8];
    let mut seen_lower = [false; 8];

    for year_branch in 0u8..=11 {
        for month in 1u8..=12 {
            for day in 1u8..=30 {
                for hour in 0u8..=11 {
                    let cast = cast_mai_hoa(year_branch, month, day, hour);
                    assert!(
                        (1..=6).contains(&cast.dong_hao),
                        "dong_hao out of 1..=6 for ({year_branch},{month},{day},{hour}): {}",
                        cast.dong_hao
                    );
                    assert!(
                        (1..=64).contains(&cast.chu_que.0),
                        "chu_que out of 1..=64 for ({year_branch},{month},{day},{hour}): {}",
                        cast.chu_que.0
                    );
                    let upper_idx = (cast.upper_trigram as u8) - 1;
                    let lower_idx = (cast.lower_trigram as u8) - 1;
                    assert!(
                        (0..8).contains(&upper_idx),
                        "upper trigram discriminant out of 1..=8 for ({year_branch},{month},{day},{hour}): {:?}",
                        cast.upper_trigram
                    );
                    assert!(
                        (0..8).contains(&lower_idx),
                        "lower trigram discriminant out of 1..=8 for ({year_branch},{month},{day},{hour}): {:?}",
                        cast.lower_trigram
                    );
                    seen_upper[upper_idx as usize] = true;
                    seen_lower[lower_idx as usize] = true;
                }
            }
        }
    }

    // Sanity: the sweep should hit all 8 trigrams on both sides (the modulo
    // distribution over a 51,840-cast sweep is dense enough that every
    // Tiên Thiên position appears at least once).
    assert!(
        seen_upper.iter().all(|&x| x),
        "full sweep should visit every upper Tiên Thiên trigram at least once: {:?}",
        seen_upper
    );
    assert!(
        seen_lower.iter().all(|&x| x),
        "full sweep should visit every lower Tiên Thiên trigram at least once: {:?}",
        seen_lower
    );

    // All discriminants are valid 1..=8 (sanity check).
    for d in &all_discriminants {
        assert!(
            (1..=8).contains(d),
            "TienThienTrigram discriminant out of range: {d}"
        );
    }
}

// ---------------------------------------------------------------------------
// CRIT-4: 384-case biến quẻ contract
// ---------------------------------------------------------------------------

/// CRIT-4 HEADLINE TEST — the 384-case (64 chủ quẻ × 6 động hào) exhaustive
/// contract test.
///
/// For every `(upper, lower)` in `TienThienTrigram::ALL × TienThienTrigram::ALL`
/// (8 × 8 = 64 chủ quẻ) and every `dong_hao in 1..=6`:
///
/// 1. `bien.king_wen.0` is a valid King Wen index `1..=64`.
/// 2. `bien.king_wen != cast.chu_que` — flipping a line ALWAYS changes the
///    hexagram (the CRIT-4 invariant). This is the strongest invariant:
///    a 1-line flip on any hexagram can NEVER produce the same hexagram.
/// 3. EXACTLY ONE of `bien.upper_trigram != upper` / `bien.lower_trigram != lower`
///    is true (the flip changes EXACTLY ONE trigram, not both, not neither).
///
/// Total: 64 × 6 = 384 cases.
///
/// Cites ADR-0006 Consequences (re-composed via `COMPOSITION_TABLE` lookup).
#[test]
fn crit4_bien_que_384_case_exhaustive_contract() {
    for upper in TienThienTrigram::ALL {
        for lower in TienThienTrigram::ALL {
            let chu_que = compose(upper, lower);
            for dong_hao in 1u8..=6 {
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

                let bien: BienQue = derive_bien_que(&cast);

                // (1) king_wen is a valid 1..=64 index.
                assert!(
                    (1..=64).contains(&bien.king_wen.0),
                    "biến quẻ king_wen out of 1..=64 for (upper={upper:?}, lower={lower:?}, \
                     dong_hao={dong_hao}): got {}",
                    bien.king_wen.0
                );

                // (2) CRIT-4 invariant: flipping a line ALWAYS changes the hexagram.
                assert_ne!(
                    bien.king_wen, chu_que,
                    "CRIT-4 violation: flipping dong_hao={dong_hao} on chủ quẻ \
                     (upper={upper:?}, lower={lower:?}, chu_que=#{}) did NOT change the \
                     hexagram (biến quẻ == chủ quẻ == #{})",
                    chu_que.0, bien.king_wen.0
                );

                // (3) Exactly one trigram flipped.
                let upper_changed = bien.upper_trigram != upper;
                let lower_changed = bien.lower_trigram != lower;
                assert!(
                    upper_changed ^ lower_changed,
                    "CRIT-4 violation: flipping dong_hao={dong_hao} on chủ quẻ \
                     (upper={upper:?}, lower={lower:?}) should change EXACTLY ONE trigram \
                     but upper_changed={upper_changed} lower_changed={lower_changed}"
                );

                // Echo of the flipped line.
                assert_eq!(
                    bien.flipped_dong_hao, dong_hao,
                    "flipped_dong_hao must echo the input cast's dong_hao"
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Worked biến quẻ case (audit-grade documentation)
// ---------------------------------------------------------------------------

/// Worked biến quẻ for the CRIT-2 boundary cast.
///
/// `cast_mai_hoa(8, 8, 8, 8)` → chủ quẻ King Wen #2 (Thuần Khôn) with
/// dong_hao = 2 (a lower-trigram line). Flipping động hào 2:
///   lower Khôn (0,0,0) → flip line 2 → (0,1,0) = Khảm
///   upper Khôn unchanged (động hào 2 is a lower-trigram line)
/// biến quẻ = compose(upper=Khôn, lower=Khảm) = King Wen #7 (Sư,
/// earth above water) per COMPOSITION_TABLE line 189.
///
/// GUARDS:
/// - Explicit assertion of #7 (Sư).
/// - Explicit rejection of #8 (Tỷ), the upper/lower-trigram-order
///   inversion trap (`compose(Khảm, Khôn)` = #8 Tỷ is the WRONG order
///   for this case).
#[test]
fn bien_que_known_case_all_eights() {
    let cast = cast_mai_hoa(8, 8, 8, 8);
    let bien = derive_bien_que(&cast);

    // upper Khôn unchanged (động hào 2 is a lower-trigram line, lines 1-3).
    assert_eq!(
        bien.upper_trigram,
        TienThienTrigram::Khon,
        "upper trigram must be Khôn (unchanged) for (8,8,8,8) biến quẻ"
    );

    // lower Khôn (0,0,0) → flip line 2 → (0,1,0) = Khảm.
    assert_eq!(
        bien.lower_trigram,
        TienThienTrigram::Kham,
        "lower trigram must be Khảm (Khôn line 2 flipped yin→yang) for (8,8,8,8) biến quẻ"
    );

    // compose(Khôn, Khảm) = King Wen #7 (Sư) — earth above water.
    assert_eq!(
        bien.king_wen,
        amlich_core::iching::KingWenHexagram(7),
        "biến quẻ must be King Wen #7 (Sư) for (8,8,8,8)"
    );

    // Explicit rejection of the trigram-order inversion trap.
    // compose(Khảm, Khôn) would be #8 (Tỷ) — but we used the CORRECT
    // (upper=Khôn, lower=Khảm) order, so the result is #7 (Sư).
    assert_ne!(
        bien.king_wen,
        amlich_core::iching::KingWenHexagram(8),
        "biến quẻ must NOT be King Wen #8 (Tỷ) — that would indicate the \
         upper/lower trigram order was inverted (compose(Khảm, Khôn) = #8)."
    );

    // Echo.
    assert_eq!(bien.flipped_dong_hao, 2);
}

// ---------------------------------------------------------------------------
// CRIT-3 + WASM-safety grep guards
// ---------------------------------------------------------------------------

/// CRIT-3 isolation + WASM-safety + determinism grep guard.
///
/// Reads both source files via `include_str!` and asserts:
/// 1. NEITHER file defines any cross-newtype `From` impl
///    (`impl From<TienThienTrigram>`, `impl From<HauThienTrigram>`,
///    `impl From<KingWenHexagram>` — and the reverse-direction `impl<...>
///    From` form). CRIT-3 prevention at the integration level.
/// 2. NEITHER file uses `rand`, `Utc::now`, `std::fs::`, or
///    `use std::fs;` — pure integer arithmetic, no filesystem I/O, no
///    wall-clock, no RNG (WASM-safety + determinism discipline).
///
/// Mirrors the v1.5/v1.6 grep-guard discipline (Phase 21-02
/// `wasm_safety_no_fs_no_utc`, Phase 18 `fengshui_crit3_isolation`).
///
/// Constructed needle patterns are built at RUNTIME so the test's own
/// source doesn't trip the grep (the self-tripping problem).
#[test]
fn crit3_isolation_no_cross_newtype_from_impls() {
    const MAI_HOA_SRC: &str = include_str!("../src/iching/mai_hoa.rs");
    const BIEN_QUE_SRC: &str = include_str!("../src/iching/bien_que.rs");

    // Build cross-newtype From needles at runtime.
    let from_needles: Vec<String> = [
        ("Tien", "ThienTrigram"),
        ("Hau", "ThienTrigram"),
        ("King", "WenHexagram"),
    ]
    .iter()
    .flat_map(|(a, b)| [format!("impl From<{a}{b}"), format!("impl<{a}{b}> From")])
    .collect();

    for src_name in [("mai_hoa.rs", MAI_HOA_SRC), ("bien_que.rs", BIEN_QUE_SRC)] {
        for needle in &from_needles {
            assert!(
                !src_name.1.contains(needle.as_str()),
                "CRIT-3 violation: `{needle}` found in {srcn}. The three iching newtypes \
                 must NOT have cross-type From impls.",
                srcn = src_name.0
            );
        }
    }

    // WASM-safety + determinism guards. Anchored on actual USAGE patterns
    // (not bare substrings) to avoid false-positives on doc comments
    // legitimately mentioning the rule.
    for src_name in [("mai_hoa.rs", MAI_HOA_SRC), ("bien_que.rs", BIEN_QUE_SRC)] {
        let src = src_name.1;
        assert!(
            !src.contains("std::fs::") && !src.contains("use std::fs;"),
            "WASM-safety violation: filesystem path appears in {} (filesystem I/O is not WASM-safe)",
            src_name.0
        );
        assert!(
            !src.contains("Utc::now"),
            "WASM-safety violation: Utc::now appears in {} (wall-clock is not WASM-safe)",
            src_name.0
        );
        assert!(
            !src.contains("rand::"),
            "Determinism violation: rand:: usage appears in {} (RNG breaks CRIT-2 determinism)",
            src_name.0
        );
    }
}
