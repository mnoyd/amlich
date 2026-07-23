//! Thể / Dụng (体用) classification + Ngũ Hành sinh/khắc → Cát/Hùng/Bình
//! verdict for a [`MaiHoaCast`].
//!
//! # Classical Mai Hoa Thể Dụng rules (cite Thiệu Khang Tiết *Mai Hoa Dịch Số*
//! + nhantu.net "Cách lập quẻ Mai Hoa")
//!
//! ## Thể / Dụng assignment
//!
//! - Thể (体, "body/subject") = the trigram NOT containing the động hào.
//! - Dụng (用, "function/object") = the trigram CONTAINING the động hào.
//! - động hào 1, 2, 3 → lower trigram is Dụng, upper trigram is Thể.
//! - động hào 4, 5, 6 → upper trigram is Dụng, lower trigram is Thể.
//!
//! ## Trigram → Ngũ Hành (classical Bát Quái Ngũ Hành, intrinsic to the trigram)
//!
//! | Trigram | Element |
//! |---------|---------|
//! | Kiền ☰, Đoài ☱ | Kim (Metal)   |
//! | Ly      ☲       | Hoa (Fire)   |
//! | Chấn ☳, Tốn ☴ | Moc (Wood)   |
//! | Khảm    ☵       | Thuy (Water) |
//! | Cấn ☶, Khôn ☷ | Tho (Earth)  |
//!
//! ## Ngũ Hành sinh cycle (generates): Mộc → Hỏa → Thổ → Kim → Thủy → Mộc.
//!
//! ## Ngũ Hành khắc cycle (controls): Mộc → Thổ → Thủy → Hỏa → Kim → Mộc.
//!
//! ## Cát / Hung / Bình verdict table (v1.7 scope: the 5-way relation + verdict).
//! DF-02 (full nuanced matrix with seasonal strength) is DEFERRED to v1.8 per
//! research SUMMARY.
//!
//! | Relation        | Meaning                                          | Verdict |
//! |-----------------|--------------------------------------------------|---------|
//! | Dụng sinh Thể   | the situation nourishes the subject              | Cát     |
//! | Thể khắc Dụng   | the subject controls the situation               | Cát     |
//! | Thể Dụng đồng   | same element                                     | Bình    |
//! | Thể sinh Dụng   | the subject is depleted by the situation ("hao") | Hung    |
//! | Dụng khắc Thể   | the situation attacks the subject                | Hung    |
//!
//! # CRIT-3 isolation
//!
//! This module deliberately defines `trigram_element` as a plain `fn`, NOT as
//! an `From`-trait impl between `TienThienTrigram` and `FiveElement`. The three
//! iching newtypes carry no cross-`From` impls; this module participates in that
//! discipline.
//!
//! # Determinism
//!
//! Pure element mapping + classification. No RNG, no wall-clock, no filesystem.
//!
//! See ADR-0006 (casting convention) + Plan 22-02's `<the_dung_rules_pinned>`.

use serde::{Deserialize, Serialize};

use crate::almanac::types::FiveElement;
use crate::iching::mai_hoa::MaiHoaCast;
use crate::iching::schema::TienThienTrigram;

// ===========================================================================
// Trigram → Ngũ Hành mapping
// ===========================================================================

/// Map a [`TienThienTrigram`] to its intrinsic Ngũ Hành element.
///
/// Classical Bát Quái Ngũ Hành — same mapping for Tiên Thiên and Hậu Thiên
/// (the element is intrinsic to the trigram, not the arrangement).
///
/// CRIT-3 isolation: this is a plain `fn`, not `impl From<...>`. The three
/// iching newtypes carry no cross-`From` impls.
pub fn trigram_element(t: TienThienTrigram) -> FiveElement {
    match t {
        // Kim (Metal) — Kiền ☰ heaven, Đoài ☱ lake.
        TienThienTrigram::Kien | TienThienTrigram::Doai => FiveElement::Kim,
        // Hoa (Fire) — Ly ☲ fire.
        TienThienTrigram::Ly => FiveElement::Hoa,
        // Moc (Wood) — Chấn ☳ thunder, Tốn ☴ wind.
        TienThienTrigram::Chan | TienThienTrigram::Ton => FiveElement::Moc,
        // Thuy (Water) — Khảm ☵ water.
        TienThienTrigram::Kham => FiveElement::Thuy,
        // Tho (Earth) — Cấn ☶ mountain, Khôn ☷ earth.
        TienThienTrigram::Can | TienThienTrigram::Khon => FiveElement::Tho,
    }
}

// ===========================================================================
// Sinh / Khắc cycle helpers (CLASSICAL Mai Hoa — does NOT reuse
// `interaction::element_resonance` because that encodes Bazi day/target scoring
// semantics, not Mai Hoa Thể/Dụng semantics).
// ===========================================================================

/// Classic Ngũ Hành sinh relation: does `a` generate `b`?
///
/// sinh cycle: Mộc → Hỏa → Thổ → Kim → Thủy → Mộc.
fn generates(a: FiveElement, b: FiveElement) -> bool {
    matches!(
        (a, b),
        (FiveElement::Moc, FiveElement::Hoa)
            | (FiveElement::Hoa, FiveElement::Tho)
            | (FiveElement::Tho, FiveElement::Kim)
            | (FiveElement::Kim, FiveElement::Thuy)
            | (FiveElement::Thuy, FiveElement::Moc)
    )
}

/// Classic Ngũ Hành khắc relation: does `a` control (overcome) `b`?
///
/// khắc cycle: Mộc → Thổ → Thủy → Hỏa → Kim → Mộc.
fn controls(a: FiveElement, b: FiveElement) -> bool {
    matches!(
        (a, b),
        (FiveElement::Moc, FiveElement::Tho)
            | (FiveElement::Tho, FiveElement::Thuy)
            | (FiveElement::Thuy, FiveElement::Hoa)
            | (FiveElement::Hoa, FiveElement::Kim)
            | (FiveElement::Kim, FiveElement::Moc)
    )
}

// ===========================================================================
// TheDungRelation + CatHung verdict enum
// ===========================================================================

/// The 5-way relational classification between Thể and Dụng per the classical
/// Mai Hoa Thể Dụng rule (cite Thiệu Khang Tiết *Mai Hoa Dịch Số*).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TheDungRelation {
    /// Dụng sinh Thể — situation nourishes subject.
    DungSinhThe,
    /// Thể khắc Dụng — subject controls situation.
    TheKhacDung,
    /// Thể Dụng đồng — same element.
    Dong,
    /// Thể sinh Dụng — subject is depleted (hao).
    TheSinhDung,
    /// Dụng khắc Thể — situation attacks subject.
    DungKhacThe,
}

impl TheDungRelation {
    /// Map the 5-way relation to its cát / hùng / bình verdict.
    ///
    /// | Relation        | Verdict |
    /// |-----------------|---------|
    /// | DungSinhThe     | Cat     |
    /// | TheKhacDung     | Cat     |
    /// | Dong            | Binh    |
    /// | TheSinhDung     | Hung    |
    /// | DungKhacThe     | Hung    |
    pub fn cat_hung(self) -> CatHung {
        match self {
            TheDungRelation::DungSinhThe | TheDungRelation::TheKhacDung => CatHung::Cat,
            TheDungRelation::Dong => CatHung::Binh,
            TheDungRelation::TheSinhDung | TheDungRelation::DungKhacThe => CatHung::Hung,
        }
    }
}

/// Cát / Hung / Bình — the classical auspicious verdict for a cast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatHung {
    /// Cát — auspicious.
    Cat,
    /// Bình — neutral / stable.
    Binh,
    /// Hung — inauspicious.
    Hung,
}

// ===========================================================================
// TheDungClassification — the surfaced result for a MaiHoaCast
// ===========================================================================

/// The surfaced Thể/Dụng classification + Ngũ Hành elements + sinh/khắc
/// relation + cát/hùng verdict for a single [`MaiHoaCast`].
///
/// This is the readable interpretation layer that consumers (the IChing
/// evaluator + semantic-graph builder + downstream readers) use to render the
/// cast's "what is the situation vs what is the subject" reading.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TheDungClassification {
    /// Thể (体) trigram — the trigram NOT containing the động hào.
    pub the_trigram: TienThienTrigram,
    /// Dụng (用) trigram — the trigram CONTAINING the động hào.
    pub dung_trigram: TienThienTrigram,
    /// Echo of the cast's động hào (1..=6).
    pub dong_hao: u8,
    /// Ngũ Hành element of Thể.
    pub the_element: FiveElement,
    /// Ngũ Hành element of Dụng.
    pub dung_element: FiveElement,
    /// 5-way sinh/khắc relation between Thể and Dụng.
    pub relation: TheDungRelation,
    /// Cát / Hung / Bình verdict derived from `relation`.
    pub verdict: CatHung,
}

// ===========================================================================
// classify_the_dung — the public entry point
// ===========================================================================

/// Classify a [`MaiHoaCast`] into its Thể / Dụng / Ngũ Hành / sinh-khắc /
/// cát-hùng surface.
///
/// Pure function: no RNG, no wall-clock, no filesystem.
///
/// Algorithm:
///
/// 1. Determine Dụng trigram by động hào position:
///    - `1..=3` → Dụng = `cast.lower_trigram`, Thể = `cast.upper_trigram`.
///    - `4..=6` → Dụng = `cast.upper_trigram`, Thể = `cast.lower_trigram`.
/// 2. `the_element = trigram_element(the_trigram)`.
/// 3. `dung_element = trigram_element(dung_trigram)`.
/// 4. Derive `relation`:
///    - if `the_element == dung_element` → `Dong`,
///    - else if `generates(dung_element, the_element)` → `DungSinhThe`,
///    - else if `generates(the_element, dung_element)` → `TheSinhDung`,
///    - else if `controls(the_element, dung_element)` → `TheKhacDung`,
///    - else `controls(dung_element, the_element)` → `DungKhacThe`.
///      (Sinh + khắc cycles are complementary so the final else is exhaustive.)
/// 5. `verdict = relation.cat_hung()`.
pub fn classify_the_dung(cast: &MaiHoaCast) -> TheDungClassification {
    // 1. Determine Dụng trigram by động hào position.
    //
    //    Lower trigram covers lines 1-3 (bottom); upper trigram covers 4-6
    //    (top). động hào 1-3 lives in the lower trigram (so lower is Dụng,
    //    upper is Thể). động hào 4-6 lives in the upper trigram (so upper
    //    is Dụng, lower is Thể).
    assert!(
        (1..=6).contains(&cast.dong_hao),
        "dong_hao out of range: {} (must be 1..=6)",
        cast.dong_hao
    );
    let (the_trigram, dung_trigram) = if cast.dong_hao <= 3 {
        (cast.upper_trigram, cast.lower_trigram)
    } else {
        (cast.lower_trigram, cast.upper_trigram)
    };

    // 2. Element mapping (Bát Quái Ngũ Hành).
    let the_element = trigram_element(the_trigram);
    let dung_element = trigram_element(dung_trigram);

    // 3. Derive the 5-way sinh/khắc relation.
    //
    //    The sinh + khắc cycles are complementary, so for any pair of
    //    DISTINCT elements exactly one of `generates(a,b)`, `generates(b,a)`,
    //    `controls(a,b)`, `controls(b,a)` is true. Matching same-element
    //    first (Dong) ensures the final `else` is reachable for distinct
    //    pairs only.
    let relation = if the_element == dung_element {
        TheDungRelation::Dong
    } else if generates(dung_element, the_element) {
        TheDungRelation::DungSinhThe
    } else if generates(the_element, dung_element) {
        TheDungRelation::TheSinhDung
    } else if controls(the_element, dung_element) {
        TheDungRelation::TheKhacDung
    } else {
        // controls(dung_element, the_element) — exhaustive by the
        // sinh+khắc complementarity.
        debug_assert!(
            controls(dung_element, the_element),
            "non-same-element pair ({the_element:?}, {dung_element:?}) \
             must relate by exactly one of sinh/khắc"
        );
        TheDungRelation::DungKhacThe
    };

    // 4. Verdict derived from the relation.
    let verdict = relation.cat_hung();

    TheDungClassification {
        the_trigram,
        dung_trigram,
        dong_hao: cast.dong_hao,
        the_element,
        dung_element,
        relation,
        verdict,
    }
}

// ===========================================================================
// Inline tests — fail under RED, pass under GREEN.
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use crate::iching::schema::{compose, KingWenHexagram};

    /// Every trigram maps to its intrinsic Ngũ Hành element per the Bát Quái
    /// Ngũ Hành table.
    #[test]
    fn trigram_element_maps_eight_trigrams() {
        assert_eq!(trigram_element(TienThienTrigram::Kien), FiveElement::Kim);
        assert_eq!(trigram_element(TienThienTrigram::Doai), FiveElement::Kim);
        assert_eq!(trigram_element(TienThienTrigram::Ly), FiveElement::Hoa);
        assert_eq!(trigram_element(TienThienTrigram::Chan), FiveElement::Moc);
        assert_eq!(trigram_element(TienThienTrigram::Ton), FiveElement::Moc);
        assert_eq!(trigram_element(TienThienTrigram::Kham), FiveElement::Thuy);
        assert_eq!(trigram_element(TienThienTrigram::Can), FiveElement::Tho);
        assert_eq!(trigram_element(TienThienTrigram::Khon), FiveElement::Tho);
    }

    /// The mapping covers every variant of `TienThienTrigram::ALL` (8 entries).
    /// Asserts the table is exhaustive (no unmapped trigram).
    #[test]
    fn trigram_element_covers_all_eight_trigrams() {
        for t in TienThienTrigram::ALL {
            // Just exercising the mapping — the assertion is implicit (no panic).
            let _e = trigram_element(t);
        }
    }

    /// The (8, 8, 8, 8) boundary cast is the classic all-Khôn case (chủ quẻ
    /// #2 Thuần Khôn, dong_hao 2 → lower trigram is Dụng).
    ///
    /// Both Thể and Dụng are Khôn → same element → `Dong` → verdict = Bình.
    #[test]
    fn classify_all_eights_yields_dong_binh() {
        use crate::iching::mai_hoa::cast_mai_hoa;
        let cast = cast_mai_hoa(8, 8, 8, 8);
        // Preconditions from the boundary cast.
        assert_eq!(cast.chu_que, KingWenHexagram(2));
        assert_eq!(cast.upper_trigram, TienThienTrigram::Khon);
        assert_eq!(cast.lower_trigram, TienThienTrigram::Khon);
        assert_eq!(cast.dong_hao, 2);

        let td = classify_the_dung(&cast);

        assert_eq!(td.the_trigram, TienThienTrigram::Khon);
        assert_eq!(td.dung_trigram, TienThienTrigram::Khon);
        assert_eq!(td.dong_hao, 2);
        assert_eq!(td.the_element, FiveElement::Tho);
        assert_eq!(td.dung_element, FiveElement::Tho);
        assert_eq!(td.relation, TheDungRelation::Dong);
        assert_eq!(td.verdict, CatHung::Binh);
    }

    /// Synthetic cast with động hào in the UPPER trigram (dong_hao=4):
    /// upper is Dụng, lower is Thể.
    ///
    /// upper = Kiền (Kim), lower = Chấn (Mộc). Kim khắc Mộc → Dụng khắc Thể
    /// → verdict = Hung.
    #[test]
    fn classify_upper_dong_hao_dung_khac_the_is_hung() {
        let upper = TienThienTrigram::Kien;
        let lower = TienThienTrigram::Chan;
        let chu_que = compose(upper, lower);
        let cast = MaiHoaCast {
            lunar_year_branch: 0,
            lunar_month: 1,
            lunar_day: 1,
            chi_hour_index: 0,
            upper_trigram: upper,
            lower_trigram: lower,
            dong_hao: 4,
            chu_que,
        };

        let td = classify_the_dung(&cast);

        // dong_hao 4 → upper is Dụng, lower is Thể.
        assert_eq!(td.the_trigram, TienThienTrigram::Chan);
        assert_eq!(td.dung_trigram, TienThienTrigram::Kien);
        assert_eq!(td.the_element, FiveElement::Moc);
        assert_eq!(td.dung_element, FiveElement::Kim);
        // Kim khắc Mộc: DongHa → Kim controls Moc.
        assert_eq!(td.relation, TheDungRelation::DungKhacThe);
        assert_eq!(td.verdict, CatHung::Hung);
    }

    /// Dụng sinh Thể (situation nourishes subject) → verdict = Cát.
    ///
    /// Thể = Kiền (Kim), Dụng = Thổ → Thổ sinh Kim.
    /// Use a cast where Dụng is the lower trigram (dong_hao in 1..=3):
    /// lower Cấn (Tho), upper Kiền (Kim), dong_hao = 1 → lower is Dụng.
    #[test]
    fn classify_dung_sinh_the_is_cat() {
        let upper = TienThienTrigram::Kien; // Kim → Thể
        let lower = TienThienTrigram::Can; // Tho → Dụng
        let chu_que = compose(upper, lower);
        let cast = MaiHoaCast {
            lunar_year_branch: 0,
            lunar_month: 1,
            lunar_day: 1,
            chi_hour_index: 0,
            upper_trigram: upper,
            lower_trigram: lower,
            dong_hao: 1, // → lower Dụng
            chu_que,
        };

        let td = classify_the_dung(&cast);

        assert_eq!(td.the_trigram, TienThienTrigram::Kien);
        assert_eq!(td.dung_trigram, TienThienTrigram::Can);
        assert_eq!(td.the_element, FiveElement::Kim);
        assert_eq!(td.dung_element, FiveElement::Tho);
        assert_eq!(td.relation, TheDungRelation::DungSinhThe);
        assert_eq!(td.verdict, CatHung::Cat);
    }

    /// Thể khắc Dụng (subject controls situation) → verdict = Cát.
    ///
    /// Thể = Kiền (Kim), Dụng = Chấn (Mộc). Kim khắc Mộc (the subject
    /// overcomes the situation — auspicious).
    /// Use a cast where Dụng is upper (dong_hao = 4 → upper is Dụng):
    /// upper Chấn (Moc), lower Kiền (Kim), dong_hao = 4 → upper is Dụng.
    #[test]
    fn classify_the_khac_dung_is_cat() {
        let upper = TienThienTrigram::Chan; // Moc → Dụng
        let lower = TienThienTrigram::Kien; // Kim → Thể
        let chu_que = compose(upper, lower);
        let cast = MaiHoaCast {
            lunar_year_branch: 0,
            lunar_month: 1,
            lunar_day: 1,
            chi_hour_index: 0,
            upper_trigram: upper,
            lower_trigram: lower,
            dong_hao: 4, // → upper Dụng
            chu_que,
        };

        let td = classify_the_dung(&cast);

        assert_eq!(td.the_trigram, TienThienTrigram::Kien);
        assert_eq!(td.dung_trigram, TienThienTrigram::Chan);
        assert_eq!(td.the_element, FiveElement::Kim);
        assert_eq!(td.dung_element, FiveElement::Moc);
        assert_eq!(td.relation, TheDungRelation::TheKhacDung);
        assert_eq!(td.verdict, CatHung::Cat);
    }

    /// Thể sinh Dụng (subject depleted by situation — "chủ hao") → verdict
    /// = Hung.
    ///
    /// Thể = Chấn (Mộc), Dụng = Ly (Hỏa). Mộc sinh Hỏa.
    /// Use a cast where Dụng is lower (dong_hao = 1 → lower is Dụng).
    /// upper Chấn (Moc) → Thể, lower Ly (Hoa) → Dụng, dong_hao = 1.
    #[test]
    fn classify_the_sinh_dung_is_hung() {
        let upper = TienThienTrigram::Chan; // Moc → Thể
        let lower = TienThienTrigram::Ly; // Hoa → Dụng
        let chu_que = compose(upper, lower);
        let cast = MaiHoaCast {
            lunar_year_branch: 0,
            lunar_month: 1,
            lunar_day: 1,
            chi_hour_index: 0,
            upper_trigram: upper,
            lower_trigram: lower,
            dong_hao: 1, // → lower Dụng
            chu_que,
        };

        let td = classify_the_dung(&cast);

        assert_eq!(td.the_trigram, TienThienTrigram::Chan);
        assert_eq!(td.dung_trigram, TienThienTrigram::Ly);
        assert_eq!(td.the_element, FiveElement::Moc);
        assert_eq!(td.dung_element, FiveElement::Hoa);
        assert_eq!(td.relation, TheDungRelation::TheSinhDung);
        assert_eq!(td.verdict, CatHung::Hung);
    }

    /// All 5 sinh pair assertions + all 5 khắc pair assertions (8 entries:
    /// 5 (Moc,Hoa), 5 (Hoa,Tho), 5 (Tho,Kim), 5 (Kim,Thuy), 5 (Thuy,Moc) for
    /// sinh; 5 (Moc,Tho), 5 (Tho,Thuy), 5 (Thuy,Hoa), 5 (Hoa,Kim), 5
    /// (Kim,Moc) for khắc).
    #[test]
    fn sinh_khac_cycles_match_classical_tables() {
        // sinh cycle: Moc -> Hoa -> Tho -> Kim -> Thuy -> Moc.
        assert!(
            generates(FiveElement::Moc, FiveElement::Hoa),
            "Moc sinh Hoa"
        );
        assert!(
            generates(FiveElement::Hoa, FiveElement::Tho),
            "Hoa sinh Tho"
        );
        assert!(
            generates(FiveElement::Tho, FiveElement::Kim),
            "Tho sinh Kim"
        );
        assert!(
            generates(FiveElement::Kim, FiveElement::Thuy),
            "Kim sinh Thuy"
        );
        assert!(
            generates(FiveElement::Thuy, FiveElement::Moc),
            "Thuy sinh Moc"
        );

        // khắc cycle: Moc -> Tho -> Thuy -> Hoa -> Kim -> Moc.
        assert!(controls(FiveElement::Moc, FiveElement::Tho), "Moc khac Tho");
        assert!(
            controls(FiveElement::Tho, FiveElement::Thuy),
            "Tho khac Thuy"
        );
        assert!(
            controls(FiveElement::Thuy, FiveElement::Hoa),
            "Thuy khac Hoa"
        );
        assert!(controls(FiveElement::Hoa, FiveElement::Kim), "Hoa khac Kim");
        assert!(controls(FiveElement::Kim, FiveElement::Moc), "Kim khac Moc");
    }

    /// Helper coverage: the sinh/khắc helpers cover all 5×5 = 25 distinct
    /// (a, b) pairs without panic.
    #[test]
    fn sinh_khac_helpers_cover_all_25_pairs() {
        for a in FiveElement::ALL {
            for b in FiveElement::ALL {
                let _ = generates(a, b);
                let _ = controls(a, b);
            }
        }
    }

    /// CRIT-3 isolation guard — this module must NOT define any cross-newtype
    /// `From` impl. `trigram_element` is a plain `fn`, not a trait impl. See
    /// the mai_hoa.rs companion test for the runtime-built needle pattern.
    #[test]
    fn crit3_isolation_no_cross_newtype_from_impls_inline() {
        const SRC: &str = include_str!("the_dung.rs");
        let needles: Vec<String> = [
            ("Tien", "ThienTrigram"),
            ("Hau", "ThienTrigram"),
            ("King", "WenHexagram"),
        ]
        .iter()
        .flat_map(|(a, b)| [format!("impl From<{a}{b}"), format!("impl<{a}{b}> From")])
        .collect();
        for needle in &needles {
            assert!(
                !SRC.contains(needle.as_str()),
                "CRIT-3 violation: `{needle}` found in the_dung.rs. \
                 The three iching newtypes must NOT have cross-type From impls."
            );
        }
    }
}
